#![forbid(unsafe_code)]
//! Diagnóstico documental MD01: mismo suministro, contrato temporal y transporte común.
use sv_cliente_api::{self as api,need,parse,save,sha,Perfil,R};
use serde_json::{json,Value};
use std::{fs,path::{Path,PathBuf},time::Instant};
use zeroize::Zeroizing;
#[path="../manual/contrato.rs"] mod contrato;
#[path="../manual/localizadores.rs"] mod localizadores;
#[path="../manual/suministro.rs"] mod suministro;
mod suministro_pdf {
 pub use sv_cliente_api::{parse,need,sha,R};use serde_json::Value;
 pub fn num(v:&Value)->R<usize>{v.as_u64().and_then(|x|usize::try_from(x).ok()).ok_or("Entero requerido".into())}
 pub fn contenido(v:&Value,id:usize)->R<Value>{need(v["jsonrpc"]=="2.0"&&v["id"]==id&&v.get("error").is_none(),"Identidad RPC discordante")?;let r=&v["result"];need(r["isError"]==false&&r["content"].as_array().map(Vec::len)==Some(1)&&r["content"][0]["type"]=="text","Recepción MCP inválida")?;let text=parse(r["content"][0]["text"].as_str().ok_or("Texto ausente")?.as_bytes())?;need(text==r["structuredContent"],"Representaciones RPC discordantes")?;Ok(text)}
}
const ROOT:&str="C:/SV-LABORATORIO";
const LIMIT:u64=3_000_000_000;
fn path(p:&Path)->R<PathBuf>{let p=if p.is_absolute(){p.to_path_buf()}else{Path::new(ROOT).join(p)};api::guard(&p)?;need(p.exists(),"Fuente ausente")?;Ok(p)}
fn read(p:&Path)->R<Value>{parse(&fs::read(path(p)?).map_err(|e|e.to_string())?)}
fn ident(p:&Path)->R<Value>{let p=path(p)?;let b=fs::read(&p).map_err(|e|e.to_string())?;Ok(json!({"ruta":p.to_string_lossy(),"bytes":b.len(),"sha256":sha(&b)}))}
fn profile(p:&Path)->R<Perfil>{let v:Perfil=serde_json::from_value(read(p)?).map_err(|e|e.to_string())?;v.comprobar()?;need(v.proveedor=="xAI"&&v.modelo=="grok-4.7"&&v.endpoint=="https://api.x.ai/v1/responses"&&v.exigir_zdr&&v.presupuesto_ticks==LIMIT&&v.entrada_ticks_por_token==20000&&v.salida_ticks_por_token==60000,"Perfil distinto del autorizado")?;Ok(v)}
fn frozen(root:&Path,p:&Path)->R<Vec<Value>>{
 let mut out=vec![ident(p)?,ident(&std::env::current_exe().map_err(|e|e.to_string())?)?];
 let common=Path::new(ROOT).join("desarrollo/acoplamientos-con-el-sv/cliente-api-rust");
 for s in ["Cargo.toml","Cargo.lock","src/lib.rs","src/estricto.rs","src/bin/sv-manual-diagnostico.rs","src/manual/contrato.rs","src/manual/contrato-base.rs","src/manual/localizadores.rs","src/manual/suministro.rs"]{out.push(ident(&common.join(s))?);}
 for s in ["Cargo.toml","src/lib.rs"]{out.push(ident(&Path::new(ROOT).join("desarrollo/acoplamientos-con-el-sv/instrumentacion-rust").join(s))?);}
 for s in ["ADMISION.md","fuentes/preparado/CATALOGO.json","fuentes/preparado/FUENTES.json","fuentes/BANCO.json","reservado/CLAVE.json","fuentes-admitidas/MD01.json","suministro/CONTROL-ARBITRO.json","RECEPCION-TRANSPORTE-PREVIA.json"]{out.push(ident(&root.join(s))?);}
 for s in ["sv-mcp-documental","verificar-diario"]{out.push(ident(&Path::new(ROOT).join("compilacion/mcp-mdbook/debug").join(s))?);}Ok(out)
}
fn check(root:&Path,p:&Path)->R<()>{need(read(&root.join("PREVIA.json"))?["archivos"]==json!(frozen(root,p)?),"Fuentes o controlador alterados")?;suministro::verificar(root)}
fn prior(root:&Path)->R<Value>{
 let r=read(&Path::new(ROOT).join("ejecucion/grok47-examen25-20261008-r2/RECEPCION-TRANSPORTE.json"))?;
 let f=PathBuf::from(r["resultado_sha256"]["ruta"].as_str().ok_or("Procedencia ausente")?);
 need(r["conforme"]==true&&r["modelo"]=="grok-4.7"&&r["resultado_sha256"]==ident(&f)?&&r["resultado_sha256"]["sha256"]=="aeca59876ebd2a1bf84035d2a8e40808412470063caf8d37b341ed60ea413622","Transporte previo no cotejado")?;
 let result=read(&f)?;need(result["completa"]==true&&result["entrega"]["zdr_confirmado"]==true,"Transporte previo insuficiente")?;
 let v=json!({"conforme":true,"referencia":r,"inferencia_nueva":false,"coste_imputado_a_esta_prueba_ticks":0});save(&root.join("RECEPCION-TRANSPORTE-PREVIA.json"),&v)?;Ok(v)
}
fn protect_history(q:&mut Value,p:&Perfil)->R<()>{
 for input in q["input"].as_array_mut().ok_or("Entradas ausentes")?{
  if input["role"]!="user"{continue;}let Some(t)=input["content"].as_str() else{continue;};
  if let Ok(mut record)=parse(t.as_bytes()){if record["tipo"]=="registro_de_instrucciones_historicas"{
   record["instrucciones_aplicadas"]=json!(format!("{}\n{}\n{}",record["instrucciones_aplicadas"].as_str().ok_or("Instrucciones históricas ausentes")?,api::LICENCIA,api::AVISO));input["content"]=json!(record.to_string());
  }}
 }api::proteger(q,p)
}
fn compose(root:&Path,p:&Perfil,stage:usize)->R<Value>{
 let mut history=vec![];
 for s in 0..stage {let d=root.join(format!("intentos/MD01-R{s}"));let r=read(&d.join("RESULTADO.json"))?;let h=read(&root.join(format!("recibidos/MD01-R{s}.json")))?;let b=fs::read(d.join("FINAL.txt")).map_err(|e|e.to_string())?;
 need(r["completa"]==true&&r["telemetria_conforme"]==true&&h["resultado"]==ident(&d.join("RESULTADO.json"))?&&h["texto"]==ident(&d.join("FINAL.txt"))?,"Historia alterada o incompleta")?;
 let t=String::from_utf8(b).map_err(|e|e.to_string())?;need(r["entrega"]["texto_original"]==t,"Original discordante")?;history.push(t);}
 let base=read(&root.join("fuentes-admitidas/MD01.json"))?;let mut q=contrato::compose(&base,stage,&history)?;protect_history(&mut q,p)?;
 // Cotejo exacto de cada obligación histórica con la solicitud realmente enviada.
 for i in q["input"].as_array().unwrap(){if i["role"]=="user"{if let Some(t)=i["content"].as_str(){if let Ok(record)=parse(t.as_bytes()){if record["tipo"]=="registro_de_instrucciones_historicas"{let s=record["etapa_historica"].as_u64().ok_or("Etapa ausente")?;need(record["instrucciones_aplicadas"]==read(&root.join(format!("intentos/MD01-R{s}/SOLICITUD.json")))?["instructions"],"Obligación histórica no idéntica")?;}}}}}
 Ok(q)
}
fn fits(spent:u64,reserve:u64)->bool{spent.checked_add(reserve).is_some_and(|x|x<=LIMIT)}
fn execute(root:&Path,p:&Perfil,pfile:&Path)->R<Value>{
 check(root,pfile)?;need(!root.join("INICIO.json").exists(),"Prueba ya iniciada; no reanudar automáticamente")?;
 let secret_file=std::env::var("SV_API_KEY_FILE").map_err(|_|"Falta ruta de credencial")?;let secret=Zeroizing::new(fs::read_to_string(path(Path::new(&secret_file))?).map_err(|_|"Credencial no accesible")?.trim().to_owned());need(secret.len()>=24&&!secret.chars().any(char::is_whitespace),"Credencial inválida")?;
 save(&root.join("INICIO.json"),&json!({"utc_ms":sv_instrumentacion::utc_ms(),"perfil":p,"preguntas":1,"etapas":3,"reintentos":0,"licencia":api::LICENCIA}))?;
 let start=Instant::now();let mut spent=0u64;let mut rows=vec![];let mut status="completo";
 for s in 0..3 {
  check(root,pfile)?;let q=compose(root,p,s)?;let reserve=api::reserva(&q,p)?;let allowed=fits(spent,reserve);
  save(&root.join(format!("reservas/MD01-R{s}.json")),&json!({"coste_previo_ticks":spent,"reserva_ticks":reserve,"limite_ticks":LIMIT,"admision":allowed,"solicitud_sha256":sha(&serde_json::to_vec(&q).map_err(|e|e.to_string())?)}))?;
  if !allowed{status="suspendido_antes_del_envio_por_reserva_presupuestaria";break;}
  let d=root.join(format!("intentos/MD01-R{s}"));let r=api::enviar(p,&secret,&q,&d,300000)?;
  let fallback=read(&d.join("ENTREGA-PROVEEDOR.json")).unwrap_or(Value::Null);let cost=r["entrega"]["uso_proveedor"]["cost_in_usd_ticks"].as_u64().or(fallback["uso_proveedor"]["cost_in_usd_ticks"].as_u64());spent=spent.checked_add(cost.unwrap_or(reserve)).ok_or("Desbordamiento")?;
  let complete=r["completa"]==true&&r["telemetria_conforme"]==true;
  let formal=if complete{parse(r["entrega"]["texto_original"].as_str().ok_or("Texto ausente")?.as_bytes()).and_then(|v|contrato::formal(&v,&q)).unwrap_or_else(|e|json!({"conforme":false,"defecto":e}))}else{Value::Null};
  save(&d.join("AUDITORIA-FORMAL.json"),&formal)?;
  if complete{save(&root.join(format!("recibidos/MD01-R{s}.json")),&json!({"resultado":ident(&d.join("RESULTADO.json"))?,"texto":ident(&d.join("FINAL.txt"))?}))?;}
  let row=json!({"caso":"MD01","etapa":s,"completa":complete,"coste_comunicado_ticks":cost,"reserva_ticks":reserve,"resultado":ident(&d.join("RESULTADO.json"))?,"auditoria_formal":formal});save(&root.join(format!("hitos/MD01-R{s}.json")),&row)?;rows.push(row);
  println!("MD01 R{s}: completa={complete}; coste_ticks={cost:?}; acumulado={spent}");
  if !complete{status="suspendido_por_incidencia_tecnica";break;}if cost.is_none(){status="suspendido_por_coste_desconocido";break;}if cost.is_some_and(|c|c>reserve)||spent>LIMIT{status="desviacion_presupuestaria";break;}
 }
 let result=json!({"estado":status,"duracion_ms":start.elapsed().as_millis(),"coste_o_reserva_ticks":spent,"limite_ticks":LIMIT,"entregas":rows,"adjudicacion":"pendiente de revisión exterior al candidato"});save(&root.join("RESULTADO-BANCO.json"),&result)?;Ok(result)
}
fn receive(root:&Path,p:&Perfil,pfile:&Path)->R<Value>{
 check(root,pfile)?;let bank=read(&root.join("RESULTADO-BANCO.json"))?;let mut rows=vec![];let(mut input,mut output,mut total,mut cost,mut samples,mut gap)=(0u64,0u64,0u64,0u64,0u64,0u64);
 for row in bank["entregas"].as_array().ok_or("Entregas ausentes")?{let s=row["etapa"].as_u64().ok_or("Etapa")? as usize;need(s==rows.len(),"Etapas discontinuas")?;let d=root.join(format!("intentos/MD01-R{s}"));let r=read(&d.join("RESULTADO.json"))?;need(row["resultado"]==ident(&d.join("RESULTADO.json"))?,"Resultado alterado")?;let q=compose(root,p,s)?;need(read(&d.join("SOLICITUD.json"))?==q,"Solicitud no recompuesta")?;
 let measured=sv_instrumentacion::verify(&d.join("instrumentacion/telemetria.jsonl"))?;need(measured==r["telemetria"],"Telemetría discordante")?;let http=read(&d.join("HTTP.json")).unwrap_or(Value::Null);
 let mut usage=Value::Null;
 if row["completa"]==true{let mut stream=api::Flujo::default();stream.feed(&fs::read(d.join("SALIDA-SSE.txt")).map_err(|e|e.to_string())?)?;let received=stream.recibir(&p.modelo)?;need(received["texto_original"]==r["entrega"]["texto_original"]&&received["texto_original"]==fs::read_to_string(d.join("FINAL.txt")).map_err(|e|e.to_string())?&&received["uso_proveedor"]==r["entrega"]["uso_proveedor"],"Entrega no concordante")?;need(http["status"]==200&&http["cabeceras"]["x-zero-data-retention"]=="true","HTTP o ZDR no conforme")?;usage=received["uso_proveedor"].clone();input+=usage["input_tokens"].as_u64().ok_or("Entrada")?;output+=usage["output_tokens"].as_u64().ok_or("Salida")?;total+=usage["total_tokens"].as_u64().ok_or("Total")?;cost+=usage["cost_in_usd_ticks"].as_u64().ok_or("Coste")?;
 let audit=parse(received["texto_original"].as_str().unwrap().as_bytes()).and_then(|v|contrato::formal(&v,&q)).unwrap_or_else(|e|json!({"conforme":false,"defecto":e}));need(audit==row["auditoria_formal"],"Auditoría formal discordante")?;}
 samples+=measured["muestras"].as_u64().ok_or("Muestras")?;gap=gap.max(measured["intervalo_maximo_ms"].as_u64().ok_or("Intervalo")?);
 rows.push(json!({"caso":"MD01","etapa":s,"completa":row["completa"],"uso_proveedor":usage,"duracion_operacion_ms":r["duracion_operacion_ms"],"primer_texto_ms":r["entrega"]["primer_texto_ms"],"telemetria":measured,"auditoria_formal":row["auditoria_formal"],"reserva_ticks":row["reserva_ticks"],"coste_comunicado_ticks":row["coste_comunicado_ticks"],"http":http["status"],"zdr":http["cabeceras"]["x-zero-data-retention"]}));
 }
 need(input+output==total,"Suma de tokens discordante")?;
 let result=json!({"conforme":true,"estado_banco":bank["estado"],"etapas_recibidas":rows.len(),"casos":rows,"tokens_entrada":input,"tokens_salida":output,"tokens_totales":total,"coste_conocido_ticks":cost,"duracion_ms":bank["duracion_ms"],"muestras":samples,"intervalo_maximo_ms":gap,"suministro":read(&root.join("suministro/CONTROL-ARBITRO.json"))?,"limite":"Recepción técnica; no adjudica contenido científico"});save(&root.join("RECEPCION-RUST.json"),&result)?;Ok(result)
}
fn run()->R<()>{let a=std::env::args().collect::<Vec<_>>();need(a.len()==4,"Uso: preparar|examinar|recibir PERFIL DIRECTORIO")?;let pf=path(Path::new(&a[2]))?;let root=path(Path::new(&a[3]))?;let p=profile(&pf)?;let v=match a[1].as_str(){"preparar"=>{let supply=suministro::preparar(&root)?;prior(&root)?;let q=compose(&root,&p,0)?;let v=json!({"conforme":true,"suministro":supply,"archivos":frozen(&root,&pf)?,"reserva_R0_ticks":api::reserva(&q,&p)?,"inferencias":0});save(&root.join("PREVIA.json"),&v)?;v},"examinar"=>execute(&root,&p,&pf)?,"recibir"=>receive(&root,&p,&pf)?,_=>return Err("Operación no admitida".into())};println!("{}: {}",a[1],v["estado"].as_str().unwrap_or("conforme"));Ok(())}
fn main(){if let Err(e)=run(){eprintln!("IMPEDIMENTO: {e}");std::process::exit(1)}}
#[cfg(test)]mod tests{use super::*;
 #[test]fn presupuesto_no_admite_desbordamiento_ni_exceso(){assert!(fits(1,LIMIT-1));assert!(!fits(1,LIMIT));assert!(!fits(u64::MAX,1));}
 #[test]fn conserva_licencia_en_obligaciones_historicas(){let p=Perfil{proveedor:"xAI".into(),modelo:"grok-4.7".into(),endpoint:"https://api.x.ai/v1/responses".into(),presupuesto_ticks:LIMIT,exigir_zdr:true,entrada_ticks_por_token:20000,salida_ticks_por_token:60000,cuota_gratuita_tokens:None};let b=json!({"input":[{"role":"user","content":json!({"caso":"MD01","pregunta":"Q","fragmentos_documentales_completos":[]}).to_string()}],"instructions":"Fuentes exclusivas","tools":[],"tool_choice":"none"});let mut q0=contrato::compose(&b,0,&[]).unwrap();protect_history(&mut q0,&p).unwrap();let mut q1=contrato::compose(&b,1,&["original".into()]).unwrap();protect_history(&mut q1,&p).unwrap();let record=parse(q1["input"][1]["content"].as_str().unwrap().as_bytes()).unwrap();assert_eq!(record["instrucciones_aplicadas"],q0["instructions"]);assert_eq!(q1["tools"],json!([]));assert_eq!(q1["store"],false);}
}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
