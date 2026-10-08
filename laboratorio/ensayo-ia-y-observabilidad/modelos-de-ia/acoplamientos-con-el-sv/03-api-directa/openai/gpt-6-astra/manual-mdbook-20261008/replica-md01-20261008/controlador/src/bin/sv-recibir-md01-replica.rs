//! Recepción exterior al candidato: integridad, medidas y adjudicación documental.
#![forbid(unsafe_code)]
#[path="../catalogo/estricto.rs"]mod estricto;
#[path="../catalogo/recepcion.rs"]mod recepcion;
#[path="astra-md01-replica/contrato.rs"]mod contrato;
#[path="astra-manual/localizadores.rs"]mod localizadores;
use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs::{self,OpenOptions},io::Write,path::{Path,PathBuf}};
type R<T>=Result<T,String>;
mod suministro_pdf{pub type R<T>=Result<T,String>;pub fn need(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}pub fn parse(b:&[u8])->R<serde_json::Value>{super::estricto::parse(b).map_err(|e|e.to_string())}pub fn sha(b:&[u8])->String{super::sha(b)}pub fn num(v:&serde_json::Value)->R<usize>{v.as_u64().and_then(|n|usize::try_from(n).ok()).ok_or("Entero inválido".into())}}
use suministro_pdf::{need,parse};
#[path="md01-dictamen/mod.rs"]mod dictamen;

