use std::{fs,io::Write,path::{Path,Component}};use serde_json::{Value,json};use sha2::{Digest,Sha256};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn ck(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn save(p:&Path,b:&[u8])->R<()>{fs::create_dir_all(p.parent().ok_or("Directorio")?)?;let mut f=fs::OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(b)?;f.sync_all()?;Ok(())}
fn rep(s:&mut String,a:&str,b:&str)->R<()>{ck(s.contains(a),&format!("Patrón ausente: {a}"))?;*s=s.replace(a,b);Ok(())}
fn copy(src:&Path,dst:&Path)->R<()>{for e in fs::read_dir(src)?{let e=e?;let p=e.path();let m=fs::symlink_metadata(&p)?;ck(!m.file_type().is_symlink(),"Enlace rechazado")?;if m.is_dir(){copy(&p,&dst.join(e.file_name()))?;continue}
 let mut s=fs::read_to_string(&p)?.replace("diagnostico-A08-02","diagnostico-A08-03").replace("diag-A08-02","diag-A08-03").replace("diag-a08-02","diag-a08-03").replace("diag02","diag03").replace("INSTRUMENTAL-02","INSTRUMENTAL-03");
 if p.ends_with("custodio/src/main.rs"){
  rep(&mut s,"Duration::from_secs(3600)","Duration::from_secs(9000)")?;
  rep(&mut s,"Cota total de una hora alcanzada; conservar sin reintento","Cota total de dos horas y media alcanzada; conservar sin cuarto intento")?;
 }
 if p.ends_with("conductor/src/main.rs"){rep(&mut s,"\"intento\":2,\"razonamiento\":\"high\"","\"intento\":3,\"razonamiento\":\"high\"")?;}
 if p.ends_with("instrumentacion/src/bin/recuperar.rs"){
  rep(&mut s,"[\"intento\"]!=2","[\"intento\"]!=3")?;
  rep(&mut s,"\"intento\":2,\"caso\":\"A08\"","\"intento\":3,\"caso\":\"A08\"")?;
 }
 save(&dst.join(e.file_name()),s.as_bytes())?;
 }Ok(())}
fn inventory(base:&Path,p:&Path,rows:&mut Vec<Value>)->R<()>{for e in fs::read_dir(p)?{let e=e?;let p=e.path();ck(!fs::symlink_metadata(&p)?.file_type().is_symlink(),"Enlace rechazado")?;if p.is_dir(){inventory(base,&p,rows)?;}else{let rel=p.strip_prefix(base)?;ck(rel.components().all(|c|matches!(c,Component::Normal(_))),"Ruta")?;let b=fs::read(&p)?;rows.push(json!({"ruta":rel.to_string_lossy().replace('\\',"/"),"bytes":b.len(),"sha256":h(&b)}));}}Ok(())}
fn main()->R<()>{
 let arg=std::env::args().nth(1).ok_or("Directorio")?;let base=Path::new(&arg);let old=base.join("diagnostico-A08-02");let new=base.join("diagnostico-A08-03");ck(!new.exists(),"D03 ya preparado")?;
 let raw=fs::read(old.join("cotejos/INCIDENTE-D02.json"))?;ck(raw.len()==3645&&h(&raw)=="8b69d7d51b2e2eef9a002aaa09cab516f781968185328d6d1856b0945c9a1dbb","Incidente distinto de la huella remota Rust")?;
 let r:Value=serde_json::from_slice(&raw)?;ck(r["intento"]==2&&r["custodia"]["conforme"]==true&&r["custodia"]["recorrido_completo"]==false&&r["custodia"]["emisiones"]==0&&r["tokens_generados"]==1982&&r["respuesta_final"].is_null(),"Resultado D02 discordante")?;
 for n in ["conductor","custodio","instrumentacion","verificador","pensamiento-afinado"]{copy(&old.join(n),&new.join(n))?;}
 for n in ["cache/catalogo.json","config/politica.txt","config/plan.json","config/contrato.json","config/MODELO-REFERENCIA.json","config/NUCLEO-REFERENCIA.json"]{save(&new.join(n),&fs::read(old.join(n))?)?;}
 let prep=fs::read_to_string(old.join("preparar.rs"))?.replace("diagnostico-A08-02","diagnostico-A08-03");save(&new.join("preparar.rs"),prep.as_bytes())?;
 for n in ["cotejos","autorizaciones"]{fs::create_dir_all(new.join(n))?;}
 let condition=json!({"intento":3,"maximo_total_intentos":3,"caso":"A08","referencia":"D02","cambio":"Ampliación de cota total de 3600 a 9000 segundos; ninguna alteración de entrada o parámetros de generación","fundamento":"D02 fue interrumpido con 1982 tokens, sin final y mientras progresaba, por el límite administrativo de una hora. Su tasa observada entre extremos fue 0.6087605618 tokens/s. El margen de 9000 s cubre aproximadamente 4096 tokens a esa tasa, carga y preparación, con holgura; no garantiza el tiempo futuro ni obliga a agotar la cota.","autorizacion":"Orden humana expresa de activar y concluir la prueba, manteniendo el máximo total de tres intentos","razonamiento":"high","temperatura":1.0,"top_p":1.0,"top_k":null,"semilla":42,"penalizaciones":false,"salida_maxima_tokens":4096,"limite_total_segundos":9000,"limite_sin_progreso_segundos":900,"parada_exterior_segundos":9900,"generaciones":1,"fuentes_politica_pregunta_antecedentes_identicos_a_d02":true,"antecedentes":2,"resultados_d01_d02_entregados":false,"clave_entregada":false,"custodio_gib":2,"modelo_gib":110,"conjunto_gib":114,"nuevos_recursos":false,"nucleo_semantica_ir_pesos":"Intactos","limite_final":"No existe cuarto diagnóstico automático. Conservar limitaciones instrumentales separadas; no fabricar U ni respuesta final. Un acierto aislado no autoriza el examen.","conservacion_github":"Final pospuesta hasta terminar las pruebas"});
 save(&new.join("CONDICION.json"),&serde_json::to_vec_pretty(&condition)?)?;
 let mut rows=Vec::new();inventory(&new,&new,&mut rows)?;rows.sort_by_key(|r|r["ruta"].as_str().unwrap().to_string());save(&new.join("MANIFIESTO-PREPARACION.json"),&serde_json::to_vec_pretty(&json!({"intento":3,"archivos":rows,"sin_clave":true,"incidente_d02_recuperado_sha256":h(&raw)}))?)?;
 println!("D02 recibido y cotejado; D03 preparado. Misma entrada y generación; cota 9000 s. Tercer y último intento, aún sin inferencia.");Ok(())
}
