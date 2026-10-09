#![forbid(unsafe_code)]
//! Recepción local del libro y prueba de adaptación. Nunca invoca enviar().
use sv_cliente_api::{self as api,need,parse,sha,save,Perfil,R};
use serde_json::json;
use std::{fs,path::{Path,PathBuf}};
#[path="../manual/suministro.rs"] mod suministro;
#[path="../manual/contrato.rs"] mod contrato;
#[path="../manual/localizadores.rs"] mod localizadores;
mod suministro_pdf {
 pub use sv_cliente_api::{parse,need,sha,R}; use serde_json::Value;
 pub fn num(v:&Value)->R<usize>{v.as_u64().and_then(|x|usize::try_from(x).ok()).ok_or("Entero requerido".into())}
 pub fn contenido(v:&Value,id:usize)->R<Value>{need(v["jsonrpc"]=="2.0"&&v["id"]==id&&v.get("error").is_none(),"Identidad RPC discordante")?;let r=&v["result"];need(r["isError"]==false&&r["content"].as_array().map(Vec::len)==Some(1)&&r["content"][0]["type"]=="text","Recepción MCP inválida")?;let t=parse(r["content"][0]["text"].as_str().ok_or("Texto ausente")?.as_bytes())?;need(t==r["structuredContent"],"Representaciones RPC discordantes")?;Ok(t)}
}
fn read(p:&Path)->R<Vec<u8>>{api::guard(p)?;let m=fs::metadata(p).map_err(|e|e.to_string())?;need(m.is_file()&&m.len()<=2*1024*1024,"Archivo fuera de cota")?;fs::read(p).map_err(|e|e.to_string())}
fn run()->R<()> {
 let a:Vec<String>=std::env::args().collect();need(a.len()==4,"Uso: sv-preparar-libro-api RAIZ SHA256_CONTRATO preparar|verificar")?;
 let root=PathBuf::from(&a[1]);api::guard(&root)?;
 let config=read(&root.join("CONTRATO-LIBRO.json"))?;need(sha(&config)==a[2],"Contrato no recibido")?;
 let cfg:suministro::ContratoLibro=serde_json::from_value(parse(&config)?).map_err(|e|e.to_string())?;cfg.comprobar()?;
 match a[3].as_str(){"preparar"=>{suministro::preparar_contrato(&root,&cfg)?;},"verificar"=>{},_=>return Err("Acción no recibida".into())};
 suministro::verificar_contrato(&root,&cfg)?;
 let p:Perfil=serde_json::from_value(parse(&read(&root.join("PERFIL-PREPARACION.json"))?)?).map_err(|e|e.to_string())?;
 need(p.proveedor=="Z.ai"&&p.modelo=="glm-5.3"&&p.presupuesto_ticks==0,"Perfil ajeno a la preparación sin envío")?;
 let bank=parse(&read(&root.join("fuentes/BANCO.json"))?)?;
 let key=parse(&read(&root.join("reservado/CLAVE.json"))?)?;
 let questions=bank["preguntas"].as_array().ok_or("Banco ausente")?;
 let keys=key["preguntas"].as_array().ok_or("Clave ausente")?;
 need(keys.len()==questions.len(),"Clave incompleta")?;
 let mut records=vec![];
 for (i,q) in questions.iter().enumerate() {
  need(keys[i]["id"]==q["id"]&&keys[i]["critica"].is_boolean(),"Clave desordenada")?;
  let id=q["id"].as_str().ok_or("ID")?;
  let base=parse(&read(&root.join(format!("fuentes-admitidas/{id}.json")))?)?;
  let original=base["input"][0]["content"].as_str().ok_or("Contenido")?;
  let source=parse(original.as_bytes())?;
  let source_fields=source.as_object().ok_or("Fuente no estructurada")?;
  need(source_fields.len()==4&&["caso","pregunta","secciones","fragmentos_documentales_completos"].iter().all(|k|source_fields.contains_key(*k)),"Campos del evaluador en candidato")?;
  let mut max_bytes=0;let mut reserve=0;
  for stage in 0..3 {
   // Antecedentes artificiales para comprobar la composición; no son respuestas GLM.
   let history=(0..stage).map(|i|format!("ANTECEDENTE SINTÉTICO DE PRUEBA LOCAL {i}: no atribuir al candidato")).collect::<Vec<_>>();
   let mut request=contrato::compose_documental(&base,stage,&history)?;
   need(request["input"][0]==base["input"][0],"Fuente alterada entre etapas")?;
   let visible=request["input"].as_array().unwrap().iter().filter(|m|m["role"]=="assistant").map(|m|m["content"].as_str().unwrap().to_owned()).collect::<Vec<_>>();
   need(visible==history,"Historia alterada")?;
   api::proteger(&mut request,&p)?;api::chat::validar(&request,&p.modelo)?;
   need(request["messages"][1]["content"]==original,"Fuente alterada en adaptación")?;
   let bytes=serde_json::to_vec(&request).map_err(|e|e.to_string())?;
   max_bytes=max_bytes.max(bytes.len());reserve=reserve.max(api::reserva(&request,&p)?);
  }
  records.push(json!({"caso":id,"etapas_comprobadas":3,"bytes_maximos_sinteticos":max_bytes,"reserva_estimada_ticks":reserve,"inferencias":0,"fuente_conservada":true,"historia_integra":true}));
 }
 let result=json!({"conforme":true,"tipo":"preparacion_local_sin_inferencia","casos":records,"criticos":keys.iter().filter(|v|v["critica"]==true).count(),"contrato_sha256":sha(&config),"cliente_sha256":sha(&fs::read(std::env::current_exe().map_err(|e|e.to_string())?).map_err(|e|e.to_string())?),"credencial_leida":false,"inferencias":0,"bytes_http_enviados":0,"coste_proveedor":null,"limite":"Los antecedentes de comprobación son sintéticos. No acredita aceptación HTTP, acceso efectivo, uso real ni aptitud de GLM-5.3"});
 if !root.join("preparacion/RECEPCION-LOCAL.json").exists(){save(&root.join("preparacion/RECEPCION-LOCAL.json"),&result)?;}
 println!("{}",json!({"conforme":true,"casos":questions.len(),"etapas_comprobadas":questions.len()*3,"criticos":result["criticos"],"inferencias":0}));Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1);}}
