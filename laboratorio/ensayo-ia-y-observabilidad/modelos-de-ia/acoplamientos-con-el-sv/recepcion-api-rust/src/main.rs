#![forbid(unsafe_code)]
//! Recepción posterior, sin credenciales, red ni inferencia. Reutiliza el receptor SSE
//! común y las mediciones recibidas en Astra; la evaluación sustantiva permanece exterior.
use sv_cliente_api::{need,parse,sha,save,guard,Flujo,R,LICENCIA};
use serde_json::{json,Value};
use std::{fs,path::{Path,PathBuf},collections::BTreeSet};
fn load(p:&Path)->R<Value>{guard(p)?;parse(&fs::read(p).map_err(|e|e.to_string())?)}
fn ident(p:&Path)->R<Value>{guard(p)?;let b=fs::read(p).map_err(|e|e.to_string())?;Ok(json!({"bytes":b.len(),"sha256":sha(&b)}))}
fn num(v:&Value)->R<u64>{v.as_u64().ok_or("Entero requerido".into())}
fn metrics(p:&Path,model:&str)->R<Value>{
 let r=load(&p.join("RESULTADO.json"))?;let t=sv_instrumentacion::verify(&p.join("instrumentacion/telemetria.jsonl"))?;need(t==r["telemetria"],"Telemetría alterada")?;
 let raw=fs::read(p.join("SALIDA-SSE.txt")).map_err(|e|e.to_string())?;
 let mut received=Value::Null;
 if r["completa"]==true{
  let mut s=Flujo::default();s.feed(&raw)?;received=s.recibir(model)?;
  let original=load(&p.join("ENTREGA-PROVEEDOR.json"))?;
  for k in ["texto_original","uso_proveedor","respuesta_proveedor","eventos","concordancia_sse_texto"]{need(received[k]==original[k]&&received[k]==r["entrega"][k],"Recepciones discordantes")?;}
  need(received["texto_original"].as_str().ok_or("Texto")?.as_bytes()==fs::read(p.join("FINAL.txt")).map_err(|e|e.to_string())?,"Original final alterado")?;
  need(received["uso_proveedor"]["num_server_side_tools_used"]==0&&received["uso_proveedor"]["num_sources_used"]==0,"Proveedor declara herramienta o fuente externa")?;
  need(load(&p.join("HTTP.json"))?["cabeceras"]["x-zero-data-retention"]=="true"&&original["zdr_confirmado"]==true,"ZDR no acreditado")?;
 }
 let rows=fs::read(p.join("instrumentacion/telemetria.jsonl")).map_err(|e|e.to_string())?.split(|b|*b==b'\n').filter(|l|!l.is_empty()).map(|l|parse(l).and_then(|v|parse(v["cuerpo"].as_str().ok_or("Cuerpo")?.as_bytes()))).collect::<R<Vec<_>>>()?;
 let samples:Vec<_>=rows.iter().filter(|r|r["tipo"]=="muestra").collect();need(!samples.is_empty(),"Sin muestras")?;
 let first=samples[0];let last=samples.last().unwrap();let mut states=BTreeSet::new();let mut tcp_max=0;let mut udp=0;
 for s in &samples{let mut tcp=0;for c in s["datos"]["conexiones"].as_array().ok_or("Conexiones ausentes")?{if c["protocolo"]=="TCP"{tcp+=1;states.insert(c["estado"].as_str().ok_or("Estado")?.to_string());}else if c["protocolo"]=="UDP"{udp+=1;}}tcp_max=tcp_max.max(tcp);}
 let u=&received["uso_proveedor"];if !u.is_null(){need(num(&u["input_tokens"])?+num(&u["output_tokens"])?==num(&u["total_tokens"])?,"Tokens discordantes")?;}
 let mut m=json!({"completa":r["completa"],"telemetria_conforme":r["telemetria_conforme"],"duracion_ms":r["entrega"]["duracion_ms"],"duracion_operacion_ms":r["duracion_operacion_ms"],"primer_texto_ms":r["entrega"]["primer_texto_ms"],"muestras":samples.len(),"intervalo_maximo_ms":t["intervalo_maximo_ms"],"fallos_medicion":t["fallos_medicion"],"rss_maximo_bytes":samples.iter().filter_map(|s|s["datos"]["proceso"]["rss_bytes"].as_u64()).max(),"memoria_virtual_maxima_bytes":samples.iter().filter_map(|s|s["datos"]["proceso"]["virtual_bytes"].as_u64()).max(),"tcp_maximo_simultaneo":tcp_max,"tcp_estados":states,"observaciones_udp":udp,"uso_proveedor":u,"cost_in_usd_ticks":u["cost_in_usd_ticks"],"tokens_entrada":u["input_tokens"],"tokens_salida":u["output_tokens"],"tokens_totales":u["total_tokens"],"tokens_cache":u["input_tokens_details"]["cached_tokens"],"tokens_razonamiento":u["output_tokens_details"]["reasoning_tokens"],"eventos_sse":received["eventos"],"bytes_sse":raw.len(),"sse_sha256":sha(&raw),"telemetria_sha256":t["sha256"],"solicitud":ident(&p.join("SOLICITUD.json"))?,"resultado":ident(&p.join("RESULTADO.json"))?,"alcance":"Medición local del cliente; no recursos internos del proveedor. Contadores de caché y razonamiento incluidos, sin sumarlos dos veces.","autoria_licencia":LICENCIA});
 for(k,out)in[("cpu_acumulada_ms","cpu_delta_ms"),("io_lectura_acumulada_bytes","io_lectura_delta_bytes"),("io_escritura_acumulada_bytes","io_escritura_delta_bytes")]{m[out]=json!(num(&last["datos"]["proceso"][k])?.checked_sub(num(&first["datos"]["proceso"][k])?).ok_or("Contador regresivo")?);}
 if r["completa"]==true{m["final_sha256"]=ident(&p.join("FINAL.txt"))?["sha256"].clone();}Ok(m)
}
fn receive(root:&Path)->R<Value>{
 let bank=load(&root.join("RESULTADO-BANCO.json"))?;let model=bank["modelo"].as_str().ok_or("Modelo")?;
 let mut rows=vec![];
 for h in bank["intentos"].as_array().ok_or("Intentos")?{
  let p=PathBuf::from(h["directorio"].as_str().ok_or("Procedencia")?);guard(&p)?;let mut m=metrics(&p,model)?;let result=load(&p.join("RESULTADO.json"))?;need(result==h["resultado"],"Hito no coincide con original")?;
  m["caso"]=h["caso"].clone();m["etapa"]=h["etapa"].clone();m["intento"]=h["intento"].clone();m["directorio"]=json!(p.to_string_lossy());
  if result["completa"]==true{m["formal_conforme"]=load(&p.join("AUDITORIA-FORMAL.json"))?["conforme"].clone();}
  save(&root.join(format!("mediciones/I{:03}.json",num(&h["intento"])?)),&m)?;rows.push(m);
 }
 let mut totals=json!({});for k in ["tokens_entrada","tokens_salida","tokens_totales","tokens_cache","tokens_razonamiento","cost_in_usd_ticks","muestras","eventos_sse","bytes_sse"]{totals[k]=json!(rows.iter().filter_map(|v|v[k].as_u64()).sum::<u64>());}
 totals["importe_conocido_usd"]=json!(format!("{}.{:010}",num(&totals["cost_in_usd_ticks"])?/10_000_000_000,num(&totals["cost_in_usd_ticks"])?%10_000_000_000));
 for k in ["duracion_operacion_ms","cpu_delta_ms","io_lectura_delta_bytes","io_escritura_delta_bytes","fallos_medicion"]{totals[format!("suma_{k}")]=json!(rows.iter().filter_map(|r|r[k].as_u64()).sum::<u64>());}
 for k in ["intervalo_maximo_ms","rss_maximo_bytes","memoria_virtual_maxima_bytes","tcp_maximo_simultaneo","duracion_operacion_ms","primer_texto_ms"]{totals[format!("max_{k}")]=json!(rows.iter().filter_map(|r|r[k].as_u64()).max());}
 let mut tiempos=rows.iter().filter_map(|r|r["duracion_operacion_ms"].as_u64()).collect::<Vec<_>>();tiempos.sort_unstable();totals["mediana_inferior_operacion_ms"]=json!(tiempos.get(tiempos.len().saturating_sub(1)/2));
 let technical=metrics(&root.join("comprobacion-transporte"),model)?;
 let complete=bank["estado"]=="completo"&&bank["etapas_completas_por_pregunta"]==json!(vec![3;25])&&rows.iter().filter(|r|r["completa"]==true).count()==75;
 let v=json!({"conforme":rows.iter().all(|r|r["telemetria_conforme"]==true),"examen_completo":complete,"estado":bank["estado"],"modelo":model,"casos":rows,"totales":totals,"comprobacion_tecnica":technical,"duracion_banco_ms":bank["duracion_ms"],"reserva_y_coste_acumulados_ticks":bank["coste_o_reserva_acumulados_ticks"],"adjudicacion":"pendiente de revisión sustantiva exterior al candidato","inferencias_nuevas":0,"autoria_licencia":LICENCIA});save(&root.join("METRICAS-RUST.json"),&v)?;Ok(v)
}
fn main(){let a=std::env::args().collect::<Vec<_>>();let result=(||->R<Value>{need(a.len()==2||a.len()==4,"Uso: sv-recepcion-api DIRECTORIO | individual DIRECTORIO MODELO")?;if a.len()==4{need(a[1]=="individual","Operación desconocida")?;let p=PathBuf::from(&a[2]);guard(&p)?;metrics(&p,&a[3])}else{let p=PathBuf::from(&a[1]);guard(&p)?;receive(&p)}})();match result{Ok(v)=>println!("{}",v),Err(e)=>{eprintln!("Recepción impedida: {e}");std::process::exit(1);}}}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
