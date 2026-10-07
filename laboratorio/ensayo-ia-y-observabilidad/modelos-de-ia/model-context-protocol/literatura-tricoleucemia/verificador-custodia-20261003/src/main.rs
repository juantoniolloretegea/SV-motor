use std::{fs, path::{Component, Path}};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
type R<T> = Result<T, Box<dyn std::error::Error>>;
fn read(p:&Path)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn digest(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn check(v:bool,msg:&str)->R<()>{if v{Ok(())}else{Err(msg.into())}}
fn main()->R<()>{
 let args:Vec<String>=std::env::args().collect();
 let local=Path::new(args.get(1).ok_or("Falta carpeta preparada")?);
 let remote=args.get(2).map(Path::new);
 let mut result=Vec::new(); let mut total=0u64;
 for folder in ["nci-pdq-profesionales","lls-hoja-informativa-2018"]{
  let base=local.join(folder); let mf=base.join("MANIFIESTO-SHA256.json"); let manifest=read(&mf)?;
  let rows=manifest["archivos"].as_array().ok_or("Manifiesto sin archivos")?;
  for row in rows{
   let name=row["ruta"].as_str().ok_or("Ruta ausente")?;
   check(!name.contains(':')&&Path::new(name).components().all(|c|matches!(c,Component::Normal(_))),"Ruta fuera de ámbito")?;
   let bytes=fs::read(base.join(name))?;
   check(row["bytes"]==bytes.len()as u64&&row["sha256"]==digest(&bytes),"Contenido local discordante")?;
   if let Some(r)=remote{check(fs::read(r.join(folder).join(name))?==bytes,"Copia recuperada discordante")?;}
   total+=bytes.len()as u64;
  }
  if let Some(r)=remote{check(fs::read(r.join(folder).join("MANIFIESTO-SHA256.json"))?==fs::read(&mf)?,"Manifiesto recuperado discordante")?;}
  let doc=read(&base.join("documento.json"))?;let proof=read(&base.join("cotejo.json"))?;
  check(proof["conforme"]==true,"Cotejo documental no conforme")?;
  let parts=doc["segmentos"].as_array().ok_or("Segmentos ausentes")?;
  let expected=if folder.starts_with("lls"){10}else{5};check(parts.len()==expected,"Número de segmentos discordante")?;
  for (i,part) in parts.iter().enumerate(){check(part["indice"]==i as u64,"Índices incompletos")?;if expected==10{check(part["pagina_indice"]==i as u64&&part["pagina_impresa"]==(i+1)as u64,"Numeración PDF discordante")?;}}
  result.push(json!({"carpeta":folder,"archivos_cotejados":rows.len()+1,"segmentos":parts.len(),"manifiesto_sha256":digest(&fs::read(&mf)?)}));
 }
 let readme=fs::read(local.join("readme.md"))?;
 if let Some(r)=remote{check(fs::read(r.join("readme.md"))?==readme,"Objetivo recuperado discordante")?;}
 println!("{}",serde_json::to_string_pretty(&json!({"conforme":true,"recuperacion_remota_cotejada":remote.is_some(),"carpetas":result,"bytes_contenidos":total,"readme_sha256":digest(&readme),"alcance":"Identidad material, estructura y numeración documental; no es una evaluación clínica."}))?);
 Ok(())
}
