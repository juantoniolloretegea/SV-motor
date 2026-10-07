use std::{fs,io::{Read,Write,BufRead,BufReader},path::{Path,PathBuf},process::{Command,Stdio,Child},sync::{Arc,Mutex,atomic::{AtomicBool,Ordering},mpsc},thread,time::{Duration,SystemTime,UNIX_EPOCH}};
use sv_arbitro_comprobaciones::ciclo::{Puerta,validar_solicitud};
use serde_json::{Value,json};use sha2::{Sha256,Digest};
type R<T>=Result<T,Box<dyn std::error::Error+Send+Sync>>;
const BASE:&str="/opt/sv-safeguard";
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn now()->u128{SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros()}
fn command(p:&str,a:&[&str])->R<Vec<u8>>{let o=Command::new(p).args(a).output()?;if !o.status.success(){return Err(format!("{p}: {}",String::from_utf8_lossy(&o.stderr)).into())}Ok(o.stdout)}
fn save(p:&Path,b:&[u8])->R<()>{let mut f=fs::OpenOptions::new().write(true).create_new(true).open(p)?;f.write_all(b)?;f.sync_all()?;Ok(())}
fn sandbox(unit:&str,user:&str,memory:&str)->Command {
 let mut c=Command::new("systemd-run");c.args(["--quiet","--wait","--pipe","--service-type=exec","--slice=svsgdiag02.slice"]);
 c.arg(format!("--unit={unit}")).arg(format!("--uid={user}")).arg(format!("--gid={user}"));
 for p in ["BindsTo=sv-diag02-custodia.service","After=sv-diag02-custodia.service","RootDirectory=/opt/sv-safeguard/aislado-v2-r1","MountAPIVFS=yes","PrivateNetwork=yes","PrivateDevices=yes","PrivateTmp=yes","PrivatePIDs=yes","ProtectProc=invisible","ProtectSystem=strict","ProtectHome=yes","NoNewPrivileges=yes","CapabilityBoundingSet=","RestrictAddressFamilies=AF_UNIX","RestrictNamespaces=yes","RestrictSUIDSGID=yes","LockPersonality=yes","MemoryDenyWriteExecute=yes","MemorySwapMax=0","CPUAffinity=0-29","TasksMax=1024","LimitCORE=0","Environment=HF_HUB_OFFLINE=1 HF_HOME=/tmp/hf TOKENIZERS_PARALLELISM=false RAYON_NUM_THREADS=30 HOME=/tmp OMP_NUM_THREADS=30","UnsetEnvironment=HTTP_PROXY HTTPS_PROXY ALL_PROXY http_proxy https_proxy all_proxy NO_PROXY no_proxy","BindReadOnlyPaths=/opt/sv-safeguard/retroalimentacion-20261003/diagnostico-A08-02/cache:/cache /opt/sv-safeguard/retroalimentacion-20261003/diagnostico-A08-02/config:/config /opt/sv-safeguard/modelo:/modelo /opt/sv-safeguard/fuentes/mistral.rs/target/release/sv-safeguard-conductor-diag-a08-02:/bin/conductor"]{c.arg(format!("--property={p}"));}
 c.arg(format!("--property=MemoryMax={memory}")).arg("--property=TimeoutStopSec=5s").arg("--setenv=TIKTOKEN_ENCODINGS_BASE=/vocabulario");c
}
struct Journal{file:fs::File,seq:u64,previous:String,bytes:usize}
impl Journal{
 fn append(&mut self,channel:&str,raw:&[u8])->R<()> {
  let mut v=json!({"n":self.seq,"unix_us":now(),"canal":channel,"datos_utf8":String::from_utf8_lossy(raw),"bytes_hex":raw.iter().map(|b|format!("{b:02x}")).collect::<String>(),"bytes_sha256":h(raw),"anterior_sha256":self.previous});let hash=h(&serde_json::to_vec(&v)?);v["sha256"]=json!(hash);let bytes=serde_json::to_vec(&v)?;self.bytes+=raw.len()+bytes.len()+2;if self.bytes>512*1024*1024{return Err("Presupuesto de registros agotado".into())}self.file.write_all(&bytes)?;self.file.write_all(b"\n")?;self.file.sync_data()?;self.previous=hash;self.seq+=1;Ok(())}
}
type Log=Arc<Mutex<Journal>>;
fn log(l:&Log,c:&str,b:&[u8])->R<()>{l.lock().map_err(|_|"Custodia bloqueada")?.append(c,b)}
fn pump<Rd:Read+Send+'static>(reader:Rd,channel:&'static str,path:PathBuf,l:Log,tx:Option<mpsc::Sender<Vec<u8>>>)->thread::JoinHandle<R<()>>{thread::spawn(move||{let mut raw=fs::OpenOptions::new().create_new(true).write(true).open(path)?;let mut r=BufReader::new(reader);loop{let mut b=Vec::new();let n=r.by_ref().take(1024*1024).read_until(b'\n',&mut b)?;if n==0{break}if !b.ends_with(b"\n"){return Err("Trama incompleta o superior a un MiB".into())}raw.write_all(&b)?;raw.sync_data()?;log(&l,channel,&b)?;if let Some(t)=&tx{t.send(b).map_err(|_|"Receptor cerrado")?}}Ok(())})}
fn stop_own(){for u in ["sv-diag02-modelo.service","sv-diag02-mcp.service"]{let _=Command::new("systemctl").args(["stop",u]).status();}}
fn watch(out:PathBuf,l:Log,done:Arc<AtomicBool>)->thread::JoinHandle<R<()>>{thread::spawn(move||{let mut f=fs::OpenOptions::new().create_new(true).write(true).open(out)?;while !done.load(Ordering::SeqCst){let mut v=json!({"unix_us":now(),"meminfo":fs::read_to_string("/proc/meminfo")?,"loadavg":fs::read_to_string("/proc/loadavg")?});
 for(unit,p)in [("conjunto","/sys/fs/cgroup/svsgdiag02.slice"),("modelo","/sys/fs/cgroup/svsgdiag02.slice/sv-diag02-modelo.service"),("mcp","/sys/fs/cgroup/svsgdiag02.slice/sv-diag02-mcp.service")]{let mut d=json!({});for n in ["memory.current","memory.peak","memory.events","memory.swap.current","memory.swap.peak","cpu.stat","pids.current","cgroup.procs"]{d[n]=match fs::read_to_string(Path::new(p).join(n)){Ok(s)=>json!(s),Err(e)=>json!({"ausente":e.to_string()})};}
  if let Some(s)=d["cgroup.procs"].as_str(){let procs=s.lines().map(|pid|json!({"pid":pid,"status":fs::read_to_string(format!("/proc/{pid}/status")).ok(),"stat":fs::read_to_string(format!("/proc/{pid}/stat")).ok(),"io":fs::read_to_string(format!("/proc/{pid}/io")).ok()})).collect::<Vec<_>>();d["procesos"]=json!(procs);}v[unit]=d;
 }
 let b=serde_json::to_vec(&v)?;f.write_all(&b)?;f.write_all(b"\n")?;f.sync_data()?;log(&l,"telemetria",&b)?;thread::sleep(Duration::from_secs(2));}Ok(())})}

