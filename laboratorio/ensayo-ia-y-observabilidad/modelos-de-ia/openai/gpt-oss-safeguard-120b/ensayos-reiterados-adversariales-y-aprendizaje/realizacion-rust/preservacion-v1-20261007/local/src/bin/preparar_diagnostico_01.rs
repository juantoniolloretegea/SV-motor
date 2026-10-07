//! Diagnóstico único A08: selección de tokens frente a la referencia A2, sin nueva capa.
use std::{fs,io::Write,path::Path};
use serde_json::{json,Value}; use sha2::{Digest,Sha256};
type R<T> = Result<T,Box<dyn std::error::Error>>;
fn ck(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn save(p:&Path,b:&[u8])->R<()>{fs::create_dir_all(p.parent().ok_or("Directorio")?)?;let mut f=fs::OpenOptions::new().write(true).create_new(true).open(p)?;f.write_all(b)?;f.sync_all()?;Ok(())}
fn swap(s:&mut String,a:&str,b:&str)->R<()>{ck(s.contains(a),&format!("Patrón no encontrado: {a}"))?;*s=s.replace(a,b);Ok(())}
fn fuentes(src:&Path,dst:&Path,plan_sha:&str)->R<()>{
 for e in fs::read_dir(src)?{let e=e?;let p=e.path();let meta=fs::symlink_metadata(&p)?;ck(!meta.file_type().is_symlink(),"Enlace rechazado")?;
  if meta.is_dir(){fuentes(&p,&dst.join(e.file_name()),plan_sha)?;continue}
  if e.file_name()=="origen.rs"{continue}
  let mut s=fs::read_to_string(&p)?.replace("\r\n","\n");
  s=s.replace("retroalimentacion-20261003/A2","retroalimentacion-20261003/diagnostico-A08-01")
   .replace("-retro-a2","-diag-a08-01").replace("retro-A2-01","diag-A08-01")
   .replace("4c7a980084d6261a33a18401b42abc408ec8af0f92108b1343962c6312082ab1",plan_sha)
   .replace("svsafeguard.slice","svsgdiag01.slice").replace("sv-retro-","sv-diag01-");
  let name=e.file_name();let name=name.to_string_lossy();
  if name=="retroalimentacion.rs"{
   swap(&mut s,"pub const NUM_CASOS: usize = 9;","pub const NUM_CASOS: usize = 1;")?;
   swap(&mut s,"pub const SALIDA: usize = 8192;","pub const SALIDA: usize = 2048;")?;
   s=s.replace("SG-RETROALIMENTACION-20261003","SG-DIAGNOSTICO-A08-20261004");
   swap(&mut s,"let id = format!(\"{bloque}{:02}\", i + 1);","let id = String::from(\"A08\");")?;
   s=s.replace("(1..=9)","(1..=1)").replace("format!(\"A{i:02}\")","String::from(\"A08\")");
  }
  if name=="ciclo.rs"{
   swap(&mut s,"pub const IDS:[&str;9]=[\"A01\",\"A02\",\"A03\",\"A04\",\"A05\",\"A06\",\"A07\",\"A08\",\"A09\"];","pub const IDS:[&str;1]=[\"A08\"];")?;
   swap(&mut s,"pub const DOCS:[&str;9]=[\"BANCO-A\";9];","pub const DOCS:[&str;1]=[\"BANCO-A\";1];")?;
   s=s.split("#[cfg(test)]mod tests{").next().ok_or("Pruebas")?.to_string();
   s.push_str(r#"
#[cfg(test)]mod tests{
 use super::*;
 fn entrada()->Value{let t=json!([10]);json!({"id":"A08","documento":"BANCO-A","seccion":"A08","tokens":t,"tokens_sha256":huella(&serde_json::to_vec(&t).unwrap()),"max_salida":SALIDA,"ronda":0,"paginas":[0,1],"funciones":[],"mensajes":["sintetico"],"plantilla_efectiva":"sintetica"})}
 fn preparada()->Puerta{let mut p=Puerta::nueva(&crate::retroalimentacion::plan_prueba(0)).unwrap();let v=entrada();p.preparar(&v,Some(&v)).unwrap();p.solicitar("preparacion","A08",v["tokens_sha256"].as_str().unwrap(),"contraste").unwrap();p.solicitar("carga","A08",v["tokens_sha256"].as_str().unwrap(),"contraste").unwrap();p}
 fn emitida()->Puerta{let mut p=preparada();let v=entrada();p.motor(&v).unwrap();p.solicitar("generacion","A08",v["tokens_sha256"].as_str().unwrap(),"contraste").unwrap();p.emision(&json!({"id":"A08","texto":"original","tokens":[1]})).unwrap();p.terminar(&json!({"id":"A08","completo":true})).unwrap();p.solicitar("adjudicacion","A08",v["tokens_sha256"].as_str().unwrap(),"contraste").unwrap();p}
 #[test]fn unico_caso_sin_ampliacion(){let mut p=emitida();assert!(p.adjudicar(&json!({"id":"A08","salida_sha256":huella(b"original"),"accion":"continuar"})).is_err());p.adjudicar(&json!({"id":"A08","salida_sha256":huella(b"original"),"accion":"cerrar"})).unwrap();assert!(p.cerrado());assert!(p.motor(&entrada()).is_err());}
 #[test]fn no_carga_ni_generacion_repetida(){let mut p=preparada();assert!(p.solicitar("carga","A08",entrada()["tokens_sha256"].as_str().unwrap(),"contraste").is_err());let mut p=emitida();assert!(p.motor(&entrada()).is_err());}
 #[test]fn original_y_clave_protegidos(){let mut p=emitida();assert!(p.adjudicar(&json!({"id":"A08","salida_sha256":"falsa","accion":"cerrar"})).is_err());assert!(p.adjudicar(&json!({"id":"A08","salida_sha256":huella(b"original"),"accion":"cerrar","solucion":"ajena"})).is_err());}
 #[test]fn fuente_y_contexto_fijados(){let mut p=preparada();let mut v=entrada();v["mensajes"]=json!(["alterado"]);assert!(p.motor(&v).is_err());let b=serde_json::to_vec(&json!({"method":"tools/call","params":{"name":"leer_documento","arguments":{"documento":"BANCO-A","seccion":"A09","pagina":0}}})).unwrap();assert!(validar_solicitud(&b,"BANCO-A","A08").is_err());}
}
"#);
  }
  if name=="main.rs"&&src.ends_with("conductor/src"){
   swap(&mut s,"casos.len()==9","casos.len()==1")?;
   swap(&mut s,"id==format!(\"{}{:02}\",plan[\"bloque\"].as_str().unwrap(),i+1)","id==\"A08\"")?;
   s=s.replace("\"rutas_completas\":9","\"rutas_completas\":1");
   swap(&mut s,"let f=esperadas(\"BANCO-A\",\"A09\")?","let f=esperadas(\"BANCO-A\",\"A08\")?")?;
   swap(&mut s,"let mut sampling=SamplingParams::deterministic();","let mut sampling=SamplingParams::neutral(); sampling.temperature=Some(1.0); sampling.top_p=Some(1.0);\n  emit(json!({\"evento\":\"configuracion_generacion_solicitada\",\"intento\":1,\"razonamiento\":\"medium\",\"semilla\":42,\"sampling\":sampling}));")?;
   s.push_str("\n#[test]fn muestreo_diagnostico_sin_restriccion_topk(){let mut s=SamplingParams::neutral();s.temperature=Some(1.0);s.top_p=Some(1.0);assert_eq!(s.top_k,None);assert_eq!(s.min_p,None);assert_eq!(s.repetition_penalty,None);assert_eq!(s.frequency_penalty,None);assert_eq!(s.presence_penalty,None);assert_eq!(s.n_choices,1);}\n");
  }
  if name=="main.rs"&&src.ends_with("verificador/src"){
   s=s.replace("previas.len()==9&&reales.len()<=9","previas.len()==1&&reales.len()<=1").replace("\"entradas_prefijadas\":9","\"entradas_prefijadas\":1");
  }
  if name=="main.rs"&&src.ends_with("custodio/src"){
   s=s.replace("\"112G\"","\"110G\"").replace("entradas<9","entradas<1");
   swap(&mut s,"let mut pendiente:Option<Value>=None;let mut progreso=std::time::Instant::now();","let mut pendiente:Option<Value>=None;let inicio_diagnostico=std::time::Instant::now();let mut progreso=std::time::Instant::now();")?;
   swap(&mut s,"if progreso.elapsed()>Duration::from_secs(3600)","if inicio_diagnostico.elapsed()>Duration::from_secs(3600){return Err(\"Cota total de una hora alcanzada; conservar sin reintento\".into())}\n   if progreso.elapsed()>Duration::from_secs(900)")?;
   s=s.replace("Una hora sin avance", "Quince minutos sin avance");
   let pos=s.find("#[cfg(test)]mod tests{").ok_or("Pruebas del custodio")?;
   s.truncate(pos);
   s.push_str(r#"
#[cfg(test)] mod tests{use super::*;
 #[test]fn admision_unica(){assert!(comprobar_admision_contraste(&json!([{}])).is_ok());for n in [0,2,3,8,9,10]{assert!(comprobar_admision_contraste(&json!(vec![json!({});n])).is_err());}}
 #[test]fn guardas_y_aislamiento(){let c=sandbox("sv-diag01-prueba","sv-sg-engine","110G");let a=c.get_args().map(|x|x.to_string_lossy().into_owned()).collect::<Vec<_>>();for p in ["--property=PrivateNetwork=yes","--property=MemorySwapMax=0","--property=MemoryMax=110G","--property=ProtectSystem=strict"]{assert!(a.contains(&p.to_string()));}assert!(a.iter().any(|s|s.contains("diagnostico-A08-01/config:/config")));}
}
"#);
  }
  if name=="fijar_entradas.rs"{s=s.replace("[\"entradas_prefijadas\"]==9","[\"entradas_prefijadas\"]==1").replace("entradas.len()==9","entradas.len()==1").replace("Nueve entradas","Una entrada");}
  if name=="preparar.rs"{s=s.replace("compilacion-01","compilacion-01");}
  if name=="Cargo.toml"&&src.ends_with("instrumentacion"){
   s.push_str("\nautobins = false\n"); // Se coloca correctamente antes de las tablas al final de esta función.
   s=s.replace("\nautobins = false\n","").replace("publish=false","publish=false\nautobins=false");
   s.push_str("\n[[bin]]\nname=\"comprobar-base-diag01\"\npath=\"src/bin/comprobar_base.rs\"\n[[bin]]\nname=\"fijar-entradas-diag01\"\npath=\"src/bin/fijar_entradas.rs\"\n");
  }
  save(&dst.join(e.file_name()),s.as_bytes())?;
 }
 Ok(())
}
fn main()->R<()>{
 let arg=std::env::args().nth(1).ok_or("Directorio")?;let b=Path::new(&arg);let prev=b.join("A2");let new=b.join("diagnostico-A08-01");ck(!new.exists(),"La preparación ya existe")?;
 let raw=fs::read(prev.join("config/plan.json"))?;ck(h(&raw)=="4c7a980084d6261a33a18401b42abc408ec8af0f92108b1343962c6312082ab1","Referencia A2 alterada")?;
 let mut p:Value=serde_json::from_slice(&raw)?;let caso=p["casos"][7].clone();ck(caso["id"]=="A08","Caso distinto")?;
 p["casos"]=json!([caso]);p["campana"]=json!("SG-DIAGNOSTICO-A08-20261004");let raw=serde_json::to_vec_pretty(&p)?;let plan_sha=h(&raw);
 for cr in ["instrumentacion","conductor","custodio","verificador"]{fuentes(&prev.join(cr),&new.join(cr),&plan_sha)?;}
 for n in ["cache/catalogo.json","config/politica.txt","config/MODELO-REFERENCIA.json","config/NUCLEO-REFERENCIA.json"]{save(&new.join(n),&fs::read(prev.join(n))?)?;}
 save(&new.join("config/plan.json"),&raw)?;
 let mut c:Value=serde_json::from_slice(&fs::read(prev.join("config/contrato.json"))?)?;c["campana"]=p["campana"].clone();c["plan_sha256"]=json!(plan_sha);c["salida_maxima_tokens"]=json!(2048);save(&new.join("config/contrato.json"),&serde_json::to_vec_pretty(&c)?)?;
 let s=fs::read_to_string(prev.join("preparar.rs"))?.replace("/A2","/diagnostico-A08-01");save(&new.join("preparar.rs"),s.as_bytes())?;
 for a in p["casos"][0]["antecedentes"].as_array().ok_or("Antecedentes")?{let t=a["respuesta_final"].as_str().ok_or("Final")?;ck(a["sha256"]==h(t.as_bytes()),"Antecedente alterado")?;save(&new.join(format!("pensamiento-afinado/A{}/A08.txt",a["capa"])),t.as_bytes())?;}
 fs::create_dir_all(new.join("autorizaciones"))?;fs::create_dir_all(new.join("cotejos"))?;
 let r=json!({"intento":1,"maximo_total_intentos":3,"caso":"A08","referencia":"A2/A08","capa_del_contexto_reproducido":2,"nueva_capa":false,"razonamiento":"medium","cambio":"SamplingParams::deterministic (top_k=1) a neutral con temperature=1.0 y top_p=1.0; semilla 42 conservada; sin penalizaciones ni filtros adicionales","fuentes_politica_afirmacion_antecedentes_identicos":true,"salida_maxima_tokens":2048,"limite_total_segundos":3600,"limite_sin_progreso_segundos":900,"limite_no_alcanzado_en_referencia":true,"custodio_gib":2,"modelo_gib":110,"conjunto_gib":114,"sin_intercambio":true,"pesos_nucleo_semantica_ir_intactos":true,"criterio":"Clasificación y argumento conforme a la clave reservada, además de requisitos formales; no entregar la clave. Una salida truncada o fallo instrumental no se convierte en respuesta errónea ni U. Un acierto aislado no habilita el examen.","condicion_de_parada":"Una generación como máximo. No repetir con otra semilla. Si resuelve, comunicar alcance antes de nuevos bancos. Si no resuelve, revisar resultado y fundamento del siguiente diagnóstico dentro del máximo total de tres.","limite_causal":"Si la salida alcanza la nueva cota, no atribuir el desenlace sólo al muestreo; temperatura es una hipótesis de configuración, no una solución demostrada para Safeguard.","github_final":"Pospuesto por instrucción humana hasta terminar las pruebas del modelo","plan_sha256":plan_sha});
 save(&new.join("CONDICION.json"),&serde_json::to_vec_pretty(&r)?)?;
 println!("D01 preparado sin carga ni inferencia; plan {plan_sha}");Ok(())
}
