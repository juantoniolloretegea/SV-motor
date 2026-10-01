use std::{path::Path,process};
use sv_mcp_documental::{Catalog,read_bounded,auditoria::{verify,MAX_DIARIO}};
fn run()->Result<(),String>{
 let a:Vec<String>=std::env::args().collect();
 if a.len()!=5||a[4]!="--sintetico"&&a[4]!="--oficial"{return Err("Uso: verificar-diario CATALOGO SHA256 DIARIO --sintetico|--oficial".into())}
 let c=Catalog::load(Path::new(&a[1]),&a[2],a[4]=="--sintetico")?;
 let raw=read_bounded(Path::new(&a[3]),MAX_DIARIO).map_err(|e|e.to_string())?;
 println!("{}",verify(&c,&a[2],&raw)?);Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("Cotejo no conforme: {e}");process::exit(1)}}
