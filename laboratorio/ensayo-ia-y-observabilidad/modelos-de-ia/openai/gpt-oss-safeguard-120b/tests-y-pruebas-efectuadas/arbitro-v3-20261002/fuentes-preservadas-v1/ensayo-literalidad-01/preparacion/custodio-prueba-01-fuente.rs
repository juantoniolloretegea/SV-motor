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
 let mut c=Command::new("systemd-run");c.args(["--quiet","--wait","--pipe","--service-type=exec","--slice=svsafeguard.slice"]);
 c.arg(format!("--unit={unit}")).arg(format!("--uid={user}")).arg(format!("--gid={user}"));
 for p in ["RootDirectory=/opt/sv-safeguard/aislado-v2-r1","MountAPIVFS=yes","PrivateNetwork=yes","PrivateDevices=yes","PrivateTmp=yes","PrivatePIDs=yes","ProtectProc=invisible","ProtectSystem=strict","ProtectHome=yes","NoNewPrivileges=yes","CapabilityBoundingSet=","RestrictAddressFamilies=AF_UNIX","RestrictNamespaces=yes","RestrictSUIDSGID=yes","LockPersonality=yes","MemoryDenyWriteExecute=yes","MemorySwapMax=0","CPUAffinity=0-29","TasksMax=1024","LimitCORE=0","Environment=HF_HUB_OFFLINE=1 HF_HOME=/tmp/hf TOKENIZERS_PARALLELISM=false RAYON_NUM_THREADS=30 HOME=/tmp OMP_NUM_THREADS=30","UnsetEnvironment=HTTP_PROXY HTTPS_PROXY ALL_PROXY http_proxy https_proxy all_proxy NO_PROXY no_proxy","BindReadOnlyPaths=/opt/sv-safeguard/arbitro-literalidad-01/contraste/cache:/cache /opt/sv-safeguard/arbitro-literalidad-01/contraste/config:/config /opt/sv-safeguard/modelo:/modelo /opt/sv-safeguard/fuentes/mistral.rs/target/release/sv-safeguard-conductor-lit1:/bin/conductor"]{c.arg(format!("--property={p}"));}
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
fn stop_own(){for u in ["sv-arblit-modelo.service","sv-arblit-mcp.service"]{let _=Command::new("systemctl").args(["stop",u]).status();}}
fn watch(out:PathBuf,l:Log,done:Arc<AtomicBool>)->thread::JoinHandle<R<()>>{thread::spawn(move||{let mut f=fs::OpenOptions::new().create_new(true).write(true).open(out)?;while !done.load(Ordering::SeqCst){let mut v=json!({"unix_us":now(),"meminfo":fs::read_to_string("/proc/meminfo")?,"loadavg":fs::read_to_string("/proc/loadavg")?});
 for(unit,p)in [("conjunto","/sys/fs/cgroup/svsafeguard.slice"),("modelo","/sys/fs/cgroup/svsafeguard.slice/sv-arblit-modelo.service"),("mcp","/sys/fs/cgroup/svsafeguard.slice/sv-arblit-mcp.service")]{let mut d=json!({});for n in ["memory.current","memory.peak","memory.events","memory.swap.current","memory.swap.peak","cpu.stat","pids.current","cgroup.procs"]{d[n]=match fs::read_to_string(Path::new(p).join(n)){Ok(s)=>json!(s),Err(e)=>json!({"ausente":e.to_string()})};}
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
fn run(mode:&str,id:&str)->R<()> {
 if !id.chars().all(|c|c.is_ascii_alphanumeric()||c=='-'){return Err("Identificador inválido".into())}
 if mode!="instrumental" && mode!="contraste"{return Err("Modo no autorizado".into())}
 if mode=="contraste" && id!="arbitro-literalidad-01"{return Err("Sólo la condición fijada".into())}
 if mode=="contraste" && Path::new("/opt/sv-safeguard/arbitro-literalidad-01/UNICA-CARGA-ADMITIDA.json").exists(){return Err("Carga anterior preservada; no se admite repetición".into())}
 let esperada:Option<Value>=if mode=="contraste"{Some(serde_json::from_slice(&fs::read("/opt/sv-safeguard/arbitro-literalidad-01/config/ADMISION.json")?)?)}else{None};
 if mode=="contraste" && esperada.as_ref().and_then(|v|v.as_array()).map(|v|v.len())!=Some(6){return Err("Admisión externa sin seis entradas".into())}

 let out=Path::new(BASE).join("evidencias").join(id);if out.exists(){return Err("Antecedente preservado".into())}fs::create_dir(&out)?;
 let file=fs::OpenOptions::new().write(true).create_new(true).open(out.join("RECEPCION.jsonl"))?;let l=Arc::new(Mutex::new(Journal{file,seq:0,previous:String::new(),bytes:0}));
 let cat=Path::new(BASE).join("arbitro-literalidad-01/contraste/cache/catalogo.json");let hash=h(&fs::read(&cat)?);let mcpdir=out.join("mcp");fs::create_dir(&mcpdir)?;command("chown",&["sv-sg-mcp:sv-sg-mcp",mcpdir.to_str().unwrap()])?;command("chmod",&["700",mcpdir.to_str().unwrap()])?;
 let mut mc=sandbox("sv-arblit-mcp","sv-sg-mcp","512M");mc.arg(format!("--property=BindPaths={}:/custodia",mcpdir.display())).arg("--property=ReadWritePaths=/custodia").args(["/bin/mcp","/cache/catalogo.json",&hash,"/custodia/diario.jsonl","256","--sintetico"]);
 let mut ec=sandbox("sv-arblit-modelo","sv-sg-engine","112G");ec.args(["/bin/conductor",mode]);
 log(&l,"orden_mcp",format!("{mc:?}").as_bytes())?;log(&l,"orden_modelo",format!("{ec:?}").as_bytes())?;
 let done=Arc::new(AtomicBool::new(false));let observer=watch(out.join("TELEMETRIA.jsonl"),l.clone(),done.clone());
 let mut mcp=mc.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
 let (mtx,mrx)=mpsc::channel();let mo=pump(mcp.stdout.take().unwrap(),"mcp_stdout",out.join("mcp.stdout"),l.clone(),Some(mtx));let me=pump(mcp.stderr.take().unwrap(),"mcp_stderr",out.join("mcp.stderr"),l.clone(),None);
 let mut engine=ec.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
 let (etx,erx)=mpsc::channel();let eo=pump(engine.stdout.take().unwrap(),"modelo_stdout",out.join("modelo.stdout"),l.clone(),Some(etx));let ee=pump(engine.stderr.take().unwrap(),"modelo_stderr",out.join("modelo.stderr"),l.clone(),None);
 let mut success=false;let mut error=None;
 let mut puerta=Puerta::default();let mut solicitudes=0usize;let mut entradas=0usize;
 let mut pendiente:Option<Value>=None;
 let recorrido:R<()>= (||{
  loop {
   if observer.is_finished(){return Err("Pérdida de observabilidad".into())}
   let events=fs::read_to_string("/sys/fs/cgroup/svsafeguard.slice/memory.events")?;
   if events.lines().any(|s|(s.starts_with("oom ")||s.starts_with("oom_kill "))&&!s.ends_with(" 0")){return Err("Incidencia de memoria conjunta".into())}
   if let Some(d)=pendiente.as_ref(){
    let ruta=Path::new("/opt/sv-safeguard/arbitro-literalidad-01/autorizaciones").join(format!("{}.json",puerta.id()));
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
     match d["evento"].as_str(){
      Some("mcp_solicitud")=>{
       let wire=d["wire"].as_str().ok_or("Solicitud sin bytes")?.as_bytes();
       if d["actor"]!="arbitro"{return Err("Atribución MCP discordante".into())}
       solicitudes+=1;if solicitudes>128{return Err("Presupuesto de comunicaciones agotado".into())}
       if mode=="contraste"{validar_solicitud(wire,puerta.documento())?;}
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
       if etapa=="carga"{save(Path::new("/opt/sv-safeguard/arbitro-literalidad-01/UNICA-CARGA-ADMITIDA.json"),&serde_json::to_vec(d)?)?;}
       if etapa=="generacion"{save(&Path::new("/opt/sv-safeguard/arbitro-literalidad-01").join(format!("UNICA-GENERACION-{caso}.json")),&serde_json::to_vec(d)?)?;}
       if etapa=="adjudicacion"{punto(&out,&l,&puerta)?;pendiente=Some(d.clone());}
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
 let info=l.lock().unwrap();let result=json!({"fin_unix_us":now(),"fin_conductor":success,"codigo_modelo":ec,"codigo_mcp":mc,"diario_mcp_conforme":verifier.status.success(),"error":error,"ultimo_hash_recepcion":info.previous,"registros":info.seq,"bytes_registrados":info.bytes,"memoria_conjunta_peak":fs::read_to_string("/sys/fs/cgroup/svsafeguard.slice/memory.peak").ok(),"memoria_eventos_final":fs::read_to_string("/sys/fs/cgroup/svsafeguard.slice/memory.events").ok(),"swap_conjunto_final":fs::read_to_string("/sys/fs/cgroup/svsafeguard.slice/memory.swap.current").ok()});drop(info);
 save(&out.join("CIERRE.json"),&serde_json::to_vec_pretty(&result)?)?;println!("{result}");
 if !success||ec!=0||mc!=0||error.is_some()||!verifier.status.success(){return Err("Recorrido no conforme; originales preservados".into())}Ok(())
}
fn probe_mcp()->R<()>{let mut c=sandbox("sv-arblit-sonda-mcp","sv-sg-mcp","512M");c.args(["/bin/conductor","probe"]);let o=c.output()?;std::io::stdout().write_all(&o.stdout)?;std::io::stderr().write_all(&o.stderr)?;if !o.status.success(){return Err("Aislamiento bajo identidad MCP no conforme".into())}Ok(())}
fn main(){let a=std::env::args().collect::<Vec<_>>();let r=match a.get(1).map(String::as_str){Some("comprobar-mcp")=>probe_mcp(),Some("instrumental")|Some("contraste")=>run(&a[1],a.get(2).map(String::as_str).unwrap_or("sin-id")),_=>Err("Modo requerido".into())};if let Err(e)=r{stop_own();eprintln!("{e}");std::process::exit(1)}}

#[cfg(test)]mod tests{
 use super::*;
 fn preparar_punto()->R<(PathBuf,Log,Puerta)>{
  let out=std::env::current_dir()?.join(format!("prueba-punto-{}",now()));fs::create_dir(&out)?;fs::create_dir(out.join("mcp"))?;
  let file=fs::OpenOptions::new().create_new(true).write(true).open(out.join("RECEPCION.jsonl"))?;
  let l=Arc::new(Mutex::new(Journal{file,seq:0,previous:String::new(),bytes:0}));
  for(c,f,b)in [("modelo_stdout","modelo.stdout",b"original\n".as_slice()),("modelo_stderr","modelo.stderr",b"".as_slice()),("mcp_stdout","mcp.stdout",b"respuesta\n".as_slice()),("mcp_stderr","mcp.stderr",b"".as_slice()),("telemetria","TELEMETRIA.jsonl",b"{}\n".as_slice())]{
   save(&out.join(f),b)?;if !b.is_empty(){log(&l,c,if c=="telemetria"{&b[..b.len()-1]}else{b})?;}
  }
  save(&out.join("mcp/diario.jsonl"),b"diario sintetico\n")?;
  let mut puerta=Puerta::default();let mut primera=Value::Null;
  for i in 0..7{
   let t=json!([i+1]);let v=json!({"id":sv_arbitro_comprobaciones::ciclo::IDS[i],"documento":sv_arbitro_comprobaciones::ciclo::DOCS[i],"seccion":"S1","tokens":t,"tokens_sha256":h(&serde_json::to_vec(&t)?),"max_salida":4096,"paginas":[0,1],"funciones":[],"mensajes":["sintetico"],"plantilla_efectiva":"sintetica"});
   puerta.preparar(&v,None)?;puerta.solicitar("preparacion",v["id"].as_str().unwrap(),v["tokens_sha256"].as_str().unwrap(),"contraste")?;if i==0{primera=v;}
  }
  let sha=primera["tokens_sha256"].as_str().unwrap();puerta.solicitar("carga","D01",sha,"contraste")?;puerta.motor(&primera)?;puerta.solicitar("generacion","D01",sha,"contraste")?;
  puerta.emision(&json!({"id":"D01","texto":"original","tokens":[1]}))?;puerta.terminar(&json!({"id":"D01","completo":true}))?;puerta.solicitar("adjudicacion","D01",sha,"contraste")?;
  Ok((out,l,puerta))
 }
 #[test]fn punto_restaura_originales_y_excluye_telemetria_aun_no_registrada()->R<()>{
  let(out,l,p)=preparar_punto()?;let mut f=fs::OpenOptions::new().append(true).open(out.join("TELEMETRIA.jsonl"))?;f.write_all(b"{\"posterior\":true}\n")?;f.sync_all()?;
  punto(&out,&l,&p)?;let dst=out.join("puntos/D01");
  assert_eq!(fs::read(dst.join("modelo.stdout"))?,b"original\n");assert_eq!(fs::read(dst.join("TELEMETRIA.jsonl"))?,b"{}\n");
  let sello:Value=serde_json::from_slice(&fs::read(dst.join("PUNTO.json"))?)?;assert_eq!(sello["id"],"D01");assert_eq!(sello["salida_sha256"],h(b"original"));assert_eq!(sello["adjudicacion_pendiente"],true);
  assert!(punto(&out,&l,&p).is_err());Ok(())
 }
 #[test]fn punto_rechaza_original_alterado()->R<()>{let(out,l,p)=preparar_punto()?;fs::write(out.join("modelo.stdout"),b"alterado\n")?;assert!(punto(&out,&l,&p).is_err());Ok(())}

 fn recorrido_sintetico()->R<(PathBuf,Log,Puerta)> {
  let(out,_,p)=preparar_punto()?;
  // Sustituir sólo la maqueta creada por esta prueba antes de sellarla.
  let file=fs::OpenOptions::new().write(true).truncate(true).open(out.join("RECEPCION.jsonl"))?;
  let l=Arc::new(Mutex::new(Journal{file,seq:0,previous:String::new(),bytes:0}));
  let req="{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\"}\n";let rep="{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{}}\n";
  let ts=json!([1]);let ev=vec![json!({"evento":"mcp_solicitud","wire":req}),json!({"evento":"mcp_recibido","wire":rep}),json!({"evento":"contexto_conductor","id":"D01","ronda":0,"tokens":ts,"tokens_sha256":h(&serde_json::to_vec(&ts)?)}),json!({"evento":"entrada_motor","tokens":ts,"truncamiento":false}),json!({"evento":"calculo_entrada","tokens":[ts]}),json!({"evento":"calculo_fin","correcto":true}),json!({"evento":"token","posicion":0,"token":1}),json!({"evento":"emision_integra","id":"D01","tokens":[1],"texto":"original"})];
  let output=ev.iter().map(|d|format!("SV_EVENT {}\n",json!({"datos":d}))).collect::<String>();
  fs::write(out.join("modelo.stdout"),output.as_bytes())?;log(&l,"modelo_stdout",output.as_bytes())?;
  for c in ["mcp_stdin"]{log(&l,c,req.as_bytes())?;}for c in ["mcp_stdout","modelo_stdin"]{log(&l,c,rep.as_bytes())?;}
  fs::write(out.join("mcp.stdout"),rep)?;fs::write(out.join("TELEMETRIA.jsonl"),b"{}\n")?;log(&l,"telemetria",b"{}")?;
  let mut prev=String::new();let mut diary=String::new();
  for (i,d) in [json!({"evento":"solicitud","bytes_hex":req.bytes().map(|b|format!("{b:02x}")).collect::<String>()}),json!({"evento":"resultado","bytes_hex":rep.bytes().map(|b|format!("{b:02x}")).collect::<String>(),"sha256":h(rep.as_bytes())}),json!({"evento":"entrega"})].into_iter().enumerate(){let mut v=json!({"secuencia":i,"anterior_sha256":prev,"datos":d});prev=h(&serde_json::to_vec(&v)?);v["sha256"]=json!(prev);diary.push_str(&format!("{v}\n"));}
  fs::write(out.join("mcp/diario.jsonl"),diary)?;Ok((out,l,p))
 }
 #[test]fn recorrido_integro_sello_recuperacion_informe_decision_y_cierre()->R<()> {
  let(out,l,mut p)=recorrido_sintetico()?;punto(&out,&l,&p)?;
  let src=out.join("puntos/D01");let copy=out.join("recuperacion");fs::create_dir(&copy)?;fs::create_dir(copy.join("mcp"))?;
  for n in ["PUNTO.json","RECEPCION.jsonl","modelo.stdout","modelo.stderr","mcp.stdout","mcp.stderr","TELEMETRIA.jsonl","mcp/diario.jsonl"]{fs::copy(src.join(n),copy.join(n))?;assert_eq!(fs::read(src.join(n))?,fs::read(copy.join(n))?);}
  let v=sv_arbitro_comprobaciones::auditoria::auditar(&copy,false,true).map_err(|e|e.to_string())?;
  assert_eq!(v["conforme"],true);assert_eq!(v["emisiones"],1);assert_eq!(v["sello_archivo"],"PUNTO.json");
  let report=out.join("INFORME.json");save(&report,&serde_json::to_vec(&v)?)?;assert_eq!(serde_json::from_slice::<Value>(&fs::read(&report)?)?,v);assert!(save(&report,b"falso").is_err());
  assert!(p.adjudicar(&json!({"id":"D01","salida_sha256":"falsa","accion":"continuar"})).is_err());
  p.adjudicar(&json!({"id":"D01","salida_sha256":h(b"original"),"accion":"continuar"}))?;assert_eq!(p.id(),"F01");
  // Caso siguiente sintético; el control final cierra sin otra admisión.
  let t=json!([2]);let entrada=json!({"id":"F01","documento":"F-A","seccion":"S1","tokens":t,"tokens_sha256":h(&serde_json::to_vec(&t)?),"max_salida":4096,"paginas":[0,1],"funciones":[],"mensajes":["sintetico"],"plantilla_efectiva":"sintetica"});
  p.motor(&entrada)?;p.solicitar("generacion","F01",entrada["tokens_sha256"].as_str().unwrap(),"contraste")?;p.emision(&json!({"id":"F01","texto":"segundo","tokens":[1]}))?;p.terminar(&json!({"id":"F01","completo":true}))?;p.solicitar("adjudicacion","F01",entrada["tokens_sha256"].as_str().unwrap(),"contraste")?;p.adjudicar(&json!({"id":"F01","salida_sha256":h(b"segundo"),"accion":"cerrar"}))?;assert!(p.cerrado());assert!(p.motor(&entrada).is_err());
  Ok(())
 }
 #[test]fn auditor_rechaza_datos_alterados_y_un_informe_favorable_falso()->R<()> {
  let(out,l,p)=recorrido_sintetico()?;punto(&out,&l,&p)?;let q=out.join("puntos/D01");save(&q.join("INFORME-FALSO.json"),br#"{"conforme":true}"#)?;
  fs::write(q.join("modelo.stdout"),b"alterado\n")?;assert!(sv_arbitro_comprobaciones::auditoria::auditar(&q,false,true).is_err());Ok(())
 }
}
