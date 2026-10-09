#![forbid(unsafe_code)]
use sv_cliente_api::{parse,need,save,sha,Perfil,R};
use serde_json::{json,Value};
use std::{fs,path::PathBuf};
fn main(){let r=(||->R<()>{
 let a=std::env::args().collect::<Vec<_>>();need(a.len()==3,"Uso: INTENTO PERFIL")?;
 let dir=PathBuf::from(&a[1]);sv_cliente_api::guard(&dir)?;
 let p:Perfil=serde_json::from_value(parse(&fs::read(&a[2]).map_err(|e|e.to_string())?)?).map_err(|e|e.to_string())?;
 let raw=fs::read(dir.join("SALIDA-SSE.txt")).map_err(|e|e.to_string())?;
 let mut id=None;let mut done=false;let mut terminal=false;let mut usage=Value::Null;let mut events=0;let mut text=String::new();
 for line in std::str::from_utf8(&raw).map_err(|_|"UTF8")?.lines(){let line=line.trim();if line.is_empty(){continue;}need(!done,"Posterior a DONE")?;let data=line.strip_prefix("data:").ok_or("Campo SSE")?.trim();if data=="[DONE]"{need(terminal,"Sin terminación")?;done=true;continue;}
  let v=parse(data.as_bytes())?;need(!terminal && v["model"]==p.modelo && v.get("error").is_none() && v.get("web_search").is_none(),"Evento ajeno")?;
  if let Some(old)=&id{need(old==&v["id"],"Identidad discordante")?;}else{id=Some(v["id"].clone());}
  let c=v["choices"].as_array().filter(|c|c.len()==1).ok_or("Alternativas")?;let c=&c[0];need(c["index"]==0,"Índice")?;
  let d=c["delta"].as_object().ok_or("Delta")?;need(d.keys().all(|k|["role","content","reasoning_content"].contains(&k.as_str())),"Delta ajeno")?;
  if let Some(t)=c["delta"]["content"].as_str(){text.push_str(t);}
  if let Some(f)=c["finish_reason"].as_str(){need(f=="length","No es límite de salida")?;terminal=true;usage=v["usage"].clone();}
  events+=1;
 }
 need(done&&terminal,"SSE incompleto")?;let estimate=sv_cliente_api::presupuesto::estimacion_chat(&usage,&p)?;
 let t=sv_instrumentacion::verify(&dir.join("instrumentacion/telemetria.jsonl"))?;
 let result=json!({"estado":"sin_respuesta_final_por_limite_de_salida","valido_para_calificar":false,"finish_reason":"length","texto_visible_bytes":text.len(),"eventos":events,"sse_sha256":sha(&raw),"sse_bytes":raw.len(),"uso_proveedor":usage,"estimacion_tarifaria":estimate,"telemetria":t,"cargo_liquidado_usd":null,"licencia":sv_cliente_api::LICENCIA});
 save(&dir.join("RECEPCION-LIMITE-RUST.json"),&result)?;println!("{}",serde_json::to_string_pretty(&result).unwrap());Ok(())})();if let Err(e)=r{eprintln!("{e}");std::process::exit(1)}}
