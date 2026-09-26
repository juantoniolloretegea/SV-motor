
use std::{fs,path::Path,process::{Command,Stdio},io::{Write,BufReader},time::{Instant,Duration},os::unix::process::ExitStatusExt};
use serde_json::{json,Value};
use sv_mcp_documental::{*,custodia::{Custodia,append}};
fn main()->Result<(),Box<dyn std::error::Error>>{
 let a:Vec<String>=std::env::args().collect();if a.get(1).map(String::as_str)==Some("worker"){return sv_mcp_documental::custodia::worker(&a)}
 let bin=&a[1];let cat=&a[2];let root=Path::new(&a[3]);fs::create_dir_all(root)?;let hash=sha256(&fs::read(cat)?);
 let mut cmd=Command::new(bin);let mut child=cmd.args([cat,&hash,root.join("RAW-SERVICIO.jsonl").to_str().unwrap(),"16"]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
 let mut input=child.stdin.take().unwrap();let mut output=BufReader::new(child.stdout.take().unwrap());
 let mut seq=0;
 for raw in [r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"prueba","version":"0.1.1"}}}"#,r#"{"jsonrpc":"2.0","id":2,"method":"ping","method":"tools/list"}"#,r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"leer_documento","arguments":{"pagina":0,"pagina":1}}}"#]{
 seq+=1;writeln!(input,"{raw}")?;input.flush()?;let bytes=read_frame(&mut output)?.ok_or("sin respuesta")?;let v=parse_strict(&bytes)?;
 let ok=if seq==1{v["result"]["protocolVersion"]==PROTOCOL}else{v["error"]["code"]==-32700};
 append(&root.join("RAW-RESULTADOS.jsonl"),&json!({"secuencia":seq,"solicitud":raw,"respuesta":v,"conforme":ok}))?;if !ok{return Err("duplicados no rechazados".into())}
 }
 drop(input);let status=child.wait()?;append(&root.join("RAW-RESULTADOS.jsonl"),&json!({"codigo":status.code(),"senal":status.signal()}))?;
 let mut c=Custodia::new(&root.join("interrupcion"))?;
 let at=Instant::now();let r=c.transact(Path::new(bin),Path::new(cat),&hash,json!({"jsonrpc":"2.0","id":1,"method":"tools/list"}),false,30000,"espera-servicio");
 append(&root.join("INTERRUPCION.jsonl"),&json!({"resultado":r,"duracion_ms":at.elapsed().as_millis(),"conforme":r.is_err()&&at.elapsed()<Duration::from_secs(30)}))?;
 if r.is_ok()||at.elapsed()>=Duration::from_secs(30){return Err("vencimiento no conforme".into())}
 Ok(())
}
