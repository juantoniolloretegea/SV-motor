//! Recuperación delimitada de MD01-R0: conserva la respuesta sin repetirla.
use super::{admitir,segmentar,need,parse,put,save,sha,R};
use std::{fs,path::Path};
use serde_json::{json,Value};
use sv_claude_kaggle as sv;
fn read(d:&Path,n:&str)->R<Vec<u8>>{let p=d.join(n);let m=fs::symlink_metadata(&p).map_err(|e|e.to_string())?;need(m.is_file()&&!m.file_type().is_symlink(),"Original no es archivo regular")?;fs::read(p).map_err(|e|e.to_string())}
pub fn recuperar_original(q:&Value,w:&Value,d:&Path,root:&Path)->R<Value>{
 let source=root.join("manual/MD01-R0");
 let raw=read(&source,"RESPUESTA-HTTP.json")?;
 need(sha(&raw)=="1a7752ea04e53b7d2676478a1395505d4c895b24ddde68740f14ff11745daa75","Respuesta distinta del original recuperado")?;
 let body=parse(&raw)?;admitir(&body)?;
 need(body["choices"][0]["message"]["refusal"]=="","El defecto recuperable no es refusal vacío")?;
 need(parse(&read(&source,"SOLICITUD-DOCUMENTAL.json")?)?==*q&&parse(&read(&source,"SOLICITUD.json")?)?==*w,"Contrato o petición cambiados; no se adopta respuesta ajena")?;
 need(parse(&read(&source,"HTTP.json")?)?["status"]==200,"No hubo recepción HTTP normal")?;
 let original=body["choices"][0]["message"]["content"].as_str().ok_or("Sin texto")?;
 let (final_text,reasoning,segments)=segmentar(original)?;
 need(read(&source,"CONTENIDO-ORIGINAL.txt")?==original.as_bytes()&&read(&source,"FINAL.txt")?==final_text.as_bytes()&&read(&source,"RAZONAMIENTO-THINK.txt")?==reasoning.as_bytes(),"Contenido original discordante")?;
 need(parse(&read(&source,"SEGMENTACION.json")?)?==segments,"Segmentación discordante")?;
 let prior=parse(&read(&source,"RESULTADO.json")?)?;
 need(prior["completa"]==false&&prior["error"]=="Negativa comunicada por servicio","No es el defecto instrumental delimitado")?;
 let tel=sv::instrumentacion::verify(&source.join("instrumentacion/telemetria.jsonl"))?;
 need(tel==prior["telemetria"]&&tel["fallos_medicion"]==0&&tel["muestras"]==203&&tel["intervalo_maximo_ms"].as_u64().is_some_and(|v|v<=750),"Telemetría original no conforme")?;
 let records=sv::instrumentacion::records(&source.join("instrumentacion/telemetria.jsonl"))?;
 need(records.iter().any(|r|r["tipo"]=="recepcion"&&r["datos"]["respuesta_http_sha256"]==sha(&raw)),"Recepción no correlacionada con el diario")?;
 need(prior["uso_proveedor"]==body["usage"]&&prior["coste_nanodolares"]==255580000,"Uso original discordante")?;
 let formal=parse(final_text.as_bytes()).and_then(|a|sv::formal(&a,q)).unwrap_or_else(|e|json!({"conforme":false,"defecto":e}));
 let mut row=prior;row["completa"]=json!(true);row["error"]=Value::Null;row["auditoria_formal"]=formal;
 row["binario_solicitud_original_sha256"]=json!("0244981796fcfe352e379d342c13056a16542b4e804b0cda74811e8a21f8af18");row["recuperacion_instrumental_sin_inferencia"]=json!(true);row["original_respuesta_http_sha256"]=json!(sha(&raw));
 row["limite_razonamiento"]=json!("El proveedor comunicó tokens de razonamiento pero no emitió el texto; ninguna recuperación puede reconstruirlo");
 for name in ["SOLICITUD-DOCUMENTAL.json","SOLICITUD.json","HTTP.json","RESPUESTA-HTTP.json","CONTENIDO-ORIGINAL.txt","FINAL.txt","RAZONAMIENTO-THINK.txt","RAZONAMIENTO-CANALES.json","SEGMENTACION.json","instrumentacion/telemetria.jsonl","instrumentacion/COTEJO-TELEMETRIA.json"]{put(&d.join(name),&read(&source,name)?)?;}
 put(&d.join("RESULTADO-ANTERIOR.json"),&read(&source,"RESULTADO.json")?)?;
 put(&d.join("RESERVA-ANTERIOR.json"),&read(&source,"RESERVA.json")?)?;
 save(&d.join("RECUPERACION-RUST.json"),&json!({"conforme":true,"solicitudes_de_modelo":0,"respuesta_repetida":false,"contrato_y_peticion_identicos":true,"correccion":"refusal vacío no constituye una negativa","respuesta_http_sha256":sha(&raw),"telemetria_original":tel,"originales_preservados":true,"recepcion_cientifica_independiente":"pendiente","licencia":sv::LICENCIA}))?;
 save(&d.join("RESULTADO.json"),&row)?;Ok(row)
}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
