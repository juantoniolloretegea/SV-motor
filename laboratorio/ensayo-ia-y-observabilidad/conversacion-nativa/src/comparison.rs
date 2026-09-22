//! Campaña exploratoria nativa: doce generaciones, directorio nuevo, sin expedientes del usuario.
use super::*;

pub fn execute()->Result<()>{
 let output=PathBuf::from(std::env::args().nth(2).ok_or("Uso: --compare DIRECTORIO_NUEVO")?);
 fs::create_dir(&output)?;
 let models=PathBuf::from(std::env::var("EIO_MODELS").unwrap_or("/workspaces/eio-instalacion-nativa-20260922".into()));
 for(file,expected)in[("Qwen3-0.6B-Q4_K_M.gguf",MODEL_HASH),("tokenizer.json",TOKENIZER_HASH)]{if store::file_hash(&models.join(file))?!=expected{return Err(format!("Identidad no conforme: {file}").into())}}
 let tokenizer=tokenizers::Tokenizer::from_file(models.join("tokenizer.json")).map_err(|e|e.to_string())?;
 let root=output.join("datos");let lifecycle=lifecycle::Lifecycle::open(&root)?;
 let identity=json!({"model":"Qwen3-0.6B · Q4_K_M","model_sha256":MODEL_HASH,"tokenizer_sha256":TOKENIZER_HASH,
  "binary_sha256":store::file_hash(&std::env::current_exe()?)?,"candle_revision":"ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a","device":"CPU","experiment":"comparacion-01"});
 let app=App{db:Arc::new(Mutex::new(Database::open(root)?)),active:Arc::new(Mutex::new(None)),tokenizer:Arc::new(tokenizer),models,session_key:"sin-servidor-http".into(),identity:identity.clone(),lifecycle,stopping:Arc::new(AtomicBool::new(false))};
 // La campaña comparte los pesos y la implementación, pero no inicia HTTP ni abre los expedientes reales.
 let questions=[
  ("lexico","¿La palabra «tonto» pertenece al español? Explique brevemente su significado.","Reconoce la palabra española y explica su significado; no deriva la pregunta a asistencia sanitaria."),
  ("entidad","En este ejercicio la enfermedad indicada es tricoleucemia. Repita exclusivamente el nombre de esa enfermedad, sin sustituirlo por otra.","Respuesta final exacta: tricoleucemia. No se evalúa conocimiento ni consejo clínico."),
  ("cantidad","En un inventario hay 17 piezas. Se retiran 5. ¿Cuántas quedan? Responda con el número y una frase breve.","12 piezas; conserva los datos 17 y 5 y no inventa operaciones adicionales."),
 ];
 let mut rows=Vec::new();
 let mut report=fs::OpenOptions::new().create_new(true).write(true).open(output.join("RESULTADOS.jsonl"))?;
 for (name,question,oracle) in questions {for history in [false,true]{for thinking in [false,true]{
  let label=format!("{name}-historia-{history}-razonamiento-{thinking}");
  let case=handle(&app,Op::CreateCase{title:format!("Prueba sintética: {label}")})?["id"].as_str().ok_or("Sin expediente")?.to_string();
  let chat=handle(&app,Op::CreateChat{case_id:case.clone(),title:label.clone()})?["id"].as_str().ok_or("Sin conversación")?.to_string();
  if history {
   let text=(1..=24).map(|n|format!("Registro de ensayo {n}: el contenedor azul tiene una etiqueta; este antecedente no modifica la próxima pregunta.\n")).collect::<String>();
   let profile=Profile{thinking:false,max_output:384,seconds:180,seed:299792458};
   let ctx=context(&app,&chat,&text,&profile)?;let request=store::id("antecedente-sintetico");
   let mut db=app.db.lock().map_err(|_|"Registro bloqueado")?;
   db.append(&case,"peticion_admitida",json!({"chat_id":chat,"request_id":request,"user":text,"context":ctx,"profile":profile,"origin":"fixture_sintetico_no_generado_por_modelo"}))?;
   db.append(&case,"respuesta_finalizada",json!({"chat_id":chat,"request_id":request,"raw":"Antecedentes del ensayo recibidos.","answer":"Antecedentes del ensayo recibidos.","thinking":"","finish":"fin_normal","origin":"fixture_sintetico_no_generado_por_modelo"}))?;
  }
  let profile=Profile{thinking,max_output:384,seconds:180,seed:299792458};
  let ctx=context(&app,&chat,question,&profile)?;let request=store::id("comparacion");
  handle(&app,Op::Send{chat_id:chat.clone(),text:question.into(),profile,request_id:request.clone(),context_sha256:ctx.sha256.clone()})?;
  let limit=Instant::now()+Duration::from_secs(200);
  while app.active.lock().map_err(|_|"Estado bloqueado")?.is_some(){if Instant::now()>limit{return Err("La prueba excedió el cierre previsto; revisar la salida conservada".into())}std::thread::sleep(Duration::from_millis(100));}
  let db=app.db.lock().map_err(|_|"Registro bloqueado")?;let turn=db.chats.get(&chat).and_then(|c|c.turns.iter().find(|t|t.id==request)).ok_or("Sin resultado")?;
  let row=json!({"case":label,"synthetic_history":history,"thinking":thinking,"question":question,"oracle":oracle,
   "input_tokens":ctx.input_tokens,"context_sha256":ctx.sha256,"request_id":request,"result":turn.result,
   "content_assessment":"pendiente_de_revision_explicita"});
  serde_json::to_writer(&mut report,&row)?;report.write_all(b"\n")?;report.sync_all()?;
  println!("EIO_COMPARACION {} {}",label,turn.status);rows.push(row);
 }}}
 let summary=json!({"schema":"EIO-COMPARACION-1","identity":identity,"utc_ms":store::now(),"cases":rows,
  "design":"Tres preguntas por dos condiciones de antecedentes y dos modos. Una ejecución por condición; sin inferencia estadística ni atribución causal exclusiva.",
  "limits":"384 tokens de salida y 180 segundos por petición. Una interrupción no se clasifica como respuesta semánticamente incorrecta. Los modos también cambian sus perfiles de muestreo; no se aísla únicamente el texto de razonamiento.",
  "runtime":"Rust/Cargo 1.98.0; confirmar identidad antes de ejecutar; no compilar ni mantener otra inferencia simultánea"});
 fs::write(output.join("RESUMEN.json"),serde_json::to_vec_pretty(&summary)?)?;Ok(())
}
