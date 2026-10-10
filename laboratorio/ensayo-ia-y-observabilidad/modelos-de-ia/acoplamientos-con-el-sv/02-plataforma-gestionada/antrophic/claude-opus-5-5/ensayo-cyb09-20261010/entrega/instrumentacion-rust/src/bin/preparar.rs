#![forbid(unsafe_code)]
use sv_claude_kaggle::{self as sv,need,parse,sha,R,suministro::ContratoLibro};
use serde_json::{json,Value};
use std::{fs,path::Path,io::Write};
const ROOT:&str="C:/laboratorio/watson-local/lenguaje-computacion-sv";
fn read(p:&Path)->R<Value>{parse(&fs::read(p).map_err(|e|e.to_string())?)}
fn put(p:&Path,v:&Value)->R<()>{let mut f=fs::OpenOptions::new().create_new(true).write(true).open(p).map_err(|e|e.to_string())?;f.write_all(&serde_json::to_vec_pretty(v).map_err(|e|e.to_string())?).and_then(|_|f.sync_all()).map_err(|e|e.to_string())}
fn copy(from:&Path,to:&Path)->R<()>{need(!to.exists(),"Destino ya presente")?;fs::create_dir_all(to.parent().ok_or("Sin padre")?).map_err(|e|e.to_string())?;fs::copy(from,to).map_err(|e|e.to_string())?;need(sha(&fs::read(from).map_err(|e|e.to_string())?)==sha(&fs::read(to).map_err(|e|e.to_string())?),"Copia discordante")}
fn run()->R<()>{
 let root=Path::new(ROOT);let dest=root.join("ejecucion/claude-kaggle-20261009");need(!dest.exists(),"Preparación ya iniciada; conservar antes de continuar")?;fs::create_dir(&dest).map_err(|e|e.to_string())?;
 let mut banks=vec![];
 for (name,source,template) in [("manual", "ejecucion/astra-manual-20261008", "ejecucion/kimi-manual-20261008-r2/CONTRATO-LIBRO.json"),("cyb16","ejecucion/astra-cyb16-20261008-r2","ejecucion/astra-cyb16-20261008-r2/CONTRATO-LIBRO.json")]{
  let origin=root.join(source);let target=dest.join(name);fs::create_dir_all(target.join("fuentes/preparado")).map_err(|e|e.to_string())?;fs::create_dir(target.join("reservado")).map_err(|e|e.to_string())?;
  for f in ["fuentes/preparado/CATALOGO.json","fuentes/preparado/FUENTES.json","fuentes/BANCO.json","reservado/CLAVE.json"]{copy(&origin.join(f),&target.join(f))?;}
  let mut c:ContratoLibro=serde_json::from_value(read(&root.join(template))?).map_err(|e|e.to_string())?;
  c.banco_sha256=sha(&fs::read(target.join("fuentes/BANCO.json")).map_err(|e|e.to_string())?);c.clave_sha256=sha(&fs::read(target.join("reservado/CLAVE.json")).map_err(|e|e.to_string())?);c.modelo=Some(sv::MODELO.into());
  put(&target.join("CONTRATO-LIBRO.json"),&serde_json::to_value(&c).map_err(|e|e.to_string())?)?;
  let supply=sv::suministro::preparar_contrato(&target,&c)?;sv::suministro::verificar_contrato(&target,&c)?;
  let mut cases=vec![];
  for i in 1..=c.preguntas{let id=format!("{}{i:02}",c.prefijo);let base=read(&target.join(format!("fuentes-admitidas/{id}.json")))?;for s in 0..3{let h=(0..s).map(|_|"Antecedente sintético de comprobación, no respuesta real".to_owned()).collect::<Vec<_>>();sv::compose(&base,s,&h)?;}cases.push(json!({"id":id,"base":base}));}
  // Sólo bases admitidas: no se incorpora clave, criticidad ni resultado previo al paquete remoto.
  banks.push(json!({"id":name,"casos":cases,"suministro":supply}));
 }
 let payload=json!({"version":"claude-kaggle-1.0","modelo":sv::MODELO,"bancos":banks,"max_tokens":16384,"razonamiento":"high","reintentos_automaticos":0,"limite_banco_s":5400,"cuota_diaria_usd":10,"licencia":sv::LICENCIA});
 put(&dest.join("PAQUETE-CANDIDATO.json"),&payload)?;put(&dest.join("PREPARACION-RUST.json"),&json!({"conforme":true,"inferencias":0,"entregas_previstas":75,"paquete_sha256":sha(&fs::read(dest.join("PAQUETE-CANDIDATO.json")).map_err(|e|e.to_string())?),"clave_en_paquete":false,"gemini_excluido":true,"recepcion_transporte_remoto":"pendiente"}))?;
 println!("Preparación documental conforme; 25 casos, 75 composiciones; cero inferencias");Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("IMPEDIMENTO: {e}");std::process::exit(1)}}
