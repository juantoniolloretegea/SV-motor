#![forbid(unsafe_code)]
//! Transporte y recepción comunes. La configuración limita proveedor y modelo.
pub mod estricto;
pub mod presupuesto;
pub mod chat;
pub mod responses;
use serde::{Deserialize,Serialize};
use serde_json::{json,Value};
use std::{fs::{self,OpenOptions},io::{Read,Write},path::Path,time::{Duration,Instant}};
pub type R<T> = Result<T,String>;
pub fn need(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
pub fn parse(b:&[u8])->R<Value>{estricto::parse(b).map_err(|e|e.to_string())}
pub fn sha(b:&[u8])->String{sv_instrumentacion::sha(b)}
pub fn guard(p:&Path)->R<()>{
 let root=Path::new("C:/SV");let rel=p.strip_prefix(root).map_err(|_|"Destino fuera del perímetro")?;let mut cur=root.to_path_buf();
 for c in rel.components(){need(matches!(c,std::path::Component::Normal(_)),"Ruta no normal")?;cur.push(c);if cur.exists(){let m=fs::symlink_metadata(&cur).map_err(|e|e.to_string())?;need(!m.file_type().is_symlink(),"Enlace no permitido")?;#[cfg(windows)]{use std::os::windows::fs::MetadataExt;need(m.file_attributes()&0x400==0,"Reanálisis no permitido")?;}}}Ok(())
}
pub fn put(p:&Path,b:&[u8])->R<()>{guard(p)?;fs::create_dir_all(p.parent().ok_or("Sin directorio")?).map_err(|e|e.to_string())?;let mut f=OpenOptions::new().create_new(true).write(true).open(p).map_err(|e|e.to_string())?;f.write_all(b).and_then(|_|f.sync_all()).map_err(|e|e.to_string())}
pub fn save(p:&Path,v:&Value)->R<()>{put(p,&serde_json::to_vec_pretty(v).map_err(|e|e.to_string())?)}
pub const LICENCIA:&str="© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
pub const AVISO:&str="Aviso de derechos: la licencia indicada corresponde al material propio del SV; las fuentes de terceros conservan su autoría y régimen original. El envío tiene por única finalidad procesar y devolver esta respuesta experimental. No se autoriza usar el material propio para entrenamiento, desarrollo de otros productos, publicación ni cesión de derechos. Este contenido no constituye feedback o cesión de propiedad intelectual. La autorización de procesamiento necesaria para responder no transfiere la titularidad. No añada este aviso a la respuesta científica solicitada.";

#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Perfil {pub proveedor:String,pub modelo:String,pub endpoint:String,pub presupuesto_ticks:u64,pub exigir_zdr:bool,pub entrada_ticks_por_token:u64,pub salida_ticks_por_token:u64,#[serde(default,skip_serializing_if="Option::is_none")]pub cuota_gratuita_tokens:Option<u64>}
impl Perfil{pub fn comprobar(&self)->R<()>{
 let destino=match self.proveedor.as_str(){"Z.ai"=>self.endpoint=="https://api.z.ai/api/paas/v4/chat/completions"&&self.modelo=="glm-5.3","xAI"=>self.endpoint=="https://api.x.ai/v1/responses","OpenAI"=>self.endpoint=="https://api.openai.com/v1/responses","Alibaba Cloud"=>{
  let u=reqwest::Url::parse(&self.endpoint).map_err(|_|"URL inválida")?;
  let workspace=u.host_str().and_then(|s|s.strip_suffix(".ap-southeast-1.maas.aliyuncs.com")).and_then(|s|s.strip_prefix("ws-"));
  u.scheme()=="https"&&u.username().is_empty()&&u.password().is_none()&&u.port().is_none()&&u.query().is_none()&&u.fragment().is_none()&&u.path()=="/compatible-mode/v1/responses"&&workspace.is_some_and(|s|!s.is_empty()&&s.len()<=64&&s.bytes().all(|b|b.is_ascii_lowercase()||b.is_ascii_digit()))
 },_=>false};need(destino,"Destino o proveedor no admitido")?;
 need(!self.modelo.is_empty()&&self.entrada_ticks_por_token>0&&self.salida_ticks_por_token>0,"Perfil incompleto")?;
 if let Some(n)=self.cuota_gratuita_tokens{need(self.proveedor=="Alibaba Cloud"&&self.modelo=="qwen3.8-max-0902"&&self.presupuesto_ticks==0&&n>0&&n<=1_000_000,"Cuota gratuita o presupuesto no autorizado")?;}else{need(self.proveedor!="Alibaba Cloud"&&(self.presupuesto_ticks>0||matches!(self.proveedor.as_str(),"Z.ai"|"OpenAI")),"Falta protección de cuota gratuita")?;}Ok(())
}}

pub fn proteger(q:&mut Value,p:&Perfil)->R<()> {
 p.comprobar()?;q["model"]=json!(p.modelo);q["tools"]=json!([]);q["tool_choice"]=json!("none");q["store"]=json!(false);q["stream"]=json!(true);
 q["instructions"]=json!(format!("{}\n{}\n{}",q["instructions"].as_str().ok_or("Faltan instrucciones")?,LICENCIA,AVISO));
 // Compatibilidad explícita de proveedor; el motor del ensayo no cambia.
 if p.proveedor=="xAI"{q["reasoning"]=json!({"effort":"medium"});q.as_object_mut().ok_or("Solicitud no es objeto")?.remove("tool_choice");}
 if p.proveedor=="Alibaba Cloud"{
  q["reasoning"]=json!({"effort":"medium"});
  // Responses de Alibaba no documenta text.format. El mismo esquema se exige
  // en las instrucciones y se comprueba en Rust; no se presume garantía remota.
  let schema=q.pointer("/text/format/schema").cloned().ok_or("Falta esquema de entrega")?;
  q["instructions"]=json!(format!("{}\nEntregue únicamente un objeto JSON válido, sin cercas Markdown, conforme a este esquema íntegro: {}",q["instructions"].as_str().unwrap(),schema));
  q.as_object_mut().unwrap().remove("text");
 }
 need(q.get("previous_response_id").is_none()&&q.get("conversation").is_none(),"Estado remoto no autorizado")?;
 if p.proveedor=="Z.ai"{*q=chat::solicitud(q,&p.modelo)?;}
 Ok(())
}
pub fn reserva(q:&Value,p:&Perfil)->R<u64>{
 let b=serde_json::to_vec(q).map_err(|e|e.to_string())?;
 need(b.len()<190000,"Contexto excluido del tramo corto de tarifa")?;
 let field=if p.proveedor=="Z.ai"{"max_tokens"}else{"max_output_tokens"};
 let cap=if p.proveedor=="Z.ai"{16384}else{8192}; let out=q[field].as_u64().filter(|x|*x<=cap&&*x>0).ok_or("Límite de salida no admitido")?;
 // Cota deliberadamente conservadora: bytes UTF-8 de toda la petición más 4096
 // unidades de margen. No es una medición de tokens del proveedor.
 (b.len() as u64+4096).checked_mul(p.entrada_ticks_por_token).and_then(|x|out.checked_mul(p.salida_ticks_por_token).and_then(|y|x.checked_add(y))).ok_or("Desbordamiento de reserva".into())
}

#[derive(Default)]
pub struct Flujo {pending:Vec<u8>,pub eventos:Vec<Value>,terminal:bool,done:bool,ultima:Option<u64>}
impl Flujo{
 pub fn feed(&mut self,b:&[u8])->R<Vec<String>>{
  self.pending.extend_from_slice(b);let mut tipos=vec![];
  while let Some(i)=self.pending.iter().position(|b|*b==b'\n'){
   let line=self.pending.drain(..=i).collect::<Vec<_>>();let s=std::str::from_utf8(&line).map_err(|_|"SSE no UTF-8")?.trim_end_matches(['\r','\n']);
   if let Some(data)=s.strip_prefix("data:"){
    if data.trim()=="[DONE]"{need(self.terminal&&!self.done,"Cierre SSE inválido")?;self.done=true;continue;}
    need(!self.terminal&&!self.done,"Evento posterior al cierre")?;
    let v=parse(data.trim().as_bytes())?;
    if let Some(n)=v["sequence_number"].as_u64(){if let Some(last)=self.ultima{need(n==last+1,"Secuencia SSE discontinua")?;}self.ultima=Some(n);}
    let kind=v["type"].as_str().ok_or("Tipo SSE ausente")?;
    need(matches!(kind,"response.created"|"response.in_progress"|"response.output_item.added"|"response.content_part.added"|"response.output_text.delta"|"response.output_text.done"|"response.content_part.done"|"response.output_item.done"|"response.completed"|"response.failed"|"response.incomplete"|"error"|"response.reasoning_summary_part.added"|"response.reasoning_summary_part.done"|"response.reasoning_summary_text.delta"|"response.reasoning_summary_text.done"|"response.reasoning_text.delta"|"response.reasoning_text.done"|"keepalive"),"Evento ajeno al contrato; original conservado")?;
    if let Some(item)=v.get("item"){need(matches!(item["type"].as_str(),Some("message"|"reasoning")),"Herramienta no admitida")?;}
    self.terminal=matches!(kind,"response.completed"|"response.failed"|"response.incomplete");tipos.push(kind.into());self.eventos.push(v);
   }else{need(s.is_empty()||s.starts_with(':')||s.starts_with("event:")||(s.starts_with("id:")&&!s.contains('\0')),"Campo SSE desconocido")?;}
  }Ok(tipos)
 }
 pub fn recibir(&self,modelo:&str)->R<Value>{
  need(self.terminal&&self.pending.iter().all(u8::is_ascii_whitespace),"Entrega incompleta")?;
  if modelo=="gpt-6-astra"{
   let mut received=responses::extract(&self.eventos,modelo)?;
   received["respuesta_proveedor"]=self.eventos.last().ok_or("Sin eventos")?["response"].clone();
   received["eventos"]=json!(self.eventos.len());received["concordancia_sse_texto"]=json!(true);
   return Ok(received);
  }
  let terminal=self.eventos.last().ok_or("Sin eventos")?;need(terminal["type"]=="response.completed","Proveedor no completó la respuesta")?;
  let r=&terminal["response"];need(r["status"]=="completed"&&r["model"]==modelo,"Modelo o estado no conforme")?;
  let mut text=String::new();for item in r["output"].as_array().ok_or("Salida ausente")?{
   match item["type"].as_str(){Some("message")=>{need(item["role"]=="assistant","Emisor ajeno")?;for c in item["content"].as_array().ok_or("Contenido ausente")?{need(c["type"]=="output_text","Contenido no textual")?;text.push_str(c["text"].as_str().ok_or("Texto ausente")?);}},Some("reasoning")=>{},_=>return Err("Salida de herramienta no autorizada".into())}
  }
  let deltas=self.eventos.iter().filter(|v|v["type"]=="response.output_text.delta").map(|v|v["delta"].as_str().ok_or("Delta no textual")).collect::<Result<Vec<_>,_>>()?.concat();
  need(!text.is_empty()&&text==deltas,"Texto final y deltas discordantes")?;
  Ok(json!({"texto_original":text,"uso_proveedor":r["usage"],"respuesta_proveedor":r,"eventos":self.eventos.len(),"concordancia_sse_texto":true}))
 }
}

pub fn enviar(p:&Perfil,key:&str,q:&Value,dest:&Path,timeout_ms:u64)->R<Value>{
 need(p.proveedor!="OpenAI"||p.presupuesto_ticks>0,"Envío con clave API sin presupuesto recibido")?;
 enviar_interno(p,key,q,dest,timeout_ms)
}
/// Sesión verificada por el adaptador OAuth, sin clave API ni presupuesto en USD.
/// Las cotas de preguntas, intentos, tiempo y tokens pertenecen al Árbitro.
pub fn enviar_chatgpt(p:&Perfil,token:&str,q:&Value,dest:&Path,timeout_ms:u64)->R<Value>{
 need(p.proveedor=="OpenAI"&&p.modelo=="gpt-6-astra"&&p.presupuesto_ticks==0&&p.cuota_gratuita_tokens.is_none(),"Modalidad ChatGPT ajena")?;
 need(q["max_output_tokens"].as_u64().is_some_and(|n|n>0&&n<=16384),"Salida fuera de cota")?;
 enviar_interno(p,token,q,dest,timeout_ms)
}
fn enviar_interno(p:&Perfil,key:&str,q:&Value,dest:&Path,timeout_ms:u64)->R<Value>{
 p.comprobar()?;
 if p.proveedor=="Z.ai"{chat::validar(q,&p.modelo)?;need(p.presupuesto_ticks>0,"Perfil Z.ai limitado a preparación; envío impedido")?;}else{
 let choice=if p.proveedor=="xAI"{q.get("tool_choice").is_none()}else{q["tool_choice"]=="none"};need(q["model"]==p.modelo&&q["tools"]==json!([])&&choice&&q["store"]==false&&q["stream"]==true,"Solicitud fuera de perfil")?;
 let instructions=q["instructions"].as_str().ok_or("Sin instrucciones")?;need(instructions.contains(LICENCIA)&&instructions.contains(AVISO),"Licencia ausente: envío impedido")?;}
 guard(dest)?;need(!dest.exists(),"Directorio de entrega ya existe")?;fs::create_dir_all(dest).map_err(|e|e.to_string())?;let req=serde_json::to_vec_pretty(q).map_err(|e|e.to_string())?;put(&dest.join("SOLICITUD.json"),&req)?;
 let m=sv_instrumentacion::Monitor::start_bounded(&dest.join("instrumentacion"),330)?;m.event("fase",json!({"fase":"antes","solicitud_sha256":sha(&req)}))?;std::thread::sleep(Duration::from_secs(1));let t=Instant::now();
 let op=(||->R<Value>{
  m.healthy()?;let http=reqwest::blocking::Client::builder().https_only(true).no_proxy().redirect(reqwest::redirect::Policy::none()).retry(reqwest::retry::never()).connect_timeout(Duration::from_secs(10)).timeout(Duration::from_millis(timeout_ms.min(300000))).user_agent("SV-Cliente-API/0.1.0").build().map_err(|_|"Configuración HTTPS")?;
  save(&dest.join("ENVIO.json"),&json!({"utc_ms":sv_instrumentacion::utc_ms(),"proveedor":p.proveedor,"modelo":p.modelo,"endpoint":p.endpoint,"solicitud_sha256":sha(&req),"reintentos_transporte":0}))?;
  m.event("fase",json!({"fase":"durante","bytes":req.len()}))?;
  let mut res=http.post(&p.endpoint).bearer_auth(key).header("Content-Type","application/json").body(req).send().map_err(|_|"Transporte interrumpido; coste desconocido")?;
  let status=res.status().as_u16();let mut headers=serde_json::Map::new();for n in ["content-type","x-request-id","x-zero-data-retention","x-ratelimit-limit-requests","x-ratelimit-remaining-requests","x-ratelimit-remaining-tokens","retry-after"]{if let Some(v)=res.headers().get(n).and_then(|v|v.to_str().ok()){headers.insert(n.into(),json!(v));}}
  save(&dest.join("HTTP.json"),&json!({"status":status,"cabeceras":headers,"cabeceras_ms":t.elapsed().as_millis(),"remoto":res.remote_addr().map(|v|v.to_string()),"version":format!("{:?}",res.version())}))?;
  let mut raw=OpenOptions::new().create_new(true).write(true).open(dest.join("SALIDA-SSE.txt")).map_err(|e|e.to_string())?;
  let mut stream=Flujo::default();let mut chat_stream=chat::Flujo::default();let mut b=[0;8192];let mut total=0;let mut first=None;let mut first_text=None;
  loop{let n=res.read(&mut b).map_err(|_|"Lectura interrumpida; original parcial conservado")?;if n==0{break;}raw.write_all(&b[..n]).and_then(|_|raw.sync_data()).map_err(|e|e.to_string())?;total+=n;need(total<=(if p.proveedor=="Z.ai"{16}else{4})*1024*1024,"Respuesta excede límite")?;m.healthy()?;m.event("lectura_https",json!({"bytes":n,"acumulados":total,"ms":t.elapsed().as_millis()}))?;
   if status==200{let kinds=if p.proveedor=="Z.ai"{chat_stream.feed(&b[..n])?}else{stream.feed(&b[..n])?};for kind in kinds{first.get_or_insert(t.elapsed().as_millis());if kind=="response.output_text.delta"{first_text.get_or_insert(t.elapsed().as_millis());}m.event("evento_sse",json!({"tipo":kind,"ms":t.elapsed().as_millis()}))?;}}
  }raw.sync_all().map_err(|e|e.to_string())?;
  need(status==200,"HTTP de error; argumento original conservado")?;
  let mut received=if p.proveedor=="Z.ai"{save(&dest.join("EVENTOS.json"),&json!(chat_stream.eventos))?;chat_stream.recibir(&p.modelo)?}else{save(&dest.join("EVENTOS.json"),&json!(stream.eventos))?;stream.recibir(&p.modelo)?};
  put(&dest.join("FINAL.txt"),received["texto_original"].as_str().unwrap().as_bytes())?;
  received["duracion_ms"]=json!(t.elapsed().as_millis());received["primer_evento_ms"]=json!(first);received["primer_texto_ms"]=json!(first_text);
  received["zdr_confirmado"]=json!(headers.get("x-zero-data-retention").and_then(Value::as_str)==Some("true"));save(&dest.join("ENTREGA-PROVEEDOR.json"),&received)?;
  need(!p.exigir_zdr||received["zdr_confirmado"]==true,"Retención cero no confirmada por cabecera")?;Ok(received)
 })();
 let phase=m.event("fase",json!({"fase":"despues","entrega":op.is_ok()}));std::thread::sleep(Duration::from_secs(1));let measured=m.finish();let ok=phase.is_ok()&&measured.as_ref().is_ok_and(|v|v["fallos_medicion"]==0&&v["intervalo_maximo_ms"].as_u64().is_some_and(|n|n<=750));
 let v=json!({"completa":op.is_ok(),"entrega":op.as_ref().ok(),"error":op.err(),"telemetria_conforme":ok,"telemetria":measured.unwrap_or_else(|e|json!({"error":e})),"duracion_operacion_ms":t.elapsed().as_millis(),"inferencias_iniciadas":usize::from(dest.join("ENVIO.json").exists()),"herramientas_habilitadas":0});save(&dest.join("RESULTADO.json"),&v)?;Ok(v)
}

#[cfg(test)]mod tests{use super::*;
fn perfil()->Perfil{Perfil{proveedor:"xAI".into(),modelo:"grok-4.7".into(),endpoint:"https://api.x.ai/v1/responses".into(),presupuesto_ticks:50000000000,exigir_zdr:true,entrada_ticks_por_token:20000,salida_ticks_por_token:60000,cuota_gratuita_tokens:None}}
#[test]fn licencia_y_herramientas_cerradas(){let mut q=json!({"instructions":"Fuente exclusiva","tools":[{"type":"web_search"}],"store":true,"max_output_tokens":8192});proteger(&mut q,&perfil()).unwrap();assert!(q["instructions"].as_str().unwrap().contains(LICENCIA));assert_eq!(q["tools"],json!([]));assert_eq!(q["store"],false);assert!(reserva(&q,&perfil()).unwrap()>8192*60000);}
#[test]fn destino_fijo_y_contexto_acotado(){let mut p=perfil();p.endpoint="http://example.com".into();assert!(p.comprobar().is_err());assert!(reserva(&json!({"input":"x".repeat(200000),"max_output_tokens":8192}),&perfil()).is_err());}
#[test]fn rechaza_herramienta_y_json_duplicado(){let mut s=Flujo::default();assert!(s.feed(b"data: {\"type\":\"response.output_item.added\",\"item\":{\"type\":\"web_search_call\"}}\n").is_err());assert!(parse(b"{\"a\":1,\"a\":2}").is_err());}
#[test]fn cierre_y_texto_deben_concordar(){let mut s=Flujo::default();for v in [json!({"type":"response.output_text.delta","delta":"ok","sequence_number":1}),json!({"type":"response.completed","sequence_number":2,"response":{"model":"grok-4.7","status":"completed","output":[{"type":"message","role":"assistant","content":[{"type":"output_text","text":"ok"}]}]}})]{s.feed(format!("data: {v}\n\n").as_bytes()).unwrap();}assert_eq!(s.recibir("grok-4.7").unwrap()["texto_original"],"ok");assert!(s.recibir("otro").is_err());assert!(s.feed(b"data: {}\n").is_err());}
}
