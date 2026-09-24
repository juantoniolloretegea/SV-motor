//! Cliente Rust de un motor residente local. No interpreta órdenes del modelo.
use super::{Result,Work};
use std::{io::{Read,Write},net::TcpStream,process::Command,time::{Duration,Instant},fs};
use serde_json::{json,Value};
pub const ENGINE:&str="/opt/sv-lab/optimizacion-20260924/target/release/mistralrs";
pub const ENGINE_HASH:&str="f6ec0bb908e18ced633d66d6a762a915349281bdbb1397991b268fe487bcc418";
const UNIT:&str="sv-conversacion-motor.service";
pub fn split(raw:&str,_thinking:bool)->(String,String){(String::new(),raw.trim().to_string())}
fn emit(v:Value)->Result<()>{let mut out=std::io::stdout().lock();serde_json::to_writer(&mut out,&v)?;out.write_all(b"\n")?;out.flush()?;Ok(())}
pub fn memory()->Value{
 let root="/sys/fs/cgroup/system.slice/sv-conversacion-motor.service";
 let read=|name:&str|fs::read_to_string(format!("{root}/{name}")).ok().map(|s|s.trim().to_string());
 json!({"current_bytes":read("memory.current"),"peak_bytes":read("memory.peak"),"maximum_bytes":read("memory.max"),"events":read("memory.events"),"scope":"Cgroup del motor residente; máximo acumulado desde el inicio de la unidad, no incremento atribuible exclusivamente a esta petición"})
}
pub fn stop_engine()->Result<()>{
 let status=Command::new("timeout").args(["15","systemctl","stop",UNIT]).status()?;
 let check=Command::new("systemctl").args(["show",UNIT,"--property=MainPID","--value"]).output()?;
 if !status.success()||!check.status.success()||String::from_utf8_lossy(&check.stdout).trim()!="0"{return Err("No se confirmó la parada del motor residente".into())}Ok(())
}
pub fn http(port:u16,method:&str,path:&str,body:&str,seconds:u64,session:Option<&str>)->Result<Value>{
 let mut stream=TcpStream::connect_timeout(&format!("127.0.0.1:{port}").parse()?,Duration::from_secs(2))?;
 stream.set_read_timeout(Some(Duration::from_secs(seconds)))?;stream.set_write_timeout(Some(Duration::from_secs(3)))?;
 let extra=session.map(|s|format!("X-EIO-Session: {s}\r\n")).unwrap_or_default();
 write!(stream,"{method} {path} HTTP/1.0\r\nHost: localhost\r\nConnection: close\r\nContent-Type: application/json\r\n{extra}Content-Length: {}\r\n\r\n{body}",body.len())?;
 let mut bytes=Vec::new();stream.take(8*1024*1024+1).read_to_end(&mut bytes)?;
 if bytes.len()>8*1024*1024{return Err("Respuesta HTTP demasiado extensa".into())}
 let text=String::from_utf8(bytes)?;let(head,body)=text.split_once("\r\n\r\n").ok_or("Respuesta HTTP incompleta")?;
 if head.lines().next().and_then(|s|s.split_whitespace().nth(1))!=Some("200"){return Err(format!("Rechazo HTTP: {}: {}",head.lines().next().unwrap_or("sin estado"),body).into())}
 Ok(serde_json::from_str(body)?)
}
fn ready()->Result<()>{
 let status=Command::new("timeout").args(["15","systemctl","start",UNIT]).status()?;if !status.success(){return Err("No se pudo iniciar el motor".into())}
 let end=Instant::now()+Duration::from_secs(180);
 loop{if let Ok(v)=http(8089,"GET","/v1/models","",2,None){if v["data"].as_array().is_some_and(|a|a.iter().any(|m|m["id"]=="gguf"&&m["status"]=="loaded")){return Ok(())}}
  if Instant::now()>=end{return Err("El motor no confirmó disponibilidad".into())}std::thread::sleep(Duration::from_millis(500));}
}
pub fn worker()->Result<()>{
 let result=(||->Result<()>{
  nix::sys::prctl::set_pdeathsig(nix::sys::signal::Signal::SIGTERM)?;
  let mut bytes=Vec::new();std::io::stdin().take(2*1024*1024).read_to_end(&mut bytes)?;let w:Work=serde_json::from_slice(&bytes)?;
  if nix::unistd::getppid().as_raw() as u32!=w.parent{return Err("Supervisor ausente".into())}
  let start=Instant::now();emit(json!({"kind":"phase","text":"Comprobando disponibilidad del motor"}))?;ready()?;
  let readiness=start.elapsed().as_secs_f64();emit(json!({"kind":"phase","text":"Procesando el contexto completo y generando la respuesta"}))?;
  let request=json!({"model":"gguf","prompt":w.context.prompt,"max_tokens":w.profile.max_output,"temperature":0,"logprobs":1,"echo_prompt":false,"stream":false});
  let inference=Instant::now();let v=http(8089,"POST","/v1/completions",&request.to_string(),w.profile.seconds,None)?;
  // Conservar la respuesta original antes de comprobarla facilita distinguir transporte y modelo.
  let timings=json!({"readiness_seconds":readiness,"request_seconds":inference.elapsed().as_secs_f64(),"engine_response":v,"first_token_seconds":null,"scope":"Petición completa al motor; sin medida separada de prellenado ni primer token"});
  emit(json!({"kind":"timings","timings":timings}))?;
  if v["usage"]["prompt_tokens"].as_u64()!=Some(w.context.input_tokens as u64){return Err(format!("Recuento de entrada distinto: interfaz {}, motor {}",w.context.input_tokens,v["usage"]["prompt_tokens"]).into())}
  let choice=v["choices"].as_array().and_then(|a|a.first()).ok_or("Respuesta sin alternativa")?;
  let raw=choice["text"].as_str().ok_or("Respuesta sin texto")?;
  let finish=match choice["finish_reason"].as_str(){Some("stop")=>"fin_normal",Some("length")=>"limite_generacion",_=>return Err("Causa de terminación desconocida".into())};
  emit(json!({"kind":"done","raw":raw,"tokens":v["usage"]["completion_tokens"],"finish":finish,"timings":timings}))?;Ok(())
 })();if let Err(e)=&result{let _=emit(json!({"kind":"error","error":e.to_string()}));}result
}
#[cfg(test)]mod tests{use super::*;#[test]fn final_sin_interpretacion(){assert_eq!(split(" Texto <think> literal ",false),(String::new(),"Texto <think> literal".into()));}}
