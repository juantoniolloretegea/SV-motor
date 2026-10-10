#![forbid(unsafe_code)]
//! Recepción retrospectiva sin red; nunca vuelve a pedir una respuesta.
use sv_cliente_api::{self as api,need,parse,save,sha,R};
use serde_json::{json,Value};use std::{fs,path::Path};
fn load(p:&Path)->R<Value>{api::guard(p)?;parse(&fs::read(p).map_err(|e|e.to_string())?)}
fn copy(source:&Path,dest:&Path)->R<()>{api::guard(source)?;api::guard(dest)?;if source.is_dir(){fs::create_dir_all(dest).map_err(|e|e.to_string())?;for f in fs::read_dir(source).map_err(|e|e.to_string())?{let f=f.map_err(|e|e.to_string())?;let name=if f.file_name()=="RESULTADO.json"{"RESULTADO-INICIAL.json".into()}else if f.file_name()=="AUDITORIA-FORMAL.json"{"AUDITORIA-INICIAL.json".into()}else{f.file_name()};copy(&f.path(),&dest.join(name))?;}}else{api::put(dest,&fs::read(source).map_err(|e|e.to_string())?)?;}Ok(())}
fn run()->R<()>{let a=std::env::args().collect::<Vec<_>>();need(a.len()==3,"Origen y destino requeridos")?;let src=Path::new(&a[1]);let dest=Path::new(&a[2]);api::guard(src)?;api::guard(dest)?;need(!dest.exists(),"Destino ya existente")?;
 let original=load(&src.join("RESULTADO.json"))?;need(original["completa"]==false&&original["error"]=="Texto final y deltas discordantes"&&original["telemetria_conforme"]==true,"Incidencia distinta")?;
 let http=load(&src.join("HTTP.json"))?;need(http["status"]==200,"HTTP ajeno")?;
 let raw=fs::read(src.join("SALIDA-SSE.txt")).map_err(|e|e.to_string())?;let mut f=api::Flujo::default();f.feed(&raw)?;let received=f.recibir("gpt-6-astra")?;
 let t=sv_instrumentacion::verify(&src.join("instrumentacion/telemetria.jsonl"))?;need(t==original["telemetria"],"Telemetría distinta")?;
 copy(src,dest)?;api::put(&dest.join("FINAL.txt"),received["texto_original"].as_str().ok_or("Texto")?.as_bytes())?;save(&dest.join("ENTREGA-PROVEEDOR.json"),&received)?;
 let mut result=original.clone();result["completa"]=json!(true);result["error"]=Value::Null;result["entrega"]=received;
 result["recuperacion_sin_inferencia"]=json!({"original_resultado_sha256":sha(&fs::read(src.join("RESULTADO.json")).map_err(|e|e.to_string())?),"sse_sha256":sha(&raw),"causa":"El receptor común exigía response.completed.output no vacío; el protocolo concluyó el mensaje en output_item.done, con deltas y output_text.done concordantes","inferencias_nuevas":0,"original_conservado":"RESULTADO-INICIAL.json"});
 save(&dest.join("RESULTADO.json"),&result)?;save(&dest.join("RECUPERACION-RUST.json"),&json!({"conforme":true,"inferencia_nueva":false,"eventos":f.eventos.len(),"uso":result["entrega"]["uso_proveedor"],"sse_sha256":sha(&raw),"final_sha256":sha(result["entrega"]["texto_original"].as_str().unwrap().as_bytes()),"origen":src.strip_prefix("C:/SV-LABORATORIO").map_err(|_|"Origen")?.to_string_lossy(),"licencia":api::LICENCIA}))?;
 println!("Recepción recuperada en Rust: {} eventos; {} tokens; cero inferencias nuevas",f.eventos.len(),result["entrega"]["uso_proveedor"]["total_tokens"]);Ok(())}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1)}}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
