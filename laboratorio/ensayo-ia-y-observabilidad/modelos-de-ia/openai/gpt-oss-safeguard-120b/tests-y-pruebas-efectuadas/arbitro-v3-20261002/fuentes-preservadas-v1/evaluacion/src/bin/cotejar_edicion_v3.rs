use std::{fs, io::Write, path::Path, process::Command, time::{SystemTime,UNIX_EPOCH}};
use serde_json::{Value,json};
use sha2::{Sha256,Digest};
type R<T> = Result<T,Box<dyn std::error::Error>>;
fn hash(b:&[u8])->String {format!("{:x}",Sha256::digest(b))}
fn require(b:bool,s:&str)->R<()> {if b {Ok(())}else{Err(s.into())}}
fn main()->R<()> {
 let a=std::env::args().collect::<Vec<_>>();
 require(a.len()==6,"Uso: indice originales examinados informe etapa")?;
 let index_bytes=fs::read(&a[1])?;
 let index:Value=serde_json::from_slice(&index_bytes)?;
 let originals=Path::new(&a[2]);let examined=Path::new(&a[3]);let report_path=Path::new(&a[4]);
 let rows=index["archivos"].as_array().ok_or("Lista de archivos ausente")?;
 let expected=rows.iter().map(|r|r["archivo"].as_str().ok_or("Nombre ausente")).collect::<Result<Vec<_>,_>>()?;
 require(!expected.is_empty()&&expected.iter().all(|n|n.ends_with(".tar.gz")&&!n.contains(['/','\\',':'])&&!n.contains("..")),"Nombres no acotados")?;
 require(rows.len()<=8,"Número no delimitado de archivos")?;
 let mut report=Vec::new();let mut total=0;
 for (r,name) in rows.iter().zip(expected) {
  require(r["archivo"]==name,"Nombre u orden inesperado")?;
  let original=fs::read(originals.join(name))?;let copy=fs::read(examined.join(name))?;
  require(original==copy,"Bytes discordantes")?;
  let sha=hash(&copy);
  require(r["bytes"]==copy.len(),"Tamaño discordante")?;
  require(r["sha256"]==sha,"Huella discordante")?;
  let out=Command::new("C:/Windows/System32/tar.exe").arg("-tzf").arg(examined.join(name)).output()?;
  require(out.status.success(),"Archivo no legible")?;
  let listing=String::from_utf8(out.stdout)?;
  require(!listing.trim().is_empty(),"Archivo sin contenido")?;
  require(!listing.lines().any(|p|p.starts_with('/')||p.split('/').any(|s|s=="..")),"Ruta de archivo improcedente")?;
  total+=copy.len();report.push(json!({"archivo":name,"bytes":copy.len(),"sha256":sha,"igualdad_exacta":true,"archivo_legible":true,"entradas":listing.lines().count(),"url":r["url"]}));
 }
 let result=json!({"expediente":index["expediente"],"entrega":"03","etapa":a[5],"fecha_unix_s":SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),"conforme":true,"archivos":report,"total_bytes":total,"indice_sha256":hash(&index_bytes),"edicion":index["edicion"],"metodo":"Cotejo en Rust de bytes, tamaño y SHA-256 con el índice fijado; apertura de cada archivo mediante tar.","recepcion_cientifica_independiente":"pendiente","autoria_licencia":index["autoria_licencia"]});
 let mut out=fs::OpenOptions::new().write(true).create_new(true).open(report_path)?;
 out.write_all(&serde_json::to_vec_pretty(&result)?)?;out.sync_all()?;
 println!("{}",json!({"conforme":true,"archivos":rows.len(),"total_bytes":total,"etapa":a[5]}));Ok(())
}
