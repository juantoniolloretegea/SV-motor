
use std::{fs::{self,File,OpenOptions},io::{self,Write,BufReader},path::{Path,PathBuf},process::{Command,Stdio},time::{Instant,Duration,SystemTime,UNIX_EPOCH},os::unix::process::{CommandExt,ExitStatusExt},sync::{Arc,atomic::{AtomicUsize,Ordering}}};
use serde_json::{json,Value};
use opentelemetry::trace::{Tracer,Span,TracerProvider};
use opentelemetry_sdk::trace::{SdkTracerProvider,SpanExporter,SpanData};
use crate::{parse_strict,sha256,MAX_FRAME,MAX_RESPONSE,PROTOCOL};
pub fn save(p:&Path,b:&[u8])->io::Result<()>{let mut f=OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(b)?;f.sync_all()?;File::open(p.parent().unwrap())?.sync_all()}
pub fn append(p:&Path,v:&Value)->io::Result<()>{let mut f=OpenOptions::new().create(true).append(true).open(p)?;writeln!(f,"{v}")?;f.sync_all()}
pub fn utc_ms()->u128{SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis()}
#[derive(Debug,Clone)]struct Exporter{path:PathBuf,errors:Arc<AtomicUsize>}
impl SpanExporter for Exporter{async fn export(&self,batch:Vec<SpanData>)->opentelemetry_sdk::error::OTelSdkResult{
 for s in batch{let v=json!({"trace_id":s.span_context.trace_id().to_string(),"span_id":s.span_context.span_id().to_string(),"nombre":s.name,"inicio_unix_ms":s.start_time.duration_since(UNIX_EPOCH).unwrap_or_default().as_millis(),"fin_unix_ms":s.end_time.duration_since(UNIX_EPOCH).unwrap_or_default().as_millis(),"estado":format!("{:?}",s.status)});
 if let Err(e)=append(&self.path,&v){self.errors.fetch_add(1,Ordering::SeqCst);return Err(opentelemetry_sdk::error::OTelSdkError::InternalFailure(e.to_string()));}}
 Ok(())}}
