#![forbid(unsafe_code)]
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs::{self, File, OpenOptions}, io::Write, path::{Path, PathBuf}, sync::{Arc, Mutex, mpsc}, thread::{self, JoinHandle}, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};
pub type Result<T> = std::result::Result<T, String>;
pub fn sha(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
pub fn utc_ms() -> u128 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() }
fn err(e: impl std::fmt::Display) -> String {e.to_string()}
fn new_file(p: &Path) -> Result<File> {OpenOptions::new().write(true).create_new(true).open(p).map_err(err)}
fn store(p: &Path, v: &Value) -> Result<()> {let mut f=new_file(p)?; f.write_all(&serde_json::to_vec_pretty(v).map_err(err)?).and_then(|_| f.sync_all()).map_err(err)}

// Solo se conserva el proceso inscrito. Ni argumentos, ni entorno, ni usuarios.
pub struct Sampler { system: System, pid: Pid, identity: u64, count: u64 }
impl Sampler {
    pub fn own() -> Result<Self> {
        let mut s=Self {system:System::new(),pid:Pid::from_u32(std::process::id()),identity:0,count:0};
        s.refresh();
        s.identity=s.system.process(s.pid).ok_or("Proceso propio no observable")?.start_time();
        if s.identity==0 {return Err("Inicio del proceso no observable".into());} Ok(s)
    }
    fn refresh(&mut self) {self.system.refresh_processes_specifics(ProcessesToUpdate::Some(&[self.pid]),true,ProcessRefreshKind::nothing().without_tasks().with_cpu().with_memory().with_disk_usage());}
    pub fn sample(&mut self) -> Result<Value> {
        let started=Instant::now(); self.refresh();
        let p=self.system.process(self.pid).ok_or("Proceso ausente o inaccesible")?;
        if p.start_time()!=self.identity {return Err("Identidad de proceso discordante; posible reutilización de PID".into());}
        let io=p.disk_usage();
        let process=json!({"pid":p.pid().as_u32(),"ppid":p.parent().map(|v|v.as_u32()),"inicio_unix_s":self.identity,
            "nombre":p.name().to_string_lossy(),"cpu_porcentaje_nucleo":if self.count==0 {None}else{Some(p.cpu_usage())},
            "cpu_acumulada_ms":p.accumulated_cpu_time(),"rss_bytes":p.memory(),"virtual_bytes":p.virtual_memory(),
            "io_lectura_acumulada_bytes":io.total_read_bytes,"io_escritura_acumulada_bytes":io.total_written_bytes,
            "estado":p.status().to_string(),"hilos_so":null,"handles_so":null,
            "io_alcance":"Windows: E/S del proceso; no equivale a tráfico TCP ni exclusivamente disco"});
        let sockets=get_sockets_info(AddressFamilyFlags::IPV4|AddressFamilyFlags::IPV6,ProtocolFlags::TCP|ProtocolFlags::UDP).map_err(err)?;
        let mut rows=Vec::new();
        for s in sockets.into_iter().filter(|s|s.associated_pids.contains(&self.pid.as_u32())) {
            let row=match s.protocol_socket_info {
                ProtocolSocketInfo::Tcp(t)=>json!({"protocolo":"TCP","local":format!("{}:{}",t.local_addr,t.local_port),"remoto":format!("{}:{}",t.remote_addr,t.remote_port),"puerto_local":t.local_port,"puerto_remoto":t.remote_port,"estado":format!("{:?}",t.state),"pid":self.pid.as_u32()}),
                ProtocolSocketInfo::Udp(u)=>json!({"protocolo":"UDP","local":format!("{}:{}",u.local_addr,u.local_port),"puerto_local":u.local_port,"estado":"sin conexión","pid":self.pid.as_u32()})};
            rows.push(row);
        }
        rows.sort_by_key(Value::to_string); self.count+=1;
        Ok(json!({"proceso":process,"conexiones":rows,"duracion_captura_us":started.elapsed().as_micros(),"estado_medicion":"observado"}))
    }
}

