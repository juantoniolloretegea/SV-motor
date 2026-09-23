//! Control instrumental Linux. Adaptación de OwnedChild, Instant y cierre final de Qwen 0.1.3.
//! No constituye la guarda exterior ni una cuota de memoria del conjunto.
pub mod resources;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs::{self, File, OpenOptions}, io::{self, Read, Write}, net::{SocketAddr, TcpListener, TcpStream}, os::unix::{fs::OpenOptionsExt, process::{CommandExt, ExitStatusExt}}, path::{Path, PathBuf}, process::{Child, Command, ExitStatus, Stdio}, sync::{Arc, atomic::{AtomicBool, AtomicI32, Ordering}, mpsc::{self, SyncSender}}, thread, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};
type Result<T> = std::result::Result<T, String>;
const TICK: Duration = Duration::from_millis(50);
const ACK: Duration = Duration::from_secs(2);
const MAX_HTTP: usize = 256 * 1024;
pub const LICENSE_NOTICE: &str = include_str!("../AVISO_LICENCIAS.json");
pub const LICENSE_FOOTER: &str = "Sistema Vectorial SV · CC BY-NC-ND 4.0 · https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es\nComponentes de terceros: licencias propias; consulte AVISO_LICENCIAS.json y DEPENDENCIAS.json.";
static STOP: AtomicI32 = AtomicI32::new(0);
extern "C" fn signal_received(sig: i32) { let _ = STOP.compare_exchange(0, sig, Ordering::SeqCst, Ordering::SeqCst); }
pub struct Signals { old: Vec<(i32, libc::sigaction)> }
impl Signals {
    pub fn install() -> Result<Self> {
        STOP.store(0, Ordering::SeqCst);
        let mut guard = Self { old: Vec::new() };
        for sig in [libc::SIGTERM, libc::SIGINT] {
            // Linux: el manejador solo escribe un atómico; no asigna ni realiza E/S.
            let mut action: libc::sigaction = unsafe { std::mem::zeroed() };
            let mut old: libc::sigaction = unsafe { std::mem::zeroed() };
            action.sa_sigaction = signal_received as *const () as usize;
            unsafe { libc::sigemptyset(&mut action.sa_mask); }
            if unsafe { libc::sigaction(sig, &action, &mut old) } != 0 { return Err(io::Error::last_os_error().to_string()); }
            guard.old.push((sig, old));
        }
        Ok(guard)
    }
}
impl Drop for Signals { fn drop(&mut self) { for (sig, old) in &self.old { unsafe { libc::sigaction(*sig, old, std::ptr::null_mut()); } } } }

pub fn hash_file(path: &Path) -> Result<String> {
    let mut f = File::open(path).map_err(|e| e.to_string())?;
    let mut hash = Sha256::new(); let mut buffer = [0u8; 65536];
    loop { let n = f.read(&mut buffer).map_err(|e|e.to_string())?; if n == 0 { break; } hash.update(&buffer[..n]); }
    Ok(format!("{:x}", hash.finalize()))
}
fn bounded(path: impl AsRef<Path>, limit: usize) -> Result<String> {
    let path=path.as_ref();
    let mut text = String::new(); File::open(path).map_err(|e| format!("abrir {}: {e}",path.display()))?.take(limit as u64 + 1).read_to_string(&mut text).map_err(|e| format!("leer {}: {e}",path.display()))?;
    if text.len() > limit { return Err("lectura_excedida".into()); } Ok(text)
}
// Procede del observador Qwen: PID junto con start_ticks evita confundir identidades.
fn identity(pid: u32) -> Result<u64> {
    let stat = bounded(format!("/proc/{pid}/stat"), 8192)?;
    stat.rsplit_once(") ").and_then(|(_, fields)| fields.split_whitespace().nth(19)).ok_or("stat_incompleto")?.parse().map_err(|_| "start_ticks_invalido".into())
}
fn utc_ms() -> Option<u128> { SystemTime::now().duration_since(UNIX_EPOCH).ok().map(|v| v.as_millis()) }

