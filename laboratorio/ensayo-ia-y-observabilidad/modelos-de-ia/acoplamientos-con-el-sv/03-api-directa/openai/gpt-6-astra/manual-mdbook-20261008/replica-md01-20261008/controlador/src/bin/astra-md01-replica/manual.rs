//! Ensayo MD01: réplica diagnóstica. Contextos independientes; admisión y comprobación fuera del candidato.
use crate::{catalogo::{path, put, save}, suministro_pdf::*};
use serde_json::{json, Value};
use std::{fs::{self, File, OpenOptions}, io::{Read, Write}, path::{Path, PathBuf}, thread, time::{Duration, Instant}};
const RUN: &str = "ejecucion/astra-md01-replica-20261008";
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
static ATTEMPT:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
fn attempt()->usize{ATTEMPT.load(std::sync::atomic::Ordering::SeqCst)}
fn frozen()->R<Vec<Value>>{
 let mut pp=vec![std::env::current_exe().map_err(err)?,base().join("ADMISION.md"),base().join("suministro/CONTROL-ARBITRO.json"),base().join("fuentes/BANCO.json"),base().join("reservado/CLAVE.json")];
 for f in ["main.rs","prueba.rs","manual.rs","contrato.rs","suministro.rs","servicio.rs","preparacion.rs"]{pp.push(path(&format!("{ACCESS}/src/bin/astra-md01-replica/{f}")));}
 for f in ["contrato.rs","localizadores.rs"]{pp.push(path(&format!("{ACCESS}/src/bin/astra-manual/{f}")));}
 for f in ["Cargo.toml","Cargo.lock","src/catalogo/mod.rs","src/catalogo/recepcion.rs","src/catalogo/estricto.rs","src/catalogo/contrato.rs"]{pp.push(path(&format!("{ACCESS}/{f}")));}
 for f in ["desarrollo/acoplamientos-con-el-sv/instrumentacion-rust/src/lib.rs","desarrollo/acoplamientos-con-el-sv/instrumentacion-rust/Cargo.toml","desarrollo/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/anexo-pdf-20261007/instrumento/src/lib.rs","compilacion/mcp-mdbook/debug/sv-mcp-documental","compilacion/mcp-mdbook/debug/verificar-diario"]{pp.push(path(f));}
 pp.iter().map(|p|identity(p)).collect()
}
fn baseline(n:usize)->PathBuf{base().join(format!("fuentes-admitidas/MD{n:02}.json"))}
pub fn case(n:usize)->PathBuf{base().join(format!("intentos/ENVIO-{:03}/MD{:02}/R{}",attempt(),(n-1)/3+1,(n-1)%3))}
fn completed(n:usize)->R<Option<PathBuf>>{
 
 let mut found=None;
 let d=base().join("hitos");if !d.exists(){return Ok(None);}
 for f in fs::read_dir(d).map_err(err)?{let h=load(&f.map_err(err)?.path().join("HITO-INSTRUMENTAL.json"))?;
  if h["entrega_numero"]!=n||h["resultado"]["completa"]!=true{continue;}
  need(h["resultado"]["telemetria_conforme"]==true,"Historia no medida")?;
  for v in h["originales"].as_array().ok_or("Hito incompleto")?{need(identity(&path(v["ruta"].as_str().ok_or("Ruta")?))?==*v,"Historia alterada")?;}
  let p=path(h["directorio"].as_str().ok_or("Directorio")?);need(found.replace(p).is_none(),"Entrega completa repetida")?;
 }Ok(found)
}
fn request(n:usize)->R<Value>{
 need((1..=3).contains(&n),"Entrega fuera de banco")?;
 let first=(n-1)/3*3+1;let mut history=vec![];
 for q in first..n{let p=completed(q)?.ok_or("Historia incompleta")?;
  let mut stream=Stream::default();stream.feed(&bytes(&p.join("SALIDA-SSE.txt"))?)?;stream.finish()?;
  let got=crate::catalogo::recepcion::extract(&stream.events)?;let text=String::from_utf8(bytes(&p.join("FINAL.txt"))?).map_err(err)?;
  need(got==load(&p.join("ENTREGA-PROVEEDOR.json"))?&&got["texto_original"]==text,"Historia sin cotejo")?;history.push(text);
 }crate::contrato::compose(&load(&baseline((n-1)/3+1))?,(n-1)%3,&history)
}
pub fn preparar()->R<Value>{
 need(!base().join("PREVIA.json").exists(),"Admisión existente")?;
 crate::suministro::preparar(&base())?;
 need(load(&baseline(1))?==load(&path("ejecucion/astra-manual-20261008/fuentes-admitidas/MD01.json"))?,"Fuente, pregunta o parámetros distintos del ensayo original")?;
 for n in 1..=1 {crate::contrato::compose(&load(&baseline(n))?,0,&[])?;}
 let v=json!({"conforme":true,"utc_ms":sv_instrumentacion::utc_ms(),"archivos_fijados":frozen()?,"preguntas":1,"entregas_previstas":3,"maximo_intentos":6,"limite_entrega_ms":300000,"limite_sin_entrega_ms":300000,"limite_revision_ms":1200000,"inferencia_ejecutada":false,"clave_en_candidato":false});
 save(&base().join("PREVIA.json"),&v)?;Ok(v)
}
fn comprobar()->R<()>{let v=load(&base().join("PREVIA.json"))?;need(v["conforme"]==true&&v["archivos_fijados"]==json!(frozen()?),"Admisión alterada")?;crate::suministro::verificar(&base())}
pub fn prepare()->R<()>{need(!base().join("envio-unico.json").exists(),"Continuación ya iniciada")?;comprobar()}
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
                self.terminal=matches!(kind,"response.completed"|"response.failed"|"response.incomplete");kinds.push(kind.to_string());self.events.push(v);
            } else {need(s.is_empty()||s.starts_with(':')||s.starts_with("event:"),"Campo SSE no admitido")?;}
        }Ok(kinds)
    }
    fn finish(&self) -> R<()> {need(self.terminal&&self.pending.iter().all(u8::is_ascii_whitespace),"Entrega incompleta")}
}
fn formal(v:&Value,req:&Value)->R<Value>{crate::contrato::formal(v,req)}
pub fn infer(token:&str)->R<Value>{
 prepare()?;save(&base().join("envio-unico.json"),&json!({"utc_ms":sv_instrumentacion::utc_ms(),"maximo_intentos":6,"limite_revision_ms":1200000,"previa_sha256":sha(&bytes(&base().join("PREVIA.json"))?)}))?;
 let started=Instant::now();let mut queue=crate::servicio::Agenda::new();let mut rows=vec![];let mut periods=vec![];let mut unavailable:Option<(Instant,u128)>=None;let mut status="entregas_recibidas";
 let mut last_failed=std::collections::BTreeMap::<usize,Instant>::new();
 'bank:while let Some(q)=queue.next(){
  for stage in 0..3{let n=(q-1)*3+stage+1;if completed(n)?.is_some(){continue;}
   if started.elapsed().as_millis()+330000>1200000||rows.len()>=6{status="suspendido_por_limite_de_ejecucion";break 'bank;}
   if let Some(last)=last_failed.get(&q){let delay=Duration::from_secs(15).saturating_sub(last.elapsed());if !delay.is_zero(){thread::sleep(delay);}}
   let remaining=unavailable.as_ref().map(|(t,_)|crate::servicio::remaining_ms(t.elapsed().as_millis())).unwrap_or(300000);
   if remaining==0{status="prueba_no_valida_por_falta_de_recursos_que_garanticen_el_examen";break 'bank;}
   ATTEMPT.store(rows.len()+1,std::sync::atomic::Ordering::SeqCst);save(&case(n).join("SOLICITUD.json"),&request(n)?)?;
   let t=Instant::now();let mut r=one(n,token,remaining)?;
   let valid=r["completa"]==true&&r["telemetria_conforme"]==true;
   let failure=r["incidencia_servicio"]["reanudable"]==true;
   let cutoff=unavailable.as_ref().is_some_and(|(t,_)|t.elapsed().as_millis()>=300000)||(!valid&&r["duracion_operacion_ms"].as_u64().is_some_and(|x|x>=300000));
   r["duracion_intento_instrumentado_ms"]=json!(t.elapsed().as_millis());r["entrega_numero"]=json!(n);r["numero_intento"]=json!(attempt());
   let files=["SOLICITUD.json","RESULTADO.json","HTTP.json","SALIDA-SSE.txt","FINAL.txt","ENTREGA-PROVEEDOR.json","AUDITORIA-FORMAL.json","INCIDENCIA-SERVICIO.json","instrumentacion/telemetria.jsonl"].iter().filter(|p|case(n).join(p).is_file()).map(|p|identity(&case(n).join(p))).collect::<R<Vec<_>>>()?;
   save(&base().join(format!("hitos/INTENTO-{:03}/HITO-INSTRUMENTAL.json",attempt())),&json!({"esquema":"sv-hito-instrumental/2","nodo":"03","modelo":"gpt-6-astra","caso":format!("MD{q:02}"),"etapa":stage,"entrega_numero":n,"numero_intento":attempt(),"directorio":case(n).strip_prefix(crate::catalogo::ROOT).map_err(err)?.to_string_lossy(),"fecha_registro_ms":sv_instrumentacion::utc_ms(),"resultado":r,"originales":files,"adjudicacion":"pendiente de contraste exterior al candidato"}))?;
   rows.push(r);println!("ENTREGA=MD{q:02}/R{stage} COMPLETA={valid} INTENTO={}",attempt());std::io::stdout().flush().map_err(err)?;
   if cutoff{status="prueba_no_valida_por_falta_de_recursos_que_garanticen_el_examen";break 'bank;}
   if valid{if let Some((t,utc))=unavailable.take(){periods.push(json!({"proveedor":"OpenAI","inicio_observacion_ms":utc,"fin_recepcion_ms":sv_instrumentacion::utc_ms(),"intervalo_sin_entrega_ms":t.elapsed().as_millis(),"limite":"Ventana de observación operativa; no acredita indisponibilidad continua entre solicitudes"}));}continue;}
   if failure&&rows.last().unwrap()["telemetria_conforme"]==true{
    unavailable.get_or_insert((Instant::now(),sv_instrumentacion::utc_ms()));last_failed.insert(q,Instant::now());queue.defer(q);break;
   }else{status="suspendido_por_incidencia_no_atribuida_a_recursos_del_proveedor";break 'bank;}
  }
 }
 if let Some((t,utc))=unavailable{periods.push(json!({"proveedor":"OpenAI","inicio_observacion_ms":utc,"fin_observacion_ms":sv_instrumentacion::utc_ms(),"intervalo_sin_entrega_ms":t.elapsed().as_millis(),"recuperacion_comprobada":false}));}
 let v=json!({"estado":status,"casos":rows,"duracion_banco_ms":started.elapsed().as_millis(),"inferencias_iniciadas":rows.len(),"entregas_heredadas":0,"periodos_servicio":periods,"intervencion_local_previa":"No se suma como indisponibilidad demostrada del proveedor","adjudicacion_cientifica":false});save(&base().join("RESULTADO-BANCO.json"),&v)?;Ok(v)
}
fn one(n:usize,token:&str,timeout_ms:u64)->R<Value> {
    comprobar()?;let dest=case(n);let id=format!("MD{:02}-R{}",(n-1)/3+1,(n-1)%3);
    let m=sv_instrumentacion::Monitor::start_bounded(&dest.join("instrumentacion"),330)?;
    m.event("fase",json!({"fase":"antes","caso":id}))?;thread::sleep(Duration::from_secs(2));
    let operation_start=Instant::now();let operation_utc=sv_instrumentacion::utc_ms();let mut provider_issue=Value::Null; let operation=(||->R<Value>{m.healthy()?;comprobar()?;
        let req=bytes(&dest.join("SOLICITUD.json"))?;need(parse(&req)?==request(n)?,"Solicitud distinta de la composición admitida")?;
        let http=reqwest::blocking::Client::builder().https_only(true).no_proxy().redirect(reqwest::redirect::Policy::none()).retry(reqwest::retry::never()).connect_timeout(Duration::from_secs(10)).timeout(Duration::from_millis(timeout_ms.max(1))).user_agent("SV-Manual-Astra/1.0.0").build().map_err(err)?;
        save(&dest.join("envio-unico.json"),&json!({"utc_unix_ms":sv_instrumentacion::utc_ms(),"caso":id,"solicitud_sha256":sha(&req),"previa_sha256":sha(&bytes(&base().join("PREVIA.json"))?),"catalogo_autorizado_sha256":sha(&bytes(&base().join("catalogo.json"))?),"reintentos":0,"herramientas":0,"limite_ms":timeout_ms,"credito_adicional_habilitado":false}))?;
        let t=Instant::now();m.event("fase",json!({"fase":"durante","solicitud_bytes":req.len(),"solicitud_sha256":sha(&req),"destino":"https://api.openai.com/v1/responses"}))?;
        let mut res=http.post("https://api.openai.com/v1/responses").bearer_auth(token).header("Content-Type","application/json").body(req.clone()).send().map_err(|e|{if e.is_timeout(){provider_issue=json!({"proveedor":"OpenAI","causa_atribuida":"no determinada: tiempo de espera de transporte","codigo":"timeout_transporte","reanudable":true,"argumento_proveedor":null});} "Transporte interrumpido"})?;
        let status=res.status().as_u16();let mut headers=serde_json::Map::new();
        for n in ["content-type","x-request-id","openai-processing-ms","x-ratelimit-limit-requests","x-ratelimit-remaining-requests","x-ratelimit-remaining-tokens","x-ratelimit-reset-tokens","x-ratelimit-reset-requests"] {if let Some(v)=res.headers().get(n).and_then(|v|v.to_str().ok()){headers.insert(n.into(),json!(v));}}
        let h=json!({"status":status,"cabeceras_permitidas":headers,"tiempo_cabeceras_ms":t.elapsed().as_millis(),"version":format!("{:?}",res.version()),"remoto":res.remote_addr().map(|a|a.to_string())});save(&dest.join("HTTP.json"),&h)?;
        let mut raw=OpenOptions::new().write(true).create_new(true).open(dest.join("SALIDA-SSE.txt")).map_err(err)?;
        if status!=200{let mut b=vec![];res.by_ref().take(262145).read_to_end(&mut b).map_err(err)?;raw.write_all(&b).and_then(|_|raw.sync_all()).map_err(err)?;let body=parse(&b).unwrap_or(Value::Null);provider_issue=json!({"proveedor":"OpenAI","http":status,"codigo":body["error"]["code"],"argumento_proveedor":body["error"]["message"],"reanudable":matches!(status,429|500|502|503|504),"cuerpo_sha256":sha(&b)});return Err("HTTP de error, cuerpo conservado".into());} let mut s=Stream::default();let mut buffer=[0;8192];let mut count=0usize;let mut first=None;let mut first_text=None;
        loop {m.healthy()?;let n=res.read(&mut buffer).map_err(|_|{if operation_start.elapsed().as_millis()+1000>=timeout_ms as u128{provider_issue=json!({"proveedor":"OpenAI","codigo":"timeout_lectura","causa_atribuida":"no determinada: no entrega completa en plazo","argumento_proveedor":null,"reanudable":true});} "Lectura interrumpida; estado remoto no acreditado"})?;if n==0 {break;}
            raw.write_all(&buffer[..n]).and_then(|_|raw.sync_data()).map_err(err)?;count+=n;need(count<=4*1024*1024,"Salida excesiva")?;
            m.event("lectura_https",json!({"ms":t.elapsed().as_millis(),"bytes":n,"acumulados":count}))?;
            for k in s.feed(&buffer[..n])? {first.get_or_insert(t.elapsed().as_millis());if k=="response.output_text.delta" {first_text.get_or_insert(t.elapsed().as_millis());}m.event("evento_sse",json!({"ms":t.elapsed().as_millis(),"tipo":k,"eventos_recibidos":s.events.len()}))?;}
        }
        raw.sync_all().map_err(err)?; if let Some(f)=s.events.iter().find(|e|e["type"]=="response.failed"||e["type"]=="error"){let failure=s.events.iter().find(|e|e["type"]=="response.failed").unwrap_or(f);let e=if failure["type"]=="response.failed"{&failure["response"]["error"]}else{failure};provider_issue=json!({"proveedor":"OpenAI","http":status,"codigo":e["code"],"argumento_proveedor":e["message"],"evento_terminal":s.events.last().map(|e|e["type"].clone()),"reanudable":matches!(e["code"].as_str(),Some("server_is_overloaded"|"server_error"|"rate_limit_exceeded")),"uso_proveedor":failure["response"]["usage"],"utc_observacion_ms":sv_instrumentacion::utc_ms()});return Err("Fallo explícito del proveedor".into());} need(status==200,"HTTP no satisfactorio")?;s.finish()?;
        let received=crate::catalogo::recepcion::extract(&s.events)?;let text=received["texto_original"].as_str().ok_or("Texto final")?;
        put(&dest.join("FINAL.txt"),text.as_bytes())?;save(&dest.join("ENTREGA-PROVEEDOR.json"),&received)?;
        let audit=match parse(text.as_bytes()).and_then(|v|formal(&v,&parse(&req)?)){Ok(v)=>v,Err(e)=>json!({"conforme":false,"defecto":e})};save(&dest.join("AUDITORIA-FORMAL.json"),&audit)?;
        Ok(json!({"estado":"entrega_recibida","caso":id,"completa":true,"utc_fin_ms":sv_instrumentacion::utc_ms(),"duracion_ms":t.elapsed().as_millis(),"primer_evento_ms":first,"primer_texto_ms":first_text,"eventos":s.events.len(),"respuesta_bytes":count,"uso_proveedor":received["uso_proveedor"],"modelo_solicitado":"gpt-6-astra","modelo_declarado":s.events.last().unwrap()["response"]["model"],"evento_terminal":s.events.last().unwrap()["type"],"http":status,"cabeceras_proveedor":headers,"par_remoto_http":h["remoto"],"version_http":h["version"],"respuesta_texto":text,"resumen_proveedor":received["resumen_proveedor"],"contrato_formal_conforme":audit["conforme"],"herramientas_habilitadas":0,"herramientas_ejecutadas":0,"adjudicado":false,"creditos_atribuibles":null,"coste_monetario_atribuible":null}))
    })();
    let operation_duration=operation_start.elapsed().as_millis();if !provider_issue.is_null(){save(&dest.join("INCIDENCIA-SERVICIO.json"),&provider_issue)?;}let end=m.event("fase",json!({"fase":"despues","entrega_obtenida":operation.is_ok()}));thread::sleep(Duration::from_secs(1));let telemetry=m.finish();
    let ok=end.is_ok()&&telemetry.as_ref().is_ok_and(|v|v["fallos_medicion"]==0&&v["intervalo_maximo_ms"].as_u64().is_some_and(|n|n<=750));
    let mut v=operation.unwrap_or_else(|cause|json!({"estado":"impedimento","caso":id,"completa":false,"causa":cause,"envio_iniciado_o_incierto":dest.join("envio-unico.json").exists(),"uso_proveedor":null,"creditos_atribuibles":null,"coste_monetario_atribuible":null,"reintento_automatico":false}));
    v["utc_inicio_operacion_ms"]=json!(operation_utc);v["duracion_operacion_ms"]=json!(operation_duration);v["incidencia_servicio"]=provider_issue;v["telemetria"]=telemetry.unwrap_or_else(|e|json!({"error":e}));v["telemetria_conforme"]=json!(ok);
    v["limites"]=json!(["CPU porcentual sin calibración externa","Sin medida separada DNS/TLS, RTT, retransmisiones, asignaciones del montículo, hilos ni recursos internos del proveedor","La ausencia de herramientas no prueba aislamiento de infraestructura de OpenAI; fuente exclusiva exigida y respuesta pendiente de contraste sustantivo","Criptografía C/ensamblador pendiente conforme a excepción experimental autorizada"]);
    save(&dest.join("RESULTADO.json"),&v)?;Ok(v)
}
pub fn auditar()->R<Value>{comprobar()?;Ok(json!({"fijacion_conforme":true,"adjudicacion":false}))}
pub fn page(v:&Value)->String {format!("<!doctype html><html lang='es'><meta charset='utf-8'><h1>SV · Astra · Réplica diagnóstica de MD01</h1><p>Entrega instrumental del banco. Adjudicación científica separada.</p><pre>{}</pre></html>",sv_instrumentacion::escape(&serde_json::to_string_pretty(v).unwrap()))}
pub fn start_page()->String {"<!doctype html><html lang='es'><meta charset='utf-8'><h1>SV · Astra · Respuesta, autocrítica y verificación neutral</h1><p>Una única pregunta MD01 sobre la naturaleza del constructor del manual. Sin preguntas adicionales. Secciones completas recibidas mediante MCP e historial exclusivo de cada pregunta. Control y observación en Rust. Sin herramientas del candidato. Fallos del proveedor aplazados al final; suspensión al alcanzar cinco minutos sin entrega durante una interrupción observada. Cada intento queda conservado. Adjudicación posterior, externa al candidato.</p><a href='/start'>Autorizar con ChatGPT e iniciar las tres etapas autorizadas</a></html>".into()}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn mantenimiento_observado_no_equivale_a_respuesta(){let mut s=Stream::default();s.feed(&std::fs::read(path("ejecucion/astra-examen25-20261007/originales/P01/R0/SALIDA-SSE.txt")).unwrap()).unwrap();assert_eq!(s.events.len(),3);assert_eq!(s.events[2]["type"],"keepalive");assert!(s.finish().is_err());assert!(crate::catalogo::recepcion::extract(&s.events).is_err());}
    #[test] fn mantenimiento_rechaza_contenido_saltos_y_poscierre(){for b in [b"data: {\"type\":\"keepalive\",\"sequence_number\":0,\"item\":{\"type\":\"function_call\"}}\n".as_slice(),b"data: {\"type\":\"keepalive\",\"sequence_number\":1}\n"]{assert!(Stream::default().feed(b).is_err());}let mut s=Stream::default();s.feed(b"data: {\"type\":\"response.completed\",\"sequence_number\":0}\n").unwrap();assert!(s.feed(b"data: {\"type\":\"keepalive\",\"sequence_number\":1}\n").is_err());}
    #[test] fn mantenimiento_conserva_texto_y_uso(){let original=std::fs::read(path("ejecucion/astra-pdf-doble-20261007/originales/PDF01/R0/SALIDA-SSE.txt")).unwrap();let mut a=Stream::default();a.feed(&original).unwrap();a.finish().unwrap();let expected=crate::catalogo::recepcion::extract(&a.events).unwrap();let mut events=a.events.clone();events.insert(2,json!({"type":"keepalive","sequence_number":2}));let mut b=Stream::default();for(i,v)in events.iter_mut().enumerate(){v["sequence_number"]=json!(i);b.feed(format!("data: {}\n\n",v).as_bytes()).unwrap();}b.finish().unwrap();assert_eq!(crate::catalogo::recepcion::extract(&b.events).unwrap(),expected);}
    #[test] fn sobrecarga_conserva_error_y_cierre(){let mut s=Stream::default();s.feed(&std::fs::read(path("ejecucion/astra-examen25-r2-20261007/originales/P15/R0/SALIDA-SSE.txt")).unwrap()).unwrap();s.finish().unwrap();assert_eq!(s.events.len(),4);assert_eq!(s.events[2]["type"],"error");assert_eq!(s.events[3]["type"],"response.failed");assert_eq!(s.events[3]["response"]["error"]["code"],"server_is_overloaded");assert!(crate::catalogo::recepcion::extract(&s.events).is_err());}
    #[test] fn error_sin_cierre_no_es_respuesta(){let mut s=Stream::default();s.feed(b"data: {\"type\":\"error\",\"sequence_number\":0,\"code\":\"server_error\"}\n").unwrap();assert!(s.finish().is_err());}
    #[test] fn sse_fragmentado_unicode(){let mut s=Stream::default();let b="data: {\"type\":\"response.created\",\"sequence_number\":0,\"x\":\"ñ\"}\n\ndata: {\"type\":\"response.completed\",\"sequence_number\":1}\n\n".as_bytes();for q in b {s.feed(&[*q]).unwrap();}s.finish().unwrap();assert_eq!(s.events.len(),2);}
    #[test] fn sse_no_truncado_ni_doble_cierre(){let mut s=Stream::default();s.feed(b"data: {\"type\":\"response.completed\",\"sequence_number\":0}\n").unwrap();s.finish().unwrap();assert!(s.feed(b"data: {\"type\":\"response.completed\",\"sequence_number\":1}\n").is_err());let mut s=Stream::default();s.feed(b"data: {").unwrap();assert!(s.finish().is_err());}
    #[test] fn sse_rechaza_herramientas_y_saltos(){for b in [b"data: {\"type\":\"response.output_item.added\",\"sequence_number\":0,\"item\":{\"type\":\"function_call\"}}\n".as_slice(),b"data: {\"type\":\"response.created\",\"sequence_number\":1}\n",b"data: [DONE]\n"] {assert!(Stream::default().feed(b).is_err());}}
}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
