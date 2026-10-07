#![forbid(unsafe_code)]
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,path::Path};
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn main()->Result<(),Box<dyn std::error::Error>>{
 let args:Vec<_>=std::env::args().collect();let base=Path::new(args.get(1).ok_or("Directorio")?);
 let p:Value=serde_json::from_slice(&fs::read(base.join("PLAN.json"))?)?;
 let r:Value=serde_json::from_slice(&fs::read(base.join("RECUPERADO.json"))?)?;
 let pf=p["files"].as_array().ok_or("Archivos locales")?;let rf=r["files"].as_array().ok_or("Archivos recuperados")?;
 if pf.len()!=rf.len()||p["repo"]!=r["repo"]||p["base"]!=r["base"]{return Err("Identidad o inventario distintos".into());}
 let mut ids=std::collections::BTreeSet::new();let mut rows=vec![];
 for f in pf {let name=f["path"].as_str().ok_or("Ruta")?;if !ids.insert(name){return Err("Ruta duplicada".into());}
  let matches=rf.iter().filter(|x|x["path"]==name).collect::<Vec<_>>();if matches.len()!=1{return Err("Falta archivo o duplicación".into());}
  let x=matches[0];let a=f["content"].as_str().ok_or("Contenido")?.as_bytes();let b=x["content"].as_str().ok_or("Contenido recuperado")?.as_bytes();
  if a!=b||x["sha256"]!=hash(b)||x["bytes"]!=b.len(){return Err("Contenido o huella distinta".into());}
  rows.push(json!({"ruta":name,"bytes":b.len(),"sha256":hash(b)}));
 }
 let v=json!({"conforme":true,"realizacion":"Rust","revision":r["commit"],"repositorio":r["repo"],"archivos":rows,"alcance":"Correspondencia por bytes y SHA-256 de la respuesta recuperada de GitHub; no recepción científica independiente"});
 use std::io::Write;let mut file=fs::OpenOptions::new().write(true).create_new(true).open(base.join("COTEJO-RUST.json"))?;file.write_all(&serde_json::to_vec_pretty(&v)?)?;file.sync_all()?;
 println!("Conforme: {} archivos; revisión {}",pf.len(),r["commit"]);Ok(())
}
