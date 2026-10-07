#![forbid(unsafe_code)]
#[path="../catalogo/estricto.rs"] mod estricto;
#[path="../catalogo/recepcion.rs"] mod recepcion;
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,io::Write,path::Path,collections::BTreeSet};
type E=Box<dyn std::error::Error+Send+Sync>;
const ROOT:&str=".";
const FOOTER:&str="© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn read(p:&Path)->Result<Value,E>{Ok(estricto::parse(&fs::read(p)?)?)}
fn put(p:&Path,b:&[u8])->Result<(),E>{fs::create_dir_all(p.parent().ok_or("Carpeta")?)?;let mut f=fs::OpenOptions::new().write(true).create_new(true).open(p)?;f.write_all(b)?;f.sync_all()?;Ok(())}
fn save(p:&Path,v:&Value)->Result<(),E>{put(p,&serde_json::to_vec_pretty(v)?)}
fn originals(root:&Path,dir:&Path,rows:&mut Vec<Value>)->Result<(),E>{
 let mut paths:Vec<_>=fs::read_dir(dir)?.map(|e|e.map(|e|e.path())).collect::<Result<_,_>>()?;paths.sort();
 for p in paths{let m=fs::symlink_metadata(&p)?;if m.file_type().is_symlink(){return Err("Enlace no admitido".into())}if m.is_dir(){originals(root,&p,rows)?}else{let b=fs::read(&p)?;rows.push(json!({"ruta":p.strip_prefix(root)?.to_string_lossy().replace('\\',"/"),"bytes":b.len(),"sha256":hash(&b),"contenido_utf8":String::from_utf8(b)?}));}}Ok(())
}
fn main()->Result<(),E>{
 let root=Path::new(ROOT);let run=root.join("ejecucion/astra-catalogo-20261007");let archive=root.join("publicaciones/usos-gasto-creditos-tokens-sv-20261007/contenido");
 let block=read(&run.join("resultado.json"))?;let adjud=read(&run.join("adjudicacion/CAPA.json"))?;let cases=block["casos"].as_array().ok_or("Casos")?;
 if cases.len()!=9||adjud["medidas"]["terna_completa"]!=true{return Err("Vector incompleto".into())}
 let policy=fs::read_to_string(root.join("ejecucion/astra-catalogo-20261006/servidor-pruebas/control/POLITICA.txt"))?;
 let mut rows=vec![];let mut inputs=0u64;let mut outputs=0u64;let mut total_ms=0u64;let mut total_samples=0u64;let mut max_gap=0u64;let mut total_events=0u64;let mut first_utc=u64::MAX;let mut last_utc=0u64;
 let mut prepared=vec![];
 let mut csv=fs::read_to_string(archive.join("indices/PRUEBAS.csv"))?;if !csv.ends_with('\n'){csv.push('\n');}
 for(i,r)in cases.iter().enumerate(){
  let id=format!("A{:02}",i+1);if r["caso"]!=id||r["completa"]!=true{return Err("Identidad o cierre".into())}
  let dir=run.join(format!("originales/A/{id}/A0"));let request=read(&dir.join("SOLICITUD.json"))?;
  if request["instructions"]!=policy||request["tools"]!=json!([])||request["tool_choice"]!="none"||request["model"]!="gpt-6-astra"||request["store"]!=false{return Err("Condición de frontera distinta".into())}
  let raw=fs::read_to_string(dir.join("SALIDA-SSE.txt"))?;let events:Vec<Value>=raw.lines().filter_map(|l|l.strip_prefix("data: ")).filter(|s|*s!="[DONE]").map(|s|estricto::parse(s.as_bytes())).collect::<Result<_,_>>()?;
  let delivered=recepcion::extract(&events)?;let provider=read(&dir.join("ENTREGA-PROVEEDOR.json"))?;if delivered!=provider{return Err("Entrega no coincide con SSE".into())}
  let audit=read(&dir.join("AUDITORIA-FORMAL.json"))?;let final_bytes=fs::read(dir.join("FINAL.txt"))?;
  if audit["final_sha256"]!=hash(&final_bytes)||audit["solicitud_sha256"]!=hash(&fs::read(dir.join("SOLICITUD.json"))?)||audit["fuente_sse_sha256"]!=hash(raw.as_bytes()){return Err("Custodia discordante".into())}
  let telemetry=dir.join("instrumentacion/telemetria.jsonl");let check=sv_instrumentacion::verify(&telemetry)?;if check!=r["telemetria"]||check["fallos_medicion"]!=0{return Err("Instrumentación discordante".into())}
  let records=sv_instrumentacion::records(&telemetry)?;let samples:Vec<_>=records.iter().filter(|v|v["tipo"]=="muestra").collect();
  let first=samples.first().ok_or("Sin muestras")?;let last=samples.last().unwrap();
  first_utc=first_utc.min(first["utc_unix_ms"].as_u64().ok_or("Fecha")?);last_utc=last_utc.max(last["utc_unix_ms"].as_u64().ok_or("Fecha")?);
  let delta=|k:&str|->Result<u64,E>{Ok(last["datos"]["proceso"][k].as_u64().ok_or("Contador final")?.checked_sub(first["datos"]["proceso"][k].as_u64().ok_or("Contador inicial")?).ok_or("Contador decreciente")?)};
  let cpu_ms=delta("cpu_acumulada_ms")?;let interval=last["transcurrido_ms"].as_u64().unwrap()-first["transcurrido_ms"].as_u64().unwrap();
  let mut ports=BTreeSet::new();let mut tcp_states=BTreeSet::new();let mut udp=0u64;let mut max_tcp=0usize;let mut rss=0u64;
  for s in &samples{rss=rss.max(s["datos"]["proceso"]["rss_bytes"].as_u64().ok_or("RSS")?);let con=s["datos"]["conexiones"].as_array().ok_or("Conexiones")?;max_tcp=max_tcp.max(con.iter().filter(|c|c["protocolo"]=="TCP").count());for c in con{if c["protocolo"]=="TCP"{tcp_states.insert(c["estado"].as_str().ok_or("Estado")?.to_owned());if let Some(n)=c["puerto_remoto"].as_u64(){if n!=0{ports.insert(n);}}}else{udp+=1;}}}
  let prep=read(&dir.join("PREPARACION.json"))?;if prep["paginas"]!=2||prep["mcp_aislamiento"]["red_externa_error"]!=1||prep["mcp_aislamiento"]["red_local_error"]!=1{return Err("Recorrido MCP no conforme".into())}
  let u=&r["uso_proveedor"];let input=u["input_tokens"].as_u64().ok_or("Entrada")?;let output=u["output_tokens"].as_u64().ok_or("Salida")?;let total=u["total_tokens"].as_u64().ok_or("Total")?;
  if input+output!=total{return Err("Tokens discordantes".into())}inputs+=input;outputs+=output;let duration=r["duracion_ms"].as_u64().unwrap();total_ms+=duration;total_samples+=samples.len() as u64;max_gap=max_gap.max(check["intervalo_maximo_ms"].as_u64().unwrap());total_events+=events.len() as u64;
  let observed=json!({"caso":id,"duracion_ms":duration,"primer_texto_ms":r["primer_texto_ms"],"entrada_tokens":input,"salida_tokens":output,"total_tokens":total,"tokens_cache":u["input_tokens_details"]["cached_tokens"],"tokens_razonamiento":u["output_tokens_details"]["reasoning_tokens"],"eventos_sse":events.len(),"bytes_sse":raw.len(),"muestras":samples.len(),"intervalo_maximo_ms":check["intervalo_maximo_ms"],"rss_maximo_bytes":rss,"cpu_delta_ms":cpu_ms,"cpu_media_porcentaje_nucleo_derivada":100.*cpu_ms as f64/interval as f64,"duracion_observada_ms":interval,"io_lectura_delta_bytes":delta("io_lectura_acumulada_bytes")?,"io_escritura_delta_bytes":delta("io_escritura_acumulada_bytes")?,"tcp_maximo_simultaneo":max_tcp,"tcp_estados":tcp_states,"observaciones_udp":udp,"paginas":2,"herramientas":0,"final_sha256":hash(&final_bytes),"sse_sha256":hash(raw.as_bytes()),"telemetria_sha256":check["sha256"],"decision":estricto::parse(&final_bytes)?["decision"],"valor_sv":adjud["casos"][i]["valor"]});
  rows.push(observed.clone());
  let econid=format!("SV-GASTO-20261007-{:03}",i+1);let base=format!("pruebas/2026/10/{econid}");let bundlepath=format!("evidencias/2026/10/astra-catalogo-a0/{id}.json");let mut originals_rows=vec![];originals(&dir,&dir,&mut originals_rows)?;
  let bundle=json!({"caso":id,"alcance":"Originales instrumentales del caso; acceso restringido; no contiene credenciales OAuth","archivos":originals_rows});
  prepared.push((bundlepath.clone(),serde_json::to_vec_pretty(&bundle)?));
  let record=json!({"esquema":"sv.control-gastos.prueba.v1","id":econid,"fecha_registro":"2026-10-07","proveedor":"OpenAI","servicio":"Responses con acceso autorizado mediante ChatGPT","nodo":"03","modelo_solicitado":"gpt-6-astra","modelo_efectivo":"gpt-6-astra","finalidad":format!("Catálogo A0 / {id}; dos páginas artificiales completas"),"referencia_autorizacion":"Inicio autorizado del catálogo el 07/10/2026; S39 r46, TT-0021, Acta 004 §40","referencia_tecnica":"ASTRA-CATALOGO-20261007/A0","intentos":[{"numero":1,"http":200,"terminacion":"response.completed","duracion_ms":duration}],"uso":{"entrada_tokens":input,"entrada_cache_tokens":u["input_tokens_details"]["cached_tokens"],"salida_tokens":output,"razonamiento_tokens":u["output_tokens_details"]["reasoning_tokens"],"total_tokens":total},"economia":{"creditos_cobrados":null,"unidad_creditos":null,"importe":null,"moneda":null,"impuestos":null,"estado_conciliacion":"pendiente","estimacion":null,"fuente_cobro":null},"saldos":{"antes":null,"despues":null,"variacion_atribuible":false,"otros_consumidores":"Uso simultáneo de Codex; saldo global no atribuido al caso"},"evidencias":[bundlepath],"observaciones":["Cuota ordinaria habilitada; créditos adicionales para aplicaciones y recarga automática desactivados. Sin cambio de permisos.","Proveedor no comunica importe o créditos atribuibles. Desconocido no equivale a cero.","Justificación documental en JSON; resumen explicativo adicional solicitado pero no recibido."],"instrumentacion":observed,"autoria_licencia":FOOTER});
  let md=format!("# {econid} · Astra · {id} / A0\n\nUna petición autorizada, finalizada con HTTP 200 y response.completed. Duración de transporte: {} s. Entrada: {input} tokens; salida: {output}; total: {total}. Fuente: uso comunicado por el proveedor, cotejado con los eventos originales.\n\nDos páginas artificiales completas; sin herramientas del candidato. Instrumentación Rust por proceso, TCP/UDP, memoria, CPU acumulada, E/S, eventos, relojes e integridad. El registro JSON conserva las magnitudes y su alcance.\n\nCréditos cobrados e importe atribuible: **no comunicados; conciliación pendiente**. La cuota ordinaria estaba disponible; no se habilitaron créditos adicionales ni recargas. El saldo de la cuenta no se reparte artificialmente entre consumidores.\n\n[Registro específico](REGISTRO.json) · [Originales del caso](../../../../{bundlepath}). El paquete conserva texto y huella de cada archivo para su restitución exacta. No incluye credenciales OAuth.\n\n{FOOTER}\n",duration as f64/1000.);
  prepared.push((format!("{base}/REGISTRO.json"),serde_json::to_vec_pretty(&record)?));prepared.push((format!("{base}/INFORME.md"),md.into_bytes()));
  csv.push_str(&format!("\"{econid}\",\"2026-10-07\",\"2026-10-07\",\"OpenAI\",\"03\",\"gpt-6-astra\",\"1\",\"{input}\",\"{output}\",\"{total}\",\"\",\"\",\"\",\"pendiente\",\"{base}/INFORME.md\"\n"));
 }
 let summary=json!({"esquema":"sv.astra.catalogo-a0.cierre.v1","modelo":"gpt-6-astra","bloque":"A","capa":0,"casos":rows,"totales":{"casos":9,"paginas":18,"entrada_tokens":inputs,"salida_tokens":outputs,"total_tokens":inputs+outputs,"duracion_inferencias_ms":total_ms,"duracion_bloque_con_observacion_ms":block["duracion_bloque_con_observacion_ms"],"duracion_media_inferencia_ms":total_ms as f64/9.,"muestras":total_samples,"eventos_sse":total_events,"intervalo_maximo_ms":max_gap,"fallos_medicion":0,"primera_muestra_utc_unix_ms":first_utc,"ultima_muestra_utc_unix_ms":last_utc},"medidas":adjud["medidas"],"alcance_cpu":"Diferencia del contador acumulado y reloj monotónico. El porcentaje instantáneo de sysinfo se conserva en el original pero no se valida como magnitud calibrada.","limites":["Infraestructura y actividad interna del proveedor no observables","Sin medida de asignaciones individuales Rust, hilos/handles, DNS/TLS desagregados o retransmisiones TCP","Ausencia de herramientas observada no prueba aislamiento de los servidores de OpenAI","Conformidad de A0; no admisión clínica, repetibilidad o recepción independiente"],"creditos_atribuibles":null,"importe_atribuible":null,"autoria_licencia":FOOTER});
 let mut sources=vec![];originals(&run.join("realizacion"),&run.join("realizacion"),&mut sources)?;
 prepared.push(("evidencias/2026/10/astra-catalogo-a0/REALIZACION-EJECUTADA.json".into(),serde_json::to_vec_pretty(&json!({"alcance":"Fuentes preservadas antes de las inferencias","archivos":sources}))?));
 for(p,_)in &prepared{if archive.join(p).exists(){return Err("Expediente existente; no duplicar".into())}}
 save(&run.join("CIERRE-METRICO-RUST.json"),&summary)?;
 for(p,b)in &prepared{put(&archive.join(p),b)?;}fs::write(archive.join("indices/PRUEBAS.csv"),csv.as_bytes())?;
 println!("{}",summary["totales"]);Ok(())
}