fn punto(out:&Path,l:&Log,puerta:&Puerta)->R<()>{
 let info=l.lock().map_err(|_|"Custodia bloqueada")?;info.file.sync_all()?;
 let dest=out.join("puntos").join(puerta.id());fs::create_dir_all(dest.parent().unwrap())?;fs::create_dir(&dest)?;fs::create_dir(dest.join("mcp"))?;
 let mut reception=fs::File::open(out.join("RECEPCION.jsonl"))?;
 let mut copy=fs::OpenOptions::new().create_new(true).write(true).open(dest.join("RECEPCION.jsonl"))?;
 std::io::copy(&mut reception,&mut copy)?;copy.sync_all()?;
 let mut canales=std::collections::BTreeMap::new();
 for(ch,file)in [("modelo_stdout","modelo.stdout"),("modelo_stderr","modelo.stderr"),("mcp_stdout","mcp.stdout"),("mcp_stderr","mcp.stderr"),("telemetria","TELEMETRIA.jsonl")]{
  canales.insert(ch,(file,fs::OpenOptions::new().create_new(true).write(true).open(dest.join(file))?,0u64,Sha256::new()));
 }
 let reader=BufReader::new(fs::File::open(dest.join("RECEPCION.jsonl"))?);
 for line in reader.lines(){
  let line=line?;let v:Value=serde_json::from_str(&line)?;
  if let Some((_,file,len,digest))=canales.get_mut(v["canal"].as_str().ok_or("Canal ausente")?){
   let hex=v["bytes_hex"].as_str().ok_or("Bytes ausentes")?;
   let mut raw=(0..hex.len()).step_by(2).map(|i|u8::from_str_radix(&hex[i..i+2],16)).collect::<Result<Vec<_>,_>>()?;
   if v["canal"]=="telemetria"{raw.push(b'\n');}file.write_all(&raw)?;*len+=raw.len()as u64;digest.update(&raw);
  }
 }
 let mut archivos=Vec::new();
 for(_, (name,file,len,digest))in canales{
  file.sync_all()?;drop(file);
  let mut actual=fs::File::open(out.join(name))?;let original_len=actual.metadata()?.len();let mut prefix=fs::File::open(dest.join(name))?;
  let mut restantes=len;let mut pa=[0u8;65536];let mut pb=[0u8;65536];
  while restantes>0{let n=(restantes as usize).min(pa.len());prefix.read_exact(&mut pa[..n])?;actual.read_exact(&mut pb[..n])?;if pa[..n]!=pb[..n]{return Err("Punto no coincide con originales conservados".into())}restantes-=n as u64;}
  archivos.push(json!({"archivo":name,"bytes":len,"sha256":format!("{:x}",digest.finalize()),"original_bytes_observados":original_len,"prefijo_exacto":true}));
 }
 let diary=fs::read(out.join("mcp/diario.jsonl"))?;save(&dest.join("mcp/diario.jsonl"),&diary)?;
 let seal=json!({"id":puerta.id(),"salida_sha256":puerta.salida_sha256(),"adjudicacion_pendiente":puerta.pendientes(),"ultimo_hash_recepcion":info.previous,"registros":info.seq,"archivos":archivos,"diario_mcp_sha256":h(&diary),"inferencia_siguiente_impedida":true});
 save(&dest.join("PUNTO.json"),&serde_json::to_vec_pretty(&seal)?)?;Ok(())
}

