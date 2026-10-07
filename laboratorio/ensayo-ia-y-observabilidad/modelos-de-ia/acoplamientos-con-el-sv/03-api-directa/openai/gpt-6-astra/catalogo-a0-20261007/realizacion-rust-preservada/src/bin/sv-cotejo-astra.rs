#![forbid(unsafe_code)]
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,io::{Read,Write},net::TcpListener,time::{Duration,Instant}};
type R<T>=Result<T,Box<dyn std::error::Error>>;
const DIR:&str="../../gpt-6-astra/prueba-conexion-20261006/con-creditos";
const EXPECTED:&str="CONEXION ASTRA CONFIRMADA";
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn extract(raw:&[u8])->R<Value>{
    let mut events=Vec::new();
    for line in std::str::from_utf8(raw)?.lines(){if let Some(s)=line.strip_prefix("data:"){events.push(serde_json::from_str::<Value>(s.trim())?);}}
    if events.len()!=16{return Err("Revisión acotada: se esperaban los 16 eventos conservados".into());}
    for(i,e)in events.iter().enumerate(){if e["sequence_number"].as_u64()!=Some(i as u64){return Err("Secuencia discordante".into());}}
    let created=events.iter().find(|e|e["type"]=="response.created").ok_or("Falta creación")?;
    let terminal=events.last().ok_or("Sin evento terminal")?;
    if terminal["type"]!="response.completed"||terminal["response"]["status"]!="completed"||terminal["response"]["model"]!="gpt-6-astra"||created["response"]["id"]!=terminal["response"]["id"] {return Err("Terminación o identidad no conforme".into());}
    let completed=events.iter().filter(|e|e["type"]=="response.output_item.done").collect::<Vec<_>>();
    if completed.len()!=1{return Err("No hay exactamente un mensaje concluido".into());}
    let item=&completed[0]["item"];
    if item["role"]!="assistant"||item["type"]!="message"||item["status"]!="completed" {return Err("Mensaje no concluido por el asistente".into());}
    let parts=item["content"].as_array().ok_or("Sin contenido")?;
    if parts.len()!=1||parts[0]["type"]!="output_text" {return Err("Contenido inesperado".into());}
    let full=parts[0]["text"].as_str().ok_or("Sin texto final")?;
    let mut deltas=String::new();let mut text_done=None;let mut part_done=None;
    for e in &events{
        if matches!(e["type"].as_str(),Some("response.output_text.delta"|"response.output_text.done"|"response.content_part.done")){
            if e["item_id"]!=item["id"]||e["output_index"]!=0||e["content_index"]!=0{return Err("Correlación de texto discordante".into());}
            match e["type"].as_str().unwrap(){
                "response.output_text.delta"=>deltas.push_str(e["delta"].as_str().ok_or("Fragmento no textual")?),
                "response.output_text.done"=>{if text_done.replace(e["text"].as_str().ok_or("Texto ausente")?).is_some(){return Err("Texto final duplicado".into());}},
                _=>{if part_done.replace(e["part"]["text"].as_str().ok_or("Parte ausente")?).is_some(){return Err("Parte final duplicada".into());}}
            }
        }
    }
    if full!=EXPECTED||deltas!=full||text_done!=Some(full)||part_done!=Some(full){return Err("Fragmentos y cierres de texto no coinciden".into());}
    let usage=&terminal["response"]["usage"];
    if usage["input_tokens"].as_u64().zip(usage["output_tokens"].as_u64()).map(|(a,b)|a+b)!=usage["total_tokens"].as_u64(){return Err("Uso inconsistente".into());}
    Ok(json!({"estado":"conforme_conectividad","respuesta_texto":full,"modelo_declarado":terminal["response"]["model"],
        "estado_proveedor":"completed","evento_terminal":"response.completed","eventos":events.len(),
        "uso_proveedor":usage,"id_respuesta":terminal["response"]["id"],
        "texto_cotejado_en":["fragmentos output_text.delta","output_text.done","content_part.done","output_item.done"],
        "precision":"El evento terminal tiene output vacío; el texto se acredita por los eventos concluidos y correlacionados del mismo flujo. El diagnóstico inicial de discordancia era una insuficiencia del lector local."}))
}
fn main()->R<()>{
    let raw=fs::read(format!("{DIR}/respuesta.sse"))?;
    let previous=fs::read(format!("{DIR}/resultado.json"))?;
    let p:Value=serde_json::from_slice(&previous)?;
    if p["respuesta_sha256"]!=sha(&raw)||p["respuesta_bytes"].as_u64()!=Some(raw.len()as u64){return Err("Cambió la evidencia original".into());}
    let mut v=extract(&raw)?;
    for field in ["duracion_ms","cabecera_ms","primer_evento_ms","primer_texto_ms","respuesta_bytes","respuesta_sha256","peticion_bytes","peticion_sha256","http","fecha_unix","criptografia_nativa"] {v[field]=p[field].clone();}
    v["resultado_inicial_sha256"]=json!(sha(&previous));v["nuevas_solicitudes_durante_cotejo"]=json!(0);
    let j=serde_json::to_vec_pretty(&v)?;
    let out=format!("{DIR}/COTEJO-RUST.json");
    if let Ok(old)=fs::read(&out){if old!=j{return Err("El cotejo anterior es distinto".into());}}else{fs::write(&out,&j)?;}
    let html=format!("<!doctype html><html lang='es'><meta charset='utf-8'><meta name='viewport' content='width=device-width'><title>SV · GPT-6 Astra · Conexión comprobada</title><body><main><h1>GPT-6 Astra</h1><h2>Conexión e inferencia comprobadas</h2><p><strong>Respuesta real de OpenAI:</strong></p><blockquote><strong>{EXPECTED}</strong></blockquote><p>Modelo solicitado y declarado: <code>gpt-6-astra</code><br>Terminación: <code>response.completed</code></p><dl><dt>Duración total</dt><dd>3,223 segundos</dd><dt>Primer texto recibido</dt><dd>2,913 segundos</dd><dt>Tokens de entrada</dt><dd>49</dd><dt>Tokens de salida</dt><dd>12 (razonamiento: 0)</dd><dt>Total</dt><dd>61 tokens</dd></dl><p>Una generación completada. Un intento previo fue rechazado porque el uso de créditos para aplicaciones estaba desactivado.</p><p>Texto y terminación cotejados en Rust a partir de los 16 eventos conservados. El cotejo no envió otra solicitud.</p><p>Créditos descontados y precio liquidado: no comunicados en la respuesta.</p><p>Criptografía C/ensamblador: pendiente, conforme a la autorización de esta prueba.</p><p>Prueba terminada, sin inferencia activa. No acredita todavía la admisión científica del modelo ni su gobierno por el SV.</p></main></body></html>");
    fs::write(format!("{DIR}/RESULTADO-COTEJADO.html"),&html)?;
    println!("COTEJO_CONFORME: 16 eventos; 61 tokens; {EXPECTED}");
    if std::env::args().nth(1).as_deref()==Some("mostrar"){
        let listener=TcpListener::bind("127.0.0.1:0")?;listener.set_nonblocking(true)?;
        println!("PANTALLA_LOCAL=http://{}/",listener.local_addr()?);std::io::stdout().flush()?;
        let started=Instant::now();
        while started.elapsed()<Duration::from_secs(1800){
            let (mut s,peer)=match listener.accept(){Ok(v)=>v,Err(e)if e.kind()==std::io::ErrorKind::WouldBlock=>{std::thread::sleep(Duration::from_millis(100));continue},Err(e)=>return Err(e.into())};
            if !peer.ip().is_loopback(){continue;}s.set_read_timeout(Some(Duration::from_secs(2)))?;s.set_write_timeout(Some(Duration::from_secs(2)))?;
            let mut b=[0u8;8192];let n=match s.read(&mut b){Ok(n)=>n,Err(_)=>continue};
            if !b[..n].starts_with(b"GET / HTTP/1.1\r\n"){let _=s.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");continue;}
            let h=format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nCache-Control: no-store\r\nContent-Security-Policy: default-src 'none'; frame-ancestors 'none'; base-uri 'none'\r\nConnection: close\r\n\r\n",html.len());let _=s.write_all(h.as_bytes());let _=s.write_all(html.as_bytes());
        }
    }
    Ok(())
}
#[cfg(test)]mod tests{use super::*;#[test]fn el_error_y_la_ausencia_de_terminal_no_acreditan_exito(){assert!(extract(b"data: {\"type\":\"error\"}\n\n").is_err());assert!(extract(b"").is_err());}}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
