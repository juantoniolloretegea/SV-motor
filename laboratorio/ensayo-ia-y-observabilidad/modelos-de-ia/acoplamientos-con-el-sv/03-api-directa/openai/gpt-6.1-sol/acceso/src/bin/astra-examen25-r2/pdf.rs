//! Examen P01–P25. Contextos independientes; admisión y comprobación fuera del candidato.
use crate::{catalogo::{path, put, save}, suministro_pdf::*};
use serde_json::{json, Value};
use std::{fs::{self, File, OpenOptions}, io::{Read, Write}, path::{Path, PathBuf}, process::{Command, Stdio}, thread, time::{Duration, Instant}};
const RUN: &str = "ejecucion/astra-examen25-r2-20261007";
const ACCESS: &str = "desarrollo/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6.1-sol/acceso";
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
fn identity(p: &Path) -> R<Value> { let b=bytes(p)?; Ok(json!({"ruta":p.strip_prefix(crate::catalogo::ROOT).map_err(err)?.to_string_lossy(),"bytes":b.len(),"sha256":sha(&b)})) }
fn frozen()->R<Vec<Value>> {
    let mut paths=vec![std::env::current_exe().map_err(err)?];
    for f in ["ADMISION.md","fuentes/preguntas.json","fuentes/catalogo-pdq.json","fuentes/pdq.html","fuentes/CLAVE-CORRECCION.md","fuentes/PROTOCOLO.md","fuentes/MORFOLOGIA.md","suministro/CONTROL-ARBITRO.json","suministro/MCP-DIARIO.jsonl","suministro/MCP-SOLICITUDES.jsonl","suministro/MCP-RESPUESTAS.jsonl","suministro/instrumentacion/telemetria.jsonl"] {paths.push(base().join(f));}
    for f in ["Cargo.toml","Cargo.lock","src/bin/astra-examen25-r2/main.rs","src/bin/astra-examen25-r2/prueba.rs","src/bin/astra-examen25-r2/pdf.rs","src/bin/astra-examen25-r2/contrato.rs","src/bin/astra-examen25-r2/localizadores.rs","src/bin/astra-examen25-r2/suministro.rs","src/catalogo/mod.rs","src/catalogo/recepcion.rs","src/catalogo/estricto.rs","src/catalogo/contrato.rs"] {paths.push(path(&format!("{ACCESS}/{f}")));}
    for f in ["compilacion/mcp-pdf-linux/debug/sv-mcp-documental","compilacion/mcp-pdf-linux/debug/verificar-diario","desarrollo/acoplamientos-con-el-sv/instrumentacion-rust/src/lib.rs"] {paths.push(path(f));}
    for n in 1..=25 {paths.push(baseline(n));}
    paths.iter().map(|p|identity(p)).collect()
}fn baseline(n:usize)->PathBuf {base().join("fuentes-admitidas").join(format!("P{n:02}.json"))}
pub fn case(n:usize)->PathBuf {base().join("originales").join(format!("P{:02}",(n-1)/3+1)).join(format!("R{}",(n-1)%3))}
fn request(n:usize)->R<Value>{
    need((1..=75).contains(&n),"Número de entrega ajeno")?;
    let first=(n-1)/3*3+1;let stage=(n-1)%3;let mut h=vec![];
    for prior in first..n {
        let dest=case(prior);let receipt=load(&base().join(format!("hitos/ENTREGA-{prior:02}/HITO-INSTRUMENTAL.json")))?;
        for record in receipt["originales"].as_array().ok_or("Hito incompleto")? {
            let file=path(record["ruta"].as_str().ok_or("Ruta de hito")?);
            need(identity(&file)?==*record,"Historia o recibo anterior alterados")?;
        }
        let result=load(&dest.join("RESULTADO.json"))?;
        need(result["completa"]==true&&result["telemetria_conforme"]==true,"Historia no recibida íntegramente")?;
        let mut stream=Stream::default();stream.feed(&bytes(&dest.join("SALIDA-SSE.txt"))?)?;stream.finish()?;
        let got=crate::catalogo::recepcion::extract(&stream.events)?;
        let text=String::from_utf8(bytes(&dest.join("FINAL.txt"))?).map_err(err)?;
        need(got==load(&dest.join("ENTREGA-PROVEEDOR.json"))?&&got["texto_original"]==text,"Historia sin correspondencia con proveedor")?;
        h.push(text);
    }
    crate::contrato::compose(&load(&baseline((n-1)/3+1))?,stage,&h)
}
pub fn preparar()->R<Value>{
    need(!base().join("INICIO.json").exists(),"Preparación existente: conservar")?;
    need(base().join("ADMISION.md").is_file(),"Falta admisión")?;
    save(&base().join("INICIO.json"),&json!({"utc_ms":sv_instrumentacion::utc_ms(),"suceso":"S39","tique":"TT-0016","transporte":"TT-0021","casos":25,"inferencias":0}))?;
    let control=crate::suministro::preparar(&base())?;
    for n in 1..=25 {need(bytes(&baseline(n))?==bytes(&path(&format!("ejecucion/astra-examen25-20261007/fuentes-admitidas/P{n:02}.json")))?,"Cambio de fuente admitida entre revisiones")?;}
    need(request(1)?==load(&path("ejecucion/astra-examen25-20261007/originales/P01/R0/SOLICITUD.json"))?,"Cambio de solicitud inicial entre revisiones")?;
    save(&base().join("IDENTIDAD-ENTRE-REVISIONES.json"),&json!({"conforme":true,"fuentes_admitidas_identicas":25,"solicitud_P01_R0_identica":true,"inferencias_anteriores_interrumpidas":1,"tiempo_anteriores_ms":36603,"limite_revision_s":5363,"limite_conjunto_s":5400,"cambio":"Receptor de mantenimiento keepalive; sin cambio del contrato del candidato"}))?;
    let v=json!({"conforme":true,"utc_ms":sv_instrumentacion::utc_ms(),"archivos_fijados":frozen()?,"control":control,"envios_autorizados":75,"reintentos":0,"limite_por_entrega_s":300,"limite_global_s":5363,"inferencia_ejecutada":false,"casos_independientes":true});
    save(&base().join("PREVIA.json"),&v)?;Ok(v)
}
fn comprobar()->R<()>{let v=load(&base().join("PREVIA.json"))?;need(v["conforme"]==true&&v["envios_autorizados"]==75,"Admisión ajena")?;
    need(v["archivos_fijados"]==json!(frozen()?),"Cambió fuente o ejecutable fijado")?;
    crate::suministro::verificar(&base())?;Ok(())}pub fn prepare()->R<()>{need(!base().join("envio-unico.json").exists(),"Banco ya iniciado; no repetir")?;comprobar()}
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
                need(matches!(kind,"keepalive"|"response.created"|"response.in_progress"|"response.output_item.added"|"response.content_part.added"|"response.output_text.delta"|"response.output_text.done"|"response.content_part.done"|"response.output_item.done"|"response.completed"|"response.failed"|"response.incomplete"|"error"|"response.reasoning_summary_part.added"|"response.reasoning_summary_part.done"|"response.reasoning_summary_text.delta"|"response.reasoning_summary_text.done"),"Evento ajeno al contrato")?;
                if kind=="keepalive" {need(v.as_object().is_some_and(|o|o.len()==2&&o.contains_key("type")&&o.contains_key("sequence_number")),"Mantenimiento con contenido ajeno")?;}
                if v.get("item").is_some(){need(matches!(v["item"]["type"].as_str(),Some("message"|"reasoning")),"Herramienta rechazada, no ejecutada")?;}
                self.terminal=matches!(kind,"response.completed"|"response.failed"|"response.incomplete"|"error");kinds.push(kind.to_string());self.events.push(v);
            } else {need(s.is_empty()||s.starts_with(':')||s.starts_with("event:"),"Campo SSE no admitido")?;}
        }Ok(kinds)
    }
    fn finish(&self) -> R<()> {need(self.terminal&&self.pending.iter().all(u8::is_ascii_whitespace),"Entrega incompleta")}
}
fn formal(v:&Value,req:&Value)->R<Value>{crate::contrato::formal(v,req)}
pub fn infer(token:&str)->R<Value>{
    prepare()?;save(&base().join("envio-unico.json"),&json!({"utc_ms":sv_instrumentacion::utc_ms(),"maximo":75,"reintentos":0,"previa_sha256":sha(&bytes(&base().join("PREVIA.json"))?)}))?;
    let start=Instant::now();let mut rows=vec![];
    for n in 1..=75 {
        need(start.elapsed()<Duration::from_secs(5363-330),"Límite global antes de otro envío")?;
        save(&case(n).join("SOLICITUD.json"),&request(n)?)?;
        let r=one(n,token)?;
        // Hito instrumental durable antes de decidir el caso siguiente. Sin adjudicación fingida.
        let files=["SOLICITUD.json","RESULTADO.json","HTTP.json","SALIDA-SSE.txt","FINAL.txt","ENTREGA-PROVEEDOR.json","AUDITORIA-FORMAL.json","instrumentacion/telemetria.jsonl"].iter().filter(|p|case(n).join(p).is_file()).map(|p|identity(&case(n).join(p))).collect::<R<Vec<_>>>()?;
        save(&base().join(format!("hitos/ENTREGA-{n:02}/HITO-INSTRUMENTAL.json")),&json!({"esquema":"sv-hito-instrumental/1","nodo":"03","modelo":"gpt-6-astra","caso":format!("P{:02}",(n-1)/3+1),"etapa":(n-1)%3,"fecha_registro_ms":sv_instrumentacion::utc_ms(),"momento":"inmediatamente tras la entrega y el cierre de observación","resultado":r,"originales":files,"adjudicacion":"pendiente de contraste externo al candidato","custodia_remota":false}))?;
        let proceed=r["completa"]==true&&r["telemetria_conforme"]==true;
        rows.push(r);if !proceed {break;}
    }
    let v=json!({"estado":if rows.len()==75&&rows.iter().all(|r|r["completa"]==true&&r["telemetria_conforme"]==true){"setenta_y_cinco_entregas_recibidas"}else{"detenido_por_incidencia"},"casos":rows,"duracion_banco_ms":start.elapsed().as_millis(),"inferencias_iniciadas":rows.len(),"reintentos":0,"adjudicacion_cientifica":false});
    save(&base().join("RESULTADO-BANCO.json"),&v)?;Ok(v)
}
fn one(n:usize,token:&str)->R<Value> {
    comprobar()?;let dest=case(n);let id=format!("P{:02}-R{}",(n-1)/3+1,(n-1)%3);
    let m=sv_instrumentacion::Monitor::start_bounded(&dest.join("instrumentacion"),330)?;
    m.event("fase",json!({"fase":"antes","caso":id}))?;thread::sleep(Duration::from_secs(2));
    let operation=(||->R<Value>{m.healthy()?;comprobar()?;
        let req=bytes(&dest.join("SOLICITUD.json"))?;need(parse(&req)?==request(n)?,"Solicitud distinta de la composición admitida")?;
        let http=reqwest::blocking::Client::builder().https_only(true).no_proxy().redirect(reqwest::redirect::Policy::none()).retry(reqwest::retry::never()).connect_timeout(Duration::from_secs(10)).timeout(Duration::from_secs(300)).user_agent("SV-Examen25-Astra/1.0.1").build().map_err(err)?;
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
        need(load(&dest.join("SOLICITUD.json"))?==request(n)?,"Historia efectiva discordante")?;let raw=bytes(&dest.join("SALIDA-SSE.txt"))?;let mut stream=Stream::default();stream.feed(&raw)?;stream.finish()?;
        let got=crate::catalogo::recepcion::extract(&stream.events)?;need(got==load(&dest.join("ENTREGA-PROVEEDOR.json"))?&&got["uso_proveedor"]==r["uso_proveedor"],"Entrega o uso discordante")?;
        need(got["texto_original"].as_str().ok_or("Texto")?.as_bytes()==bytes(&dest.join("FINAL.txt"))?,"Texto distinto")?;
        let check=match parse(got["texto_original"].as_str().unwrap().as_bytes()).and_then(|v|formal(&v,&load(&dest.join("SOLICITUD.json"))?)){Ok(v)=>v,Err(e)=>json!({"conforme":false,"defecto":e})};
        need(check==load(&dest.join("AUDITORIA-FORMAL.json"))?,"Cotejo no reproducible")?;
        let u=&got["uso_proveedor"];let mut ai=0;let mut ao=0;
        for g in ["items","request_fields"]{for v in u["attribution"][g].as_object().ok_or("Atribución ausente")?.values(){ai+=num(&v["input_tokens"])?;ao+=num(&v["output_tokens"])?;}}
        need(ai==num(&u["input_tokens"])?&&ao==num(&u["output_tokens"])?&&ai+ao==num(&u["total_tokens"])?,"Atribución de tokens distinta")?;
        proofs.push(json!({"caso":format!("P{:02}",(n-1)/3+1),"etapa":(n-1)%3,"conforme":true,"contrato_formal_conforme":check["conforme"],"original_resultado_sha256":sha(&bytes(&dest.join("RESULTADO.json"))?),"sse_sha256":sha(&raw),"telemetria":t,"uso_proveedor":u,"atribucion_cotejada":true}));
    }
    let v=json!({"conforme":true,"casos":proofs,"inferencias_nuevas":0,"adjudicacion_cientifica":false,"utc_ms":sv_instrumentacion::utc_ms()});save(&base().join("COTEJO-BANCO-RUST.json"),&v)?;Ok(v)
}
pub fn page(v:&Value)->String {format!("<!doctype html><html lang='es'><meta charset='utf-8'><h1>SV · Astra · Examen documental de 25 preguntas</h1><p>Entrega instrumental del banco. Adjudicación científica separada.</p><pre>{}</pre></html>",sv_instrumentacion::escape(&serde_json::to_string_pretty(v).unwrap()))}
pub fn start_page()->String {"<!doctype html><html lang='es'><meta charset='utf-8'><h1>SV · Astra · Respuesta, autocrítica y verificación neutral</h1><p>Veinticinco preguntas, tres etapas por pregunta: 75 solicitudes como máximo. Secciones completas recibidas mediante MCP e historial exclusivo de cada pregunta. Control y observación en Rust. Sin herramientas del candidato ni reintentos. Máximo 300 segundos por solicitud y 90 minutos de ensayo. Adjudicación posterior, externa al candidato.</p><a href='/start'>Continuar con ChatGPT e iniciar las tres etapas autorizadas</a></html>".into()}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn mantenimiento_observado_no_equivale_a_respuesta(){let mut s=Stream::default();s.feed(&std::fs::read(path("ejecucion/astra-examen25-20261007/originales/P01/R0/SALIDA-SSE.txt")).unwrap()).unwrap();assert_eq!(s.events.len(),3);assert_eq!(s.events[2]["type"],"keepalive");assert!(s.finish().is_err());assert!(crate::catalogo::recepcion::extract(&s.events).is_err());}
    #[test] fn mantenimiento_rechaza_contenido_saltos_y_poscierre(){for b in [b"data: {\"type\":\"keepalive\",\"sequence_number\":0,\"item\":{\"type\":\"function_call\"}}\n".as_slice(),b"data: {\"type\":\"keepalive\",\"sequence_number\":1}\n"]{assert!(Stream::default().feed(b).is_err());}let mut s=Stream::default();s.feed(b"data: {\"type\":\"response.completed\",\"sequence_number\":0}\n").unwrap();assert!(s.feed(b"data: {\"type\":\"keepalive\",\"sequence_number\":1}\n").is_err());}
    #[test] fn mantenimiento_conserva_texto_y_uso(){let original=std::fs::read(path("ejecucion/astra-pdf-doble-20261007/originales/PDF01/R0/SALIDA-SSE.txt")).unwrap();let mut a=Stream::default();a.feed(&original).unwrap();a.finish().unwrap();let expected=crate::catalogo::recepcion::extract(&a.events).unwrap();let mut events=a.events.clone();events.insert(2,json!({"type":"keepalive","sequence_number":2}));let mut b=Stream::default();for(i,v)in events.iter_mut().enumerate(){v["sequence_number"]=json!(i);b.feed(format!("data: {}\n\n",v).as_bytes()).unwrap();}b.finish().unwrap();assert_eq!(crate::catalogo::recepcion::extract(&b.events).unwrap(),expected);}
    #[test] fn sse_fragmentado_unicode(){let mut s=Stream::default();let b="data: {\"type\":\"response.created\",\"sequence_number\":0,\"x\":\"ñ\"}\n\ndata: {\"type\":\"response.completed\",\"sequence_number\":1}\n\n".as_bytes();for q in b {s.feed(&[*q]).unwrap();}s.finish().unwrap();assert_eq!(s.events.len(),2);}
    #[test] fn sse_no_truncado_ni_doble_cierre(){let mut s=Stream::default();s.feed(b"data: {\"type\":\"response.completed\",\"sequence_number\":0}\n").unwrap();s.finish().unwrap();assert!(s.feed(b"data: {\"type\":\"response.completed\",\"sequence_number\":1}\n").is_err());let mut s=Stream::default();s.feed(b"data: {").unwrap();assert!(s.finish().is_err());}
    #[test] fn sse_rechaza_herramientas_y_saltos(){for b in [b"data: {\"type\":\"response.output_item.added\",\"sequence_number\":0,\"item\":{\"type\":\"function_call\"}}\n".as_slice(),b"data: {\"type\":\"response.created\",\"sequence_number\":1}\n",b"data: [DONE]\n"] {assert!(Stream::default().feed(b).is_err());}}
}
