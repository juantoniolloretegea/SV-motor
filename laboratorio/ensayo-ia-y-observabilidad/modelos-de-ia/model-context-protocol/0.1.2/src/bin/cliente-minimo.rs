
use std::{fs,path::Path,io::{Read,Write},net::TcpListener,thread,time::Duration};
use serde_json::{json,Value};
use sv_mcp_documental::{*,custodia::{save,append,traced},supervision::supervise};
type R<T> = Result<T,Box<dyn std::error::Error>>;
fn call(name:&str,args:Value)->Value{json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":name,"arguments":args}})}
fn worker(a:&[String])->R<()>{
 let mode=&a[2];
 if mode=="reporte"{append(&Path::new(&a[3]).join("SUPERVISION.jsonl"),&parse_strict(a[4].as_bytes())?)?;println!("conservado");return Ok(())}
 if mode=="bloqueo-lectura"||mode=="bloqueo-escritura"{
 let mut f=fs::OpenOptions::new().read(true).write(true).open(&a[3])?;
 println!("{}",json!({"inicio_bloqueo":mode}));std::io::stdout().flush()?;
 if mode=="bloqueo-lectura"{let mut b=[0];f.read_exact(&mut b)?;}else{let b=vec![0u8;16*1024*1024];f.write_all(&b)?;f.sync_all()?;}return Ok(())
 }
 if mode=="mcp"{
 let root=Path::new(&a[3]);fs::create_dir_all(root)?;let req=parse_strict(a[7].as_bytes())?;
 traced(root,"mcp.documento",|_|{
 save(&root.join("SOLICITUD.json"),&serde_json::to_vec(&req)?)?;
 let old=vec![a[0].clone(),"worker".into(),a[4].clone(),a[5].clone(),a[6].clone(),a[3].clone(),"no".into(),"".into()];
 sv_mcp_documental::custodia::worker(&old)?;
 Ok(())
 })?;
 std::io::stdout().write_all(&fs::read(root.join("RESPUESTA.json"))?)?;return Ok(())
 }
 if mode=="http"{
 let root=Path::new(&a[3]);let phase=&a[4];let body=fs::read(&a[5])?;
 let endpoint=match phase.as_str(){"recuento"=>"http://127.0.0.1:1234/v1/messages/count_tokens","generacion"|"sintetico"=>"http://127.0.0.1:1234/v1/messages",_=>return Err("FASE_NO_ADMITIDA".into())};
 traced(root,&format!("http.{phase}"),|trace|{
 save(&root.join(format!("{phase}-SOLICITUD.json")),&body)?;
 let client=reqwest::blocking::Client::builder().no_proxy().redirect(reqwest::redirect::Policy::none()).retry(reqwest::retry::never()).timeout(Duration::from_secs(600)).build()?;
 let response=client.post(endpoint).header("content-type","application/json").body(body.clone()).send()?;
 let status=response.status();let bytes=response.bytes()?;
 save(&root.join(format!("{phase}-RESPUESTA.json")),&bytes)?;
 append(&root.join("HTTP.jsonl"),&json!({"phase":phase,"trace_id":trace,"status":status.as_u16(),"solicitud_sha256":sha256(&body),"respuesta_sha256":sha256(&bytes),"bytes":bytes.len(),"destino":endpoint,"proxies":false,"redirecciones":false,"reintentos":false}))?;
 if !status.is_success(){return Err(format!("HTTP_{status}").into())}Ok(())
 })?;
 println!("{}",json!({"conservado":true}));return Ok(())
 }
 Err("WORKER_DESCONOCIDO".into())
}
fn guarded(root:&Path,args:Vec<String>,ms:u64)->R<Vec<u8>>{
 let begin=std::time::Instant::now();let (r,b)=supervise(&args,ms.saturating_sub(1000))?;
 let remain=ms.saturating_sub(begin.elapsed().as_millis() as u64);
 let (record,_)=supervise(&["worker".into(),"reporte".into(),root.display().to_string(),r.to_string()],remain)?;
 if record["confirmado"]!=true{return Err("CONSERVACION_SUPERVISION_NO_CONFIRMADA".into())}
 if r["confirmado"]!=true{return Err(format!("SUPERVISION_NO_CONFORME:{r}").into())}Ok(b)
}
fn prepare(a:&[String])->R<()>{
 let plan:Value=parse_strict(&fs::read(&a[2])?)?;let root=Path::new(&a[3]);let bin=&a[4];let cat=&a[5];fs::create_dir_all(root)?;
 let cb=fs::read(cat)?;let hash=sha256(&cb);let mut section=String::new();let mut page=0;
 loop{let d=root.join(format!("mcp-pagina-{page}"));let raw=guarded(root,vec!["worker".into(),"mcp".into(),d.display().to_string(),bin.clone(),cat.clone(),hash.clone(),call("leer_documento",json!({"documento":"pdq-nci-hcl-es","seccion":"_1","pagina":page})).to_string()],30000)?;
 let v=parse_strict(&raw)?;if v["result"]["isError"]!=false{return Err("LECTURA_MCP".into())}
 let data=parse_strict(v["result"]["content"][0]["text"].as_str().ok_or("texto")?.as_bytes())?;section.push_str(data["texto"].as_str().ok_or("pagina sin texto")?);if data["siguiente_pagina"].is_null(){break}page=data["siguiente_pagina"].as_u64().ok_or("cursor")?;
 }
 if sha256(section.as_bytes())!=plan["fuente"]["seccion_sha256"].as_str().unwrap(){return Err("HUELLA_SECCION".into())}
 save(&root.join("SECCION_COMPLETA.txt"),section.as_bytes())?;
 let begin=plan["fuente"]["inicio_incluido"].as_str().unwrap();let end=plan["fuente"]["fin_excluido"].as_str().unwrap();
 if section.matches(begin).count()!=1||section.matches(end).count()!=1{return Err("DELIMITADORES_NO_UNICOS".into())}
 let i=section.find(begin).unwrap();let j=section.find(end).unwrap();let unit=&section[i..j];
 if unit.len()!=2041||unit.chars().count()!=1986||sha256(unit.as_bytes())!=plan["fuente"]["unidad_sha256"].as_str().unwrap(){return Err("UNIDAD_NO_CONFORME".into())}
 save(&root.join("UNIDAD_DOCUMENTAL.txt"),unit.as_bytes())?;
 let metadata="Documento: pdq-nci-hcl-es\nTitulo: Tratamiento de la leucemia de celulas pilosas (PDQ), version profesional\nSeccion: _1; Evaluacion diagnostica\nProcedencia: https://www.cancer.gov/espanol/tipos/leucemia/pro/tratamiento-celulas-pilosas-pdq\nCaptura: 2026-09-26T12:20:31Z\nActualizacion declarada: 11/14/2024\nLocalizador: pdq-nci-hcl-es/_1 — Evaluación diagnóstica";
 let text=format!("{}\n\n{}\n\n<UNIDAD_DOCUMENTAL_LITERAL>\n{}</UNIDAD_DOCUMENTAL_LITERAL>",plan["pregunta"].as_str().unwrap(),metadata,unit);
 let request=json!({"model":"default","max_tokens":384,"temperature":0,"stream":false,"enable_thinking":false,"truncate_sequence":false,"enable_code_execution":false,"tool_choice":{"type":"none"},"messages":[{"role":"user","content":text}]});
 let bytes=serde_json::to_vec(&request)?;let decoded=parse_strict(&bytes)?;
 if decoded["messages"][0]["content"]!=text{return Err("SERIALIZACION_NO_CONCORDE".into())}
 let recovered=decoded["messages"][0]["content"].as_str().unwrap().split("<UNIDAD_DOCUMENTAL_LITERAL>\n").nth(1).unwrap().strip_suffix("</UNIDAD_DOCUMENTAL_LITERAL>").unwrap();
 if recovered!=unit{return Err("UNIDAD_MODIFICADA".into())}
 save(&root.join("PETICION.json"),&bytes)?;
 save(&root.join("COTEJO-DOCUMENTAL.json"),&serde_json::to_vec_pretty(&json!({"paginas":page+1,"unidad_bytes":unit.len(),"unidad_caracteres":unit.chars().count(),"unidad_sha256":sha256(unit.as_bytes()),"seccion_sha256":sha256(section.as_bytes()),"peticion_sha256":sha256(&bytes),"roundtrip_exacto":true}))?)?;Ok(())
}
fn tests(a:&[String])->R<()>{
 let root=Path::new(&a[2]);fs::create_dir_all(root)?;
 for mode in ["bloqueo-lectura","bloqueo-escritura"]{
 let fifo=root.join(mode);let (r,b)=supervise(&["worker".into(),mode.into(),fifo.display().to_string()],1500)?;
 append(&root.join("BLOQUEOS.jsonl"),&json!({"recurso":mode,"supervision":r,"marcador":String::from_utf8_lossy(&b),"conforme":r["vencimiento"]==true&&r["recolectado"]==true&&r["senal"]==9}))?;
 if r["vencimiento"]!=true||r["recolectado"]!=true||r["senal"]!=9{return Err("BLOQUEO_NO_ACREDITADO".into())}
 }
 let listener=TcpListener::bind("127.0.0.1:1234")?;
 let synthetic=json!({"model":"default","messages":[{"role":"user","content":"Control sintético: ñ 漢字 🙂 \" \\"}],"max_tokens":384,"temperature":0,"stream":false,"enable_thinking":false,"truncate_sequence":false,"enable_code_execution":false,"tool_choice":{"type":"none"}});
 let bytes=serde_json::to_vec(&synthetic)?;let expected=bytes.clone();let p=root.join("PETICION-SINTETICA.json");save(&p,&bytes)?;
 let capture=root.join("CAPTURA-RECEPTOR.json");
 let th=thread::spawn(move||->Result<bool,String>{
 let (mut stream,_)=listener.accept().map_err(|e|e.to_string())?;stream.set_read_timeout(Some(Duration::from_secs(20))).map_err(|e|e.to_string())?;
 let mut h=Vec::new();let mut b=[0];while !h.ends_with(b"\r\n\r\n"){stream.read_exact(&mut b).map_err(|e|e.to_string())?;h.push(b[0]);}
 let hs=String::from_utf8(h).map_err(|e|e.to_string())?;let n=hs.lines().find_map(|l|l.to_lowercase().strip_prefix("content-length:").and_then(|n|n.trim().parse::<usize>().ok())).ok_or("longitud")?;
 let mut data=vec![0;n];stream.read_exact(&mut data).map_err(|e|e.to_string())?;save(&capture,&data).map_err(|e|e.to_string())?;
 let ok=data==expected&&hs.starts_with("POST /v1/messages HTTP/1.1");
 stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}").map_err(|e|e.to_string())?;Ok(ok)
 });
 guarded(root,vec!["worker".into(),"http".into(),root.display().to_string(),"sintetico".into(),p.display().to_string()],30000)?;
 let ok=th.join().map_err(|_|"hilo receptor")??;
 save(&root.join("CLIENTE-RESULTADO.json"),&serde_json::to_vec_pretty(&json!({"conforme":ok,"bytes_iguales":ok,"unicode":true,"receptor":"aislado en espacio de red distinto del motor","alcance":"Cliente HTTP; no adaptador interno del motor"}))?)?;
 if !ok{return Err("CLIENTE_NO_CONFORME".into())}Ok(())
}
fn run(a:&[String])->R<()>{
 let root=Path::new(&a[2]);let req=root.join("PETICION.json");
 guarded(root,vec!["worker".into(),"http".into(),root.display().to_string(),"recuento".into(),req.display().to_string()],30000)?;
 let n=parse_strict(&fs::read(root.join("recuento-RESPUESTA.json"))?)?["input_tokens"].as_u64().ok_or("recuento")?;
 save(&root.join("PRESUPUESTO.json"),&serde_json::to_vec(&json!({"entrada":n,"reserva":384,"max_entrada":2000,"contexto":8192}))?)?;
 if n>2000||n+384>8192{return Err("IMPEDIMENTO_CONTEXTO".into())}
 guarded(root,vec!["worker".into(),"http".into(),root.display().to_string(),"generacion".into(),req.display().to_string()],600000)?;Ok(())
}
fn main(){
 let a:Vec<String>=std::env::args().collect();
 let r=match a.get(1).map(String::as_str){Some("worker")=>worker(&a),Some("preparar")=>prepare(&a),Some("probar")=>tests(&a),Some("ejecutar")=>run(&a),_=>Err("modo no admitido".into())};
 if let Err(e)=r{eprintln!("{e}");std::process::exit(1)}
}
