use super::*;

pub const MODEL: &str = "gpt-6-astra";
pub const DIR: &str = "../../gpt-6-astra/prueba-conexion-20261006";
pub fn tercera()->bool {std::env::args().nth(1).as_deref()==Some("prueba-entrega")}
pub fn dir() -> String { match std::env::args().nth(1).as_deref() {Some("prueba-entrega")=>"../../gpt-6-astra/prueba-entrega-20261006".into(),Some("prueba-instrumentada")=>"../../gpt-6-astra/prueba-instrumentada-20261006".into(),Some("prueba-creditos")=>format!("{DIR}/con-creditos"),_=>DIR.into()} }
const EXPECTED: &str = "CONEXION ASTRA CONFIRMADA";
const MAX_BYTES: usize = 1024 * 1024;

pub fn prepare() -> R<()> {
    let dir=dir();
    fs::create_dir_all(&dir).map_err(|_|"No se crea expediente Astra")?;
    // No se permite repetir una solicitud cuyo envío ya pudo haber ocurrido.
    if Path::new(&format!("{dir}/envio-unico.json")).exists() { return Err("La única solicitud autorizada ya fue iniciada; no se repite".into()); }
    if tercera() || std::env::args().nth(1).as_deref()==Some("prueba-instrumentada") {
        let code_dir=format!("{dir}/cliente-ejecutado");fs::create_dir_all(&code_dir).map_err(|_|"No se crea conservación de código")?;
        for (src,name) in [("src/main.rs","main.rs"),("src/prueba.rs","prueba.rs"),("src/respuesta.rs","respuesta.rs"),("src/entrega.rs","entrega.rs"),("Cargo.toml","Cargo.toml"),("Cargo.lock","Cargo.lock"),("../../../../instrumentacion-rust/src/lib.rs","instrumentacion.rs"),("../../../../instrumentacion-rust/Cargo.toml","instrumentacion-Cargo.toml")] {
            let bytes=fs::read(src).map_err(|_|"Falta fuente de esta compilación")?;let p=format!("{code_dir}/{name}");
            if let Ok(old)=fs::read(&p){if old!=bytes{return Err("La conservación previa del código es discordante".into());}}else{fs::write(p,bytes).map_err(|_|"No se conserva fuente")?;}
        }
    }
    if tercera(){save(&format!("{dir}/CONTRATO-JSON.json"),&super::entrega::schema())?;save(&format!("{dir}/PETICION-PREVISTA.json"),&super::entrega::payload())?;}
    Ok(())
}

