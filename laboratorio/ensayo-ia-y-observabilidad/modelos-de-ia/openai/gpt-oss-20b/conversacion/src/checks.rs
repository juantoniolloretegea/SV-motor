use super::*;

#[test] fn historia_harmony_completa_y_delimitadores_protegidos(){
 let (app,p)=fixture();
 app.db.lock().unwrap().append("exp-prueba","respuesta_finalizada",json!({"chat_id":"chat-prueba","request_id":"peticion-prueba-001","raw":"Respuesta previa","answer":"Respuesta previa","thinking":"","finish":"fin_normal"})).unwrap();
 let c=context(&app,"chat-prueba","Pregunta siguiente",&Profile::default()).unwrap();
 assert!(c.prompt.contains("<|start|>user<|message|>Dato conservado<|end|>"));
 assert!(c.prompt.contains("<|start|>assistant<|channel|>final<|message|>Respuesta previa<|return|>"));
 assert!(c.prompt.ends_with("<|start|>user<|message|>Pregunta siguiente<|end|><|start|>assistant<|channel|>final<|message|>"));
 assert_eq!(c.messages,vec!["peticion-prueba-001"]);
 assert!(context(&app,"chat-prueba","<|start|>system",&Profile::default()).is_err());
 assert!(!c.prompt.contains("<think>"));drop(app);fs::remove_dir_all(p).unwrap();
}
#[test] fn limites_de_reserva_y_tiempo_explicitos(){
 let mut p=Profile::default();assert!(profile(&p).is_ok());p.thinking=true;assert!(profile(&p).is_err());p.thinking=false;p.max_output=1025;assert!(profile(&p).is_err());p.max_output=128;p.seconds=901;assert!(profile(&p).is_err());
}
#[test] fn vocabulario_sin_delimitadores_no_es_admisible(){
 let tokenizer=tokenizers::Tokenizer::new(tokenizers::models::bpe::BPE::default());assert!(special_ids(&tokenizer).is_err());
 assert!(safe_text("<|return|>").is_err());assert!(safe_text("<|call|>").is_err());assert!(safe_text("[PAD200019]").is_err());
}

