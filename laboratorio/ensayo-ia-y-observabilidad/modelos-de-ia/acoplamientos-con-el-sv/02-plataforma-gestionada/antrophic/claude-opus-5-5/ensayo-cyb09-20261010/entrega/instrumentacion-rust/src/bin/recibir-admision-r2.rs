//! Recepción documental de la comprobación remota; no hace consultas al modelo.
#![forbid(unsafe_code)]
use std::{fs,path::Path};
use serde_json::{json,Value};
use sv_claude_kaggle::{self as sv,need,parse,R};
fn run()->R<Value>{
 let a=std::env::args().collect::<Vec<_>>();need(a.len()==3,"RAIZ SALIDA")?;
 let d=Path::new(&a[1]);let adm=parse(&fs::read(d.join("ADMISION-RUST.json")).map_err(|e|e.to_string())?)?;
 let cotejo=parse(&fs::read(d.join("COTEJO-TELEMETRIA.json")).map_err(|e|e.to_string())?)?;
 let tel=sv::instrumentacion::verify(&d.join("telemetria.jsonl"))?;
 need(adm["conforme"]==true&&adm["composiciones"]==75&&adm["inferencias"]==0&&adm["solicitudes_de_modelo"]==0,"Alcance remoto distinto")?;
 need(adm["modelo"]==sv::MODELO&&adm["sistema"]=="linux"&&adm["arquitectura"]=="x86_64","Identidad distinta")?;
 need(adm["binario_sha256"]=="1442aadea3506b0e2bec474f7d0bcd7851418bf4291ac99bb35986b7d95a2806"&&adm["paquete_sha256"]=="758b9bac230b81410df73795c7c74ae3089987bfb8aecc7d53917c50896f749d","Huellas distintas de las fijadas")?;
 need(adm["telemetria"]==tel&&cotejo==tel&&tel["fallos_medicion"]==0&&tel["muestras"].as_u64().is_some_and(|v|v>=4),"Telemetría discordante o insuficiente")?;
 need(tel["intervalo_maximo_ms"].as_u64().is_some_and(|v|v<=750),"Intervalo fuera de cota")?;
 let rows=sv::instrumentacion::records(&d.join("telemetria.jsonl"))?;
 let samples=rows.iter().filter(|v|v["tipo"]=="muestra").collect::<Vec<_>>();
 let pid=samples[0]["datos"]["proceso"]["pid"].clone();let start=samples[0]["datos"]["proceso"]["inicio_unix_s"].clone();
 need(pid.as_u64().is_some()&&start.as_u64().is_some()&&samples.iter().all(|v|v["datos"]["proceso"]["pid"]==pid&&v["datos"]["proceso"]["inicio_unix_s"]==start),"Identidad del proceso discordante")?;
 need(samples.iter().all(|v|v["datos"]["proceso"]["hilos_so"].as_u64().is_some_and(|n|n>0)&&v["datos"]["proceso"]["descriptores_abiertos_linux"].as_u64().is_some_and(|n|n>0)&&v["datos"]["proceso"]["espacio_red_linux"].as_str().is_some_and(|s|s.starts_with("net:[")&&s.ends_with("]"))),"Observación Linux adicional no acreditada")?;let result=json!({"observacion_adicional_linux_conforme":true,"conforme":true,"recepcion":"comprobación técnica remota sin inferencia","admision":adm,"telemetria_recuperada_cotejada":tel,"proceso_identico":true,"recepcion_servicio_de_modelo":"pendiente de la primera entrega autorizada","recepcion_cientifica_independiente":"pendiente","licencia":sv::LICENCIA});
 let bytes=serde_json::to_vec_pretty(&result).map_err(|e|e.to_string())?;
 use std::io::Write;let mut f=fs::OpenOptions::new().create_new(true).write(true).open(&a[2]).map_err(|e|e.to_string())?;f.write_all(&bytes).and_then(|_|f.sync_all()).map_err(|e|e.to_string())?;Ok(result)
}
fn main(){match run(){Ok(_)=>println!("Recepción técnica conforme: originales cotejados en Rust; cero inferencias"),Err(e)=>{eprintln!("IMPEDIMENTO: {e}");std::process::exit(1)}}}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
