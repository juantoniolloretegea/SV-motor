use std::{fs,io::{Read,Write},path::Path};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
mod auditoria;
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn ck(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn save(p:&Path,b:&[u8])->R<()>{fs::create_dir_all(p.parent().ok_or("Directorio")?)?;let mut f=fs::OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(b)?;f.sync_all()?;Ok(())}
fn val(p:&Path)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn finales(p:&Path)->R<Vec<Value>>{let s=fs::read_to_string(p)?;let v=s.lines().filter_map(|l|l.strip_prefix("SV_EVENT ")).map(serde_json::from_str::<Value>).collect::<Result<Vec<_>,_>>()?;let r=v.into_iter().filter(|v|v["datos"]["evento"]=="respuesta_final").collect::<Vec<_>>();ck(r.len()==9,"Nueve respuestas exigidas")?;for(i,v)in r.iter().enumerate(){ck(v["datos"]["id"]==format!("A{:02}",i+1),"Orden o identidad")?;}Ok(r)}
fn ante(id:&Value,texto:&str)->Value{json!({"id":id,"capa":0,"respuesta_final":texto,"sha256":sha(texto.as_bytes())})}
fn main()->R<()>{let a=std::env::args().collect::<Vec<_>>();match a.get(1).map(String::as_str){
 Some("auditar")=>{let p=Path::new(a.get(2).ok_or("Original")?);let out=Path::new(a.get(3).ok_or("Destino")?);let v=auditoria::auditar(p,false,false)?;ck(v["conforme"]==true&&v["emisiones"]==9&&v["recorrido_completo"]==true,"Capa incompleta")?;save(out,&serde_json::to_vec_pretty(&v)?)?;println!("{}",serde_json::to_string(&v)?);},
 Some("preparar-a1")=>{
  let anterior=Path::new(a.get(2).ok_or("Preparación A0")?);let recorrido=Path::new(a.get(3).ok_or("Original A0")?);let nueva=Path::new(a.get(4).ok_or("Preparación A1")?);
  let audit=val(&nueva.join("cotejos/CUSTODIA-A0.json"))?;let raw=fs::read(recorrido.join("modelo.stdout"))?;
  ck(audit["conforme"]==true&&audit["recorrido_completo"]==true&&audit["sello_sha256"]==sha(&fs::read(recorrido.join("CIERRE.json"))?),"Cierre no cotejado")?;
  ck(sha(&raw)=="fc88f81a4d4808d09d2a9ef43d670b0fb81dc3c9eb91f8a9378155c67e21ad61","Original A0 distinto")?;
  let prevraw=fs::read(anterior.join("config/plan.json"))?;ck(sha(&prevraw)=="378f643c5bd7401d8b6a970ff581df566f0ce712f91029e17ef38ba3946c83e6","Plan A0 distinto")?;
  let mut p:Value=serde_json::from_slice(&prevraw)?;ck(p["capa"]==0&&p["casos"].as_array().is_some_and(|v|v.len()==9),"Plan previo incompleto")?;
  let f=finales(&recorrido.join("modelo.stdout"))?;p["capa"]=json!(1);let mut recibos=Vec::new();
  for(i,c)in p["casos"].as_array_mut().unwrap().iter_mut().enumerate(){ck(c["antecedentes"]==json!([]),"Historia A0 inesperada")?;let t=f[i]["datos"]["contenido"].as_str().ok_or("Respuesta ausente")?;let q=ante(&c["id"],t);c["antecedentes"]=json!([q]);save(&nueva.join(format!("pensamiento-afinado/A0/{}.txt",c["id"].as_str().unwrap())),t.as_bytes())?;recibos.push(json!({"id":c["id"],"capa":0,"sha256":sha(t.as_bytes()),"bytes":t.len(),"original_completo":true}));}
  let b=serde_json::to_vec_pretty(&p)?;save(&nueva.join("config/plan.json"),&b)?;
  let mut c=val(&anterior.join("config/contrato.json"))?;c["capa"]=json!(1);c["plan_sha256"]=json!(sha(&b));
  for n in ["cache/catalogo.json","config/politica.txt","config/MODELO-REFERENCIA.json","config/NUCLEO-REFERENCIA.json"]{save(&nueva.join(n),&fs::read(anterior.join(n))?)?;}
  ck(c["catalogo_sha256"]==sha(&fs::read(nueva.join("cache/catalogo.json"))?)&&c["politica_sha256"]==sha(&fs::read(nueva.join("config/politica.txt"))?),"Fuente o política distinta")?;
  save(&nueva.join("config/contrato.json"),&serde_json::to_vec_pretty(&c)?)?;
  save(&nueva.join("cotejos/ANTECEDENTES-A0.json"),&serde_json::to_vec_pretty(&json!({"conforme":true,"original_sha256":sha(&raw),"originales":recibos,"plan_a1_sha256":sha(&b),"clave_entregada":false,"fuente_y_politica_sin_cambios":true}))?)?;fs::create_dir_all(nueva.join("autorizaciones"))?;println!("Plan A1: {}",sha(&b));
 },
 Some("extraer")=>{let p=Path::new(a.get(2).ok_or("Original")?);let out=Path::new(a.get(3).ok_or("Destino")?);let f=finales(p)?;save(out,&serde_json::to_vec_pretty(&f)?)?;println!("Nueve respuestas originales extraídas");},
 Some("huella")=>{let p=Path::new(a.get(2).ok_or("Archivo")?);let mut f=fs::File::open(p)?;let mut b=[0u8;1048576];let mut h=Sha256::new();let mut n=0;loop{let c=f.read(&mut b)?;if c==0{break}h.update(&b[..c]);n+=c;}println!("{}",json!({"archivo":p.file_name().unwrap().to_string_lossy(),"bytes":n,"sha256":format!("{:x}",h.finalize())}));},
 _=>return Err("Modo no autorizado".into())}Ok(())}
#[cfg(test)]mod tests{use super::*;#[test]fn antecedente_conserva_bytes_y_solo_cuatro_campos(){let t="ñ\n{\"decision\":\"CONTRADICHA\"}";let v=ante(&json!("A08"),t);assert_eq!(v["respuesta_final"],t);assert_eq!(v["sha256"],sha(t.as_bytes()));assert_eq!(v.as_object().unwrap().len(),4);assert!(v.get("correcta").is_none());}#[test]fn huella_detecta_correccion_silenciosa(){assert_ne!(ante(&json!("A08"),"CONTRADICHA")["sha256"],ante(&json!("A08"),"EVIDENCIA_INSUFICIENTE")["sha256"]);}}
