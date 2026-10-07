//! Conserva una preparación interrumpida antes de cualquier cambio de configuración.
use std::{fs,path::Path};
fn main()->Result<(),Box<dyn std::error::Error>>{
 let p=Path::new("/opt/sv-qwen80/examen-09/recuperacion-parametros/cierre-consulta");let dest=p.with_file_name("cierre-consulta-preintento-registrador");
 if dest.exists()||p.join("preparacion-ejecutada.json").exists(){return Err("ESTADO_NO_PREVISTO".into())}
 let mut names=vec![];for e in fs::read_dir(p)?{let e=e?;if !e.file_type()?.is_file(){return Err("ENTRADA_NO_PREVISTA".into())}let n=e.file_name().into_string().map_err(|_|"NOMBRE")?;if !["estado-previo-ORDEN.json","estado-previo-RESULTADO.json","estado-previo.stdout","estado-previo.stderr"].contains(&n.as_str()){return Err("ARCHIVO_NO_PREVISTO".into())}names.push(n);}
 if names.len()!=4{return Err("PREINTENTO_INCOMPLETO".into())}fs::rename(p,&dest)?;fs::File::open(dest.parent().unwrap())?.sync_all()?;println!("PREINTENTO_CONSERVADO_SIN_CAMBIO_DE_CONFIGURACION");Ok(())
}
