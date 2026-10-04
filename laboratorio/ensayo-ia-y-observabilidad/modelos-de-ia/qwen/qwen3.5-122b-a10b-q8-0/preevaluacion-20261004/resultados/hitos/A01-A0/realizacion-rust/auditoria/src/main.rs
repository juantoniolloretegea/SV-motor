mod estricto;
mod diarios;
mod contrato;
mod puntuacion;
mod transiciones;
mod custodia;
use serde_json::{Value,json};
use sha2::{Sha256,Digest};
use std::{fs::{self,OpenOptions},io::Write,path::Path};
type E=Box<dyn std::error::Error>;
const FOOTER:&str="© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn read(p:&Path)->Result<Value,E>{Ok(estricto::parse(&fs::read(p)?)?)}
fn save(p:&Path,v:&Value)->Result<(),E>{let mut f=OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(&serde_json::to_vec_pretty(v)?)?;f.sync_all()?;Ok(())}
fn metric(s:&str,k:&str)->Option<f64>{s.lines().find(|l|l.starts_with(&format!("{k} "))).and_then(|l|l.split_whitespace().nth(1)).and_then(|n|n.parse().ok())}
fn audit(root:&Path,out:&Path)->Result<(),E>{
 let result=read(&root.join("RESULTADO.json"))?;
 if root.join("INCIDENCIA.json").exists()||root.join("INCIDENCIA-ENTRADA.json").exists(){return Err("Incidencia presente; requiere revisión separada".into())}
 if result["completa"]!=true||result["done"]!=true||result["terminacion"]!="stop"{return Err("Generación incompleta; no es adjudicable como 0/1/U".into())}
 let preparation=read(&root.join("PREPARACION.json"))?;
 let case=preparation["caso"].as_str().ok_or("Caso")?; let layer=preparation["capa"].as_u64().ok_or("Capa")?;
 let final_bytes=fs::read(root.join("FINAL.txt"))?;
 let thought=fs::read(root.join("RAZONAMIENTO-EMITIDO.txt"))?;
 if result["final_sha256"]!=hash(&final_bytes)||result["razonamiento_sha256"]!=hash(&thought){return Err("Huella de salida discordante".into())}
 if preparation["solicitud_sha256"]!=hash(&fs::read(root.join("SOLICITUD.json"))?)||preparation["entrada_sha256"]!=hash(&fs::read(root.join("ENTRADA-RENDERIZADA.txt"))?)||preparation["plantilla_sha256"]!=hash(&fs::read(root.join("PLANTILLA.jinja"))?){return Err("Preparación alterada".into())}
 let admission=read(&root.join("ADMISION-UNICA.json"))?;
 if admission["caso"]!=case||admission["capa"]!=layer||admission["solicitud_sha256"]!=preparation["solicitud_sha256"]{return Err("Identidad de admisión discordante".into())}
 let ids=read(&root.join("ENTRADA-TOKENS.json"))?; let n=ids.as_array().ok_or("Tokens de entrada")?.len();
 if preparation["entrada_tokens"]!=n||result["usage"]["prompt_tokens"]!=n||read(&root.join("TOKENIZACION-NATIVA-RESPUESTA.json"))?["input_tokens"]!=n{return Err("Entrada efectiva no concordante".into())}
 let raw=fs::read(root.join("SALIDA-SSE.txt"))?; if raw.len()>16*1024*1024{return Err("Salida excede cota".into())}
 let mut events=Vec::new(); let mut done=0; let mut final_rebuilt=String::new(); let mut thought_rebuilt=String::new(); let mut finish=Value::Null; let mut usage=Value::Null;
 for line in std::str::from_utf8(&raw)?.lines(){if let Some(s)=line.strip_prefix("data:").map(str::trim_start){if s=="[DONE]"{done+=1;events.push(json!("done"));continue}let v=estricto::parse(s.as_bytes())?;if v.get("error").is_some(){return Err("Error emitido por servicio".into())}if let Some(t)=v["choices"][0]["delta"]["content"].as_str(){final_rebuilt.push_str(t)}if let Some(t)=v["choices"][0]["delta"]["reasoning_content"].as_str(){thought_rebuilt.push_str(t)}if !v["choices"][0]["finish_reason"].is_null(){finish=v["choices"][0]["finish_reason"].clone()}if !v["usage"].is_null(){usage=v["usage"].clone()}events.push(v)}}
 if done!=1||events.last()!=Some(&json!("done"))||finish!=result["terminacion"]||usage!=result["usage"]||final_rebuilt.as_bytes()!=final_bytes||thought_rebuilt.as_bytes()!=thought{return Err("Canales o cierre distintos del original SSE".into())}
 let mut native_ids=std::collections::BTreeSet::new();
 for e in events.iter().filter(|e|e.is_object()){native_ids.insert(e["id"].as_str().ok_or("Identificador nativo ausente")?.to_owned());let choices=e["choices"].as_array().ok_or("Opciones nativas ausentes")?;if choices.len()>1||choices.iter().any(|c|c["index"]!=0){return Err("Salida de varias secuencias".into())}}
 if native_ids.len()!=1{return Err("Identidad de solicitud nativa no única".into())}
 let mut pos=0;let mut first=None;let mut first_final=None;let mut last=0.0;
 let output_journal=diarios::recorrer(&root.join("SALIDA-OBSERVADA.jsonl"),|d|{let e=&d["evento"];if events.get(pos)!=Some(e){return Err("Emisión y diario no coinciden".into())}let t=d["segundos"].as_f64().ok_or("Tiempo de emisión")?;if t<last{return Err("Tiempo regresivo".into())}last=t;let has_final=e["choices"][0]["delta"]["content"].as_str().is_some_and(|s|!s.is_empty());let has_thought=e["choices"][0]["delta"]["reasoning_content"].as_str().is_some_and(|s|!s.is_empty());if has_final||has_thought{first.get_or_insert(t);}if has_final{first_final.get_or_insert(t);}pos+=1;Ok(())})?;
 if pos!=events.len(){return Err("Eventos sin registro".into())}
 for (observed,key) in [(first,"primer_token_observable_segundos"),(first_final,"primer_contenido_final_segundos")]{match (observed,result[key].as_f64()){(Some(a),Some(b)) if (a-b).abs()<=1.0=>{},(None,None)=>{},_=>return Err("Tiempo de canal discordante".into())}}
 let mcp=read(&root.join("MCP-COTEJO-RUST.json"))?;
 if mcp["estado"]!="conforme"||mcp["diario_sha256"]!=hash(&fs::read(root.join("MCP-DIARIO.jsonl"))?){return Err("Diario MCP no cotejado".into())}
 let exchanges=diarios::recorrer(&root.join("MCP-INTERCAMBIOS.jsonl"),|_|Ok(()))?;
 let initial=fs::read_to_string(root.join("METRICAS-INICIALES.txt"))?; let mut prefill=metric(&initial,"mistralrs_prefill_tokens_processed_total").ok_or("Prefill inicial")?;let mut decode=metric(&initial,"mistralrs_decode_tokens_processed_total").ok_or("Decode inicial")?;
 let mut prefill_end=None; let mut last_time=0.0; let mut max_memory=0u64; let mut max_stall=0.0f64; let mut memory_events_first=String::new();let mut memory_events_last=String::new();let mut max_swap=0u64;
 let telemetry=diarios::recorrer(&root.join("TELEMETRIA.jsonl"),|d|{if d.get("error_telemetria").is_some(){return Err("Pérdida de observabilidad registrada".into())}let time=d["segundos"].as_f64().ok_or("Tiempo de telemetría")?;if time<last_time{return Err("Tiempo regresivo".into())}last_time=time;let m=d["metricas_nativas"].as_str().ok_or("Métricas")?;let p=metric(m,"mistralrs_prefill_tokens_processed_total").ok_or("Prefill")?;let de=metric(m,"mistralrs_decode_tokens_processed_total").ok_or("Decode")?;if p<prefill||de<decode{return Err("Contador regresivo; posible reinicio".into())}if p>prefill{prefill_end.get_or_insert(time);}prefill=p;decode=de;max_memory=max_memory.max(d["memoria"]["memory.current"].as_str().ok_or("Memoria")?.trim().parse()?);max_swap=max_swap.max(d["memoria"]["memory.swap.current"].as_str().ok_or("Intercambio")?.trim().parse()?);max_stall=max_stall.max(d["segundos_sin_progreso_nativo"].as_f64().ok_or("Progreso")?);let ev=d["memoria"]["memory.events"].as_str().ok_or("Eventos memoria")?;if memory_events_first.is_empty(){memory_events_first=ev.into();}memory_events_last=ev.into();Ok(())})?;
 let pages=read(&root.join("PAGINAS.json"))?;let pages=pages.as_array().ok_or("Páginas")?;let mut texts=Vec::new();let doc=format!("D{case}");for (i,p) in pages.iter().enumerate(){if p["pagina"]!=i||p["documento"]!=doc||p["seccion"]!="S1"{return Err("Orden de páginas".into())}texts.push(p["texto"].as_str().ok_or("Texto")?.to_owned());}
 let catalog=read(&root.join("CATALOGO.json"))?;let joined=texts.concat();let source=&catalog["documents"][0];if source["id"]!=doc||source["sections"][0]["id"]!="S1"||source["sections"][0]["text"]!=joined||source["sections"][0]["sha256"]!=hash(joined.as_bytes()){return Err("Páginas distintas de fuente fijada".into())}
 let parsed=estricto::parse(&final_bytes);let (format_ok,format_error)=match &parsed{Ok(v)=>match contrato::check(v,&doc,"S1",&texts,case,layer){Ok(())=>(true,None),Err(e)=>(false,Some(e.to_string()))},Err(e)=>(false,Some(e.to_string()))};
 let report=json!({"encargo":"QWEN35-PRE-20261004/r1","caso":case,"capa":layer,"integridad_conforme":true,"fuente_resultado_sha256":hash(&fs::read(root.join("RESULTADO.json"))?),"final_sha256":hash(&final_bytes),"razonamiento_sha256":hash(&thought),"salida_sse_sha256":hash(&raw),"diarios":{"salida":output_journal,"mcp_intercambios":exchanges,"telemetria":telemetry},"contrato_formal_conforme":format_ok,"defecto_formal":format_error,"decision_observada":parsed.as_ref().ok().map(|v|v["decision"].clone()),"adjudicacion_sustantiva":"Pendiente de lectura y contraste externo; no se infiere por coincidencia de etiqueta","tiempos":{"segundos_totales":result["segundos"],"fin_entrada_observado_segundos":prefill_end,"primer_token_emitido_segundos":first,"primer_contenido_final_segundos":first_final,"cierre_emitido_segundos":last,"resolucion_telemetria_nominal_segundos":5,"limite_observacion_entrada":"Intervalo hasta actualización del contador nativo; no temporización interna exacta"},"usage_nativo":usage,"memoria":{"maximo_muestreado":max_memory,"intercambio_maximo_muestreado":max_swap,"eventos_iniciales":memory_events_first,"eventos_finales":memory_events_last,"mayor_intervalo_sin_progreso_observado":max_stall}});
 save(out,&report)?;println!("{}",json!({"integridad_conforme":true,"contrato_formal_conforme":format_ok,"caso":case,"capa":layer}));Ok(())
}
fn main()->Result<(),E>{let a:Vec<_>=std::env::args().collect();match a.get(1).map(String::as_str){Some("auditar")=>audit(Path::new(a.get(2).ok_or("Origen")?),Path::new(a.get(3).ok_or("Informe")?)),Some("capa")=>puntuacion::capa(Path::new(a.get(2).ok_or("Adjudicaciones")?),Path::new(a.get(3).ok_or("Destino")?)),Some("comparar")=>transiciones::run(Path::new(a.get(2).ok_or("Capa anterior")?),Path::new(a.get(3).ok_or("Capa posterior")?),Path::new(a.get(4).ok_or("Informe")?)),Some("cotejar")=>custodia::run(Path::new(a.get(2).ok_or("Manifiesto")?),Path::new(a.get(3).ok_or("Recuperación")?),Path::new(a.get(4).ok_or("Informe")?)),_=>Err("Uso: auditar ORIGINALES INFORME | capa ADJUDICACIONES DESTINO | comparar ANTERIOR POSTERIOR INFORME | cotejar MANIFIESTO RAIZ INFORME".into())}}
