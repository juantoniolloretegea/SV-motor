//! Preparación delimitada de A3: conserva A0, A1 y A2 íntegros y no contiene la clave de corrección.
use std::{fs,io::Write,path::Path};use serde_json::{json,Value};use sha2::{Digest,Sha256};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn ck(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn val(p:&Path)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn save(p:&Path,b:&[u8])->R<()>{fs::create_dir_all(p.parent().ok_or("Directorio")?)?;let mut f=fs::OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(b)?;f.sync_all()?;Ok(())}
fn anexar(c:&mut Value,prev:&[&Value],nueva:&Value)->R<()>{
 ck(prev.len()==2,"Dos capas previas requeridas")?;
 let ant=c["antecedentes"].as_array().ok_or("Historia")?;ck(ant.len()==2,"Historia previa A0 y A1")?;
 for (j,r) in prev.iter().enumerate(){
  ck(c["id"]==r["datos"]["id"],"Antecedente de otro caso")?;
  let t=r["datos"]["contenido"].as_str().ok_or("Final previo ausente")?;
  ck(ant[j].as_object().is_some_and(|o|o.len()==4)&&ant[j]["id"]==c["id"]&&ant[j]["capa"]==j&&ant[j]["respuesta_final"]==t&&ant[j]["sha256"]==h(t.as_bytes()),"Historia distinta del original")?;
 }
 ck(c["id"]==nueva["datos"]["id"],"Nueva respuesta de otro caso")?;
 let t=nueva["datos"]["contenido"].as_str().ok_or("A2 sin final")?;
 let nuevo=json!({"id":c["id"],"capa":2,"respuesta_final":t,"sha256":h(t.as_bytes())});
 c["antecedentes"].as_array_mut().unwrap().push(nuevo);Ok(())
}
fn copiar_fuentes(origen:&Path,destino:&Path,oldhash:&str,newhash:&str)->R<()>{
 for e in fs::read_dir(origen)?{let e=e?;let p=e.path();let m=fs::symlink_metadata(&p)?;ck(!m.file_type().is_symlink(),"Enlace no autorizado")?;
  if m.is_dir(){copiar_fuentes(&p,&destino.join(e.file_name()),oldhash,newhash)?;}else{
   let mut s=fs::read_to_string(&p)?;s=s.replace("/retroalimentacion-20261003/A2","/retroalimentacion-20261003/A3").replace("retroalimentacion-20261003/A2","retroalimentacion-20261003/A3").replace("retro-A2-01","retro-A3-01").replace("-retro-a2","-retro-a3").replace(oldhash,newhash);
   if e.file_name()=="fijar_entradas.rs"{ck(s.contains("plan[\"capa\"]==2"),"Fijación no reconocida")?;s=s.replace("plan[\"capa\"]==2","plan[\"capa\"]==3");}
   if e.file_name()=="main.rs"&&(origen.ends_with("conductor/src")||origen.ends_with("verificador/src")){
    ck(s.matches("reasoning_effort=>\"medium\"").count()==1,"Esfuerzo previo no reconocido")?;
    s=s.replace("reasoning_effort=>\"medium\"","reasoning_effort=>\"high\"");
    if origen.ends_with("conductor/src") {s=s.replace("let tokens=tokenizer.encode", "anyhow::ensure!(renderizado.contains(\"Reasoning: high\"),\"Esfuerzo alto ausente\");\n let tokens=tokenizer.encode");}
    if origen.ends_with("verificador/src") {s=s.replace("let tokens=tok.encode", "ck(texto.contains(\"Reasoning: high\"),\"Esfuerzo alto ausente\")?;\n  let tokens=tok.encode");}
   }
   save(&destino.join(e.file_name()),s.as_bytes())?;
  }
 }Ok(())
}
fn main()->R<()>{
 let a=std::env::args().nth(1).ok_or("Directorio")?;let b=Path::new(&a);let old=b.join("A2");let new=b.join("A3");ck(!new.exists(),"Preparación anterior preservada")?;
 let audit=val(&b.join("A2-resultados/CUSTODIA-A2.json"))?;let entrada=val(&b.join("A2-resultados/FINAL-A2.json"))?;
 ck(audit["conforme"]==true&&audit["recorrido_completo"]==true&&audit["emisiones"]==9&&audit["sello_sha256"]==h(&fs::read(b.join("A2-resultados/CIERRE.json"))?),"Cierre A2 sin cotejo")?;
 ck(entrada["conforme"]==true&&entrada["modelo_stdout_sha256"]=="bd15d7a19eef44b5a653af2799e674563e5a62568486b665867aa149197e456a","A2 sin cotejo de entrada")?;
 let r0=val(&b.join("A0-resultados/RESPUESTAS-A0.json"))?;let rprev=val(&b.join("A1-resultados/RESPUESTAS-A1.json"))?;let raw1=fs::read(b.join("A2-resultados/RESPUESTAS-A2.json"))?;let r1:Value=serde_json::from_slice(&raw1)?;
 ck(r0.as_array().is_some_and(|a|a.len()==9)&&r1.as_array().is_some_and(|a|a.len()==9)&&rprev.as_array().is_some_and(|a|a.len()==9),"Originales incompletos")?;
 let planraw=fs::read(old.join("config/plan.json"))?;let oldhash=h(&planraw);ck(oldhash=="4c7a980084d6261a33a18401b42abc408ec8af0f92108b1343962c6312082ab1","Plan A2 distinto")?;
 let mut plan:Value=serde_json::from_slice(&planraw)?;ck(plan["capa"]==2&&plan["max_revisiones"]==3&&plan["casos"].as_array().is_some_and(|a|a.len()==9),"Plan previo inválido")?;
 let mut manifest=Vec::new();for(i,c)in plan["casos"].as_array_mut().unwrap().iter_mut().enumerate(){ck(c["id"]==format!("A{:02}",i+1),"Orden")?;anexar(c,&[&r0[i],&rprev[i]],&r1[i])?;
  for ant in c["antecedentes"].as_array().unwrap(){let t=ant["respuesta_final"].as_str().unwrap();let capa=ant["capa"].as_u64().unwrap();let id=c["id"].as_str().unwrap();save(&new.join(format!("pensamiento-afinado/A{capa}/{id}.txt")),t.as_bytes())?;manifest.push(json!({"id":id,"capa":capa,"bytes":t.len(),"sha256":h(t.as_bytes()),"original_completo":true}));}
 }
 plan["capa"]=json!(3);let pb=serde_json::to_vec_pretty(&plan)?;let newhash=h(&pb);save(&new.join("config/plan.json"),&pb)?;
 let mut contrato=val(&old.join("config/contrato.json"))?;contrato["capa"]=json!(3);contrato["plan_sha256"]=json!(newhash);
 for n in ["cache/catalogo.json","config/politica.txt","config/MODELO-REFERENCIA.json","config/NUCLEO-REFERENCIA.json"]{save(&new.join(n),&fs::read(old.join(n))?)?;}
 ck(contrato["catalogo_sha256"]==h(&fs::read(new.join("cache/catalogo.json"))?)&&contrato["politica_sha256"]==h(&fs::read(new.join("config/politica.txt"))?),"Fuentes o política distintas")?;
 save(&new.join("config/contrato.json"),&serde_json::to_vec_pretty(&contrato)?)?;
 for cr in ["instrumentacion","conductor","custodio","verificador"]{copiar_fuentes(&old.join(cr).join("src"),&new.join(cr).join("src"),&oldhash,&newhash)?;let s=fs::read_to_string(old.join(cr).join("Cargo.toml"))?.replace("-retro-a2","-retro-a3");save(&new.join(cr).join("Cargo.toml"),s.as_bytes())?;}
 save(&new.join("preparar.rs"),fs::read_to_string(old.join("preparar.rs"))?.replace("/A2","/A3").as_bytes())?;fs::create_dir_all(new.join("autorizaciones"))?;
 let condicion=json!({"capa":3,"razonamiento_anterior":"medium","razonamiento_actual":"high","autorizacion":"Instrucción humana expresa del 03/10/2026 para elevar el razonamiento en A3","cambio_adicional_a_antecedentes":"Sólo esfuerzo de razonamiento; política, fuentes, selección, semilla, límites y pesos conservados","comparabilidad":"A2 a A3 combina otra revisión contextual con mayor esfuerzo; no identifica el efecto causal aislado de ninguno","nucleo_semantica_ir_y_pesos_intactos":true});
 save(&new.join("CONDICION-A3.json"),&serde_json::to_vec_pretty(&condicion)?)?;
 let reporte=json!({"conforme_preparacion":true,"capa":3,"max_revisiones":3,"original_a2_sha256":entrada["modelo_stdout_sha256"],"respuestas_a2_sha256":h(&raw1),"plan_a2_sha256":oldhash,"plan_a3_sha256":newhash,"antecedentes":manifest,"razonamiento":"high","politica_y_fuentes_identicas":true,"clave_entregada":false,"compilacion_realizada":false,"inferencia_iniciada":false});
 save(&new.join("cotejos/ANTECEDENTES-A0-A1-A2.json"),&serde_json::to_vec_pretty(&reporte)?)?;println!("A3 preparada, 27 antecedentes íntegros, plan {newhash}; aún sin compilar ni inferir");Ok(())
}
#[cfg(test)]mod tests{use super::*;
fn respuesta(id:&str,t:&str)->Value{json!({"datos":{"id":id,"contenido":t}})}
fn caso()->Value{json!({"id":"A08","antecedentes":[{"id":"A08","capa":0,"respuesta_final":"ñ 0","sha256":h("ñ 0".as_bytes())},{"id":"A08","capa":1,"respuesta_final":"ñ 1","sha256":h("ñ 1".as_bytes())}]})}
#[test]fn conserva_tres_originales(){let mut c=caso();let(a,b,n)=(respuesta("A08","ñ 0"),respuesta("A08","ñ 1"),respuesta("A08","ñ 2"));anexar(&mut c,&[&a,&b],&n).unwrap();assert_eq!(c["antecedentes"][2]["respuesta_final"],"ñ 2");assert_eq!(c["antecedentes"][2]["capa"],2);assert_eq!(c["antecedentes"][2].as_object().unwrap().len(),4);assert!(anexar(&mut c,&[&a,&b],&n).is_err());}
#[test]fn rechaza_modificacion_cualquier_capa(){let(a,b,n)=(respuesta("A08","ñ 0"),respuesta("A08","ñ 1"),respuesta("A08","ñ 2"));for j in 0..2{let mut c=caso();c["antecedentes"][j]["respuesta_final"]=json!("modificada");assert!(anexar(&mut c,&[&a,&b],&n).is_err());}}
#[test]fn rechaza_cruce(){let(a,b,n)=(respuesta("A08","ñ 0"),respuesta("A07","ñ 1"),respuesta("A08","ñ 2"));assert!(anexar(&mut caso(),&[&a,&b],&n).is_err());}
#[test]fn rechaza_reorden_y_campos_ajenos(){let(a,b,n)=(respuesta("A08","ñ 0"),respuesta("A08","ñ 1"),respuesta("A08","ñ 2"));let mut c=caso();c["antecedentes"].as_array_mut().unwrap().swap(0,1);assert!(anexar(&mut c,&[&a,&b],&n).is_err());let mut c=caso();c["antecedentes"][0]["correccion"]=json!("ajena");assert!(anexar(&mut c,&[&a,&b],&n).is_err());}
}
