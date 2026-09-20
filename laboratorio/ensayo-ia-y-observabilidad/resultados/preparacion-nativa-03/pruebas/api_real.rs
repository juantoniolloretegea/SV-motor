//! Banco HTTP preparado, no ejecutado. No inicia servidor ni guarda.
//! Una sesión nueva por caso exacto; errores de transporte NO son conformidad.
use eio_candidato::{nativa::*,oraculos_nat03 as o};
use serde_json::{Value,json};
use std::{process::Command,time::{Instant,Duration},thread};
fn api(v:Value,fin:Instant)->Result<Value,Error>{
 let restante=fin.checked_duration_since(Instant::now()).filter(|x|!x.is_zero()).ok_or("PLAZO_GLOBAL")?;
 let limite=restante.min(Duration::from_secs(2)).as_secs_f64().to_string();
 let salida=Command::new("curl").args(["--silent","--show-error","--max-time",&limite,"--noproxy","*",
  "--proto","=http","--request","POST","--header","Origin: https://eio.test","--header","Host: eio.test",
  "--header","Content-Type: application/json","--data-binary",&serde_json::to_string(&v)?,
  "http://127.0.0.1:3000/api"]).output()?;
 if Instant::now()>=fin{return Err("PLAZO_GLOBAL".into())}
 if !salida.status.success(){return Err(format!("API_INDISPONIBLE curl={} stderr={}; se exige evidencia exterior, no conformidad",salida.status,String::from_utf8_lossy(&salida.stderr)).into())}
 if salida.stdout.len()>2*MAX_FRAME{return Err("RESPUESTA_LIMITE".into())}
 Ok(serde_json::from_slice(&salida.stdout)?)
}
fn obtener(nombre:&str,sello:&str,p:&mut o::Presupuesto)->Result<Vec<u8>,Error>{
 let mut bytes=Vec::new();let mut total=None;
 loop{
  p.llamada()?;
  let v=api(json!({"op":"evidencia","archivo":nombre,"offset":bytes.len(),"sello":sello}),p.fin)?;
  let (b,fin)=o::fragmento(&v,nombre,sello,bytes.len(),&mut total,p)?;
  bytes.extend_from_slice(&b);if fin{return Ok(bytes)}
 }
}
fn pausa(d:Duration,fin:Instant)->Result<(),Error>{
 if fin.checked_duration_since(Instant::now()).is_none_or(|x|x<=d){return Err("PAUSA_FUERA_DE_PLAZO".into())}
 thread::sleep(d);if Instant::now()>=fin{return Err("PLAZO_GLOBAL".into())}Ok(())
}
fn main()->Result<(),Error>{
 let args:Vec<_>=std::env::args().collect();if args.len()!=2{return Err("api-real CASO".into())}
 let c=o::caso(&args[1])?; // Rechazo anterior a cualquier acceso HTTP.
 let fin=Instant::now()+Duration::from_secs(60);
 let antes=api(json!({"op":"estado"}),fin)?;o::identidad(&c,&antes)?;
 if antes["estado"]!="ausente"{return Err("SESION_NO_NUEVA".into())}
 let inicio=api(json!({"op":"iniciar","contrato":"EIO-NAT/2","peticion":"referencia","texto":literal("referencia")?}),fin)?;
 let id=inicio["id"].as_str().ok_or("ID")?.to_owned();
 if ["tardio","bloqueo_antes","bloqueo_entre"].contains(&c.caso.as_str()){
  pausa(Duration::from_millis(500),fin)?;
  let r=api(json!({"op":"cancelar","id":id}),fin)?;
  if r["estado"]!="cancelacion_solicitada"{return Err("CANCELACION".into())}
 }
 // Un presupuesto para TODA la recuperación, incluyendo consultas concurrentes,
 // ambas pasadas de los cinco archivos y la pausa de estabilidad.
 let mut p=o::Presupuesto::nuevo(fin);
 let limite_cierre=Instant::now()+Duration::from_secs(12);
 let mut sello_previo:Option<String>=None;let mut prefijo_manifesto:Option<Vec<u8>>=None;let mut total_manifesto=None;
 let finalv=loop{
  if Instant::now()>=limite_cierre{return Err("CIERRE_NO_OBSERVADO".into())}
  let v=api(json!({"op":"estado"}),p.fin)?;o::identidad(&c,&v)?;
  if v["admisible"]!=true&&!v["resultado"].is_null(){return Err("RESULTADO_INADMISIBLE_EXPUESTO".into())}
  // Esta recuperación transcurre mientras el supervisor drena/sella.
  p.llamada()?;
  if let Some(s)=v["sello"].as_str(){
   if !c.sello{return Err("SELLO_PROHIBIDO".into())}
   if sello_previo.as_ref().is_some_and(|x|x!=s){return Err("SELLO_MUTABLE".into())}sello_previo=Some(s.into());
   let r=api(json!({"op":"evidencia","archivo":"MANIFIESTO.json","offset":0,"sello":s}),p.fin)?;
   let (b,_)=o::fragmento(&r,"MANIFIESTO.json",s,0,&mut total_manifesto,&mut p)?;
   if prefijo_manifesto.as_ref().is_some_and(|x|*x!=b){return Err("PREFIJO_MUTABLE".into())}prefijo_manifesto=Some(b);
  }else{
   let r=api(json!({"op":"evidencia","archivo":"MANIFIESTO.json","offset":0,"sello":""}),p.fin)?;
   if r["error"]=="SELLO_DISTINTO"{
    // Única carrera permitida: se selló entre Estado y Evidencia.
    let despues=api(json!({"op":"estado"}),p.fin)?;o::identidad(&c,&despues)?;
    if !c.sello||despues["sello"].as_str().is_none(){return Err("ERROR_SIN_TRANSICION_DE_FASE".into())}
   }else{o::error_no_sellado(&r)?;}
  }
  if ["terminada","interrumpida","desconocida"].contains(&v["estado"].as_str().unwrap_or("")){
   o::terminal(&c,&v)?;break v;
  }
  if v["estado"]=="parada_no_confirmada"{return Err("PARADA_NO_CONFIRMADA: requiere cgroup.events, waitpid y acta exterior; no conforme".into())}
  pausa(Duration::from_millis(50),p.fin)?;
 };
 if let Some(sello)=finalv["sello"].as_str(){
  let mut originales=Vec::new();for n in FILES{originales.push((n,obtener(n,sello,&mut p)?));}
  o::conjunto(&c,&originales,&id,sello)?;
  if c.admisible{
   let out=&originales.iter().find(|(n,_)|*n=="inferidor.jsonl").ok_or("ORIGINAL")?.1;
   let linea=out.split(|x|*x==b'\n').filter(|l|!l.is_empty()).last().ok_or("RESULTADO")?;
   let frame:Frame=serde_json::from_slice(linea)?;
   if finalv["resultado"]!=frame.datos{return Err("RESULTADO_API_DISTINTO".into())}
  }
  if let Some(prefijo)=prefijo_manifesto{let m=&originales.iter().find(|(n,_)|*n=="MANIFIESTO.json").ok_or("MANIFIESTO")?.1;if total_manifesto!=Some(m.len())||!m.starts_with(&prefijo){return Err("PREFIJO_NO_CONSERVADO".into())}}
  pausa(Duration::from_secs(5),p.fin)?;
  for (n,b) in &originales{if *b!=obtener(n,sello,&mut p)?{return Err("CONJUNTO_SELLADO_MUTADO".into())}}
 }else{
  p.llamada()?;
  let r=api(json!({"op":"evidencia","archivo":"MANIFIESTO.json","offset":0,"sello":""}),p.fin)?;
  if r!=json!({"error":c.error_recuperacion}){return Err("ERROR_DE_CUSTODIA_DISTINTO".into())}
 }
 let segundo=api(json!({"op":"iniciar","contrato":"EIO-NAT/2","peticion":"referencia","texto":literal("referencia")?}),fin)?;
 if segundo["error"]!="SESION_CONSUMIDA_O_BLOQUEADA"{return Err("SEGUNDO_INTENTO".into())}
 println!("{}",json!({"caso":c.caso,"conforme_banco_api":true,"estado":finalv,
  "fragmentos_recuperacion":p.fragmentos,"bytes_recuperacion":p.bytes,
  "alcance":"testigo sintético; falta cotejo exterior de parada y recuperación receptora; no modelo ni SV"}));Ok(())
}
