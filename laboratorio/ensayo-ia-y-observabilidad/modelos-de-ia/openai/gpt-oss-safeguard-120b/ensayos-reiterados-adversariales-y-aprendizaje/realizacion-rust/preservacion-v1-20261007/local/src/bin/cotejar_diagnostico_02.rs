use std::{fs,io::Write,path::Path};use serde_json::{Value,json};use sha2::{Digest,Sha256};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn ck(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn main()->R<()>{let a=std::env::args().nth(1).ok_or("Directorio")?;let b=Path::new(&a);let prev=b.join("diagnostico-A08-01");let new=b.join("diagnostico-A08-02");
 let p:Value=serde_json::from_slice(&fs::read(prev.join("config/ADMISION.json"))?)?;let n:Value=serde_json::from_slice(&fs::read(new.join("config/ADMISION.json"))?)?;
 ck(p.as_array().map(Vec::len)==Some(1)&&n.as_array().map(Vec::len)==Some(1),"Una entrada por diagnóstico")?;
 for k in ["id","documento","seccion","ronda","mensajes","funciones","paginas"]{ck(p[0][k]==n[0][k],&format!("Campo alterado: {k}"))?;}
 let before=p[0]["plantilla_efectiva"].as_str().ok_or("Plantilla anterior")?;let after=n[0]["plantilla_efectiva"].as_str().ok_or("Plantilla nueva")?;
 ck(before.matches("Reasoning: medium").count()==1&&after==before.replacen("Reasoning: medium","Reasoning: high",1),"Cambio de contexto mayor que la cabecera de esfuerzo")?;
 ck(p[0]["max_salida"]==2048&&n[0]["max_salida"]==4096,"Margen distinto del declarado")?;
 for name in ["cache/catalogo.json","config/politica.txt","config/plan.json","config/MODELO-REFERENCIA.json","config/NUCLEO-REFERENCIA.json"]{ck(fs::read(prev.join(name))?==fs::read(new.join(name))?,&format!("Referencia alterada: {name}"))?;}
 let tokens=n[0]["tokens"].as_array().ok_or("Tokens")?;ck(n[0]["tokens_sha256"]==h(&serde_json::to_vec(tokens)?),"Huella de tokens")?;
 let report=json!({"conforme":true,"intento":2,"referencia":"D01","caso":"A08","pregunta_politica_fuentes_antecedentes_identicos":true,"unico_cambio_textual":"Reasoning: medium -> Reasoning: high","cota_salida_anterior":2048,"cota_salida_actual":4096,"entrada_tokens":tokens.len(),"tokens_sha256":n[0]["tokens_sha256"],"admision_sha256":h(&fs::read(new.join("config/ADMISION.json"))?),"clave_entregada":false,"inferencia_iniciada":false});
 let mut f=fs::OpenOptions::new().create_new(true).write(true).open(new.join("cotejos/VINCULO-D01.json"))?;f.write_all(&serde_json::to_vec_pretty(&report)?)?;f.sync_all()?;println!("{report}");Ok(())}
