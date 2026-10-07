#[cfg(test)] mod parameter_recovery_tests {
 use super::*;
 #[test] fn p15_sse_original_rechazado_antes_de_mcp(){
  use std::net::TcpListener;
  let source=Path::new(crate::param_recovery::P15);
  let payload=fs::read(source.join("01-GENERACION-ORIGINAL.sse")).unwrap();
  let listener=TcpListener::bind("127.0.0.1:1234").unwrap();let body=payload.clone();
  let server=std::thread::spawn(move||{let(mut s,_)=listener.accept().unwrap();s.set_read_timeout(Some(Duration::from_secs(10))).unwrap();let mut input=BufReader::new(s.try_clone().unwrap());let mut n=0;loop{let mut line=String::new();input.read_line(&mut line).unwrap();if line=="\r\n"{break}if let Some((k,v))=line.split_once(':'){if k.eq_ignore_ascii_case("content-length"){n=v.trim().parse::<usize>().unwrap()}}}let mut req=vec![0;n];input.read_exact(&mut req).unwrap();write!(s,"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",body.len()).unwrap();s.write_all(&body).unwrap();});
  let root=std::env::temp_dir().join(store::id("replay-p15"));fs::create_dir(&root).unwrap();let client=reqwest::blocking::Client::builder().no_proxy().build().unwrap();
  let e=generate(&client,&json!({}),&root,"01-GENERACION",Instant::now()+Duration::from_secs(20)).unwrap_err();server.join().unwrap();
  assert_eq!(e.downcast_ref::<crate::recovery::ItemFault>().unwrap().0,crate::recovery::ItemCode::MalformedParameters);
  assert_eq!(fs::read(root.join("01-GENERACION-ORIGINAL.sse")).unwrap(),payload);
  assert_eq!(fs::read(root.join("01-GENERACION-EMISION.bin")).unwrap(),fs::read(source.join("01-GENERACION-EMISION.bin")).unwrap());
  assert!(!root.join("mcp").exists());assert!(!root.join("01-GENERACION-RESULTADO.json").exists());
  let e=generate(&client,&json!({}),&root,"01-GENERACION",Instant::now()+Duration::from_secs(1)).unwrap_err();assert!(e.downcast_ref::<crate::recovery::ItemFault>().is_none());fs::remove_dir_all(root).unwrap();
 }
 #[test] fn parametros_ambiguos_no_se_reparan(){for b in ["<tool_call><function=buscar_documentos><parameter=consulta>x</parameter><parameter=consulta>y</parameter></function></tool_call>","<tool_call><function=buscar_documentos><parameter=consulta>x<parameter=limite>1</parameter></parameter></function></tool_call>"]{let e=parsed_calls(b.as_bytes()).unwrap_err();assert_eq!(e.downcast_ref::<crate::recovery::ItemFault>().unwrap().0,crate::recovery::ItemCode::MalformedParameters);}}
 #[test] fn herramienta_ajena_y_ruta_invalidas_siguen_generales(){for b in [r#"<tool_call>{"name":"shell","arguments":{}}</tool_call>"#,r#"<tool_call>{"name":"leer_documento","arguments":{"documento":"../x","seccion":"s"}}</tool_call>"#]{assert!(parsed_calls(b.as_bytes()).unwrap_err().downcast_ref::<crate::recovery::ItemFault>().is_none());}}
}
