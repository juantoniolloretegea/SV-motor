//! Control CYB16: contrato idéntico, credencial de sesión y transporte compartido.
use sv_cliente_api::{self as api,need,parse,save,sha,Perfil,R};
use serde_json::{json,Value};
use std::{fs,path::{Path,PathBuf},time::{Instant,Duration}};
use crate::{contrato,suministro};
const WORK:&str="C:/SV";
const RUN:&str="ejecucion/astra-cyb16-20261008-r2";
const REFERENCE:&str="ejecucion/zai-cyb16-20261008";
pub fn root()->PathBuf{Path::new(WORK).join(RUN)}
fn load(p:&Path)->R<Value>{api::guard(p)?;parse(&fs::read(p).map_err(|e|e.to_string())?)}
fn identity(p:&Path)->R<Value>{api::guard(p)?;let b=fs::read(p).map_err(|e|e.to_string())?;Ok(json!({"ruta":p.strip_prefix(WORK).map_err(|_|"Fuera del perímetro")?.to_string_lossy(),"bytes":b.len(),"sha256":sha(&b)}))}
fn config()->R<(Perfil,suministro::ContratoLibro)>{
 let p:Perfil=serde_json::from_value(load(&root().join("PERFIL.json"))?).map_err(|e|e.to_string())?;p.comprobar()?;
 need(p.proveedor=="OpenAI"&&p.modelo=="gpt-6-astra"&&p.presupuesto_ticks==0&&!p.exigir_zdr&&p.cuota_gratuita_tokens.is_none(),"Modalidad o modelo distintos")?;
 let c:suministro::ContratoLibro=serde_json::from_value(load(&root().join("CONTRATO-LIBRO.json"))?).map_err(|e|e.to_string())?;c.comprobar()?;
 need(c.documentos==5&&c.preguntas==16&&c.prefijo=="C"&&c.modelo.as_deref()==Some("gpt-6-astra"),"Banco distinto")?;Ok((p,c))
}
fn compose(base:&Value,p:&Perfil,stage:usize,history:&[String])->R<Value>{
 let mut q=contrato::compose_documental(base,stage,history)?;
 q["max_output_tokens"]=json!(16384);q["reasoning"]=json!({"effort":"high","summary":"auto"});
 // El registro histórico contiene las instrucciones realmente transmitidas,
 // incluida la protección de derechos, no una reconstrucción abreviada.
 for m in q["input"].as_array_mut().ok_or("Entrada ausente")?{
  if m["role"]!="user"{continue;}let Some(t)=m["content"].as_str()else{continue;};
  if let Ok(mut v)=parse(t.as_bytes()){if v["tipo"]=="registro_de_instrucciones_historicas"{
   let s=v["etapa_historica"].as_u64().ok_or("Etapa histórica")? as usize;need(s<stage,"Etapa futura")?;
   v["instrucciones_aplicadas"]=compose(base,p,s,&history[..s])?["instructions"].clone();m["content"]=json!(v.to_string());
  }}
 }
 api::proteger(&mut q,p)?;
 need(q["input"][0]==base["input"][0]&&q["tools"]==json!([])&&q["tool_choice"]=="none"&&q["store"]==false,"Composición fuera del contrato")?;
 need(serde_json::to_vec(&q).map_err(|e|e.to_string())?.len()<190000,"Contexto fuera de cota")?;Ok(q)
}
fn compare_reference()->R<Value>{
 let old=Path::new(WORK).join(REFERENCE);let mut rows=vec![];
 for f in ["fuentes/BANCO.json","reservado/CLAVE.json","fuentes/preparado/CATALOGO.json","fuentes/preparado/FUENTES.json","CRITERIOS.md","SELECCION.json"]{
  let a=identity(&old.join(f))?;let b=identity(&root().join(f))?;need(a["bytes"]==b["bytes"]&&a["sha256"]==b["sha256"],"Referencia documental distinta")?;rows.push(json!({"archivo":f,"sha256":a["sha256"],"bytes":a["bytes"]}));
 }
 let mut c=load(&old.join("CONTRATO-LIBRO.json"))?;c["modelo"]=json!("gpt-6-astra");need(c==load(&root().join("CONTRATO-LIBRO.json"))?,"Encargo semántico distinto")?;
 for i in 1..=16{let a=load(&old.join(format!("fuentes-admitidas/C{i:02}.json")))?;let b=load(&root().join(format!("fuentes-admitidas/C{i:02}.json")))?;need(a["input"]==b["input"]&&a["instructions"]==b["instructions"],"Suministro del candidato distinto")?;}
 Ok(json!({"conforme":true,"archivos":rows,"preguntas":16,"criticos":14,"entregas_previstas":48,"comparacion":"Mismos bytes de corpus, preguntas, clave y criterios; composición de proveedor declarada","licencia":api::LICENCIA}))
}
fn frozen()->R<Value>{
 let mut files=vec![];
 for f in ["ADMISION.md","CRITERIOS.md","SELECCION.json","ANTECEDENTE.json","PERFIL.json","CONTRATO-LIBRO.json","COMPARABILIDAD-PREVIA.json","HITO-HEREDADO.json","fuentes/BANCO.json","reservado/CLAVE.json","fuentes/preparado/CATALOGO.json","fuentes/preparado/FUENTES.json","suministro/CONTROL-ARBITRO.json"]{files.push(identity(&root().join(f))?);}
 for i in 1..=16{files.push(identity(&root().join(format!("fuentes-admitidas/C{i:02}.json")))?);}
 let common=Path::new(WORK).join("desarrollo/acoplamientos-con-el-sv/cliente-api-rust");
 for f in ["Cargo.toml","Cargo.lock","src/lib.rs","src/responses.rs","src/estricto.rs","src/presupuesto.rs","src/chat.rs","src/manual/contrato.rs","src/manual/contrato-base.rs","src/manual/localizadores.rs","src/manual/suministro.rs","src/bin/sv-cyb16-chatgpt/main.rs","src/bin/sv-cyb16-chatgpt/cyb.rs","src/bin/sv-recuperar-responses.rs"]{files.push(identity(&common.join(f))?);}
 for f in ["Cargo.toml","src/lib.rs"]{files.push(identity(&Path::new(WORK).join("desarrollo/acoplamientos-con-el-sv/instrumentacion-rust").join(f))?);}
 files.push(identity(&std::env::current_exe().map_err(|e|e.to_string())?)?);
 for f in ["sv-mcp-documental","verificar-diario"]{files.push(identity(&Path::new(WORK).join("compilacion/mcp-mdbook/debug").join(f))?);}
 Ok(json!(files))
}
pub fn check()->R<()>{let(_,c)=config()?;need(load(&root().join("PREVIA.json"))?["archivos"]==frozen()?,"Fuentes o controlador alterados")?;suministro::verificar_contrato(&root(),&c)}
pub fn prepare()->R<Value>{
 let(p,c)=config()?;need(!root().join("PREVIA.json").exists(),"Admisión existente")?;
 suministro::preparar_contrato(&root(),&c)?;save(&root().join("COMPARABILIDAD-PREVIA.json"),&compare_reference()?)?;
 // Recepción corregida de la única solicitud ya realizada: no se repite C01/R0.
 let relative="intentos/I001-C01-R0";let d=root().join(relative);let recovered=load(&d.join("RESULTADO.json"))?;
 need(recovered["completa"]==true&&recovered["recuperacion_sin_inferencia"]["inferencias_nuevas"]==0,"Herencia no recibida")?;
 let prior=Path::new(WORK).join("ejecucion/astra-cyb16-20261008/intentos/I001-C01-R0");
 need(identity(&prior.join("SALIDA-SSE.txt"))?["sha256"]==identity(&d.join("SALIDA-SSE.txt"))?["sha256"]&&identity(&prior.join("SOLICITUD.json"))?["sha256"]==identity(&d.join("SOLICITUD.json"))?["sha256"],"Originales heredados discordantes")?;
 let q=compose(&load(&root().join("fuentes-admitidas/C01.json"))?,&p,0,&[])?;need(q==load(&d.join("SOLICITUD.json"))?,"Encargo heredado distinto")?;
 let formal=parse(recovered["entrega"]["texto_original"].as_str().ok_or("Texto")?.as_bytes()).and_then(|v|contrato::formal(&v,&q)).unwrap_or_else(|e|json!({"conforme":false,"defecto":e}));save(&d.join("AUDITORIA-FORMAL.json"),&formal)?;
 let row=json!({"intento":1,"caso":"C01","etapa":0,"pasada":0,"directorio":relative,"completa":true,"resultado":identity(&d.join("RESULTADO.json"))?,"texto":identity(&d.join("FINAL.txt"))?,"auditoria_formal":formal,"http":200,"incidencia":null,"duracion_operacion_ms":recovered["duracion_operacion_ms"],"creditos_atribuibles":null,"importe_atribuible":null,"heredado_sin_inferencia":true});save(&root().join("HITO-HEREDADO.json"),&row)?;
 let mut rows=vec![];
 for i in 1..=16{let b=load(&root().join(format!("fuentes-admitidas/C{i:02}.json")))?;for s in 0..3{
  let h=(0..s).map(|_|"Antecedente sintético de comprobación; no respuesta del candidato".to_string()).collect::<Vec<_>>();let q=compose(&b,&p,s,&h)?;
  rows.push(json!({"caso":format!("C{i:02}"),"etapa":s,"bytes_sinteticos":serde_json::to_vec(&q).map_err(|e|e.to_string())?.len(),"herramientas":0,"esfuerzo":"high","maximo_salida_tokens":16384}));
 }}
 let v=json!({"conforme":true,"inferencias":0,"preguntas":16,"archivos":frozen()?,"composiciones":rows,"utc_ms":sv_instrumentacion::utc_ms(),"licencia":api::LICENCIA});save(&root().join("PREVIA.json"),&v)?;Ok(v)
}
fn history(id:&str,stage:usize,rows:&[Value])->R<Vec<String>>{
 let mut h=vec![];for s in 0..stage{let r=rows.iter().find(|r|r["caso"]==id&&r["etapa"]==s&&r["completa"]==true).ok_or("Historia completa ausente")?;let d=root().join(r["directorio"].as_str().ok_or("Directorio")?);need(r["resultado"]==identity(&d.join("RESULTADO.json"))?&&r["texto"]==identity(&d.join("FINAL.txt"))?,"Antecedente alterado")?;h.push(fs::read_to_string(d.join("FINAL.txt")).map_err(|e|e.to_string())?);}Ok(h)
}
fn issue(dir:&Path,result:&Value,http:&Value)->R<Value>{
 let mut code=Value::Null;let mut argument=Value::Null;
 if dir.join("SALIDA-SSE.txt").exists(){let raw=fs::read(dir.join("SALIDA-SSE.txt")).map_err(|e|e.to_string())?;
  if let Ok(v)=parse(&raw){code=v["error"]["code"].clone();argument=v["error"]["message"].clone();}
  else {for line in raw.split(|b|*b==b'\n'){if let Ok(s)=std::str::from_utf8(line){if let Some(data)=s.strip_prefix("data:"){if let Ok(v)=parse(data.trim().as_bytes()){if matches!(v["type"].as_str(),Some("response.failed"|"error")){let e=if v["type"]=="response.failed"{&v["response"]["error"]}else{&v};code=e["code"].clone();argument=e["message"].clone();}}}}}}
 }
 let retry=http["status"].as_u64().is_some_and(|s|s==429||s>=500)||matches!(code.as_str(),Some("server_is_overloaded"|"server_error"|"rate_limit_exceeded"))||result["error"].as_str().is_some_and(|s|s.contains("Transporte interrumpido")||s.contains("Lectura interrumpida"));
 Ok(json!({"proveedor":"OpenAI","http":http["status"],"codigo":code,"argumento_proveedor":argument,"reanudable":retry,"duracion_operacion_ms":result["duracion_operacion_ms"],"error_local":result["error"],"alcance":"Solicitud sin recepción completa; no se atribuye causalidad interna sin argumento del proveedor"}))
}
fn remaining(interruption:Option<Instant>)->u64{interruption.map(|t|300000u128.saturating_sub(t.elapsed().as_millis()) as u64).unwrap_or(300000)}
pub fn execute(token:&str)->R<Value>{
 check()?;let(p,_)=config()?;need(!root().join("INICIO.json").exists(),"Ensayo ya iniciado; no repetición automática")?;
 let cat=load(&root().join("catalogo.json"))?;need(cat["identidad_verificada"]==true&&cat["modelos"].as_array().is_some_and(|a|a.iter().any(|m|m["slug"]=="gpt-6-astra")),"Sesión o modelo no verificados")?;
 save(&root().join("INICIO.json"),&json!({"utc_ms":sv_instrumentacion::utc_ms(),"modalidad":"ChatGPT OAuth","preguntas":16,"etapas":3,"maximo_intentos":64,"catalogo_sha256":identity(&root().join("catalogo.json"))?["sha256"],"compras":false,"recargas":false,"licencia":api::LICENCIA}))?;
 // Compatibilidad con el informe de sesión: marca de envío, sin credenciales.
 save(&root().join("envio-unico.json"),&json!({"utc_ms":sv_instrumentacion::utc_ms(),"control":"INICIO.json"}))?;
 let start=Instant::now();let inherited=load(&root().join("HITO-HEREDADO.json"))?;save(&root().join("hitos/I001.json"),&inherited)?;let mut rows=vec![inherited];let mut progress=[0usize;16];progress[0]=1;let mut status="completo";let mut interruption:Option<Instant>=None;
 'bank:for pass in 0..2{for(i,next)in progress.iter_mut().enumerate(){let id=format!("C{:02}",i+1);while *next<3{
  if start.elapsed().as_secs()>=5400||rows.len()>=64{status="suspendido_por_cota_de_ejecucion";break 'bank;}
  let timeout=remaining(interruption).min(5400000u128.saturating_sub(start.elapsed().as_millis()) as u64);
  if timeout==0{status="no_valido_por_interrupcion_superior_a_cinco_minutos";break 'bank;}
  check()?;let s=*next;let base=load(&root().join(format!("fuentes-admitidas/{id}.json")))?;let q=compose(&base,&p,s,&history(&id,s,&rows)?)?;
  let n=rows.len()+1;let relative=format!("intentos/I{n:03}-{id}-R{s}");let dir=root().join(&relative);let attempt_start=Instant::now();
  let r=api::enviar_chatgpt(&p,token,&q,&dir,timeout)?;let complete=r["completa"]==true&&r["telemetria_conforme"]==true;
  let formal=if r["completa"]==true{parse(r["entrega"]["texto_original"].as_str().ok_or("Texto ausente")?.as_bytes()).and_then(|v|contrato::formal(&v,&q)).unwrap_or_else(|e|json!({"conforme":false,"defecto":e}))}else{Value::Null};save(&dir.join("AUDITORIA-FORMAL.json"),&formal)?;
  let http=if dir.join("HTTP.json").exists(){load(&dir.join("HTTP.json"))?}else{Value::Null};let incident=if complete{Value::Null}else{issue(&dir,&r,&http)?};save(&dir.join("INCIDENCIA-SERVICIO.json"),&incident)?;
  let row=json!({"intento":n,"caso":id,"etapa":s,"pasada":pass,"directorio":relative,"completa":complete,"resultado":identity(&dir.join("RESULTADO.json"))?,"texto":if r["completa"]==true{identity(&dir.join("FINAL.txt"))?}else{Value::Null},"auditoria_formal":formal,"http":http["status"],"incidencia":incident,"duracion_operacion_ms":r["duracion_operacion_ms"],"creditos_atribuibles":null,"importe_atribuible":null});
  save(&root().join(format!("hitos/I{n:03}.json")),&row)?;rows.push(row);println!("{id} R{s}: completa={complete}; formal={}; intento={n}",formal["conforme"]);
  if r["telemetria_conforme"]!=true{status="suspendido_por_instrumentacion";break 'bank;}
  if complete{*next+=1;interruption=None;}else{if incident["reanudable"]==true{interruption.get_or_insert(attempt_start);if remaining(interruption)==0{status="no_valido_por_interrupcion_superior_a_cinco_minutos";break 'bank;}break;}status="suspendido_por_incidencia_de_recepcion";break 'bank;}
 }}if progress.iter().all(|n|*n==3){break;}if pass==0{std::thread::sleep(Duration::from_secs(1));}}
 if status=="completo"&&!progress.iter().all(|n|*n==3){status="incompleto_por_interrupciones";}
 let v=json!({"estado":status,"duracion_ms":start.elapsed().as_millis(),"entregas_completas":progress.iter().sum::<usize>(),"progreso":progress,"intentos":rows,"adjudicacion":"pendiente de revisión exterior","licencia":api::LICENCIA});save(&root().join("RESULTADO-BANCO.json"),&v)?;Ok(v)
}
pub fn receive()->R<Value>{
 check()?;let(p,_)=config()?;let bank=load(&root().join("RESULTADO-BANCO.json"))?;let rows=bank["intentos"].as_array().ok_or("Diario ausente")?;
 let(mut input,mut output,mut total,mut cached,mut reasoning,mut samples,mut gap,mut unknown)=(0u64,0u64,0u64,0u64,0u64,0u64,0u64,0u64);let mut entries=vec![];
 for row in rows{
  let dir=root().join(row["directorio"].as_str().ok_or("Directorio")?);need(row["resultado"]==identity(&dir.join("RESULTADO.json"))?,"Resultado alterado")?;
  need(load(&root().join(format!("hitos/I{:03}.json",row["intento"].as_u64().ok_or("Intento")?)))?==*row,"Hito discordante")?;
  let r=load(&dir.join("RESULTADO.json"))?;let t=sv_instrumentacion::verify(&dir.join("instrumentacion/telemetria.jsonl"))?;need(t==r["telemetria"],"Medidas discordantes")?;
  let id=row["caso"].as_str().ok_or("Caso")?;let s=row["etapa"].as_u64().ok_or("Etapa")? as usize;let b=load(&root().join(format!("fuentes-admitidas/{id}.json")))?;let q=compose(&b,&p,s,&history(id,s,rows)?)?;need(q==load(&dir.join("SOLICITUD.json"))?,"Solicitud no reproducida")?;
  let mut usage=Value::Null;
  if row["completa"]==true{
   let mut stream=api::Flujo::default();stream.feed(&fs::read(dir.join("SALIDA-SSE.txt")).map_err(|e|e.to_string())?)?;let got=stream.recibir(&p.modelo)?;
   need(got["texto_original"]==fs::read_to_string(dir.join("FINAL.txt")).map_err(|e|e.to_string())?&&got["uso_proveedor"]==r["entrega"]["uso_proveedor"]&&row["texto"]==identity(&dir.join("FINAL.txt"))?,"Entrega sin concordancia")?;
   usage=got["uso_proveedor"].clone();let u=&usage;let i=u["input_tokens"].as_u64().ok_or("Entrada desconocida")?;let o=u["output_tokens"].as_u64().ok_or("Salida desconocida")?;let n=u["total_tokens"].as_u64().ok_or("Total desconocido")?;need(i.checked_add(o)==Some(n),"Tokens incoherentes")?;input+=i;output+=o;total+=n;cached+=u["input_tokens_details"]["cached_tokens"].as_u64().unwrap_or(0);reasoning+=u["output_tokens_details"]["reasoning_tokens"].as_u64().unwrap_or(0);
   let f=parse(got["texto_original"].as_str().unwrap().as_bytes()).and_then(|v|contrato::formal(&v,&q)).unwrap_or_else(|e|json!({"conforme":false,"defecto":e}));need(f==row["auditoria_formal"],"Auditoría discordante")?;
  }else{unknown+=1;}
  samples+=t["muestras"].as_u64().ok_or("Muestras")?;gap=gap.max(t["intervalo_maximo_ms"].as_u64().ok_or("Intervalo")?);
  entries.push(json!({"caso":id,"etapa":s,"intento":row["intento"],"directorio":row["directorio"],"completa":row["completa"],"uso":usage,"telemetria":t,"duracion_operacion_ms":r["duracion_operacion_ms"],"primer_texto_ms":r["entrega"]["primer_texto_ms"],"auditoria_formal":row["auditoria_formal"],"final_sha256":row["texto"]["sha256"],"creditos_atribuibles":null,"importe_atribuible":null}));
 }
 let v=json!({"conforme":true,"estado_banco":bank["estado"],"entregas_completas":bank["entregas_completas"],"casos":entries,"tokens_entrada":input,"tokens_salida":output,"tokens_totales":total,"tokens_cache_incluidos":cached,"tokens_razonamiento_incluidos":reasoning,"intentos_uso_desconocido":unknown,"creditos_atribuibles":null,"importe_atribuible":null,"duracion_ms":bank["duracion_ms"],"muestras":samples,"intervalo_maximo_ms":gap,"suministro":load(&root().join("suministro/CONTROL-ARBITRO.json"))?,"adjudicacion":false,"licencia":api::LICENCIA});save(&root().join("RECEPCION-RUST.json"),&v)?;Ok(v)
}
#[cfg(test)]mod tests{use super::*;
 fn p()->Perfil{Perfil{proveedor:"OpenAI".into(),modelo:"gpt-6-astra".into(),endpoint:"https://api.openai.com/v1/responses".into(),presupuesto_ticks:0,exigir_zdr:false,entrada_ticks_por_token:100000,salida_ticks_por_token:500000,cuota_gratuita_tokens:None}}
 #[test]fn contexto_historia_y_cotas(){let b=json!({"input":[{"role":"user","content":json!({"caso":"C01","pregunta":"Q","fragmentos_documentales_completos":[]}).to_string()}],"instructions":"Sólo corpus","tools":[],"tool_choice":"none","store":false});let h=vec!["R0 original".into(),"R1 original".into()];let a=compose(&b,&p(),0,&[]).unwrap();let q=compose(&b,&p(),2,&h).unwrap();let record=parse(q["input"][1]["content"].as_str().unwrap().as_bytes()).unwrap();assert_eq!(record["instrucciones_aplicadas"],a["instructions"]);assert_eq!(q["input"][0],b["input"][0]);assert_eq!(q["input"][2]["content"],h[0]);assert_eq!(q["input"][5]["content"],h[1]);assert_eq!(q["reasoning"]["effort"],"high");assert_eq!(q["max_output_tokens"],16384);assert!(compose(&b,&p(),2,&h[..1]).is_err());}
 #[test]fn nunca_emplea_clave_api_sin_presupuesto(){assert!(api::enviar(&p(),"no es credencial",&json!({}),Path::new("fuera"),1).is_err());let mut other=p();other.proveedor="xAI".into();assert!(api::enviar_chatgpt(&other,"no es credencial",&json!({}),Path::new("fuera"),1).is_err());}
 #[test]fn plazo_no_reinicia_interrupcion(){assert_eq!(remaining(None),300000);let t=Instant::now()-Duration::from_secs(301);assert_eq!(remaining(Some(t)),0);}
}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
