//! Recorrido real del servicio MCP heredado. El supervisor y el cotejo son Rust.
use serde_json::{json, Value};
use std::{fs, io::{BufReader, Write}, path::Path, process::{Child, Command, Stdio}, sync::{mpsc, Arc, Mutex}, time::{Duration, Instant}};
use sv_mcp_documental::{sha256, read_bounded};
struct Proceso(Arc<Mutex<Child>>);
impl Drop for Proceso { fn drop(&mut self) { if let Ok(mut c)=self.0.lock() { let _ = c.kill(); let _ = c.wait(); } } }
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let a:Vec<_> = std::env::args().collect();
    if a.len()!=6 && a.len()!=8 {return Err("Uso: recibir-biblioteca BINARIO_MCP SHA256_MCP PREPARACION SHA256_RECIBO SALIDA_NUEVA [DOCUMENTO SECCION]".into());}
    sv_mcp_documental::aislamiento::no_network()?;
    if sha256(&fs::read(&a[1])?)!=a[2] {return Err("BINARIO_MCP_NO_COTEJADO".into());}
    let dir=Path::new(&a[3]); let out=Path::new(&a[5]); fs::create_dir(out)?;
    let (receipt,policy,c)=sv_biblioteca_documental::recibir_autorizacion(dir,&a[4])?;
    if a.len()==8 && !c.documents.iter().any(|d|d.id==a[6] && d.sections.iter().any(|s|s.id==a[7])) {
        return Err("SELECCION_NO_AUTORIZADA".into());
    }
    let cat=read_bounded(&dir.join("CATALOGO.json"),sv_mcp_documental::MAX_CATALOG)?;
    let source_raw=read_bounded(&dir.join("FUENTES.json"),65536)?;
    let mut cmd=Command::new(&a[1]);
    cmd.arg(dir.join("CATALOGO.json")).arg(&receipt.catalogo.sha256).arg(out.join("diario-mcp.jsonl"))
        .arg("256");
    if policy.permitir_sinteticos { cmd.arg("--sintetico"); }
    cmd.arg("--fuentes-autorizadas").arg(dir.join("FUENTES.json")).arg(&receipt.fuentes.sha256)
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::inherit());
    let mut spawned=cmd.spawn()?;
    let mut input=spawned.stdin.take().ok_or("SIN_ENTRADA")?;
    let stdout=spawned.stdout.take().ok_or("SIN_SALIDA")?;
    let child=Proceso(Arc::new(Mutex::new(spawned)));
    let supervised=child.0.clone(); let timeout=out.join("PLAZO-AGOTADO.json");
    let (done,wait)=mpsc::channel();
    let supervisor=std::thread::spawn(move || {
        if matches!(wait.recv_timeout(Duration::from_secs(65)), Err(mpsc::RecvTimeoutError::Timeout)) {
            if let Ok(mut c)=supervised.lock() { let _=c.kill(); }
            let _=sv_mcp_documental::libro::guardar_nuevo(&timeout,b"{\"conforme\":false,\"causa\":\"plazo_agotado\"}");
            std::process::exit(124);
        }
    });
    let mut request_file=fs::OpenOptions::new().write(true).create_new(true).open(out.join("solicitudes.jsonl"))?;
    let mut response_file=fs::OpenOptions::new().write(true).create_new(true).open(out.join("respuestas.jsonl"))?;
    let (tx,rx)=mpsc::sync_channel(1);
    let reader=std::thread::spawn(move || {
        let mut buf=BufReader::new(stdout);
        loop {
            let frame=sv_mcp_documental::read_frame(&mut buf);
            let end=!matches!(frame,Ok(Some(_)));
            if tx.send(frame).is_err() || end {break;}
        }
    });
    let start=Instant::now(); let mut requests=Vec::new(); let mut responses=Vec::new(); let mut seq=0;
    let mut ask=|method:&str,params:Value,notice:bool|->Result<Value,Box<dyn std::error::Error>> {
        if start.elapsed()>Duration::from_secs(60) {return Err("PLAZO_TOTAL_AGOTADO".into());}
        seq+=1; let mut req=json!({"jsonrpc":"2.0","method":method,"params":params});
        if !notice {req["id"]=json!(seq);}
        let mut bytes=serde_json::to_vec(&req)?; bytes.push(b'\n');
        request_file.write_all(&bytes)?; request_file.sync_all()?;
        input.write_all(&bytes)?; input.flush()?; requests.extend(bytes);
        if notice {return Ok(Value::Null);}
        let bytes=rx.recv_timeout(Duration::from_secs(10))??.ok_or("FIN_PREMATURO")?;
        response_file.write_all(&bytes)?; response_file.sync_all()?;
        let resp=sv_mcp_documental::parse_strict(&bytes)?;
        if resp["id"]!=seq || resp.get("error").is_some() {return Err("RESPUESTA_NO_CONFORME".into());}
        responses.extend(bytes); Ok(resp["result"].clone())
    };
    ask("initialize",json!({"protocolVersion":sv_mcp_documental::PROTOCOL,"capabilities":{},"clientInfo":{"name":"recepcion-biblioteca-sv","version":"0.1.0"}}),false)?;
    ask("notifications/initialized",json!({}),true)?;
    let tools=ask("tools/list",json!({}),false)?;
    let mut names=tools["tools"].as_array().ok_or("SIN_HERRAMIENTAS")?.iter().map(|t|t["name"].as_str().unwrap_or("")).collect::<Vec<_>>(); names.sort();
    if names!=["buscar_documentos","leer_documento"] {return Err("INTERFAZ_INESPERADA".into());}
    let mut sections=Vec::new();
    for doc in &c.documents {
        for section in &doc.sections {
            if a.len()==8 && (doc.id!=a[6] || section.id!=a[7]) {continue;}
            let mut recovered=String::new(); let mut page=0;
            loop {
                if page>=64 {return Err("PAGINACION_SUPERA_COTA".into());}
                let result=ask("tools/call",json!({"name":"leer_documento","arguments":{"documento":doc.id,"seccion":section.id,"pagina":page}}),false)?;
                if result["isError"]!=false {return Err("LECTURA_RECHAZADA".into());}
                let data=&result["structuredContent"];
                let literal=sv_mcp_documental::parse_strict(result["content"][0]["text"].as_str().ok_or("SIN_REPRESENTACION_TEXTUAL")?.as_bytes())?;
                if &literal!=data {return Err("REPRESENTACIONES_DISCORDANTES".into());}
                if data["sha256_original"]!=doc.raw_sha256 || data["sha256_seccion"]!=section.sha256
                    || data["sintetico"]!=doc.synthetic || data["inicio_caracter"]!=recovered.chars().count() {
                    return Err("FRAGMENTO_DISCORDANTE".into());
                }
                recovered.push_str(data["texto"].as_str().ok_or("SIN_TEXTO")?);
                if data["siguiente_pagina"].is_null() {break;}
                if data["siguiente_pagina"]!=page+1 {return Err("SALTO_PAGINACION".into());}
                page+=1;
            }
            if recovered!=section.text {return Err("RECONSTRUCCION_NO_IDENTICA".into());}
            sections.push(json!({"documento":doc.id,"seccion":section.id,"fragmentos":page+1,"sha256":sha256(recovered.as_bytes()),"identico":true}));
            if a.len()==8 {
                sv_mcp_documental::libro::guardar_nuevo(&out.join("LECTURA.json"), &serde_json::to_vec_pretty(&json!({"documento":doc.id,"seccion":section.id,"titulo":section.title,"texto":recovered,"sha256":section.sha256,"fragmentos":page+1}))?)?;
            }
        }
    }
    let mut rejected=0;
    for args in [json!({"documento":"AJENO","seccion":"S1"}),
        json!({"documento":"BIBLIOTECA_INDICE","seccion":"inicio","url":"https://example.invalid"}),
        json!({"documento":"../../reservado","seccion":"S1"})] {
        let r=ask("tools/call",json!({"name":"leer_documento","arguments":args}),false)?;
        if r["isError"]!=true {return Err("ACCESO_AJENO_ACEPTADO".into());} rejected+=1;
    }
    drop(ask); drop(input);
    let until=Instant::now()+Duration::from_secs(5);
    let status=loop {
        if let Some(s)=child.0.lock().map_err(|_|"SUPERVISOR_FALLIDO")?.try_wait()? {break s;}
        if Instant::now()>until {return Err("CIERRE_NO_RECIBIDO".into());}
        std::thread::sleep(Duration::from_millis(10));
    };
    reader.join().map_err(|_|"LECTOR_FALLIDO")?;
    if !status.success() {return Err("SERVICIO_MCP_FALLIDO".into());}
    let journal=read_bounded(&out.join("diario-mcp.jsonl"),sv_mcp_documental::auditoria::MAX_DIARIO)?;
    let replay=sv_mcp_documental::auditoria::verify(&c,&sha256(&cat),&journal)?;
    let mut observed_input=Vec::new(); let mut observed_output=Vec::new(); let mut samples=0;
    for line in journal.split(|b|*b==b'\n').filter(|b|!b.is_empty()) {
        let entry:Value=serde_json::from_slice(line)?; let e=&entry["datos"];
        if e["evento"]=="solicitud" {observed_input.extend(sv_mcp_documental::auditoria::unhex(e["bytes_hex"].as_str().ok_or("SIN_BYTES")?)?);}
        if e["evento"]=="resultado" {
            observed_output.extend(sv_mcp_documental::auditoria::unhex(e["bytes_hex"].as_str().ok_or("SIN_BYTES")?)?);
            if !e["proceso"]["rss_kib"].is_u64() || e["proceso"]["sockets_propios"]!=0 {return Err("MEDICION_NO_CONFORME".into());} samples+=1;
        }
    }
    if requests!=observed_input || responses!=observed_output {return Err("TRANSPORTE_Y_DIARIO_DISCORDANTES".into());}
    let report=json!({"conforme":true,"documentos":c.documents.len(),"secciones":sections,"diario":replay,
        "solicitudes":seq,"muestras_mcp":samples,"rechazos":rejected,"duracion_us":start.elapsed().as_micros(),
        "binario_mcp_sha256":a[2],"catalogo_sha256":sha256(&cat),"fuentes_sha256":sha256(&source_raw),"recibo_sha256":a[4],
        "politica_sha256":receipt.politica.sha256,"sinteticos_autorizados":policy.permitir_sinteticos,"reconstruccion_identica":true,
        "transporte_cotejado":true,"navegacion_autonoma_modelo":false,"inferencia":false,
        "muestra_receptor":sv_mcp_documental::instrumentacion::muestra()});
    sv_mcp_documental::libro::guardar_nuevo(&out.join("RECEPCION.json"),&serde_json::to_vec_pretty(&report)?)?;
    done.send(())?; supervisor.join().map_err(|_|"SUPERVISOR_FALLIDO")?;
    println!("{}",json!({"conforme":true,"documentos":c.documents.len(),"secciones":report["secciones"].as_array().unwrap().len(),"solicitudes":seq,"rechazos":rejected}));
    Ok(())
}
fn main(){if let Err(e)=run(){
    if let Some(out)=std::env::args().nth(5) {
        let report=json!({"conforme":false,"causa":e.to_string(),"inferencia":false,"muestra":sv_mcp_documental::instrumentacion::muestra()});
        let _=sv_mcp_documental::libro::guardar_nuevo(&Path::new(&out).join("RECHAZO.json"),&serde_json::to_vec_pretty(&report).unwrap_or_default());
    }
    eprintln!("Recepción detenida: {e}");std::process::exit(1);
}}
