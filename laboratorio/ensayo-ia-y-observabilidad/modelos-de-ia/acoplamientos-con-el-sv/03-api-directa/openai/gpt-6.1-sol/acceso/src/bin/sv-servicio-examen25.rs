//! Registro de servicio y atribución de consumos a partir de evidencia conservada.
#![forbid(unsafe_code)]
use serde_json::{json,Value};use std::{fs,path::Path,io::Write};use sha2::{Digest,Sha256};
type R<T>=Result<T,String>;
fn e(v:impl std::fmt::Display)->String{v.to_string()}
fn load(p:&Path)->R<Value>{serde_json::from_slice(&fs::read(p).map_err(e)?).map_err(e)}
fn need(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn sha(p:&Path)->R<String>{Ok(format!("{:x}",Sha256::digest(&fs::read(p).map_err(e)?)))}
fn run()->R<()>{let root=Path::new(".");let base=root.join("ejecucion/astra-examen25-conjunto-20261007");let met=load(&base.join("METRICAS-RUST.json"))?;let proof=load(&base.join("COTEJO-BANCO-RUST.json"))?;need(met["conforme"]==true&&proof["conforme"]==true,"Sin cotejo")?;
 let mut paths=vec![("r1".to_string(),root.join("ejecucion/astra-examen25-20261007/originales/P01/R0"))];
 for n in 1..=43{paths.push(("r2".into(),root.join(format!("ejecucion/astra-examen25-r2-20261007/originales/P{:02}/R{}",(n-1)/3+1,(n-1)%3))));}
 let r3=root.join("ejecucion/astra-examen25-r3-20261007");let bank=load(&r3.join("RESULTADO-BANCO.json"))?;
 for i in 1..=bank["inferencias_iniciadas"].as_u64().ok_or("Intentos")?{let h=load(&r3.join(format!("hitos/INTENTO-{i:03}/HITO-INSTRUMENTAL.json")))?;paths.push(("r3".into(),root.join(h["directorio"].as_str().ok_or("Ruta")?)));}
 let mut rows=vec![];
 for (rev,p) in paths {let rr=load(&p.join("RESULTADO.json"))?;let t=sv_instrumentacion::verify(&p.join("instrumentacion/telemetria.jsonl"))?;need(t==rr["telemetria"],"Medición distinta")?;
  let log=fs::read_to_string(p.join("instrumentacion/telemetria.jsonl")).map_err(e)?.lines().map(|s|{let a:Value=serde_json::from_str(s).map_err(e)?;serde_json::from_str::<Value>(a["cuerpo"].as_str().ok_or("Cuerpo")?).map_err(e)}).collect::<R<Vec<_>>>()?;
  let a=log.iter().find(|v|v["tipo"]=="fase"&&v["datos"]["fase"]=="durante").ok_or("Envío sin fase")?;
  let z=log.iter().find(|v|v["tipo"]=="fase"&&v["datos"]["fase"]=="despues").ok_or("Sin cierre")?;
  let http=load(&p.join("HTTP.json"))?;let complete=rr["completa"]==true;let raw=fs::read_to_string(p.join("SALIDA-SSE.txt")).map_err(e)?;
  let ev=raw.lines().filter_map(|s|s.strip_prefix("data:")).filter_map(|s|serde_json::from_str::<Value>(s).ok()).collect::<Vec<_>>();
  let failure=ev.iter().find(|v|v["type"]=="response.failed");let error=failure.map(|v|v["response"]["error"].clone()).unwrap_or_else(||rr["incidencia_servicio"].clone());
  let terminal=ev.last().map(|v|v["type"].clone()).unwrap_or(Value::Null);
  let m=if complete{met["casos"].as_array().ok_or("Mediciones")?.iter().find(|v|v["caso"]==rr["caso"]).ok_or("Sin medición atribuible")?.clone()}else{json!({"caso":rr["caso"],"uso":{"entrada_tokens":null,"salida_tokens":null,"total_tokens":null},"telemetria":t})};
  let receipt=log.iter().find(|v|v["tipo"]=="evento_sse"&&v["datos"]["tipo"]=="error");
  rows.push(json!({"numero":rows.len()+1,"revision":rev,"caso":rr["caso"],"proveedor":"OpenAI","completa":complete,"http":http["status"],"evento_terminal":terminal,"inicio_envio_utc_ms":a["utc_unix_ms"],"fin_operacion_utc_ms":z["utc_unix_ms"],"duracion_operacion_ms":z["transcurrido_ms"].as_u64().ok_or("Tiempo")?.checked_sub(a["transcurrido_ms"].as_u64().ok_or("Tiempo")?).ok_or("Reloj regresivo")?,"latencia_error_ms":receipt.map(|v|v["datos"]["ms"].clone()),"error_proveedor":error,"causa_receptor_local":rr["causa"],"medicion":m,"directorio":p.strip_prefix(root).map_err(e)?.to_string_lossy(),"sse_sha256":sha(&p.join("SALIDA-SSE.txt"))?,"resultado_sha256":sha(&p.join("RESULTADO.json"))?,"credito_atribuible":null,"importe_atribuible":null}));
 }
 let interrupted:Vec<_>=rows.iter().filter(|r|r["completa"]!=true).collect();
 let public=json!({"proveedor":"OpenAI","modelo":"gpt-6-astra","intentos":rows.len(),"entregas_completas":rows.len()-interrupted.len(),"intentos_sin_entrega":interrupted.len(),"incidencias":interrupted.iter().map(|r|json!({"revision":r["revision"],"caso":r["caso"],"http":r["http"],"evento_terminal":r["evento_terminal"],"inicio_envio_utc_ms":r["inicio_envio_utc_ms"],"fin_operacion_utc_ms":r["fin_operacion_utc_ms"],"duracion_operacion_ms":r["duracion_operacion_ms"],"latencia_error_ms":r["latencia_error_ms"],"error_proveedor":r["error_proveedor"],"causa_receptor_local":r["causa_receptor_local"],"sse_sha256":r["sse_sha256"],"respuesta_entregada":false,"uso_proveedor":null})).collect::<Vec<_>>(),"periodos_observados_r3":bank["periodos_servicio"],"estado_examen":bank["estado"],"duracion_total_caida_proveedor":null,"limite":"Las pausas locales y los intervalos sin peticiones no demuestran indisponibilidad continua del proveedor","puntuacion_proveedor":null,"conformidad_ISO":"No evaluada; referencias y alcance en la adenda de continuidad"});
 for (name,v) in [("INTENTOS-CONSUMO-RUST.json",json!({"conforme":true,"casos":rows,"tokens_conocidos":met["totales"],"sin_inferencia_nueva":true})),("SERVICIO-PROVEEDOR-RUST.json",public)]{let mut f=fs::OpenOptions::new().write(true).create_new(true).open(base.join(name)).map_err(e)?;f.write_all(&serde_json::to_vec_pretty(&v).map_err(e)?).and_then(|_|f.sync_all()).map_err(e)?;}
 println!("Registro de servicio y consumo conforme");Ok(())}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1)}}
