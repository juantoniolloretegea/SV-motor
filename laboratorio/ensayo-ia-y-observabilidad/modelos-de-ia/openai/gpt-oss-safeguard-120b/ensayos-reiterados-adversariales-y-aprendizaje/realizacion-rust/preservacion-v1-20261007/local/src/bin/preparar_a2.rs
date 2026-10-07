//! Preparación delimitada de A2: conserva A0 y A1 íntegros y no contiene la clave de corrección.
use std::{fs,io::Write,path::Path};use serde_json::{json,Value};use sha2::{Digest,Sha256};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn ck(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn val(p:&Path)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn save(p:&Path,b:&[u8])->R<()>{fs::create_dir_all(p.parent().ok_or("Directorio")?)?;let mut f=fs::OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(b)?;f.sync_all()?;Ok(())}
fn anexar(c:&mut Value,prev:&Value,nueva:&Value)->R<()>{
 ck(c["id"]==prev["datos"]["id"]&&c["id"]==nueva["datos"]["id"],"Antecedente de otro caso")?;
 let ant=c["antecedentes"].as_array().ok_or("Historia")?;ck(ant.len()==1,"Historia previa debe contener sólo A0")?;
 let texto0=prev["datos"]["contenido"].as_str().ok_or("A0 sin final")?;
 ck(ant[0].as_object().is_some_and(|o|o.len()==4)&&ant[0]["id"]==c["id"]&&ant[0]["capa"]==0&&ant[0]["respuesta_final"]==texto0&&ant[0]["sha256"]==h(texto0.as_bytes()),"Historia A0 distinta del original")?;
 let texto1=nueva["datos"]["contenido"].as_str().ok_or("A1 sin final")?;
 let nuevo=json!({"id":c["id"],"capa":1,"respuesta_final":texto1,"sha256":h(texto1.as_bytes())});
 c["antecedentes"].as_array_mut().unwrap().push(nuevo);Ok(())
}
fn copiar_fuentes(origen:&Path,destino:&Path,oldhash:&str,newhash:&str)->R<()>{
 for e in fs::read_dir(origen)?{let e=e?;let p=e.path();let m=fs::symlink_metadata(&p)?;ck(!m.file_type().is_symlink(),"Enlace no autorizado")?;
  if m.is_dir(){copiar_fuentes(&p,&destino.join(e.file_name()),oldhash,newhash)?;}else{
   let mut s=fs::read_to_string(&p)?;s=s.replace("/retroalimentacion-20261003/A1","/retroalimentacion-20261003/A2").replace("retroalimentacion-20261003/A1","retroalimentacion-20261003/A2").replace("retro-A1-01","retro-A2-01").replace("-retro-a1","-retro-a2").replace(oldhash,newhash);
   if e.file_name()=="fijar_entradas.rs"{ck(s.contains("plan[\"capa\"]==1"),"Fijación no reconocida")?;s=s.replace("plan[\"capa\"]==1","plan[\"capa\"]==2");}
   save(&destino.join(e.file_name()),s.as_bytes())?;
  }
 }Ok(())
}
fn main()->R<()>{
 let a=std::env::args().nth(1).ok_or("Directorio")?;let b=Path::new(&a);let old=b.join("A1");let new=b.join("A2");ck(!new.exists(),"Preparación anterior preservada")?;
 let audit=val(&b.join("A1-resultados/CUSTODIA-A1.json"))?;let entrada=val(&b.join("A1-resultados/FINAL-A1.json"))?;
 ck(audit["conforme"]==true&&audit["recorrido_completo"]==true&&audit["emisiones"]==9&&audit["sello_sha256"]==h(&fs::read(b.join("A1-resultados/CIERRE.json"))?),"Cierre A1 sin cotejo")?;
 ck(entrada["conforme"]==true&&entrada["modelo_stdout_sha256"]=="adb829044f50abdfca1125ce56afa8ade23f6847edb2e99b25f61918fb16cf44","A1 sin cotejo de entrada")?;
 let r0=val(&b.join("A0-resultados/RESPUESTAS-A0.json"))?;let raw1=fs::read(b.join("A1-resultados/RESPUESTAS-A1.json"))?;let r1:Value=serde_json::from_slice(&raw1)?;
 ck(r0.as_array().is_some_and(|a|a.len()==9)&&r1.as_array().is_some_and(|a|a.len()==9),"Originales incompletos")?;
 let planraw=fs::read(old.join("config/plan.json"))?;let oldhash=h(&planraw);ck(oldhash=="ffe2513591c8d6478000a304940a6054fdc039e1373a902cda441cf54ab9714c","Plan A1 distinto")?;
 let mut plan:Value=serde_json::from_slice(&planraw)?;ck(plan["capa"]==1&&plan["max_revisiones"]==3&&plan["casos"].as_array().is_some_and(|a|a.len()==9),"Plan previo inválido")?;
 let mut manifest=Vec::new();for(i,c)in plan["casos"].as_array_mut().unwrap().iter_mut().enumerate(){ck(c["id"]==format!("A{:02}",i+1),"Orden")?;anexar(c,&r0[i],&r1[i])?;
  for ant in c["antecedentes"].as_array().unwrap(){let t=ant["respuesta_final"].as_str().unwrap();let capa=ant["capa"].as_u64().unwrap();let id=c["id"].as_str().unwrap();save(&new.join(format!("pensamiento-afinado/A{capa}/{id}.txt")),t.as_bytes())?;manifest.push(json!({"id":id,"capa":capa,"bytes":t.len(),"sha256":h(t.as_bytes()),"original_completo":true}));}
 }
 plan["capa"]=json!(2);let pb=serde_json::to_vec_pretty(&plan)?;let newhash=h(&pb);save(&new.join("config/plan.json"),&pb)?;
 let mut contrato=val(&old.join("config/contrato.json"))?;contrato["capa"]=json!(2);contrato["plan_sha256"]=json!(newhash);
 for n in ["cache/catalogo.json","config/politica.txt","config/MODELO-REFERENCIA.json","config/NUCLEO-REFERENCIA.json"]{save(&new.join(n),&fs::read(old.join(n))?)?;}
 ck(contrato["catalogo_sha256"]==h(&fs::read(new.join("cache/catalogo.json"))?)&&contrato["politica_sha256"]==h(&fs::read(new.join("config/politica.txt"))?),"Fuentes o política distintas")?;
 save(&new.join("config/contrato.json"),&serde_json::to_vec_pretty(&contrato)?)?;
 for cr in ["instrumentacion","conductor","custodio","verificador"]{copiar_fuentes(&old.join(cr).join("src"),&new.join(cr).join("src"),&oldhash,&newhash)?;let s=fs::read_to_string(old.join(cr).join("Cargo.toml"))?.replace("-retro-a1","-retro-a2");save(&new.join(cr).join("Cargo.toml"),s.as_bytes())?;}
 save(&new.join("preparar.rs"),fs::read_to_string(old.join("preparar.rs"))?.replace("/A1","/A2").as_bytes())?;fs::create_dir_all(new.join("autorizaciones"))?;
 let reporte=json!({"conforme_preparacion":true,"capa":2,"max_revisiones":3,"original_a1_sha256":entrada["modelo_stdout_sha256"],"respuestas_a1_sha256":h(&raw1),"plan_a1_sha256":oldhash,"plan_a2_sha256":newhash,"antecedentes":manifest,"politica_y_fuentes_identicas":true,"clave_entregada":false,"compilacion_realizada":false,"inferencia_iniciada":false});
 save(&new.join("cotejos/ANTECEDENTES-A0-A1.json"),&serde_json::to_vec_pretty(&reporte)?)?;println!("A2 preparada, 18 antecedentes íntegros, plan {newhash}; aún sin compilar ni inferir");Ok(())
}
#[cfg(test)]mod tests{use super::*;fn caso()->Value{let t="ñ\n{\"decision\":\"CONTRADICHA\"}";json!({"id":"A08","antecedentes":[{"id":"A08","capa":0,"respuesta_final":t,"sha256":h(t.as_bytes())}]})}fn respuesta()->Value{json!({"datos":{"id":"A08","contenido":"ñ\n{\"decision\":\"CONTRADICHA\"}"}})}
 #[test]fn rechaza_original_alterado(){let mut c=caso();c["antecedentes"][0]["respuesta_final"]=json!("corregida");assert!(anexar(&mut c,&respuesta(),&respuesta()).is_err());}
 #[test]fn rechaza_cruce_entre_casos(){let mut c=caso();let mut r=respuesta();r["datos"]["id"]=json!("A07");assert!(anexar(&mut c,&respuesta(),&r).is_err());}
 #[test]fn conserva_ambas_capas_sin_adjudicaciones(){let mut c=caso();anexar(&mut c,&respuesta(),&respuesta()).unwrap();let a=c["antecedentes"].as_array().unwrap();assert_eq!(a.len(),2);assert_eq!(a[0]["respuesta_final"],a[1]["respuesta_final"]);assert_eq!(a[1]["capa"],1);assert_eq!(a[1].as_object().unwrap().len(),4);assert!(anexar(&mut c,&respuesta(),&respuesta()).is_err());}
}
