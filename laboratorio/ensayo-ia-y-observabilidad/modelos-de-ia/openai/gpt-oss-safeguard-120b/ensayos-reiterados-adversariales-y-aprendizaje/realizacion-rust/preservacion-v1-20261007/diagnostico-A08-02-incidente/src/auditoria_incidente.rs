use std::{fs,path::Path,collections::BTreeMap};
use serde_json::{Value,json};use sha2::{Sha256,Digest};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn ensure(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn hex(s:&str)->R<Vec<u8>>{ensure(s.len()%2==0,"Hexadecimal incompleto")?;(0..s.len()).step_by(2).map(|i|Ok(u8::from_str_radix(&s[i..i+2],16)?)).collect()}
fn jsonlines(p:&Path)->R<Vec<Value>>{fs::read(p)?.split(|b|*b==b'\n').filter(|b|!b.is_empty()).map(|b|Ok(serde_json::from_slice(b)?)).collect()}
pub fn auditar(p:&Path, interrupted:bool, parcial:bool)->R<Value> {
 let sello=if parcial{"PUNTO.json"}else{"CIERRE.json"};
 let closure:Value=serde_json::from_slice(&fs::read(p.join(sello))?)?;
 if !parcial{ensure(closure["codigo_mcp"]==0&&closure["diario_mcp_conforme"]==true&&interrupted && closure["error"]=="Cota total de una hora alcanzada; conservar sin reintento" && closure["codigo_modelo"]==1 && closure["fin_conductor"]==false,"Custodia sin cierre íntegro")?;}else{ensure(closure["adjudicacion_pendiente"]==true&&closure["inferencia_siguiente_impedida"]==true,"Punto no detenido")?;}
 if !interrupted&&!parcial {ensure(closure["codigo_modelo"]==0&&closure["fin_conductor"]==true,"Cierre no conforme")?;}
 let rows=jsonlines(&p.join("RECEPCION.jsonl"))?;let mut prev=String::new();let mut channels:BTreeMap<String,Vec<Vec<u8>>>=BTreeMap::new();
 for(i,v)in rows.iter().enumerate(){ensure(v["n"]==i&&v["anterior_sha256"]==prev,"Secuencia exterior no conforme")?;let mut bare=v.clone();bare.as_object_mut().unwrap().remove("sha256");bare.sort_all_objects();let sha=h(&serde_json::to_vec(&bare)?);ensure(v["sha256"]==sha,"Huella exterior no conforme")?;prev=sha;let raw=hex(v["bytes_hex"].as_str().ok_or("Sin bytes")?)?;ensure(v["bytes_sha256"]==h(&raw)&&v["datos_utf8"]==String::from_utf8_lossy(&raw).as_ref(),"Bytes exteriores no conformes")?;channels.entry(v["canal"].as_str().unwrap().into()).or_default().push(raw);}
 ensure(closure["ultimo_hash_recepcion"]==prev&&closure["registros"]==rows.len(),"Cierre exterior discordante")?;
 for(ch,file)in [("modelo_stdout","modelo.stdout"),("modelo_stderr","modelo.stderr"),("mcp_stdout","mcp.stdout"),("mcp_stderr","mcp.stderr")]{let all=channels.get(ch).cloned().unwrap_or_default().concat();ensure(all==fs::read(p.join(file))?,"Original y diario exterior discordantes")?;}
 let telemetry=channels.get("telemetria").ok_or("Telemetría ausente")?;let mut all=Vec::new();for b in telemetry{all.extend(b);all.push(b'\n');}ensure(all==fs::read(p.join("TELEMETRIA.jsonl"))?,"Telemetría discordante")?;
 let output=fs::read_to_string(p.join("modelo.stdout"))?;let events=output.lines().filter_map(|s|s.strip_prefix("SV_EVENT ")).map(serde_json::from_str::<Value>).collect::<Result<Vec<_>,_>>()?;
 let requests=events.iter().filter(|v|v["datos"]["evento"]=="mcp_solicitud").map(|v|v["datos"]["wire"].as_str().unwrap().as_bytes().to_vec()).collect::<Vec<_>>();
 let received=events.iter().filter(|v|v["datos"]["evento"]=="mcp_recibido").map(|v|v["datos"]["wire"].as_str().unwrap().as_bytes().to_vec()).collect::<Vec<_>>();
 ensure(Some(&requests)==channels.get("mcp_stdin"),"Frontera conductor a MCP discordante")?;
 ensure(Some(&received)==channels.get("modelo_stdin")&&Some(&received)==channels.get("mcp_stdout"),"Frontera MCP a conductor discordante")?;
 let diary=jsonlines(&p.join("mcp/diario.jsonl"))?;let mut mreq=Vec::new();let mut mrep=Vec::new();let mut deliveries=0;let mut dprev=String::new();
 for(i,v)in diary.iter().enumerate(){ensure(v["secuencia"]==i&&v["anterior_sha256"]==dprev,"Secuencia MCP discordante")?;let mut bare=v.clone();bare.as_object_mut().unwrap().remove("sha256");bare.sort_all_objects();let sha=h(&serde_json::to_vec(&bare)?);ensure(v["sha256"]==sha,"Huella MCP discordante")?;dprev=sha;let d=&v["datos"];match d["evento"].as_str(){Some("solicitud")=>mreq.push(hex(d["bytes_hex"].as_str().unwrap())?),Some("resultado")=>{if let Some(s)=d["bytes_hex"].as_str(){let b=hex(s)?;if !b.is_empty(){ensure(d["sha256"]==h(&b),"Resultado MCP corrupto")?;mrep.push(b);}}},Some("entrega")=>deliveries+=1,_=>{}}}
 ensure(mreq==requests&&mrep==received&&deliveries==requests.len(),"Diario MCP y ambas fronteras discordantes")?;
 let mut count=0;let mut chosen=Vec::<Value>::new();let mut prompt=Value::Null;let mut actual=Vec::<Value>::new();let mut first=true;let mut active=false;let mut engine=false;let mut calcs=0;let mut ends=0;let mut summaries=Vec::new();let mut id=Value::Null;let mut round=Value::Null;
 for v in &events{let d=&v["datos"];match d["evento"].as_str(){
 Some("contexto_conductor")=>{ensure(!active,"Emisiones superpuestas")?;active=true;engine=false;chosen.clear();actual.clear();first=true;prompt=d["tokens"].clone();id=d["id"].clone();round=d["ronda"].clone();ensure(d["tokens_sha256"]==h(&serde_json::to_vec(&prompt)?),"Huella de entrada discordante")?;},
 Some("entrada_motor")=>{ensure(active&&!engine&&d["tokens"]==prompt&&d["truncamiento"]==false,"Frontera conductor motor discordante o truncada")?;engine=true;},
 Some("calculo_entrada")=>{ensure(active&&engine,"Cálculo sin entrada")?;let b=d["tokens"].as_array().ok_or("Tensor ausente")?;ensure(b.len()==1,"Lote distinto de uno")?;let t=b[0].as_array().ok_or("Tensor inválido")?;if first{actual.extend(t.clone());}else{ensure(chosen.last().is_some()&&t.as_slice()==std::slice::from_ref(chosen.last().unwrap()),"Entrada de continuación no coincide con token anterior")?;}calcs+=1;},
 Some("calculo_fin")=>{ensure(d["correcto"]==true,"Cálculo fallido")?;ends+=1;},
 Some("token")=>{ensure(active&&engine,"Token sin solicitud")?;if first{ensure(actual==*prompt.as_array().unwrap(),"Entrada al cálculo discordante")?;first=false;}ensure(d["posicion"]==chosen.len(),"Pérdida o duplicación de tokens")?;chosen.push(d["token"].clone());},
 Some("emision_integra")=>{ensure(active&&d["tokens"]==json!(chosen),"Salida íntegra distinta de los tokens elegidos")?;summaries.push(json!({"id":id,"ronda":round,"entrada_tokens":prompt.as_array().unwrap().len(),"salida_tokens":chosen.len(),"tokens_sha256":h(&serde_json::to_vec(&chosen)?)}));active=false;count+=1;},_=>{}}}
 if !interrupted {ensure(!active&&calcs==ends,"Cálculo o emisión sin cierre")?;}else{ensure(ends<=calcs&&calcs-ends<=1,"Más de un cálculo sin cierre")?;}
 let mut expert_events=0;for v in &events {let d=&v["datos"];if d["evento"]=="expertos_efectivos" {ensure(d["expertos_por_token"]==4&&d["indices"][1]==4&&d["activacion"][1]==4&&d["salida"][1]==4,"Número efectivo de expertos discordante")?;expert_events+=1;}}
 let report=json!({"conforme":true,"alcance":"Integridad encadenada y cotejo exacto de bytes en ambas fronteras MCP; entradas efectivas y todos los tokens elegidos en las emisiones presentes. No adjudica corrección semántica.","registros_exteriores":rows.len(),"registros_mcp":diary.len(),"solicitudes_mcp":requests.len(),"respuestas_mcp":received.len(),"muestras_telemetria":telemetry.len(),"emisiones":count,"calculos":calcs,"detalle_emisiones":summaries,"recepcion_sha256":h(&fs::read(p.join("RECEPCION.jsonl"))?),"diario_mcp_sha256":h(&fs::read(p.join("mcp/diario.jsonl"))?),"sello_archivo":sello,"sello_sha256":h(&fs::read(p.join(sello))?),"cierre_sha256":if parcial{Value::Null}else{json!(h(&fs::read(p.join(sello))?))}});
 let mut report=report;report["recorrido_completo"]=json!(!interrupted&&!parcial);report["punto_previo_adjudicacion"]=json!(parcial);report["conformidad_inferencia"]=if interrupted||count==0{Value::Null}else{json!(true)};report["emision_parcial"]=json!(if active{chosen.len()}else{0});report["calculos_finalizados"]=json!(ends);report["comprobaciones_cuatro_expertos"]=json!(expert_events);
 Ok(report)
}
