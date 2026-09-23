use std::{fs,path::Path};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
fn main(){
 let arg=std::env::args().nth(1).expect("directorio del intento");let root=Path::new(&arg);
 let number=std::env::args().nth(2).unwrap_or_else(||"13".into());
 assert!(number=="13"||number=="14");
 let bytes=fs::read(root.join(format!("carga-{number}/SUCESOS.jsonl"))).expect("registro");
 let rows:Vec<Value>=std::str::from_utf8(&bytes).unwrap().lines().map(|l|serde_json::from_str(l).unwrap()).collect();
 assert!(rows.windows(2).all(|r|r[0]["elapsed_ms"].as_u64()<=r[1]["elapsed_ms"].as_u64()));
 let get=|kind:&str|rows.iter().find(|v|v["kind"]==kind).cloned().unwrap_or(Value::Null);
 let samples:Vec<_>=rows.iter().filter(|v|v["kind"]=="muestra_recursos").collect();
 let result=get("resultado_final");
 let out=json!({"journal_sha256":format!("{:x}",Sha256::digest(&bytes)),"events":rows.len(),"initial_capacity":get("capacidad_previa"),"identity":get("inicio"),"configuration":get("configuracion_carga"),"limits":get("limites_declarados"),"resource_samples_persisted":samples.len(),"last_sample":samples.last(),"peak_anon_persisted":samples.iter().filter_map(|v|v["data"]["rss_anon_bytes"].as_u64()).max(),"min_available_persisted":samples.iter().filter_map(|v|v["data"]["capacity"]["effective_available_bytes"].as_u64()).min(),"signals":rows.iter().filter(|v|v["kind"]=="senal_solicitada"||v["kind"]=="senal_resultado").collect::<Vec<_>>(),"service":get("servicio_comprobado"),"request":get("peticion_prevista"),"response":get("respuesta_original"),"result":result});
 println!("{}",serde_json::to_string_pretty(&out).unwrap());
}
