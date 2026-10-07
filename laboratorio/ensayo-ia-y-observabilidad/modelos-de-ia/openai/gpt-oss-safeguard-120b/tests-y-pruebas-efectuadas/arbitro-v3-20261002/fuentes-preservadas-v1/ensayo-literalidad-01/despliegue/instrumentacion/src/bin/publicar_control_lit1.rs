use std::{fs,path::Path};use serde_json::Value;
fn validar(v:&Value,sello:&Value,id:&str)->Result<(),String>{
 if !["D01","F01","F02","F03","F04","F05","F06"].contains(&id)||v.as_object().map(|x|x.len())!=Some(3)||v["id"]!=id||sello["id"]!=id||v["salida_sha256"]!=sello["salida_sha256"]||sello["adjudicacion_pendiente"]!=true||!["continuar","cerrar"].contains(&v["accion"].as_str().unwrap_or(""))||(id=="F06"&&v["accion"]!="cerrar"){return Err("Control no ligado al punto pendiente".into())}Ok(())
}
fn publicar(incoming:&Path,dst:&Path)->Result<(),Box<dyn std::error::Error>>{fs::hard_link(incoming,dst)?;Ok(())}
fn main()->Result<(),Box<dyn std::error::Error>>{
 let a=std::env::args().collect::<Vec<_>>();let id=a.get(1).ok_or("Caso requerido")?;if !["D01","F01","F02","F03","F04","F05","F06"].contains(&id.as_str()){return Err("Caso no autorizado".into())}
 let base=Path::new("/opt/sv-safeguard/arbitro-literalidad-01/autorizaciones");let incoming=base.join(format!("{id}.recibido"));let dst=base.join(format!("{id}.json"));
 let b=fs::read(&incoming)?;let v:Value=serde_json::from_slice(&b)?;let seal:Value=serde_json::from_slice(&fs::read(format!("/opt/sv-safeguard/evidencias/arbitro-literalidad-01/puntos/{id}/PUNTO.json"))?)?;validar(&v,&seal,id)?;publicar(&incoming,&dst)?;println!("Control completo publicado sin sobrescritura: {id}");Ok(())
}
#[cfg(test)]mod tests{use super::*;use serde_json::json;
 #[test]fn rechaza_control_ajeno_y_clave(){let s=json!({"id":"D01","salida_sha256":"h","adjudicacion_pendiente":true});let mut v=json!({"id":"D01","salida_sha256":"h","accion":"continuar"});assert!(validar(&v,&s,"D01").is_ok());v["salida_sha256"]=json!("x");assert!(validar(&v,&s,"D01").is_err());v["salida_sha256"]=json!("h");v["clave"]=json!("no permitida");assert!(validar(&v,&s,"D01").is_err());}
 #[test]fn publicacion_atomica_no_sobrescribe()->Result<(),Box<dyn std::error::Error>>{let p=std::env::current_dir()?.join(format!("prueba-control-{}",std::process::id()));fs::create_dir(&p)?;let src=p.join("entrada");let dst=p.join("control");fs::write(&src,b"completo")?;publicar(&src,&dst)?;assert_eq!(fs::read(&dst)?,b"completo");assert!(publicar(&src,&dst).is_err());assert_eq!(fs::read(&dst)?,b"completo");Ok(())}
}
