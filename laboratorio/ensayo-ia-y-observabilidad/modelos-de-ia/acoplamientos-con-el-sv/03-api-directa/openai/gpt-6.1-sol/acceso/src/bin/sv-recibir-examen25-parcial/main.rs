//! Cotejo local de recepción parcial; sin autenticación ni envío de inferencia.
#![forbid(unsafe_code)]
type E=Box<dyn std::error::Error+Send+Sync>;
#[path="../../catalogo/mod.rs"]mod catalogo;
#[path="../../../../../gpt-6-astra/anexo-pdf-20261007/instrumento/src/lib.rs"]mod suministro_pdf;
#[path="../astra-examen25-r2/suministro.rs"]mod suministro;
#[path="../astra-examen25-r2/contrato.rs"]mod contrato;
#[path="../astra-examen25-r2/localizadores.rs"]mod localizadores;
use serde_json::{json,Value};use std::{fs,path::{Path,PathBuf}};
use suministro_pdf::{need,parse,sha,R};
const ROOT:&str=".";
fn read(p:&Path)->R<Vec<u8>>{fs::read(p).map_err(|e|e.to_string())}
fn load(p:&Path)->R<Value>{parse(&read(p)?)}
fn events(p:&Path)->R<Vec<Value>>{read(p)?.split(|b|*b==b'\n').filter_map(|l|l.strip_prefix(b"data:")).filter(|l|!String::from_utf8_lossy(l).trim().eq("[DONE]")).map(parse).collect()}
fn identity(p:&Path)->R<Value>{let b=read(p)?;Ok(json!({"ruta":p.strip_prefix(ROOT).map_err(|e|e.to_string())?.to_string_lossy(),"bytes":b.len(),"sha256":sha(&b)}))}
fn case(base:&Path,n:usize)->PathBuf{base.join(format!("originales/P{:02}/R{}",(n-1)/3+1,(n-1)%3))}
fn check_failed(ev:&[Value])->R<()> {
 need(ev.len()==4,"Secuencia de incidencia distinta")?;
 for(i,k)in["response.created","response.in_progress","error","response.failed"].iter().enumerate(){need(ev[i]["type"]==*k&&ev[i]["sequence_number"]==i,"Secuencia de fallo discordante")?;}
 need(ev[0]["response"]["id"]==ev[3]["response"]["id"]&&ev[3]["response"]["status"]=="failed","Identidad o fallo no acreditados")?;
 need(ev[3]["response"]["error"]["code"]=="server_is_overloaded"&&ev[3]["response"]["usage"].is_null(),"Incidencia distinta o uso presente")
}
fn run()->R<()> {
 let root=Path::new(ROOT);let base=root.join("ejecucion/astra-examen25-r2-20261007");let prev=load(&base.join("PREVIA.json"))?;
 for v in prev["archivos_fijados"].as_array().ok_or("Fijación")?{need(identity(&root.join(v["ruta"].as_str().ok_or("Ruta")?))?==*v,"Fuente fijada alterada")?;}
 suministro::verificar(&base)?;
 let bank=load(&base.join("RESULTADO-BANCO.json"))?;need(bank["inferencias_iniciadas"]==43&&bank["estado"]=="detenido_por_incidencia","Banco distinto")?;
 let mut proofs=vec![];
 for n in 1..=42 {
  let p=case(&base,n);let r=load(&p.join("RESULTADO.json"))?;need(bank["casos"][n-1]==r&&r["completa"]==true&&r["telemetria_conforme"]==true,"Recepción incompleta")?;
  let h=load(&base.join(format!("hitos/ENTREGA-{n:02}/HITO-INSTRUMENTAL.json")))?;
  for v in h["originales"].as_array().ok_or("Hito")?{need(identity(&root.join(v["ruta"].as_str().ok_or("Ruta")?))?==*v,"Original alterado")?;}
  let t=sv_instrumentacion::verify(&p.join("instrumentacion/telemetria.jsonl"))?;need(t==r["telemetria"],"Medición alterada")?;
  let first=(n-1)/3*3+1;let stage=(n-1)%3;let mut history=vec![];
  for prior in first..n {history.push(String::from_utf8(read(&case(&base,prior).join("FINAL.txt"))?).map_err(|e|e.to_string())?);}
  let baseline=load(&base.join(format!("fuentes-admitidas/P{:02}.json",(n-1)/3+1)))?;let req=load(&p.join("SOLICITUD.json"))?;need(req==contrato::compose(&baseline,stage,&history)?,"Historia o fuente distintas")?;
  let raw=read(&p.join("SALIDA-SSE.txt"))?;let ev=events(&p.join("SALIDA-SSE.txt"))?;let got=catalogo::recepcion::extract(&ev)?;
  need(got==load(&p.join("ENTREGA-PROVEEDOR.json"))?&&got["uso_proveedor"]==r["uso_proveedor"],"Recepción o uso alterados")?;
  let text=read(&p.join("FINAL.txt"))?;need(got["texto_original"].as_str().ok_or("Texto")?.as_bytes()==text,"Texto distinto")?;
  let formal=contrato::formal(&parse(&text)?,&req)?;need(formal==load(&p.join("AUDITORIA-FORMAL.json"))?,"Cotejo formal distinto")?;
  let u=&got["uso_proveedor"];let(mut ai,mut ao)=(0u64,0u64);for g in ["items","request_fields"] {for v in u["attribution"][g].as_object().ok_or("Atribución")?.values(){ai+=v["input_tokens"].as_u64().ok_or("Entrada")?;ao+=v["output_tokens"].as_u64().ok_or("Salida")?;}}
  need(u["input_tokens"]==ai&&u["output_tokens"]==ao&&u["total_tokens"]==ai+ao,"Atribución discordante")?;
  proofs.push(json!({"caso":format!("P{:02}",(n-1)/3+1),"etapa":stage,"conforme":true,"contrato_formal_conforme":formal["conforme"],"original_resultado_sha256":sha(&read(&p.join("RESULTADO.json"))?),"sse_sha256":sha(&raw),"telemetria":t,"uso_proveedor":u,"atribucion_cotejada":true}));
 }
 let p=case(&base,43);let ev=events(&p.join("SALIDA-SSE.txt"))?;check_failed(&ev)?;let r=load(&p.join("RESULTADO.json"))?;need(r==bank["casos"][42],"Intento alterado")?;
 let t=sv_instrumentacion::verify(&p.join("instrumentacion/telemetria.jsonl"))?;need(t==r["telemetria"]&&!p.join("FINAL.txt").exists(),"Incidencia no concordante")?;
 let issue=json!({"conforme":true,"caso":"P15-R0","tipo":"interrupcion_proveedor","codigo_proveedor":"server_is_overloaded","causa_local_secundaria":"El receptor marcó error como cierre antes del evento response.failed; el flujo original conserva ambos","http":load(&p.join("HTTP.json"))?["status"],"respuesta_completa":false,"calificacion":null,"uso":{"entrada_tokens":null,"salida_tokens":null,"total_tokens":null},"creditos":null,"importe":null,"telemetria":t,"sse_sha256":sha(&read(&p.join("SALIDA-SSE.txt"))?),"reintentos":0,"necesidad":"Retomar P15 sólo cuando se determine la continuación; preservar P01–P14"});
 catalogo::save(&base.join("INCIDENCIA-P15-RUST.json"),&issue)?;
 catalogo::save(&base.join("COTEJO-PARCIAL-RUST.json"),&json!({"conforme":true,"examen_completo":false,"preguntas_completas":14,"preguntas_previstas":25,"entregas_completas":42,"inferencias_iniciadas_revision":43,"intentos_anteriores_interrumpidos":1,"casos":proofs,"incidencia":issue,"inferencias_nuevas":0,"adjudicacion_cientifica":false}))?;println!("Conforme: 42 entregas; incidencia P15 separada; examen incompleto.");Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1)}}
