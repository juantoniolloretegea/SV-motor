#![forbid(unsafe_code)]
//! Recepción acotada; reutiliza el cotejo SSE y la instrumentación del receptor común.
#[allow(dead_code)] #[path="../main.rs"] mod receptor;
use sv_cliente_api::{guard,need,parse,save,sha,R,LICENCIA};
use serde_json::{json,Value};
use std::{fs,path::{Path,PathBuf}};
fn read(p:&Path)->R<Value>{guard(p)?;parse(&fs::read(p).map_err(|e|e.to_string())?)}
fn check(root:&Path)->R<Value>{
 let prev=read(&root.join("PREVIA.json"))?;let mut files=vec![];
 for row in prev["archivos"].as_array().ok_or("Fuentes")?{let p=PathBuf::from(row["ruta"].as_str().ok_or("Ruta")?);guard(&p)?;let b=fs::read(&p).map_err(|e|e.to_string())?;need(row["bytes"]==b.len()&&row["sha256"]==sha(&b),"Fuente congelada alterada")?;files.push(json!({"bytes":b.len(),"sha256":sha(&b),"nombre":p.file_name().unwrap().to_string_lossy()}));}Ok(json!(files))
}
fn measure(root:&Path)->R<Value>{
 let frozen=check(root)?;let b=read(&root.join("RESULTADO-BANCO.json"))?;need(b["caso"]=="P13","Caso ajeno")?;let model=b["modelo"].as_str().ok_or("Modelo")?;let mut cases=vec![];
 for (i,h) in b["intentos"].as_array().ok_or("Intentos")?.iter().enumerate(){need(h["caso"]=="P13"&&h["etapa"]==i&&h["intento"]==i+1&&i<3,"Orden o número de intentos inválido")?;let d=PathBuf::from(h["directorio"].as_str().ok_or("Directorio")?);guard(&d)?;need(read(&d.join("RESULTADO.json"))?==h["resultado"],"Hito y resultado discordantes")?;let mut m=receptor::metrics(&d,model)?;m["caso"]=json!("P13");m["etapa"]=json!(i);m["intento"]=json!(i+1);m["directorio"]=json!(d.to_string_lossy());m["formal_conforme"]=read(&d.join("AUDITORIA-FORMAL.json"))?["conforme"].clone();save(&root.join(format!("mediciones/I{:03}.json",i+1)),&m)?;cases.push(m);}
 let mut sums=json!({});for k in ["tokens_entrada","tokens_salida","tokens_totales","tokens_razonamiento","tokens_cache","muestras","eventos_sse","bytes_sse","cpu_delta_ms","io_lectura_delta_bytes","io_escritura_delta_bytes","fallos_medicion"]{sums[k]=if cases.iter().any(|m|!m[k].is_u64()){Value::Null}else{json!(cases.iter().map(|m|m[k].as_u64().unwrap()).sum::<u64>())};}
 for k in ["intervalo_maximo_ms","rss_maximo_bytes","memoria_virtual_maxima_bytes","tcp_maximo_simultaneo","duracion_operacion_ms","primer_texto_ms"]{sums[format!("max_{k}")]=json!(cases.iter().filter_map(|m|m[k].as_u64()).max());}
 let complete=b["estado"]=="completo"&&cases.len()==3&&cases.iter().all(|m|m["completa"]==true&&m["telemetria_conforme"]==true);
 let out=json!({"conforme":complete,"estado":b["estado"],"modelo":model,"casos":cases,"totales":sums,"duracion_banco_ms":b["duracion_ms"],"fuentes_congeladas":frozen,"fuentes_integras":true,"importe_liquidado":null,"consulta_tecnica_nueva":false,"inferencias_nuevas_del_receptor":0,"autoria_licencia":LICENCIA});save(&root.join("METRICAS-RUST.json"),&out)?;Ok(out)
}
fn judge(root:&Path)->R<Value>{
 check(root)?;let m=read(&root.join("METRICAS-RUST.json"))?;need(m["conforme"]==true,"Diagnóstico instrumental incompleto")?;
 let ext=read(&root.join("REVISION-EXTERIOR.json"))?;let rows=ext["etapas"].as_array().ok_or("Revisión ausente")?;need(rows.len()==3,"Revisión incompleta")?;let mut results=vec![];
 for (i,r) in rows.iter().enumerate(){need(r["caso"]=="P13"&&r["etapa"]==i,"Orden de revisión")?;let measured=&m["casos"][i];need(r["respuesta_sha256"]==measured["final_sha256"],"Revisión no corresponde al original")?;
 let value=r["contenido"].as_str().ok_or("Valor")?;need(matches!(value,"0"|"1"|"U"),"Valor fuera de Σ")?;
 for k in ["restriccion_cumplida","evidencia_suficiente"]{need(r[k].is_boolean(),"Criterio sin revisión")?;}
 need(r["fundamento"].as_str().is_some_and(|s|!s.trim().is_empty()),"Fundamento ausente")?;
 let ok=value=="0"&&r["restriccion_cumplida"]==true&&r["evidencia_suficiente"]==true&&measured["formal_conforme"]==true;
 results.push(json!({"caso":"P13","etapa":i,"contenido":value,"restriccion_cumplida":r["restriccion_cumplida"],"evidencia_suficiente":r["evidencia_suficiente"],"formal_conforme":measured["formal_conforme"],"diagnostico_conforme":ok,"fundamento":r["fundamento"],"respuesta_sha256":r["respuesta_sha256"]}));
 }
 let out=json!({"caso":"P13","modelo":m["modelo"],"etapas":results,"resultado_final":if results[2]["diagnostico_conforme"]==true{"P13 conforme en R2"}else{"P13 no conforme en R2"},"alcance":"Réplica diagnóstica separada; no recalifica el examen histórico ni acredita aptitud global o clínica","revision":"Exterior asistida por IA; recepción científica independiente pendiente","inferencias_nuevas":0,"autoria_licencia":LICENCIA});save(&root.join("DICTAMEN-RUST.json"),&out)?;Ok(out)
}
fn main(){let a=std::env::args().collect::<Vec<_>>();let result=(||->R<Value>{need(a.len()==3,"Uso: sv-recibir-diagnostico medir|adjudicar DIRECTORIO")?;let root=PathBuf::from(&a[2]);guard(&root)?;match a[1].as_str(){"medir"=>measure(&root),"adjudicar"=>judge(&root),_=>Err("Operación desconocida".into())}})();match result{Ok(v)=>println!("{}",json!({"conforme":v["conforme"],"resultado_final":v["resultado_final"],"totales":v["totales"]})),Err(e)=>{eprintln!("Recepción impedida: {e}");std::process::exit(1);}}}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
