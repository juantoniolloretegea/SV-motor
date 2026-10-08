#![forbid(unsafe_code)]
//! Réplica documental acotada: reutiliza suministro, contrato, transporte y medición.
use sv_cliente_api::{self as api, need, parse, save, sha, Perfil, R};
use serde_json::{json, Value};
use std::{fs, path::{Path, PathBuf}, time::Instant};
use zeroize::Zeroizing;
#[path="../documental/contrato.rs"] mod contrato;
#[path="../documental/localizadores.rs"] mod localizadores;
#[path="../documental/suministro.rs"] mod suministro;
mod suministro_pdf {
 pub use sv_cliente_api::{parse,need,sha,R}; use serde_json::Value;
 pub fn num(v:&Value)->R<usize>{v.as_u64().and_then(|x|usize::try_from(x).ok()).ok_or("Entero requerido".into())}
 pub fn contenido(v:&Value,id:usize)->R<Value>{need(v["jsonrpc"]=="2.0"&&v["id"]==id&&v.get("error").is_none(),"Identidad RPC discordante")?;let r=&v["result"];need(r["isError"]==false&&r["content"].as_array().map(Vec::len)==Some(1)&&r["content"][0]["type"]=="text","Recepción MCP inválida")?;let text=parse(r["content"][0]["text"].as_str().ok_or("Texto ausente")?.as_bytes())?;need(text==r["structuredContent"],"Representaciones RPC discordantes")?;Ok(text)}
}
const ROOT:&str="C:/SV";
const OLD:&str="ejecucion/qwen38-examen25-20261008";
const PRECISION:&str="La prohibición de indicar una técnica significa que no debe nombrar ninguna técnica de laboratorio en ningún campo de su entrega, incluidas las citas, los fundamentos, las insuficiencias y la revisión. Seleccione citas breves continuas de la fuente que cumplan esta restricción. No suprima palabras internas ni altere el texto de una cita.";
fn read(p:&Path)->R<Value>{api::guard(p)?;parse(&fs::read(p).map_err(|e|e.to_string())?)}
fn ident(p:&Path)->R<Value>{api::guard(p)?;let b=fs::read(p).map_err(|e|e.to_string())?;Ok(json!({"ruta":p.to_string_lossy(),"bytes":b.len(),"sha256":sha(&b)}))}
fn base(root:&Path)->R<Value>{
 let mut q=read(&root.join("fuentes-admitidas/P13.json"))?;
 let mut source=parse(q["input"][0]["content"].as_str().ok_or("Fuente ausente")?.as_bytes())?;
 need(source["caso"]=="P13","Caso no autorizado")?;
 let original=source["pregunta"].as_str().ok_or("Pregunta ausente")?;
 need(original.ends_with("No indiques una técnica de laboratorio."),"Enunciado histórico distinto")?;
 source["pregunta"]=json!(format!("{original} {PRECISION}"));
 q["input"][0]["content"]=json!(source.to_string()); Ok(q)
}
fn compose(b:&Value,p:&Perfil,s:usize,history:&[String],sent:&[Value])->R<Value>{
 need(sent.len()==s,"Solicitudes históricas incompletas")?;
 let mut q=contrato::compose(b,s,history)?;
 for input in q["input"].as_array_mut().ok_or("Entradas")? {
  if input["role"]!="user"{continue;}let Some(t)=input["content"].as_str()else{continue;};
  if let Ok(mut record)=parse(t.as_bytes()){if record["tipo"]=="registro_de_instrucciones_historicas"{
   let i=record["etapa_historica"].as_u64().ok_or("Etapa")? as usize;need(i<s,"Historia futura")?;
   let mut old=contrato::compose(b,i,&history[..i])?;api::proteger(&mut old,p)?;
   need(old["instructions"]==sent[i]["instructions"],"Obligaciones históricas no coinciden con envío original")?;
   record["instrucciones_aplicadas"]=sent[i]["instructions"].clone();input["content"]=json!(record.to_string());
  }}
 }api::proteger(&mut q,p)?;Ok(q)
}
fn frozen(root:&Path,profile:&Path)->R<Vec<Value>>{
 let mut out=vec![ident(profile)?,ident(&std::env::current_exe().map_err(|e|e.to_string())?)?];
 let common=Path::new(ROOT).join("desarrollo/acoplamientos-con-el-sv/cliente-api-rust");
 for name in ["Cargo.toml","Cargo.lock","src/lib.rs","src/estricto.rs","src/presupuesto.rs","src/bin/sv-diagnostico-documental.rs","src/documental/contrato.rs","src/documental/localizadores.rs","src/documental/suministro.rs"] {out.push(ident(&common.join(name))?);}
 for name in ["Cargo.toml","src/lib.rs"]{out.push(ident(&Path::new(ROOT).join("desarrollo/acoplamientos-con-el-sv/instrumentacion-rust").join(name))?);}
 for name in ["ADMISION.md","CRITERIOS.json","CUOTA-PREVIA.json","BASE-DIAGNOSTICA.json","RECEPCION-TRANSPORTE-PREVIA.json","fuentes/preguntas.json","fuentes/catalogo-pdq.json","fuentes/pdq.html","fuentes/CLAVE-CORRECCION.md","fuentes/PROTOCOLO.md","suministro/CONTROL-ARBITRO.json"]{out.push(ident(&root.join(name))?);}
 for name in ["sv-mcp-documental","verificar-diario"]{out.push(ident(&Path::new(ROOT).join("compilacion/mcp-pdf-linux/debug").join(name))?);}
 Ok(out)
}
fn check(root:&Path,profile:&Path)->R<()>{
 need(read(&root.join("PREVIA.json"))?["archivos"]==json!(frozen(root,profile)?),"Fuente, perfil o controlador alterados")?;
 suministro::verificar(root)?;need(read(&root.join("BASE-DIAGNOSTICA.json"))?==base(root)?,"Base diagnóstica distinta")?;Ok(())
}
fn prepare(root:&Path,profile:&Path,p:&Perfil)->R<Value>{
 let c=read(&root.join("CRITERIOS.json"))?;need(c["caso"]=="P13"&&c["etapas"]==json!([0,1,2])&&c["preguntas_nuevas"]==0,"Alcance diagnóstico distinto")?;
 let quota=read(&root.join("CUOTA-PREVIA.json"))?;need(quota["modelo"]==p.modelo&&quota["stop_on_exhaust"]==true&&quota["presupuesto_pagado"]==0&&quota["cuota_restante_cota_inferior"].as_u64().is_some_and(|v|v>=api::presupuesto::limite(p)),"Protección de cuota insuficiente")?;
 let previous=Path::new(ROOT).join(OLD);let receipt=read(&previous.join("RECEPCION-TRANSPORTE.json"))?;
 let priorfile=PathBuf::from(receipt["resultado_sha256"]["ruta"].as_str().ok_or("Ruta previa ausente")?);api::guard(&priorfile)?;
 need(priorfile.canonicalize().map_err(|e|e.to_string())?==previous.join("comprobacion-transporte/RESULTADO.json").canonicalize().map_err(|e|e.to_string())?,"Recepción de otro expediente")?;
 need(receipt["conforme"]==true&&receipt["modelo"]==p.modelo&&receipt["resultado_sha256"]==ident(&priorfile)?,"Transporte previo no cotejado")?;
 let supplied=suministro::preparar(root)?;save(&root.join("BASE-DIAGNOSTICA.json"),&base(root)?)?;
 save(&root.join("RECEPCION-TRANSPORTE-PREVIA.json"),&json!({"conforme":true,"referencia":receipt,"inferencias_nuevas":0,"uso_previo_no_imputado":true}))?;
 let v=json!({"conforme":true,"archivos":frozen(root,profile)?,"suministro":supplied,"utc_ms":sv_instrumentacion::utc_ms(),"inferencias":0,"casos_enviables":["P13"],"autoria_licencia":api::LICENCIA});save(&root.join("PREVIA.json"),&v)?;Ok(v)
}
fn execute(root:&Path,profile:&Path,p:&Perfil)->R<Value>{
 check(root,profile)?;
 let keypath=PathBuf::from(std::env::var("SV_API_KEY_FILE").map_err(|_|"Falta ruta de credencial")?);api::guard(&keypath)?;
 let secret=Zeroizing::new(fs::read_to_string(keypath).map_err(|_|"Credencial no disponible")?.trim().to_string());need(secret.len()>=24&&!secret.chars().any(char::is_whitespace),"Credencial inválida")?;
 save(&root.join("INICIO-DIAGNOSTICO.json"),&json!({"utc_ms":sv_instrumentacion::utc_ms(),"caso":"P13","modelo":p.modelo,"etapas":[0,1,2],"maximo_intentos":3,"presupuesto_pagado":0,"licencia":api::LICENCIA}))?;
 let start=Instant::now();let mut history=vec![];let mut sent=vec![];let mut rows=vec![];let mut spent=0u64;let mut state="completo";
 for s in 0..3 {
  check(root,profile)?;
  for (i,h) in history.iter().enumerate(){let d=root.join(format!("intentos/I{:03}-P13-R{i}",i+1));need(read(&d.join("SOLICITUD.json"))?==sent[i]&&fs::read_to_string(d.join("FINAL.txt")).map_err(|e|e.to_string())?==*h,"Historia alterada")?;}
  let q=compose(&base(root)?,p,s,&history,&sent)?;let reserve=api::presupuesto::reserva(&q,p)?;
  if spent.checked_add(reserve).is_none_or(|v|v>api::presupuesto::limite(p)){state="suspendido_por_reserva_de_cuota";break;}
  let d=root.join(format!("intentos/I{:03}-P13-R{s}",s+1));
  save(&root.join(format!("reservas/I{:03}.json",s+1)),&json!({"consumo_previo":spent,"reserva":reserve,"limite":api::presupuesto::limite(p),"unidad":"tokens_cuota_gratuita","presupuesto_pagado":0}))?;
  let result=api::enviar(p,&secret,&q,&d,300000)?;let known=api::presupuesto::consumo(&result["entrega"]["uso_proveedor"],p).ok();spent=spent.checked_add(known.unwrap_or(reserve)).ok_or("Desbordamiento")?;
  let complete=result["completa"]==true&&result["telemetria_conforme"]==true;
  let audit=if complete {let t=result["entrega"]["texto_original"].as_str().ok_or("Texto")?;let a=parse(t.as_bytes()).and_then(|v|contrato::formal(&v,&q)).unwrap_or_else(|e|json!({"conforme":false,"defecto":e}));history.push(t.to_string());sent.push(q);a}else{json!({"conforme":false,"no_adjudicable":"Entrega o telemetría incompleta"})};
  save(&d.join("AUDITORIA-FORMAL.json"),&audit)?;
  let row=json!({"caso":"P13","etapa":s,"intento":s+1,"directorio":d.to_string_lossy(),"resultado":result,"auditoria_formal":audit,"consumo_control":known,"reserva_si_desconocido":if known.is_none(){Some(reserve)}else{None},"solicitud":ident(&d.join("SOLICITUD.json"))?,"resultado_identidad":ident(&d.join("RESULTADO.json"))?});save(&root.join(format!("hitos/I{:03}.json",s+1)),&row)?;rows.push(row);
  println!("P13/R{s}: entrega={complete}; consumo={known:?}; acumulado={spent}");
  if !complete{state="suspendido_por_incidencia_tecnica_o_servicio";save(&root.join("INCIDENCIA.json"),&json!({"proveedor":p.proveedor,"duracion_ms":result["duracion_operacion_ms"],"argumento":result["error"],"fuente_argumento":"HTTP.json y SALIDA-SSE.txt del intento","alcance":"Única pregunta diagnóstica; sin otras preguntas pendientes ni reintento automático","no_adjudicar_al_candidato":true}))?;break;}
  if known.is_none(){state="suspendido_por_consumo_desconocido";break;}
  if known.is_some_and(|v|v>reserve)||spent>api::presupuesto::limite(p){state="suspendido_por_desviacion_de_reserva";break;}
 }
 check(root,profile)?;
 let v=json!({"estado":state,"caso":"P13","proveedor":p.proveedor,"modelo":p.modelo,"duracion_ms":start.elapsed().as_millis(),"intentos":rows,"etapas_completas":history.len(),"consumo_o_reserva_acumulados":spent,"unidad_control":"tokens_cuota_gratuita","adjudicacion":"pendiente de revisión exterior","autoria_licencia":api::LICENCIA});save(&root.join("RESULTADO-BANCO.json"),&v)?;Ok(v)
}
fn run()->R<()>{let a=std::env::args().collect::<Vec<_>>();need(a.len()==4,"Uso: sv-diagnostico-documental preparar|ejecutar PERFIL DIRECTORIO")?;let profile=PathBuf::from(&a[2]);let root=PathBuf::from(&a[3]);api::guard(&root)?;let p:Perfil=serde_json::from_value(read(&profile)?).map_err(|e|e.to_string())?;p.comprobar()?;need(p.proveedor=="Alibaba Cloud"&&p.modelo=="qwen3.8-max-0902"&&p.presupuesto_ticks==0&&p.cuota_gratuita_tokens==Some(300000),"Perfil no autorizado para esta réplica")?;let v=match a[1].as_str(){"preparar"=>prepare(&root,&profile,&p)?,"ejecutar"=>execute(&root,&profile,&p)?,_=>return Err("Operación desconocida".into())};println!("ESTADO={}",v["estado"].as_str().unwrap_or("preparado"));Ok(())}
fn main(){if let Err(e)=run(){eprintln!("IMPEDIMENTO: {e}");std::process::exit(1);}}
#[cfg(test)]mod tests{use super::*;
 fn p()->Perfil{serde_json::from_value(json!({"proveedor":"Alibaba Cloud","modelo":"qwen3.8-max-0902","endpoint":"https://ws-prueba123.ap-southeast-1.maas.aliyuncs.com/compatible-mode/v1/responses","presupuesto_ticks":0,"exigir_zdr":false,"entrada_ticks_por_token":20000,"salida_ticks_por_token":60000,"cuota_gratuita_tokens":300000})).unwrap()}
 #[test]fn historia_protegida_es_la_real_y_no_admite_sustitucion(){let b=json!({"instructions":"Fuente exclusiva","tools":[],"tool_choice":"none","input":[{"role":"user","content":"{\"caso\":\"P13\"}"}]});let r0=compose(&b,&p(),0,&[],&[]).unwrap();let r1=compose(&b,&p(),1,&["respuesta original".into()],&[r0.clone()]).unwrap();assert!(r1["instructions"].as_str().unwrap().contains(api::LICENCIA));assert_eq!(r1["tools"],json!([]));let mut bad=r0;bad["instructions"]=json!("sustituidas");assert!(compose(&b,&p(),1,&["respuesta original".into()],&[bad]).is_err());}
}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
