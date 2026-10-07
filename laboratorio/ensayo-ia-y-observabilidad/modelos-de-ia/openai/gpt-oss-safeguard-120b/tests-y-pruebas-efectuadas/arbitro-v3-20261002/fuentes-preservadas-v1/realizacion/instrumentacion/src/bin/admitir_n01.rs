use std::{fs,path::Path,io::Write};use serde_json::{Value,json};use sv_arbitro_comprobaciones::{huella,auditoria::auditar};
fn main()->Result<(),Box<dyn std::error::Error>>{
 let a=std::env::args().collect::<Vec<_>>();let old=Path::new(&a[1]);let new=Path::new(&a[2]);
 let informe=auditar(&old.join("recuperado/puntos/N01"),false,true)?;
 let guardado:Value=serde_json::from_slice(&fs::read(new.join("comprobaciones/COTEJO-N01-PUNTO.json"))?)?;
 if informe!=guardado{return Err("Informe ajeno al original o alterado".into())}
 let b=fs::read(new.join("entrega/casos/N01/ADJUDICACION.json"))?;let v:Value=serde_json::from_slice(&b)?;
 let s="7bdd77d47d383f6a128e5a33540bed63db81dbd18f046e44186020528062c0fe";
 if v["id"]!="N01"||v["salida_sha256"]!=s||v["critico"]!=false||!["acierto","error_no_critico","U"].contains(&v["categoria"].as_str().unwrap_or(""))||v["recepcion_independiente"]!="pendiente"||v["motivacion"].as_str().map(str::len).unwrap_or(0)<100{return Err("Evaluación ausente o no permite continuar".into())}
 let control=json!({"revision":"ARBITRO-SV-SAFEGUARD-20261002/v3","id":"N01","siguiente":"N02","salida_sha256":s,"evaluacion_sha256":huella(&b),"cotejo_sha256":huella(&serde_json::to_vec(&informe)?),"accion":"continuar"});
 let bytes=serde_json::to_vec(&control)?;let p=new.join("realizacion/config/REANUDACION.json");let mut f=fs::OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(&bytes)?;f.sync_all()?;println!("{}",json!({"control":control,"sha256":huella(&bytes),"alcance":"Admisión documental N01; no autoriza carga sin las pruebas del recorrido v3"}));Ok(())
}