type Message = (Value, mpsc::Sender<Result<()>>);
#[derive(Clone)]
struct Journal { tx: SyncSender<Message>, healthy: Arc<AtomicBool>, origin: Instant }
impl Journal {
    fn with_writer<W: Write + Send + 'static>(mut writer: W) -> Self {
        let (tx, rx) = mpsc::sync_channel::<Message>(32);
        let healthy = Arc::new(AtomicBool::new(true)); let state = healthy.clone();
        thread::spawn(move || {
            while let Ok((value, ack)) = rx.recv() {
                let result = (|| -> io::Result<()> { serde_json::to_writer(&mut writer, &value)?; writer.write_all(b"\n")?; writer.flush() })().map_err(|e| e.to_string());
                if result.is_err() { state.store(false, Ordering::SeqCst); }
                let _ = ack.send(result);
            }
        });
        Self { tx, healthy, origin: Instant::now() }
    }
    fn event(&self, kind: &str, data: Value) -> Result<()> {
        if !self.healthy.load(Ordering::SeqCst) {return Err("custodia_previamente_fallida".into());}
        let value = json!({"schema":"EIO-CONTROLADOR-OSS-1", "kind":kind, "utc_ms":utc_ms(), "elapsed_ms":self.origin.elapsed().as_millis(), "controller_pid":std::process::id(), "data":data});
        let (tx, rx) = mpsc::channel();
        let result = self.tx.try_send((value, tx)).map_err(|e|e.to_string()).and_then(|_| rx.recv_timeout(ACK).map_err(|e| e.to_string())?);
        if result.is_err() { self.healthy.store(false, Ordering::SeqCst); }
        result
    }
}
struct Durable(File);
impl Write for Durable {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> { self.0.write(b) }
    fn flush(&mut self) -> io::Result<()> { self.0.sync_all() }
}

