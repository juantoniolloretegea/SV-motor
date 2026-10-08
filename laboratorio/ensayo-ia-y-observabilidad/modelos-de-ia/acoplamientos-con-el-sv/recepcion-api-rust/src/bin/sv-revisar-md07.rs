//! Diagnóstico local de MD07; sin conexión, reparación de originales ni recalificación.
#![forbid(unsafe_code)]
use serde_json::{json,Value};
use std::{fs,path::Path};
use sv_cliente_api::{need,parse,sha,R};
mod suministro_pdf { pub use sv_cliente_api::{need,parse,sha,R}; pub fn num(v:&serde_json::Value)->R<usize>{v.as_u64().and_then(|n|usize::try_from(n).ok()).ok_or("Entero requerido".into())} }
#[path="../../../cliente-api-rust/src/manual/localizadores.rs"] mod localizadores;
#[path="../../../cliente-api-rust/src/manual/contrato.rs"] mod contrato;
fn raw(p:&Path)->R<Vec<u8>>{sv_cliente_api::guard(p)?;fs::read(p).map_err(|e|e.to_string())}
fn load(p:&Path)->R<Value>{parse(&raw(p)?)}
fn main(){let result=(||->R<()>{
 let args=std::env::args().collect::<Vec<_>>();need(args.len()==3,"Raíz original y salida diagnóstica")?;let root=Path::new(&args[1]);let out=Path::new(&args[2]);
 let receipt=load(&root.join("RECEPCION-RUST.json"))?;let source_base=load(&root.join("fuentes-admitidas/MD07.json"))?;
 let mut rows=vec![];let mut originals=vec![];let mut history=vec![];let mut final_answer=Value::Null;let mut final_q=Value::Null;
 for stage in 0..3 {let dir=root.join(format!("intentos/I{:03}-MD07-R{stage}",19+stage));
  for name in ["SOLICITUD.json","SALIDA-SSE.txt","FINAL.txt","AUDITORIA-FORMAL.json","HTTP.json","RESULTADO.json"]{let path=dir.join(name);originals.push((path.clone(),sha(&raw(&path)?)));}
  let req=load(&dir.join("SOLICITUD.json"))?;sv_cliente_api::chat::validar(&req,"glm-5.3")?;
  need(req["messages"][1]["content"]==source_base["input"][0]["content"],"Fuente alterada en transporte")?;
  let sys=req["messages"][0]["content"].as_str().ok_or("Instrucciones")?;need(sys.contains("Use citas breves exactas")&&sys.contains("sólo las citas de apoyo deben ser literales"),"Contrato diferente")?;
  let messages=req["messages"].as_array().ok_or("Mensajes")?;let prior=messages.iter().filter(|m|m["role"]=="assistant").map(|m|m["content"].as_str().unwrap().to_string()).collect::<Vec<_>>();need(prior==history,"Antecedentes modificados")?;
  let q=json!({"input":&messages[1..]});let source=parse(messages[1]["content"].as_str().unwrap().as_bytes())?;
  let src=source["fragmentos_documentales_completos"].as_array().unwrap().iter().find(|s|s["documento"]=="MANUAL_CONSTRUCTOR"&&s["seccion"]=="MD-L000009-L000017").ok_or("Sección ausente")?;
  let exact="- subordinado al pliego, a la Frontera normativa, a la IR y a la gramática vigente;";need(src["texto"].as_str().unwrap().contains(exact),"Fuente inesperada")?;
  let bytes=raw(&dir.join("FINAL.txt"))?;let answer=parse(&bytes)?;let mut stream=sv_cliente_api::chat::Flujo::default();stream.feed(&raw(&dir.join("SALIDA-SSE.txt"))?)?;let got=stream.recibir("glm-5.3")?;
  need(got["texto_original"].as_str().unwrap().as_bytes()==bytes,"SSE y texto final diferentes")?;
  let old=receipt["casos"].as_array().unwrap().iter().find(|c|c["caso"]=="MD07"&&c["etapa"]==stage).ok_or("Recepción ausente")?;need(old["final_sha256"]==sha(&bytes),"Identidad final diferente")?;
  let formal=contrato::formal(&answer,&q);need(formal.is_ok()==old["auditoria_formal"]["conforme"].as_bool().unwrap(),"Resultado formal cambiado")?;
  let mut citations=vec![];for (i,c) in answer["evidencias"].as_array().unwrap().iter().enumerate(){let mut single=answer.clone();single["evidencias"]=json!([c]);let test=contrato::formal(&single,&q);citations.push(json!({"indice":i,"documento":c["documento"],"seccion":c["seccion"],"cita":c["cita_literal_breve"],"conforme":test.is_ok(),"error":test.err()}));}
  rows.push(json!({"etapa":stage,"http":load(&dir.join("HTTP.json"))?["status"],"modelo":got["modelo_proveedor"],"terminacion":"stop y DONE comprobados por receptor","final_sha256":sha(&bytes),"sse_sha256":sha(&raw(&dir.join("SALIDA-SSE.txt"))?),"solicitud_sha256":sha(&raw(&dir.join("SOLICITUD.json"))?),"fuente_y_antecedentes_integros":true,"fuente_literal":exact,"reconstruccion_sse_identica":true,"formal_conforme":formal.is_ok(),"citas":citations,"plural_fuera_de_citas":answer["fundamentos_verificables"].as_array().unwrap().iter().any(|v|v.as_str().unwrap().contains("gramática vigentes")),"uso_original":got["uso_normalizado"]}));
  history.push(String::from_utf8(bytes).map_err(|e|e.to_string())?);if stage==2 {final_answer=answer;final_q=q;}
 }
 let cites=final_answer["evidencias"].as_array().unwrap();let wrong="- subordinado al pliego, a la Frontera normativa, a la IR y a la gramática vigentes;";let pos=cites.iter().position(|c|c["cita_literal_breve"]==wrong).ok_or("Discrepancia no encontrada")?;
 need(!history[..2].iter().any(|s|parse(s.as_bytes()).unwrap()["evidencias"].as_array().unwrap().iter().any(|c|c["seccion"]=="MD-L000009-L000017")),"Cita ya presente")?;
 let mut omit=final_answer.clone();omit["evidencias"].as_array_mut().unwrap().remove(pos);let mut exact=final_answer.clone();exact["evidencias"][pos]["cita_literal_breve"]=json!(wrong.replace("vigentes;","vigente;"));
 need(contrato::formal(&omit,&final_q).is_ok()&&contrato::formal(&exact,&final_q).is_ok(),"La discrepancia no es causa formal única")?;
 for(p,h)in &originals{need(sha(&raw(p)?)==*h,"Original modificado durante diagnóstico")?;}
 let report=json!({"conforme":true,"objeto":"Diagnóstico MD07 sobre originales; sin inferencia ni recalificación","etapas":rows,"causa_formal_aislada":{"indice_cita":pos,"nueva_en_R2":true,"sin_cita_adicional_formal_conforme":true,"con_cita_exacta_formal_conforme":true,"alcance":"Variantes en memoria para aislar causa; no respuestas del candidato ni adjudicaciones"},"originales_cotejados_antes_y_despues":originals.len(),"originales_modificados":0,"llamadas_proveedor":0,"licencia":sv_cliente_api::LICENCIA});sv_cliente_api::save(out,&report)?;println!("Conforme: 3 etapas reconstruidas; 18 originales intactos; causa formal única aislada; cero llamadas");Ok(())})();if let Err(e)=result{eprintln!("{e}");std::process::exit(1)}}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
