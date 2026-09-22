use super::*;

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
