use std::{fs,io::Write,path::Path};use serde_json::{json,Value};use sha2::{Digest,Sha256};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn val(p:&Path)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn main()->R<()>{
 let arg=std::env::args().nth(1).ok_or("Directorio")?;let b=Path::new(&arg);let d=b.join("diagnostico-A08-01");let a=b.join("A2");
 let anterior=val(&a.join("config/ADMISION.json"))?;let actual=val(&d.join("config/ADMISION.json"))?;
 if actual.as_array().map(Vec::len)!=Some(1)||actual[0]["id"]!="A08"{return Err("No es el único caso A08".into())}
 let mut claves=Vec::new();for k in ["id","documento","seccion","ronda","mensajes","funciones","plantilla_efectiva","tokens","tokens_sha256","paginas"]{
  if anterior[7][k]!=actual[0][k]{return Err(format!("Entrada distinta de A2: {k}").into())}claves.push(k);
 }
 if anterior[7]["max_salida"]!=8192||actual[0]["max_salida"]!=2048{return Err("Cotas distintas de las declaradas".into())}
 let plan=val(&d.join("config/plan.json"))?;let previo=val(&a.join("config/plan.json"))?;
 if plan["casos"][0]!=previo["casos"][7]{return Err("Pregunta o antecedentes cambiados".into())}
 for path in ["config/politica.txt","cache/catalogo.json","config/MODELO-REFERENCIA.json","config/NUCLEO-REFERENCIA.json"]{if fs::read(d.join(path))?!=fs::read(a.join(path))?{return Err(format!("Referencia cambiada: {path}").into())}}
 let informe=json!({"conforme":true,"caso":"A08","intento":1,"referencia":"A2/A08","entrada_tokens":actual[0]["tokens"].as_array().ok_or("Tokens")?.len(),"tokens_sha256":actual[0]["tokens_sha256"],"campos_identicos":claves,"politica_fuentes_afirmacion_antecedentes_identicos":true,"razonamiento":"medium","entrada_identica_byte_a_byte":true,"salida_maxima_anterior":8192,"salida_maxima_actual":2048,"plan_sha256":h(&fs::read(d.join("config/plan.json"))?),"admision_sha256":h(&fs::read(d.join("config/ADMISION.json"))?),"clave_entregada":false,"inferencia_iniciada_por_este_comprobador":false});
 let mut f=fs::OpenOptions::new().write(true).create_new(true).open(d.join("cotejos/VINCULO-A2.json"))?;f.write_all(&serde_json::to_vec_pretty(&informe)?)?;f.sync_all()?;println!("{informe}");Ok(())
}
