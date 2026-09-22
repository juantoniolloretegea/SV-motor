use std::{fs,io::Read,path::{Path,Component}};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
fn main()->Result<(),Box<dyn std::error::Error>>{
 let dir=std::env::args().nth(1).ok_or("Falta directorio")?;let root=Path::new(&dir);
 let m:Value=serde_json::from_slice(&fs::read(root.join("MANIFIESTO.json"))?)?;
 let rows=m["files"].as_array().ok_or("Sin archivos")?;let mut total=0u64;
 for row in rows{let name=row["path"].as_str().ok_or("Sin ruta")?;let relative=Path::new(name);if !relative.components().all(|c|matches!(c,Component::Normal(_))){return Err("Ruta inadmisible".into())}
  let file=root.join(relative);let meta=fs::symlink_metadata(&file)?;if !meta.is_file()||meta.file_type().is_symlink(){return Err("Archivo no ordinario".into())}
  if Some(meta.len())!=row["bytes"].as_u64(){return Err(format!("Tamaño distinto: {name}").into())}
  let mut f=fs::File::open(&file)?;let mut h=Sha256::new();let mut b=[0u8;65536];loop{let n=f.read(&mut b)?;if n==0{break}h.update(&b[..n])}let sha=format!("{:x}",h.finalize());if Some(sha.as_str())!=row["sha256"].as_str(){return Err(format!("Huella distinta: {name}").into())}total+=meta.len();
 }
 let bin=rows.iter().find(|v|v["path"]=="bin/eio-conversacion").ok_or("Sin ejecutable")?;
 assert_eq!(bin["sha256"],"3b29ef59d82145575d5efaba1ec76e50c70bf4a576e2d13008fa3baf6de70e61");
 let c:Value=serde_json::from_slice(&fs::read(root.join("COMPOSICION.json"))?)?;assert!(c["packages_without_collected_license_text"].as_array().ok_or("Sin cobertura")?.is_empty());
 let r:Value=serde_json::from_slice(&fs::read(root.join("consulta-documental/resultados/RESULTADO.json"))?)?;assert_eq!(r["cases"].as_array().ok_or("Sin casos")?.len(),4);
 println!("{}",json!({"schema":"SV-ENTREGA-COTEJO-1","result":"conforme","files_checked":rows.len(),"bytes_checked":total,"binary_identity":"conforme","third_party_license_text_coverage":"identificada","documental_cases":4,"scope":"Tamaños y SHA-256 de archivos enumerados; identidad del ejecutable y presencia documental. No es certificación funcional, jurídica ni clínica."}));Ok(())
}
