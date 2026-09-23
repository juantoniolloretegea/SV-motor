use super::*;
use std::sync::Mutex;

// Regresión observada en almacenamiento remoto: una escritura correcta que
// supera 250 ms no es, por sí sola, un fallo de persistencia.
#[test] fn escritura_demorada_pero_correcta_se_confirma() {
    struct Slow(Vec<u8>);
    impl Write for Slow {
        fn write(&mut self,b:&[u8])->io::Result<usize>{self.0.extend_from_slice(b);Ok(b.len())}
        fn flush(&mut self)->io::Result<()>{thread::sleep(Duration::from_millis(400));Ok(())}
    }
    let journal=Journal::with_writer(Slow(Vec::new()));
    assert!(journal.event("escritura_demorada",json!({})).is_ok());
    assert!(journal.healthy.load(Ordering::SeqCst));
}

struct Area { root:PathBuf, config:Option<Config> }
impl Area {
    fn new(mode:&str)->Self {
        let root=std::env::temp_dir().join(format!("eio-oss-{}-{}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
        fs::create_dir_all(root.join("motor")).unwrap();
        let program=root.join("motor/mistralrs"); fs::copy(std::env::current_exe().unwrap(),&program).unwrap();
        let listener=TcpListener::bind("127.0.0.1:0").unwrap(); let address=listener.local_addr().unwrap(); drop(listener);
        let mut c=Config::new(root.clone(),root.join("evidencias"),hash_file(&program).unwrap());
        c.address=address; c.window=Duration::from_secs(5); c.load=Duration::from_secs(2); c.request=Duration::from_secs(2);
        c.virtual_limit=Some(128*1024*1024);c.minimum_memory=32*1024*1024;c.reserve_memory=64*1024*1024;
        c.args=Some(vec!["--exact".into(),"tests::auxiliar".into(),"--nocapture".into()]);
        c.env=vec![("EIO_TEST_MODE".into(),mode.into()),("EIO_TEST_PORT".into(),address.port().to_string())];
        Self {root,config:Some(c)}
    }
    fn take(&mut self)->Config {self.config.take().unwrap()}
    fn rows(&self)->Vec<Value> {fs::read_to_string(self.root.join("evidencias/SUCESOS.jsonl")).unwrap().lines().map(|v|serde_json::from_str(v).unwrap()).collect()}
    fn assert_gone(&self) {if let Ok(pid)=fs::read_to_string(self.root.join("auxiliar.pid")){assert!(!Path::new(&format!("/proc/{}",pid.trim())).exists(),"auxiliar vivo");}}
}
impl Drop for Area {fn drop(&mut self){let _=fs::remove_dir_all(&self.root);}}

#[test] fn auxiliar() {
    let Ok(mode)=std::env::var("EIO_TEST_MODE") else{return};
    fs::write("auxiliar.pid",std::process::id().to_string()).unwrap();
    let memory=vec![1u8;8*1024*1024];std::hint::black_box(&memory);
    if mode=="ignorar_term" {unsafe{libc::signal(libc::SIGTERM,libc::SIG_IGN);}}
    if mode=="sigterm" {thread::sleep(Duration::from_millis(200));unsafe{libc::kill(std::process::id() as i32,libc::SIGTERM);}return;}
    if mode=="esperar"||mode=="ignorar_term" {thread::sleep(Duration::from_secs(10));return;}
    let port=std::env::var("EIO_TEST_PORT").unwrap();let listener=TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    for stream in listener.incoming(){let mut stream=stream.unwrap();stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();let mut data=Vec::new();let mut b=[0u8;4096];
        loop {
            if let Some(end)=data.windows(4).position(|v|v==b"\r\n\r\n") {
                let header=String::from_utf8_lossy(&data[..end]);
                let length=header.lines().find_map(|line|line.strip_prefix("Content-Length:")?.trim().parse::<usize>().ok()).unwrap_or(0);
                if data.len()>=end+4+length {break;}
            }
            let n=stream.read(&mut b).unwrap();if n==0{break;}data.extend_from_slice(&b[..n]);
        }
        let post=data.starts_with(b"POST");if post {fs::write("peticion_recibida","si").unwrap();}
        let body=if mode=="json_invalido"{"{incorrecto"}else if post{r#"{"choices":[{"message":{"content":"Cinco."}}]}"#}else{r#"{"data":[{"id":"modelo-sintetico"}]}"#};
        write!(stream,"HTTP/1.0 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).unwrap();
    }
}

#[test] fn peticion_unica_y_parada_trazada() {
    let _s=Signals::install().unwrap();let mut a=Area::new("http");let v=execute(a.take()).unwrap();
    assert!(v["error"].is_null(),"{v}");assert_eq!(v["response_received"],true);assert_eq!(v["child_stop_confirmed"],true);assert!(v["peak_rss_bytes"].as_u64().unwrap()>0);
    let rows=a.rows();let before=rows.iter().position(|v|v["kind"]=="senal_solicitada").unwrap();let after=rows.iter().position(|v|v["kind"]=="senal_resultado").unwrap();assert!(before<after);
    assert_eq!(rows[before]["data"]["reason"],"fin_peticion");assert_eq!(rows[after]["data"]["return"],0);
    assert_eq!(rows.iter().filter(|v|v["kind"]=="peticion_prevista").count(),1);a.assert_gone();
}
#[test] fn sigterm_anticipado_conserva_memoria_sin_atribuir_emisor() {
    let _s=Signals::install().unwrap();let mut a=Area::new("sigterm");let v=execute(a.take()).unwrap();
    assert_eq!(v["child_exit_signal"],15);assert!(v["peak_rss_bytes"].as_u64().unwrap()>0);assert_eq!(v["http_request_attempted"],false);
    assert!(!a.rows().iter().any(|v|v["kind"]=="senal_solicitada"));assert_eq!(v["external_signal_sender"],"no_atribuido");a.assert_gone();
}
#[test] fn muestras_persistidas_mientras_el_hijo_sigue_activo(){
 let _s=Signals::install().unwrap();let mut a=Area::new("esperar");a.config.as_mut().unwrap().load=Duration::from_millis(1500);
 let path=a.root.join("evidencias/SUCESOS.jsonl");
 let reader=thread::spawn(move||{
  let end=Instant::now()+Duration::from_secs(4);
  loop{let text=fs::read_to_string(&path).unwrap_or_default();
   let rows:Vec<Value>=text.lines().filter_map(|l|serde_json::from_str(l).ok()).collect();
   let samples:Vec<_>=rows.iter().filter(|r|r["kind"]=="muestra_recursos").collect();
   if samples.len()>=2 {let sample=samples.last().unwrap();let pid=sample["data"]["pid"].as_u64().unwrap();
    assert!(Path::new(&format!("/proc/{pid}")).exists());assert!(sample["data"]["rss_bytes"].as_u64().unwrap()>0);return}
   assert!(Instant::now()<end,"No hay dos muestras persistidas durante la carga");thread::sleep(TICK);
  }
 });
 let v=execute(a.take()).unwrap();reader.join().unwrap();assert_eq!(v["error"],"limite_temporal");a.assert_gone();
}
#[test] fn plazo_monotono_y_escalada_term_kill() {
    let _s=Signals::install().unwrap();let mut a=Area::new("ignorar_term");a.config.as_mut().unwrap().load=Duration::from_millis(400);
    let begin=Instant::now();let v=execute(a.take()).unwrap();assert!(begin.elapsed()<Duration::from_secs(4));assert_eq!(v["error"],"limite_temporal");assert_eq!(v["child_exit_signal"],9);
    let signals:Vec<_>=a.rows().iter().filter(|v|v["kind"]=="senal_solicitada").map(|v|v["data"]["signal"].as_i64().unwrap()).collect();assert_eq!(signals,vec![15,9]);a.assert_gone();
}
#[test] fn sigterm_controlador_cierra_hijo() {
    let _s=Signals::install().unwrap();let mut a=Area::new("esperar");let pidpath=a.root.join("auxiliar.pid");
    let signal=thread::spawn(move||{let end=Instant::now()+Duration::from_secs(3);while !pidpath.exists(){assert!(Instant::now()<end);thread::sleep(TICK);}unsafe{assert_eq!(libc::kill(std::process::id() as i32,libc::SIGTERM),0);}});
    let v=execute(a.take()).unwrap();signal.join().unwrap();assert_eq!(v["controller_signal"],15);assert_eq!(v["child_stop_confirmed"],true);a.assert_gone();
}
#[test] fn puerto_ajeno_se_rechaza_antes_del_arranque() {
    let _s=Signals::install().unwrap();let mut a=Area::new("http");let _busy=TcpListener::bind(a.config.as_ref().unwrap().address).unwrap();
    let v=execute(a.take()).unwrap();assert!(v["error"].as_str().unwrap().starts_with("puerto_ocupado"));assert!(v["pid"].is_null());assert!(!a.root.join("auxiliar.pid").exists());
}
#[test] fn json_invalido_se_conserva_y_cierra_sin_peticion() {
    let _s=Signals::install().unwrap();let mut a=Area::new("json_invalido");let v=execute(a.take()).unwrap();
    assert!(v["error"].as_str().unwrap().starts_with("json_invalido"));assert_eq!(v["child_stop_confirmed"],true);assert_eq!(v["http_request_attempted"],false);
    assert!(a.rows().iter().any(|v|v["kind"]=="http_respuesta"&&v["data"]["raw"].as_str().unwrap().contains("{incorrecto")));a.assert_gone();
}
#[test] fn ventana_nueva_exclusion_y_presupuesto_invalido() {
    let _s=Signals::install().unwrap();let mut a=Area::new("http");a.config.as_mut().unwrap().window=Duration::ZERO;
    assert!(execute(a.take()).is_err());assert!(!a.root.join("auxiliar.pid").exists());
    let mut b=Area::new("http");let lock=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(b.root.join("controlador.lock")).unwrap();lock.try_lock().unwrap();
    assert!(execute(b.take()).unwrap_err().starts_with("instancia_ya_activa"));assert!(!b.root.join("auxiliar.pid").exists());
}
struct Faulty { bytes:Arc<Mutex<Vec<u8>>>, armed:bool, block:bool }
impl Write for Faulty {
    fn write(&mut self,b:&[u8])->io::Result<usize>{let mut bytes=self.bytes.lock().unwrap();bytes.extend_from_slice(b);self.armed=String::from_utf8_lossy(&bytes).contains("muestra_recursos");Ok(b.len())}
    fn flush(&mut self)->io::Result<()>{if self.armed {if self.block {thread::sleep(Duration::from_secs(1));}return Err(io::Error::other("fallo_del_escritor_sintetico"));}Ok(())}
}
#[test] fn fallo_y_bloqueo_de_custodia_no_impiden_parada() {
    let _s=Signals::install().unwrap();
    for block in [false,true] {
        let mut a=Area::new("esperar");let c=a.take();fs::create_dir(&c.evidence).unwrap();
        let bytes=Arc::new(Mutex::new(Vec::new()));let journal=Journal::with_writer(Faulty{bytes:bytes.clone(),armed:false,block});
        let begin=Instant::now();let e=run(c,a.root.clone(),a.root.join("evidencias"),journal).unwrap_err();
        assert!(e.starts_with("cierre_o_custodia_no_conforme"));assert!(begin.elapsed()<Duration::from_secs(4),"la custodia bloqueó el cierre");
        let snapshot=bytes.lock().unwrap().clone();
        let rows:Vec<Value>=snapshot.split_inclusive(|b|*b==b'\n').filter(|l|l.ends_with(b"\n")).map(|l|serde_json::from_slice(l).unwrap()).collect();
        let pid=rows.iter().find(|r|r["kind"]=="proceso_creado").expect("El fallo debe producirse después del arranque")["data"]["pid"].as_u64().unwrap();
        assert!(!Path::new(&format!("/proc/{pid}")).exists());
    }
}

