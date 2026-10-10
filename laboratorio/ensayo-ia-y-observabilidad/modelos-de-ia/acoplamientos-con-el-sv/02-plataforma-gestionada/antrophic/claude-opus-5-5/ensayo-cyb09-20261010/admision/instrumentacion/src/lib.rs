#![forbid(unsafe_code)]
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
pub type R<T>=Result<T,String>;
pub fn need(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
pub fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
pub mod estricto;
pub fn parse(b:&[u8])->R<Value>{estricto::parse(b).map_err(|e|e.to_string())}
pub mod instrumentacion;
pub mod observabilidad_modelo;
pub mod contrato;
pub mod localizadores;
pub mod suministro;
pub mod suministro_pdf {
 pub use crate::{parse,need,sha,R};
 use serde_json::Value;
 pub fn num(v:&Value)->R<usize>{v.as_u64().and_then(|n|usize::try_from(n).ok()).ok_or("Entero requerido".into())}
 pub fn contenido(v:&Value,id:usize)->R<Value>{need(v["jsonrpc"]=="2.0"&&v["id"]==id&&v.get("error").is_none(),"Identidad RPC discordante")?;let r=&v["result"];need(r["isError"]==false&&r["content"].as_array().map(Vec::len)==Some(1)&&r["content"][0]["type"]=="text","Recepción MCP inválida")?;let text=parse(r["content"][0]["text"].as_str().ok_or("Texto ausente")?.as_bytes())?;need(text==r["structuredContent"],"Representaciones RPC discordantes")?;Ok(text)}
}
pub const MODELO:&str="anthropic/claude-opus-5-5@default";
pub const LICENCIA:&str="© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
pub const AVISO:&str="El material propio conserva sus derechos y sólo se autoriza su procesamiento para devolver la respuesta experimental. Las fuentes de terceros conservan su régimen original. No se autoriza entrenamiento, publicación, desarrollo de otros productos ni cesión de derechos.";
pub fn compose(base:&Value,stage:usize,history:&[String])->R<(Value,Value)>{
 let mut q=contrato::compose_documental(base,stage,history)?;
 let instructions=observabilidad_modelo::instrucciones(&format!("{}\n{}\n{}",q["instructions"].as_str().ok_or("Instrucciones ausentes")?,LICENCIA,AVISO));
 let mut schema=q["text"]["format"]["schema"].clone();observabilidad_modelo::incorporar_esquema(&mut schema)?;
 q["text"]["format"]["schema"]=schema.clone();
 q["instructions"]=json!(format!("{instructions}\nEntregue únicamente un objeto JSON válido, sin cercas, con este esquema íntegro: {schema}. El informe_operativo es una declaración separada; los campos científicos no cambian."));
 for message in q["input"].as_array_mut().ok_or("Entrada ausente")?{
  if message["role"]!="user"{continue;}let Some(text)=message["content"].as_str()else{continue;};
  if let Ok(mut record)=parse(text.as_bytes()){if record["tipo"]=="registro_de_instrucciones_historicas"{
   let s=num_stage(&record["etapa_historica"])?;need(s<stage,"Etapa futura")?;record["instrucciones_aplicadas"]=compose(base,s,&history[..s])?.0["instructions"].clone();message["content"]=json!(record.to_string());
  }}
 }
 observabilidad_modelo::comprobar_solicitud(&q)?;
 let mut messages=vec![json!({"role":"system","content":q["instructions"]})];messages.extend(q["input"].as_array().ok_or("Entrada")?.iter().cloned());
 // La biblioteca oficial de Kaggle desactiva el flujo en ModelProxy.
 // La fuente oficial proxy_openai.py omite response_format para modelos anidados.
 // El esquema íntegro ya consta en las instrucciones; la recepción lo comprueba.
 let wire=json!({"model":MODELO,"messages":messages,"max_tokens":16384,"reasoning_effort":"high","stream":false});
 need(!wire.to_string().to_lowercase().contains("gemini"),"Gemini excluido del contexto de inferencia")?;
 Ok((q,wire))
}
fn num_stage(v:&Value)->R<usize>{v.as_u64().filter(|n|*n<3).map(|n|n as usize).ok_or("Etapa inválida".into())}
fn validate(v:&Value,s:&Value)->R<()>{
 if let Some(e)=s["enum"].as_array(){need(e.contains(v),"Enumeración operativa discordante")?;}
 let types=s["type"].as_array().map(|a|a.iter().filter_map(Value::as_str).collect::<Vec<_>>()).unwrap_or_else(||vec![s["type"].as_str().unwrap_or("")]);
 need(types.iter().any(|t|match *t{"null"=>v.is_null(),"string"=>v.is_string(),"boolean"=>v.is_boolean(),"integer"=>v.is_u64(),"object"=>v.is_object(),"array"=>v.is_array(),_=>false}),"Tipo operativo discordante")?;
 if v.is_object(){let m=v.as_object().unwrap();let p=s["properties"].as_object().ok_or("Propiedades operativas ausentes")?;need(m.len()==p.len(),"Campos operativos ausentes o adicionales")?;for(k,t)in p{validate(m.get(k).ok_or("Campo operativo ausente")?,t)?;}}
 if let Some(a)=v.as_array(){for x in a{validate(x,&s["items"])?;}}Ok(())
}
pub fn formal(answer:&Value,q:&Value)->R<Value>{let mut scientific=answer.clone();let report=scientific.as_object_mut().ok_or("Respuesta no es objeto")?.remove("informe_operativo").ok_or("Falta informe operativo")?;validate(&report,&observabilidad_modelo::esquema())?;let mut r=contrato::formal(&scientific,q)?;r["informe_operativo_esquema_conforme"]=json!(true);Ok(r)}

#[cfg(test)]mod tests{
 use super::*;
 fn base()->Value{json!({"input":[{"role":"user","content":json!({"caso":"MD01","pregunta":"Q","secciones":[],"fragmentos_documentales_completos":[]}).to_string()}],"instructions":"Fuentes exclusivas","tools":[],"tool_choice":"none"})}
 #[test]fn protege_historia_modelo_y_peticion(){let b=base();let h=vec!["original R0".into(),"original R1".into()];let(q0,_)=compose(&b,0,&[]).unwrap();let(q2,w)=compose(&b,2,&h).unwrap();let historic=parse(q2["input"][1]["content"].as_str().unwrap().as_bytes()).unwrap();assert_eq!(historic["instrucciones_aplicadas"],q0["instructions"]);assert_eq!(w["model"],MODELO);assert_eq!(w["max_tokens"],16384);assert!(w.get("tools").is_none());assert_eq!(q2["input"][0],b["input"][0]);observabilidad_modelo::comprobar_solicitud(&w).unwrap();}
 #[test]fn rechaza_gemini_y_historia_incompleta(){let mut b=base();b["instructions"]=json!("Use Gemini");assert!(compose(&b,0,&[]).is_err());assert!(compose(&base(),2,&[]).is_err());}
 #[test]fn adapta_esquema_anidado_sin_cambiar_contrato(){let(q,w)=compose(&base(),0,&[]).unwrap();assert!(w.get("response_format").is_none());assert_eq!(w["messages"][0]["content"],q["instructions"]);assert!(q["instructions"].as_str().unwrap().contains(&q["text"]["format"]["schema"].to_string()));assert_eq!(w["stream"],false);assert_eq!(w["reasoning_effort"],"high");}
 #[test]fn informe_no_es_medicion_y_esquema_estricto(){let s=observabilidad_modelo::esquema();let mut v=json!({"procedimiento_resumido":"Consulta de la fuente","procesos_y_herramientas":[],"magnitudes":[{"nombre":"tokens","valor":null,"unidad":null,"origen":"no_disponible","evidencia":null}],"limites":["Sin contadores accesibles"]});validate(&v,&s).unwrap();v["magnitudes"][0]["valor"]=json!(42);assert!(validate(&v,&s).is_err());v["magnitudes"][0]["valor"]=Value::Null;v["extra"]=json!(0);assert!(validate(&v,&s).is_err());}
}
