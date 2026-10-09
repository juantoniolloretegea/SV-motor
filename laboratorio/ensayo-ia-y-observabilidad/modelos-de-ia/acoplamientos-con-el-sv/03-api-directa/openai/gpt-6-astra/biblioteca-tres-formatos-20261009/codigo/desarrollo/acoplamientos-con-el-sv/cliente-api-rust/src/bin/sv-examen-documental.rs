#![forbid(unsafe_code)]
use sv_cliente_api::{self as api,need,parse,save,sha,Perfil,R};
use serde_json::{json,Value};
use std::{fs,path::{Path,PathBuf},time::{Instant,Duration},collections::VecDeque};
use zeroize::Zeroizing;
#[path="../documental/contrato.rs"] mod contrato;
#[path="../documental/localizadores.rs"] mod localizadores;
#[path="../documental/suministro.rs"] mod suministro;
mod suministro_pdf{
 pub use sv_cliente_api::{parse,need,sha,R};use serde_json::Value;
 pub fn num(v:&Value)->R<usize>{v.as_u64().and_then(|x|usize::try_from(x).ok()).ok_or("Entero requerido".into())}
 pub fn contenido(v:&Value,id:usize)->R<Value>{need(v["jsonrpc"]=="2.0"&&v["id"]==id&&v.get("error").is_none(),"Identidad RPC discordante")?;let r=&v["result"];need(r["isError"]==false&&r["content"].as_array().map(Vec::len)==Some(1)&&r["content"][0]["type"]=="text","Recepción MCP inválida")?;let text=parse(r["content"][0]["text"].as_str().ok_or("Texto ausente")?.as_bytes())?;need(text==r["structuredContent"],"Representaciones RPC discordantes")?;Ok(text)}
}
const ROOT:&str="C:/laboratorio/watson-local/lenguaje-computacion-sv";
fn read(p:&Path)->R<Value>{parse(&fs::read(p).map_err(|e|e.to_string())?)}
fn confined(p:&Path)->R<PathBuf>{
 let lexical=if p.is_absolute(){p.to_path_buf()}else{std::env::current_dir().map_err(|e|e.to_string())?.join(p)};
 let relative=lexical.strip_prefix(ROOT).map_err(|_|"Ruta original fuera del perímetro")?;
 let mut cur=PathBuf::from(ROOT);
 for c in relative.components(){need(matches!(c,std::path::Component::Normal(_)),"Componente de ruta no admitido")?;cur.push(c);let m=fs::symlink_metadata(&cur).map_err(|e|e.to_string())?;need(!m.file_type().is_symlink(),"Enlace no admitido")?;#[cfg(windows)]{use std::os::windows::fs::MetadataExt;need(m.file_attributes()&0x400==0,"Reanálisis no admitido")?;}}
 let canonical=cur.canonicalize().map_err(|e|e.to_string())?;need(canonical.starts_with(Path::new(ROOT).canonicalize().map_err(|e|e.to_string())?),"Destino efectivo fuera del perímetro")?;Ok(cur)
}
fn identity(p:&Path)->R<Value>{let a=confined(p)?;let b=fs::read(&a).map_err(|e|e.to_string())?;Ok(json!({"ruta":a.to_string_lossy(),"bytes":b.len(),"sha256":sha(&b)}))}
fn frozen(root:&Path,profile:&Path)->R<Vec<Value>>{
 let mut out=vec![identity(profile)?,identity(&std::env::current_exe().map_err(|e|e.to_string())?)?,identity(&root.join("ADMISION.md"))?];
 let common=Path::new(ROOT).join("desarrollo/acoplamientos-con-el-sv/cliente-api-rust");
 for s in ["Cargo.toml","Cargo.lock","src/lib.rs","src/estricto.rs","src/presupuesto.rs","src/bin/sv-examen-documental.rs","src/documental/contrato.rs","src/documental/localizadores.rs","src/documental/suministro.rs"]{out.push(identity(&common.join(s))?);}
 for s in ["Cargo.toml","src/lib.rs"]{out.push(identity(&Path::new(ROOT).join("desarrollo/acoplamientos-con-el-sv/instrumentacion-rust").join(s))?);}
 for s in ["preguntas.json","catalogo-pdq.json","pdq.html","CLAVE-CORRECCION.md","PROTOCOLO.md"]{out.push(identity(&root.join("fuentes").join(s))?);}
 for n in 1..=25{out.push(identity(&root.join(format!("fuentes-admitidas/P{n:02}.json")))?);}Ok(out)
}
fn check(root:&Path,profile:&Path)->R<()>{let previous=read(&root.join("PREVIA.json"))?;need(previous["archivos"]==json!(frozen(root,profile)?),"Fuentes, perfil o controlador alterados")?;suministro::verificar(root)}
fn key()->R<Zeroizing<String>>{let file=std::env::var("SV_API_KEY_FILE").map_err(|_|"SV_API_KEY_FILE no definido")?;let path=confined(Path::new(&file))?;let key=Zeroizing::new(fs::read_to_string(path).map_err(|_|"No se lee credencial privada")?.trim().to_string());need(key.len()>=24&&!key.chars().any(char::is_whitespace),"Credencial inválida")?;Ok(key)}
fn smoke(p:&Perfil,root:&Path)->R<Value>{
 let mut q=json!({"instructions":"Prueba exclusivamente técnica. Sin Internet ni herramientas. Devuelva exactamente el objeto JSON solicitado, sin contenido adicional.","input":"Devuelva {\"conexion\":\"confirmada\",\"valor\":7}","max_output_tokens":1024,"reasoning":{"effort":"medium"},"text":{"format":{"type":"json_schema","name":"sv_prueba_tecnica","strict":true,"schema":{"type":"object","properties":{"conexion":{"type":"string","enum":["confirmada"]},"valor":{"type":"integer","enum":[7]}},"required":["conexion","valor"],"additionalProperties":false}}}});
 api::proteger(&mut q,p)?;save(&root.join("RESERVA-TECNICA.json"),&json!({"ticks":api::presupuesto::reserva(&q,p)?,"limite":api::presupuesto::limite(p),"unidad":api::presupuesto::unidad(p)}))?;
 let r=api::enviar(p,&key()?,&q,&root.join("comprobacion-transporte"),300000)?;
 need(r["completa"]==true&&r["telemetria_conforme"]==true,"Comprobación técnica no conforme; véase original")?;
 need(parse(r["entrega"]["texto_original"].as_str().ok_or("Texto")?.as_bytes())?==json!({"conexion":"confirmada","valor":7}),"Respuesta técnica discordante")?;
 let cost=api::presupuesto::consumo(&r["entrega"]["uso_proveedor"],p)?;
 need(cost<=api::presupuesto::reserva(&q,p)?&&cost<api::presupuesto::limite(p),"Desviación de coste técnico")?;
 let ok=json!({"conforme":true,"modelo":p.modelo,"cost_in_usd_ticks":r["entrega"]["uso_proveedor"]["cost_in_usd_ticks"],"consumo_control":cost,"unidad_control":api::presupuesto::unidad(p),"resultado_sha256":identity(&root.join("comprobacion-transporte/RESULTADO.json"))?,"reserva_ticks":api::presupuesto::reserva(&q,p)?});save(&root.join("RECEPCION-TRANSPORTE.json"),&ok)?;Ok(ok)
}
fn compose(root:&Path,p:&Perfil,n:usize,stage:usize)->R<Value>{
 let b=read(&root.join(format!("fuentes-admitidas/P{n:02}.json")))?;let mut history=vec![];
 for s in 0..stage{let h=read(&root.join(format!("recibidos/P{n:02}-R{s}.json")))?;let path=confined(Path::new(h["directorio"].as_str().ok_or("Procedencia ausente")?))?;let r=read(&path.join("RESULTADO.json"))?;need(h["resultado"]==identity(&path.join("RESULTADO.json"))?&&h["texto"]==identity(&path.join("FINAL.txt"))?&&r["completa"]==true&&r["telemetria_conforme"]==true,"Historia no cotejada")?;let text=fs::read_to_string(path.join("FINAL.txt")).map_err(|_|"Historia no recibida")?;need(r["entrega"]["texto_original"]==text,"Historia discordante")?;history.push(text);}
 protected_compose(&b,stage,&history,p)
}
fn protected_compose(b:&Value,stage:usize,history:&[String],p:&Perfil)->R<Value>{
 let mut q=contrato::compose(b,stage,history)?;
 for input in q["input"].as_array_mut().ok_or("Entradas ausentes")?{
  if input["role"]!="user"{continue;}let Some(t)=input["content"].as_str()else{continue;};
  if let Ok(mut record)=parse(t.as_bytes()){if record["tipo"]=="registro_de_instrucciones_historicas"{
   let i=record["etapa_historica"].as_u64().ok_or("Etapa histórica ausente")? as usize;need(i<stage,"Antecedente futuro")?;
   let mut old=contrato::compose(b,i,&history[..i])?;api::proteger(&mut old,p)?;
   record["instrucciones_aplicadas"]=old["instructions"].clone();input["content"]=json!(record.to_string());
  }}
 }api::proteger(&mut q,p)?;Ok(q)
}
fn execute(p:&Perfil,root:&Path,profile:&Path)->R<Value>{
 check(root,profile)?;let receipt=read(&root.join("RECEPCION-TRANSPORTE.json"))?;need(receipt["conforme"]==true,"Falta recepción del transporte")?;
 let mut spent=receipt["consumo_control"].as_u64().or(receipt["cost_in_usd_ticks"].as_u64()).ok_or("Consumo técnico ausente")?;need(spent<api::presupuesto::limite(p),"Presupuesto agotado")?;
 save(&root.join("INICIO-EXAMEN.json"),&json!({"utc_ms":sv_instrumentacion::utc_ms(),"perfil":p,"licencia":api::LICENCIA,"herramientas":0,"etapas_por_pregunta":3}))?;
 let secret=key()?;let start=Instant::now();let mut unavailable:Option<Instant>=None;let mut queue:VecDeque<usize>=(1..=25).collect();let mut stage=[0usize;25];let mut rows=vec![];let mut status="completo";let mut attempt=0;
 'bank:while let Some(n)=queue.pop_front(){for s in stage[n-1]..3{
  check(root,profile)?;if start.elapsed()>Duration::from_secs(5400)||attempt>=100{status="suspendido_por_limite_temporal_o_intentos";break 'bank;}
  let remaining=unavailable.as_ref().map(|t|300000u64.saturating_sub(t.elapsed().as_millis() as u64)).unwrap_or(300000);if remaining==0{status="prueba_no_valida_por_falta_de_recursos_que_garanticen_el_examen";break 'bank;}
  let q=compose(root,p,n,s)?;let reserve=api::presupuesto::reserva(&q,p)?;
  if spent.checked_add(reserve).is_none_or(|v|v>api::presupuesto::limite(p)){status="suspendido_por_reserva_presupuestaria";break 'bank;}
  attempt+=1;let dest=root.join(format!("intentos/I{attempt:03}-P{n:02}-R{s}"));save(&root.join(format!("reservas/I{attempt:03}.json")),&json!({"consumo_previo":spent,"reserva":reserve,"limite":api::presupuesto::limite(p),"unidad":api::presupuesto::unidad(p),"presupuesto_pagado_ticks":p.presupuesto_ticks,"solicitud_sha256":sha(&serde_json::to_vec(&q).unwrap())}))?;
  let call_start=Instant::now();let result=api::enviar(p,&secret,&q,&dest,remaining)?;
  let evidence=read(&dest.join("ENTREGA-PROVEEDOR.json")).unwrap_or(Value::Null);
  let known=api::presupuesto::consumo(&result["entrega"]["uso_proveedor"],p).ok().or_else(||api::presupuesto::consumo(&evidence["uso_proveedor"],p).ok());let accounted=known.unwrap_or(reserve);spent=spent.checked_add(accounted).ok_or("Desbordamiento de gasto")?;
  let mut row=json!({"caso":format!("P{n:02}"),"etapa":s,"intento":attempt,"directorio":dest.to_string_lossy(),"coste_comunicado_ticks":evidence["uso_proveedor"]["cost_in_usd_ticks"],"consumo_control":known,"unidad_control":api::presupuesto::unidad(p),"reserva_si_consumo_desconocido":if known.is_none(){Some(reserve)}else{None},"resultado":result,"adjudicacion_sustantiva":"pendiente"});
  let complete=result["completa"]==true&&result["telemetria_conforme"]==true;
  if complete{let text=result["entrega"]["texto_original"].as_str().ok_or("Texto ausente")?;let audit=parse(text.as_bytes()).and_then(|v|contrato::formal(&v,&q)).unwrap_or_else(|e|json!({"conforme":false,"defecto":e}));save(&dest.join("AUDITORIA-FORMAL.json"),&audit)?;save(&root.join(format!("recibidos/P{n:02}-R{s}.json")),&json!({"directorio":dest.to_string_lossy(),"resultado":identity(&dest.join("RESULTADO.json"))?,"texto":identity(&dest.join("FINAL.txt"))?}))?;row["auditoria_formal"]=audit;stage[n-1]=s+1;unavailable=None;}
  row["originales"]=json!([identity(&dest.join("RESULTADO.json"))?,identity(&dest.join("SOLICITUD.json"))?]);save(&root.join(format!("hitos/I{attempt:03}.json")),&row)?;rows.push(row);println!("ENTREGA=P{n:02}/R{s} COMPLETA={complete} CONSUMO={known:?} ACUMULADO={spent}");
  if known.is_some_and(|x|x>reserve)||spent>api::presupuesto::limite(p){status="suspendido_por_desviacion_de_coste";break 'bank;}
  if !complete{
   let http=read(&dest.join("HTTP.json")).unwrap_or(Value::Null);let code=http["status"].as_u64();
   let events=read(&dest.join("EVENTOS.json")).unwrap_or(Value::Null);let provider_failed=events.as_array().is_some_and(|a|a.iter().any(|v|v["type"]=="response.failed"));
   if (matches!(code,Some(429|500|502|503|504))||provider_failed)&&result["telemetria_conforme"]==true{unavailable.get_or_insert(call_start);save(&dest.join("INCIDENCIA-SERVICIO.json"),&json!({"proveedor":p.proveedor,"codigo_http":code,"duracion_intento_ms":call_start.elapsed().as_millis(),"duracion_interrupcion_ms":unavailable.unwrap().elapsed().as_millis(),"argumento_original":"SALIDA-SSE.txt y EVENTOS.json cuando exista","alcance":"Entrega solicitada no disponible; no se adjudica al candidato","accion":"aplazada al final"}))?;if unavailable.unwrap().elapsed()>=Duration::from_secs(300){status="prueba_no_valida_por_falta_de_recursos_que_garanticen_el_examen";break 'bank;}queue.push_back(n);std::thread::sleep(Duration::from_secs(6));break;}
   status="suspendido_por_incidencia_tecnica";break 'bank;
  }
  if known.is_none(){status="suspendido_por_consumo_no_comunicado";break 'bank;}
 }}
 let v=json!({"estado":status,"proveedor":p.proveedor,"modelo":p.modelo,"duracion_ms":start.elapsed().as_millis(),"consumo_o_reserva_acumulados":spent,"unidad_control":api::presupuesto::unidad(p),"coste_o_reserva_acumulados_ticks":if p.cuota_gratuita_tokens.is_none(){Some(spent)}else{None},"duracion_interrupcion_abierta_ms":unavailable.map(|t|t.elapsed().as_millis()),"etapas_completas_por_pregunta":stage,"intentos":rows,"adjudicacion_sustantiva":"pendiente de revisión externa al candidato"});save(&root.join("RESULTADO-BANCO.json"),&v)?;Ok(v)
}
fn run()->R<()>{let args=std::env::args().collect::<Vec<_>>();need(args.len()==4,"Uso: sv-examen-documental preparar|tecnica|examinar PERFIL DIRECTORIO")?;let profile=confined(Path::new(&args[2]))?;let root=confined(Path::new(&args[3]))?;let p:Perfil=serde_json::from_value(read(&profile)?).map_err(|e|e.to_string())?;p.comprobar()?;
 let v=match args[1].as_str(){"preparar"=>{let supply=suministro::preparar(&root)?;let v=json!({"conforme":true,"perfil":p,"suministro":supply,"archivos":frozen(&root,&profile)?,"utc_ms":sv_instrumentacion::utc_ms(),"inferencia":false});save(&root.join("PREVIA.json"),&v)?;v},"tecnica"=>{check(&root,&profile)?;smoke(&p,&root)?},"examinar"=>execute(&p,&root,&profile)?,_=>return Err("Operación no admitida".into())};println!("OPERACION={} ESTADO={}",args[1],v["estado"].as_str().unwrap_or("conforme"));Ok(())}
fn main(){if let Err(e)=run(){eprintln!("IMPEDIMENTO: {e}");std::process::exit(1);}}
