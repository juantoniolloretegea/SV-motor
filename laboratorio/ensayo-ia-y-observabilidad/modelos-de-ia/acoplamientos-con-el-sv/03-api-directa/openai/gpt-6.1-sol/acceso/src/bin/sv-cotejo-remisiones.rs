//! Cotejo documental acotado de remisiones.
#![forbid(unsafe_code)]
use std::{fs,io::Write,path::Path};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
type R<T>=Result<T,Box<dyn std::error::Error>>;
const DIR:&str="publicaciones/remisiones-nodo01-20261006";
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn read(p:impl AsRef<Path>)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn save(p:impl AsRef<Path>,b:&[u8])->R<()>{let p=p.as_ref();if p.exists(){assert_eq!(fs::read(p)?,b);}else{let mut f=fs::OpenOptions::new().write(true).create_new(true).open(p)?;f.write_all(b)?;f.sync_all()?;}Ok(())}
fn links(s:&str)->Vec<&str>{s.split("](").skip(1).filter_map(|s|s.split_once(')').map(|p|p.0)).collect()}
fn diagrams(s:&str)->Vec<&str>{let fence=char::from(96).to_string().repeat(3);s.split(&format!("{fence}mermaid")).skip(1).map(|s|s.split_once(&fence).expect("Diagrama abierto").0).collect()}
fn main()->R<()>{
 let root=std::env::current_dir()?.canonicalize()?;let dir=Path::new(DIR).canonicalize()?;assert!(dir.starts_with(&root));
 let plan=read(dir.join("PLAN-r2.json"))?;let files=plan["files"].as_array().ok_or("Archivos")?;assert_eq!(files.len(),5);
 let received=dir.join("RECUPERADOS.json");let mut rows=vec![];
 for(i,f)in files.iter().enumerate(){
  let p=f["path"].as_str().ok_or("Ruta")?;assert!(p.starts_with("laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/")&&p.to_lowercase().ends_with("/readme.md")&&!p.contains(".."));
  let old=f["original"].as_str().ok_or("Original")?;let new=f["content"].as_str().ok_or("Edición")?;
  assert!(!new.contains('\u{fffd}'));assert!(new.trim_end().ends_with(plan["autoria_licencia"].as_str().ok_or("Licencia")?));assert_eq!(diagrams(old),diagrams(new));
  let newlinks=links(new);for l in links(old){assert!(newlinks.contains(&l),"Referencia anterior retirada: {l}");}
  if p.contains("/01-inferencia-"){for term in ["## Expedientes en su ubicación de origen","../../qwen/README.md","../../openai/README.md","../../kimi/kimi-k3/ESTUDIO-VIABILIDAD-20261005.md","../../zai-org/glm-5.3/ESTUDIO-VIABILIDAD-20261005.md","disponibilidad de recursos"]{assert!(new.contains(term),"Falta remisión {term}");}}
  save(dir.join(format!("PREPARADO-{i}.md")),new.as_bytes())?;
  rows.push(json!({"path":p,"original_sha256":hash(old.as_bytes()),"bytes":new.len(),"sha256":hash(new.as_bytes())}));
 }
 if received.exists(){
  let r=read(received)?;let rf=r["files"].as_array().ok_or("Recuperados")?;assert_eq!(rf.len(),files.len());
  for f in files{let found:Vec<_>=rf.iter().filter(|x|x["path"]==f["path"]).collect();assert_eq!(found.len(),1);assert_eq!(f["content"].as_str().unwrap().as_bytes(),found[0]["content"].as_str().unwrap().as_bytes());}
  save(dir.join("COTEJO-REMOTO.json"),&serde_json::to_vec_pretty(&json!({"estado":"conforme","commit":r["commit"],"archivos":rows,"enlaces_anteriores_conservados":true,"diagramas_intactos":true,"traslados":0,"autoria_licencia":plan["autoria_licencia"]}))?)?;
  println!("Cotejo Rust conforme: cinco README recuperados idénticos por bytes/SHA-256; enlaces previos y diagramas conservados.");
 }else{
  save(dir.join("CONTROL-PREVIO.json"),&serde_json::to_vec_pretty(&json!({"estado":"conforme_preparacion","base":plan["base"],"archivos":rows,"enlaces_anteriores_conservados":true,"diagramas_intactos":true}))?)?;
  println!("Preparación conforme: cinco README; referencias previas, diagramas y licencia conservados.");
 }Ok(())
}