pub struct Custodia{pub root:PathBuf,pub seq:u64,provider:SdkTracerProvider,errors:Arc<AtomicUsize>,_lock:File}
impl Custodia{
 pub fn new(root:&Path)->Result<Self,String>{fs::create_dir_all(root).map_err(|e|e.to_string())?;let lock=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(root.join("EXCLUSION.lock")).map_err(|e|e.to_string())?;lock.try_lock().map_err(|_|"CONCURRENCIA_NO_PERMITIDA")?;
 let errors=Arc::new(AtomicUsize::new(0));let provider=SdkTracerProvider::builder().with_simple_exporter(Exporter{path:root.join("OTEL.jsonl"),errors:errors.clone()}).build();
 Ok(Self{root:root.to_owned(),seq:0,provider,errors,_lock:lock})}
 pub fn transact(&mut self,bin:&Path,cat:&Path,hash:&str,req:Value,synthetic:bool,ms:u64,fault:&str)->Result<Value,String>{
 self.seq+=1;let seq=self.seq;let start=Instant::now();let deadline=start+Duration::from_millis(ms.min(30000));let tracer=self.provider.tracer("sv-mcp-custodia-0.1.1");let mut span=tracer.start("mcp.transaccion");let trace=span.span_context().trace_id().to_string();let d=self.root.join(format!("llamada-{seq:03}"));fs::create_dir(&d).map_err(|e|e.to_string())?;
 let result=(||->Result<Value,String>{
 let raw=serde_json::to_vec(&req).map_err(|e|e.to_string())?;if raw.len()+1>MAX_FRAME{return Err("SOLICITUD_SUPERA_COTA".into())}
 save(&d.join("SOLICITUD.json"),&raw).map_err(|e|e.to_string())?;
 let log=File::create(d.join("WORKER.stderr")).map_err(|e|e.to_string())?;
 let mut cmd=Command::new(std::env::current_exe().map_err(|e|e.to_string())?);cmd.args(["worker",bin.to_str().unwrap(),cat.to_str().unwrap(),hash,d.to_str().unwrap(),if synthetic{"si"}else{"no"},fault]);cmd.process_group(0).stdout(Stdio::null()).stderr(log);
 let mut child=cmd.spawn().map_err(|e|e.to_string())?;let pid=child.id();
 let mut timeout=false;let status=loop{match child.try_wait(){Ok(Some(s))=>break s,Err(e)=>{unsafe{kill(-(pid as i32),9);}let _=child.wait();return Err(e.to_string())},Ok(None)=>{}}
 if Instant::now()+Duration::from_millis(200)>=deadline{timeout=true;unsafe{kill(-(pid as i32),9);}break child.wait().map_err(|e|e.to_string())?}
 std::thread::sleep(Duration::from_millis(2));};
 // El grupo se elimina tambien ante salida anomala del supervisor intermedio.
 unsafe{kill(-(pid as i32),9);}
 append(&self.root.join("PROCESOS.jsonl"),&json!({"TT":"TT-0014","suceso":"S39","secuencia":seq,"trace_id":trace,"pid":pid,"codigo":status.code(),"senal":status.signal(),"vencimiento":timeout,"duracion_ms":start.elapsed().as_millis()})).map_err(|e|e.to_string())?;
 if timeout{return Err("PLAZO_INTEGRAL_AGOTADO".into())}if !status.success(){return Err(format!("WORKER_NO_CONFORME:{:?}",status))}
 let bytes=fs::read(d.join("RESPUESTA.json")).map_err(|e|e.to_string())?;if String::from_utf8_lossy(&bytes).chars().count()>MAX_RESPONSE{return Err("RESPUESTA_SUPERA_COTA".into())}
 let v=parse_strict(&bytes).map_err(|e|e.to_string())?;if v["id"]!=req["id"]||v["jsonrpc"]!="2.0"{return Err("CORRELACION_INVALIDA".into())}
 if Instant::now()>=deadline{return Err("PLAZO_INTEGRAL_AGOTADO".into())}Ok(v)
 })();
 if let Err(e)=&result{span.set_status(opentelemetry::trace::Status::error(e.clone()));}span.end();let _=self.provider.force_flush();
 let record=json!({"TT":"TT-0014","suceso":"S39","version":"0.1.1","trace_id":trace,"secuencia":seq,"request_id":req["id"],"utc_unix_ms":utc_ms(),"duracion_ms":start.elapsed().as_millis(),"estado":if result.is_ok(){"conservado"}else{"error"},"error":result.as_ref().err(),"solicitud_sha256":sha256(&serde_json::to_vec(&req).unwrap()),"respuesta_sha256":fs::read(d.join("RESPUESTA.json")).ok().map(|v|sha256(&v)),"respuesta_bytes":fs::metadata(d.join("RESPUESTA.json")).ok().map(|m|m.len()),"otel_errores":self.errors.load(Ordering::SeqCst)});
 append(&self.root.join("INTERCAMBIOS.jsonl"),&record).map_err(|e|format!("CONSERVACION_FALLIDA:{e}"))?;
 if Instant::now()>=deadline{return Err("PLAZO_INTEGRAL_AGOTADO".into())}
 if self.errors.load(Ordering::SeqCst)>0{return Err("TELEMETRIA_NO_CONSERVADA".into())}result
 }
}
impl Drop for Custodia{fn drop(&mut self){let _=self.provider.shutdown();}}
unsafe extern "C"{fn kill(pid:i32,sig:i32)->i32;}
pub fn worker(a:&[String])->Result<(),Box<dyn std::error::Error>>{
 let d=Path::new(&a[5]);let fault=&a[7];
 if fault=="espera"{std::thread::sleep(Duration::from_secs(60));}
 if fault=="salida"{std::process::exit(7)}
 if fault=="senal"{unsafe{kill(std::process::id() as i32,15);}std::thread::sleep(Duration::from_secs(2));}
 let log=File::create(d.join("SERVICIO.stderr"))?;
 let mut cmd=Command::new(&a[2]);cmd.args([&a[3],&a[4],d.join("SERVICIO.jsonl").to_str().unwrap(),"8"]);if a[6]=="si"{cmd.arg("--sintetico");}
 let mut child=cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(log).spawn()?;
 struct Guard(std::process::Child);impl Drop for Guard{fn drop(&mut self){let _=self.0.kill();let _=self.0.wait();}}
 let input=child.stdin.take().ok_or("stdin")?;let output=child.stdout.take().ok_or("stdout")?;let mut guard=Guard(child);let mut input=Some(input);let mut reader=BufReader::new(output);
 let init=json!({"jsonrpc":"2.0","id":"init","method":"initialize","params":{"protocolVersion":PROTOCOL,"capabilities":{},"clientInfo":{"name":"sv-custodia","version":"0.1.1"}}});
 let mut send=|v:&Value|->io::Result<()>{writeln!(input.as_mut().unwrap(),"{v}")?;input.as_mut().unwrap().flush()};
 send(&init)?;let initraw=crate::read_frame(&mut reader)?.ok_or("sin inicializacion")?;save(&d.join("INICIALIZACION.json"),&initraw)?;
 send(&json!({"jsonrpc":"2.0","method":"notifications/initialized"}))?;
 if fault=="espera-servicio"{std::thread::sleep(Duration::from_secs(60));}
 let request=fs::read(d.join("SOLICITUD.json"))?;let req=parse_strict(&request)?;send(&req)?;drop(send);
 if fault=="desconexion"{drop(input.take());return Err("DESCONEXION_SINTETICA".into())}
 let out=crate::read_frame(&mut reader)?.ok_or("EOF_SIN_RESPUESTA")?;
 if fault=="conservacion"{return Err("CONSERVACION_SINTETICA_FALLIDA".into())}
 save(&d.join("RESPUESTA.json"),&out)?;
 drop(input.take());let status=guard.0.wait()?;
 save(&d.join("SERVICIO-ESTADO.json"),serde_json::to_string(&json!({"codigo":status.code(),"senal":status.signal()}))?.as_bytes())?;
 if !status.success(){return Err("SERVICIO_NO_CONFORME".into())}Ok(())
}

pub fn traced<T,F>(root:&Path,name:&str,f:F)->Result<T,Box<dyn std::error::Error>>
where F:FnOnce(&str)->Result<T,Box<dyn std::error::Error>>{
 let start=Instant::now();let errors=Arc::new(AtomicUsize::new(0));
 let provider=SdkTracerProvider::builder().with_simple_exporter(Exporter{path:root.join("OTEL-HTTP.jsonl"),errors:errors.clone()}).build();
 let mut span=provider.tracer("sv-cliente-http-0.1.1").start(name.to_owned());let trace=span.span_context().trace_id().to_string();
 let result=f(&trace);if let Err(e)=&result{span.set_status(opentelemetry::trace::Status::error(e.to_string()));}span.end();provider.force_flush()?;
 append(&root.join("HTTP-TRANSACCIONES.jsonl"),&json!({"TT":"TT-0014","suceso":"S39","fase":name,"trace_id":trace,"duracion_ms":start.elapsed().as_millis(),"utc_unix_ms":utc_ms(),"error":result.as_ref().err().map(|e|e.to_string()),"otel_errores":errors.load(Ordering::SeqCst)}))?;
 provider.shutdown()?;if errors.load(Ordering::SeqCst)>0{return Err("CONSERVACION_OTEL_HTTP".into())}result
}
