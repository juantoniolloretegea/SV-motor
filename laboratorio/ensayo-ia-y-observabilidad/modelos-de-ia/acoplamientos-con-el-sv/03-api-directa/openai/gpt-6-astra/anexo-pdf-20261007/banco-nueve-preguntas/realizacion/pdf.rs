//! Banco PDF01–PDF09. Contextos independientes; admisión y comprobación fuera del candidato.
use crate::{catalogo::{path, put, save}, suministro_pdf::*};
use serde_json::{json, Value};
use std::{fs::{self, File, OpenOptions}, io::{Read, Write}, path::{Path, PathBuf}, process::{Command, Stdio}, thread, time::{Duration, Instant}};
const RUN: &str = "ejecucion/astra-pdf-banco-20261007-r2";
const SUPPLY: &str = "ejecucion/astra-pdf-20261007/recepcion-banco-r2";
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
    for n in ["CONTROL-ARBITRO.json","COTEJO-SUMINISTRO-RUST.json","documento/CATALOGO.json","documento/EXTRACCION.json","documento/FUENTES.json","MCP-SOLICITUDES.jsonl","MCP-RESPUESTAS.jsonl","MCP-DIARIO.jsonl","AISLAMIENTO-RESPUESTAS.jsonl","VERIFICACION-DIARIO-RESPUESTAS.jsonl","instrumentacion/telemetria.jsonl"] {paths.push(supply(n));}
    for n in ["Cargo.toml","Cargo.lock","src/bin/astra-pdf-banco/main.rs","src/bin/astra-pdf-banco/prueba.rs","src/bin/astra-pdf-banco/pdf.rs","src/bin/astra-pdf-banco/localizadores.rs","src/catalogo/recepcion.rs","src/catalogo/estricto.rs"] {paths.push(path(&format!("{ACCESS}/{n}")));}
    for n in ["lib.rs","estricto.rs"] {paths.push(path(&format!("{ANNEX}/instrumento/src/{n}")));}
    paths.push(path("desarrollo/acoplamientos-con-el-sv/instrumentacion-rust/src/lib.rs"));
    for n in ["src/catalogo/mod.rs","src/catalogo/contrato.rs"] {paths.push(path(&format!("{ACCESS}/{n}")));}
    paths.push(base().join("ADMISION.md"));
    for n in 1..=9 {paths.push(case(n).join("SOLICITUD.json"));}
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
pub fn case(n:usize)->PathBuf {base().join("originales").join(format!("PDF{n:02}"))}
pub fn preparar()->R<Value>{
    need(!base().join("INICIO.json").exists()&&!path(SUPPLY).exists(),"Preparación existente: conservar")?;
    need(base().join("ADMISION.md").is_file(),"Falta admisión anterior al candidato")?;
    save(&base().join("INICIO.json"),&json!({"utc_ms":sv_instrumentacion::utc_ms(),"suceso":"S39","tique":"TT-0021","casos":9,"inferencias":0}))?;
    let m=sv_instrumentacion::Monitor::start_bounded(&base().join("preparacion-instrumentacion"),210)?;
    let op=(||{local("preparar")?;local("verificar")?;let d=director()?;
        for n in 1..=9 {let b=bytes(&supply(&format!("solicitudes-previstas/PDF{n:02}.json")))?;d.cotejar_solicitud(n-1,&parse(&b)?)?;put(&case(n).join("SOLICITUD.json"),&b)?;}
        frozen()})();
    let measured=m.finish()?;let files=op?;need(measured["fallos_medicion"]==0,"Medición previa degradada")?;
    let v=json!({"conforme":true,"utc_ms":sv_instrumentacion::utc_ms(),"suceso":"S39","tique":"TT-0021","archivos_fijados":files,"telemetria":measured,"envios_autorizados":9,"reintentos":0,"limite_por_caso_s":300,"inferencia_ejecutada":false,"casos_independientes":true});
    save(&base().join("PREVIA.json"),&v)?;Ok(v)
}
fn comprobar()->R<()>{let v=load(&base().join("PREVIA.json"))?;need(v["conforme"]==true&&v["envios_autorizados"]==9,"Admisión ajena")?;
    need(v["archivos_fijados"]==json!(frozen()?),"Cambió fuente o ejecutable fijado")?;
    let d=director()?;for n in 1..=9 {d.cotejar_solicitud(n-1,&load(&case(n).join("SOLICITUD.json"))?)?;}Ok(())}
