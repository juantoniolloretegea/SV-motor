//! Un único ensayo PDF06. Admisión y comprobación independientes del candidato.
use crate::{catalogo::{path, put, save}, suministro_pdf::*};
use serde_json::{json, Value};
use std::{fs::{self, File, OpenOptions}, io::{Read, Write}, path::{Path, PathBuf}, process::{Command, Stdio}, thread, time::{Duration, Instant}};
const RUN: &str = "ejecucion/astra-pdf-transporte-20261007";
const SUPPLY: &str = "ejecucion/astra-pdf-20261007/recepcion-transporte-r1";
const ANNEX: &str = "desarrollo/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/anexo-pdf-20261007";
const ACCESS: &str = "desarrollo/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6.1-sol/acceso";
const BIN: &str = "compilacion/telemetria/debug/sv-suministro-pdf-astra.exe";
const BIN_SHA: &str = "be54c5eefc76835520533a6ec26fed75bd8ed4166ae54fa1a5cb4fa83b649cc9";
fn err(e: impl std::fmt::Display) -> String { e.to_string() }
pub fn base() -> PathBuf { path(RUN) }
fn bytes(p: &Path) -> R<Vec<u8>> {
    let rel = p.strip_prefix(crate::catalogo::ROOT).map_err(err)?;
    let mut x = path("");
    for c in rel.components() {
        need(matches!(c, std::path::Component::Normal(_)), "Ruta no normal")?; x.push(c);
        let m = fs::symlink_metadata(&x).map_err(err)?;
        need(!m.file_type().is_symlink(), "Enlace no admitido")?;
        #[cfg(windows)] { use std::os::windows::fs::MetadataExt; need(m.file_attributes() & 0x400 == 0, "Reanálisis no admitido")?; }
    }
    let f = File::open(p).map_err(err)?; need(f.metadata().map_err(err)?.is_file(), "Archivo no regular")?;
    let mut b = vec![]; f.take(128*1024*1024+1).read_to_end(&mut b).map_err(err)?;
    need(b.len() <= 128*1024*1024, "Archivo excesivo")?; Ok(b)
}
fn load(p: &Path) -> R<Value> { parse(&bytes(p)?) }
fn supply(n: &str) -> PathBuf { path(SUPPLY).join(n) }
fn identity(p: &Path) -> R<Value> { let b=bytes(p)?; Ok(json!({"ruta":p.strip_prefix(crate::catalogo::ROOT).map_err(err)?.to_string_lossy(),"bytes":b.len(),"sha256":sha(&b)})) }
fn director() -> R<DireccionPdf> {
    let control=load(&supply("CONTROL-ARBITRO.json"))?;
    let proof=load(&supply("COTEJO-SUMINISTRO-RUST.json"))?;
    need(proof["conforme"]==true && proof["control_sha256"]==sha(&bytes(&supply("CONTROL-ARBITRO.json"))?), "Recepción no acreditada")?;
    let mut d=DireccionPdf::nueva(&bytes(&path(&format!("{ANNEX}/CANDIDATO.json")))?,load(&supply("documento/CATALOGO.json"))?,&load(&supply("documento/EXTRACCION.json"))?,&load(&supply("documento/FUENTES.json"))?)?;
    let mut pages:Vec<Pagina>=(1..=10).map(|ordinal|Pagina{ordinal,fragmentos:vec![]}).collect();
    for (i,line) in bytes(&supply("MCP-RESPUESTAS.jsonl"))?.split(|b|*b==b'\n').filter(|l|!l.is_empty()).skip(2).enumerate() {
        let f=contenido(&parse(line)?,i+2)?; let ordinal=num(&f["pagina_pdf_ordinal"])?;
        need((1..=10).contains(&ordinal),"Página ajena")?; pages[ordinal-1].fragmentos.push(f);
    }
    let measured=sv_instrumentacion::verify(&supply("instrumentacion/telemetria.jsonl"))?;
    need(measured==control["telemetria"],"Telemetría del suministro modificada")?;
    d.recibir(pages,&load(&supply("AISLAMIENTO-RESPUESTAS.jsonl"))?,&load(&supply("VERIFICACION-DIARIO-RESPUESTAS.jsonl"))?,&sha(&bytes(&supply("MCP-DIARIO.jsonl"))?))?;
    d.admitir(&measured,&sha(&bytes(&path(&format!("{ANNEX}/CRITERIOS-EVALUADOR.md")))?))?; Ok(d)
}
fn frozen() -> R<Vec<Value>> {
    let mut paths=vec![path(BIN),std::env::current_exe().map_err(err)?,path(&format!("{ANNEX}/CANDIDATO.json")),path(&format!("{ANNEX}/CRITERIOS-EVALUADOR.md"))];
    for n in ["CONTROL-ARBITRO.json","COTEJO-SUMINISTRO-RUST.json","documento/CATALOGO.json","documento/EXTRACCION.json","documento/FUENTES.json","MCP-SOLICITUDES.jsonl","MCP-RESPUESTAS.jsonl","MCP-DIARIO.jsonl","AISLAMIENTO-RESPUESTAS.jsonl","VERIFICACION-DIARIO-RESPUESTAS.jsonl","instrumentacion/telemetria.jsonl","solicitudes-previstas/PDF06.json"] {paths.push(supply(n));}
    for n in ["Cargo.toml","Cargo.lock","src/main.rs","src/prueba.rs","src/pdf.rs","src/catalogo/recepcion.rs","src/catalogo/estricto.rs"] {paths.push(path(&format!("{ACCESS}/{n}")));}
    for n in ["lib.rs","estricto.rs"] {paths.push(path(&format!("{ANNEX}/instrumento/src/{n}")));}
    paths.push(path("desarrollo/acoplamientos-con-el-sv/instrumentacion-rust/src/lib.rs"));
    paths.iter().map(|p|identity(p)).collect()
}
fn local(mode: &str) -> R<()> {
    need(sha(&bytes(&path(BIN))?)==BIN_SHA,"Ejecutable documental distinto del recibido")?;
    let out=File::create(base().join(format!("{mode}-stdout.txt"))).map_err(err)?;
    let stderr=File::create(base().join(format!("{mode}-stderr.txt"))).map_err(err)?;
    let mut cmd=Command::new(path(BIN)); cmd.args([mode,SUPPLY]).current_dir(crate::catalogo::ROOT).stdin(Stdio::null()).stdout(out).stderr(stderr);
    #[cfg(windows)] {use std::os::windows::process::CommandExt;cmd.creation_flags(0x08000000);}
    let start=Instant::now(); let mut child=cmd.spawn().map_err(err)?; let pid=child.id();
    let mut sampler=sv_instrumentacion::Sampler::for_pid(pid)?; let mut samples=vec![];
    let status=loop {if let Some(s)=child.try_wait().map_err(err)? {break s;}
        if start.elapsed()>Duration::from_secs(90) {let _=child.kill();let _=child.wait();return Err("Plazo de preparación agotado".into());}
        match sampler.sample() {Ok(s)=>samples.push(json!({"ms":start.elapsed().as_millis(),"muestra":s})),Err(e)=>{if child.try_wait().map_err(err)?.is_none(){let _=child.kill();let _=child.wait();return Err(e);}}}
        thread::sleep(Duration::from_millis(250));
    };
    save(&base().join(format!("{mode}-PROCESO.json")),&json!({"pid":pid,"codigo":status.code(),"duracion_ms":start.elapsed().as_millis(),"muestras":samples,"ejecutable_sha256":BIN_SHA,"alcance":"auxiliar nativo Rust; procesos Linux constan en su diario propio"}))?;
    need(status.success(),"Preparación documental fallida: conservar recibos")
}
pub fn preparar() -> R<Value> {
    need(!base().exists()&&!path(SUPPLY).exists(),"Preparación existente: no sobrescribir")?;
    put(&base().join("INICIO.json"),&serde_json::to_vec(&json!({"utc_ms":sv_instrumentacion::utc_ms(),"suceso":"S39","tique":"TT-0021","caso":"PDF06","inferencias":0})).map_err(err)?)?;
    let m=sv_instrumentacion::Monitor::start_bounded(&base().join("preparacion-instrumentacion"),210)?;
    let result=(|| {local("preparar")?;local("verificar")?;let d=director()?;
        let request=bytes(&supply("solicitudes-previstas/PDF06.json"))?;d.cotejar_solicitud(5,&parse(&request)?)?;
        put(&base().join("SOLICITUD.json"),&request)?;Ok::<_,String>((frozen()?,sha(&request)))})();
    let telemetry=m.finish()?; let (files,hash)=result?;
    need(telemetry["fallos_medicion"]==0,"Instrumentación previa degradada")?;
    let v=json!({"conforme":true,"fecha_unix_ms":sv_instrumentacion::utc_ms(),"suceso":"S39","tique":"TT-0021","caso":"PDF06","paginas_fisicas":[5,8],"fragmentos":6,"archivos_fijados":files,"solicitud_sha256":hash,"telemetria":telemetry,"envios_autorizados":1,"reintentos":0,"limite_s":300,"inferencia_ejecutada":false,"autorizacion":"Instrucción humana expresa: acoplar suministro al transporte de Astra y realizar una prueba real con telemetría, auditoría y trazabilidad","alcance_control":"admisión documental y de transporte del Árbitro-Director; adjudicación científica separada"});
    save(&base().join("PREVIA.json"),&v)?;Ok(v)
}
fn comprobar() -> R<()> {
    let v=load(&base().join("PREVIA.json"))?;
    need(v["conforme"]==true&&v["caso"]=="PDF06"&&v["envios_autorizados"]==1,"Admisión previa ajena")?;
    need(v["archivos_fijados"]==json!(frozen()?),"Cambió un archivo o ejecutable fijado")?;
    let b=bytes(&base().join("SOLICITUD.json"))?;
    need(v["solicitud_sha256"]==sha(&b),"Solicitud alterada")?;director()?.cotejar_solicitud(5,&parse(&b)?)
}
pub fn prepare() -> R<()> {need(!base().join("envio-unico.json").exists(),"Envío ya iniciado; no repetir")?;comprobar()}

