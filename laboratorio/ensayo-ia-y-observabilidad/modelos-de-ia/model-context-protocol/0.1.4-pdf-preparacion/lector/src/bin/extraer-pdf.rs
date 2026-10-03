use std::{fs::{File,OpenOptions},io::{Read,Write}};
fn main()->Result<(),Box<dyn std::error::Error>>{
 let a:Vec<_>=std::env::args().collect();
 if a.len()!=4{return Err("Uso: extraer-pdf PDF SHA256 SALIDA_NUEVA_JSON".into());}
 let mut bytes=Vec::new();File::open(&a[1])?.take(sv_pdf_documental::MAX_PDF as u64+1).read_to_end(&mut bytes)?;
 let result=sv_pdf_documental::extract(&bytes,&a[2])?;
 let mut f=OpenOptions::new().create_new(true).write(true).open(&a[3])?;
 f.write_all(&serde_json::to_vec_pretty(&result)?)?;f.sync_all()?;
 println!("Extracción conservada: {} páginas",result.paginas.len());Ok(())
}
