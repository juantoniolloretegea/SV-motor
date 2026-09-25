//! Cotejo documental independiente del ejecutor; no realiza inferencias ni escribe originales.
use std::{collections::BTreeSet,fs,path::Path};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
type R<T> = Result<T,Box<dyn std::error::Error>>;
fn read(p:&Path)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn check(ok:bool,msg:&str)->R<()>{if ok{Ok(())}else{Err(msg.into())}}
fn main()->R<()>{
 let args:Vec<String>=std::env::args().collect();check(args.len()==2,"Se requiere directorio de campaña")?;
 let root=Path::new(&args[1]);let dir=root.join("ejecucion");
 let input=read(&root.join("ENTRADAS.json"))?;let frozen=read(&dir.join("ENTRADAS.json"))?;
 check(input==frozen,"Entradas de ejecución distintas")?;
 let final_case=read(&dir.join("EXPEDIENTE-FINAL.json"))?;
 let chats=final_case["chats"].as_array().ok_or("Conversaciones ausentes")?;
 check(chats.len()==12,"Cardinalidad de conversaciones distinta")?;
 let rows:Vec<Value>=fs::read_to_string(dir.join("RESULTADOS.jsonl"))?.lines().map(serde_json::from_str).collect::<Result<_,_>>()?;
 check(rows.len()==12,"Cardinalidad de resultados distinta")?;
 let mut ids=BTreeSet::new();let mut chat_ids=BTreeSet::new();let mut report=Vec::new();
 for q in input["questions"].as_array().ok_or("Consultas ausentes")?{
  let id=q["id"].as_str().ok_or("Identificador ausente")?;
  check(ids.insert(id),"Identificador duplicado")?;
  let row=read(&dir.join(format!("{id}-RESPUESTA.json")))?;
  let req=read(&dir.join(format!("{id}-PETICION.json")))?;
  let prev=read(&dir.join(format!("{id}-PREVIA.json")))?;
  let adm=read(&dir.join(format!("{id}-ADMISION.json")))?;
  let t=&row["turn"];let c=&t["context"];let r=&t["result"];
  check(row["question"]==*q && row["id"]==q["id"],"Pregunta distinta")?;
  check(rows.iter().filter(|x|**x==row).count()==1,"Resultado no único en JSONL")?;
  check(req["text"]==q["text"]&&t["user"]==q["text"],"Texto de petición distinto")?;
  check(req["profile"]==input["profile"]&&t["profile"]==input["profile"],"Perfil distinto")?;
  check(prev["fits"]==true&&prev["context"]==*c,"Contexto distinto de vista previa")?;
  let prompt=c["prompt"].as_str().ok_or("Contexto ausente")?;
  check(c["sha256"]==hash(prompt.as_bytes())&&req["context_sha256"]==c["sha256"],"Huella de contexto distinta")?;
  check(c["messages"].as_array().is_some_and(|x|x.is_empty()),"Historia ajena presente")?;
  check(prompt.contains(q["text"].as_str().unwrap()),"Consulta ausente del contexto")?;
  check(c["token_ids"].as_array().map(|x|x.len() as u64)==c["input_tokens"].as_u64(),"Cardinalidad de tokens distinta")?;
  check(c["input_tokens"].as_u64().unwrap()+768<=4096&&c["reserved_output"]==768,"Presupuesto de contexto inválido")?;
  check(t["id"]==req["request_id"]&&r["request_id"]==t["id"],"Correlación de petición distinta")?;
  let chat=req["chat_id"].as_str().ok_or("Chat ausente")?;check(chat_ids.insert(chat.to_owned()),"Chat reutilizado")?;
  check(r["chat_id"]==chat,"Correlación de conversación distinta")?;
  let saved=chats.iter().find(|x|x["id"]==chat).ok_or("Chat no exportado")?;
  check(saved["turns"].as_array().is_some_and(|x|x.len()==1&&x[0]==*t),"Exportación distinta")?;
  let engine=&r["phase_timings"]["engine_response"];let choice=&engine["choices"][0];
  check(t["answer"]==r["answer"]&&t["raw"]==choice["text"],"Respuesta distinta de salida del motor")?;
  check(engine["usage"]["prompt_tokens"]==c["input_tokens"]&&engine["usage"]["completion_tokens"]==r["tokens"],"Recuentos distintos")?;
  let normal=t["status"]=="fin_normal";let limited=t["status"]=="limite_generacion";
  check(normal||limited,"Terminación técnica inesperada")?;
  check((normal&&choice["finish_reason"]=="stop")||(limited&&choice["finish_reason"]=="length"),"Causa de terminación distinta")?;
  check(r["error"].is_null()&&r["exit_code"]==0&&r["termination"]["stop_confirmed"]==true,"Cierre técnico no conforme")?;
  check(r["observability"]["export"]["ok"]==true&&r["observability"]["export"]["errors"]==0&&r["observability"]["export"]["dropped"]==0,"Exportación instrumental no conforme")?;
  let words=t["answer"].as_str().unwrap().split_whitespace().count();
  report.push(json!({"id":id,"request_id":t["id"],"chat_id":chat,"cotejo":"conforme","estado":t["status"],"tokens_entrada":c["input_tokens"],"tokens_salida":r["tokens"],"segundos":r["seconds"],"palabras_separadas_por_espacio":words,"supera_230_palabras":words>230,"respuesta_sha256":hash(t["answer"].as_str().unwrap().as_bytes()),"archivo_respuesta_sha256":hash(&fs::read(dir.join(format!("{id}-RESPUESTA.json")))?),"admision":adm}));
 }
 let events=final_case["events"].as_array().ok_or("Sucesos ausentes")?;let mut previous=String::new();
 for (i,event) in events.iter().enumerate(){
  check(event["seq"]==(i+1) as u64&&event["previous_sha256"]==previous,"Secuencia o enlace alterado")?;
  let mut canonical=event.clone();canonical.as_object_mut().ok_or("Suceso inválido")?.remove("sha256");
  check(event["sha256"]==hash(&serde_json::to_vec(&canonical)?),"Huella de suceso no conforme")?;
  previous=event["sha256"].as_str().ok_or("Huella ausente")?.into();
 }
 let sum=read(&dir.join("RESUMEN.json"))?;check(sum["rows"]==12&&sum["error"].is_null(),"Resumen no conforme")?;
 println!("{}",serde_json::to_string_pretty(&json!({"schema":"SV-HCL12-COTEJO-CIERRE-1","resultado":"conforme","consultas":report,"conversaciones":chats.len(),"sucesos":events.len(),"ultimo_sha256":previous,"expediente_sha256":hash(&fs::read(dir.join("EXPEDIENTE-FINAL.json"))?),"semilla":"Solicitada y conservada; el adaptador no la envía al motor. No acredita determinismo.","alcance":"Cotejo de archivos, entradas, contextos, respuestas, exportación y cadena local. No reejecuta inferencias ni acredita exactitud clínica, autenticidad externa o ausencia de eliminación de sufijos completos."}))?);
 Ok(())
}
