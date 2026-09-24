//! Repetición identificada de preguntas recuperadas; una sola inferencia simultánea.
use std::{fs,io::{Read,Write},net::TcpStream,path::PathBuf,time::{Duration,Instant,SystemTime,UNIX_EPOCH}};
use serde_json::{json,Value};
type Result<T>=std::result::Result<T,Box<dyn std::error::Error+Send+Sync>>;
fn now()->u64{SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()}
fn http(port:u16,method:&str,path:&str,body:&str,key:&str,seconds:u64)->Result<String>{let mut s=TcpStream::connect(("127.0.0.1",port))?;s.set_read_timeout(Some(Duration::from_secs(seconds)))?;s.set_write_timeout(Some(Duration::from_secs(3)))?;write!(s,"{method} {path} HTTP/1.0\r\nHost: localhost\r\nConnection: close\r\nContent-Type: application/json\r\nX-EIO-Session: {key}\r\nContent-Length: {}\r\n\r\n{body}",body.len())?;let mut b=String::new();s.take(8*1024*1024+1).read_to_string(&mut b)?;if b.len()>8*1024*1024{return Err("Respuesta excedida".into())}let(h,b)=b.split_once("\r\n\r\n").ok_or("HTTP incompleto")?;if h.split_whitespace().nth(1)!=Some("200"){return Err(format!("Rechazo HTTP: {h}: {b}").into())}Ok(b.into())}
fn api(key:&str,v:Value)->Result<Value>{Ok(serde_json::from_str(&http(3000,"POST","/api",&v.to_string(),key,20)?)?)}
fn row(out:&mut fs::File,v:Value)->Result<()>{serde_json::to_writer(&mut *out,&v)?;out.write_all(b"\n")?;out.sync_all()?;Ok(())}
fn main()->Result<()>{
 let a:Vec<_>=std::env::args().collect();if a.len()!=4{return Err("Uso: ENTRADAS DIRECTORIO_NUEVO PLAZO_UNIX".into())}let input:Value=serde_json::from_slice(&fs::read(&a[1])?)?;let dir=PathBuf::from(&a[2]);let deadline:u64=a[3].parse()?;fs::create_dir(&dir)?;fs::write(dir.join("ENTRADAS.json"),serde_json::to_vec_pretty(&input)?)?;
 let page=http(3000,"GET","/","","",20)?;let key=page.split("name=\"eio-session\" content=\"").nth(1).and_then(|s|s.split('"').next()).ok_or("Sin sesión")?;
 let state=api(key,json!({"op":"state"}))?;if !state["active"].is_null(){return Err("Otra inferencia activa".into())}fs::write(dir.join("IDENTIDAD.json"),serde_json::to_vec_pretty(&state)?)?;
 let mut out=fs::OpenOptions::new().create_new(true).write(true).open(dir.join("RESULTADOS.jsonl"))?;let started=now();let mut count=0;let mut cases=vec![];
 let outcome=(||->Result<()>{
 // Las rondas conservan todas las respuestas nuevas; los antecedentes no se sustituyen por respuestas ideales.
 for bank in input["banks"].as_array().ok_or("Sin rondas")?{
  let bid=bank["id"].as_str().ok_or("Sin ID")?;
  let case=api(key,json!({"op":"create_case","title":format!("Comparación de preguntas recuperadas {bid}")}))?["id"].as_str().ok_or("Sin expediente")?.to_string();cases.push(case.clone());
  let chat=api(key,json!({"op":"create_chat","case_id":case,"title":format!("Ronda {bid}, historial íntegro")}))?["id"].as_str().ok_or("Sin conversación")?.to_string();
  for task in bank["turns"].as_array().ok_or("Sin preguntas")?{
   if now()+625>deadline{return Err("Plazo restante insuficiente".into())}let text=task["user"].as_str().ok_or("Sin pregunta")?;let label=task["id"].as_str().ok_or("Sin etiqueta")?;let profile=json!({"thinking":false,"max_output":256,"seconds":600,"seed":299792458});
   let preview=api(key,json!({"op":"preview","chat_id":chat,"text":text,"profile":profile}))?;
   if preview["fits"]!=true{row(&mut out,json!({"case":label,"status":"no_admitida_contexto","preview":preview,"source":task}))?;return Err("Historial completo excede 4096; no se recorta".into())}
   let id=format!("comparacion-{started}-{label}");api(key,json!({"op":"send","chat_id":chat,"text":text,"profile":profile,"request_id":id,"context_sha256":preview["context"]["sha256"]}))?;
   let end=Instant::now()+Duration::from_secs(625);let turn=loop{let v=api(key,json!({"op":"get_chat","chat_id":chat}))?;let t=v["chat"]["turns"].as_array().and_then(|ts|ts.iter().find(|t|t["id"]==id)).ok_or("Sin turno")?;if t["status"]!="en_curso"{break t.clone()}if Instant::now()>=end||now()>=deadline{let _=api(key,json!({"op":"cancel","request_id":id}));return Err("Plazo de petición agotado".into())}std::thread::sleep(Duration::from_secs(2));};
   row(&mut out,json!({"case":label,"source":task,"turn":turn,"assessment":"pendiente"}))?;count+=1;println!("COMPARACION {label} {} entrada={} segundos={} respuesta={}",turn["status"],turn["context"]["input_tokens"],turn["result"]["seconds"],turn["answer"]);
   if turn["status"]!="fin_normal"&&turn["status"]!="limite_generacion"{return Err("Fallo técnico; no se fuerza continuación".into())}
  }
  let v=api(key,json!({"op":"export","case_id":case}))?;fs::write(dir.join(format!("{bid}-EXPEDIENTE.json")),serde_json::to_vec_pretty(&v)?)?;
 }
 Ok(())})();
 for (i,case)in cases.iter().enumerate(){if let Ok(v)=api(key,json!({"op":"export","case_id":case})){fs::write(dir.join(format!("CONSERVACION-{i}.json")),serde_json::to_vec_pretty(&v)?)?;}}
 fs::write(dir.join("RESUMEN.json"),serde_json::to_vec_pretty(&json!({"started":started,"ended":now(),"deadline":deadline,"rows":count,"error":outcome.as_ref().err().map(|e|e.to_string())}))?)?;outcome
}
