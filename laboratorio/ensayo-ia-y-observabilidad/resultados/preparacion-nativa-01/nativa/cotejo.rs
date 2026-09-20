//! Cotejo sobre la copia receptora; jamás reescribe evidencias.
use eio_candidato::nativa::{Error,FILES};
use serde::Deserialize;
use sha2::{Digest,Sha256};
use std::{fs,io::Read,path::PathBuf};
#[derive(Deserialize)]#[serde(deny_unknown_fields)]
struct Item{archivo:String,bytes:u64,sha256:String}
#[derive(Deserialize)]#[serde(deny_unknown_fields)]
struct Manifiesto{contrato:String,estado:String,completa:bool,archivos:Vec<Item>,coste_escritura_ns:u128,escrituras:u64,excluido:String}
fn main()->Result<(),Error>{
 let a:Vec<_>=std::env::args().collect();if a.len()!=2{return Err("cotejo DIRECTORIO_RECUPERADO".into())}
 let dir=PathBuf::from(&a[1]);let b=fs::read(dir.join("MANIFIESTO.json"))?;if b.len()>65536{return Err("MANIFIESTO_TAMANO".into())}
 let m:Manifiesto=serde_json::from_slice(&b)?;if m.contrato!="EIO-NAT/1"||m.archivos.len()!=4{return Err("INVENTARIO".into())}
 for nombre in &FILES[..4]{
  let items:Vec<_>=m.archivos.iter().filter(|x|x.archivo==*nombre).collect();if items.len()!=1{return Err("INVENTARIO".into())}let item=items[0];
  let p=dir.join(nombre);let md=fs::symlink_metadata(&p)?;if !md.is_file()||md.len()>8*1024*1024{return Err("TIPO_TAMANO".into())}
  let mut h=Sha256::new();let mut file=fs::File::open(p)?;let mut n=0;let mut b=[0;65536];loop{let z=file.read(&mut b)?;if z==0{break}n+=z as u64;h.update(&b[..z]);}
  let digest=format!("{:x}",h.finalize());let ok=n==item.bytes&&digest==item.sha256;
  println!("{}",serde_json::json!({"archivo":nombre,"bytes":n,"sha256":digest,"identidad":ok}));if !ok{return Err("NO_CONFORME_IDENTIDAD".into())}
 }
 println!("{}",serde_json::json!({"estado_emisor":m.estado,"completitud_declarada_emisor":m.completa,"coste_escritura_ns":m.coste_escritura_ns,"escrituras":m.escrituras,"excluido":m.excluido,"alcance":"identidad de cuatro archivos; no prueba exhaustividad ni aceptación"}));Ok(())
}
