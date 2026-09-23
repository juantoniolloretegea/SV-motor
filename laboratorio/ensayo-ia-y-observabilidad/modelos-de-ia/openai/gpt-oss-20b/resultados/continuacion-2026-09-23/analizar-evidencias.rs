use std::{fs,path::Path};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
fn main(){
 let root=std::env::args().nth(1).expect("directorio");let root=Path::new(&root);let mut summary=Vec::new();
 for n in 1..=8 {
  let p=root.join(format!("carga-{n:02}/SUCESOS.jsonl"));let text=fs::read_to_string(&p).unwrap();
  let rows:Vec<Value>=text.lines().map(|s|serde_json::from_str(s).unwrap()).collect();
  assert!(rows.windows(2).all(|r|r[0]["elapsed_ms"].as_u64()<=r[1]["elapsed_ms"].as_u64()));
  let get=|kind:&str|rows.iter().find(|v|v["kind"]==kind).cloned().unwrap_or(Value::Null);
  let last=get("resultado_final");assert_eq!(last["data"]["child_stop_confirmed"],true);
  let samples:Vec<_>=rows.iter().filter(|v|v["kind"]=="muestra_recursos").collect();
  let signals:Vec<_>=rows.iter().filter(|v|v["kind"]=="senal_solicitada"||v["kind"]=="senal_resultado").collect();
  summary.push(json!({"attempt":n,"journal_sha256":format!("{:x}",Sha256::digest(text.as_bytes())),"journal_events":rows.len(),"start":rows[0]["utc_ms"],"initial_capacity":get("capacidad_previa")["data"],"limits":get("limites_declarados")["data"],"identity":get("inicio")["data"],"configuration":get("configuracion_carga")["data"],"resource_samples_persisted":samples.len(),"last_resource_sample":samples.last(),"min_available_persisted":samples.iter().filter_map(|v|v["data"]["capacity"]["effective_available_bytes"].as_u64()).min(),"signals":signals,"result":last,"response":get("respuesta_original")}));
 }
 println!("{}",serde_json::to_string_pretty(&summary).unwrap());
}
