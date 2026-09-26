
use std::{net::TcpListener,io::{Read,Write},fs,path::Path,process::Command};
use serde_json::{json,Value};
use sv_mcp_documental::{parse_strict,custodia::{save,append}};
fn main()->Result<(),Box<dyn std::error::Error>>{
 let a:Vec<String>=std::env::args().collect();let root=Path::new(&a[1]);fs::create_dir_all(root)?;
 let listener=TcpListener::bind("127.0.0.1:1234")?;
 let base="/srv/tt0014-20260926";
 let mut child=Command::new(format!("{base}/bin/ensayo-local")).args(["consulta",&format!("{base}/bin/sv-mcp-documental"),&format!("{base}/emulacion-dominio-inmunologia/catalogo-pdq.json"),root.join("cliente").to_str().unwrap()]).spawn()?;
 let mut gen=0;let mut received=0;
 for _ in 0..6{
 let (mut s,_)=listener.accept()?;s.set_read_timeout(Some(std::time::Duration::from_secs(10)))?;
 let mut header=Vec::new();let mut one=[0];while !header.ends_with(b"\r\n\r\n"){s.read_exact(&mut one)?;header.push(one[0]);if header.len()>16384{return Err("cabecera".into())}}
 let h=String::from_utf8(header)?;let n=h.lines().find_map(|l|l.to_lowercase().strip_prefix("content-length:").and_then(|v|v.trim().parse::<usize>().ok())).ok_or("longitud")?;
 let mut b=vec![0;n];s.read_exact(&mut b)?;let req=parse_strict(&b)?;
 let response=if h.lines().next().unwrap().contains("count_tokens"){json!({"input_tokens":1000})}else{
 gen+=1;
 if gen>1{let messages=req["messages"].as_array().ok_or("messages")?;let block=&messages.last().ok_or("last")?["content"][0];
 let retained=fs::read(root.join(format!("cliente/RESULTADO-REENVIABLE-{}.json",gen-1)))?;
 if parse_strict(&retained)?!=*block{return Err("Reenvio difiere de evidencia".into())}
 let raw=fs::read(root.join(format!("cliente/mcp/llamada-{:03}/RESPUESTA.json",gen)))?;
 let sent=parse_strict(block["content"].as_str().ok_or("content")?.as_bytes())?;
 if parse_strict(&raw)?!=sent{return Err("Respuesta MCP modificada".into())}received+=1;
 }
 let content=match gen{1=>json!([{"type":"tool_use","id":"s1","name":"buscar_documentos","input":{"consulta":"leucemia"}}]),2=>json!([{"type":"tool_use","id":"s2","name":"leer_documento","input":{"documento":"pdq-nci-hcl-es","seccion":"_1","pagina":0}}]),_=>json!([{"type":"text","text":"Respuesta sintética exclusiva de comprobación del cliente; no procede del candidato."}])};
 json!({"role":"assistant","content":content,"stop_reason":if gen<3{"tool_use"}else{"end_turn"}})
 };
 let out=serde_json::to_vec(&response)?;write!(s,"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",out.len())?;s.write_all(&out)?;s.flush()?;
 }
 drop(listener);let status=child.wait()?;save(&root.join("RESULTADO.json"),&serde_json::to_vec_pretty(&json!({"sintetico":true,"candidato_cargado":false,"generaciones_simuladas":gen,"reenvios_verificados":received,"cliente_codigo":status.code(),"conforme":status.success()&&received==2}))?)?;
 if !status.success()||received!=2{return Err("cliente no conforme".into())}Ok(())
}
