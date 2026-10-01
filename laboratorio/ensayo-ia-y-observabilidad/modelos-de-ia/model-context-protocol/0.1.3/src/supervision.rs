
use std::{process::{Command,Stdio},os::unix::{process::{CommandExt,ExitStatusExt},io::AsRawFd},io::Read,time::{Instant,Duration}};
use serde_json::{json,Value};
unsafe extern "C"{fn fcntl(fd:i32,cmd:i32,...)->i32;fn kill(pid:i32,sig:i32)->i32;}
pub fn supervise(args:&[String],limit_ms:u64)->Result<(Value,Vec<u8>),String>{
 let start=Instant::now();let mut child=Command::new(std::env::current_exe().map_err(|e|e.to_string())?).args(args).stdout(Stdio::piped()).stderr(Stdio::inherit()).process_group(0).spawn().map_err(|e|e.to_string())?;
 let pid=child.id();let mut pipe=child.stdout.take().unwrap();let fd=pipe.as_raw_fd();
 unsafe{let flags=fcntl(fd,3);if fcntl(fd,4,flags|2048)<0{kill(-(pid as i32),9);return Err("NONBLOCK_FALLO".into())}}
 let mut out=Vec::new();let mut timed=false;let mut killed=false;let status;
 loop{
 let mut buf=[0u8;4096];loop{match pipe.read(&mut buf){Ok(0)=>break,Ok(n)=>{out.extend_from_slice(&buf[..n]);if out.len()>65536{unsafe{kill(-(pid as i32),9);}return Err("SALIDA_SUPERA_COTA".into())}},Err(e)if e.kind()==std::io::ErrorKind::WouldBlock=>break,Err(e)=>return Err(e.to_string())}}
 if let Some(s)=child.try_wait().map_err(|e|e.to_string())?{status=Some(s);break}
 if start.elapsed()>=Duration::from_millis(limit_ms.saturating_sub(500))&&!killed{timed=true;killed=true;unsafe{kill(-(pid as i32),9);}}
 if start.elapsed()>=Duration::from_millis(limit_ms){status=None;break}
 std::thread::sleep(Duration::from_millis(2));
 }
 unsafe{kill(-(pid as i32),9);}
 let mut buf=[0u8;4096];while let Ok(n)=pipe.read(&mut buf){if n==0{break}out.extend_from_slice(&buf[..n]);}
 let report=json!({"pid":pid,"plazo_ms":limit_ms,"duracion_monotona_ms":start.elapsed().as_millis(),"vencimiento":timed,"recolectado":status.is_some(),"codigo":status.as_ref().and_then(|s|s.code()),"senal":status.as_ref().and_then(|s|s.signal()),"confirmado":!timed&&status.is_some_and(|s|s.success())});
 Ok((report,out))
}