#[derive(Default)]
struct Events { pending: Vec<u8>, count: u64, first_ms: Option<u128>, text_ms: Option<u128>, terminal: Option<Value>, received:Vec<Value> }
impl Events {
    fn push(&mut self, bytes: &[u8], elapsed: u128) -> R<()> {
        self.pending.extend_from_slice(bytes);
        while let Some(end)=self.pending.iter().position(|&b|b==b'\n') {
            // Cada línea completa conserva UTF-8 aunque las lecturas lo fragmenten.
            let line:Vec<u8>=self.pending.drain(..=end).collect();
            let line=std::str::from_utf8(&line).map_err(|_|"Evento no UTF-8")?.trim_end_matches(['\r','\n']);
            if let Some(data)=line.strip_prefix("data:") {
                if data.trim()=="[DONE]" {continue;}
                let event:Value=serde_json::from_str(data.trim()).map_err(|_|"Evento no JSON")?;
                if self.terminal.is_some(){return Err("Evento posterior a terminación".into());}
                self.received.push(event.clone());
                self.count+=1;
                self.first_ms.get_or_insert(elapsed);
                let kind=event["type"].as_str().unwrap_or("");
                if kind=="response.output_text.delta" {self.text_ms.get_or_insert(elapsed);}
                if matches!(kind,"response.completed"|"response.failed"|"response.incomplete"|"error") {
                    if self.terminal.is_some() {return Err("Más de un estado terminal".into());}
                    self.terminal=Some(event);
                }
            }
        }
        Ok(())
    }
}
#[cfg(test)]fn response_text(response: &Value)->String {
    response["output"].as_array().into_iter().flatten()
        .flat_map(|item|item["content"].as_array().into_iter().flatten())
        .filter(|c|c["type"]=="output_text")
        .filter_map(|c|c["text"].as_str()).collect::<Vec<_>>().join("")
}
#[cfg(test)]fn classification(terminal:&Option<Value>,fault:&Option<String>)-> &'static str {
    if fault.is_some() {return "resultado_incierto";}
    match terminal {
        Some(t) if t["type"]=="response.completed" && t["response"]["status"]=="completed" => {
            if t["response"]["model"]==MODEL && response_text(&t["response"]).trim()==EXPECTED {"conforme"} else {"respuesta_recibida_discordante"}
        }
        Some(_) => "fallo_o_respuesta_incompleta",
        None => "resultado_incierto",
    }
}
pub fn infer(http:&Client,token:&str)->R<Value> {
    let monitor=sv_instrumentacion::Monitor::start(Path::new(&dir()).join("instrumentacion").as_path())?;
    monitor.event("identidad_y_catalogo_verificados",json!({"modelo":MODEL,"solicitud_aun_no_enviada":true}))?;
    std::thread::sleep(Duration::from_secs(2));
    monitor.healthy()?;
    let measured=infer_measured(http,token,&monitor);
    monitor.event("fin_funcion_inferencia",json!({"resultado_local_disponible":measured.is_ok()}))?;
    std::thread::sleep(Duration::from_secs(1));
    let telemetry=monitor.finish()?;
    let mut result=measured?;
    result["instrumentacion_sin_fallos"]=json!(telemetry["fallos_medicion"]==0 && telemetry["intervalo_maximo_ms"].as_u64().is_some_and(|v|v<=750));
    result["telemetria"]=telemetry;
    result["recepcion_instrumental_completa"]=json!(false);
    Ok(result)
}
fn infer_measured(_http:&Client,token:&str,monitor:&sv_instrumentacion::Monitor)->R<Value> {
    let dir=dir();
    let http=client()?; // Transporte independiente del catálogo y la autorización.
    let exe=std::env::current_exe().map_err(|_|"No se identifica ejecutable")?;
    let exe_bytes=fs::read(&exe).map_err(|_|"No se coteja ejecutable")?;
    monitor.event("ejecutable_identificado",json!({"nombre":exe.file_name().and_then(|x|x.to_str()),"bytes":exe_bytes.len(),"sha256":sha(&exe_bytes),"perfil":"compilación de comprobación; no optimizada"}))?;
    let payload=if tercera(){super::entrega::payload()}else{json!({"model":MODEL,"store":false,"stream":true,"reasoning":{"effort":"low"},
        "instructions":"Esta es una prueba instrumental de conectividad. Responda únicamente con el texto exacto solicitado, sin explicación ni formato adicional.",
        "input":[{"role":"user","content":"Responda exactamente: CONEXION ASTRA CONFIRMADA"}]})};
    let serialized=serde_json::to_vec(&payload).map_err(|_|"No se codifica petición")?;
    monitor.event("peticion_serializada",json!({"bytes":serialized.len(),"sha256":sha(&serialized)}))?;
    fs::write(format!("{dir}/peticion.json"),&serialized).map_err(|_|"No se conserva petición")?;
    let marker=json!({"inicio_unix":now(),"modelo":MODEL,"maximo_solicitudes":1,"reintentos":0,
        "autorizacion":"Prueba breve expresamente autorizada el 06/10/2026 con créditos existentes; dependencia criptográfica nativa pendiente.",
        "peticion_sha256":sha(&serialized),"limite_local_segundos":120,
        "observacion":"El cierre local de transporte no demuestra cancelación remota ni constituye una cota económica."});
    let mut guard=fs::OpenOptions::new().write(true).create_new(true).open(format!("{dir}/envio-unico.json")).map_err(|_|"Solicitud ya iniciada o no registrable")?;
    guard.write_all(serde_json::to_string_pretty(&marker).unwrap().as_bytes()).and_then(|_|guard.sync_all()).map_err(|_|"No se confirma registro previo al envío")?;
    monitor.healthy()?;
    monitor.event("inicio_envio_https",json!({"metodo":"POST","destino":"api.openai.com/v1/responses","store":false,"stream":true,"solicitudes":1,"reintentos":0}))?;
    let started=Instant::now();
    let mut report=json!({"fecha_unix":now(),"modelo_solicitado":MODEL,"identidad_verificada":true,
        "operacion":"POST /v1/responses","solicitudes":1,"reintentos":0,"estado":"resultado_incierto",
        "creditos_cobrados":null,"coste_liquidado":null,"datos":"texto artificial exclusivamente",
        "peticion_bytes":serialized.len(),"peticion_sha256":sha(&serialized),
        "criptografia_nativa":"pendiente: ring incorpora C y ensamblador; excepción autorizada únicamente para esta prueba"});
    let received=http.post("https://api.openai.com/v1/responses").bearer_auth(token)
        .header("Content-Type","application/json").header("Accept","text/event-stream")
        .timeout(Duration::from_secs(120)).body(serialized).send();
    let mut response=match received {
        Ok(r)=>r,
        Err(_)=>{monitor.event("error_transporte",json!({"fase":"espera_cabeceras","consumo":"incierto"}))?;report["error_local"]=json!("No se recibió cabecera HTTP; no se reintenta ni se presume ausencia de consumo");report["duracion_ms"]=json!(started.elapsed().as_millis());return Ok(report);}
    };
    let status=response.status().as_u16();
    report["http"]=json!(status); report["cabecera_ms"]=json!(started.elapsed().as_millis());
    report["par_remoto_http"]=json!(response.remote_addr().map(|a|a.to_string()));
    report["version_http"]=json!(format!("{:?}",response.version()));
    monitor.event("cabeceras_recibidas",json!({"http":status,"desde_envio_ms":started.elapsed().as_millis(),"par_remoto":report["par_remoto_http"],"version":report["version_http"]}))?;
    for name in ["x-request-id","openai-processing-ms","content-type","x-ratelimit-limit-requests","x-ratelimit-limit-tokens","x-ratelimit-remaining-requests","x-ratelimit-remaining-tokens","x-ratelimit-reset-requests","x-ratelimit-reset-tokens"] {
        if let Some(value)=response.headers().get(name).and_then(|v|v.to_str().ok()) {report["cabeceras_proveedor"][name]=json!(value);}
    }
    let mut raw=Vec::new();let mut events=Events::default();let mut fault=None;let mut buffer=[0u8;8192];
    loop {
        let n=match response.read(&mut buffer) {Ok(n)=>n,Err(_)=>{fault=Some("Lectura interrumpida o límite temporal local alcanzado".into());break;}};
        if n==0 {break;}
        if raw.len()+n>MAX_BYTES {fault=Some("Se alcanzó el límite local de un MiB".into());break;}
        raw.extend_from_slice(&buffer[..n]);
        monitor.event("lectura_cuerpo",json!({"bytes":n,"acumulado_bytes":raw.len(),"desde_envio_ms":started.elapsed().as_millis()}))?;
        if status==200 {
            let before=events.received.len();
            if let Err(e)=events.push(&buffer[..n],started.elapsed().as_millis()) {fault=Some(e);break;}
            for e in &events.received[before..] {monitor.event("evento_sse",json!({"tipo_proveedor":e["type"],"secuencia_proveedor":e["sequence_number"],"desde_envio_ms":started.elapsed().as_millis()}))?;}
            if events.terminal.is_some(){break;}
        }
    }
    report["duracion_ms"]=json!(started.elapsed().as_millis());
    report["primer_evento_ms"]=json!(events.first_ms); report["primer_texto_ms"]=json!(events.text_ms);
    report["eventos"]=json!(events.count);report["respuesta_bytes"]=json!(raw.len()); report["respuesta_sha256"]=json!(sha(&raw));
    fs::write(format!("{dir}/respuesta.sse"),&raw).map_err(|_|"No se conserva respuesta recibida")?;
    report["error_local"]=json!(fault);
    let accepted=super::respuesta::text(&events.received,MODEL,EXPECTED);
    let delivery=if tercera(){Some(super::entrega::extract(&events.received))}else{None};
    if tercera(){
        let items:Vec<_>=events.received.iter().filter(|e|e["type"]=="response.output_item.done").map(|e|e["item"].clone()).collect();
        save(&format!("{dir}/salidas-proveedor.json"),&json!(items))?;
        report["tercera_prueba"]=json!(true);report["herramientas_habilitadas"]=json!(0);
        report["codigo_ejecutado_por_controlador"]=json!(false);
    }
    if status!=200 {
        report["estado"]=json!("rechazo_http");
        report["error_proveedor"]=serde_json::from_slice(&raw).unwrap_or(json!({"cuerpo_no_json":true}));
    } else {
        report["estado"]=json!(if fault.is_some(){"resultado_incierto"}else if accepted.is_ok(){"conforme"}else{"respuesta_no_acreditada"});
        report["cotejo_texto"]=json!(accepted.as_ref().err());
        if let Some(event)=events.terminal {
            save(&format!("{dir}/evento-terminal.json"),&event)?;
            report["evento_terminal"]=event["type"].clone();
            report["error_proveedor"]=event["error"].clone();
            report["modelo_declarado"]=event["response"]["model"].clone();
            report["id_respuesta"]=event["response"]["id"].clone();
            report["estado_proveedor"]=event["response"]["status"].clone();
            report["uso_proveedor"]=event["response"]["usage"].clone();
            report["respuesta_texto"]=json!(accepted.unwrap_or_default());
        }
        if let Some(delivery)=delivery {match delivery {
            Ok(v)=>{
                report["respuesta_texto"]=v["texto_original"].clone();
                report["estado"]=json!(if fault.is_some(){"resultado_incierto"}else if v["incidencias_contenido"].as_array().is_some_and(|a|a.is_empty()){"entrega_estructurada_cotejada"}else{"entrega_con_incidencias"});
                report["cotejo_texto"]=Value::Null;save(&format!("{dir}/ENTREGA-MODELO.json"),&v)?;report["entrega"]=v;
            },
            Err(e)=>{
                report["estado"]=json!(if fault.is_some(){"resultado_incierto"}else{"entrega_no_acreditada"});report["cotejo_texto"]=json!(e);
                report["respuesta_texto"]=json!(events.received.iter().filter(|e|e["type"]=="response.output_text.delta").filter_map(|e|e["delta"].as_str()).collect::<String>());
            }
        }}
    }
    drop(response);drop(http);
    monitor.event("transporte_inferencia_liberado",json!({"estado":report["estado"],"respuesta_sha256":report["respuesta_sha256"],"duracion_ms":report["duracion_ms"]}))?;
    save(&format!("{dir}/resultado-transporte.json"),&report)?;
    Ok(report)
}
fn escape(s:&str)->String {s.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;").replace('"',"&quot;")}
fn shell(body:&str)->String {format!("<!doctype html><html lang='es'><meta charset='utf-8'><meta name='viewport' content='width=device-width'><title>SV · GPT-6 Astra · Prueba de conexión</title><body><main><h1>GPT-6 Astra · Prueba de conexión</h1>{body}</main></body></html>")}
pub fn start_page()->String {if tercera(){return shell("<h2>Tercera prueba · entrega estructurada</h2><p>Una única consulta artificial: 37 grupos de 24 elementos y 16 adicionales.</p><p>Se solicita JSON con resultado, justificación resumida, datos de apoyo, acciones declaradas, código Rust propuesto y límites. También se solicita el resumen explicativo opcional del proveedor. No hay herramientas habilitadas ni ejecución del código recibido.</p><p>Telemetría Rust, conservación de eventos y cotejo posterior. Sin reintentos; límite de transporte 120 segundos. Catálogo de evaluación, adversariales y examen pertenecen a fases posteriores.</p><p><a href='/start'>Continuar con ChatGPT e iniciar la tercera prueba autorizada</a></p>");}shell("<p><strong>Preparado; todavía no se ha enviado inferencia.</strong></p><p>Una solicitud breve, autorizada, con los créditos existentes de ChatGPT.</p><p>Texto de prueba: <code>Responda exactamente: CONEXION ASTRA CONFIRMADA</code></p><p>Después del inicio de sesión se enviará una única solicitud y se mostrarán aquí su respuesta, duración y tokens.</p><p><a href='/start'>Continuar con ChatGPT e iniciar la prueba autorizada</a></p><p>Criptografía nativa: anotada como pendiente. Esta comprobación no constituye admisión del modelo en el SV.</p>")}
pub fn result_page(status:Option<&str>)->String {
    if let Ok(bytes)=fs::read(format!("{}/resultado.json",dir())) {if let Ok(v)=serde_json::from_slice(&bytes){return page(&v);}}
    shell(&format!("<p>{}</p><p><a href='/resultado'>Actualizar resultado</a></p>",escape(status.unwrap_or("Autorización y prueba en curso."))))
}
pub fn page(v:&Value)->String {
    if v.get("telemetria").is_some() {return sv_instrumentacion::dashboard(Path::new(&dir()).join("instrumentacion").as_path(),v).map(|s|if tercera(){tercera_page(s,v)}else{s}).unwrap_or_else(|e|shell(&format!("<p>No se puede cotejar el cuadro de instrumentación: {}</p>",escape(&e))));}
    if v["error_proveedor"]["code"].as_str().is_some() {return shell(&format!("<p><strong>Solicitud rechazada por OpenAI</strong></p><p>Código: <code>{}</code></p><p>{}</p><p>Duración: {} ms. Sin texto generado ni tokens comunicados.</p><p>La solicitud ha terminado. Este resultado no acredita una inferencia completada.</p>",escape(v["error_proveedor"]["code"].as_str().unwrap()),escape(v["error_proveedor"]["message"].as_str().unwrap_or("")),v["duracion_ms"]));}
    let val=|key:&str|v[key].as_str().map(str::to_string).unwrap_or_else(||v[key].to_string());
    shell(&format!("<p><strong>Resultado: {}</strong></p><h2>Respuesta recibida</h2><pre>{}</pre><dl><dt>Modelo solicitado</dt><dd>{}</dd><dt>Modelo declarado por OpenAI</dt><dd>{}</dd><dt>Estado del proveedor</dt><dd>{}</dd><dt>Duración total medida en Rust</dt><dd>{} ms</dd><dt>Primer texto recibido</dt><dd>{} ms</dd><dt>Uso comunicado por OpenAI</dt><dd><pre>{}</pre></dd></dl><p>Una solicitud; sin reintentos. Créditos descontados y liquidación monetaria: no comunicados en esta respuesta.</p><p>Criptografía nativa pendiente. Prueba instrumental de conexión, sin evaluación científica del candidato ni documentación sensible.</p><p>La solicitud ha terminado. Esta pantalla presenta el resultado conservado; no mantiene una inferencia activa.</p>",escape(&val("estado")),escape(&val("respuesta_texto")),escape(&val("modelo_solicitado")),escape(&val("modelo_declarado")),escape(&val("estado_proveedor")),escape(&val("duracion_ms")),escape(&val("primer_texto_ms")),escape(&serde_json::to_string_pretty(&v["uso_proveedor"]).unwrap_or_default())))
}

pub fn tercera_page(s:String,v:&Value)->String {
    let delivery=&v["entrega"];let d=&delivery["declaracion_modelo"];
    let pretty=|x:&Value|escape(&serde_json::to_string_pretty(x).unwrap_or_default());
    let summary=if delivery["resumen_recibido"]==true {delivery["resumen_proveedor"].as_array().unwrap().iter().filter_map(Value::as_str).map(escape).collect::<Vec<_>>().join("\n\n")}else{"No recibido; no se reconstruye ni se inventa.".into()};
    let body=format!("<h2>Entrega efectiva de Astra</h2><p><strong>{}</strong></p><p>Justificación declarada: {}</p><h3>Resumen explicativo comunicado por OpenAI</h3><pre>{summary}</pre><p class='muted'>Resumen opcional del proveedor; no es acceso al razonamiento interno íntegro ni certificación de veracidad.</p><h3>Código propuesto por el modelo</h3><pre>{}</pre><p><strong>Código no ejecutado. Ninguna herramienta habilitada.</strong></p><h3>Respuesta JSON íntegra del modelo</h3><pre>{}</pre><h3>Cotejo y alcance</h3><pre>{}</pre><p>Catálogo A01–A09, tres revisiones adversariales y examen: todavía no ejecutados. Esta prueba observa la entrega y no acredita fidelidad clínica.</p>",escape(d["conclusion"].as_str().unwrap_or("Respuesta estructurada no acreditada")),escape(d["justificacion_resumida"].as_str().unwrap_or("No disponible")),escape(d["codigo_rust_propuesto"].as_str().unwrap_or("No recibido")),pretty(d),pretty(&json!({"estructura":delivery["cotejo_estructural"],"incidencias_contenido":delivery["incidencias_contenido"],"error_lectura":v["cotejo_texto"],"resumen_recibido":delivery["resumen_recibido"],"recepcion_semantica":delivery["cotejo_semantico_integral"]})));
    s.replace("Instrumentación de la segunda prueba","Tercera prueba · entrega y trazabilidad")
     .replace("<h2>Proceso observado y recursos locales</h2>",&format!("{body}<h2>Proceso observado y recursos locales</h2>"))
     .replace("<details>","<details open>").replace("</style>","@media print{details>*{display:block!important}body{background:white;color:#111}pre,code{white-space:pre-wrap;overflow-wrap:anywhere}tr{break-inside:avoid}.scroll{overflow:visible}h2,h3{break-after:avoid}}</style>")
}

#[cfg(test)]mod tests{
    use super::*;
    fn done()->Value{json!({"type":"response.completed","response":{"status":"completed","model":MODEL,"output":[{"content":[{"type":"output_text","text":EXPECTED}]}],"usage":{"input_tokens":12,"output_tokens":6,"total_tokens":18}}})}
    #[test]fn fragmentacion_y_terminal(){let s=format!("data: {{\"type\":\"response.output_text.delta\",\"delta\":\"conexión\"}}\r\n\r\ndata: {}\n\n",done());let mut e=Events::default();for b in s.bytes(){e.push(&[b],3).unwrap();}assert_eq!(classification(&e.terminal,&None),"conforme");assert_eq!(e.count,2);assert_eq!(e.text_ms,Some(3));}
    #[test]fn parcial_no_es_exito(){let mut e=Events::default();e.push(b"data: {\"type\":\"response.output_text.delta\",\"delta\":\"texto\"}\n\n",2).unwrap();assert_eq!(classification(&e.terminal,&None),"resultado_incierto");}
    #[test]fn fallo_y_modelo_discordante(){assert_eq!(classification(&Some(json!({"type":"response.failed"})),&None),"fallo_o_respuesta_incompleta");let mut v=done();v["response"]["model"]=json!("otro");assert_eq!(classification(&Some(v),&None),"respuesta_recibida_discordante");}
    #[test]fn no_doble_terminal_ni_html(){let s=format!("data: {}\n\ndata: {}\n\n",done(),done());assert!(Events::default().push(s.as_bytes(),1).is_err());assert_eq!(escape("<script>&"),"&lt;script&gt;&amp;");}
}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
