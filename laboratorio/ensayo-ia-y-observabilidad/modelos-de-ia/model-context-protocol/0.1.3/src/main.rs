use std::{io::{self, Write, BufReader}, path::Path, time::Instant};
use serde_json::{json, Value};
use sv_mcp_documental::{Catalog, Session, sha256, rpc_error, auditoria::{Diario,hex,observed_frame}};
fn run()->Result<(),Box<dyn std::error::Error>>{
 let args:Vec<String>=std::env::args().collect();
 sv_mcp_documental::aislamiento::no_network()?;
 if args.len()==3&&args[1]=="--probe-isolation"{println!("{}",sv_mcp_documental::aislamiento::probe(Path::new(&args[2])));return Ok(())}
 if args.len()==2&&args[1]=="--version"{println!("sv-mcp-documental {}",env!("CARGO_PKG_VERSION"));return Ok(())}
 if args.len()!=5&&args.len()!=6{return Err("Uso: sv-mcp-documental CATALOGO SHA256 DIARIO MAX_TRAMAS [--sintetico]".into())}
 let synthetic=args.len()==6&&args[5]=="--sintetico";
 if args.len()==6&&!synthetic{return Err("Opción no reconocida".into())}
 let max_frames:u32=args[4].parse()?;
 if !(1..=256).contains(&max_frames){return Err("MAX_TRAMAS debe estar entre 1 y 256".into())}
 let mut log=Diario::create(Path::new(&args[3]))?;
 log.append(json!({"evento":"inicio","protocolo":sv_mcp_documental::PROTOCOL,"version":env!("CARGO_PKG_VERSION"),
  "catalogo_sha256":args[2],"sintetico":synthetic,"max_tramas":max_frames,"max_llamadas":128,"max_diario_bytes":sv_mcp_documental::auditoria::MAX_DIARIO,
  "binario_sha256":sha256(&std::fs::read(std::env::current_exe()?)?)}))?;
 let catalog=match Catalog::load(Path::new(&args[1]),&args[2],synthetic){
  Ok(c)=>c,Err(e)=>{log.append(json!({"evento":"fallo_catalogo","causa":e}))?;return Err(e.into())}
 };
 let mut session=Session::default();
 let mut input=BufReader::new(io::stdin().lock());let mut output=io::stdout().lock();
 for seq in 0..max_frames{
  let (frame,error)=observed_frame(&mut input);
  if let Some(e)=error{
   log.append(json!({"evento":"trama_rechazada","trama":seq,"causa":e,"bytes_consumidos":frame.len(),"bytes_hex":hex(&frame),"sha256":sha256(&frame),"ejecutada":false}))?;
   return Err(e.into())
  }
  if frame.is_empty(){log.append(json!({"evento":"fin","tramas":seq,"motivo":"entrada_cerrada"}))?;return Ok(())}
  log.append(json!({"evento":"solicitud","trama":seq,"bytes_hex":hex(&frame),"sha256":sha256(&frame)}))?;
  let at=Instant::now();
  let response=match sv_mcp_documental::parse_strict(&frame){Ok(v)=>session.handle(&catalog,v),Err(_)=>Some(rpc_error(Value::Null,-32700,"JSON inválido"))};
  let wire=match &response{Some(v)=>{let mut bytes=serde_json::to_vec(v)?;bytes.push(b'\n');bytes},None=>Vec::new()};
  // Resultado duradero antes de emitirlo; una interrupción entre preparación y entrega queda detectable.
  log.append(json!({"evento":"resultado","trama":seq,"duracion_us":at.elapsed().as_micros(),"respuesta":response,"bytes_hex":hex(&wire),"sha256":sha256(&wire)}))?;
  if let Err(e)=output.write_all(&wire).and_then(|_|output.flush()){
   log.append(json!({"evento":"fallo_entrega","trama":seq,"causa":e.to_string()}))?;return Err(e.into())
  }
  // Acredita escritura al transporte; el conductor debe conservar su recibo.
  log.append(json!({"evento":"entrega","trama":seq,"bytes":wire.len()}))?;
 }
 log.append(json!({"evento":"limite_tramas","tramas":max_frames,"motivo":"presupuesto_agotado"}))?;
 Err("Presupuesto de tramas agotado; recorrido sin cierre normal".into())
}
fn main(){if let Err(e)=run(){eprintln!("Servicio detenido: {e}");std::process::exit(1)}}