fn owned_auxiliary(a:&Area,journal:Journal)->OwnedChild {
    let c=a.config.as_ref().unwrap();let child=Command::new(a.root.join("motor/mistralrs")).args(c.args.as_ref().unwrap()).envs(c.env.iter().cloned()).current_dir(&a.root).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).process_group(0).spawn().unwrap();
    OwnedChild{child,status:None,journal,label:"auxiliar",errors:Vec::new()}
}
#[test] fn limpieza_raii_durante_panico() {
    let _s=Signals::install().unwrap();let a=Area::new("esperar");let journal=Journal::with_writer(Vec::<u8>::new());let pid;
    let owned=owned_auxiliary(&a,journal);pid=owned.child.id();
    let caught=std::panic::catch_unwind(std::panic::AssertUnwindSafe(move||{let _owned=owned;panic!("panico_del_banco");}));
    assert!(caught.is_err());assert!(!Path::new(&format!("/proc/{pid}")).exists());
}
#[test] fn error_real_de_envio_se_conserva() {
    let _s=Signals::install().unwrap();let a=Area::new("esperar");let journal=Journal::with_writer(Vec::<u8>::new());let mut child=owned_auxiliary(&a,journal);
    assert!(child.send(9999,"senal_invalida_del_banco").is_err());assert!(!child.errors.is_empty());
    assert!(child.stop("limpieza_tras_error"));assert!(!Path::new(&format!("/proc/{}",child.child.id())).exists());
}
