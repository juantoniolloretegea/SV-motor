//! Comprobación HTTP local. No imprime la clave de sesión ni el contenido de los expedientes.
use std::{collections::BTreeMap,fs,io::{Read,Write},net::TcpStream,path::Path,process::{Command,Stdio},thread,time::{Duration,Instant}};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
fn request(method:&str,path:&str,key:&str,body:&str)->Result<String,Box<dyn std::error::Error>>{
 let mut stream=TcpStream::connect_timeout(&"127.0.0.1:3000".parse()?,Duration::from_secs(3))?;stream.set_read_timeout(Some(Duration::from_secs(10)))?;
 write!(stream,"{method} {path} HTTP/1.0\r\nHost: localhost\r\nConnection: close\r\nContent-Type: application/json\r\nX-EIO-Session: {key}\r\nContent-Length: {}\r\n\r\n{body}",body.len())?;
 let mut response=String::new();stream.take(2_097_152).read_to_string(&mut response)?;let (h,b)=response.split_once("\r\n\r\n").ok_or("Sin respuesta HTTP")?;if h.split_whitespace().nth(1)!=Some("200"){return Err("Estado HTTP no conforme".into())}Ok(b.into())
}
fn state()->Result<Value,Box<dyn std::error::Error>>{let page=request("GET","/","","")?;let key=page.split_once("<meta name=\"eio-session\" content=\"").ok_or("Sin sesión")?.1.split('"').next().ok_or("Sin clave")?;Ok(serde_json::from_str(&request("POST","/api",key,"{\"op\":\"state\"}")?)?)}
fn hash(path:impl AsRef<Path>)->String{let mut f=fs::File::open(path).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn files()->BTreeMap<String,String>{fs::read_dir("/workspaces/eio-conversaciones/datos").unwrap().map(|e|e.unwrap().path()).filter(|p|p.is_file()&&p.extension().is_some_and(|s|s=="jsonl")).map(|p|(p.file_name().unwrap().to_string_lossy().into_owned(),hash(&p))).collect()}
fn start_ticks(pid:u32)->String{fs::read_to_string(format!("/proc/{pid}/stat")).unwrap().rsplit_once(") ").unwrap().1.split_whitespace().nth(19).unwrap().to_string()}
fn summary(s:&Value)->Value{json!({"identity":s["identity"],"service":s["service"],"observability":s["observability"],"active_request":!s["active"].is_null(),"cases":s["cases"].as_object().map(|v|v.len()),"chats":s["chats"].as_array().map(|v|v.len())})}
fn main()->Result<(),Box<dyn std::error::Error>>{
 let args:Vec<_>=std::env::args().collect();let before=state()?;
 if args.get(1).map(String::as_str)==Some("--deploy"){
  assert_eq!(args.len(),5,"--deploy PID START_TICKS NUEVO_BINARIO");assert!(before["active"].is_null(),"Hay una inferencia en curso: no se sustituye el servicio");
  let pid:u32=args[2].parse()?;assert_eq!(start_ticks(pid),args[3]);assert_eq!(hash(format!("/proc/{pid}/exe")),before["identity"]["binary_sha256"].as_str().unwrap());
  assert_eq!(before["identity"]["application"],"EIO conversación 0.1.2");let preserved=files();
  nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid as i32),nix::sys::signal::Signal::SIGTERM)?;
  let deadline=Instant::now()+Duration::from_secs(20);while Path::new(&format!("/proc/{pid}")).exists(){assert!(Instant::now()<deadline,"No se confirmó la parada");thread::sleep(Duration::from_millis(100));}
  use std::os::unix::{fs::OpenOptionsExt,process::CommandExt};let log=fs::OpenOptions::new().write(true).create_new(true).mode(0o600).open("/workspaces/eio-observabilidad-013-20260922/servicio-013.log")?;
  let mut command=Command::new(&args[4]);command.stdin(Stdio::null()).stdout(log.try_clone()?).stderr(log);unsafe{command.pre_exec(||{nix::unistd::setsid().map_err(std::io::Error::from)?;Ok(())});}
  let mut child=command.spawn()?;let deadline=Instant::now()+Duration::from_secs(90);
  let first=loop{assert!(child.try_wait()?.is_none(),"El servicio nuevo ha terminado");if let Ok(s)=state(){if s["identity"]["application"]=="EIO conversación 0.1.3"&&s["observability"]["observer"]["fresh"]==true{break s}}assert!(Instant::now()<deadline,"No se confirmó el servicio nuevo");thread::sleep(Duration::from_secs(1));};
  thread::sleep(Duration::from_secs(6));let second=state()?;assert!(second["observability"]["observer"]["samples"].as_u64()>first["observability"]["observer"]["samples"].as_u64());assert_eq!(second["observability"]["telemetry"]["ok"],true);let after=files();
  let report=json!({"schema":"EIO-DESPLIEGUE-OBSERVABILIDAD-1","utc_ms":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_millis(),"previous":summary(&before),"current":summary(&second),"new_pid":child.id(),"samples_advance":true,"files_before":preserved,"files_after":after,"files_identical":preserved==after,"external_access_verified":false,"scope":"Comprobación nativa del HTTP local y de observación reciente. No acredita acceso externo, identidad profesional ni causa del apagado anterior."});
  fs::write("/workspaces/eio-observabilidad-013-20260922/DESPLIEGUE.json",serde_json::to_vec_pretty(&report)?)?;println!("DESPLIEGUE_CONFIRMADO pid={} expedientes_identicos={}",child.id(),preserved==after);
 }else{println!("{}",serde_json::to_string_pretty(&summary(&before))?);}
 Ok(())
}
