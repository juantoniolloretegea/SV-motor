#![forbid(unsafe_code)]
use sv_claude_kaggle::{self as sv,need,parse,sha,R};
use serde_json::{json,Value};
use std::{fs,path::Path,io::Write};
fn bytes(d:&Path,n:&str)->R<Vec<u8>>{fs::read(d.join(n)).map_err(|e|e.to_string())}
fn read(d:&Path,n:&str)->R<Value>{parse(&bytes(d,n)?)}
fn sum(v:&Value,k:&str)->R<u64>{v[k].as_u64().ok_or(format!("Ausente: {k}"))}
fn run()->R<()>{
 let args=std::env::args().collect::<Vec<_>>();need(args.len()==4,"Uso: auditar-corte PAQUETE CORTE INFORME")?;
 let pbytes=fs::read(&args[1]).map_err(|e|e.to_string())?;need(sha(&pbytes)=="758b9bac230b81410df73795c7c74ae3089987bfb8aecc7d53917c50896f749d","Paquete diferente")?;
 let p=parse(&pbytes)?;let root=Path::new(&args[2]);let start=read(root,"INICIO.json")?;let close=read(root,"RESULTADO-CAMPANA.json")?;
 need(start["binario_sha256"]=="1442aadea3506b0e2bec474f7d0bcd7851418bf4291ac99bb35986b7d95a2806"&&start["paquete_sha256"]==sha(&pbytes),"Origen de ejecución discordante")?;
 need(close["estado"]=="cuota_reservada_insuficiente","Estado de corte diferente")?;
 let delivered=close["entregas"].as_array().ok_or("Entregas ausentes")?;need(delivered.len()==31,"Corte esperado de 31 entregas")?;
 let(mut cost,mut input,mut output,mut reasoning,mut samples,mut maxgap)=(0u64,0u64,0u64,0u64,0u64,0u64);let mut rows=vec![];let mut count=0;
 'banks:for bank in p["bancos"].as_array().ok_or("Bancos ausentes")?{for case in bank["casos"].as_array().ok_or("Casos ausentes")?{let id=case["id"].as_str().ok_or("Identidad ausente")?;let mut history=vec![];for stage in 0..3{
 if count==delivered.len(){break 'banks;}let row=&delivered[count];need(row["caso"]==id&&row["etapa"]==stage,"Orden discordante")?;
 let d=root.join(format!("{}/{id}-R{stage}",bank["id"].as_str().ok_or("Banco")?));let(q,w)=sv::compose(&case["base"],stage,&history)?;
 need(read(&d,"SOLICITUD-DOCUMENTAL.json")?==q&&read(&d,"SOLICITUD.json")?==w,"Solicitud real distinta de composición fijada")?;
 let result=read(&d,"RESULTADO.json")?;need(result==row["resultado"]&&read(root,&format!("HITO-{:03}.json",count+1))?==*row,"Hito o resultado discordante")?;
 need(result["completa"]==true&&result["error"].is_null()&&read(&d,"HTTP.json")?["status"]==200,"Recepción incompleta")?;
 let raw=bytes(&d,"RESPUESTA-HTTP.json")?;let b=parse(&raw)?;need(b["model"]==sv::MODELO,"Modelo diferente")?;
 let choices=b["choices"].as_array().ok_or("Choices")?;need(choices.len()==1&&choices[0]["index"]==0&&choices[0]["finish_reason"]=="stop","Generación incompleta o múltiple")?;
 let m=&choices[0]["message"];need(m["role"]=="assistant"&&(m["refusal"].is_null()||m["refusal"]=="")&&(m["tool_calls"].is_null()||m["tool_calls"]==json!([]))&&m["function_call"].is_null(),"Frontera de herramientas o negativa discordante")?;
 let content=m["content"].as_str().ok_or("Sin contenido")?;need(bytes(&d,"CONTENIDO-ORIGINAL.txt")?==content.as_bytes(),"Contenido alterado")?;
 let seg=read(&d,"SEGMENTACION.json")?;let offset=sum(&seg,"inicio_final_bytes")? as usize;need(offset<=content.len()&&content.is_char_boundary(offset)&&seg["contenido_sha256"]==sha(content.as_bytes())&&seg["reconstruccion_conforme"]==true,"Segmentación inválida")?;
 let mut emitted=String::new();for s in seg["segmentos"].as_array().ok_or("Segmentos")?{let a=sum(s,"contenido_inicio_bytes")? as usize;let z=sum(s,"contenido_fin_bytes")? as usize;need(a<=z&&z<=content.len()&&content.is_char_boundary(a)&&content.is_char_boundary(z),"Canal inválido")?;emitted.push_str(&content[a..z]);}
 need(bytes(&d,"RAZONAMIENTO-THINK.txt")?==emitted.as_bytes()&&bytes(&d,"FINAL.txt")?==content[offset..].as_bytes(),"Separación alterada")?;
 need(read(&d,"RAZONAMIENTO-CANALES.json")?==json!({"reasoning_content":m["reasoning_content"],"reasoning":m["reasoning"]}),"Canal perdido")?;
 let final_text=&content[offset..];let formal=parse(final_text.as_bytes()).and_then(|a|sv::formal(&a,&q)).unwrap_or_else(|e|json!({"conforme":false,"defecto":e}));need(formal==result["auditoria_formal"],"Comprobación formal discordante")?;
 let u=&b["usage"];need(result["uso_proveedor"]==*u,"Uso perdido")?;let i=sum(u,"prompt_tokens")?;let o=sum(u,"completion_tokens")?;need(i.checked_add(o)==u["total_tokens"].as_u64(),"Tokens discordantes")?;let r=sum(&u["completion_tokens_details"],"reasoning_tokens")?;need(r<=o,"Razón fuera de salida")?;
 let c=sum(&u["cost"],"input_tokens_cost_nanodollars")?.checked_add(sum(&u["cost"],"output_tokens_cost_nanodollars")?).ok_or("Coste desbordado")?;need(result["coste_nanodolares"]==c,"Coste discordante")?;
 let reserve=read(&d,"RESERVA.json")?;need(reserve["coste_anterior_nanodolares"]==cost&&reserve["limite_nanodolares"]==9_383_976_000u64&&c<=sum(&reserve,"reserva_nanodolares")?,"Reserva discordante")?;
 let tel=sv::instrumentacion::verify(&d.join("instrumentacion/telemetria.jsonl"))?;need(tel==result["telemetria"]&&tel==read(&d,"instrumentacion/COTEJO-TELEMETRIA.json")?&&tel["fallos_medicion"]==0&&sum(&tel,"intervalo_maximo_ms")?<=750,"Telemetría no conforme")?;
 let records=sv::instrumentacion::records(&d.join("instrumentacion/telemetria.jsonl"))?;
 need(records.iter().any(|v|v["tipo"]=="recepcion"&&v["datos"]["respuesta_http_sha256"]==sha(&raw)&&v["datos"]["uso"]==*u),"Recepción sin correlación")?;
 let wire_sha=sha(&serde_json::to_vec(&w).map_err(|e|e.to_string())?);need(records.iter().any(|v|v["tipo"]=="envio"&&v["datos"]["solicitud_sha256"]==wire_sha),"Petición sin correlación")?;
 let measurements=records.iter().filter(|v|v["tipo"]=="muestra").collect::<Vec<_>>();let pid=measurements.first().ok_or("Muestras ausentes")?["datos"]["proceso"]["pid"].clone();need(measurements.iter().all(|v|v["datos"]["proceso"]["pid"]==pid),"PID cambió")?;
 let recovered=count==0;if recovered{need(sha(&raw)=="1a7752ea04e53b7d2676478a1395505d4c895b24ddde68740f14ff11745daa75"&&result["recuperacion_instrumental_sin_inferencia"]==true&&read(&d,"RECUPERACION-RUST.json")?["solicitudes_de_modelo"]==0,"Recuperación distinta")?;}else{need(pid==105&&measurements.iter().all(|v|v["datos"]["proceso"]["hilos_so"].as_u64().is_some_and(|n|n>0)&&v["datos"]["proceso"]["descriptores_abiertos_linux"].as_u64().is_some_and(|n|n>0)&&v["datos"]["proceso"]["espacio_red_linux"].as_str().is_some_and(|s|s.starts_with("net:["))),"Medidas adicionales no recibidas")?;}
 cost+=c;input+=i;output+=o;reasoning+=r;samples+=sum(&tel,"muestras")?;maxgap=maxgap.max(sum(&tel,"intervalo_maximo_ms")?);
 rows.push(json!({"banco":bank["id"],"caso":id,"etapa":stage,"solicitud_documental_sha256":sha(&bytes(&d,"SOLICITUD-DOCUMENTAL.json")?),"solicitud_sha256":sha(&bytes(&d,"SOLICITUD.json")?),"respuesta_http_sha256":sha(&raw),"final_sha256":sha(final_text.as_bytes()),"telemetria":tel,"pid":pid,"recuperado_sin_repetir":recovered,"formal":formal,"tokens_entrada":i,"tokens_salida":o,"tokens_razonamiento_incluidos_salida":r,"coste_nanodolares":c,"duracion_ms":result["duracion_ms"],"latencia_proveedor_ms":u["total_backend_latency_ms"],"razonamiento_emitido_caracteres":emitted.chars().count(),"canales_razonamiento":read(&d,"RAZONAMIENTO-CANALES.json")?}));history.push(final_text.to_owned());count+=1;
 }}}
 need(cost==8_698_073_000u64&&close["coste_conocido_nanodolares"]==cost&&close["costes_no_recibidos"]==0,"Agregado discordante")?;
 let report=json!({"conforme":true,"alcance":"Custodia, composición, transporte, formato y medidas; juicio semántico y recepción independiente separados","modelo":sv::MODELO,"entregas":rows,"numero_entregas":count,"numero_generaciones_en_continuacion":30,"original_r0_reutilizado":1,"pendientes":44,"coste_conocido_nanodolares":cost,"reserva_http400_coste_no_comunicado_nanodolares":606024000,"tokens_entrada":input,"tokens_salida":output,"tokens_total":input+output,"tokens_razonamiento_incluidos_salida":reasoning,"muestras":samples,"intervalo_maximo_ms":maxgap,"primer_token_ms":null,"cache_tokens":null,"cpu_gpu_memoria_internos_proveedor":null,"recepcion_independiente":"pendiente","licencia":sv::LICENCIA});
 let mut f=fs::OpenOptions::new().write(true).create_new(true).open(&args[3]).map_err(|e|e.to_string())?;f.write_all(&serde_json::to_vec_pretty(&report).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;println!("Cotejo Rust conforme: {count} entregas, coste {cost} nanodólares");Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("NO CONFORME: {e}");std::process::exit(1)}}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
