//! Utilidad administrativa local: un mensaje humano identificado, sin red ni secreto en argumentos.
#![forbid(unsafe_code)]
use std::{fs::{self,OpenOptions},io::{BufRead,BufReader,Read,Write},path::Path};
use serde_json::Value;
use zeroize::{Zeroize,Zeroizing};
use sv_cliente_api::{guard,need,R};
fn limpiar(v:&mut Value){match v{Value::String(s)=>s.zeroize(),Value::Array(a)=>for x in a{limpiar(x)},Value::Object(m)=>for x in m.values_mut(){limpiar(x)},_=>{}}}
fn candidato(s:&str)->bool{let Some((a,b))=s.split_once('.')else{return false};a.len()>=20&&b.len()>=12&&s.len()<=256&&s.bytes().all(|c|c.is_ascii_alphanumeric()||b"._-".contains(&c))}
fn run()->R<()> {
 let a:Vec<String>=std::env::args().collect();need(a.len()==4,"Uso: FUENTE_PRIVADA ID_MENSAJE DESTINO_PRIVADO")?;
 let dest=Path::new(&a[3]);guard(dest)?;
 need(dest.starts_with("C:/SV-LABORATORIO/privado")&&dest.extension().is_some_and(|s|s=="key")&&!dest.exists(),"Destino no recibido")?;
 let parent=dest.parent().ok_or("Directorio ausente")?;need(parent.is_dir(),"Prepare previamente el directorio privado y sus permisos")?;
 let mut options=OpenOptions::new();options.read(true);
 #[cfg(windows)]{use std::os::windows::fs::OpenOptionsExt;options.share_mode(7);}
 let input=options.open(&a[1]).map_err(|_|"No se pudo leer la fuente privada")?;
 let mut reader=BufReader::new(input);let mut found=None;
 loop {
  let mut raw=Zeroizing::new(Vec::new());let n=reader.by_ref().take(4*1024*1024+1).read_until(b'\n',&mut raw).map_err(|_|"Lectura incompleta")?;
  if n==0{break;}
  // Los adjuntos grandes se omiten sin conservar el registro completo en memoria.
  if n>4*1024*1024 {
   if raw.last()!=Some(&b'\n'){reader.skip_until(b'\n').map_err(|_|"No se pudo omitir un registro extenso")?;}
   continue;
  }
  if !raw.windows(a[2].len()).any(|w|w==a[2].as_bytes()){continue;}
  let mut v:Value=serde_json::from_slice(&raw).map_err(|_|"Registro no recibido")?;
  let result=(||->R<Option<Zeroizing<String>>>{
   if v["type"]!="event_msg"||v["payload"]["item"]["id"]!=a[2]||v["payload"]["item"]["type"]!="UserMessage"{return Ok(None);}
   let mut tokens=Vec::new();
   for p in v["payload"]["item"]["content"].as_array().ok_or("Contenido ausente")? {
    if p["type"]!="text"{continue;}
    for s in p["text"].as_str().ok_or("Texto ausente")?.split(|c:char|!(c.is_ascii_alphanumeric()||"._-".contains(c))) {if candidato(s){tokens.push(Zeroizing::new(s.to_owned()));}}
   }
   need(tokens.len()==1,"No se identifica una única credencial en el mensaje autorizado")?;
   Ok(tokens.pop())
  })();limpiar(&mut v);
  if let Some(key)=result?{found=Some(key);break;}
 }
 let key=found.ok_or("No se localizó la entrega humana identificada")?;
 let mut output=OpenOptions::new().create_new(true).write(true).open(dest).map_err(|_|"Custodia no escrita")?;
 output.write_all(key.as_bytes()).and_then(|_|output.sync_all()).map_err(|_|"Custodia incompleta")?;drop(output);
 let recovered=Zeroizing::new(fs::read(dest).map_err(|_|"No se pudo cotejar la custodia")?);
 need(recovered.as_slice()==key.as_bytes(),"Cotejo de custodia discordante")?;
 println!("{{\"custodia_local\":true,\"lectura_posterior_concordante\":true,\"secreto_emitido\":false,\"inferencias\":0}}");Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1);}}
