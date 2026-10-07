#![forbid(unsafe_code)]
#[path="../respuesta.rs"]mod respuesta;
use serde_json::{json,Value};
use std::{fs,io::{Read,Write},path::Path,net::TcpListener,time::{Duration,Instant}};
use sv_instrumentacion::{sha,records,verify};
type R<T>=Result<T,Box<dyn std::error::Error>>;
const DIR:&str="../../gpt-6-astra/prueba-instrumentada-20261006";
fn save_once(path:&Path,bytes:&[u8])->R<()>{if let Ok(old)=fs::read(path){if old!=bytes{return Err("Resultado previo discordante".into());}}else{let mut f=fs::OpenOptions::new().write(true).create_new(true).open(path)?;f.write_all(bytes)?;f.sync_all()?;}Ok(())}
fn main()->R<()> {
    let dir=Path::new(DIR);let raw=fs::read(dir.join("respuesta.sse"))?;let rb=fs::read(dir.join("resultado.json"))?;let r:Value=serde_json::from_slice(&rb)?;
    if r["respuesta_sha256"]!=sha(&raw)||r["respuesta_bytes"].as_u64()!=Some(raw.len()as u64){return Err("Respuesta discordante".into());}
    let events=std::str::from_utf8(&raw)?.lines().filter_map(|s|s.strip_prefix("data:")).map(|s|serde_json::from_str::<Value>(s.trim())).collect::<Result<Vec<_>,_>>()?;
    let text=respuesta::text(&events,"gpt-6-astra","CONEXION ASTRA CONFIRMADA")?;
    if r["respuesta_texto"]!=text||r["estado"]!="conforme"{return Err("Texto o estado del resultado discordante".into());}
    let tl=dir.join("instrumentacion/telemetria.jsonl");let checked=verify(&tl)?;
    if checked!=r["telemetria"]{return Err("Cotejo de telemetría discordante".into());}
    let rows=records(&tl)?;let samples=rows.iter().filter(|v|v["tipo"]=="muestra").collect::<Vec<_>>();
    let begin=rows.iter().find(|v|v["tipo"]=="inicio_envio_https").ok_or("Sin traza de envío")?["transcurrido_ms"].as_u64().unwrap();
    let end=rows.iter().find(|v|v["tipo"]=="transporte_inferencia_liberado").ok_or("Sin traza de cierre")?["transcurrido_ms"].as_u64().unwrap();
    let before=samples.iter().filter(|v|v["transcurrido_ms"].as_u64().unwrap()<begin).count();
    let during=samples.iter().filter(|v|{let t=v["transcurrido_ms"].as_u64().unwrap();t>=begin&&t<=end}).count();
    let after=samples.iter().filter(|v|v["transcurrido_ms"].as_u64().unwrap()>end).count();
    if before<2||during==0||after<2||checked["fallos_medicion"]!=0||checked["intervalo_maximo_ms"].as_u64().unwrap_or(u64::MAX)>750 {return Err("Cobertura instrumental insuficiente".into());}
    let source_sequence=rows.iter().filter(|v|v["tipo"]=="evento_sse").collect::<Vec<_>>();
    if source_sequence.len()!=events.len() || source_sequence.iter().zip(&events).any(|(a,b)|a["datos"]["secuencia_proveedor"]!=b["sequence_number"]||a["datos"]["tipo_proveedor"]!=b["type"]){return Err("Eventos instrumentales no corresponden al flujo conservado".into());}
    let peak=|k:&str|samples.iter().filter_map(|v|v["datos"]["proceso"][k].as_f64()).fold(0f64,f64::max);
    let first=&samples[0]["datos"]["proceso"];let last=&samples.last().unwrap()["datos"]["proceso"];
    let sockets=samples.iter().flat_map(|v|v["datos"]["conexiones"].as_array().unwrap().iter()).map(|v|v.to_string()).collect::<std::collections::BTreeSet<_>>();
    if !sockets.iter().any(|s|serde_json::from_str::<Value>(s).unwrap()["estado"]=="Listen"){return Err("No se observa la escucha local".into());}
    let remote=r["par_remoto_http"].as_str().ok_or("No se comunicó extremo HTTP")?;
    if !sockets.iter().any(|s|{let v:Value=serde_json::from_str(s).unwrap();v["remoto"]==remote && v["estado"]=="Established"}){return Err("No se coteja conexión TCP con extremo HTTP".into());}
    let mut files=Vec::new();
    for name in ["peticion.json","envio-unico.json","respuesta.sse","evento-terminal.json","resultado.json","resultado-transporte.json","resultado.html","catalogo.json","instrumentacion/telemetria.jsonl","instrumentacion/COTEJO-TELEMETRIA.json","cliente-ejecutado/main.rs","cliente-ejecutado/prueba.rs","cliente-ejecutado/respuesta.rs","cliente-ejecutado/Cargo.toml","cliente-ejecutado/Cargo.lock","cliente-ejecutado/instrumentacion.rs","cliente-ejecutado/instrumentacion-Cargo.toml"] {let b=fs::read(dir.join(name))?;files.push(json!({"archivo":name,"bytes":b.len(),"sha256":sha(&b)}));}
    let v=json!({"estado":"conforme_en_el_ambito_comprobado","modelo":"gpt-6-astra","solicitudes_nuevas_durante_cotejo":0,"respuesta":text,"uso":r["uso_proveedor"],"duracion_ms":r["duracion_ms"],"primer_texto_ms":r["primer_texto_ms"],"cpu_ms_en_ventana":last["cpu_acumulada_ms"].as_u64().unwrap()-first["cpu_acumulada_ms"].as_u64().unwrap(),"rss_maxima_bytes":peak("rss_bytes"),"cpu_maxima_porcentaje_nucleo":peak("cpu_porcentaje_nucleo"),"muestras_antes":before,"muestras_durante":during,"muestras_despues":after,"conexiones_estados_distintos":sockets.len(),"extremo_http_cotejado_tcp":true,"telemetria":checked,"archivos":files,"recepcion_integral_sv":false});
    save_once(&dir.join("RECEPCION-RUST.json"),&serde_json::to_vec_pretty(&v)?)?;
    println!("{}",serde_json::to_string_pretty(&json!({"estado":v["estado"],"muestras":v["telemetria"]["muestras"],"antes":before,"durante":during,"despues":after,"cpu_ms":v["cpu_ms_en_ventana"],"rss_maxima_mib":peak("rss_bytes")/1048576.,"max_gap_ms":v["telemetria"]["intervalo_maximo_ms"],"duracion_ms":r["duracion_ms"],"primer_texto_ms":r["primer_texto_ms"],"tokens":r["uso_proveedor"]["total_tokens"],"archivos_cotejados":files.len()}))?);
    if std::env::args().nth(1).as_deref()==Some("mostrar") {
        let html=fs::read(dir.join("resultado.html"))?;
        let listener=TcpListener::bind("127.0.0.1:0")?;listener.set_nonblocking(true)?;
        println!("VISOR_LOCAL=http://{}/",listener.local_addr()?);std::io::stdout().flush()?;
        let started=Instant::now();while started.elapsed()<Duration::from_secs(1800){
            let(mut s,peer)=match listener.accept(){Ok(v)=>v,Err(e)if e.kind()==std::io::ErrorKind::WouldBlock=>{std::thread::sleep(Duration::from_millis(100));continue},Err(e)=>return Err(e.into())};
            if !peer.ip().is_loopback(){continue;}s.set_read_timeout(Some(Duration::from_secs(2)))?;s.set_write_timeout(Some(Duration::from_secs(2)))?;
            let mut b=[0u8;4096];let n=match s.read(&mut b){Ok(n)=>n,Err(_)=>continue};
            if !b[..n].starts_with(b"GET / HTTP/1.1\r\n"){let _=s.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");continue;}
            use base64::Engine;use sha2::{Digest,Sha256};
            let hs=std::str::from_utf8(&html)?;let st=hs.split_once("<style>").and_then(|(_,s)|s.split_once("</style>").map(|(s,_)|s)).ok_or("Sin estilo")?;
            let hash=base64::engine::general_purpose::STANDARD.encode(Sha256::digest(st.as_bytes()));
            let h=format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nCache-Control: no-store\r\nReferrer-Policy: no-referrer\r\nContent-Security-Policy: default-src 'none'; style-src 'sha256-{hash}'; frame-ancestors 'none'; base-uri 'none'\r\nConnection: close\r\n\r\n",html.len());let _=s.write_all(h.as_bytes());let _=s.write_all(&html);
        }
    } Ok(())
}
