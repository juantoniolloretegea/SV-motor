use std::{fs,path::Path,io::Write};
use serde_json::{Value,json};use sha2::{Sha256,Digest};
type R<T> = Result<T,Box<dyn std::error::Error>>;
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn ck(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn read(p:&Path)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn save(p:&Path,b:&[u8])->R<()>{let mut f=fs::OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(b)?;f.sync_all()?;Ok(())}
fn main()->R<()>{
 let args=std::env::args().collect::<Vec<_>>();let old=Path::new(&args[1]);let new=Path::new(&args[2]);let recovered=Path::new(&args[3]);
 let mut hashes=Vec::new();
 for(n,want)in [("arbitro-nuevos-01.tar.gz","361b6730918d43873be6f8fff4bf981647e88a33fb4f738928e7408e7c5e2768"),("punto-N01.tar.gz","5ddea1d1c5706a887776aa2031290b6bcc97a04d6ba61bdb9eb649cdb49a2d28"),("realizacion-y-rectificacion.tar.gz","7424bc759b112ea710f1856e7b55d8e1cae111d43b56609d19f4a0de5b78141a"),("preparacion-y-cierre.tar.gz","35300b501d584dc753d53355b8266eb44e2c4ed64c844b7049183948454e211c")]{
  let a=fs::read(old.join("edicion").join(n))?;let b=fs::read(recovered.join(n))?;ck(a==b&&h(&a)==want,"Custodia original y recuperada discordantes")?;hashes.push(json!({"archivo":n,"sha256":want,"bytes":a.len(),"igualdad_exacta":true}));
 }
 let auditor=fs::read(old.join("instrumentacion/src/auditar_rectificado.rs"))?;ck(h(&auditor)=="90d80c6c90b7c21b230c54fd49c25b3ca941bcd69cd3acbfd2059bfb4d445852","Rectificación distinta")?;
 for(n,want)in [("BANCO.json","66dcde13277d583624be5568cbd3a86d703e668b2fe12ac834779c03b340dbba"),("REFERENCIA-EXTERNA.json","c6c0a1ac681ed723ae1d2d0fe2cd6e633d971aeb38f980c22bddb0ffcb9f7e48"),("politica.txt","1adcc501f1b94aed756df0520ce551029e9cc742ed8db7a9f17fbd867b584bf3"),("corpus/N-A.txt","e6fd4fae101e686a63dad381a8b7d28bebc5a37c0f277e4c7f78826e44e04ab5"),("corpus/N-B.txt","6b54e1b0b2c5f3c8166e5fe48559896acee072ff4c91aba182f988d475e92aea"),("corpus/N-C.txt","58de4de3939b7ba1b6e4d13bdaa41c23de310d7657ace72486e3ffa8e51d77f5")]{ck(h(&fs::read(old.join("fuentes").join(n))?)==want,"Fuente fijada discordante")?;}
 let p=old.join("recuperado/puntos/N01");let seal=read(&p.join("PUNTO.json"))?;
 let text=fs::read_to_string(p.join("modelo.stdout"))?;let ev=text.lines().filter_map(|l|l.strip_prefix("SV_EVENT ")).map(serde_json::from_str::<Value>).collect::<Result<Vec<_>,_>>()?;
 let events=|name:&str|ev.iter().filter(|v|v["datos"]["evento"]==name).map(|v|&v["datos"]).collect::<Vec<_>>();
 let emissions=events("emision_integra");let contexts=events("contexto_conductor");let finals=events("respuesta_final");let prev=events("contexto_previsto");
 ck(emissions.len()==1&&contexts.len()==1&&finals.len()==1&&prev.len()==6,"N01 original no único")?;
 let emitted=emissions[0]["texto"].as_str().ok_or("Sin emisión")?;
 let output_hash=h(emitted.as_bytes());ck(output_hash=="7bdd77d47d383f6a128e5a33540bed63db81dbd18f046e44186020528062c0fe"&&seal["salida_sha256"]==output_hash,"Salida N01 discordante")?;
 ck(contexts[0]["id"]=="N01"&&emissions[0]["id"]=="N01"&&finals[0]["id"]=="N01","Identidad N01 discordante")?;
 for k in ["id","mensajes","tokens","tokens_sha256","plantilla_efectiva","funciones"]{ck(contexts[0][k]==prev[0][k],"Entrada no prefijada")?;}
 let template=contexts[0]["plantilla_efectiva"].as_str().ok_or("Plantilla ausente")?;
 let user=contexts[0]["mensajes"][1]["content"].as_str().ok_or("Mensaje ausente")?;
 let pages:Value=serde_json::from_str(user.split_once("Documentación proporcionada:\n").ok_or("Páginas ausentes")?.1)?;
 ck(pages.as_array().map(Vec::len)==Some(2)&&pages[0]["pagina"]==0&&pages[1]["pagina"]==1,"Dos páginas no identificadas")?;
 let all=format!("{}{}",pages[0]["texto"].as_str().ok_or("Texto")?,pages[1]["texto"].as_str().ok_or("Texto")?);
 ck(all==fs::read_to_string(old.join("fuentes/corpus/N-A.txt"))?,"Documento N01 incompleto")?;
 let audit=read(&new.join("comprobaciones/COTEJO-N01-PUNTO.json"))?;let close=read(&new.join("comprobaciones/COTEJO-N01-CIERRE.json"))?;let tokens=read(&p.join("COTEJO-ENTRADAS-PUNTO.json"))?;
 ck(audit["conforme"]==true&&close["conforme"]==true&&tokens["conforme"]==true&&tokens["modelo_stdout_sha256"]==h(text.as_bytes()),"Cotejos no ligados al original")?;
 let finaltext=finals[0]["contenido"].as_str().ok_or("Final ausente")?;ck(emitted.contains(finaltext),"Final ajeno a la emisión")?;let finalv:Value=serde_json::from_str(finaltext)?;
 let citations=finalv["evidencias"].as_array().ok_or("Evidencias ausentes")?.iter().map(|v|json!({"original":v,"literal_en_documento":all.contains(v["fragmento"].as_str().unwrap_or("")),"literal_en_pagina_declarada":v["pagina"].as_u64().and_then(|i|pages.get(i as usize)).and_then(|p|p["texto"].as_str()).is_some_and(|s|s.contains(v["fragmento"].as_str().unwrap_or("")))})).collect::<Vec<_>>();
 let dst=new.join("entrega/casos/N01");save(&dst.join("EMISION-ORIGINAL.txt"),emitted.as_bytes())?;save(&dst.join("RESPUESTA-FINAL-ORIGINAL.txt"),finaltext.as_bytes())?;save(&dst.join("ENTRADA-EFECTIVA.txt"),template.as_bytes())?;
 let report=json!({"conforme_integridad":true,"id":"N01","salida_sha256":output_hash,"final_sha256":h(finaltext.as_bytes()),"entrada_tokens":contexts[0]["tokens"].as_array().unwrap().len(),"salida_tokens":emissions[0]["tokens"].as_array().unwrap().len(),"paginas":[0,1],"fuentes_prefijadas_conformes":true,"archivos_custodiados":hashes,"auditor_sha256":h(&auditor),"punto_sha256":h(&fs::read(p.join("PUNTO.json"))?),"citas":citations,"adjudicacion":"Externa y separada; este cotejo no decide semántica.","cotejo_punto":audit,"cotejo_cierre":close,"cotejo_plantilla_tokens_conservado":tokens});
 save(&dst.join("COTEJO-INTEGRIDAD.json"),&serde_json::to_vec_pretty(&report)?)?;println!("{}",serde_json::to_string_pretty(&report)?);Ok(())
}