pub fn prepare()->R<()>{need(!base().join("envio-unico.json").exists(),"Banco ya iniciado; no repetir")?;comprobar()}
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
fn formal(v:&Value,req:&Value)->R<Value>{crate::localizadores::cotejar(v,req)}
pub fn infer(token:&str)->R<Value>{
    prepare()?;save(&base().join("envio-unico.json"),&json!({"utc_ms":sv_instrumentacion::utc_ms(),"maximo":9,"reintentos":0,"previa_sha256":sha(&bytes(&base().join("PREVIA.json"))?)}))?;
    let start=Instant::now();let mut rows=vec![];
    for n in 1..=9 {
        let r=one(n,token)?;
        // Hito instrumental durable antes de decidir el caso siguiente. Sin adjudicación fingida.
        let files=["SOLICITUD.json","RESULTADO.json","HTTP.json","SALIDA-SSE.txt","FINAL.txt","ENTREGA-PROVEEDOR.json","AUDITORIA-FORMAL.json","instrumentacion/telemetria.jsonl"].iter().filter(|p|case(n).join(p).is_file()).map(|p|identity(&case(n).join(p))).collect::<R<Vec<_>>>()?;
        save(&base().join(format!("hitos/PDF{n:02}/HITO-INSTRUMENTAL.json")),&json!({"esquema":"sv-hito-instrumental/1","nodo":"03","modelo":"gpt-6-astra","caso":format!("PDF{n:02}"),"fecha_registro_ms":sv_instrumentacion::utc_ms(),"momento":"inmediatamente tras la entrega y el cierre de observación","resultado":r,"originales":files,"adjudicacion":"pendiente de contraste externo al candidato","custodia_remota":false}))?;
        let proceed=r["completa"]==true&&r["telemetria_conforme"]==true;
        rows.push(r);if !proceed {break;}
    }
    let v=json!({"estado":if rows.len()==9&&rows.iter().all(|r|r["completa"]==true&&r["telemetria_conforme"]==true){"nueve_entregas_recibidas"}else{"detenido_por_incidencia"},"casos":rows,"duracion_banco_ms":start.elapsed().as_millis(),"inferencias_iniciadas":rows.len(),"reintentos":0,"adjudicacion_cientifica":false});
    save(&base().join("RESULTADO-BANCO.json"),&v)?;Ok(v)
}
fn one(n:usize,token:&str)->R<Value> {
    comprobar()?;let dest=case(n);let id=format!("PDF{n:02}");
    let m=sv_instrumentacion::Monitor::start_bounded(&dest.join("instrumentacion"),330)?;
    m.event("fase",json!({"fase":"antes","caso":id}))?;thread::sleep(Duration::from_secs(2));
    let operation=(||->R<Value>{m.healthy()?;comprobar()?;
        let req=bytes(&dest.join("SOLICITUD.json"))?;
        let http=reqwest::blocking::Client::builder().https_only(true).no_proxy().redirect(reqwest::redirect::Policy::none()).retry(reqwest::retry::never()).connect_timeout(Duration::from_secs(10)).timeout(Duration::from_secs(300)).user_agent("SV-PDF-Astra/0.2.0").build().map_err(err)?;
        save(&dest.join("envio-unico.json"),&json!({"utc_unix_ms":sv_instrumentacion::utc_ms(),"caso":id,"solicitud_sha256":sha(&req),"previa_sha256":sha(&bytes(&base().join("PREVIA.json"))?),"catalogo_autorizado_sha256":sha(&bytes(&base().join("catalogo.json"))?),"reintentos":0,"herramientas":0,"limite_s":300,"credito_adicional_habilitado":false}))?;
        let t=Instant::now();m.event("fase",json!({"fase":"durante","solicitud_bytes":req.len(),"solicitud_sha256":sha(&req),"destino":"https://api.openai.com/v1/responses"}))?;
        let mut res=http.post("https://api.openai.com/v1/responses").bearer_auth(token).header("Content-Type","application/json").body(req.clone()).send().map_err(|_|"Transporte interrumpido; no repetir automáticamente")?;
        let status=res.status().as_u16();let mut headers=serde_json::Map::new();
        for n in ["content-type","x-request-id","openai-processing-ms","x-ratelimit-limit-requests","x-ratelimit-remaining-requests","x-ratelimit-remaining-tokens","x-ratelimit-reset-tokens","x-ratelimit-reset-requests"] {if let Some(v)=res.headers().get(n).and_then(|v|v.to_str().ok()){headers.insert(n.into(),json!(v));}}
        let h=json!({"status":status,"cabeceras_permitidas":headers,"tiempo_cabeceras_ms":t.elapsed().as_millis(),"version":format!("{:?}",res.version()),"remoto":res.remote_addr().map(|a|a.to_string())});save(&dest.join("HTTP.json"),&h)?;
        let mut raw=OpenOptions::new().write(true).create_new(true).open(dest.join("SALIDA-SSE.txt")).map_err(err)?;
        let mut s=Stream::default();let mut buffer=[0;8192];let mut count=0usize;let mut first=None;let mut first_text=None;
        loop {m.healthy()?;let n=res.read(&mut buffer).map_err(|_|"Lectura interrumpida; estado remoto no acreditado")?;if n==0 {break;}
            raw.write_all(&buffer[..n]).and_then(|_|raw.sync_data()).map_err(err)?;count+=n;need(count<=4*1024*1024,"Salida excesiva")?;
            m.event("lectura_https",json!({"ms":t.elapsed().as_millis(),"bytes":n,"acumulados":count}))?;
            for k in s.feed(&buffer[..n])? {first.get_or_insert(t.elapsed().as_millis());if k=="response.output_text.delta" {first_text.get_or_insert(t.elapsed().as_millis());}m.event("evento_sse",json!({"ms":t.elapsed().as_millis(),"tipo":k,"eventos_recibidos":s.events.len()}))?;}
        }
        raw.sync_all().map_err(err)?;need(status==200,"HTTP no satisfactorio")?;s.finish()?;
        let received=crate::catalogo::recepcion::extract(&s.events)?;let text=received["texto_original"].as_str().ok_or("Texto final")?;
        put(&dest.join("FINAL.txt"),text.as_bytes())?;save(&dest.join("ENTREGA-PROVEEDOR.json"),&received)?;
        let audit=match parse(text.as_bytes()).and_then(|v|formal(&v,&parse(&req)?)){Ok(v)=>v,Err(e)=>json!({"conforme":false,"defecto":e})};save(&dest.join("AUDITORIA-FORMAL.json"),&audit)?;
        Ok(json!({"estado":"entrega_recibida","caso":id,"completa":true,"utc_fin_ms":sv_instrumentacion::utc_ms(),"duracion_ms":t.elapsed().as_millis(),"primer_evento_ms":first,"primer_texto_ms":first_text,"eventos":s.events.len(),"respuesta_bytes":count,"uso_proveedor":received["uso_proveedor"],"modelo_solicitado":"gpt-6-astra","modelo_declarado":s.events.last().unwrap()["response"]["model"],"evento_terminal":s.events.last().unwrap()["type"],"http":status,"cabeceras_proveedor":headers,"par_remoto_http":h["remoto"],"version_http":h["version"],"respuesta_texto":text,"resumen_proveedor":received["resumen_proveedor"],"contrato_formal_conforme":audit["conforme"],"herramientas_habilitadas":0,"herramientas_ejecutadas":0,"adjudicado":false,"creditos_atribuibles":null,"coste_monetario_atribuible":null}))
    })();
    let end=m.event("fase",json!({"fase":"despues","entrega_obtenida":operation.is_ok()}));thread::sleep(Duration::from_secs(1));let telemetry=m.finish();
    let ok=end.is_ok()&&telemetry.as_ref().is_ok_and(|v|v["fallos_medicion"]==0&&v["intervalo_maximo_ms"].as_u64().is_some_and(|n|n<=750));
    let mut v=operation.unwrap_or_else(|cause|json!({"estado":"impedimento","caso":id,"completa":false,"causa":cause,"envio_iniciado_o_incierto":dest.join("envio-unico.json").exists(),"uso_proveedor":null,"creditos_atribuibles":null,"coste_monetario_atribuible":null,"reintento_automatico":false}));
    v["telemetria"]=telemetry.unwrap_or_else(|e|json!({"error":e}));v["telemetria_conforme"]=json!(ok);
    v["limites"]=json!(["CPU porcentual sin calibración externa","Sin medida separada DNS/TLS, RTT, retransmisiones, asignaciones del montículo, hilos ni recursos internos del proveedor","La ausencia de herramientas no prueba aislamiento de infraestructura de OpenAI; fuente exclusiva exigida y respuesta pendiente de contraste sustantivo","Criptografía C/ensamblador pendiente conforme a excepción experimental autorizada"]);
    save(&dest.join("RESULTADO.json"),&v)?;Ok(v)
}
pub fn auditar()->R<Value>{
    comprobar()?;let bank=load(&base().join("RESULTADO-BANCO.json"))?;let mut proofs=vec![];
    for (i,r) in bank["casos"].as_array().ok_or("Casos")?.iter().enumerate(){let n=i+1;let dest=case(n);
        need(*r==load(&dest.join("RESULTADO.json"))?,"Resultado alterado")?;
        let t=sv_instrumentacion::verify(&dest.join("instrumentacion/telemetria.jsonl"))?;need(t==r["telemetria"],"Telemetría distinta")?;
        let raw=bytes(&dest.join("SALIDA-SSE.txt"))?;let mut stream=Stream::default();stream.feed(&raw)?;stream.finish()?;
        let got=crate::catalogo::recepcion::extract(&stream.events)?;need(got==load(&dest.join("ENTREGA-PROVEEDOR.json"))?&&got["uso_proveedor"]==r["uso_proveedor"],"Entrega o uso discordante")?;
        need(got["texto_original"].as_str().ok_or("Texto")?.as_bytes()==bytes(&dest.join("FINAL.txt"))?,"Texto distinto")?;
        let check=match parse(got["texto_original"].as_str().unwrap().as_bytes()).and_then(|v|formal(&v,&load(&dest.join("SOLICITUD.json"))?)){Ok(v)=>v,Err(e)=>json!({"conforme":false,"defecto":e})};
        need(check==load(&dest.join("AUDITORIA-FORMAL.json"))?,"Cotejo no reproducible")?;
        let u=&got["uso_proveedor"];let mut ai=0;let mut ao=0;
        for g in ["items","request_fields"]{for v in u["attribution"][g].as_object().ok_or("Atribución ausente")?.values(){ai+=num(&v["input_tokens"])?;ao+=num(&v["output_tokens"])?;}}
        need(ai==num(&u["input_tokens"])?&&ao==num(&u["output_tokens"])?&&ai+ao==num(&u["total_tokens"])?,"Atribución de tokens distinta")?;
        proofs.push(json!({"caso":format!("PDF{n:02}"),"conforme":true,"contrato_formal_conforme":check["conforme"],"original_resultado_sha256":sha(&bytes(&dest.join("RESULTADO.json"))?),"sse_sha256":sha(&raw),"telemetria":t,"uso_proveedor":u,"atribucion_cotejada":true}));
    }
    let v=json!({"conforme":true,"casos":proofs,"inferencias_nuevas":0,"adjudicacion_cientifica":false,"utc_ms":sv_instrumentacion::utc_ms()});save(&base().join("COTEJO-BANCO-RUST.json"),&v)?;Ok(v)
}
pub fn page(v:&Value)->String {format!("<!doctype html><html lang='es'><meta charset='utf-8'><h1>SV · Astra · Anexo PDF</h1><p>Entrega instrumental del banco. Adjudicación científica separada.</p><pre>{}</pre></html>",sv_instrumentacion::escape(&serde_json::to_string_pretty(v).unwrap()))}
pub fn start_page()->String {"<!doctype html><html lang='es'><meta charset='utf-8'><h1>SV · Astra · Nueve preguntas PDF</h1><p>Nueve solicitudes independientes con las páginas completas recibidas mediante MCP. Control y observación en Rust. Sin herramientas del candidato ni reintentos. Máximo 300 segundos por caso. Adjudicación posterior, externa al candidato.</p><a href='/start'>Continuar con ChatGPT e iniciar el banco autorizado</a></html>".into()}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn sse_fragmentado_unicode(){let mut s=Stream::default();let b="data: {\"type\":\"response.created\",\"sequence_number\":0,\"x\":\"ñ\"}\n\ndata: {\"type\":\"response.completed\",\"sequence_number\":1}\n\n".as_bytes();for q in b {s.feed(&[*q]).unwrap();}s.finish().unwrap();assert_eq!(s.events.len(),2);}
    #[test] fn sse_no_truncado_ni_doble_cierre(){let mut s=Stream::default();s.feed(b"data: {\"type\":\"response.completed\",\"sequence_number\":0}\n").unwrap();s.finish().unwrap();assert!(s.feed(b"data: {\"type\":\"response.completed\",\"sequence_number\":1}\n").is_err());let mut s=Stream::default();s.feed(b"data: {").unwrap();assert!(s.finish().is_err());}
    #[test] fn sse_rechaza_herramientas_y_saltos(){for b in [b"data: {\"type\":\"response.output_item.added\",\"sequence_number\":0,\"item\":{\"type\":\"function_call\"}}\n".as_slice(),b"data: {\"type\":\"response.created\",\"sequence_number\":1}\n",b"data: [DONE]\n"] {assert!(Stream::default().feed(b).is_err());}}
    fn fixture()->(Value,Value){let input=json!({"paginas_fisicas":[5],"fragmentos_documentales_completos":[{"pagina_pdf_ordinal":5,"pagina":0,"texto":"Remisión completa.\nSeguimiento"}]});(json!({"respuesta":"r","fundamentos_verificables":[],"insuficiencias":[],"evidencias":[{"documento":DOC,"pagina_fisica":5,"seccion":"PDF-P0004","fragmentos":[0],"cita_literal_breve":"Remisión completa. Seguimiento"}]}),json!({"input":[{"content":input.to_string()}]}))}
    #[test] fn citas_reales_y_falsas(){let(v,q)=fixture();formal(&v,&q).unwrap();for(k,x)in[("pagina_fisica",json!(8)),("fragmentos",json!([1])),("cita_literal_breve",json!("Inventado")),("fragmentos",json!([0,0]))]{let mut bad=v.clone();bad["evidencias"][0][k]=x;assert!(formal(&bad,&q).is_err());}}
}
