//! Recibo del intento interrumpido; no realiza inferencia ni adjudica al candidato.
#![forbid(unsafe_code)]
#[path="../catalogo/estricto.rs"]mod estricto;
use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,io::Write,path::Path};
fn run()->Result<(),Box<dyn std::error::Error>>{
 let root=Path::new("./ejecucion/astra-examen25-20261007");let p=root.join("originales/P01/R0");
 let r:Value=serde_json::from_slice(&fs::read(p.join("RESULTADO.json"))?)?;let t=sv_instrumentacion::verify(&p.join("instrumentacion/telemetria.jsonl"))?;
 assert_eq!(t,r["telemetria"]);assert_eq!(r["completa"],false);assert!(r["uso_proveedor"].is_null());
 let raw=fs::read(p.join("SALIDA-SSE.txt"))?;let mut types=vec![];
 for (i,line) in raw.split(|b|*b==b'\n').filter_map(|b|b.strip_prefix(b"data:")).enumerate(){let v=estricto::parse(line)?;assert_eq!(v["sequence_number"],i);types.push(v["type"].as_str().ok_or("Tipo")?.to_owned());}
 assert_eq!(types,["response.created","response.in_progress","keepalive"]);assert!(!p.join("FINAL.txt").exists());
 let bank:Value=serde_json::from_slice(&fs::read(root.join("RESULTADO-BANCO.json"))?)?;assert_eq!(bank["inferencias_iniciadas"],1);
 let http:Value=serde_json::from_slice(&fs::read(p.join("HTTP.json"))?)?;assert_eq!(http["status"],200);
 let v=json!({"conforme":true,"tipo":"incidencia_instrumental_local","caso":"P01-R0","intentos":1,"http":200,"respuesta_completa":false,"calificacion":null,"causa":"Receptor local no admitía keepalive; el mensaje observado sólo contiene tipo y número de secuencia","duracion_banco_ms":bank["duracion_banco_ms"],"tipos_recibidos":types,"sse_sha256":format!("{:x}",Sha256::digest(&raw)),"telemetria":t,"uso":{"entrada_tokens":null,"salida_tokens":null,"total_tokens":null},"creditos":null,"importe":null,"estado_remoto":"No consta terminación ni uso; conexión local cerrada","consecuencia":"Conservar intento y consumo desconocido; no adjudicar 0, 1 o U","reintento_automatico":false,"correccion":"Revisión r2 admite mantenimiento estricto, sin ampliar plazos ni cambiar preguntas o instrucciones","fuente_original_conservada":true});
 let mut f=fs::OpenOptions::new().create_new(true).write(true).open(root.join("INCIDENCIA-RUST.json"))?;f.write_all(&serde_json::to_vec_pretty(&v)?)?;f.sync_all()?;println!("Recibo instrumental conforme; consumo no determinado.");Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1)}}
