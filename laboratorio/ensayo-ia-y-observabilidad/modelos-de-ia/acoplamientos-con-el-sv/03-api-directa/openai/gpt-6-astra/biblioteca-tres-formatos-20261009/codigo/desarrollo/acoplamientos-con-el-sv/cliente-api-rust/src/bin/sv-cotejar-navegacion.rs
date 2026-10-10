//! Cotejo posterior sin red: reconstruye las entregas y suma contadores originales.
use serde_json::{json, Value};
use std::{fs, path::Path};
use sv_cliente_api::{self as api, need, parse, sha, R};
fn read(p: &Path) -> R<Vec<u8>> { api::guard(p)?; fs::read(p).map_err(|e| e.to_string()) }
fn load(p: &Path) -> R<Value> { parse(&read(p)?) }
fn n(v: &Value) -> R<u64> { v.as_u64().ok_or("Contador ausente".into()) }
fn run() -> R<()> {
 let args: Vec<_> = std::env::args().collect();
 need(args.len()==3,"Uso: sv-cotejar-navegacion DIRECTORIO RESULTADO")?;
 let root=Path::new(&args[1]); let work=Path::new("C:/SV-LABORATORIO");
 let admission=load(&root.join("ADMISION-NAVEGACION.json"))?;
 for f in admission["archivos"].as_array().ok_or("Sin admisión")? {
  need(sha(&read(&work.join(f["ruta"].as_str().ok_or("Sin ruta")?))?)==f["sha256"],"Fuente o ejecutable modificado")?;
 }
 let result=load(&root.join("resultado.json"))?;
 need(result["estado"]=="entrega_final_recibida","Sin entrega final")?;
 let count=result["turnos"].as_array().ok_or("Sin turnos")?.len();
 need(count>0&&count<=n(&admission["maximo_turnos"])? as usize,"Cantidad de turnos ajena")?;
 need(fs::read_dir(root.join("turnos")).map_err(|e|e.to_string())?.count()==count,"Turnos adicionales no incluidos")?;
 let catalog=load(&root.join("preparado/CATALOGO.json"))?;
 let mut seen=Vec::new(); let mut rows=Vec::new(); let mut old=Vec::new();
 let (mut input,mut output,mut cache,mut reasoning,mut duration,mut samples,mut events)=(0u64,0u64,0u64,0u64,0u64,0u64,0u64);
 let (mut start,mut end)=(u64::MAX,0u64); let mut sources=Vec::new();
 for i in 0..count {
  let dir=root.join(format!("consultas/lectura-{i:02}")); let v=load(&dir.join("LECTURA.json"))?; let receipt=load(&dir.join("RECEPCION.json"))?;
  need(receipt["conforme"]==true&&receipt["reconstruccion_identica"]==true&&receipt["muestra_receptor"]["sockets_propios"]==0,"Recepción MCP incompleta")?;
  need(receipt["catalogo_sha256"]==sha(&read(&root.join("preparado/CATALOGO.json"))?),"Catálogo MCP discordante")?;
  need(sha(v["texto"].as_str().ok_or("Sin texto")?.as_bytes())==v["sha256"],"Sección alterada")?;
  need(catalog["documents"].as_array().ok_or("Sin catálogo")?.iter().any(|d|d["id"]==v["documento"]&&d["sections"].as_array().is_some_and(|ss|ss.iter().any(|s|s["id"]==v["seccion"]))),"Lectura ajena al catálogo")?;
  need(receipt["secciones"][0]["sha256"]==v["sha256"],"Huella MCP discordante")?;
  seen.push(v);
 }
 for t in 1..=count {
  let dir=root.join(format!("turnos/{t:02}")); let q=load(&dir.join("SOLICITUD.json"))?;
  need(q["model"]=="gpt-6-astra"&&q["tools"]==json!([])&&q["tool_choice"]=="none"&&q["store"]==false,"Herramientas o modelo ajenos")?;
  let hist=q["input"].as_array().ok_or("Sin historial")?;
  if t==1 { need(hist.len()==1,"Inicio ajeno")?; let initial=parse(hist[0]["content"].as_str().ok_or("Sin contenido")?.as_bytes())?; need(initial["indice_recibido_desde_mcp"]==seen[0],"Índice inicial alterado")?; }
  else {need(hist==&old,"Historial no corresponde a entregas anteriores")?;}
  let raw=read(&dir.join("SALIDA-SSE.txt"))?; let mut flow=api::Flujo::default();
  for chunk in raw.chunks(4093) {flow.feed(chunk)?;}
  need(json!(flow.eventos)==load(&dir.join("EVENTOS.json"))?,"Eventos no reconstruibles")?;
  let replay=flow.recibir("gpt-6-astra")?; let delivery=load(&dir.join("ENTREGA-PROVEEDOR.json"))?;
  for key in ["texto_original","uso_proveedor","respuesta_proveedor"] {need(replay[key]==delivery[key],"Entrega no coincide con SSE")?;}
  let text=std::str::from_utf8(&read(&dir.join("FINAL.txt"))?).map_err(|e|e.to_string())?.to_owned();
  need(delivery["texto_original"]==text,"Texto final alterado")?;
  let r=load(&dir.join("RESULTADO.json"))?; let usage=load(&root.join(format!("consumos/turno-{t:02}.json")))?;
  need(r["completa"]==true&&r["telemetria_conforme"]==true&&r["entrega"]==delivery,"Resultado no completo")?;
  need(usage["uso_proveedor"]==delivery["uso_proveedor"]&&usage["duracion_ms"]==r["duracion_operacion_ms"]&&usage==result["turnos"][t-1],"Consumo discordante")?;
  need(load(&dir.join("HTTP.json"))?["status"]==200,"HTTP no conforme")?;
  let journal=dir.join("instrumentacion/telemetria.jsonl"); let check=sv_instrumentacion::verify(&journal)?;
  need(check==r["telemetria"]&&check==load(&dir.join("instrumentacion/COTEJO-TELEMETRIA.json"))?&&check["fallos_medicion"]==0&&n(&check["intervalo_maximo_ms"])?<=750,"Instrumentación no conforme")?;
  let records=sv_instrumentacion::records(&journal)?;
  start=start.min(n(&records.first().ok_or("Sin registros")?["utc_unix_ms"])?); end=end.max(n(&records.last().unwrap()["utc_unix_ms"])?);
  let u=&delivery["uso_proveedor"]; let a=n(&u["input_tokens"])?; let b=n(&u["output_tokens"])?;
  need(a+b==n(&u["total_tokens"])?,"Total de tokens discordante")?;
  let c=n(&u["input_tokens_details"]["cached_tokens"])?; let d=n(&u["output_tokens_details"]["reasoning_tokens"])?;
  input+=a;output+=b;cache+=c;reasoning+=d;duration+=n(&usage["duracion_ms"])?; samples+=n(&check["muestras"])?; events+=n(&check["registros"])?;
  let answer=parse(text.as_bytes())?;
  old=hist.clone();old.push(json!({"role":"assistant","content":text}));
  if t<count {need(answer["accion"]=="leer"&&answer["documento"]==seen[t]["documento"]&&answer["seccion"]==seen[t]["seccion"],"Selección discordante")?;
   let dec=load(&dir.join("DECISION-CONTROLADOR.json"))?;need(dec["lectura_recibida"]==seen[t],"Sección entregada discordante")?;old.push(json!({"role":"user","content":dec.to_string()}));
  } else {need(answer==load(&root.join("RESPUESTA-FINAL.json"))?&&answer["accion"]=="final","Entrega final discordante")?;}
  rows.push(json!({"turno":t,"entrada_tokens":a,"salida_tokens":b,"total_tokens":a+b,"entrada_cache_tokens":c,"razonamiento_tokens":d,"duracion_operacion_ms":usage["duracion_ms"],"primer_texto_ms":delivery["primer_texto_ms"],"muestras":check["muestras"],"registros":check["registros"],"intervalo_maximo_ms":check["intervalo_maximo_ms"],"sha256_telemetria":check["sha256"],"sse_reconstruido":true,"importe":null}));
  for f in ["SOLICITUD.json","SALIDA-SSE.txt","FINAL.txt","ENTREGA-PROVEEDOR.json","instrumentacion/telemetria.jsonl"] {let bytes=read(&dir.join(f))?;sources.push(json!({"ruta":format!("turnos/{t:02}/{f}"),"bytes":bytes.len(),"sha256":sha(&bytes)}));}
 }
 let final_v=load(&root.join("RESPUESTA-FINAL.json"))?; let answers=final_v["respuestas"].as_array().ok_or("Sin respuestas")?;
 need(answers.len()==3,"Número de respuestas")?; let mut quotes=0;
 for id in ["HTML01","PDF01","MD01"] {let aa:Vec<_>=answers.iter().filter(|a|a["documento"]==id).collect();need(aa.len()==1&&aa[0]["estado"]=="respondida","Documento ausente o indeterminado")?;
  let ee=aa[0]["evidencias"].as_array().ok_or("Sin evidencia")?;need(!ee.is_empty(),"Citas ausentes")?;
  for e in ee {let quote=e["cita"].as_str().filter(|s|s.chars().count()>=12).ok_or("Cita insuficiente")?;need(seen.iter().any(|s|s["documento"]==id&&s["seccion"]==e["seccion"]&&s["texto"].as_str().is_some_and(|s|s.contains(quote))),"Cita no suministrada")?;quotes+=1;}
 }
 let indices=seen.iter().filter(|v|v["documento"]=="BIBLIOTECA_INDICE").count();
 let out=json!({"version":1,"conforme":true,"modelo":"gpt-6-astra","nodo":3,"turnos":rows,"entrada_tokens":input,"salida_tokens":output,"total_tokens":input+output,"entrada_cache_tokens":cache,"razonamiento_tokens":reasoning,"duracion_operaciones_ms":duration,"inicio_observacion_utc_ms":start,"fin_observacion_utc_ms":end,"intervalo_exterior_ms":end-start,"muestras":samples,"registros":events,"fallos_medicion":0,"lecturas_mcp":seen.len(),"indices":indices,"secciones_contenido":seen.len()-indices,"citas_literales":quotes,"fuentes":sources,"coste_liquidado":null,"cuota_plan_consumida":null,"limite":"Cotejo posterior local. No recepción independiente ni observación de procesos internos del proveedor. La suma de duraciones no sustituye al intervalo exterior.","licencia":api::LICENCIA});
 api::save(Path::new(&args[2]),&out)?;
 println!("Conforme: {count} turnos; {input} entrada + {output} salida = {} tokens; {samples} muestras; {duration} ms acumulados; {quotes} citas",input+output);Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1);}}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
