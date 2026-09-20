//! Observador de testigo independiente de la custodia. No lee journals del supervisor.
//! Preparado, no ejecutado. Ejecutar dentro de la hoja de cgroup autorizada.
use eio_candidato::nativa::{Error,civil};
use std::{fs,os::unix::net::UnixDatagram,path::PathBuf,time::{Instant,Duration}};
fn main()->Result<(),Error>{
 let a:Vec<String>=std::env::args().collect();if a.len()!=3{return Err("observador ENTRADAS PID_TESTIGO".into())}
 let pid:u32=a[2].parse()?;let path=PathBuf::from(&a[1]).join("term-observador.sock");
 let socket=UnixDatagram::bind(&path)?;socket.set_nonblocking(true)?;
 let inicio=Instant::now();let mut term=None;let mut fin=None;let mut muestras=Vec::new();
 while inicio.elapsed()<Duration::from_secs(10){
  let mut b=[0;64];if let Ok(n)=socket.recv(&mut b){if &b[..n]==b"TERM_RECIBIDO"{term=Some(inicio.elapsed().as_nanos())}}
  let s=fs::read_to_string(format!("/proc/{pid}/stat"));
  let estado=s.as_ref().ok().and_then(|x|x.rsplit_once(") ")).and_then(|(_,x)|x.chars().next());
  muestras.push(serde_json::json!({"mono_ns_observador":inicio.elapsed().as_nanos(),"estado":estado}));
  if estado==Some('Z') || matches!(&s,Err(e) if e.kind()==std::io::ErrorKind::NotFound){fin=Some(inicio.elapsed().as_nanos());break}
  std::thread::sleep(Duration::from_millis(10));
 }
 println!("{}",serde_json::json!({"pid":pid,"civil_unix_ms":civil(),"term_recibido_en_observador_ns":term,
 "terminacion_observada_ns":fin,"intervalo_observado_ns":term.zip(fin).map(|(t,f)|f.saturating_sub(t)),
 "muestras":muestras,"limite":"latencia de recepción de datagrama y sondeo; no timestamp del syscall ni garantía de tiempo real"}));
 if term.is_none()||fin.is_none(){return Err("ACTUACION_NO_ACREDITADA".into())}Ok(())
}
