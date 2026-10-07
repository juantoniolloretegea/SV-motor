use std::{fs,io::Write,path::Path};use serde_json::{Value,json};use sha2::{Sha256,Digest};
type R<T>=Result<T,Box<dyn std::error::Error>>;fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn ck(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}fn read(p:&Path)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}fn save(p:&Path,b:&[u8])->R<()>{let mut f=fs::OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(b)?;f.sync_all()?;Ok(())}
fn main()->R<()>{
 let a=std::env::args().collect::<Vec<_>>();let root=Path::new(a.get(1).ok_or("Raíz")?);let id=a.get(2).ok_or("Caso")?;ck(["N01","N02","N03","N04","N05","N06"].contains(&id.as_str()),"Caso fuera del banco")?;
 let archivo_hash=h(&fs::read(root.join(format!("punto-{id}.tar.gz")))?);let hash_remoto=fs::read_to_string(root.join(format!("cotejo-{id}/huella.stdout")))?;ck(hash_remoto.split_whitespace().next()==Some(archivo_hash.as_str()),"Archivo recuperado distinto")?;
 let p=root.join("recuperado/puntos").join(id);let seal=read(&p.join("PUNTO.json"))?;let audit=read(&root.join(format!("comprobaciones/COTEJO-{id}-PUNTO.json")))?;let tokens=read(&root.join(format!("comprobaciones/COTEJO-{id}-ENTRADAS.json")))?;ck(sv_arbitro_comprobaciones::auditoria::auditar(&p,false,true)?==audit,"Informe no reconstruible desde el original")?;
 ck(seal["id"]==*id&&seal["adjudicacion_pendiente"]==true&&audit["conforme"]==true&&tokens["conforme"]==true&&tokens["punto_previo_adjudicacion"]==true,"Punto sin cotejo previo")?;
 let stdout=fs::read_to_string(p.join("modelo.stdout"))?;let events=stdout.lines().filter_map(|l|l.strip_prefix("SV_EVENT ")).map(serde_json::from_str::<Value>).collect::<Result<Vec<_>,_>>()?;
 let emitidos=events.iter().filter(|v|v["datos"]["evento"]=="emision_integra"&&v["datos"]["id"]==*id).collect::<Vec<_>>();
 let finales=events.iter().filter(|v|v["datos"]["evento"]=="respuesta_final"&&v["datos"]["id"]==*id).collect::<Vec<_>>();
 ck(emitidos.len()==1&&finales.len()==1,"Original no único")?;
 let raw=emitidos[0]["datos"]["texto"].as_str().ok_or("Sin original")?;let finaltxt=finales[0]["datos"]["contenido"].as_str().ok_or("Sin final")?;
 ck(h(raw.as_bytes())==seal["salida_sha256"],"Original sin sello concordante")?;
 let plan=read(&root.join("realizacion/contraste/config/plan.json"))?;let caso=plan["casos"].as_array().unwrap().iter().find(|v|v["id"]==*id).ok_or("Sin caso")?;
 let corpus=fs::read_to_string(root.join("fuentes/corpus").join(format!("{}.txt",caso["documento"].as_str().unwrap())))?;
 let clave=read(&root.join("fuentes/REFERENCIA-EXTERNA.json"))?;let referencia=clave["casos"].as_array().unwrap().iter().find(|v|v["id"]==*id).ok_or("Referencia ausente")?;
 let decoded=serde_json::from_str::<Value>(finaltxt);let mut comprobaciones=Vec::new();let mut formato=false;let mut etiqueta=false;
 if let Ok(v)=&decoded{
  formato=v.as_object().is_some_and(|o|o.len()==4&&["decision","reglas","evidencias","justificacion_breve"].iter().all(|k|o.contains_key(*k)))&&v["reglas"].is_array()&&v["evidencias"].is_array()&&v["justificacion_breve"].is_string();
  etiqueta=v["decision"]==referencia["decision"];
  if let Some(e)=v["evidencias"].as_array(){for x in e{comprobaciones.push(json!({"evidencia":x,"documento_y_seccion":x["documento"]==caso["documento"]&&x["seccion"]=="S1","fragmento_literal":x["fragmento"].as_str().is_some_and(|s|!s.is_empty()&&corpus.contains(s))}));}}
 }
 let dst=root.join("entrega/casos").join(id);fs::create_dir_all(&dst)?;
 save(&dst.join("EMISION-ORIGINAL.txt"),raw.as_bytes())?;save(&dst.join("RESPUESTA-FINAL-ORIGINAL.txt"),finaltxt.as_bytes())?;
 let reporte=json!({"id":id,"afirmacion":caso["afirmacion"],"salida_sha256":h(raw.as_bytes()),"final_sha256":h(finaltxt.as_bytes()),"salida_tokens":emitidos[0]["datos"]["tokens"].as_array().unwrap().len(),"json_interpretable":decoded.is_ok(),"error_json":decoded.as_ref().err().map(|e|e.to_string()),"formato_identificable":formato,"etiqueta_coincidente":etiqueta,"citas":comprobaciones,"referencia_externa":referencia,"cotejo_custodia":audit,"cotejo_entrada":tokens,"adjudicacion_semantica":"PENDIENTE; la coincidencia de etiqueta y citas no prueba suficiencia ni fidelidad del fundamento."});
 save(&dst.join("COTEJO-PREVIO.json"),&serde_json::to_vec_pretty(&reporte)?)?;println!("{}",serde_json::to_string_pretty(&json!({"id":id,"original":raw,"respuesta_final":decoded.as_ref().ok(),"formato":formato,"etiqueta":etiqueta,"citas":comprobaciones,"referencia":referencia}))?);Ok(())
}
