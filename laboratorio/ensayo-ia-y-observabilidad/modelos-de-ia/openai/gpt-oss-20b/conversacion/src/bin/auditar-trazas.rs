//! Cotejo de las huellas de tiempos conservados con sus tramos de cierre.
use std::{env,fs};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
type Result<T>=std::result::Result<T,Box<dyn std::error::Error>>;
fn main()->Result<()>{
 let a:Vec<_>=env::args().collect();if a.len()!=3{return Err("Uso: RESULTADOS_JSONL TRAZAS_JSONL".into())}
 let results=fs::read_to_string(&a[1])?;let traces=fs::read_to_string(&a[2])?;
 let spans:Vec<Value>=traces.lines().map(serde_json::from_str).collect::<std::result::Result<_,_>>()?;
 let mut rows=vec![];
 for line in results.lines(){
  let r:Value=serde_json::from_str(line)?;let observation=&r["turn"]["result"]["observability"];
  let trace=observation["trace_id"].as_str().ok_or("Sin identificador de traza")?;
  let candidates:Vec<_>=spans.iter().filter(|s|s["trace_id"]==trace&&s["attributes"]["result_json"].is_string()).collect();
  if candidates.len()!=1{return Err(format!("Cierre ausente o ambiguo: {}",r["case"]).into())}
  let summary:Value=serde_json::from_str(candidates[0]["attributes"]["result_json"].as_str().unwrap())?;
  let expected=&summary["phase_timings_summary"];let bytes=serde_json::to_vec(&r["turn"]["result"]["phase_timings"])?;
  let digest=format!("{:x}",Sha256::digest(&bytes));
  if expected["sha256"]!=digest||expected["serialized_bytes"]!=bytes.len()||observation["export"]["ok"]!=true{return Err(format!("Cotejo no conforme: {}",r["case"]).into())}
  rows.push(json!({"condicion":r["case"],"sha256":digest,"bytes_originales":bytes.len(),"trace_id":trace,"conforme":true}));
 }
 if rows.is_empty(){return Err("No hay resultados que auditar".into())}
 println!("{}",serde_json::to_string_pretty(&json!({"esquema":"EIO-COTEJO-TRAZAS-1","resultados":rows,"alcance":"Cotejo local de los cierres recibidos; no custodia independiente ni cobertura exhaustiva."}))?);Ok(())
}