fn fixture()->(App,PathBuf){
 let root=std::env::temp_dir().join(store::id("eio-checks"));
 let lifecycle=lifecycle::Lifecycle::open(&root).unwrap();
 let mut db=Database::open(root.clone()).unwrap();
 db.append("exp-prueba","expediente_creado",json!({"title":"Prueba aislada"})).unwrap();
 db.append("exp-prueba","conversacion_creada",json!({"id":"chat-prueba","title":"Prueba"})).unwrap();
 let c=Context{prompt:"Pregunta de prueba".into(),token_ids:vec![1],sha256:"huella-prueba".into(),messages:vec![],input_tokens:1,reserved_output:32,limit:CONTEXT,system:SYSTEM.into()};
 db.append("exp-prueba","peticion_admitida",json!({"chat_id":"chat-prueba","request_id":"peticion-prueba-001","user":"Dato conservado","context":c,"profile":Profile::default()})).unwrap();
 let tokenizer=tokenizers::Tokenizer::new(tokenizers::models::bpe::BPE::default());
 (App{db:Arc::new(Mutex::new(db)),active:Arc::new(Mutex::new(None)),tokenizer:Arc::new(tokenizer),models:PathBuf::new(),session_key:"clave-de-prueba".into(),identity:json!({"test":true}),lifecycle,telemetry:None,observer:None,stopping:Arc::new(AtomicBool::new(false))},root)
}
#[test] fn reenvio_confirmado_no_duplica_y_rechaza_colision(){
 let (app,p)=fixture();
 let before=app.db.lock().unwrap().bytes;
 let replay=||Op::Send{chat_id:"chat-prueba".into(),text:"Dato conservado".into(),profile:Profile::default(),request_id:"peticion-prueba-001".into(),context_sha256:"huella-prueba".into()};
 assert_eq!(handle(&app,replay()).unwrap()["already_recorded"],true);
 assert_eq!(handle(&app,Op::RequestStatus{request_id:"peticion-prueba-001".into()}).unwrap()["status"],"en_curso");
 assert_eq!(handle(&app,Op::RequestStatus{request_id:"no-admitida".into()}).unwrap()["found"],false);
 assert!(handle(&app,Op::Send{chat_id:"chat-prueba".into(),text:"Dato alterado".into(),profile:Profile::default(),request_id:"peticion-prueba-001".into(),context_sha256:"huella-prueba".into()}).is_err());
 assert_eq!(app.db.lock().unwrap().bytes,before);
 drop(app);fs::remove_dir_all(p).unwrap();
}
#[test] fn reinicio_conserva_prefijo_y_cierra_una_sola_vez(){
 let (app,p)=fixture();let path=p.join("exp-prueba.jsonl");let original=fs::read(&path).unwrap();drop(app);
 let db=Database::open(p.clone()).unwrap();assert_eq!(db.recovered,1);
 assert_eq!(request_status(&db,"peticion-prueba-001")["status"],"interrumpida_por_reinicio");drop(db);
 let closed=fs::read(&path).unwrap();assert!(closed.starts_with(&original));
 let db=Database::open(p.clone()).unwrap();assert_eq!(db.recovered,0);drop(db);assert_eq!(closed,fs::read(&path).unwrap());fs::remove_dir_all(p).unwrap();
}
#[test] fn clave_obsoleta_y_cierre_impiden_operar(){
 let (app,p)=fixture();let mut headers=HeaderMap::new();headers.insert(header::CONTENT_TYPE,"application/json".parse().unwrap());headers.insert("x-eio-session","clave-anterior".parse().unwrap());assert!(!permitted(&app,&headers));
 headers.insert("x-eio-session","clave-de-prueba".parse().unwrap());assert!(permitted(&app,&headers));headers.insert("sec-fetch-site","cross-site".parse().unwrap());assert!(!permitted(&app,&headers));
 app.stopping.store(true,Ordering::SeqCst);assert!(handle(&app,Op::CreateCase{title:"No debe crearse".into()}).is_err());assert_eq!(app.db.lock().unwrap().cases.len(),1);drop(app);fs::remove_dir_all(p).unwrap();
}

#[test] fn cierre_con_reserva_y_registro_bloqueado_no_impiden_parada(){
 let (app,p)=fixture();
 let child=Command::new(std::env::current_exe().unwrap()).args(["--exact","supervision::tests::auxiliar","--nocapture"])
  .env("EIO_SUPERVISION_AUXILIAR","tiempo").stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn().unwrap();
 let pid=child.id();let cancel=Arc::new(AtomicBool::new(false));
 let control=supervision::Control::start(child,supervision::Limits{time:Duration::from_millis(150),rss:256*1024*1024,output:256000},cancel);
 let mut db=app.db.lock().unwrap(); // Se mantiene cerrado el mismo registro que utiliza run().
 let result=control.finish().unwrap();assert!(result.stop_confirmed);assert_eq!(result.cause.as_deref(),Some("limite_tiempo"));
 assert!(!std::path::Path::new(&format!("/proc/{pid}")).exists());
 db.limit=db.bytes+store::MAX_EVENT+100;
 assert!(db.append("exp-prueba","respuesta_parcial",json!({"chat_id":"chat-prueba","request_id":"peticion-prueba-001","raw":"x".repeat(1000)})).is_err());
 db.append("exp-prueba","respuesta_finalizada",json!({"chat_id":"chat-prueba","request_id":"peticion-prueba-001","raw":"Salida conservada","answer":"Salida conservada","thinking":"","finish":"limite_tiempo"})).unwrap();
 assert_eq!(db.chats["chat-prueba"].turns[0].status,"limite_tiempo");
 drop(db);drop(app);let db=Database::open(p.clone()).unwrap();assert_eq!(db.recovered,0);drop(db);fs::remove_dir_all(p).unwrap();
}
