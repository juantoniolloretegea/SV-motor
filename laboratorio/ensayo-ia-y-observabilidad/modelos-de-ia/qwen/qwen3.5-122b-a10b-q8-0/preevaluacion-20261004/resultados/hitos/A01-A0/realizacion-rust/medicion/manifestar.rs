use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::{fs::{self,File,OpenOptions},io::{Read,Write},path::Path};
type E=Box<dyn std::error::Error>;
fn visit(root:&Path,dir:&Path,rows:&mut Vec<Value>)->Result<(),E>{
 let mut entries=fs::read_dir(dir)?.collect::<Result<Vec<_>,_>>()?;entries.sort_by_key(|e|e.file_name());
 for entry in entries {let p=entry.path();let ty=entry.file_type()?;if ty.is_symlink(){return Err("Enlace no admisible".into())}if ty.is_dir(){visit(root,&p,rows)?}else if ty.is_file(){let expected=entry.metadata()?.len();let mut f=File::open(&p)?;let mut h=Sha256::new();let mut buffer=[0u8;65536];let mut bytes=0u64;loop{let n=f.read(&mut buffer)?;if n==0{break}h.update(&buffer[..n]);bytes+=n as u64;}if bytes!=expected||entry.metadata()?.len()!=expected{return Err("Tamaño cambiante durante lectura".into())}rows.push(json!({"ruta":p.strip_prefix(root)?.to_string_lossy().replace('\\',"/"),"bytes":bytes,"sha256":format!("{:x}",h.finalize())}));}else{return Err("Objeto no regular".into())}}
 Ok(())
}
fn main()->Result<(),E>{let a:Vec<_>=std::env::args().collect();let root=Path::new(a.get(1).ok_or("Raíz")?).canonicalize()?;let output=Path::new(a.get(2).ok_or("Destino exterior a la raíz")?);let parent=output.parent().ok_or("Carpeta de destino")?.canonicalize()?;if parent.starts_with(&root){return Err("El manifiesto debe conservarse fuera del contenido medido".into())}let mut rows=Vec::new();visit(&root,&root,&mut rows)?;let v=json!({"encargo":"QWEN35-PRE-20261004/r1","realizacion":"Rust; lectura incremental de 65536 bytes","archivos":rows});let mut f=OpenOptions::new().create_new(true).write(true).open(output)?;f.write_all(&serde_json::to_vec_pretty(&v)?)?;f.sync_all()?;println!("{}",json!({"archivos":v["archivos"].as_array().unwrap().len(),"manifiesto":output}));Ok(())}
