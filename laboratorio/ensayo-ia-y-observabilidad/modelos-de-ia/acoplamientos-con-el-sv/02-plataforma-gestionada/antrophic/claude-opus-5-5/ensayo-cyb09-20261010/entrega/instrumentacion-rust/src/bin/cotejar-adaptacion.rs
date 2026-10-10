//! Cotejo de la corrección del transporte; no ejecuta inferencia.
#![forbid(unsafe_code)]
use std::{fs,path::Path,io::Write};
use sv_claude_kaggle::{self as sv,need,parse,R};
use serde_json::{json,Value};
fn run()->R<Value>{let a=std::env::args().collect::<Vec<_>>();need(a.len()==4,"PAQUETE INTENTO SALIDA")?;
 let paquete=parse(&fs::read(&a[1]).map_err(|e|e.to_string())?)?;let d=Path::new(&a[2]);
 let(q,w)=sv::compose(&paquete["bancos"][0]["casos"][0]["base"],0,&[])?;
 let anterior_q=parse(&fs::read(d.join("manual/MD01-R0/SOLICITUD-DOCUMENTAL.json")).map_err(|e|e.to_string())?)?;
 let mut anterior_w=parse(&fs::read(d.join("manual/MD01-R0/SOLICITUD.json")).map_err(|e|e.to_string())?)?;
 need(q==anterior_q,"Se ha alterado el contrato documental")?;
 need(anterior_w.as_object_mut().ok_or("Petición")?.remove("response_format").is_some(),"Campo original ausente")?;
 need(w==anterior_w,"Cambio adicional al campo de transporte delimitado")?;
 let r=parse(&fs::read(d.join("manual/MD01-R0/RESULTADO.json")).map_err(|e|e.to_string())?)?;
 let tel=sv::instrumentacion::verify(&d.join("manual/MD01-R0/instrumentacion/telemetria.jsonl"))?;
 need(tel==r["telemetria"]&&tel["fallos_medicion"]==0,"Originales instrumentales discordantes")?;
 let body=parse(&fs::read(d.join("manual/MD01-R0/RESPUESTA-HTTP.json")).map_err(|e|e.to_string())?)?;
 let http=parse(&fs::read(d.join("manual/MD01-R0/HTTP.json")).map_err(|e|e.to_string())?)?;
 need(http["status"]==400&&body["message"]=="failed to convert config"&&r["coste_nanodolares"].is_null(),"Incidencia original distinta")?;
 let result=json!({"conforme":true,"contrato_documental_identico":true,"cambio_exclusivo":"omisión de response_format; esquema íntegro conservado en las instrucciones y validación Rust","fuente":"Kaggle/kaggle-benchmarks@01183088103937114b723b82b8d5ea0c7a87ccce src/kaggle_benchmarks/actors/proxy_openai.py, adaptación de modelos anidados","telemetria_original_cotejada":tel,"respuesta_http":body,"coste_intento_01":null,"intentos_sin_contadores":1,"respuesta_del_modelo":false,"inferencia_efectiva":"no acreditada","nucleo_semantica_ir":"sin modificación","licencia":sv::LICENCIA});
 let bytes=serde_json::to_vec_pretty(&result).map_err(|e|e.to_string())?;let mut f=fs::OpenOptions::new().create_new(true).write(true).open(&a[3]).map_err(|e|e.to_string())?;f.write_all(&bytes).and_then(|_|f.sync_all()).map_err(|e|e.to_string())?;Ok(result)}
fn main(){match run(){Ok(_)=>println!("Corrección delimitada cotejada: contrato idéntico, sólo omisión de configuración anidada; originales íntegros"),Err(e)=>{eprintln!("IMPEDIMENTO: {e}");std::process::exit(1)}}}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
