//! Recepción de exportaciones comparativas, sin inferencia ni modificación de originales.
use serde_json::{json,Value};
use std::{collections::BTreeSet,env,fs,path::Path};
type Result<T> = std::result::Result<T,Box<dyn std::error::Error>>;
fn read(p:&Path)->Result<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn main()->Result<()>{
 let a:Vec<_>=env::args().collect();
 if a.len()!=2{return Err("Uso: resumir-comparacion DIRECTORIO".into())}
 let dir=Path::new(&a[1]);let input=read(&dir.join("ENTRADAS.json"))?;
 let lines=fs::read_to_string(dir.join("RESULTADOS.jsonl"))?;
 let records:Vec<Value>=lines.lines().map(serde_json::from_str).collect::<std::result::Result<_,_>>()?;
 let mut labels=BTreeSet::new();let mut ids=BTreeSet::new();let mut results=vec![];
 let mut matched=0;let mut pending=vec![];
 for bank in input["banks"].as_array().ok_or("Sin rondas")?{
  let bid=bank["id"].as_str().ok_or("Sin identificador")?;
  let regular=format!("{bid}-EXPEDIENTE.json");
  let checkpoint=read(&dir.join(&regular))?;
  let mut file=regular.clone();let mut export=checkpoint.clone();let mut found=0;
  for entry in fs::read_dir(dir)?{
   let p=entry?.path();let name=p.file_name().and_then(|s|s.to_str()).unwrap_or("");
   if name.starts_with("CONSERVACION-")&&name.ends_with(".json"){
    let v=read(&p)?;
    if v["case"]["id"]==checkpoint["case"]["id"]{
     found+=1;if found>1{return Err("Conservación final ambigua".into())}
     let before=checkpoint["chats"][0]["turns"].as_array().ok_or("Corte previo sin turnos")?;
     let after=v["chats"][0]["turns"].as_array().ok_or("Conservación sin turnos")?;
     if after.len()<before.len()||before.iter().zip(after).any(|(a,b)|a!=b){
      return Err("La conservación no prolonga íntegramente el corte previo".into())
     }
     file=name.to_owned();export=v;
    }
   }
  }
  let chats=export["chats"].as_array().ok_or("Sin conversaciones")?;
  if chats.len()!=1{return Err("Exportación ambigua".into())}
  let turns=chats[0]["turns"].as_array().ok_or("Sin turnos")?;
  let tasks=bank["turns"].as_array().ok_or("Sin preguntas")?;
  if turns.len()>tasks.len(){return Err("Sobran turnos".into())}
  for(i,task)in tasks.iter().enumerate(){
   let label=task["id"].as_str().ok_or("Sin condición")?;
   if !labels.insert(label.to_owned()){return Err("Condición duplicada".into())}
   let Some(turn)=turns.get(i)else{pending.push(label);continue};
   if turn["user"]!=task["user"]{return Err(format!("Pregunta distinta: {label}").into())}
   let id=turn["id"].as_str().ok_or("Sin ID de petición")?;
   if !ids.insert(id.to_owned()){return Err("ID de petición duplicado".into())}
   let associated:Vec<_>=records.iter().filter(|r|r["case"]==label).collect();
   if associated.len()>1{return Err(format!("Registro duplicado: {label}").into())}
   let inherited=bid=="R02"&&i==0;
   if let Some(r)=associated.first(){
    if r["source"]!=*task||r["turn"]!=*turn{return Err(format!("Originales divergentes: {label}").into())}
    matched+=1;
   }else if inherited{
    let prior=read(&dir.join("ANTECEDENTE.json"))?;
    if prior["chats"][0]["turns"][0]!=*turn{return Err("Antecedente divergente".into())}
   }else{return Err(format!("Turno sin registro del controlador: {label}").into())}
   results.push(json!({"condicion":label,"peticion":id,"pregunta":task["user"],
    "qwen":{"respuesta":task["qwen_answer"],"estado":task["qwen_status"],"segundos":task["qwen_seconds"],"entrada_tokens":task["qwen_input_tokens"],"perfil":task["original_profile"]},
    "gpt_oss":{"respuesta":turn["answer"],"estado":turn["status"],"segundos":turn["result"]["seconds"],"entrada_tokens":turn["context"]["input_tokens"],"salida_tokens":turn["result"]["tokens"],"segundos_entrada_motor":turn["result"]["phase_timings"]["engine_response"]["usage"]["total_prompt_time_sec"],"segundos_salida_motor":turn["result"]["phase_timings"]["engine_response"]["usage"]["total_completion_time_sec"],"perfil":turn["profile"],"error":turn["result"]["error"],"trace_id":turn["result"]["observability"]["trace_id"],"exportacion":turn["result"]["observability"]["export"]},
    "procedencia":file,"antecedente_conservado":inherited,"cotejo_literal":true}));
  }
 }
 if matched!=records.len(){return Err("Existen registros no conciliados".into())}
 println!("{}",serde_json::to_string_pretty(&json!({"esquema":"EIO-RECEPCION-COMPARACION-1","condiciones_previstas":labels.len(),"turnos_recibidos":results.len(),"registros_controlador_conciliados":matched,"no_recibidas":pending,"resultados":results,"alcance":"Identidad, orden y correspondencia estructural de los originales recibidos. No evalúa exactitud semántica ni custodia independiente."}))?);
 Ok(())
}

