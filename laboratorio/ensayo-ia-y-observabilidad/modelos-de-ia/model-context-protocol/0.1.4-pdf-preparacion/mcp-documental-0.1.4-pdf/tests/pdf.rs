use serde_json::{json,Value};
use std::{fs,io::Write,path::{Path,PathBuf},process::{Command,Stdio},time::{SystemTime,UNIX_EPOCH}};
use sv_mcp_documental::{Catalog,sha256,fuentes_autorizadas,auditoria::{verify,unhex}};
const SHA:&str="21322ed388c999bd0713ba5ec5c7ab34402d4bb4cd6100903aa5ea6d8bf35d9c";
const URL:&str="https://raw.githubusercontent.com/juantoniolloretegea/SVperitus-dataset/488d597fa1999a1fc2eb6538fd610a36046f6f01/dominios/inmunologia/literatura-tricoleucemia/lls-hoja-informativa-2018/original/hairy-cell-leukemia.pdf";
fn root()->PathBuf{
 let base=std::env::var_os("SV_PDF_EVIDENCIAS").map(PathBuf::from).unwrap_or_else(||PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/evidencias-pdf"));
 let p=base.join(format!("{}-{}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));fs::create_dir_all(&p).unwrap();p
}
fn prepare(p:&Path,hash:&str)->std::process::Output{
 Command::new(env!("CARGO_BIN_EXE_preparar-pdf")).arg(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../lector/tests/fixtures/hairy-cell-leukemia.pdf")).args([hash,"LLS-HCL-2018",URL]).arg(p).output().unwrap()
}
fn fixture()->(PathBuf,Catalog,String,String){
 let dir=root().join("preparacion");let out=prepare(&dir,SHA);assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));
 fs::write(dir.join("preparador.stdout"),out.stdout).unwrap();fs::write(dir.join("preparador.stderr"),out.stderr).unwrap();
 let cat_hash=sha256(&fs::read(dir.join("CATALOGO.json")).unwrap());let sources_hash=sha256(&fs::read(dir.join("FUENTES.json")).unwrap());
 let sources=fuentes_autorizadas(&dir.join("FUENTES.json"),&sources_hash).unwrap();
 let cat=Catalog::load_with_sources(&dir.join("CATALOGO.json"),&cat_hash,false,&sources).unwrap();(dir,cat,cat_hash,sources_hash)
}
#[test]fn pdf_completo_por_transporte_real_y_diario_reproducible(){
 let(dir,c,hash,sh)=fixture();assert_eq!(c.documents[0].sections.len(),10);
 let mut requests=vec![json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":sv_mcp_documental::PROTOCOL,"capabilities":{},"clientInfo":{"name":"comprobacion-documental","version":"1"}}}),json!({"jsonrpc":"2.0","method":"notifications/initialized"}),json!({"jsonrpc":"2.0","id":2,"method":"tools/list"})];
 // Consulta instrumental de todos los fragmentos; no es una conversación ni inferencia del candidato.
 let mut id=3;let mut expected=Vec::new();
 for s in &c.documents[0].sections{
  let mut session=sv_mcp_documental::Session::default();session.handle(&c,requests[0].clone());session.handle(&c,requests[1].clone());
  let mut page=0;
  loop{let r=json!({"jsonrpc":"2.0","id":id,"method":"tools/call","params":{"name":"leer_documento","arguments":{"documento":"LLS-HCL-2018","seccion":s.id,"pagina":page}}});
   let result=session.handle(&c,r.clone()).unwrap();assert_eq!(result["result"]["isError"],false);
   let next=result["result"]["structuredContent"]["siguiente_pagina"].as_u64();requests.push(r);expected.push((id,s.id.clone(),page));id+=1;if let Some(n)=next{page=n}else{break}
  }
 }
 let mut input=Vec::new();for r in &requests{input.extend(serde_json::to_vec(r).unwrap());input.push(b'\n');}
 fs::write(dir.join("solicitudes.jsonl"),&input).unwrap();
 let mut p=Command::new(env!("CARGO_BIN_EXE_sv-mcp-documental")).arg(dir.join("CATALOGO.json")).arg(&hash).arg(dir.join("diario.jsonl")).args(["256","--fuentes-autorizadas"]).arg(dir.join("FUENTES.json")).arg(&sh).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
 let mut pipe=p.stdin.take().unwrap();let sent=input.clone();let writer=std::thread::spawn(move||pipe.write_all(&sent).unwrap());let out=p.wait_with_output().unwrap();writer.join().unwrap();
 fs::write(dir.join("respuestas.jsonl"),&out.stdout).unwrap();fs::write(dir.join("mcp.stderr"),&out.stderr).unwrap();assert!(out.status.success());
 let replies:Vec<Value>=out.stdout.split(|b|*b==b'\n').filter(|b|!b.is_empty()).map(|b|serde_json::from_slice(b).unwrap()).collect();
 assert_eq!(replies[1]["result"]["tools"].as_array().unwrap().len(),2);
 let mut recovered=std::collections::BTreeMap::<String,String>::new();
 for(id,s,page)in &expected{let r=replies.iter().find(|r|r["id"]==*id).unwrap();assert_eq!(r["result"]["isError"],false);assert!(r.to_string().chars().count()<=sv_mcp_documental::MAX_RESPONSE);
  let d=&r["result"]["structuredContent"];assert_eq!(*d,serde_json::from_str::<Value>(r["result"]["content"][0]["text"].as_str().unwrap()).unwrap());assert_eq!(d["pagina"],*page);
  let idx=s.strip_prefix("PDF-P").unwrap().parse::<usize>().unwrap();assert_eq!(d["pagina_pdf_indice"],idx);assert_eq!(d["pagina_pdf_ordinal"],idx+1);
  recovered.entry(s.clone()).or_default().push_str(d["texto"].as_str().unwrap());
 }
 for s in &c.documents[0].sections{assert_eq!(recovered[&s.id],s.text);}
 let journal=fs::read(dir.join("diario.jsonl")).unwrap();let replay=verify(&c,&hash,&journal).unwrap();let(mut received,mut emitted)=(Vec::new(),Vec::new());
 for line in journal.split(|b|*b==b'\n').filter(|b|!b.is_empty()) {let v:Value=serde_json::from_slice(line).unwrap();let d=&v["datos"];match d["evento"].as_str(){Some("solicitud")=>received.extend(unhex(d["bytes_hex"].as_str().unwrap()).unwrap()),Some("resultado")=>emitted.extend(unhex(d["bytes_hex"].as_str().unwrap()).unwrap()),_=>{}}}
 assert_eq!(received,input);assert_eq!(emitted,out.stdout);
 let check=|source: &Path,h: &str|Command::new(env!("CARGO_BIN_EXE_verificar-diario")).arg(dir.join("CATALOGO.json")).arg(&hash).arg(dir.join("diario.jsonl")).args(["--oficial","--fuentes-autorizadas"]).arg(source).arg(h).output().unwrap();
 assert!(check(&dir.join("FUENTES.json"),&sh).status.success());
 let altered=fs::read_to_string(dir.join("FUENTES.json")).unwrap()+"\n";fs::write(dir.join("FUENTES-OTRA-IDENTIDAD.json"),&altered).unwrap();assert!(!check(&dir.join("FUENTES-OTRA-IDENTIDAD.json"),&sha256(altered.as_bytes())).status.success());
 let proof=json!({"conforme":true,"inferencia":false,"paginas_fisicas":10,"fragmentos":expected.len(),"texto_reconstruido_identico":true,"solicitudes_y_respuestas_identicas_al_diario":true,"lista_fuentes_distinta_rechazada":true,"catalogo_sha256":hash,"fuentes_sha256":sh,"cotejo_diario":replay});
 fs::write(dir.join("COMPROBACION.json"),serde_json::to_vec_pretty(&proof).unwrap()).unwrap();println!("EVIDENCIA_PDF={}",dir.display());
}
#[test]fn fuente_pdf_requiere_admision_exacta(){let(_dir,c,_h,_sh)=fixture();assert!(c.validate(false).is_err());for field in 0..3{let mut s=sv_mcp_documental::FuenteAutorizada{documento:c.documents[0].id.clone(),url:URL.into(),sha256:SHA.into()};match field{0=>s.documento="otro".into(),1=>s.url="https://example.invalid/otro.pdf".into(),_=>s.sha256="0".repeat(64)}assert!(c.validate_with_sources(false,&[s]).is_err());}}
#[test]fn preparador_rechaza_huella_sin_catalogo(){let dir=root().join("rechazo");assert!(!prepare(&dir,&"0".repeat(64)).status.success());assert!(dir.join("ERROR.json").is_file());assert!(!dir.join("CATALOGO.json").exists());}
#[test]fn preparador_preserva_carpeta_preexistente(){let(dir,_,_,_)=fixture();let before=fs::read(dir.join("CATALOGO.json")).unwrap();assert!(!prepare(&dir,SHA).status.success());assert_eq!(before,fs::read(dir.join("CATALOGO.json")).unwrap());}