const PIE:&str="© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
fn e(v:impl std::fmt::Display)->String{v.to_string()}
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn read(p:&Path)->R<Vec<u8>>{let canonical=p.canonicalize().map_err(e)?;let cwd=std::env::current_dir().map_err(e)?.canonicalize().map_err(e)?;need(canonical.starts_with(cwd),"Fuera de perímetro")?;let m=fs::metadata(&canonical).map_err(e)?;need(m.is_file()&&m.len()<=64*1024*1024,"Archivo excesivo o no regular")?;fs::read(canonical).map_err(e)}
fn load(p:&Path)->R<Value>{parse(&read(p)?)}
fn save(p:&Path,v:&Value)->R<()>{let bytes=serde_json::to_vec_pretty(v).map_err(e)?;if p.exists(){return need(read(p)?==bytes,"Derivado previo distinto");}fs::create_dir_all(p.parent().ok_or("Padre")?).map_err(e)?;let mut f=OpenOptions::new().write(true).create_new(true).open(p).map_err(e)?;f.write_all(&bytes).and_then(|_|f.sync_all()).map_err(e)}
fn events(b:&[u8])->R<Vec<Value>>{let mut out=vec![];let mut done=false;for l in b.split(|b|*b==b'\n'){let s=std::str::from_utf8(l).map_err(e)?.trim_end_matches('\r');if let Some(d)=s.strip_prefix("data:"){if d.trim()=="[DONE]"{need(!done&&out.last().is_some_and(|v:&Value|v["type"]=="response.completed"),"Final incorrecto")?;done=true;continue;}need(!done,"Evento posterior")?;let v=parse(d.trim().as_bytes())?;need(v["sequence_number"]==out.len(),"Secuencia distinta")?;out.push(v);}else{need(s.is_empty()||s.starts_with(':')||s.starts_with("event:"),"SSE ajeno")?;}}Ok(out)}
fn receive(base:&Path)->R<Value>{
 let root=std::env::current_dir().map_err(e)?;let prev=load(&base.join("PREVIA.json"))?;need(prev["conforme"]==true,"Sin admisión")?;
 for f in prev["archivos_fijados"].as_array().ok_or("Fijación")?{let b=read(&root.join(f["ruta"].as_str().ok_or("Ruta")?))?;need(f["sha256"]==sha(&b)&&f["bytes"]==b.len(),"Fijación alterada")?;}
 let bank=load(&base.join("RESULTADO-BANCO.json"))?;let mut hits=fs::read_dir(base.join("hitos")).map_err(e)?.map(|entry|entry.map(|x|x.path()).map_err(e)).collect::<R<Vec<_>>>()?;hits.sort();
 let mut rows=vec![];let mut successful=std::collections::BTreeMap::<usize,String>::new();let mut known=[0u64;3];let mut unknown=0;let mut samples=0;let mut maxinterval=0;let mut operations=0;
 for (i,p) in hits.iter().enumerate(){let h=load(&p.join("HITO-INSTRUMENTAL.json"))?;need(h["numero_intento"]==i+1,"Intentos discontinuos")?;let original=root.join(h["directorio"].as_str().ok_or("Directorio")?);
 for f in h["originales"].as_array().ok_or("Identidades")?{let b=read(&root.join(f["ruta"].as_str().ok_or("Ruta")?))?;need(sha(&b)==f["sha256"]&&f["bytes"]==b.len(),"Original alterado")?;}
 let r=load(&original.join("RESULTADO.json"))?;let tele=sv_instrumentacion::verify(&original.join("instrumentacion/telemetria.jsonl"))?;need(tele==r["telemetria"],"Medida distinta")?;
 samples+=tele["muestras"].as_u64().unwrap_or(0);maxinterval=maxinterval.max(tele["intervalo_maximo_ms"].as_u64().unwrap_or(0));operations+=r["duracion_operacion_ms"].as_u64().ok_or("Duración")?;
 let n=h["entrega_numero"].as_u64().ok_or("Número")? as usize;need((1..=3).contains(&n),"Entrega ajena")?;let q=(n-1)/3+1;let stage=(n-1)%3;let baseline=load(&base.join(format!("fuentes-admitidas/MD{q:02}.json")))?;
 let history=((q-1)*3+1..n).map(|k|successful.get(&k).cloned().ok_or("Historia ausente".into())).collect::<R<Vec<_>>>()?;
 let request=load(&original.join("SOLICITUD.json"))?;need(request==contrato::compose(&baseline,stage,&history)?,"Solicitud fuera del contrato")?;
 let mut got=Value::Null;let mut audit=Value::Null;let mut finalhash=Value::Null;
 if r["completa"]==true{need(r["telemetria_conforme"]==true,"Entrega sin medición")?;got=recepcion::extract(&events(&read(&original.join("SALIDA-SSE.txt"))?)?)?;let text=read(&original.join("FINAL.txt"))?;need(got==load(&original.join("ENTREGA-PROVEEDOR.json"))?&&got["texto_original"]==std::str::from_utf8(&text).map_err(e)?,"Texto o eventos discordantes")?;
 audit=match parse(&text).and_then(|v|contrato::formal(&v,&request)){Ok(v)=>v,Err(e)=>json!({"conforme":false,"defecto":e})};need(audit==load(&original.join("AUDITORIA-FORMAL.json"))?,"Auditoría discordante")?;
 need(successful.insert(n,std::str::from_utf8(&text).map_err(e)?.to_string()).is_none(),"Entrega completa repetida")?;finalhash=json!(sha(&text));need(got["uso_proveedor"]==r["uso_proveedor"],"Uso distinto")?;
 }
 let usage=&r["uso_proveedor"];if let (Some(a),Some(b),Some(c))=(usage["input_tokens"].as_u64(),usage["output_tokens"].as_u64(),usage["total_tokens"].as_u64()){need(a+b==c,"Tokens discordantes")?;known[0]+=a;known[1]+=b;known[2]+=c;}else{unknown+=1;}
 rows.push(json!({"caso":format!("MD{q:02}"),"etapa":stage,"intento":i+1,"entrega":n,"directorio":h["directorio"],"completa":r["completa"],"formal_conforme":audit["conforme"],"final_sha256":finalhash,"solicitud_sha256":sha(&read(&original.join("SOLICITUD.json"))?),"resultado_sha256":sha(&read(&original.join("RESULTADO.json"))?),"uso_proveedor":r["uso_proveedor"],"telemetria":tele,"duracion_ms":r["duracion_ms"],"duracion_operacion_ms":r["duracion_operacion_ms"],"http":r["http"],"respuesta":got["texto_original"],"resumen_proveedor":got["resumen_proveedor"],"adjudicacion":false}));
 }
 need(rows.len()==bank["inferencias_iniciadas"].as_u64().ok_or("Intentos")? as usize,"Banco y hitos distintos")?;
 let out=json!({"conforme":true,"completo":successful.len()==3,"intentos":rows.len(),"entregas":successful.len(),"casos":rows,"tokens_entrada_conocidos":known[0],"tokens_salida_conocidos":known[1],"tokens_totales_conocidos":known[2],"intentos_uso_desconocido":unknown,"muestras":samples,"intervalo_maximo_ms":maxinterval,"duracion_operaciones_ms":operations,"duracion_banco_ms":bank["duracion_banco_ms"],"credito_atribuible":null,"importe_atribuible":null,"adjudicacion":false,"autoria_licencia":PIE});save(&base.join("RECEPCION-RUST.json"),&out)?;Ok(out)
}
fn main(){let base=PathBuf::from("ejecucion/astra-md01-replica-20261008");match receive(&base).and_then(|v|dictamen::cerrar(&base,&v)){Ok(v)=>println!("{v}"),Err(e)=>{eprintln!("{e}");std::process::exit(1)}}}