struct Ledger {file:File, seq:u64, previous:String, start:Instant, samples:u64, failures:u64, max_gap:u128, last_sample:Option<u128>}
impl Ledger {
    fn append(&mut self,kind:&str,data:Value)->Result<()> {
        let elapsed=self.start.elapsed().as_millis();
        let body=serde_json::to_string(&json!({"secuencia":self.seq,"utc_unix_ms":utc_ms(),"transcurrido_ms":elapsed,"tipo":kind,"datos":data})).map_err(err)?;
        let digest=sha(format!("{}\n{}",self.previous,body).as_bytes());
        let line=serde_json::to_vec(&json!({"anterior_sha256":self.previous,"cuerpo":body,"sha256":digest})).map_err(err)?;
        self.file.write_all(&line).and_then(|_|self.file.write_all(b"\n")).and_then(|_|self.file.flush()).map_err(err)?;
        self.previous=digest;self.seq+=1;
        if kind=="muestra" {self.samples+=1; if let Some(p)=self.last_sample {self.max_gap=self.max_gap.max(elapsed-p);}self.last_sample=Some(elapsed);}
        if kind=="fallo_medicion" {self.failures+=1;}
        Ok(())
    }
}
pub struct Monitor {ledger:Arc<Mutex<Ledger>>, stop:Option<mpsc::Sender<()>>, join:Option<JoinHandle<Result<()>>>, dir:PathBuf}
impl Monitor {
    pub fn start(dir:&Path)->Result<Self> {
        fs::create_dir_all(dir).map_err(err)?;
        let mut sampler=Sampler::own()?; let initial=sampler.sample()?;
        let mut ledger=Ledger{file:new_file(&dir.join("telemetria.jsonl"))?,seq:0,previous:"0".repeat(64),start:Instant::now(),samples:0,failures:0,max_gap:0,last_sample:None};
        ledger.append("inicio",json!({"version":"sv-instrumentacion/0.1.0","pid":std::process::id(),"intervalo_objetivo_ms":250,"sistema":std::env::consts::OS,"arquitectura":std::env::consts::ARCH,"nucleos_logicos":thread::available_parallelism().map(|x|x.get()).ok(),"alcance":"únicamente proceso inscrito; tabla TCP/UDP filtrada por PID antes de conservar","fuentes":["sysinfo 0.39.6","netstat2 0.11.2","reloj monotónico Rust"],"sin_medicion":["recursos internos del proveedor","tiempo DNS y TLS separados","hilos y handles del SO","asignaciones del heap Rust","retransmisiones y RTT TCP"]}))?;
        ledger.append("muestra",initial)?;
        let ledger=Arc::new(Mutex::new(ledger));let shared=ledger.clone();let(tx,rx)=mpsc::channel();
        let join=thread::Builder::new().name("sv-observacion".into()).spawn(move||{
            let deadline=Instant::now()+Duration::from_secs(180);
            loop {
                match rx.recv_timeout(Duration::from_millis(250)) {Ok(())|Err(mpsc::RecvTimeoutError::Disconnected)=>break,Err(mpsc::RecvTimeoutError::Timeout)=>{}}
                let result=sampler.sample();let mut l=shared.lock().map_err(err)?;
                match result {Ok(v)=>l.append("muestra",v)?,Err(e)=>l.append("fallo_medicion",json!({"motivo":e}))?,}
                if Instant::now()>=deadline {l.append("fallo_medicion",json!({"motivo":"plazo máximo de observación alcanzado"}))?;break;}
            }
            Ok(())
        }).map_err(err)?;
        Ok(Self {ledger,stop:Some(tx),join:Some(join),dir:dir.to_owned()})
    }
    // Los datos proceden exclusivamente de campos permitidos; nunca cabeceras de autorización.
    pub fn event(&self,kind:&str,data:Value)->Result<()> {self.ledger.lock().map_err(err)?.append(kind,data)}
    pub fn healthy(&self)->Result<()> {let l=self.ledger.lock().map_err(err)?;if l.failures>0 || self.join.as_ref().is_some_and(|j|j.is_finished()) {Err("Instrumentación degradada".into())} else {Ok(())}}
    pub fn finish(mut self)->Result<Value> {
        self.stop.take().unwrap().send(()).map_err(err)?;
        self.join.take().unwrap().join().map_err(|_|"Fallo del hilo de observación")??;
        {let mut l=self.ledger.lock().map_err(err)?;l.append("cierre_observacion",json!({"estado":"concluida"}))?;l.file.sync_all().map_err(err)?;}
        let v=verify(&self.dir.join("telemetria.jsonl"))?;
        store(&self.dir.join("COTEJO-TELEMETRIA.json"),&v)?;Ok(v)
    }
}
impl Drop for Monitor {fn drop(&mut self){if let Some(tx)=self.stop.take(){let _=tx.send(());}if let Some(j)=self.join.take(){let _=j.join();}}}