fn finish_process(p:&mut Child)->R<i32>{Ok(p.wait()?.code().unwrap_or(-1))}
fn comprobar_admision_contraste(v:&Value)->R<()>{if v.as_array().map(Vec::len)!=Some(sv_arbitro_comprobaciones::ciclo::IDS.len()){return Err("Admisión externa con número de entradas distinto del banco fijado".into())}Ok(())}
fn run(mode:&str,id:&str)->R<()> {
 if !id.chars().all(|c|c.is_ascii_alphanumeric()||c=='-'){return Err("Identificador inválido".into())}
 if mode!="instrumental" && mode!="contraste"{return Err("Modo no autorizado".into())}
 if mode=="contraste" && id!="diag-A08-02"{return Err("Sólo la condición fijada".into())}
 if mode=="contraste" && Path::new("/opt/sv-safeguard/retroalimentacion-20261003/diagnostico-A08-02/UNICA-CARGA-ADMITIDA.json").exists(){return Err("Carga anterior preservada; no se admite repetición".into())}
 let esperada:Option<Value>=if mode=="contraste"{Some(serde_json::from_slice(&fs::read("/opt/sv-safeguard/retroalimentacion-20261003/diagnostico-A08-02/config/ADMISION.json")?)?)}else{None};
 if mode=="contraste"{comprobar_admision_contraste(esperada.as_ref().ok_or("Admisión ausente")?)?;}

 let out=Path::new(BASE).join("evidencias").join(id);if out.exists(){return Err("Antecedente preservado".into())}fs::create_dir(&out)?;
 let file=fs::OpenOptions::new().write(true).create_new(true).open(out.join("RECEPCION.jsonl"))?;let l=Arc::new(Mutex::new(Journal{file,seq:0,previous:String::new(),bytes:0}));
 let cat=Path::new(BASE).join("retroalimentacion-20261003/diagnostico-A08-02/cache/catalogo.json");let hash=h(&fs::read(&cat)?);let mcpdir=out.join("mcp");fs::create_dir(&mcpdir)?;command("chown",&["sv-sg-mcp:sv-sg-mcp",mcpdir.to_str().unwrap()])?;command("chmod",&["700",mcpdir.to_str().unwrap()])?;
 let mut mc=sandbox("sv-diag02-mcp","sv-sg-mcp","512M");mc.arg(format!("--property=BindPaths={}:/custodia",mcpdir.display())).arg("--property=ReadWritePaths=/custodia").args(["/bin/mcp","/cache/catalogo.json",&hash,"/custodia/diario.jsonl","256","--sintetico"]);
 let mut ec=sandbox("sv-diag02-modelo","sv-sg-engine","110G");ec.args(["/bin/conductor",mode]);
 log(&l,"orden_mcp",format!("{mc:?}").as_bytes())?;log(&l,"orden_modelo",format!("{ec:?}").as_bytes())?;
 let done=Arc::new(AtomicBool::new(false));let observer=watch(out.join("TELEMETRIA.jsonl"),l.clone(),done.clone());
 let mut mcp=mc.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
 let (mtx,mrx)=mpsc::channel();let mo=pump(mcp.stdout.take().unwrap(),"mcp_stdout",out.join("mcp.stdout"),l.clone(),Some(mtx));let me=pump(mcp.stderr.take().unwrap(),"mcp_stderr",out.join("mcp.stderr"),l.clone(),None);
 let mut engine=ec.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
 let (etx,erx)=mpsc::channel();let eo=pump(engine.stdout.take().unwrap(),"modelo_stdout",out.join("modelo.stdout"),l.clone(),Some(etx));let ee=pump(engine.stderr.take().unwrap(),"modelo_stderr",out.join("modelo.stderr"),l.clone(),None);
 let mut success=false;let mut error=None;
 let plan:Value=serde_json::from_slice(&fs::read("/opt/sv-safeguard/retroalimentacion-20261003/diagnostico-A08-02/config/plan.json")?)?;let mut puerta=Puerta::nueva(&plan)?;let mut solicitudes=0usize;let mut entradas=0usize;
 let mut pendiente:Option<Value>=None;let inicio_diagnostico=std::time::Instant::now();let mut progreso=std::time::Instant::now();
 let recorrido:R<()>= (||{
  loop {
   if inicio_diagnostico.elapsed()>Duration::from_secs(3600){return Err("Cota total de una hora alcanzada; conservar sin reintento".into())}
   if progreso.elapsed()>Duration::from_secs(900){return Err("Quince minutos sin avance en hitos de cálculo o comunicación; cierre técnico".into())}
   if observer.is_finished(){return Err("Pérdida de observabilidad".into())}
   let events=fs::read_to_string("/sys/fs/cgroup/svsgdiag02.slice/memory.events")?;
   if events.lines().any(|s|(s.starts_with("oom ")||s.starts_with("oom_kill "))&&!s.ends_with(" 0")){return Err("Incidencia de memoria conjunta".into())}
   if let Some(d)=pendiente.as_ref(){
    let ruta=Path::new("/opt/sv-safeguard/retroalimentacion-20261003/diagnostico-A08-02/autorizaciones").join(format!("{}.json",puerta.id()));
    if ruta.exists(){
     let original=fs::read(&ruta)?;let v:Value=serde_json::from_slice(&original)?;
     log(&l,"control_externo",&original)?;
     let continuar=puerta.adjudicar(&v)?;
     save(&out.join(format!("CONTROL-{}.json",v["id"].as_str().unwrap())),&original)?;
     let ack=format!("{}\n",json!({"custodia":"conservada","etapa":"adjudicacion","id":d["id"],"entrada_sha256":d["entrada_sha256"],"continuar":continuar}));
     log(&l,"confirmacion_custodia",ack.as_bytes())?;
     engine.stdin.as_mut().ok_or("Conductor sin recepción")?.write_all(ack.as_bytes())?;engine.stdin.as_mut().unwrap().flush()?;
     pendiente=None;
    }
   }
   match erx.recv_timeout(Duration::from_millis(500)){
    Ok(b)=>{if let Some(raw)=b.strip_prefix(b"SV_EVENT "){
     let v:Value=serde_json::from_slice(raw)?;let d=&v["datos"];
     if matches!(d["evento"].as_str(),Some("carga_inicio"|"carga_fin"|"calculo_entrada"|"calculo_fin"|"token"|"caso_inicio"|"mcp_solicitud"|"contexto_previsto"|"emision_integra"|"caso_fin")){progreso=std::time::Instant::now();}
     match d["evento"].as_str(){
      Some("mcp_solicitud")=>{
       let wire=d["wire"].as_str().ok_or("Solicitud sin bytes")?.as_bytes();
       if d["actor"]!="arbitro"{return Err("Atribución MCP discordante".into())}
       solicitudes+=1;if solicitudes>128{return Err("Presupuesto de comunicaciones agotado".into())}
       if mode=="contraste"||entradas<1{validar_solicitud(wire,puerta.documento(),puerta.seccion())?;}
       log(&l,"mcp_stdin",wire)?;mcp.stdin.as_mut().ok_or("MCP sin entrada")?.write_all(wire)?;mcp.stdin.as_mut().unwrap().flush()?;
       if d["espera"]==true{
        let reply=mrx.recv_timeout(Duration::from_secs(30))?;
        log(&l,"modelo_stdin",&reply)?;engine.stdin.as_mut().ok_or("Conductor sin entrada")?.write_all(&reply)?;engine.stdin.as_mut().unwrap().flush()?;
       }
      },
      Some("contexto_previsto")=>{puerta.preparar(d,esperada.as_ref().and_then(|e|e.as_array()).and_then(|a|a.get(entradas)))?;entradas+=1;},
      Some("contexto_conductor")=>{puerta.motor(d)?;},
      Some("emision_integra")=>{puerta.emision(d)?;},
      Some("caso_fin")=>{puerta.terminar(d)?;},
      Some("solicitud_custodia")=>{
       let etapa=d["etapa"].as_str().ok_or("Etapa ausente")?;
       let caso=d["id"].as_str().ok_or("Caso ausente")?;
       puerta.solicitar(etapa,caso,d["entrada_sha256"].as_str().ok_or("Huella ausente")?,mode)?;
       if etapa=="carga"{save(Path::new("/opt/sv-safeguard/retroalimentacion-20261003/diagnostico-A08-02/UNICA-CARGA-ADMITIDA.json"),&serde_json::to_vec(d)?)?;}
       if etapa=="generacion"{save(&Path::new("/opt/sv-safeguard/retroalimentacion-20261003/diagnostico-A08-02").join(format!("UNICA-GENERACION-{caso}.json")),&serde_json::to_vec(d)?)?;}
       if etapa=="adjudicacion"{
        punto(&out,&l,&puerta)?;
        let carpeta=out.join("puntos").join(puerta.id());
        let auditoria=sv_arbitro_comprobaciones::auditoria::auditar(&carpeta,false,true).map_err(|e|e.to_string())?;
        let informes=out.join("cotejos");fs::create_dir_all(&informes)?;
        save(&informes.join(format!("{}-custodia.json",puerta.id())),&serde_json::to_vec_pretty(&auditoria)?)?;
        let verificador="/opt/sv-safeguard/fuentes/mistral.rs/target/release/sv-safeguard-verificador-diag-a08-02";
        let informe=informes.join(format!("{}-entrada.json",puerta.id()));
        let salida=command(verificador,&[carpeta.to_str().unwrap(),"parcial",informe.to_str().unwrap()])?;
        log(&l,"cotejo_entrada",&salida)?;
        // Continuidad instrumental prefijada. No adjudica contenido ni aptitud.
        let control=json!({"id":puerta.id(),"salida_sha256":puerta.salida_sha256(),"accion":if puerta.ultima(){"cerrar"}else{"continuar"}});
        let ruta=Path::new("/opt/sv-safeguard/retroalimentacion-20261003/diagnostico-A08-02/autorizaciones").join(format!("{}.json",puerta.id()));
        save(&ruta,&serde_json::to_vec_pretty(&control)?)?;
        pendiente=Some(d.clone());progreso=std::time::Instant::now();
       }
       else{
        let ack=format!("{}\n",json!({"custodia":"conservada","etapa":etapa,"id":caso,"entrada_sha256":d["entrada_sha256"]}));
        log(&l,"confirmacion_custodia",ack.as_bytes())?;
        engine.stdin.as_mut().ok_or("Conductor sin recepción")?.write_all(ack.as_bytes())?;engine.stdin.as_mut().unwrap().flush()?;
       }
      },
      Some("fin_conductor")=>{if mode=="contraste"&&!puerta.cerrado(){return Err("Cierre sin adjudicación final".into())}success=true;},
      Some("fallo_conductor"|"impedimento")=>return Err(d.to_string().into()),
      _=>{}
     }
    }},
    Err(mpsc::RecvTimeoutError::Disconnected)=>break,
    Err(mpsc::RecvTimeoutError::Timeout)=>{if eo.is_finished()&&!success{return Err("Pérdida del emisor".into())}}
   }
  }Ok(())
 })();
 if let Err(e)=recorrido{error=Some(e.to_string());}
 engine.stdin.take();mcp.stdin.take();
 if error.is_some(){thread::sleep(Duration::from_millis(200));stop_own()}
 let ec=finish_process(&mut engine)?;let mc=finish_process(&mut mcp)?;
 done.store(true,Ordering::SeqCst);
 for th in [eo,ee,mo,me,observer]{match th.join(){Ok(Ok(()))=>{},Ok(Err(e))=>error=Some(e.to_string()),Err(_)=>error=Some("Hilo de custodia interrumpido".into())}}
 let verifier=Command::new("/opt/sv-safeguard/v2/fuentes/mcp-0.1.3/target/release/verificar-diario").arg(&cat).arg(&hash).arg(mcpdir.join("diario.jsonl")).arg("--sintetico").output()?;
 save(&out.join("COTEJO-MCP.json"),&verifier.stdout)?;save(&out.join("COTEJO-MCP.stderr"),&verifier.stderr)?;
 let info=l.lock().unwrap();let result=json!({"fin_unix_us":now(),"fin_conductor":success,"codigo_modelo":ec,"codigo_mcp":mc,"diario_mcp_conforme":verifier.status.success(),"error":error,"ultimo_hash_recepcion":info.previous,"registros":info.seq,"bytes_registrados":info.bytes,"memoria_conjunta_peak":fs::read_to_string("/sys/fs/cgroup/svsgdiag02.slice/memory.peak").ok(),"memoria_eventos_final":fs::read_to_string("/sys/fs/cgroup/svsgdiag02.slice/memory.events").ok(),"swap_conjunto_final":fs::read_to_string("/sys/fs/cgroup/svsgdiag02.slice/memory.swap.current").ok()});drop(info);
 save(&out.join("CIERRE.json"),&serde_json::to_vec_pretty(&result)?)?;println!("{result}");
 if !success||ec!=0||mc!=0||error.is_some()||!verifier.status.success(){return Err("Recorrido no conforme; originales preservados".into())}Ok(())
}
fn probe_mcp()->R<()>{let mut c=sandbox("sv-diag02-sonda-mcp","sv-sg-mcp","512M");c.args(["/bin/conductor","probe"]);let o=c.output()?;std::io::stdout().write_all(&o.stdout)?;std::io::stderr().write_all(&o.stderr)?;if !o.status.success(){return Err("Aislamiento bajo identidad MCP no conforme".into())}Ok(())}
fn main(){let a=std::env::args().collect::<Vec<_>>();let r=match a.get(1).map(String::as_str){Some("comprobar-mcp")=>probe_mcp(),Some("instrumental")|Some("contraste")=>run(&a[1],a.get(2).map(String::as_str).unwrap_or("sin-id")),_=>Err("Modo requerido".into())};if let Err(e)=r{stop_own();eprintln!("{e}");std::process::exit(1)}}


#[cfg(test)] mod tests{use super::*;
 #[test]fn admision_unica(){assert!(comprobar_admision_contraste(&json!([{}])).is_ok());for n in [0,2,3,8,9,10]{assert!(comprobar_admision_contraste(&json!(vec![json!({});n])).is_err());}}
 #[test]fn guardas_y_aislamiento(){let c=sandbox("sv-diag02-prueba","sv-sg-engine","110G");let a=c.get_args().map(|x|x.to_string_lossy().into_owned()).collect::<Vec<_>>();for p in ["--property=PrivateNetwork=yes","--property=MemorySwapMax=0","--property=MemoryMax=110G","--property=ProtectSystem=strict"]{assert!(a.contains(&p.to_string()));}assert!(a.iter().any(|s|s.contains("diagnostico-A08-02/config:/config")));}
}
