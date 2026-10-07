use std::{fs,io::Write,path::Path};use serde_json::{Value,json};use sha2::{Digest,Sha256};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn ck(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn main()->R<()>{let arg=std::env::args().nth(1).ok_or("Directorio")?;let base=Path::new(&arg);let prev=base.join("diagnostico-A08-02");let new=base.join("diagnostico-A08-03");
 let before=fs::read(prev.join("config/ADMISION.json"))?;let after=fs::read(new.join("config/ADMISION.json"))?;
 ck(before==after,"Admisión efectiva distinta: D03 sólo permite cambio de tiempo")?;
 let entradas:Value=serde_json::from_slice(&after)?;ck(entradas.as_array().map(Vec::len)==Some(1)&&entradas[0]["id"]=="A08"&&entradas[0]["max_salida"]==4096,"Entrada discordante")?;
 ck(entradas[0]["tokens"].as_array().map(Vec::len)==Some(2774)&&entradas[0]["tokens_sha256"]=="09cb30867e6d957640877a4aa15627fd6ca156304171c180daad8959e76d0208","Tokens diferentes")?;
 for n in ["cache/catalogo.json","config/politica.txt","config/plan.json","config/contrato.json","config/MODELO-REFERENCIA.json","config/NUCLEO-REFERENCIA.json"]{ck(fs::read(prev.join(n))?==fs::read(new.join(n))?,&format!("Referencia alterada: {n}"))?;}
 let before=fs::read_to_string(prev.join("conductor/src/main.rs"))?.replace("diagnostico-A08-02","diagnostico-A08-03").replace("diag-A08-02","diag-A08-03").replace("diag-a08-02","diag-a08-03").replace("diag02","diag03").replace("\"intento\":2,\"razonamiento\":\"high\"","\"intento\":3,\"razonamiento\":\"high\"");
 ck(before==fs::read_to_string(new.join("conductor/src/main.rs"))?,"Cambio de generación no declarado")?;
 let before=fs::read_to_string(prev.join("custodio/src/main.rs"))?.replace("diagnostico-A08-02","diagnostico-A08-03").replace("diag-A08-02","diag-A08-03").replace("diag-a08-02","diag-a08-03").replace("diag02","diag03").replace("Duration::from_secs(3600)","Duration::from_secs(9000)").replace("Cota total de una hora alcanzada; conservar sin reintento","Cota total de dos horas y media alcanzada; conservar sin cuarto intento");
 ck(before==fs::read_to_string(new.join("custodio/src/main.rs"))?,"Cambio de custodia no declarado")?;
 let report=json!({"conforme":true,"intento":3,"referencia":"D02","caso":"A08","admision_bytes_identicos":true,"admision_sha256":h(&after),"entrada_tokens":2774,"tokens_sha256":entradas[0]["tokens_sha256"],"unico_cambio_operativo":"Límite temporal total de 3600 a 9000 s; identidades nuevas para preservar originales","generacion_sin_cambios":true,"clave_entregada":false,"inferencia_iniciada":false,"maximo_total_intentos":3});
 let mut f=fs::OpenOptions::new().create_new(true).write(true).open(new.join("cotejos/VINCULO-D02.json"))?;f.write_all(&serde_json::to_vec_pretty(&report)?)?;f.sync_all()?;println!("{report}");Ok(())
}
