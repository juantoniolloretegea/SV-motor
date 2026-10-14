//! Generación real y acotada bajo el perfil experimental del Árbitro - Director.
use super::*;
use conexion::{nombre,mediana,destino,recursos,memoria,comprobar_gpu};
use selector::{Carga,Clase};
const MEM:u64=1024*1024*1024;
pub fn tokens_sha(v:&[u32])->String{format!("{:x}",Sha256::digest(serde_json::to_vec(v).unwrap()))}
pub fn carga(t:&[u32])->Carga{Carga{contrato:"qwen3-8b-generacion-gobernada/v1".into(),revision:MODELO.into(),perfil:format!("Q4_K_M;residentes;contexto64;CPU4;T4-sm75;mwv;hw_dp4a;greedy;salida16;fin151645,151643;entrada={}",tokens_sha(t)),dimensiones:vec![t.len(),16,151936]}}
struct Emision{registro:Value,vectores:Vec<Vec<f32>>,tokens:Vec<u32>,tiempo:u64}
fn argmax(v:&[f32])->R<u32>{
 if v.len()!=151936||v.iter().any(|x|!x.is_finite()){return Err("Vector incompleto o no finito".into());}
 Ok(v.iter().enumerate().max_by(|(i,a),(j,b)|a.total_cmp(b).then(j.cmp(i))).ok_or("Vector vacío")?.0 as u32)
}
fn generar(g:&mut OwnedQuantizedModelCuda,reloj:&RelojGpu,clase:Clase,tokens:&[u32],meta:&realizar::gguf::GGUFModel,dir:&Path,id:&str)->R<Emision>{
 fase(id);let t=Instant::now();let n=traza::total();let ca=cpu()?;
 let mut cache=OwnedQuantizedKVCache::new(g.model().config().num_layers,g.model().config().kv_dim(),64);
 if clase==Clase::Gpu {g.executor_mut().reset_kv_cache_gpu();g.executor().synchronize()?;}
 let mut secuencia=tokens.to_vec();let mut emitidos=Vec::new();let mut vectores=Vec::new();let mut posiciones=Vec::new();let mut fin="cota_salida";
 let mut i=0;
 while i<secuencia.len(){
  let tp=Instant::now();let antes=cpu()?;let ln=traza::total();let token=secuencia[i];
  let (v,eventos)=match clase{
   Clase::Cpu=>(g.model().forward_single_with_cache(token,&mut cache,i)?,None),
   Clase::Gpu=>{let ev=reloj.iniciar(g.executor().sv_execution_stream().raw() as usize)?;let v=g.forward_gpu_resident(token,&mut cache,i)?;let e=ev.terminar()?;g.executor().synchronize()?;(v,Some(e))},
   _=>return Err("Clase sin adaptador".into())};
  let elegido=argmax(&v)?;let lanzamientos=traza::total()-ln;
  if (clase==Clase::Cpu&&lanzamientos!=0)||(clase==Clase::Gpu&&lanzamientos==0){return Err("Destino físico discordante".into());}
  let salida=if i+1>=tokens.len(){
   vector(dir,&format!("{id}-{:02}.f32",emitidos.len()),&v)?;vectores.push(v);emitidos.push(elegido);
   Some(elegido)
  }else{None};
  posiciones.push(json!({"posicion":i,"entrada_token":token,"salida_token":salida,"lanzamientos_gpu":lanzamientos,"intervalo_gpu_ms":eventos,"pared_us":tp.elapsed().as_micros(),"cpu_antes":antes,"cpu_despues":cpu()?}));
  if salida.is_some(){
   if [151645,151643].contains(&elegido){fin="fin_secuencia";break;}
   if emitidos.len()==16{break;}
   secuencia.push(elegido);
  }i+=1;
 }
 drop(cache);
 let texto=meta.decode(&emitidos);let vocab=meta.vocabulary().ok_or("Vocabulario ausente")?;
 let fragmentos=emitidos.iter().map(|id|vocab[*id as usize].clone()).collect::<Vec<_>>();
 let salida=json!({"tokens":emitidos,"fragmentos_vocabulario":fragmentos,"texto":texto,"fin":fin,"respuesta_completa":fin=="fin_secuencia","sin_reescritura":true});
 guardar(dir,&format!("{id}-SALIDA.json"),&salida)?;
 let tiempo=t.elapsed().as_micros() as u64;
 let registro=json!({"id":id,"clase_ejecutada":nombre(clase),"entrada_sha256":tokens_sha(tokens),"entrada_tokens":tokens,"tiempo_servicio_us":tiempo,"cpu_antes":ca,"cpu_despues":cpu()?,"lanzamientos_gpu":traza::total()-n,"posiciones":posiciones,"salida":salida,"completo_instrumental":true,"alcance_temporal":"Caché, cálculo, recuperación, observación y conservación de vectores y salida; pesos preparados; sin selección ni evaluación numérica posterior."});
 guardar(dir,&format!("{id}.json"),&registro)?;
 Ok(Emision{registro,vectores,tokens:emitidos,tiempo})
}
fn contrastar(a:&Emision,b:&Emision)->R<Value>{
 if a.tokens!=b.tokens||a.vectores.len()!=b.vectores.len()||a.vectores.is_empty(){return Err("Trayectorias de generación diferentes; no se corrigen ni repiten".into());}
 let v=a.vectores.iter().zip(&b.vectores).map(|(a,b)|comparar(a,b)).collect::<R<Vec<_>>>()?;
 if v.iter().any(|v|v["conforme"]!=true){return Err("Paridad numérica no conforme".into());}
 Ok(json!({"conforme":true,"tokens_iguales":true,"comparaciones":v}))
}
fn ensayo(modelo:&Path,d:&Path)->R<Value>{
 let origen=Instant::now();fase("identidad_gobernada");
 if fs::metadata(modelo)?.len()!=BYTES||hash(modelo)?!=MODELO{return Err("Pesos no conformes".into());}
 let nv=nvml_wrapper::Nvml::init()?;
 let inv=(0..nv.device_count()?).map(|i|->R<Value>{let v=nv.device_by_index(i)?;let m=v.memory_info()?;Ok(json!({"indice":i,"uuid":v.uuid()?,"nombre":v.name()?,"memoria_total":m.total,"memoria_libre":m.free}))}).collect::<R<Vec<_>>>()?;
 let uuid=inv.first().and_then(|v|v["uuid"].as_str()).ok_or("GPU ausente")?.to_string();
 if !inv[0]["nombre"].as_str().unwrap_or("").contains("T4")||memoria(&nv,&uuid)?[0]<12*MEM||memoria(&nv,&uuid)?[1]<8*MEM{return Err("Hardware o memoria fuera del perfil".into());}
 let cp=fs::read_to_string("/proc/cpuinfo")?;let cid=format!("CPU-{:x}",Sha256::digest(cp.as_bytes()));
 guardar(d,"INVENTARIO.json",&json!({"gpu":inv,"cpu_id":cid,"cpu_descripcion":cp,"cpu_logicas":std::thread::available_parallelism()?.get(),"controlador":nv.sys_driver_version()?}))?;
 fase("preparacion_gobernada");let prep=Instant::now();
 let (m,meta,entrada)={let mapped=MappedGGUFModel::from_path(modelo)?;let vocab=mapped.model.vocabulary().ok_or("Vocabulario ausente")?;let bpe=ByteLevelBpe::from_gguf(&mapped.model.metadata,&vocab).map_err(|e|format!("BPE: {e:?}"))?;let entrada=bpe.encode(PROMPT);
  if entrada.len()!=9||vocab.len()!=151936||mapped.model.eos_token_id()!=Some(151645)||mapped.model.decode(&entrada)!=PROMPT{return Err("Tokenización o perfil diferente".into());}
  let m=OwnedQuantizedModel::from_mapped(&mapped)?;(m,mapped.model,entrada)};
 if m.config().num_layers!=36||m.config().hidden_dim!=4096||m.config().num_heads!=32||m.config().num_kv_heads!=8{return Err("Arquitectura diferente".into());}
 let entrada_json=json!({"texto":PROMPT,"tokens":entrada,"tokens_sha256":tokens_sha(&entrada),"modelo_sha256":MODELO,"contexto":64,"max_salida":16,"codificacion":"BPE oficial GGUF; continuación sin plantilla de chat","fin_tokens":[151645,151643]});
 guardar(d,"ENTRADA.json",&entrada_json)?;
 let cpu_us=prep.elapsed().as_micros();let mut g=OwnedQuantizedModelCuda::with_max_seq_len(m,0,64).map_err(|e|format!("GPU rechazada: {e}"))?;let reloj=RelojGpu::nuevo()?;
 comprobar_gpu(&g,&reloj,&nv,&uuid)?;let perfil=serde_json::to_value(g.executor().gpu_profile())?;
 if perfil["q4k"]!="mwv"||perfil["q6k"]!="hw_dp4a"||["hgemm_decode","fp8_prefill","fp8_decode","w4a16_interleaved","fused_gate_up"].iter().any(|k|perfil[*k]!=false){return Err("Perfil GPU no admitido".into());}
 guardar(d,"PREPARACION.json",&json!({"total_us":prep.elapsed().as_micros(),"carga_cpu_us":cpu_us,"perfil":perfil,"gpu":reloj.identidad()?,"paridad":{"estado":g.parity.status,"coseno":g.parity.cosine,"umbral":g.parity.threshold},"vram":g.vram_report()}))?;
 let mut tiempos=[vec![],vec![]];let mut referencia=None;let mut contrastes=vec![];
 for ronda in 0..3{
  let orden=if ronda%2==0{[Clase::Cpu,Clase::Gpu]}else{[Clase::Gpu,Clase::Cpu]};let mut par=vec![];
  for cl in orden{let s=generar(&mut g,&reloj,cl,&entrada,&meta,d,&format!("calibracion-{ronda}-{}",nombre(cl)))?;tiempos[usize::from(cl==Clase::Gpu)].push(s.tiempo);par.push((cl,s));}
  par.sort_by_key(|(cl,_)|*cl);contrastes.push(contrastar(&par[0].1,&par[1].1)?);if ronda==0{referencia=Some(par.remove(0).1);}
 }
 let referencia=referencia.ok_or("Referencia ausente")?;let estimaciones=[mediana(&tiempos[0])?,mediana(&tiempos[1])?];let medido=origen.elapsed().as_millis() as u64;
 guardar(d,"CALIBRACION.json",&json!({"observado_ms":medido,"cpu_us":tiempos[0],"gpu_us":tiempos[1],"medianas_us":estimaciones,"contrastes":contrastes,"vigencia_ms":600000,"condicion":"Misma sesión, entrada y configuración; nueva caché por solicitud"}))?;
 let evidencia=format!("CALIBRACION.json:sha256:{}",hash(&d.join("CALIBRACION.json"))?);let rev=hash(&std::env::current_exe()?)?;
 let destinos=[destino(&cid,&rev),destino(&uuid,&rev)];let car=carga(&entrada);
 let mut director=gobierno::Director::nuevo(entrada_json.clone(),destinos.to_vec(),d)?;
 // Solicitud adulterada y recurso retirado: controles reales del recorrido,
 // separados de las ocho solicitudes de generación, sin invocar el motor.
 let mut negativas=Vec::new();let antes=traza::total();
 let rs=recursos(&car,&destinos,&estimaciones,&evidencia,medido,memoria(&nv,&uuid)?);
 let mut otra=director.peticion(0);otra["entrada"]["texto"]=json!("entrada alterada");
 let r=director.ejecutar(0,&serde_json::to_vec(&otra)?,&rs,origen.elapsed().as_millis() as u64,&car,||Err("No debe revalidar".into()),|_,_|Err("No debe calcular".into()));
 if r.is_ok()||traza::total()!=antes{return Err("No rechazó entrada alterada".into());}negativas.push(json!({"caso":"entrada_alterada","rechazo":r.unwrap_err().to_string(),"lanzamientos_gpu":0}));
 let nd=d.join("negativo-recurso");fs::create_dir(&nd)?;let mut negativo=gobierno::Director::nuevo(entrada_json,destinos.to_vec(),&nd)?;let mut sin_gpu=rs.clone();sin_gpu[1].disponible=false;
 let r=negativo.ejecutar(2,&serde_json::to_vec(&negativo.peticion(2))?,&sin_gpu,origen.elapsed().as_millis() as u64,&car,||Err("No debe revalidar".into()),|_,_|Err("No debe calcular".into()));
 if r.is_ok()||traza::total()!=antes{return Err("Sustituyó GPU retirada".into());}negativas.push(json!({"caso":"gpu_obligatoria_retirada","rechazo":r.unwrap_err().to_string(),"lanzamientos_gpu":0}));negativo.cierre()?;
 let mut solicitudes=vec![];
 for i in 0..8{
  fase(&format!("admision-QAD-{:02}",i+1));let total=Instant::now();comprobar_gpu(&g,&reloj,&nv,&uuid)?;
  let rs=recursos(&car,&destinos,&estimaciones,&evidencia,medido,memoria(&nv,&uuid)?);let raw=serde_json::to_vec(&director.peticion(i))?;
  let mut emision=None;
  let resultado=director.ejecutar(i,&raw,&rs,origen.elapsed().as_millis() as u64,&car,
   ||Ok((origen.elapsed().as_millis() as u64,recursos(&car,&destinos,&estimaciones,&evidencia,medido,memoria(&nv,&uuid)?))),
   |decision,tokens|{comprobar_gpu(&g,&reloj,&nv,&uuid)?;let e=generar(&mut g,&reloj,decision.clase,tokens,&meta,d,&decision.solicitud)?;let v=e.registro.clone();emision=Some(e);Ok(v)})?;
  let transcurrido=total.elapsed().as_micros();let e=emision.ok_or("Ausencia de ejecución")?;let cotejo=contrastar(&referencia,&e)?;
  let v=json!({"arbitro":resultado,"contraste":cotejo,"recorrido_desde_solicitud_us":transcurrido});guardar(d,&format!("QAD-{:02}-CIERRE.json",i+1),&v)?;solicitudes.push(v);
 }
 let rs=recursos(&car,&destinos,&estimaciones,&evidencia,medido,memoria(&nv,&uuid)?);let ln=traza::total();
 let r=director.ejecutar(0,&serde_json::to_vec(&director.peticion(0))?,&rs,origen.elapsed().as_millis() as u64,&car,||Err("No debe revalidar".into()),|_,_|Err("No debe calcular".into()));
 if r.is_ok()||traza::total()!=ln{return Err("Repetición no impedida".into());}negativas.push(json!({"caso":"repeticion_identidad","rechazo":r.unwrap_err().to_string(),"lanzamientos_gpu":0}));director.cierre()?;
 guardar(d,"RECHAZOS.json",&json!({"casos":negativas,"ejecuciones":0}))?;
 fs::write(d.join("BIBLIOTECAS-CARGADAS.txt"),fs::read("/proc/self/maps")?)?;
 Ok(json!({"conforme":true,"nombre":"SV-Aprender","selector":selector::NOMBRE,"nucleo_revision":gobierno::NUCLEO,"selector_sha256":format!("{:x}",Sha256::digest(SELECTOR.as_bytes())),"calibraciones":6,"solicitudes":solicitudes,"negativos":negativas,"alcance":"Perfil experimental del Árbitro - Director con LIG/0.1, Selector y generación Qwen en CPU/T4; no autoridad productiva R1, aptitud del modelo ni validación del nodo"}))
}
pub fn trabajar(m:&Path,d:&Path)->R<()>{
 let monitor=Monitor::iniciar(d)?;let r=ensayo(m,d);let obs=monitor.terminar()?;
 match r{Ok(v)=>{let ok=obs["fallos"]==0&&v["conforme"]==true;guardar(d,"RESULTADO.json",&json!({"conforme":ok,"gobierno":v,"telemetria":obs}))?;if !ok{return Err("Observabilidad no conforme".into());}},Err(e)=>{guardar(d,"RESULTADO.json",&json!({"conforme":false,"error":e.to_string(),"telemetria":obs}))?;return Err(e);}}Ok(())
}
