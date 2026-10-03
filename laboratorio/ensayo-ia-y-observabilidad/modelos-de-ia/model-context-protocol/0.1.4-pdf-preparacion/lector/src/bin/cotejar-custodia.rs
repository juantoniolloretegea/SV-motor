use std::{fs,path::{Path,Component}};
use serde_json::{Value,json};
fn main()->Result<(),Box<dyn std::error::Error>>{
 let a:Vec<String>=std::env::args().collect();if a.len()!=5{return Err("Uso: cotejar-custodia MANIFIESTO ORIGINAL RECUPERADO SALIDA".into());}
 let m:Value=serde_json::from_slice(&fs::read(&a[1])?)?;let mut records=Vec::new();let mut total=0u64;
 for r in m["archivos"].as_array().ok_or("Sin archivos")?{
  let relative=r["ruta"].as_str().ok_or("Sin ruta")?;let p=Path::new(relative);
  if p.is_absolute()||p.components().any(|c|!matches!(c,Component::Normal(_))){return Err("Ruta fuera de perímetro".into());}
  let original=fs::read(Path::new(&a[2]).join(p))?;let recovered=fs::read(Path::new(&a[3]).join(p))?;
  let hash=sv_pdf_documental::hash(&original);let size=original.len()as u64;
  if original!=recovered||r["sha256"]!=hash||r["bytes"]!=size{return Err(format!("Discordancia: {relative}").into());}
  total+=size;records.push(json!({"ruta":relative,"bytes":size,"sha256":hash,"identico":true}));
 }
 if records.is_empty(){return Err("Manifiesto vacío".into());}
 let result=json!({"conforme":true,"comprobacion":"Igualdad íntegra de bytes, tamaño y SHA-256 en Rust","archivos":records,"cantidad":records.len(),"bytes":total});
 fs::write(&a[4],serde_json::to_vec_pretty(&result)?)?;println!("Conforme: {} archivos, {} bytes",records.len(),total);Ok(())
}
