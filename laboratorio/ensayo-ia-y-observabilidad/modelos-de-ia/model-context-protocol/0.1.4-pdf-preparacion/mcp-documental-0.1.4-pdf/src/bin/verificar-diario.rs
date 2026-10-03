use std::{path::Path,process};
use sv_mcp_documental::{Catalog,read_bounded,auditoria::{verify,MAX_DIARIO}};
fn run()->Result<(),String>{
 let a:Vec<String>=std::env::args().collect();
 if (a.len()!=5&&a.len()!=8)||(a[4]!="--sintetico"&&a[4]!="--oficial"){return Err("Uso: verificar-diario CATALOGO SHA256 DIARIO --sintetico|--oficial [--fuentes-autorizadas FUENTES SHA256]".into())}
 let sources=if a.len()==8 {if a[5]!="--fuentes-autorizadas"{return Err("Opción desconocida".into());}sv_mcp_documental::fuentes_autorizadas(Path::new(&a[6]),&a[7])?}else{Vec::new()};
 let c=Catalog::load_with_sources(Path::new(&a[1]),&a[2],a[4]=="--sintetico",&sources)?;
 let raw=read_bounded(Path::new(&a[3]),MAX_DIARIO).map_err(|e|e.to_string())?;
 let first=raw.split(|b|*b==b'\n').next().ok_or("Diario vacío")?;
 let start:serde_json::Value=serde_json::from_slice(first).map_err(|e|e.to_string())?;
 let expected=if a.len()==8 {serde_json::Value::String(a[7].clone())}else{serde_json::Value::Null};
 if start["datos"]["fuentes_autorizadas_sha256"]!=expected{return Err("La lista de fuentes no coincide con la registrada al inicio".into());}
 println!("{}",verify(&c,&a[2],&raw)?);Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("Cotejo no conforme: {e}");process::exit(1)}}