#[derive(Default)]
struct Stream {pending:Vec<u8>,events:Vec<Value>,terminal:bool,done:bool}
impl Stream {
    fn feed(&mut self,b:&[u8]) -> R<Vec<String>> {
        self.pending.extend_from_slice(b);let mut kinds=vec![];
        while let Some(i)=self.pending.iter().position(|b|*b==b'\n') {
            let line=self.pending.drain(..=i).collect::<Vec<_>>();let s=std::str::from_utf8(&line).map_err(err)?.trim_end_matches(['\r','\n']);
            if let Some(data)=s.strip_prefix("data:") {
                if data.trim()=="[DONE]" {need(self.terminal&&!self.done,"Marcador final prematuro o repetido")?;self.done=true;continue;}
                need(!self.terminal&&!self.done,"Evento posterior al cierre")?;
                let v=parse(data.trim().as_bytes())?;need(v["sequence_number"].as_u64()==Some(self.events.len() as u64),"Secuencia SSE discordante")?;
                let kind=v["type"].as_str().ok_or("Tipo ausente")?;
                need(matches!(kind,"response.created"|"response.in_progress"|"response.output_item.added"|"response.content_part.added"|"response.output_text.delta"|"response.output_text.done"|"response.content_part.done"|"response.output_item.done"|"response.completed"|"response.failed"|"response.incomplete"|"error"|"response.reasoning_summary_part.added"|"response.reasoning_summary_part.done"|"response.reasoning_summary_text.delta"|"response.reasoning_summary_text.done"),"Evento ajeno al contrato")?;
                if v.get("item").is_some(){need(matches!(v["item"]["type"].as_str(),Some("message"|"reasoning")),"Herramienta rechazada, no ejecutada")?;}
                self.terminal=matches!(kind,"response.completed"|"response.failed"|"response.incomplete"|"error");kinds.push(kind.to_string());self.events.push(v);
            } else {need(s.is_empty()||s.starts_with(':')||s.starts_with("event:"),"Campo SSE no admitido")?;}
        }Ok(kinds)
    }
    fn finish(&self) -> R<()> {need(self.terminal&&self.pending.iter().all(u8::is_ascii_whitespace),"Entrega incompleta")}
}
fn norm(s:&str)->String {s.split_whitespace().collect::<Vec<_>>().join(" ")}
fn formal(v:&Value,req:&Value)->R<Value> {
    need(v.is_object(),"Entrega no es objeto JSON")?;
    for key in ["respuesta","fundamentos_verificables","evidencias","insuficiencias"] {need(v.get(key).is_some(),&format!("Falta {key}"))?;}
    let input=parse(req["input"][0]["content"].as_str().ok_or("Contenido")?.as_bytes())?;
    let pages=input["fragmentos_documentales_completos"].as_array().ok_or("Fragmentos")?;
    let evidence=v["evidencias"].as_array().ok_or("Evidencias no enumeradas")?;need(!evidence.is_empty(),"Sin evidencias")?;
    let mut receipts=vec![];
    for q in evidence {
        let ordinal=num(&q["pagina_fisica"])?;need([5,8].contains(&ordinal)&&q["documento"]==DOC&&q["seccion"]==format!("PDF-P{:04}",ordinal-1),"Localizador ajeno al suministro")?;
        let ids=q["fragmentos"].as_array().ok_or("Fragmentos no enumerados")?;need(!ids.is_empty(),"Sin fragmentos citados")?;
        let mut previous=None;let mut text=String::new();
        for id in ids {let n=num(id)?;need(previous.is_none_or(|p|n==p+1),"Fragmentos repetidos, discontinuos o desordenados")?;previous=Some(n);
            let f=pages.iter().find(|f|f["pagina_pdf_ordinal"]==ordinal&&f["pagina"]==n).ok_or("Fragmento no entregado")?;text.push_str(f["texto"].as_str().ok_or("Texto")?);}
        let quote=q["cita_literal_breve"].as_str().filter(|s|!s.trim().is_empty()).ok_or("Cita vacía")?;
        need(norm(&text).contains(&norm(quote)),"Cita no literal en fragmentos indicados")?;
        receipts.push(json!({"pagina_fisica":ordinal,"fragmentos":ids,"literal_con_espacios_normalizados":true,"cita_sha256":sha(quote.as_bytes())}));
    }
    Ok(json!({"conforme":true,"citas":receipts,"alcance":"estructura declarada y pertenencia literal; no demuestra por sí sola suficiencia, fidelidad sustantiva ni ausencia de premisas externas"}))
}
pub fn infer(token:&str)->R<Value> {
    prepare()?;
    let m=sv_instrumentacion::Monitor::start_bounded(&base().join("instrumentacion"),330)?;
    m.event("fase",json!({"fase":"antes","caso":"PDF06"}))?;thread::sleep(Duration::from_secs(2));
    let operation=(||->R<Value>{m.healthy()?;comprobar()?;
        let req=bytes(&base().join("SOLICITUD.json"))?;
        let http=reqwest::blocking::Client::builder().https_only(true).no_proxy().redirect(reqwest::redirect::Policy::none()).retry(reqwest::retry::never()).connect_timeout(Duration::from_secs(10)).timeout(Duration::from_secs(300)).user_agent("SV-PDF-Astra/0.1.0").build().map_err(err)?;
        save(&base().join("envio-unico.json"),&json!({"utc_unix_ms":sv_instrumentacion::utc_ms(),"caso":"PDF06","solicitud_sha256":sha(&req),"previa_sha256":sha(&bytes(&base().join("PREVIA.json"))?),"catalogo_autorizado_sha256":sha(&bytes(&base().join("catalogo.json"))?),"reintentos":0,"herramientas":0,"limite_s":300,"credito_adicional_habilitado":false}))?;
        let t=Instant::now();m.event("fase",json!({"fase":"durante","solicitud_bytes":req.len(),"solicitud_sha256":sha(&req),"destino":"https://api.openai.com/v1/responses"}))?;
        let mut res=http.post("https://api.openai.com/v1/responses").bearer_auth(token).header("Content-Type","application/json").body(req.clone()).send().map_err(|_|"Transporte interrumpido; no repetir automáticamente")?;
        let status=res.status().as_u16();let mut headers=serde_json::Map::new();
        for n in ["content-type","x-request-id","openai-processing-ms","x-ratelimit-limit-requests","x-ratelimit-remaining-requests","x-ratelimit-remaining-tokens","x-ratelimit-reset-tokens","x-ratelimit-reset-requests"] {if let Some(v)=res.headers().get(n).and_then(|v|v.to_str().ok()){headers.insert(n.into(),json!(v));}}
        let h=json!({"status":status,"cabeceras_permitidas":headers,"tiempo_cabeceras_ms":t.elapsed().as_millis(),"version":format!("{:?}",res.version()),"remoto":res.remote_addr().map(|a|a.to_string())});save(&base().join("HTTP.json"),&h)?;
        let mut raw=OpenOptions::new().write(true).create_new(true).open(base().join("SALIDA-SSE.txt")).map_err(err)?;
        let mut s=Stream::default();let mut buffer=[0;8192];let mut count=0usize;let mut first=None;let mut first_text=None;
        loop {m.healthy()?;let n=res.read(&mut buffer).map_err(|_|"Lectura interrumpida; estado remoto no acreditado")?;if n==0 {break;}
            raw.write_all(&buffer[..n]).and_then(|_|raw.sync_data()).map_err(err)?;count+=n;need(count<=4*1024*1024,"Salida excesiva")?;
            m.event("lectura_https",json!({"ms":t.elapsed().as_millis(),"bytes":n,"acumulados":count}))?;
            for k in s.feed(&buffer[..n])? {first.get_or_insert(t.elapsed().as_millis());if k=="response.output_text.delta" {first_text.get_or_insert(t.elapsed().as_millis());}m.event("evento_sse",json!({"ms":t.elapsed().as_millis(),"tipo":k,"eventos_recibidos":s.events.len()}))?;}
        }
        raw.sync_all().map_err(err)?;need(status==200,"HTTP no satisfactorio")?;s.finish()?;
        let received=crate::catalogo::recepcion::extract(&s.events)?;let text=received["texto_original"].as_str().ok_or("Texto final")?;
        put(&base().join("FINAL.txt"),text.as_bytes())?;save(&base().join("ENTREGA-PROVEEDOR.json"),&received)?;
        let audit=match parse(text.as_bytes()).and_then(|v|formal(&v,&parse(&req)?)){Ok(v)=>v,Err(e)=>json!({"conforme":false,"defecto":e})};save(&base().join("AUDITORIA-FORMAL.json"),&audit)?;
        Ok(json!({"estado":"entrega_recibida","caso":"PDF06","completa":true,"utc_fin_ms":sv_instrumentacion::utc_ms(),"duracion_ms":t.elapsed().as_millis(),"primer_evento_ms":first,"primer_texto_ms":first_text,"eventos":s.events.len(),"respuesta_bytes":count,"uso_proveedor":received["uso_proveedor"],"modelo_solicitado":"gpt-6-astra","modelo_declarado":s.events.last().unwrap()["response"]["model"],"evento_terminal":s.events.last().unwrap()["type"],"http":status,"cabeceras_proveedor":headers,"par_remoto_http":h["remoto"],"version_http":h["version"],"respuesta_texto":text,"resumen_proveedor":received["resumen_proveedor"],"contrato_formal_conforme":audit["conforme"],"herramientas_habilitadas":0,"herramientas_ejecutadas":0,"adjudicado":false,"creditos_atribuibles":null,"coste_monetario_atribuible":null}))
    })();
    let end=m.event("fase",json!({"fase":"despues","entrega_obtenida":operation.is_ok()}));thread::sleep(Duration::from_secs(1));let telemetry=m.finish();
    let ok=end.is_ok()&&telemetry.as_ref().is_ok_and(|v|v["fallos_medicion"]==0&&v["intervalo_maximo_ms"].as_u64().is_some_and(|n|n<=750));
    let mut v=operation.unwrap_or_else(|cause|json!({"estado":"impedimento","caso":"PDF06","completa":false,"causa":cause,"envio_iniciado_o_incierto":base().join("envio-unico.json").exists(),"uso_proveedor":null,"creditos_atribuibles":null,"coste_monetario_atribuible":null,"reintento_automatico":false}));
    v["telemetria"]=telemetry.unwrap_or_else(|e|json!({"error":e}));v["telemetria_conforme"]=json!(ok);
    v["limites"]=json!(["CPU porcentual sin calibración externa","Sin medida separada DNS/TLS, RTT, retransmisiones, asignaciones del montículo, hilos ni recursos internos del proveedor","La ausencia de herramientas no prueba aislamiento de infraestructura de OpenAI; fuente exclusiva exigida y respuesta pendiente de contraste sustantivo","Criptografía C/ensamblador pendiente conforme a excepción experimental autorizada"]);
    save(&base().join("RESULTADO.json"),&v)?;Ok(v)
}
pub fn auditar()->R<Value> {
    comprobar()?;let r=load(&base().join("RESULTADO.json"))?;
    let tele=sv_instrumentacion::verify(&base().join("instrumentacion/telemetria.jsonl"))?;need(tele==r["telemetria"],"Recibo instrumental distinto")?;
    let raw=bytes(&base().join("SALIDA-SSE.txt"))?;let mut s=Stream::default();s.feed(&raw)?;s.finish()?;
    let received=crate::catalogo::recepcion::extract(&s.events)?;
    need(received==load(&base().join("ENTREGA-PROVEEDOR.json"))?&&received["uso_proveedor"]==r["uso_proveedor"],"Entrega o uso distintos")?;
    need(received["texto_original"].as_str().unwrap().as_bytes()==bytes(&base().join("FINAL.txt"))?,"Texto final distinto")?;
    let audit=match parse(received["texto_original"].as_str().unwrap().as_bytes()).and_then(|v|formal(&v,&load(&base().join("SOLICITUD.json"))?)){Ok(v)=>v,Err(e)=>json!({"conforme":false,"defecto":e})};
    need(audit==load(&base().join("AUDITORIA-FORMAL.json"))?,"Auditoría formal distinta")?;
    let v=json!({"conforme":true,"utc_ms":sv_instrumentacion::utc_ms(),"caso":"PDF06","solicitud_recompuesta":true,"sse_reproducido":true,"telemetria_cotejada":true,"telemetria_conforme":r["telemetria_conforme"],"contrato_formal_conforme":audit["conforme"],"sse_sha256":sha(&raw),"resultado_sha256":sha(&bytes(&base().join("RESULTADO.json"))?),"tokens":r["uso_proveedor"],"inferencias_en_este_cotejo":0,"adjudicacion_cientifica":false});save(&base().join("COTEJO-TRANSPORTE-RUST.json"),&v)?;Ok(v)
}
pub fn page(v:&Value)->String {sv_instrumentacion::dashboard(&base().join("instrumentacion"),v).unwrap_or_else(|_|format!("<!doctype html><meta charset='utf-8'><pre>{}</pre>",sv_instrumentacion::escape(&v.to_string()))).replace("Instrumentación de la segunda prueba","PDF06 · Suministro y transporte")}
pub fn start_page()->String {"<!doctype html><html lang='es'><meta charset='utf-8'><h1>SV · Astra · Prueba PDF06</h1><p>Una solicitud con las páginas físicas 5 y 8 completas, recibidas mediante MCP. Admisión y medición en Rust. Sin herramientas del candidato, sin reintentos. Máximo 300 segundos. Adjudicación científica separada.</p><a href='/start'>Continuar con ChatGPT e iniciar la prueba autorizada</a></html>".into()}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn sse_fragmentado_unicode(){let mut s=Stream::default();let b="data: {\"type\":\"response.created\",\"sequence_number\":0,\"x\":\"ñ\"}\n\ndata: {\"type\":\"response.completed\",\"sequence_number\":1}\n\n".as_bytes();for q in b {s.feed(&[*q]).unwrap();}s.finish().unwrap();assert_eq!(s.events.len(),2);}
    #[test] fn sse_no_truncado_ni_doble_cierre(){let mut s=Stream::default();s.feed(b"data: {\"type\":\"response.completed\",\"sequence_number\":0}\n").unwrap();s.finish().unwrap();assert!(s.feed(b"data: {\"type\":\"response.completed\",\"sequence_number\":1}\n").is_err());let mut s=Stream::default();s.feed(b"data: {").unwrap();assert!(s.finish().is_err());}
    #[test] fn sse_rechaza_herramientas_y_saltos(){for b in [b"data: {\"type\":\"response.output_item.added\",\"sequence_number\":0,\"item\":{\"type\":\"function_call\"}}\n".as_slice(),b"data: {\"type\":\"response.created\",\"sequence_number\":1}\n",b"data: [DONE]\n"] {assert!(Stream::default().feed(b).is_err());}}
    fn fixture()->(Value,Value){let input=json!({"fragmentos_documentales_completos":[{"pagina_pdf_ordinal":5,"pagina":0,"texto":"Remisión completa.\nSeguimiento"}]});(json!({"respuesta":"r","fundamentos_verificables":[],"insuficiencias":[],"evidencias":[{"documento":DOC,"pagina_fisica":5,"seccion":"PDF-P0004","fragmentos":[0],"cita_literal_breve":"Remisión completa. Seguimiento"}]}),json!({"input":[{"content":input.to_string()}]}))}
    #[test] fn citas_reales_y_falsas(){let(v,q)=fixture();formal(&v,&q).unwrap();for(k,x)in[("pagina_fisica",json!(8)),("fragmentos",json!([1])),("cita_literal_breve",json!("Inventado")),("fragmentos",json!([0,0]))]{let mut bad=v.clone();bad["evidencias"][0][k]=x;assert!(formal(&bad,&q).is_err());}}
}
