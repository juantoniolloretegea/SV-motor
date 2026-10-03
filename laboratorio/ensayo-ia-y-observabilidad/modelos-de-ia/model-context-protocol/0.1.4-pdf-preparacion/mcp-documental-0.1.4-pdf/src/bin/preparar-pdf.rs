use std::{fs::{self,OpenOptions},io::Write,path::Path};
use sv_mcp_documental::{Catalog,Document,Section,FuenteAutorizada,sha256,read_bounded};
use serde_json::json;
fn save(p:&Path,b:&[u8])->std::io::Result<()> {let mut f=OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(b)?;f.sync_all()}
#[cfg(all(target_os="linux",target_arch="x86_64"))]
fn limits()->std::io::Result<()>{
 #[repr(C)]struct Limit{current:u64,maximum:u64}
 unsafe extern "C"{fn setrlimit(resource:i32,limit:*const Limit)->i32;}
 for(resource,value)in[(0,30),(9,512*1024*1024)]{if unsafe{setrlimit(resource,&Limit{current:value,maximum:value})}!=0{return Err(std::io::Error::last_os_error());}}
 Ok(())
}
#[cfg(not(all(target_os="linux",target_arch="x86_64")))]
fn limits()->std::io::Result<()>{Err(std::io::Error::other("Preparación aislada disponible sólo en Linux x86_64"))}
fn main()->Result<(),Box<dyn std::error::Error>>{
 let a:Vec<_>=std::env::args().collect();
 if a.len()!=6{return Err("Uso: preparar-pdf PDF SHA256 IDENTIFICADOR URL_PROCEDENCIA CARPETA_NUEVA".into());}
 limits()?;sv_mcp_documental::aislamiento::no_network()?;
 let bytes=read_bounded(Path::new(&a[1]),sv_pdf_documental::MAX_PDF)?;
 let out=Path::new(&a[5]);fs::create_dir(out)?;
 let extraction=match sv_pdf_documental::extract(&bytes,&a[2]){Ok(v)=>v,Err(e)=>{
  save(&out.join("ERROR.json"),&serde_json::to_vec_pretty(&json!({"conforme":false,"causa":e,"original_sha256":sha256(&bytes),"catalogo_producido":false}))?)?;
  return Err(e.into());
 }};
 let sources=vec![FuenteAutorizada{documento:a[3].clone(),url:a[4].clone(),sha256:a[2].clone()}];
 let cat=Catalog{version:1,documents:vec![Document{
  id:a[3].clone(),title:"Documento PDF conservado; texto extraído".into(),url:a[4].clone(),
  retrieved_utc:format!("unix:{}",std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs()),
  updated_source:None,raw_sha256:a[2].clone(),synthetic:false,
  sections:extraction.paginas.iter().map(|p|Section{id:format!("PDF-P{:04}",p.indice),title:format!("Página PDF {} (índice {})",p.pagina_impresa_ordinal,p.indice),text:p.texto.clone(),sha256:p.sha256.clone()}).collect(),
 }]};
 cat.validate_with_sources(false,&sources)?;
 let catalog_bytes=serde_json::to_vec_pretty(&cat)?;
 if catalog_bytes.len()>sv_mcp_documental::MAX_CATALOG{return Err("CATALOGO_SUPERA_COTA".into());}
 let source_bytes=serde_json::to_vec_pretty(&sources)?;
 save(&out.join("EXTRACCION.json"),&serde_json::to_vec_pretty(&extraction)?)?;
 save(&out.join("CATALOGO.json"),&catalog_bytes)?;
 save(&out.join("FUENTES.json"),&source_bytes)?;
 let proof=json!({"preparado":true,"paginas":extraction.paginas.len(),"original_sha256":a[2],"catalogo_sha256":sha256(&catalog_bytes),"fuentes_sha256":sha256(&source_bytes),"binario_sha256":sha256(&fs::read(std::env::current_exe()?)?),"cpu_segundos":30,"memoria_bytes":512*1024*1024,"red_bloqueada":true,"recepcion_cientifica":false,"inferencia":false});
 save(&out.join("PREPARACION.json"),&serde_json::to_vec_pretty(&proof)?)?;
 println!("{proof}");Ok(())
}
