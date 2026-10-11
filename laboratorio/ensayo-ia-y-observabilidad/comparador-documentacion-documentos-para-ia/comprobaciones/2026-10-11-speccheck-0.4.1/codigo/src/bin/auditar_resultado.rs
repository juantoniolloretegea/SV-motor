//! Contraste de integridad, cobertura declarada y localizaciones de la salida.
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,path::Path,error::Error,collections::BTreeSet};
fn main()->Result<(),Box<dyn Error>>{
 let a:Vec<String>=std::env::args().collect();if a.len()!=2{return Err("Uso: auditar_resultado DIRECTORIO".into())}
 let root=Path::new(&a[1]);let r:Value=serde_json::from_slice(&fs::read(root.join("EJECUCION-01.json"))?)?;
 let entries:Value=serde_json::from_slice(&fs::read(root.join("ENTRADAS.json"))?)?;
 let ids:BTreeSet<_>=entries["casos"].as_array().unwrap().iter().map(|c|c["id"].as_str().unwrap()).collect();
 let outids:BTreeSet<_>=r["casos"].as_array().unwrap().iter().map(|c|c["id"].as_str().unwrap()).collect();
 if ids!=outids || outids.len()!=r["casos"].as_array().unwrap().len(){return Err("Discordancia de casos".into())}
 if r["ia_activada"]!=false || r["referencias_recibidas"]!=false || r["correcciones_aplicadas"]!=false{return Err("Recorrido no admitido".into())}
 let mut details=Vec::new();
 for c in r["casos"].as_array().unwrap(){for f in c["hallazgos"].as_array().unwrap(){
  let name=f["localizacion"]["file"].as_str().unwrap();let s=fs::read_to_string(root.join(name))?;
  let start=f["localizacion"]["byte_start"].as_u64().unwrap()as usize;let end=f["localizacion"]["byte_end"].as_u64().unwrap()as usize;
  if start>=end || end>s.len() || !s.is_char_boundary(start) || !s.is_char_boundary(end){return Err("Localización no válida".into())}
  if !c["archivos"].as_array().unwrap().iter().any(|x|x["ruta"]==name){return Err("Localización fuera del caso".into())}
  details.push(json!({"id":c["id"],"regla":f["regla"],"ruta":name,"inicio":start,"fin":end,"fragmento":&s[start..end],"localizacion_valida":true}));
 }}
 let report=json!({"elemento":"Buscador-Semántico - Diferencial","casos_unicos":ids.len(),"localizaciones_validas":details.len(),"ejecucion_sha256":format!("{:x}",Sha256::digest(fs::read(root.join("EJECUCION-01.json"))?)),"alcance":"cotejo de integridad y localizacion; no recepcion cientifica independiente","detalles":details});
 let out=root.join("AUDITORIA-LOCALIZACIONES-RUST.json");if out.exists(){return Err("Se conserva salida anterior".into())}fs::write(out,serde_json::to_vec_pretty(&report)?)?;println!("{} casos únicos; {} localizaciones válidas",ids.len(),details.len());Ok(())
}