struct OwnedChild { child: Child, status: Option<ExitStatus>, journal: Journal, label: &'static str, errors: Vec<String> }
impl OwnedChild {
    fn poll(&mut self) -> Result<Option<ExitStatus>> {
        if self.status.is_none() { self.status = self.child.try_wait().map_err(|e| e.to_string())?; }
        Ok(self.status)
    }
    fn send(&mut self, signal: i32, reason: &str) -> Result<()> {
        match self.poll() {
            Ok(Some(_)) => return Ok(()), Ok(None) => {},
            Err(e) => {self.errors.push(format!("consulta_antes_de_senal: {e}"));return Err(e);}
        }
        let pid = self.child.id();
        let before = self.journal.event("senal_solicitada", json!({"target_pid":pid,"target_pgid":pid,"signal":signal,"reason":reason,"label":self.label}));
        if let Err(e) = before { self.errors.push(format!("registro_previo: {e}")); }
        // El PID aún es hijo sin recoger: no puede reutilizarse mientras continúa vivo.
        let pgid = unsafe { libc::getpgid(pid as i32) };
        let (target, scope) = if pgid == pid as i32 { (-(pid as i32), "grupo_creado") } else { (pid as i32, "hijo_directo_grupo_no_confirmado") };
        let ret = unsafe { libc::kill(target, signal) };
        let error = if ret != 0 { Some(io::Error::last_os_error().to_string()) } else { None };
        if let Err(e) = self.journal.event("senal_resultado", json!({"target_pid":pid,"actual_target":target,"scope":scope,"signal":signal,"return":ret,"error":error})) { self.errors.push(format!("registro_posterior: {e}")); }
        if let Some(e) = error { self.errors.push(e.clone()); return Err(e); }
        Ok(())
    }
    fn await_exit(&mut self, duration: Duration) -> bool {
        let end = Instant::now() + duration;
        loop { match self.poll() { Ok(Some(_)) => return true, Ok(None) => {}, Err(e) => { self.errors.push(e); return false; } }
            if Instant::now() >= end { return false; } thread::sleep(Duration::from_millis(10)); }
    }
    fn stop(&mut self, reason: &str) -> bool {
        match self.poll() {Ok(Some(_))=>return true,Ok(None)=>{},Err(e)=>self.errors.push(format!("consulta_antes_de_parada: {e}"))}
        let _ = self.send(libc::SIGTERM, reason); // El resultado se conserva en errors y en el registro.
        if self.await_exit(Duration::from_millis(500)) { return true; }
        let _ = self.send(libc::SIGKILL, reason);
        self.await_exit(Duration::from_millis(500))
    }
}
impl Drop for OwnedChild {
    fn drop(&mut self) {
        if self.status.is_none() { let confirmed = self.stop("salida_de_ambito"); let _ = self.journal.event("limpieza_de_ambito", json!({"pid":self.child.id(),"confirmed":confirmed,"errors":self.errors})); }
    }
}

pub struct Config {
    pub installation: PathBuf, pub evidence: PathBuf, pub window: Duration,
    pub load: Duration, pub request: Duration, pub address: SocketAddr,
    pub expected_engine_sha256: String,
    pub virtual_limit: Option<u64>, pub minimum_memory: u64, pub reserve_memory: u64,
    #[cfg(test)] args: Option<Vec<String>>,
    #[cfg(test)] env: Vec<(String, String)>,
}
impl Config {
    pub fn new(installation: PathBuf, evidence: PathBuf, engine_hash: String) -> Self {
        Self { installation, evidence, window: Duration::from_secs(1000), load: Duration::from_secs(600), request: Duration::from_secs(300), address: SocketAddr::from(([127,0,0,1],8089)), expected_engine_sha256: engine_hash, virtual_limit:None, minimum_memory:13_760_462_848, reserve_memory:1024*1024*1024,
        #[cfg(test)] args:None, #[cfg(test)] env:Vec::new() }
    }
    fn validate(&self) -> Result<()> {
        if !self.virtual_limit.is_some_and(|v|v>=self.minimum_memory&&v<=64*1024*1024*1024){return Err("limite_virtual_explicito_requerido_hasta_64_GiB".into());}
        if self.window.is_zero() || self.window > Duration::from_secs(1500) || self.load.is_zero() || self.request.is_zero() || self.load > self.window || self.request > self.window { return Err("presupuesto_temporal_invalido".into()); }
        if self.address.ip() != std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST) || self.address.port() == 0 { return Err("direccion_no_admitida".into()); } Ok(())
    }
}
#[derive(Default)]
struct Metrics { peak: Option<u64>, samples: u64, missing: u64, last:Option<u64>, persisted:Option<Instant>, rss_ceiling:u64, reserve:u64, anon:Option<u64>, file:Option<u64>, shmem:Option<u64>, capacity:Value }
impl Metrics {
    fn sample(&mut self, pid: u32) {
        let status=bounded(format!("/proc/{pid}/status"),32768).unwrap_or_default();
        let get=|key:&str|status.lines().find_map(|l|l.strip_prefix(key)?.split_whitespace().next()?.parse::<u64>().ok()).and_then(|n|n.checked_mul(1024));
        let value=get("VmRSS:"); self.anon=get("RssAnon:"); self.file=get("RssFile:"); self.shmem=get("RssShmem:");
        self.last=value;
        if let Some(n) = value { self.peak = Some(self.peak.unwrap_or(0).max(n)); self.samples += 1; } else { self.missing += 1; }
    }
}
fn check(child: &mut OwnedChild, journal: &Journal, metrics: &mut Metrics, end: Instant) -> Result<()> {
    metrics.sample(child.child.id());
    if let Some(s) = child.poll()? { return Err(format!("terminacion_anticipada: {s}")); }
    // La RSS incorpora páginas respaldadas por archivos. No se declara que toda
    // RssFile sea recuperable: la disponibilidad global sigue siendo vinculante.
    let non_file=metrics.anon.zip(metrics.shmem).and_then(|(a,s)|a.checked_add(s));
    let capacity=resources::snapshot()?; metrics.capacity=capacity.clone();
    let exhausted=capacity["effective_available_bytes"].as_u64().is_none_or(|v|v<metrics.reserve);
    let exceeded=non_file.is_some_and(|v|v>metrics.rss_ceiling);
    if exhausted || exceeded || metrics.persisted.is_none_or(|t|t.elapsed()>=Duration::from_secs(1)) {
        journal.event("muestra_recursos",json!({"pid":child.child.id(),"rss_bytes":metrics.last,"rss_anon_bytes":metrics.anon,"rss_file_bytes":metrics.file,"rss_shmem_bytes":metrics.shmem,"non_file_rss_bytes":non_file,"peak_rss_bytes":metrics.peak,"samples":metrics.samples,"unavailable":metrics.missing,"capacity":capacity}))?;
        metrics.persisted=Some(Instant::now());
    }
    if exhausted{return Err("reserva_del_entorno_agotada".into());}
    if exceeded{return Err("limite_rss_anonima_y_compartida".into());}
    let signal = STOP.load(Ordering::SeqCst); if signal != 0 { return Err(format!("senal_recibida_controlador: {signal}")); }
    if !journal.healthy.load(Ordering::SeqCst) { return Err("custodia_no_integra".into()); }
    if Instant::now() >= end { return Err("limite_temporal".into()); } Ok(())
}
// Derivado de la atribución de sockets del observador Qwen: inode del descriptor
// y fila TCP LISTEN, no basta con que cualquier proceso acepte una conexión.
fn owns_listener(pid: u32, start: u64, port: u16) -> Result<bool> {
    if identity(pid)? != start { return Err("identidad_proceso_distinta".into()); }
    let mut inodes = std::collections::HashSet::new();
    for entry in fs::read_dir(format!("/proc/{pid}/fd")).map_err(|e|format!("enumerar /proc/{pid}/fd: {e}"))?.take(512) {
        if let Ok(link) = entry.and_then(|e|fs::read_link(e.path())) { if let Some(inode) = link.to_string_lossy().strip_prefix("socket:[").and_then(|v|v.strip_suffix(']')).and_then(|v|v.parse::<u64>().ok()) { inodes.insert(inode); } }
    }
    let table = bounded(format!("/proc/{pid}/net/tcp"), 262144)?;
    let owned = table.lines().skip(1).any(|line| { let f: Vec<_> = line.split_whitespace().collect(); f.get(3) == Some(&"0A") && f.get(1).and_then(|v|v.split_once(':')).and_then(|(_,p)|u16::from_str_radix(p,16).ok()) == Some(port) && f.get(9).and_then(|v|v.parse::<u64>().ok()).is_some_and(|v|inodes.contains(&v)) });
    if identity(pid)? != start { return Err("identidad_proceso_cambio".into()); } Ok(owned)
}
fn http(child: &mut OwnedChild, journal: &Journal, metrics: &mut Metrics, end: Instant, address: SocketAddr, method: &str, path: &str, body: &str) -> Result<Value> {
    check(child,journal,metrics,end)?;
    let mut stream = TcpStream::connect_timeout(&address,TICK).map_err(|e|e.to_string())?;
    stream.set_read_timeout(Some(TICK)).map_err(|e|e.to_string())?; stream.set_write_timeout(Some(TICK)).map_err(|e|e.to_string())?;
    write!(stream,"{method} {path} HTTP/1.0\r\nHost: localhost\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",body.len()).map_err(|e|e.to_string())?;
    let mut all = Vec::new(); let mut buffer = [0u8;8192];
    loop { check(child,journal,metrics,end)?; match stream.read(&mut buffer) {
        Ok(0) => break, Ok(n) => { if all.len()+n>MAX_HTTP {return Err("respuesta_http_excedida".into());} all.extend_from_slice(&buffer[..n]); },
        Err(e) if matches!(e.kind(),io::ErrorKind::WouldBlock|io::ErrorKind::TimedOut|io::ErrorKind::Interrupted) => {}, Err(e) => return Err(e.to_string()),
    } }
    let text = String::from_utf8(all).map_err(|e|e.to_string())?;
    journal.event("http_respuesta",json!({"method":method,"path":path,"raw":text}))?;
    let (head,body) = text.split_once("\r\n\r\n").ok_or("http_incompleto")?;
    if head.lines().next().and_then(|s|s.split_whitespace().nth(1)) != Some("200") {return Err(format!("http_no_200: {}",head.lines().next().unwrap_or("sin_estado")));}
    serde_json::from_str(body).map_err(|e|format!("json_invalido: {e}"))
}

fn select_loaded_gguf(models: &Value) -> Result<&str> {
    let data = models["data"].as_array().ok_or("lista_modelos_invalida")?;
    let real: Vec<_> = data.iter().filter(|v| v["id"] != "default").collect();
    if real.len() != 1 { return Err("lista_modelos_no_univoca".into()); }
    let model = real[0];
    if model["id"] != "gguf" || model["root"] != "gguf" || model["object"] != "model" || model["status"] != "loaded" || !model["parent"].is_null() {
        return Err("modelo_gguf_cargado_no_acreditado".into());
    }
    Ok("gguf")
}

pub fn execute(config: Config) -> Result<Value> {
    config.validate()?;
    let root = fs::canonicalize(&config.installation).map_err(|e|e.to_string())?;
    let lock = OpenOptions::new().create(true).truncate(false).read(true).write(true).mode(0o600).open(root.join("controlador.lock")).map_err(|e|e.to_string())?;
    lock.try_lock().map_err(|e|format!("instancia_ya_activa: {e}"))?;
    fs::create_dir(&config.evidence).map_err(|e|format!("directorio_evidencias_debe_ser_nuevo: {e}"))?;
    let evidence = fs::canonicalize(&config.evidence).map_err(|e|e.to_string())?;
    let file = OpenOptions::new().write(true).create_new(true).mode(0o600).open(evidence.join("SUCESOS.jsonl")).map_err(|e|e.to_string())?;
    run(config, root, evidence, Journal::with_writer(Durable(file)))
}
fn run(config: Config, root: PathBuf, evidence: PathBuf, journal: Journal) -> Result<Value> {
    let origin = Instant::now(); let end = origin + config.window;
    let mut child: Option<OwnedChild> = None; let mut metrics = Metrics::default(); let mut response = None;
    let mut phase = "preparacion"; let mut request_sent = false; let mut start_ticks = None;
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<()> {
        let capacity=resources::snapshot()?;
        journal.event("capacidad_previa",capacity.clone())?;
        metrics.rss_ceiling=resources::admit(&capacity,config.minimum_memory,config.reserve_memory)?;
        metrics.reserve=config.reserve_memory;
        journal.event("limites_declarados",json!({"virtual_hard_bytes":config.virtual_limit,"non_file_rss_sampled_stop_bytes":metrics.rss_ceiling,"environment_reserve_bytes":config.reserve_memory,"minimum_load_estimate_bytes":config.minimum_memory,"kernel_parent_death_signal":9,"resource_sample_period_ms":50,"scope":"Direcciones virtuales por proceso, RssAnon + RssShmem y disponibilidad global. RSS total descriptiva; no cuota agregada ni guarda exterior."}))?;
        let engine = root.join("motor/mistralrs"); let engine_hash = hash_file(&engine)?;
        if engine_hash != config.expected_engine_sha256 { return Err("identidad_motor_no_conforme".into()); }
        let available = TcpListener::bind(config.address).map_err(|e|format!("puerto_ocupado: {e}"))?;
        journal.event("inicio", json!({"controller_binary_sha256":hash_file(&std::env::current_exe().map_err(|e|e.to_string())?)?,"source_resources_sha256":format!("{:x}",Sha256::digest(include_bytes!("resources.rs"))),"source_lib_sha256":format!("{:x}",Sha256::digest(include_bytes!("lib.rs"))),"source_main_sha256":format!("{:x}",Sha256::digest(include_bytes!("main.rs"))),"lock_sha256":format!("{:x}",Sha256::digest(include_bytes!("../Cargo.lock"))),"engine_sha256":engine_hash,"window_ms":config.window.as_millis(),"load_ms":config.load.as_millis(),"request_ms":config.request.as_millis(),"address":config.address.to_string()}))?;
        if STOP.load(Ordering::SeqCst)!=0 || Instant::now()>=end {return Err("cancelacion_o_plazo_antes_del_arranque".into());}
        let log = OpenOptions::new().write(true).create_new(true).mode(0o600).open(evidence.join("motor.log")).map_err(|e|e.to_string())?;
        let port = config.address.port().to_string();
        let mut command = Command::new(&engine);
        let gguf = root.join("gguf/gpt-oss-20b-MXFP4.gguf");
        let gguf_hash = hash_file(&gguf)?;
        if gguf_hash != "27cd6c432c7672cb812a92f611cf3ba7bbc35928262bb1e1253ff4ee6ae35901" { return Err("identidad_gguf_no_conforme".into()); }
        journal.event("configuracion_carga",json!({"format":"GGUF","artifact_sha256":gguf_hash,"artifact_bytes":fs::metadata(&gguf).map_err(|e|e.to_string())?.len(),"source":"ggml-org/gpt-oss-20b-GGUF","revision":"b97cbb20d1995efd41dce8c4dd1ddf86e8db375b","isq":null,"topology":null,"minimum_admission_bytes":config.minimum_memory,"note":"Se conserva la cota conservadora previa de admisión; no es una medición de carga GGUF."}))?;
        journal.event("asignacion_explicita",json!({"device_layers":"0","host_layers":"todas","reason":"Asignación CPU explícita admitida por el motor; sustituye el estimador automático, sin retirar las cotas y observaciones del controlador. No acredita suficiencia de memoria."}))?;
        command.args(["--token-source","none","serve","-f","gguf/gpt-oss-20b-MXFP4.gguf","--tok-model-id","modelo","--cpu","--device-layers","0","--dtype","bf16","--max-seq-len","1024","--max-seqs","1","--prefix-cache-n","0","--paged-attn","off","--host","127.0.0.1","--port",&port,"--no-ui"]);
        #[cfg(test)] if let Some(args) = &config.args { command = Command::new(&engine); command.args(args); }
        #[cfg(test)] command.envs(config.env.iter().cloned());
        command.current_dir(&root).env("TIKTOKEN_ENCODINGS_BASE",root.join("harmony")).env("HF_HUB_OFFLINE","1").env_remove("MCP_CONFIG_PATH").stdin(Stdio::null()).stdout(log.try_clone().map_err(|e|e.to_string())?).stderr(log).process_group(0);
        resources::harden(&mut command,config.virtual_limit.ok_or("limite_virtual_ausente")?);
        drop(available);
        child = Some(OwnedChild { child:command.spawn().map_err(|e|e.to_string())?, status:None, journal:journal.clone(), label:"inferencia", errors:Vec::new() });
        let worker = child.as_mut().ok_or("hijo_ausente")?; let pid = worker.child.id();
        start_ticks = Some(identity(pid)?);
        journal.event("proceso_creado",json!({"pid":pid,"pgid":pid,"start_ticks":start_ticks}))?;
        phase = "carga"; let load_end = (Instant::now()+config.load).min(end);
        let mut observation_error_since=None;
        loop { check(worker,&journal,&mut metrics,load_end)?;
            match owns_listener(pid,start_ticks.ok_or("identidad_ausente")?,config.address.port()) {
                Ok(true)=>break,
                Ok(false)=>observation_error_since=None,
                Err(error)=>{
                    if observation_error_since.is_none(){
                        journal.event("observacion_puerto_fallida",json!({"error":error,"pid":pid,"status":bounded(format!("/proc/{pid}/status"),32768).ok(),"stat":bounded(format!("/proc/{pid}/stat"),8192).ok(),"capacity":resources::snapshot().ok(),"recheck_limit_ms":250}))?;
                        observation_error_since=Some(Instant::now());
                    }
                    // Reconsulta acotada: una salida concurrente no se atribuye a permisos.
                    // No se admite ninguna petición mientras falte la atribución del puerto.
                    if observation_error_since.is_some_and(|t|t.elapsed()>=Duration::from_millis(250)){return Err(error);}
                }
            }
            thread::sleep(TICK);
        }
        let models = http(worker,&journal,&mut metrics,load_end,config.address,"GET","/v1/models","")?;
        let model = select_loaded_gguf(&models)?;
        journal.event("servicio_comprobado",json!({"pid":pid,"start_ticks":start_ticks,"model":model,"check":"socket_atribuido_y_HTTP_models"}))?;
        let request = json!({"model":model,"messages":[{"role":"system","content":"Responda en español formal y preciso, en una sola frase. No use herramientas."},{"role":"user","content":"Caso sintético: un inventario tiene tres elementos y se añaden dos. ¿Cuántos elementos hay en total?"}],"max_tokens":8,"logprobs":true,"top_logprobs":1,"temperature":0,"reasoning_effort":"low","stream":false});
        phase = "peticion"; journal.event("peticion_prevista", request.clone())?;
        request_sent = true; // Intento HTTP: no equivale a aceptación ni a ejecución por el servidor.
        let value = http(worker,&journal,&mut metrics,(Instant::now()+config.request).min(end),config.address,"POST","/v1/chat/completions",&request.to_string())?;
        journal.event("respuesta_original",value.clone())?;
        if !value["choices"].as_array().is_some_and(|v|!v.is_empty()) {return Err("respuesta_sin_choices".into());}
        response = Some(value); Ok(())
    }));
    let error = match outcome { Ok(Ok(()))=>None, Ok(Err(e))=>Some(e), Err(_)=>Some("panico_controlador".into()) };
    let mut stopped = true; let mut status = None; let mut errors = Vec::new(); let mut pid = None;
    if let Some(worker) = child.as_mut() {
        pid = Some(worker.child.id()); metrics.sample(worker.child.id());
        stopped = worker.stop(error.as_deref().unwrap_or("fin_peticion")); status = worker.status; errors = worker.errors.clone();
    }
    let value = json!({"phase":phase,"error":error,"controller_signal":STOP.load(Ordering::SeqCst),"pid":pid,"start_ticks":start_ticks,"child_exit_code":status.and_then(|s|s.code()),"child_exit_signal":status.and_then(|s|s.signal()),"child_stop_confirmed":stopped,"stop_errors":errors,"peak_rss_bytes":metrics.peak,"rss_samples":metrics.samples,"rss_unavailable":metrics.missing,"rss_scope":"Solo hijo de inferencia; muestreado, no máximo exacto ni cuota agregada", "http_request_attempted":request_sent,"response_received":response.is_some(),"content_validation":"no_realizada","elapsed_ms":origin.elapsed().as_millis(),"external_signal_sender":"no_atribuido", "licensing":serde_json::from_str::<Value>(LICENSE_NOTICE).unwrap_or(Value::Null)});
    let saved = journal.event("resultado_final",value.clone());
    if !stopped || !errors.is_empty() || saved.is_err() || !journal.healthy.load(Ordering::SeqCst) {
        return Err(format!("cierre_o_custodia_no_conforme: {value}; escritura_final={saved:?}"));
    }
    Ok(value)
}

#[cfg(test)] mod tests;
