//! Consulta administrativa de modelo y saldo; no genera inferencia.
#![forbid(unsafe_code)]
use sv_cliente_api::{guard,need,save,put,parse,R};
use std::{path::PathBuf,fs,io::Read,time::{Duration,Instant}};
use serde_json::json;
use zeroize::Zeroizing;
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1)}}
fn run()->R<()>{
 let a=std::env::args().collect::<Vec<_>>();need(a.len()==2,"Uso: DESTINO_NUEVO")?;let out=PathBuf::from(&a[1]);guard(&out)?;need(!out.exists(),"Destino existente")?;
 let path=PathBuf::from(std::env::var("SV_API_KEY_FILE").map_err(|_|"Ruta de clave ausente")?);guard(&path)?;need(path.starts_with("C:/SV/privado"),"Sede de credencial no privada")?;
 let key=Zeroizing::new(fs::read_to_string(path).map_err(|_|"Credencial no accesible")?);need(key.len()>24&&!key.chars().any(char::is_whitespace),"Credencial no recibida")?;
 fs::create_dir_all(&out).map_err(|e|e.to_string())?;
 let monitor=sv_instrumentacion::Monitor::start_bounded(&out.join("instrumentacion"),90)?;
 let http=reqwest::blocking::Client::builder().https_only(true).no_proxy().redirect(reqwest::redirect::Policy::none()).retry(reqwest::retry::never()).timeout(Duration::from_secs(25)).build().map_err(|_|"Cliente HTTPS")?;
 for (name,url)in [("MODELOS","https://api.moonshot.ai/v1/models"),("SALDO","https://api.moonshot.ai/v1/users/me/balance")]{
  let start=Instant::now();monitor.event("consulta_administrativa",json!({"operacion":name,"metodo":"GET"}))?;
  let r=http.get(url).bearer_auth(key.as_str()).send().map_err(|_|"Consulta interrumpida")?;let code=r.status().as_u16();let headers=r.headers().iter().filter_map(|(k,v)|{let n=k.as_str();if n=="date"||n=="retry-after"||n.starts_with("x-ratelimit-"){v.to_str().ok().map(|s|(n.to_owned(),json!(s)))}else{None}}).collect::<serde_json::Map<String,serde_json::Value>>();let mut b=Vec::new();r.take(1024*1024+1).read_to_end(&mut b).map_err(|_|"Recepción interrumpida")?;need(b.len()<=1024*1024,"Respuesta excesiva")?;put(&out.join(format!("{name}-ORIGINAL.json")),&b)?;
  save(&out.join(format!("{name}-HTTP.json")),&json!({"http":code,"duracion_ms":start.elapsed().as_millis(),"utc_ms":sv_instrumentacion::utc_ms(),"inferencias":0,"cabeceras_limites_comunicadas":headers,"alcance_cabeceras":"Metadatos de la consulta administrativa; su presencia no acredita por sí sola el remanente TPD de inferencia"}))?;
  need(code==200,"Consulta rechazada; argumento conservado")?;let v=parse(&b)?;
  if name=="MODELOS"{let ids=v["data"].as_array().ok_or("Catálogo ausente")?.iter().filter_map(|x|x["id"].as_str()).collect::<Vec<_>>();need(ids.contains(&"kimi-k3"),"K3 no figura en catálogo")?;println!("Modelos declarados: {}",ids.join(", "));}
  else {need(v["status"]==true&&v["data"]["available_balance"].as_f64().is_some_and(|n|n>0.),"Saldo no recibido")?;println!("Saldo USD declarado: {}",v["data"]["available_balance"]);}
 }
 let t=monitor.finish()?;save(&out.join("RECEPCION.json"),&json!({"conforme":true,"inferencias":0,"telemetria":t,"licencia":sv_cliente_api::LICENCIA}))?;Ok(())
}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
