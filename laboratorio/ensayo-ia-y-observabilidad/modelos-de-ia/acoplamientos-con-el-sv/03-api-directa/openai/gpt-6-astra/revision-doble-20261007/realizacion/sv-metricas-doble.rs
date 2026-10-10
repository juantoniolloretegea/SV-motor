//! Derivación documental fuera del candidato; no contiene transporte ni inferencia.
#![forbid(unsafe_code)]
#[path="../catalogo/estricto.rs"] mod estricto;
#[path="../catalogo/recepcion.rs"] mod recepcion;
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs::{self,OpenOptions},io::Write,path::{Path,PathBuf},collections::BTreeSet};
type R<T>=Result<T,String>;
const ROOT:&str="C:/SV-LABORATORIO";
const PDF:&str="ejecucion/astra-pdf-doble-20261007";
const PIE:&str="© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
fn e(x:impl std::fmt::Display)->String{x.to_string()}
fn check(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn path(s:&str)->PathBuf{Path::new(ROOT).join(s)}
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn parse(b:&[u8])->R<Value>{estricto::parse(b).map_err(e)}
fn load(p:&Path)->R<Value>{parse(&fs::read(p).map_err(e)?)}
fn put(p:&Path,b:&[u8])->R<()>{
    let rel=p.strip_prefix(ROOT).map_err(e)?;let mut x=PathBuf::from(ROOT);
    for c in rel.components(){check(matches!(c,std::path::Component::Normal(_)),"Ruta no normal")?;x.push(c);if x.exists(){let m=fs::symlink_metadata(&x).map_err(e)?;check(!m.file_type().is_symlink(),"Enlace no admitido")?;#[cfg(windows)]{use std::os::windows::fs::MetadataExt;check(m.file_attributes()&0x400==0,"Reanálisis no admitido")?;}}}
    fs::create_dir_all(p.parent().ok_or("Padre")?).map_err(e)?;let mut f=OpenOptions::new().write(true).create_new(true).open(p).map_err(e)?;f.write_all(b).and_then(|_|f.sync_all()).map_err(e)
}
fn save(p:&Path,v:&Value)->R<()>{put(p,&serde_json::to_vec_pretty(v).map_err(e)?)}
fn num(v:&Value)->R<u64>{v.as_u64().ok_or("Entero requerido".into())}
fn identity(p:&Path)->R<Value>{let b=fs::read(p).map_err(e)?;Ok(json!({"ruta":p.strip_prefix(ROOT).map_err(e)?.to_string_lossy(),"bytes":b.len(),"sha256":sha(&b)}))}
fn files(p:&Path)->R<Vec<Value>>{let mut a=vec![];for d in fs::read_dir(p).map_err(e)?{let p=d.map_err(e)?.path();if p.is_dir(){a.extend(files(&p)?);}else{a.push(identity(&p)?);}}a.sort_by_key(|x|x["ruta"].as_str().unwrap().to_string());Ok(a)}
fn metrics(p:&Path)->R<Value>{
    let r=load(&p.join("RESULTADO.json"))?;let t=sv_instrumentacion::verify(&p.join("instrumentacion/telemetria.jsonl"))?;check(t==r["telemetria"],"Telemetría alterada")?;
    let raw=fs::read(p.join("SALIDA-SSE.txt")).map_err(e)?;
    let events=raw.split(|b|*b==b'\n').filter_map(|l|l.strip_prefix(b"data:")).filter(|l|!String::from_utf8_lossy(l).trim().eq("[DONE]")).map(parse).collect::<R<Vec<_>>>()?;
    let got=recepcion::extract(&events)?;check(got==load(&p.join("ENTREGA-PROVEEDOR.json"))?&&got["uso_proveedor"]==r["uso_proveedor"],"Entrega o uso alterados")?;
    check(got["texto_original"].as_str().ok_or("Texto")?.as_bytes()==fs::read(p.join("FINAL.txt")).map_err(e)?,"Texto alterado")?;
    let rows=fs::read(p.join("instrumentacion/telemetria.jsonl")).map_err(e)?.split(|b|*b==b'\n').filter(|l|!l.is_empty()).map(|l|parse(l).and_then(|v|parse(v["cuerpo"].as_str().ok_or("Cuerpo")?.as_bytes()))).collect::<R<Vec<_>>>()?;
    let samples:Vec<_>=rows.iter().filter(|r|r["tipo"]=="muestra").collect();check(!samples.is_empty(),"Sin muestras")?;
    let a=samples[0];let z=samples.last().unwrap();let p0=&a["datos"]["proceso"];let p1=&z["datos"]["proceso"];
    let mut states=BTreeSet::new();let mut udp=0;let mut max_tcp=0;
    for s in &samples{let mut tcp=0;for c in s["datos"]["conexiones"].as_array().ok_or("Conexiones")?{if c["protocolo"]=="TCP"{tcp+=1;states.insert(c["estado"].as_str().ok_or("Estado")?.to_string());}else if c["protocolo"]=="UDP"{udp+=1;}}max_tcp=max_tcp.max(tcp);}
    let u=&got["uso_proveedor"];check(num(&u["input_tokens"])?+num(&u["output_tokens"])?==num(&u["total_tokens"])?,"Tokens discordantes")?;
    let mut v=json!({"caso":r["caso"],"duracion_ms":r["duracion_ms"],"primer_texto_ms":r["primer_texto_ms"],"duracion_observada_ms":num(&z["transcurrido_ms"])?-num(&a["transcurrido_ms"])?,"muestras":samples.len(),"eventos_sse":events.len(),"bytes_sse":raw.len(),"intervalo_maximo_ms":t["intervalo_maximo_ms"],"fallos_medicion":t["fallos_medicion"],"rss_maximo_bytes":samples.iter().filter_map(|s|s["datos"]["proceso"]["rss_bytes"].as_u64()).max(),"tcp_maximo_simultaneo":max_tcp,"tcp_estados":states,"observaciones_udp":udp,"sse_sha256":sha(&raw),"telemetria_sha256":t["sha256"],"final_sha256":sha(&fs::read(p.join("FINAL.txt")).map_err(e)?),"solicitud_sha256":sha(&fs::read(p.join("SOLICITUD.json")).map_err(e)?),"uso":{"entrada_tokens":u["input_tokens"],"entrada_cache_tokens":u["input_tokens_details"]["cached_tokens"],"salida_tokens":u["output_tokens"],"razonamiento_tokens":u["output_tokens_details"]["reasoning_tokens"],"total_tokens":u["total_tokens"]},"autoria_licencia":PIE});
    for(k,out)in[("cpu_acumulada_ms","cpu_delta_ms"),("io_lectura_acumulada_bytes","io_lectura_delta_bytes"),("io_escritura_acumulada_bytes","io_escritura_delta_bytes")]{v[out]=json!(num(&p1[k])?.checked_sub(num(&p0[k])?).ok_or("Contador regresivo")?);}Ok(v)
}
fn pdf()->R<Value>{let base=path(PDF);let audit=load(&base.join("COTEJO-BANCO-RUST.json"))?;check(audit["conforme"]==true&&audit["casos"].as_array().is_some_and(|a|a.len()==27),"Banco no recibido")?;let mut ms=vec![];
    for n in 1..=27{let id=format!("ENTREGA-{n:02}");let p=base.join("originales").join(format!("PDF{:02}",(n-1)/3+1)).join(format!("R{}",(n-1)%3));let m=metrics(&p)?;save(&base.join(format!("hitos/{id}/medicion/MEDICION-RUST.json")),&m)?;ms.push(m);}
    let mut totals=json!({});for k in ["entrada_tokens","salida_tokens","total_tokens"] {totals[k]=json!(ms.iter().map(|m|num(&m["uso"][k])).collect::<R<Vec<_>>>()?.iter().sum::<u64>());}
    for k in ["duracion_ms","muestras","eventos_sse","bytes_sse"] {totals[k]=json!(ms.iter().map(|m|num(&m[k])).collect::<R<Vec<_>>>()?.iter().sum::<u64>());}
    let v=json!({"conforme":true,"casos":ms,"totales":totals,"fecha_derivacion_ms":sv_instrumentacion::utc_ms(),"inferencias_nuevas":0,"autoria_licencia":PIE});save(&base.join("METRICAS-RUST.json"),&v)?;Ok(v)}
fn main(){let op=std::env::args().nth(1).unwrap_or_default();let out=match op.as_str(){"pdf"=>pdf(),_=>Err("Indique pdf".into())};match out{Ok(v)=>println!("Conforme: {}",v["conforme"]),Err(e)=>{eprintln!("{e}");std::process::exit(1)}}}
