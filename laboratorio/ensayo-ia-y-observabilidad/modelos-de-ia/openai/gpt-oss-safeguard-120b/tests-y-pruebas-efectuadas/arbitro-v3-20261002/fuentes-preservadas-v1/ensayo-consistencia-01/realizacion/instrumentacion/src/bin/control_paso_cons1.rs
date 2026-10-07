use std::{fs,io::Write,path::Path,time::{SystemTime,UNIX_EPOCH}};use serde_json::{Value,json};use sha2::{Sha256,Digest};type R<T>=Result<T,Box<dyn std::error::Error>>;
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}fn ck(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}fn read(p:&Path)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}fn save(p:&Path,b:&[u8])->R<()>{let mut f=fs::OpenOptions::new().write(true).create_new(true).open(p)?;f.write_all(b)?;f.sync_all()?;Ok(())}
fn main()->R<()>{
 let a=std::env::args().collect::<Vec<_>>();let root=Path::new(a.get(1).ok_or("Raíz")?);let id=a.get(2).ok_or("Caso")?;let ids=["D01","F01","F02","F03","F04","F05","F06"];let i=ids.iter().position(|s|*s==id).ok_or("Caso fuera del banco")?;
 for anterior in &ids[..i]{let v=read(&root.join("entrega/casos").join(anterior).join("ADJUDICACION.json"))?;ck(v["critico"]==false&&v["categoria"]=="acierto"&&v["valor_sv"]=="0","Esta condición no avanza tras ningún incumplimiento")?;}
 let dir=root.join("entrega/casos").join(id);let adjudicacion=read(&dir.join("ADJUDICACION.json"))?;let cotejo=read(&dir.join("COTEJO-PREVIO.json"))?;let raw=fs::read(dir.join("EMISION-ORIGINAL.txt"))?;
 ck(adjudicacion["id"]==*id&&adjudicacion["salida_sha256"]==h(&raw)&&cotejo["salida_sha256"]==h(&raw),"Original y adjudicación discordantes")?;
 ck(adjudicacion["motivacion"].as_str().is_some_and(|s|s.chars().count()>60)&&adjudicacion["recepcion_independiente"]=="pendiente","Adjudicación no motivada o recepción confundida")?;
 let cat=adjudicacion["categoria"].as_str().ok_or("Categoría ausente")?;let critico=adjudicacion["critico"].as_bool().ok_or("Criticidad ausente")?;
 match cat{
 "acierto"=>ck(!critico&&adjudicacion["valor_sv"]=="0"&&cotejo["etiqueta_coincidente"]==true&&cotejo["formato_identificable"]==true,"Acierto no concordante")?,
 "error_critico"=>ck(critico&&adjudicacion["valor_sv"]=="1","Criticidad no concordante")?,
 "error_no_critico"=>ck(!critico&&adjudicacion["valor_sv"]=="1","Defecto formal no concordante")?,
 "U"=>ck(!critico&&adjudicacion["valor_sv"]=="U","Abstención no concordante")?,
 "blanco"=>ck(!critico&&adjudicacion["valor_sv"].is_null(),"Blanco no concordante")?,
 _=>return Err("Categoría no permite transición".into())
 }
 let punto=root.join("recuperado/puntos").join(id);let nuevo=sv_arbitro_comprobaciones::auditoria::auditar(&punto,false,true)?;ck(nuevo==cotejo["cotejo_custodia"],"Informe de custodia no auténtico")?;
 let control=json!({"id":id,"salida_sha256":h(&raw),"accion":if cat!="acierto"||i==ids.len()-1{"cerrar"}else{"continuar"}});
 let dst=root.join("controles");fs::create_dir_all(&dst)?;let b=serde_json::to_vec(&control)?;save(&dst.join(format!("{id}.json")),&b)?;
 let constancia=json!({"id":id,"fecha_unix_ms":SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),"adjudicacion_sha256":h(&fs::read(dir.join("ADJUDICACION.json"))?),"control_sha256":h(&b),"control":control,"fundamento_y_puntuacion_fuera_del_servidor":true});save(&dir.join("CONTROL-EXTERNO.json"),&serde_json::to_vec_pretty(&constancia)?)?;println!("{constancia}");Ok(())
}

