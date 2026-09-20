//! Banco preparado: atraviesa Axum -> socket -> supervisor -> custodia.
//! Requiere guarda exterior ya armada y curl local. No inicia servicios.
//! Caso y configuración deberán concordar; un intento por lanzamiento.
use eio_candidato::nativa::*;
use serde_json::{Value,json};
use std::{process::Command,time::{Instant,Duration},thread};
fn api(v:Value)->Result<Value,Error>{
 let o=Command::new("curl").args(["--silent","--show-error","--max-time","2","--noproxy","*",
  "--proto","=http","--request","POST","--header","Origin: https://eio.test","--header","Host: eio.test",
  "--header","Content-Type: application/json","--data-binary",&serde_json::to_string(&v)?,
  "http://127.0.0.1:3000/api"]).output()?;
 if !o.status.success(){return Err(format!("CURL {} {}",o.status,String::from_utf8_lossy(&o.stderr)).into())}
 if o.stdout.len()>2*MAX_FRAME{return Err("RESPUESTA_LIMITE".into())}Ok(serde_json::from_slice(&o.stdout)?)
}
fn obtener(nombre:&str,sello:&str)->Result<Vec<u8>,Error>{
 let mut b=Vec::new();loop{
  let v=api(json!({"op":"evidencia","archivo":nombre,"offset":b.len(),"sello":sello}))?;
  if v["sello"]!=sello||v["archivo"]!=nombre||v["offset"].as_u64()!=Some(b.len() as u64){return Err("IDENTIDAD_FRAGMENTO".into())}
  let s=v["hex"].as_str().ok_or("HEX")?;if s.len()%2!=0{return Err("HEX".into())}
  for p in s.as_bytes().chunks_exact(2){let h=std::str::from_utf8(p)?;b.push(u8::from_str_radix(h,16)?);}
  if b.len()>8*1024*1024{return Err("TAMANO".into())}
  if v["fin"]==true{if v["total"].as_u64()!=Some(b.len() as u64){return Err("TOTAL".into())}return Ok(b)}
 }
}
fn main()->Result<(),Error>{
 let args:Vec<_>=std::env::args().collect();if args.len()!=2{return Err("api_real CASO".into())}
 let caso=&args[1];let antes=api(json!({"op":"estado"}))?;
 if antes["estado"]!="ausente"{return Err("SESION_NO_NUEVA".into())}
 let inicio=api(json!({"op":"iniciar","contrato":"EIO-NAT/2","peticion":"referencia","texto":literal("referencia")?}))?;
 let id=inicio["id"].as_str().ok_or("ID")?.to_owned();
 if ["tardio","bloqueo_antes","bloqueo_entre"].contains(&caso.as_str()){
  // Dar margen a testigo y, en bloqueo_antes, a que open(FIFO) quede pendiente.
  thread::sleep(Duration::from_millis(500));
  let r=api(json!({"op":"cancelar","id":id}))?;if r["estado"]!="cancelacion_solicitada"{return Err("CANCELACION".into())}
 }
 // Descarga concurrente con cierre: nunca se acepta un archivo sin sello.
 let concurrente=thread::spawn(||->Result<(),Error>{
  let t=Instant::now();let mut sello_previo:Option<String>=None;
  while t.elapsed()<Duration::from_secs(9){
   let estado=api(json!({"op":"estado"}))?;
   if let Some(s)=estado["sello"].as_str(){
    if sello_previo.as_ref().is_some_and(|p|p!=s){return Err("SELLO_MUTABLE".into())}
    sello_previo=Some(s.into());
    let v=api(json!({"op":"evidencia","archivo":"MANIFIESTO.json","offset":0,"sello":s}))?;
    if v.get("error").is_none() && (v["sello"]!=s||v["fin"]!=true){return Err("RECUPERACION_CONCURRENTE".into())}
   }else{
    let v=api(json!({"op":"evidencia","archivo":"MANIFIESTO.json","offset":0,"sello":""}))?;
    if v.get("error").is_none(){return Err("EVIDENCIA_SIN_SELLAR".into())}
   }
   thread::sleep(Duration::from_millis(50));
  }Ok(())
 });
 let t=Instant::now();let mut finalv=Value::Null;
 while t.elapsed()<Duration::from_secs(12){
  let v=api(json!({"op":"estado"}))?;
  if v["admisible"]!=true && !v["resultado"].is_null(){return Err("RESULTADO_INADMISIBLE_EXPUESTO".into())}
  if ["terminada","interrumpida","desconocida"].contains(&v["estado"].as_str().unwrap_or("")){
   finalv=v;break
  }thread::sleep(Duration::from_millis(50));
 }
 if finalv.is_null(){return Err("CIERRE_NO_OBSERVADO".into())}
 let positivo=["normal","hostil","cola_pendiente"].contains(&caso.as_str());
 if finalv["admisible"]!=positivo{return Err("ADMISIBILIDAD".into())}
 if caso=="hostil"{
  if finalv["resultado"]["juicio_verificador"]!="ESTRUCTURA" ||
     !finalv["resultado"]["salida_original"]["texto"].as_str().unwrap_or("").contains("<script>"){return Err("ORIGINAL_RECHAZADO_OCULTO".into())}
 }
 if let Some(sello)=finalv["sello"].as_str(){
  let mut antes=Vec::new();for n in FILES{antes.push((n,obtener(n,sello)?));}
  // EOF ausente: el hijo emite al segundo 4, después del corte de drenaje 2 s.
  thread::sleep(Duration::from_secs(5));
  for (n,b) in &antes{let despues=obtener(n,sello)?;if *b!=despues{return Err("CONJUNTO_SELLADO_MUTADO".into())}}
  let m:&Vec<u8>=&antes.iter().find(|(n,_)|*n=="MANIFIESTO.json").ok_or("MANIFIESTO")?.1;
  if hash(m)!=sello{return Err("SELLO_SHA".into())}
  let man:Value=serde_json::from_slice(m)?;
  for item in man["archivos"].as_array().ok_or("INVENTARIO")?{
   let n=item["archivo"].as_str().ok_or("NOMBRE")?;
   let b=&antes.iter().find(|(p,_)|*p==n).ok_or("FALTA")?.1;
   if item["bytes"].as_u64()!=Some(b.len() as u64)||item["sha256"]!=hash(b){return Err("IDENTIDAD".into())}
  }
 }else if positivo{return Err("SELLO_AUSENTE".into())}
 // No segundo intento, incluso tras cierre desconocido.
 let segundo=api(json!({"op":"iniciar","contrato":"EIO-NAT/2","peticion":"referencia","texto":literal("referencia")?}))?;
 if segundo["error"]!="SESION_CONSUMIDA_O_BLOQUEADA"{return Err("SEGUNDO_INTENTO".into())}
 concurrente.join().map_err(|_|"HILO_BANCO")??;
 println!("{}",json!({"caso":caso,"conforme":true,"estado":finalv,"alcance":"banco sintético mediante HTTP real; no modelo ni SV"}));
 Ok(())
}
