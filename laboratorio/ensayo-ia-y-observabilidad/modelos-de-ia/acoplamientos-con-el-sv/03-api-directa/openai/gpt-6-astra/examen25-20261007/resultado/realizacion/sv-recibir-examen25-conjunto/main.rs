//! Autenticación de entregas distribuidas entre revisiones; sin transporte ni inferencia.
#![forbid(unsafe_code)]
type E=Box<dyn std::error::Error+Send+Sync>;
#[path="../../catalogo/mod.rs"]mod catalogo;
#[path="../../../../../gpt-6-astra/anexo-pdf-20261007/instrumento/src/lib.rs"]mod suministro_pdf;
#[path="../astra-examen25-r3/suministro.rs"]mod suministro;
#[path="../astra-examen25-r3/contrato.rs"]mod contrato;
#[path="../astra-examen25-r3/localizadores.rs"]mod localizadores;
use serde_json::{json,Value};use std::{fs,path::{Path,PathBuf},collections::BTreeMap};
use suministro_pdf::{need,parse,sha,R};
const ROOT:&str=".";
fn read(p:&Path)->R<Vec<u8>>{fs::read(p).map_err(|e|e.to_string())}
fn load(p:&Path)->R<Value>{parse(&read(p)?)}
fn events(p:&Path)->R<Vec<Value>>{read(p)?.split(|b|*b==b'\n').filter_map(|l|l.strip_prefix(b"data:")).filter(|l|!String::from_utf8_lossy(l).trim().eq("[DONE]")).map(parse).collect()}
fn identity(p:&Path)->R<Value>{let b=read(p)?;Ok(json!({"ruta":p.strip_prefix(ROOT).map_err(|e|e.to_string())?.to_string_lossy(),"bytes":b.len(),"sha256":sha(&b)}))}
fn check_hito(root:&Path,h:&Value)->R<()>{for v in h["originales"].as_array().ok_or("Originales")?{need(identity(&root.join(v["ruta"].as_str().ok_or("Ruta")?))?==*v,"Original alterado")?;}Ok(())}
fn run()->R<()>{
 let root=Path::new(ROOT);let old=root.join("ejecucion/astra-examen25-r2-20261007");let new=root.join("ejecucion/astra-examen25-r3-20261007");let out=root.join("ejecucion/astra-examen25-conjunto-20261007");
 for b in [&old,&new]{let p=load(&b.join("PREVIA.json"))?;for v in p["archivos_fijados"].as_array().ok_or("Fijación")?{need(identity(&root.join(v["ruta"].as_str().ok_or("Ruta")?))?==*v,"Admisión alterada")?;}}
 suministro::verificar(&old)?;let bank=load(&new.join("RESULTADO-BANCO.json"))?;let mut pp=BTreeMap::<usize,PathBuf>::new();let mut issues=vec![load(&root.join("ejecucion/astra-examen25-20261007/INCIDENCIA-RUST.json"))?,load(&old.join("INCIDENCIA-P15-RUST.json"))?];
 for n in 1..=42{let h=load(&old.join(format!("hitos/ENTREGA-{n:02}/HITO-INSTRUMENTAL.json")))?;check_hito(root,&h)?;pp.insert(n,old.join(format!("originales/P{:02}/R{}",(n-1)/3+1,(n-1)%3)));}
 for h in fs::read_dir(new.join("hitos")).map_err(|e|e.to_string())?{let h=load(&h.map_err(|e|e.to_string())?.path().join("HITO-INSTRUMENTAL.json"))?;check_hito(root,&h)?;let n=h["entrega_numero"].as_u64().ok_or("Número")? as usize;need((43..=75).contains(&n),"Entrega ajena")?;
  let dir=root.join(h["directorio"].as_str().ok_or("Directorio")?);let r=load(&dir.join("RESULTADO.json"))?;
  let i=h["numero_intento"].as_u64().ok_or("Intento")? as usize;need(h["resultado"]==bank["casos"][i-1],"Hito distinto del banco")?;
  for (k,v) in r.as_object().ok_or("Resultado")?{need(h["resultado"][k]==*v,"Resultado e hito distintos")?;}
  if r["completa"]==true&&r["telemetria_conforme"]==true{need(pp.insert(n,dir).is_none(),"Entrega duplicada")?;}else{let t=sv_instrumentacion::verify(&dir.join("instrumentacion/telemetria.jsonl"))?;need(t==r["telemetria"],"Medición fallida alterada")?;issues.push(json!({"caso":r["caso"],"directorio":h["directorio"],"incidencia":r["incidencia_servicio"],"duracion_operacion_ms":r["duracion_operacion_ms"],"telemetria":t,"uso_proveedor":r["uso_proveedor"],"calificacion":null}));}
 }
 let mut proof=vec![];let mut index=vec![];
 for (n,p) in &pp{let r=load(&p.join("RESULTADO.json"))?;let t=sv_instrumentacion::verify(&p.join("instrumentacion/telemetria.jsonl"))?;need(t==r["telemetria"],"Medición alterada")?;
  let first=(n-1)/3*3+1;let stage=(n-1)%3;let mut history=vec![];for k in first..*n{history.push(String::from_utf8(read(&pp.get(&k).ok_or("Historia ausente")?.join("FINAL.txt"))?).map_err(|e|e.to_string())?);}
  let req=load(&p.join("SOLICITUD.json"))?;need(req==contrato::compose(&load(&old.join(format!("fuentes-admitidas/P{:02}.json",(n-1)/3+1)))?,stage,&history)?,"Fuente o historia distintas")?;
  let ev=events(&p.join("SALIDA-SSE.txt"))?;let got=catalogo::recepcion::extract(&ev)?;need(got==load(&p.join("ENTREGA-PROVEEDOR.json"))?&&got["uso_proveedor"]==r["uso_proveedor"],"Entrega o uso distintos")?;
  let text=read(&p.join("FINAL.txt"))?;need(got["texto_original"].as_str().ok_or("Texto")?.as_bytes()==text,"Texto distinto")?;
  let formal=match parse(&text).and_then(|v|contrato::formal(&v,&req)){Ok(v)=>v,Err(e)=>json!({"conforme":false,"defecto":e})};need(formal==load(&p.join("AUDITORIA-FORMAL.json"))?,"Forma no reproducible")?;
  let u=&got["uso_proveedor"];let(mut ai,mut ao)=(0u64,0u64);for g in ["items","request_fields"]{for v in u["attribution"][g].as_object().ok_or("Atribución")?.values(){ai+=v["input_tokens"].as_u64().ok_or("Entrada")?;ao+=v["output_tokens"].as_u64().ok_or("Salida")?;}}need(u["input_tokens"]==ai&&u["output_tokens"]==ao&&u["total_tokens"]==ai+ao,"Atribución discordante")?;
  let d=p.strip_prefix(ROOT).map_err(|e|e.to_string())?.to_string_lossy();let id=format!("P{:02}",(n-1)/3+1);
  index.push(json!({"entrega_numero":n,"caso":id,"etapa":stage,"directorio":d}));proof.push(json!({"caso":id,"etapa":stage,"directorio":d,"conforme":true,"contrato_formal_conforme":formal["conforme"],"original_resultado_sha256":sha(&read(&p.join("RESULTADO.json"))?),"sse_sha256":sha(&read(&p.join("SALIDA-SSE.txt"))?),"telemetria":t,"uso_proveedor":u,"atribucion_cotejada":true}));
 }
 let complete=pp.len()==75&&bank["estado"]=="entregas_pendientes_recibidas";
 catalogo::save(&out.join("INDICE-ENTREGAS.json"),&json!({"conforme":true,"completo":complete,"casos":index}))?;
 catalogo::save(&out.join("COTEJO-BANCO-RUST.json"),&json!({"conforme":true,"examen_completo":complete,"estado_ejecucion":bank["estado"],"casos":proof,"incidencias":issues,"inferencias_nuevas":0,"adjudicacion_cientifica":false}))?;
 println!("Cotejo conforme: {} entregas; examen completo={complete}",pp.len());Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1)}}