pub fn records(path:&Path)->Result<Vec<Value>> {
    let bytes=fs::read(path).map_err(err)?; if !bytes.ends_with(b"\n"){return Err("Registro truncado".into());}
    let mut previous="0".repeat(64);let mut out=Vec::new();let mut last=0;
    for line in bytes.split(|b|*b==b'\n').filter(|b|!b.is_empty()) {
        let row:Value=serde_json::from_slice(line).map_err(err)?; let body=row["cuerpo"].as_str().ok_or("Falta cuerpo")?;
        let digest=sha(format!("{previous}\n{body}").as_bytes());
        if row["anterior_sha256"]!=previous || row["sha256"]!=digest {return Err("Cadena de integridad discordante".into());}
        let v:Value=serde_json::from_str(body).map_err(err)?;
        let t=v["transcurrido_ms"].as_u64().ok_or("Falta reloj monotónico")?;
        if v["secuencia"].as_u64()!=Some(out.len() as u64)||t<last {return Err("Secuencia o tiempo discordantes".into());}
        last=t;previous=digest;out.push(v);
    } Ok(out)
}
pub fn verify(path:&Path)->Result<Value> {
    let rows=records(path)?;
    if rows.first().map(|v|v["tipo"].as_str())!=Some(Some("inicio")) || rows.last().map(|v|v["tipo"].as_str())!=Some(Some("cierre_observacion")){return Err("Registro sin inicio/cierre".into());}
    let samples:Vec<_>=rows.iter().filter(|v|v["tipo"]=="muestra").collect();
    let fails=rows.iter().filter(|v|v["tipo"]=="fallo_medicion").count();
    let gap=samples.windows(2).map(|w|w[1]["transcurrido_ms"].as_u64().unwrap()-w[0]["transcurrido_ms"].as_u64().unwrap()).max();
    let bytes=fs::read(path).map_err(err)?;
    Ok(json!({"integridad":"conforme","registros":rows.len(),"muestras":samples.len(),"fallos_medicion":fails,"intervalo_maximo_ms":gap,"intervalo_objetivo_ms":250,"bytes":bytes.len(),"sha256":sha(&bytes),"plataforma_recibida":false,"precision":"cotejo de custodia local; no equivale a recepción integral del instrumento ni firma externa"}))
}
pub fn escape(s:&str)->String{s.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;").replace('"',"&quot;")}

pub fn dashboard(dir:&Path,report:&Value)->Result<String> {
    let rows=records(&dir.join("telemetria.jsonl"))?;
    let check=verify(&dir.join("telemetria.jsonl"))?;
    let samples=rows.iter().filter(|v|v["tipo"]=="muestra").collect::<Vec<_>>();
    let first=samples.first().ok_or("Sin muestras")?;let last=samples.last().unwrap();
    let proc=&first["datos"]["proceso"];
    let val=|v:&Value|if v.is_null(){"No comunicado".into()}else if let Some(s)=v.as_str(){escape(s)}else{escape(&v.to_string())};
    let peak=|key:&str|samples.iter().filter_map(|v|v["datos"]["proceso"][key].as_f64()).fold(0f64,f64::max);
    let delta=|key:&str|last["datos"]["proceso"][key].as_u64().unwrap_or(0).saturating_sub(proc[key].as_u64().unwrap_or(0));
    let cpu_ms=delta("cpu_acumulada_ms");
    let duration=last["transcurrido_ms"].as_u64().unwrap_or(0);
    let chart=|key:&str,label:&str,div:f64,color:&str| {
        let maximum=(peak(key)/div).max(0.01);let xmax=duration.max(1) as f64;
        let points=samples.iter().filter_map(|v|v["datos"]["proceso"][key].as_f64().map(|y|format!("{:.1},{:.1}",42.+v["transcurrido_ms"].as_f64().unwrap()/xmax*560.,148.-y/div/maximum*120.))).collect::<Vec<_>>().join(" ");
        format!("<figure><figcaption>{label} · máximo {maximum:.2}</figcaption><svg viewBox='0 0 640 180' role='img' aria-label='{label}'><path d='M42 20 V148 H602' fill='none' stroke='#657a98'/><polyline points='{points}' fill='none' stroke='{color}' stroke-width='2.5'/><text x='2' y='22'>{maximum:.1}</text><text x='18' y='150'>0</text><text x='42' y='170'>0 s</text><text x='530' y='170'>{:.2} s</text></svg></figure>",duration as f64/1000.)
    };
    let mut con=std::collections::BTreeMap::<String,(Value,u64,u64,usize)>::new();
    for s in &samples {let t=s["transcurrido_ms"].as_u64().unwrap();for v in s["datos"]["conexiones"].as_array().unwrap(){let k=v.to_string();let entry=con.entry(k).or_insert((v.clone(),t,t,0));entry.2=t;entry.3+=1;}}
    let conn_rows=con.values().map(|(v,from,to,n)|format!("<tr><td>{}</td><td>{}</td><td><code>{}</code></td><td><code>{}</code></td><td>{}</td><td>{:.3}–{:.3} s</td><td>{n}</td></tr>",val(&v["pid"]),val(&v["protocolo"]),val(&v["local"]),val(&v["remoto"]),val(&v["estado"]),*from as f64/1000.,*to as f64/1000.)).collect::<String>();
    let phases=rows.iter().filter(|v|v["tipo"]!="muestra").map(|v|format!("<tr><td>{:.3} s</td><td>{}</td><td><code>{}</code></td></tr>",v["transcurrido_ms"].as_f64().unwrap()/1000.,val(&v["tipo"]),escape(&v["datos"].to_string()))).collect::<String>();
    let raw_samples=samples.iter().map(|v|{let p=&v["datos"]["proceso"];format!("<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",val(&v["transcurrido_ms"]),val(&p["cpu_porcentaje_nucleo"]),val(&p["rss_bytes"]),val(&p["virtual_bytes"]),val(&p["cpu_acumulada_ms"]),v["datos"]["conexiones"].as_array().unwrap().len(),val(&v["datos"]["duracion_captura_us"]))}).collect::<String>();
    let footer="© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
    Ok(format!(r#"<!doctype html><html lang="es"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>SV · Astra · Instrumentación de la segunda prueba</title><style>
:root{{color-scheme:dark}}body{{background:#0c1523;color:#e0e9f5;font:15px/1.55 system-ui,sans-serif;margin:0}}main{{max-width:1200px;margin:auto;padding:32px 24px}}h1{{font-size:32px;line-height:1.2}}h2{{font-size:21px;margin-top:32px}}p{{max-width:100ch}}.eyebrow{{color:#87c3ff;letter-spacing:.13em;font-size:12px}}.status{{background:#182b40;border-left:4px solid #76b7fa;padding:14px 18px}}.grid{{display:grid;grid-template-columns:repeat(auto-fit,minmax(185px,1fr));gap:14px}}.card,figure{{background:#142237;border:1px solid #2a3e57;border-radius:10px;padding:17px;margin:0}}.card strong{{display:block;font-size:27px;color:#98ded5}}.muted{{color:#aabdd2;font-size:13px}}.plots{{display:grid;grid-template-columns:repeat(auto-fit,minmax(300px,1fr));gap:14px}}svg{{width:100%}}svg text{{font:12px system-ui;fill:#b2c4dd}}table{{width:100%;border-collapse:collapse;font-size:13px}}th,td{{padding:10px;text-align:left;border-bottom:1px solid #2a3e57;vertical-align:top}}th{{color:#a8c9ec}}code{{overflow-wrap:anywhere;word-break:break-word}}.scroll{{overflow:auto}}pre{{white-space:pre-wrap;overflow-wrap:anywhere}}details{{background:#142237;padding:14px;margin:14px 0;border-radius:8px}}summary{{cursor:pointer;color:#9bc8f6}}footer{{border-top:1px solid #2a3e57;margin-top:36px;padding-top:18px;font-size:12px;color:#9fb1c7}}.warning{{color:#edce93}}</style></head><body><main>
<p class="eyebrow">SISTEMA VECTORIAL SV · NODO 03 · OPENAI</p><h1>GPT-6 Astra<br>Instrumentación de la segunda prueba</h1>
<p class="status"><strong>Estado del resultado: {state}.</strong> Observación concluida. Esta pantalla conserva mediciones de la ejecución; no representa actividad actual ni una inferencia todavía abierta.</p>
<div class="grid"><div class="card">Duración de la solicitud<strong>{request_ms} ms</strong><span class="muted">Reloj monotónico Rust</span></div><div class="card">Primer texto recibido<strong>{first_text} ms</strong><span class="muted">Recepción, no tiempo interno del modelo</span></div><div class="card">Muestras del proceso<strong>{sample_count}</strong><span class="muted">Intervalo objetivo 250 ms</span></div><div class="card">Integridad del registro<strong>{integrity}</strong><span class="muted">{faults} fallos de medición · SHA-256</span></div></div>
<h2>Proceso observado y recursos locales</h2><p><strong>{process_name}</strong> · PID {pid} · padre {ppid} · inicio Unix {process_start} s. Observación de {duration_s:.3} s. El proceso incluye cliente, receptor local e instrumentación; su consumo no es consumo del modelo remoto.</p>
<div class="grid"><div class="card">Memoria residente máxima<strong>{rss:.2} MiB</strong></div><div class="card">CPU acumulada en la ventana<strong>{cpu_ms} ms</strong></div><div class="card">E/S leída en la ventana<strong>{read_bytes} B</strong></div><div class="card">E/S escrita en la ventana<strong>{written_bytes} B</strong></div></div><p class="muted">En Windows, E/S comprende operaciones del proceso y no sólo disco; no es un contador de bytes TCP. CPU: porcentaje equivalente a un núcleo; puede superar 100 %.</p>
<div class="plots">{cpu_chart}{rss_chart}</div>
<h2>TCP, UDP y puertos observados</h2><p>Extremo remoto comunicado por el transporte HTTP: <code>{remote}</code> · {http_version}. Se muestran estados observados dentro del intervalo, no un inventario actual de puertos abiertos.</p><div class="scroll"><table><thead><tr><th>PID</th><th>Protocolo</th><th>Extremo local</th><th>Extremo remoto</th><th>Estado</th><th>Primera–última observación</th><th>Muestras</th></tr></thead><tbody>{conn_rows}</tbody></table></div><p class="muted">LISTEN/Listen identifica escucha local; ESTABLISHED/Established identifica conexión establecida. El muestreo puede omitir conexiones efímeras. Las conexiones del mismo proceso no se atribuyen todas a una única petición.</p>
<h2>Respuesta y telemetría del proveedor</h2><blockquote>{answer}</blockquote><div class="grid"><div class="card">Entrada<strong>{input} tokens</strong></div><div class="card">Salida<strong>{output} tokens</strong></div><div class="card">Total<strong>{total} tokens</strong></div><div class="card">Eventos SSE<strong>{events}</strong></div></div><p>Modelo declarado: <code>{model}</code> · cierre: <code>{terminal}</code> · HTTP {http}. Datos de aplicación recibidos: {response_bytes} bytes.</p><details><summary>Uso completo y cabeceras permitidas recibidas</summary><pre>{usage}</pre><pre>{headers}</pre></details>
<h2>Traza de ejecución Rust</h2><p>Las fases, lecturas y eventos comparten reloj monotónico y secuencia con las muestras. No se guardan credenciales en la traza.</p><details open><summary>Fases y recepción de eventos</summary><div class="scroll"><table><thead><tr><th>Desde inicio</th><th>Suceso</th><th>Datos</th></tr></thead><tbody>{phases}</tbody></table></div></details>
<details><summary>Muestras completas del proceso ({sample_count})</summary><div class="scroll"><table><thead><tr><th>ms</th><th>CPU %</th><th>RSS B</th><th>Virtual B</th><th>CPU acumulada ms</th><th>Conexiones</th><th>Captura μs</th></tr></thead><tbody>{raw_samples}</tbody></table></div></details>
<h2>Calidad de la medición y límites</h2><p>Intervalo máximo observado: {max_gap} ms. Huella del registro:<br><code>{digest}</code></p><p class="warning">No medidos: CPU/GPU/memoria internos de OpenAI, descomposición DNS/TLS, RTT y retransmisiones TCP, hilos/handles del sistema y asignaciones del heap Rust. Créditos descontados y precio liquidado: no comunicados.</p><p>La cadena SHA-256 acredita cotejo local de los bytes; no es una firma ni una marca temporal externa. La criptografía C/ensamblador sigue pendiente conforme a la excepción autorizada. Las bibliotecas Rust de observación usan las API nativas de Windows.</p><p><strong>Recepción integral del instrumento y admisión científica: pendientes.</strong> Los datos de esta segunda prueba no reconstruyen la instrumentación ausente de la primera.</p><footer>{footer}</footer></main></body></html>"#,
        state=val(&report["estado"]), request_ms=val(&report["duracion_ms"]),first_text=val(&report["primer_texto_ms"]),sample_count=samples.len(),integrity=val(&check["integridad"]),faults=val(&check["fallos_medicion"]),process_name=val(&proc["nombre"]),pid=val(&proc["pid"]),ppid=val(&proc["ppid"]),process_start=val(&proc["inicio_unix_s"]),duration_s=duration as f64/1000.,rss=peak("rss_bytes")/1048576.,read_bytes=delta("io_lectura_acumulada_bytes"),written_bytes=delta("io_escritura_acumulada_bytes"),cpu_chart=chart("cpu_porcentaje_nucleo","CPU local (%)",1.,"#8bccff"),rss_chart=chart("rss_bytes","Memoria residente (MiB)",1048576.,"#82d3bc"),remote=val(&report["par_remoto_http"]),http_version=val(&report["version_http"]),answer=val(&report["respuesta_texto"]),input=val(&report["uso_proveedor"]["input_tokens"]),output=val(&report["uso_proveedor"]["output_tokens"]),total=val(&report["uso_proveedor"]["total_tokens"]),events=val(&report["eventos"]),model=val(&report["modelo_declarado"]),terminal=val(&report["evento_terminal"]),http=val(&report["http"]),response_bytes=val(&report["respuesta_bytes"]),usage=escape(&serde_json::to_string_pretty(&report["uso_proveedor"]).map_err(err)?),headers=escape(&serde_json::to_string_pretty(&report["cabeceras_proveedor"]).map_err(err)?),max_gap=val(&check["intervalo_maximo_ms"]),digest=val(&check["sha256"])))
}

#[cfg(test)] mod tests {
    use super::*;use std::net::{TcpListener,TcpStream};
    #[test]fn proceso_y_tcp_reales(){let mut s=Sampler::own().unwrap();let listener=TcpListener::bind("127.0.0.1:0").unwrap();let port=listener.local_addr().unwrap().port();let c=TcpStream::connect(listener.local_addr().unwrap()).unwrap();let(peer,_)=listener.accept().unwrap();thread::sleep(Duration::from_millis(300));let v=s.sample().unwrap();assert_eq!(v["proceso"]["pid"],std::process::id());let sockets=v["conexiones"].as_array().unwrap();assert!(sockets.iter().any(|v|v["puerto_local"]==port && v["estado"]=="Listen"));assert!(sockets.iter().any(|v|v["puerto_remoto"]==port && v["estado"]=="Established"));drop(c);drop(peer);drop(listener);thread::sleep(Duration::from_millis(300));let v=s.sample().unwrap();assert!(!v["conexiones"].as_array().unwrap().iter().any(|v|v["puerto_local"]==port && v["estado"]=="Listen"));}
    #[test]fn identidad_no_reutilizable(){let mut s=Sampler::own().unwrap();s.identity+=1;assert!(s.sample().is_err());}
    #[test]fn escapar_html(){assert_eq!(escape("<script>&"),"&lt;script&gt;&amp;");}
    #[test]fn registro_tamper_y_truncado(){
        let root=Path::new(env!("CARGO_MANIFEST_DIR")).join("evidencia-pruebas").join(format!("{}-{}",std::process::id(),utc_ms()));fs::create_dir_all(&root).unwrap();
        let monitor=Monitor::start(&root).unwrap();monitor.event("prueba_local",json!({"inferencia":false})).unwrap();thread::sleep(Duration::from_millis(650));let v=monitor.finish().unwrap();assert!(v["muestras"].as_u64().unwrap()>=3);
        let path=root.join("telemetria.jsonl");let bytes=fs::read(&path).unwrap();let mut altered=bytes.clone();let pos=altered.iter().position(|b|*b==b'0').unwrap();altered[pos]=b'1';fs::write(root.join("alterado.jsonl"),altered).unwrap();assert!(verify(&root.join("alterado.jsonl")).is_err());fs::write(root.join("truncado.jsonl"),&bytes[..bytes.len()-1]).unwrap();assert!(verify(&root.join("truncado.jsonl")).is_err());
    }
}
