//! Revisión contractual de los cuatro originales documentales; sin inferencia.
use std::{env,fs,collections::BTreeSet};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
type Result<T>=std::result::Result<T,Box<dyn std::error::Error>>;
fn main()->Result<()>{
 let a:Vec<_>=env::args().collect();if a.len()!=3{return Err("Uso: ENTRADAS RESULTADOS_JSONL".into())}
 let input:Value=serde_json::from_slice(&fs::read(&a[1])?)?;let raw=fs::read(&a[2])?;
 let records:Vec<Value>=std::str::from_utf8(&raw)?.lines().map(serde_json::from_str).collect::<std::result::Result<_,_>>()?;
 if records.len()!=4{return Err("Número documental distinto de cuatro".into())}
 let mut ids=BTreeSet::new();let mut rows=vec![];
 for r in records{
  let id=r["case"].as_str().ok_or("Sin condición")?;
  if !ids.insert(id.to_owned()){return Err("Condición duplicada".into())}
  let expected=match id{
   "DOC01"|"DOC04"=>json!({"estado":"documentado","fuente":"OP-IMM-001-P10@1.0","cita":"Compara un recuento absoluto de neutrófilos válido con el intervalo de referencia aplicable."}),
   "DOC02"|"DOC03"=>json!({"estado":"sin_respaldo","fuente":"","cita":""}),
   _=>return Err("Condición no prevista".into())
  };
  let candidates:Vec<_>=input["cases"].as_array().ok_or("Sin condiciones")?.iter().filter(|s|s["id"]==id).collect();
  if candidates.len()!=1{return Err("Entrada ausente o ambigua".into())}
  let source=candidates[0];let old=&source["qwen_result"];
  let text=r["response"]["choices"][0]["text"].as_str().ok_or("Sin respuesta")?;
  let qtext=old["result"]["raw"].as_str().ok_or("Sin respuesta Qwen")?;
  let conforms=r["response"]["choices"][0]["finish_reason"]=="stop"&&serde_json::from_str::<Value>(text).ok().as_ref()==Some(&expected);
  let qconforms=serde_json::from_str::<Value>(qtext).ok().as_ref()==Some(&expected);
  if r["accepted"]!=conforms||old["verification"]["accepted"]!=qconforms{return Err("Evaluación histórica divergente".into())}
  rows.push(json!({"id":id,"pregunta":source["user"],"esperado":expected,"gpt_oss":{"conforme":conforms,"respuesta":text,"segundos":r["seconds"]},"qwen":{"conforme":qconforms,"respuesta":qtext,"segundos":old["seconds"]}}));
 }
 let gpt=rows.iter().filter(|r|r["gpt_oss"]["conforme"]==true).count();
 let qwen=rows.iter().filter(|r|r["qwen"]["conforme"]==true).count();
 println!("{}",serde_json::to_string_pretty(&json!({"esquema":"EIO-AUDITORIA-DOCUMENTAL-RUST-1","condiciones":rows.len(),"gpt_oss_conformes":gpt,"qwen_conformes":qwen,"original_sha256":format!("{:x}",Sha256::digest(&raw)),"filas":rows,"alcance":"Cuatro objetos sobre OP-IMM-001-P10@1.0; conserva condiciones distintas; no evaluación clínica ni todo el universo."}))?);Ok(())
}

