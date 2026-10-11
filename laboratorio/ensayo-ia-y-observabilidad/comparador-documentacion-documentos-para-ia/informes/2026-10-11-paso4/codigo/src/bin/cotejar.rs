use std::{fs,path::Path,collections::HashSet};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
fn main()->Result<(),Box<dyn std::error::Error>>{
 let a:Vec<String>=std::env::args().collect();if a.len()!=3{return Err("Uso: cotejar EVIDENCIAS Cargo.lock".into());}
 let root=Path::new(&a[1]);let read=|p:&str|->Result<Value,Box<dyn std::error::Error>>{Ok(serde_json::from_slice(&fs::read(root.join(p))?)?)};
 let one=read("EJECUCION-01-SALIDA.json")?;let two=read("EJECUCION-02-SALIDA.json")?;
 let fijacion=read("EJECUCION-01-FIJACION.json")?;if fijacion!=read("EJECUCION-02-FIJACION.json")?{return Err("Fijaciones distintas".into());}
 for r in fijacion["archivos"].as_array().ok_or("Fijación ausente")?{
  let name=r["archivo"].as_str().ok_or("Nombre ausente")?;
  let path=if name=="Cargo.lock"{Path::new(&a[2]).to_path_buf()}else if name=="main.rs"{Path::new(&a[2]).parent().ok_or("Directorio ausente")?.join("src/main.rs")}else{root.join(name)};
  let bytes=fs::read(path)?;if r["sha256"]!=format!("{:x}",Sha256::digest(&bytes))||r["bytes"]!=bytes.len(){return Err(format!("Fuente alterada: {name}").into());}
 }
 let rows=one["resultados"].as_array().ok_or("Resultados ausentes")?;let other=two["resultados"].as_array().ok_or("Resultados ausentes")?;
 let inputs=read("ENTRADAS-ADMISION.json")?;let inputs=inputs.as_array().ok_or("Entradas ausentes")?;
 if rows.len()!=14||rows.len()!=other.len()||rows.len()!=inputs.len(){return Err("Cantidad incorrecta".into());}
 let mut ids=HashSet::new();let mut totals=json!({"control":{"admitido":0,"rechazado":0},"medicina":{"admitido":0,"rechazado":0},"ciberseguridad":{"admitido":0,"rechazado":0}});let mut audit=Vec::new();let mut times=[0u64,0u64];
 for(i,(x,y))in rows.iter().zip(other).enumerate(){
  let id=x["entrada"]["id"].as_str().ok_or("ID ausente")?;if !ids.insert(id){return Err("ID repetido".into());}
  if x["entrada"]!=inputs[i]||x["entrada"]!=y["entrada"]||x["analisis"]["resultado"]!=y["analisis"]["resultado"]{return Err(format!("Discrepancia en {id}").into());}
  for(j,r)in [x,y].iter().enumerate(){
   if r["limite_agotado"]!=false||r["codigo"]!=0||r["stderr"]!=""{return Err(format!("Incidencia en {id}").into());}
   let stdout:Value=serde_json::from_str(r["stdout"].as_str().ok_or("Salida ausente")?)?;if stdout!=r["analisis"]{return Err("Salida no idéntica al análisis".into());}
   times[j]+=r["analisis"]["duracion_ns"].as_u64().ok_or("Tiempo ausente")?;
  }
  let domain=x["entrada"]["dominio"].as_str().ok_or("Dominio ausente")?;let status=x["analisis"]["resultado"]["estado"].as_str().ok_or("Estado ausente")?;
  let n=totals[domain][status].as_u64().ok_or("Estado desconocido")?;totals[domain][status]=json!(n+1);
  audit.push(json!({"id":id,"estado":status,"salidas_identicas":true}));
 }
 let v=json!({"conforme_instrumentalmente":true,"significado":"Identidad de entradas, fijaciones previas y resultados; no certifica conservación semántica ni detección de contradicciones.","cantidad":rows.len(),"totales":totals,"tiempos_candidato_ns":times,"casos":audit});
 fs::write(root.join("COTEJO-RUST.json"),serde_json::to_vec_pretty(&v)?)?;println!("{}",v);Ok(())
}
