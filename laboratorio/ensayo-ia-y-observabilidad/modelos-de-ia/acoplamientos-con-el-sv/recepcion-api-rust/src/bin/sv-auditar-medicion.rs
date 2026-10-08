//! Auditoría de cobertura de medición; no ejecuta inferencia ni modifica originales.
#![forbid(unsafe_code)]
use std::{fs,path::{Path,PathBuf}};
use serde_json::{json,Value};
use sv_cliente_api::{need,parse,sha,R};
fn load(p:&Path)->R<Value>{sv_cliente_api::guard(p)?;parse(&fs::read(p).map_err(|e|e.to_string())?)}
fn usd(v:u64)->String{format!("{}.{:010}",v/10_000_000_000,v%10_000_000_000)}
fn run(root:&Path,out:&Path)->R<Value>{
 sv_cliente_api::guard(root)?;sv_cliente_api::guard(out)?;
 let mut paths=fs::read_dir(root.join("hitos")).map_err(|e|e.to_string())?.map(|e|e.map(|e|e.path())).collect::<Result<Vec<_>,_>>().map_err(|e|e.to_string())?;paths.sort();
 let(mut input,mut output,mut known,mut samples,mut maxgap,mut cached,mut reason,mut write)=(0u64,0u64,0u64,0u64,0u64,0u64,0u64,0u64);
 let(mut upper,mut estimated,mut miss_usage,mut miss_cache,mut miss_reason,mut miss_write)=(0u64,0u64,0usize,0usize,0usize,0usize);let mut rows=vec![];
 for path in paths{
  let h=load(&path)?;let dir=root.join(h["directorio"].as_str().ok_or("Directorio")?);let r=load(&dir.join("RESULTADO.json"))?;
  let raw=fs::read(dir.join("RESULTADO.json")).map_err(|e|e.to_string())?;need(sha(&raw)==h["resultado"]["sha256"],"Resultado cambiado")?;
  let tel=dir.join("instrumentacion/telemetria.jsonl");let t=sv_instrumentacion::verify(&tel)?;need(t==r["telemetria"]&&t["fallos_medicion"]==0,"Instrumentación discordante o con fallos")?;
  let records=sv_instrumentacion::records(&tel)?;let ss=records.iter().filter(|v|v["tipo"]=="muestra").collect::<Vec<_>>();need(!ss.is_empty(),"Sin muestras")?;
  let pid=ss[0]["datos"]["proceso"]["pid"].as_u64().ok_or("PID")?;
  need(ss.iter().all(|v|v["datos"]["proceso"]["pid"]==pid),"Cambio de proceso")?;
  let peak=|field:&str|ss.iter().filter_map(|v|v["datos"]["proceso"][field].as_u64()).max();
  let delta=|field:&str|->R<u64>{let a=ss[0]["datos"]["proceso"][field].as_u64().ok_or("Contador inicial")?;let b=ss.last().unwrap()["datos"]["proceso"][field].as_u64().ok_or("Contador final")?;b.checked_sub(a).ok_or("Contador regresivo".into())};
  let sockets=ss.iter().flat_map(|v|v["datos"]["conexiones"].as_array().unwrap().iter()).collect::<Vec<_>>();
  let observed_https=sockets.iter().any(|v|v["protocolo"]=="TCP"&&v["puerto_remoto"]==443&&v["estado"]=="Established");
  let first_text=records.iter().find(|v|v["tipo"]=="evento_sse"&&v["datos"]["tipo"]=="response.output_text.delta").map(|v|v["datos"]["ms"].clone());
  let first_event=records.iter().find(|v|v["tipo"]=="evento_sse").map(|v|v["datos"]["ms"].clone());
  let receive_bytes=records.iter().filter(|v|v["tipo"]=="lectura_https").filter_map(|v|v["datos"]["acumulados"].as_u64()).max();
  let events=records.iter().filter(|v|v["tipo"]=="evento_sse").count();
  let raw_sse=fs::read(dir.join("SALIDA-SSE.txt")).map_err(|e|e.to_string())?;need(receive_bytes==Some(raw_sse.len() as u64),"Bytes recibidos y originales discordantes")?;
  let s= t["muestras"].as_u64().ok_or("Muestras")?;samples+=s;let gap=t["intervalo_maximo_ms"].as_u64().ok_or("Intervalo")?;maxgap=maxgap.max(gap);need(gap<=750,"Intervalo fuera de cota")?;
  let est=&h["estimacion_tarifaria"];let n=&est["uso_normalizado"];
  if let(Some(a),Some(b),Some(c))=(n["input_tokens"].as_u64(),n["output_tokens"].as_u64(),n["total_tokens"].as_u64()){need(a.checked_add(b)==Some(c),"Suma de tokens")?;input+=a;output+=b;known+=c;}else{miss_usage+=1;}
  match n["cached_tokens"].as_u64(){Some(v)=>cached+=v,None=>miss_cache+=1};match n["reasoning_tokens"].as_u64(){Some(v)=>reason+=v,None=>miss_reason+=1};match est["cache_write_tokens"].as_u64(){Some(v)=>write+=v,None=>miss_write+=1};
  if let Some(v)=est["estimacion_sin_descuento_ticks"].as_u64(){upper+=v;estimated+=est["estimacion_con_cache_ticks"].as_u64().unwrap_or(v);}
  let sent=load(&dir.join("ENVIO.json"))?;let http=load(&dir.join("HTTP.json"))?;
  rows.push(json!({"caso":h["caso"],"etapa":h["etapa"],"intento":h["intento"],"directorio":h["directorio"],"completa":h["completa"],"utc_envio_ms":sent["utc_ms"],"pid":pid,"cpu_acumulada_ventana_ms":delta("cpu_acumulada_ms")?,"rss_max_bytes":peak("rss_bytes"),"virtual_max_bytes":peak("virtual_bytes"),"io_lectura_ventana_bytes":delta("io_lectura_acumulada_bytes")?,"io_escritura_ventana_bytes":delta("io_escritura_acumulada_bytes")?,"tcp_https_observado":observed_https,"observaciones_sockets":sockets.len(),"http":http["status"],"cabeceras_ms":http["cabeceras_ms"],"duracion_operacion_ms":r["duracion_operacion_ms"],"primer_evento_ms_derivado":first_event,"primer_texto_ms_derivado":first_text,"eventos_sse":events,"bytes_sse":raw_sse.len(),"uso":n,"estimacion_tarifaria":est,"telemetria":t,"solicitud_sha256":sha(&fs::read(dir.join("SOLICITUD.json")).map_err(|e|e.to_string())?),"sse_sha256":sha(&raw_sse),"final_sha256":h["texto"]["sha256"],"recuperacion_local_sin_inferencia":r["recuperacion_local_sin_inferencia"]}));
 }
 let result=json!({"conforme":true,"fecha_auditoria_utc_ms":sv_instrumentacion::utc_ms(),"alcance":"Corte de los intentos ya cerrados; no declara completada la campaña ni mide infraestructura interna del proveedor","solicitudes":rows.len(),"tokens_entrada":input,"tokens_salida":output,"tokens_totales_conocidos":known,"tokens_cache_conocidos_incluidos":cached,"tokens_razonamiento_conocidos_incluidos":reason,"tokens_escritura_cache_conocidos_incluidos":write,"intentos_sin_uso":miss_usage,"intentos_sin_cache_comunicada":miss_cache,"intentos_sin_razonamiento_comunicado":miss_reason,"intentos_sin_escritura_cache_comunicada":miss_write,"estimacion_sin_descuento_usd":usd(upper),"estimacion_disponible_usd":usd(estimated),"importe_liquidado_usd":null,"muestras":samples,"intervalo_maximo_ms":maxgap,"casos":rows,"no_medido":["recursos internos del proveedor","hilos y handles del sistema operativo","tareas internas del proveedor","asignaciones del heap Rust","descomposición DNS/TLS","RTT y retransmisiones TCP"],"tareas_observadas":"Identidad de pregunta y etapa R0/R1/R2, preparación MCP, envío, recepción, auditoría e hitos locales; no procesos internos del candidato","licencia":sv_cliente_api::LICENCIA});sv_cliente_api::save(out,&result)?;Ok(result)
}
fn main(){let r=(||->R<Value>{let args=std::env::args().collect::<Vec<_>>();need(args.len()==3,"RAIZ SALIDA")?;run(&PathBuf::from(&args[1]),&PathBuf::from(&args[2]))})();match r{Ok(v)=>println!("Auditoría conforme: {} solicitudes, {} muestras, {} tokens; coste estimado {} USD",v["solicitudes"],v["muestras"],v["tokens_totales_conocidos"],v["estimacion_disponible_usd"]),Err(e)=>{eprintln!("IMPEDIMENTO: {e}");std::process::exit(1)}}}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
