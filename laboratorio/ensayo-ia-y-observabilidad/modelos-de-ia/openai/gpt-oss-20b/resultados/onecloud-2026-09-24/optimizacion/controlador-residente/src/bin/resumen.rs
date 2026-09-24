use serde_json::{json,Value};
use std::{collections::BTreeMap,fs};
fn main()->Result<(),Box<dyn std::error::Error>> {
 let args:Vec<String>=std::env::args().collect();
 let input=fs::read_to_string(&args[1])?;
 let mut casos=Vec::<Value>::new();let mut actual=String::new();let mut respuesta=Value::Null;let mut finalizacion=Value::Null;let mut inicio=Value::Null;
 for line in input.lines(){let v:Value=serde_json::from_str(line)?;match v["kind"].as_str(){Some("inicio")=>inicio=v.clone(),Some("inicio_caso")=>actual=v["data"]["id"].as_str().unwrap_or("").into(),Some("respuesta_original")=>respuesta=v["data"].clone(),Some("fin_caso")=>casos.push(json!({"id":actual,"latencia_ms":v["data"]["latencia_ms"],"respuesta":respuesta})),Some("resultado_final")=>finalizacion=v.clone(),_=>{}}}
 let mut lat:Vec<u64>=casos.iter().filter(|v|v["id"].as_str().unwrap_or("").starts_with("C01-")).filter_map(|v|v["latencia_ms"].as_u64()).collect();lat.sort();
 let mut perfil=BTreeMap::<String,(u64,u128,u128)>::new();
 if let Some(log)=args.get(2){for line in fs::read_to_string(log)?.lines(){if let Some(start)=line.find("{\"evento\":\"perfil_mxfp4\""){let v:Value=serde_json::from_str(&line[start..])?;let key=format!("{}:tokens={}",v["modo"].as_str().unwrap_or(""),v["tokens"]);let p=perfil.entry(key).or_default();p.0+=1;p.1+=v["preparacion_ns"].as_u64().unwrap_or(0) as u128;p.2+=v["calculo_ns"].as_u64().unwrap_or(0) as u128;}}}
 println!("{}",serde_json::to_string_pretty(&json!({"inicio":inicio,"casos":casos,"mediana_referencia_ms":if lat.len()==3{Some(lat[1])}else{None},"perfil":perfil,"finalizacion":finalizacion}))?);Ok(())
}
