//! Acoplamiento del conductor documental recibido al transporte Responses.
//! El candidato no decide admisiones, adjudicaciones ni telemetría.
pub mod contrato;
pub mod estricto;
pub mod recepcion;
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs::{self,OpenOptions},io::{Read,Write},path::{Path,PathBuf},process::{Command,Stdio},thread,time::{Instant,Duration}};
type R<T> = Result<T,String>;
pub const ROOT:&str="C:/laboratorio/watson-local/lenguaje-computacion-sv";
pub const PREP:&str="ejecucion/astra-catalogo-20261006";
pub const RUN:&str="ejecucion/astra-catalogo-20261007";
const MCP:&str="compilacion/mcp-linux/debug/sv-mcp-documental";
const AUDIT:&str="compilacion/mcp-linux/debug/verificar-diario";
const POLICY:&str="5063d058df015017355e0fd836389c0791f128911d6da07e7e4a631f8f42716a";
fn e(x:impl std::fmt::Display)->String{x.to_string()}
pub fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
pub fn path(s:&str)->PathBuf{Path::new(ROOT).join(s)}
pub fn read(p:&Path)->R<Value>{estricto::parse(&fs::read(p).map_err(e)?).map_err(e)}
pub fn put(p:&Path,b:&[u8])->R<()>{
    guard(p)?;fs::create_dir_all(p.parent().ok_or("Sin padre")?).map_err(e)?;
    let mut f=OpenOptions::new().write(true).create_new(true).open(p).map_err(e)?;
    f.write_all(b).and_then(|_|f.sync_all()).map_err(e)
}
pub fn save(p:&Path,v:&Value)->R<()>{put(p,&serde_json::to_vec_pretty(v).map_err(e)?)}
fn guard(p:&Path)->R<()>{
    let rel=p.strip_prefix(ROOT).map_err(|_|"Fuera de perímetro")?;
    let mut x=PathBuf::from(ROOT);
    for c in rel.components(){if !matches!(c,std::path::Component::Normal(_)){return Err("Ruta no normal".into());}x.push(c);
        if x.exists(){let m=fs::symlink_metadata(&x).map_err(e)?;
            #[cfg(windows)]{use std::os::windows::fs::MetadataExt;if m.file_attributes()&0x400!=0{return Err("Reanálisis no admitido".into());}}
            if m.file_type().is_symlink(){return Err("Enlace no admitido".into());}
        }
    }Ok(())
}
fn check(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn fixed(p:&Path,h:&str)->R<Vec<u8>>{guard(p)?;let b=fs::read(p).map_err(e)?;check(sha(&b)==h,"Fuente fijada alterada")?;Ok(b)}
fn linux(p:&Path)->R<String>{guard(p)?;Ok(p.to_string_lossy().replace("C:","/mnt/c").replace('\\',"/"))}
fn command(bin:&str,args:&[String])->Command{
    let mut c=Command::new("wsl.exe");c.args(["-d","Ubuntu","--exec"]);
    c.arg(linux(&path(bin)).expect("Ruta binario confinada")).args(args);
    #[cfg(windows)]{use std::os::windows::process::CommandExt;c.creation_flags(0x08000000);}
    c
}
fn local(mut cmd:Command,input:Vec<u8>,out:&Path,label:&str)->R<Vec<u8>>{
    let t=Instant::now();let mut child=cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().map_err(e)?;
    let pid=child.id();let mut sampler=sv_instrumentacion::Sampler::for_pid(pid)?;
    let mut resources=vec![json!({"ms":0,"muestra":sampler.sample()?})];
    let mut w=child.stdin.take().ok_or("stdin")?;w.write_all(&input).map_err(e)?;drop(w);
    let stdout=child.stdout.take().ok_or("stdout")?;let stderr=child.stderr.take().ok_or("stderr")?;
    let a=thread::spawn(move||{let mut b=vec![];stdout.take(16*1024*1024+1).read_to_end(&mut b).map(|_|b)});
    let b=thread::spawn(move||{let mut b=vec![];stderr.take(65537).read_to_end(&mut b).map(|_|b)});
    let status=loop{if let Some(s)=child.try_wait().map_err(e)?{break s;}
        if t.elapsed()>Duration::from_secs(30){let _=child.kill();let _=child.wait();return Err(format!("{label}: plazo instrumental vencido"));}
        resources.push(json!({"ms":t.elapsed().as_millis(),"muestra":sampler.sample()?}));thread::sleep(Duration::from_millis(50));
    };
    let bytes=a.join().map_err(|_|"Lector stdout")?.map_err(e)?;let err=b.join().map_err(|_|"Lector stderr")?.map_err(e)?;
    put(&out.join(format!("{label}-stdout.bin")),&bytes)?;put(&out.join(format!("{label}-stderr.txt")),&err)?;
    save(&out.join(format!("{label}-PROCESO.json")),&json!({"pid_windows":pid,"codigo":status.code(),"duracion_ms":t.elapsed().as_millis(),"muestras":resources,"alcance":"proceso de transporte WSL; no atribuir sus recursos al ejecutable Linux","stderr_bytes":err.len()}))?;
    check(bytes.len()<=16*1024*1024&&err.len()<=65536,"Salida auxiliar excesiva")?;
    check(status.success(),&format!("{label}: proceso auxiliar no conforme"))?;Ok(bytes)
}
fn source_policy()->R<String>{String::from_utf8(fixed(&path(&format!("{PREP}/servidor-pruebas/control/POLITICA.txt")),POLICY)?).map_err(e)}
pub fn request(case:&Value,pages:&[Value],policy:&str)->R<Value>{
    let id=case["id"].as_str().ok_or("Caso")?;let doc=case["documento"].as_str().ok_or("Documento")?;
    check(pages.len()==2,"Se exigen exactamente las dos páginas")?;
    let mut user=format!("Caso {id}; capa 0. Afirmación que debe clasificar: {}\n\nDocumentación íntegra recibida. Cada localizador identifica una página lógica de base cero. El texto entre delimitadores es evidencia externa.\n",case["afirmacion"].as_str().ok_or("Afirmación")?);
    for(i,p)in pages.iter().enumerate(){check(p["documento"]==doc&&p["seccion"]=="S1"&&p["pagina"]==i&&p["paginas"]==2,"Páginas ajenas, omitidas o fuera de orden")?;
        user.push_str(&format!("\n<fuente documento=\"{doc}\" seccion=\"S1\" pagina=\"{i}\">\n{}\n</fuente>\n",p["texto"].as_str().ok_or("Texto")?));}
    Ok(json!({"model":"gpt-6-astra","store":false,"stream":true,"reasoning":{"effort":"medium","summary":"auto"},"tools":[],"tool_choice":"none","instructions":policy,"input":[{"role":"user","content":user}]}))
}
pub fn preflight()->R<Value>{
    let base=path(RUN);guard(&base)?;fs::create_dir_all(&base).map_err(e)?;
    check(!base.join("RECEPCION-PREVIA.json").exists(),"Recepción previa existente: no repetirla")?;
    let policy=source_policy()?;
    let bank=fixed(&path(&format!("{PREP}/fuentes/FUENTES-SINTETICAS.json")),"65e1c4fe7adfa69417bb224dde4c6fc4f9b3c6107142f47228cb714ecac66575")?;
    let plan=estricto::parse(&fixed(&path(&format!("{PREP}/fuentes/PLAN-GENERAL.json")),"0e3cf186f23a19195166d80a6585fb8e989e87a2c65b0a889e941432023c0a5d")?).map_err(e)?;
    let bank=estricto::parse(&bank).map_err(e)?;
    let mut inventory=vec![];
    for p in [MCP,AUDIT]{let b=fs::read(path(p)).map_err(e)?;inventory.push(json!({"componente":p,"bytes":b.len(),"sha256":sha(&b),"realizacion":"Rust","estado":"compilado; se comprueba ejercicio en los recibos de cada caso"}));}
    let monitor=sv_instrumentacion::Monitor::start_bounded(&base.join("preparacion-instrumentacion"),360)?;
    let mut prepared=vec![];
    for n in 1..=9 {
        monitor.healthy()?;let id=format!("A{n:02}");let out=base.join("originales/A").join(&id).join("A0");guard(&out)?;
        check(!out.exists(),"Preparación parcial existente: conservar y revisar")?;fs::create_dir_all(&out).map_err(e)?;
        let case=&plan["casos"][n-1];check(case["id"]==id,"Orden del plan")?;
        let d=bank["documents"].as_array().ok_or("Banco")?.iter().find(|d|d["id"]==case["documento"]).ok_or("Documento ajeno")?;
        check(d["synthetic"]==true&&d["sections"].as_array().is_some_and(|a|a.len()==1),"Admisión exclusivamente artificial")?;
        let mut d=d.clone();d["url"]=json!("urn:sv:prueba-sintetica");
        let cat=out.join("CATALOGO.json");save(&cat,&json!({"version":1,"documents":[d]}))?;let h=sha(&fs::read(&cat).map_err(e)?);
        let probe=local(command(MCP,&["--probe-isolation".into(),linux(&cat)?]),vec![],&out,"AISLAMIENTO")?;
        let probe=estricto::parse(&probe).map_err(e)?;check(probe["red_externa_error"]==1&&probe["red_local_error"]==1,"Aislamiento seccomp no acreditado")?;
        let frames=vec![json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"sv-astra-conductor","version":"0.1.0"}}}),json!({"jsonrpc":"2.0","method":"notifications/initialized","params":{}}),json!({"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}),json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"leer_documento","arguments":{"documento":case["documento"],"seccion":"S1","pagina":0}}}),json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"leer_documento","arguments":{"documento":case["documento"],"seccion":"S1","pagina":1}}})];
        let input=frames.iter().map(|v|format!("{v}\n")).collect::<String>().into_bytes();put(&out.join("MCP-SOLICITUDES.jsonl"),&input)?;
        let log=out.join("MCP-DIARIO.jsonl");let raw=local(command(MCP,&[linux(&cat)?,h.clone(),linux(&log)?,"64".into(),"--sintetico".into()]),input,&out,"MCP")?;
        let rows=raw.split(|b|*b==b'\n').filter(|b|!b.is_empty()).map(estricto::parse).collect::<Result<Vec<_>,_>>().map_err(e)?;
        check(rows.len()==4,"Número de respuestas MCP")?;for(i,r)in rows.iter().enumerate(){check(r["id"]==i&&r.get("error").is_none()&&r["result"]["isError"]!=true,"Identidad o error MCP")?;}
        let pages=rows[2..].iter().map(|r|estricto::parse(r["result"]["content"][0]["text"].as_str().ok_or("Texto MCP")?.as_bytes()).map_err(e)).collect::<R<Vec<_>>>()?;
        let expected=read(&path(&format!("{PREP}/servidor-pruebas/cache/A/{id}/PAGINAS-PREVISTAS.json")))?;
        for (i,p)in pages.iter().enumerate(){for k in ["documento","seccion","pagina","paginas","texto"]{check(p[k]==expected[i][k],"Página recibida distinta de preparación")?;}}
        let joined=pages.iter().map(|p|p["texto"].as_str().unwrap()).collect::<String>();check(joined==d["sections"][0]["text"]&&sha(joined.as_bytes())==d["sections"][0]["sha256"],"Totalidad documental no conforme")?;
        check(pages[0]["siguiente_pagina"]==1&&pages[1]["siguiente_pagina"].is_null(),"Continuación MCP incorrecta")?;
        save(&out.join("PAGINAS.json"),&json!(pages))?;
        let audited=local(command(AUDIT,&[linux(&cat)?,h,linux(&log)?,"--sintetico".into()]),vec![],&out,"MCP-COTEJO")?;
        let audit=estricto::parse(&audited).map_err(e)?;check(audit["estado"]=="conforme","Diario MCP no conforme")?;
        let req=request(case,&pages,&policy)?;let bytes=serde_json::to_vec_pretty(&req).map_err(e)?;put(&out.join("SOLICITUD.json"),&bytes)?;
        let receipt=json!({"caso":id,"capa":0,"conforme":true,"paginas":2,"solicitud_sha256":sha(&bytes),"bytes":bytes.len(),"politica_sha256":POLICY,"mcp_diario_sha256":sha(&fs::read(log).map_err(e)?),"mcp_aislamiento":probe,"inferencia_iniciada":false});
        save(&out.join("PREPARACION.json"),&receipt)?;prepared.push(receipt);
    }
    let telemetry=monitor.finish()?;check(telemetry["fallos_medicion"]==0,"Telemetría previa degradada")?;
    let v=json!({"fecha_unix_ms":sv_instrumentacion::utc_ms(),"suceso":"S39","tique":"TT-0021","alcance":"A01-A09, capa inicial A0; sin B, anexo ni examen","conforme":true,"casos":prepared,"componentes":inventory,"telemetria_preparacion":telemetry,"ejecutables_fuente":"conductor Qwen r1 adaptado al transporte Responses; MCP 0.1.3 sin modificar; verificador MCP sin modificar; contrato/lector estricto del nodo 1 conservados","politica_exclusividad":"dos páginas completas, sin hechos de entrenamiento ni navegación; deducciones necesarias admitidas","limite_por_solicitud_s":300,"herramientas_modelo":0,"credito_adicional_habilitado_por_programa":false,"inferencia_ejecutada":false,"adjudicacion_candidato":false});
    save(&base.join("RECEPCION-PREVIA.json"),&v)?;Ok(v)
}
pub fn prepare()->R<()>{
    let base=path(RUN);let v=read(&base.join("RECEPCION-PREVIA.json"))?;check(v["conforme"]==true,"Recepción previa no conforme")?;
    check(!base.join("envio-unico.json").exists(),"Bloque ya iniciado: no repetir automáticamente")?;
    source_policy()?;for n in 1..=9{let out=base.join(format!("originales/A/A{n:02}/A0"));let p=read(&out.join("PREPARACION.json"))?;check(p["solicitud_sha256"]==sha(&fs::read(out.join("SOLICITUD.json")).map_err(e)?),"Solicitud modificada")?;}
    Ok(())
}
pub fn page(v:&Value)->String{format!("<!doctype html><html lang='es'><meta charset='utf-8'><title>SV · Astra · Catálogo A0</title><h1>Catálogo A01–A09 · Astra</h1><p>Resultados instrumentales. La adjudicación científica y el polígono se conservan por separado.</p><pre>{}</pre></html>",sv_instrumentacion::escape(&serde_json::to_string_pretty(v).unwrap_or_default()))}
pub fn start_page()->String{"<!doctype html><html lang='es'><meta charset='utf-8'><h1>SV · Astra · Catálogo A0</h1><p>Nueve solicitudes artificiales A01–A09, una por caso. Dos páginas completas, sin herramientas ni navegación. Control y telemetría Rust. Límite 300 s por solicitud; sin reintentos ni activación de créditos adicionales.</p><p><a href='/start'>Continuar con ChatGPT e iniciar el catálogo autorizado</a></p></html>".into()}

fn allowed_event(v:&Value)->R<()>{
    check(matches!(v["type"].as_str(),Some("response.created"|"response.in_progress"|"response.output_item.added"|"response.content_part.added"|"response.output_text.delta"|"response.output_text.done"|"response.content_part.done"|"response.output_item.done"|"response.completed"|"response.failed"|"response.incomplete"|"error"|"response.reasoning_summary_part.added"|"response.reasoning_summary_part.done"|"response.reasoning_summary_text.delta"|"response.reasoning_summary_text.done")),"Evento ajeno al contrato sin herramientas")?;
    if v.get("item").is_some(){check(matches!(v["item"]["type"].as_str(),Some("message"|"reasoning")),"Llamada a herramienta rechazada; no ejecutada")?;}Ok(())
}
fn one(token:&str,out:&Path,case:&str)->R<Value>{
    let monitor=sv_instrumentacion::Monitor::start_bounded(&out.join("instrumentacion"),330)?;
    monitor.event("fase",json!({"fase":"antes","caso":case,"capa":0}))?;
    thread::sleep(Duration::from_secs(2));monitor.healthy()?;
    let operation=(||->R<Value>{
        let req=fs::read(out.join("SOLICITUD.json")).map_err(e)?;
        let p=read(&out.join("PREPARACION.json"))?;check(p["conforme"]==true&&p["solicitud_sha256"]==sha(&req),"Preparación no coincide")?;
        let body=estricto::parse(&req).map_err(e)?;
        check(body["tools"]==json!([])&&body["tool_choice"]=="none"&&body["store"]==false&&body["stream"]==true&&body["model"]=="gpt-6-astra"&&body["instructions"]==source_policy()?,"Controles del transporte alterados")?;
        let http=reqwest::blocking::Client::builder().https_only(true).no_proxy().redirect(reqwest::redirect::Policy::none()).retry(reqwest::retry::never()).connect_timeout(Duration::from_secs(10)).timeout(Duration::from_secs(300)).user_agent("SV-Catalogo-Astra/0.1.0").build().map_err(e)?;
        save(&out.join("ADMISION-UNICA.json"),&json!({"utc_unix_ms":sv_instrumentacion::utc_ms(),"caso":case,"capa":0,"autorizacion":"Instrucción humana expresa 07/10/2026: comenzar catálogo tras recepción instrumental; dos páginas exclusivamente","solicitud_sha256":sha(&req),"herramientas":0,"reintentos":0,"limite_s":300}))?;
        let t=Instant::now();monitor.event("fase",json!({"fase":"durante","destino":"https://api.openai.com/v1/responses","solicitud_bytes":req.len(),"solicitud_sha256":sha(&req)}))?;
        let mut response=http.post("https://api.openai.com/v1/responses").bearer_auth(token).header("Content-Type","application/json").body(req).send().map_err(|_|"Transporte no completado; no repetir automáticamente")?;
        let status=response.status().as_u16();let mut headers=serde_json::Map::new();
        for name in ["content-type","x-request-id","openai-processing-ms","x-ratelimit-limit-requests","x-ratelimit-remaining-requests","x-ratelimit-remaining-tokens","x-ratelimit-reset-tokens","x-ratelimit-reset-requests"]{if let Some(v)=response.headers().get(name).and_then(|v|v.to_str().ok()){headers.insert(name.into(),json!(v));}}
        save(&out.join("HTTP.json"),&json!({"status":status,"cabeceras_permitidas":headers,"tiempo_cabeceras_ms":t.elapsed().as_millis(),"version":format!("{:?}",response.version()),"remoto":response.remote_addr().map(|a|a.to_string())}))?;
        let mut raw=OpenOptions::new().write(true).create_new(true).open(out.join("SALIDA-SSE.txt")).map_err(e)?;
        let mut buffer=[0u8;8192];let mut pending=Vec::new();let mut events=vec![];let mut bytes=0usize;let mut first=None;let mut text=None;let mut terminal=false;
        loop{monitor.healthy()?;check(t.elapsed()<Duration::from_secs(300),"Plazo por solicitud agotado")?;
            let n=response.read(&mut buffer).map_err(|_|"Lectura interrumpida; estado remoto no acreditado")?;if n==0{break;}
            raw.write_all(&buffer[..n]).and_then(|_|raw.sync_data()).map_err(e)?;bytes+=n;check(bytes<=4*1024*1024,"Límite de salida documental superado")?;
            monitor.event("lectura_https",json!({"ms":t.elapsed().as_millis(),"bytes":n,"acumulados":bytes}))?;
            pending.extend_from_slice(&buffer[..n]);
            while let Some(end)=pending.iter().position(|b|*b==b'\n'){
                let line=pending.drain(..=end).collect::<Vec<_>>();let s=std::str::from_utf8(&line).map_err(e)?.trim_end_matches(['\n','\r']);
                if let Some(data)=s.strip_prefix("data:") {if data.trim()=="[DONE]"{continue;}
                    let v=estricto::parse(data.trim().as_bytes()).map_err(e)?;
                    monitor.event("evento_sse",json!({"ms":t.elapsed().as_millis(),"evento":v}))?;
                    check(!terminal,"Evento posterior al cierre")?;check(v["sequence_number"].as_u64()==Some(events.len() as u64),"Secuencia SSE discordante")?;
                    allowed_event(&v)?;first.get_or_insert(t.elapsed().as_millis());
                    if v["type"]=="response.output_text.delta"{text.get_or_insert(t.elapsed().as_millis());}
                    terminal=matches!(v["type"].as_str(),Some("response.completed"|"response.failed"|"response.incomplete"|"error"));events.push(v);
                }
            }
        }
        raw.sync_all().map_err(e)?;check(pending.iter().all(u8::is_ascii_whitespace),"Trama final incompleta")?;
        let elapsed=t.elapsed().as_millis();check(status==200,"HTTP de inferencia no satisfactorio")?;
        let received=recepcion::extract(&events)?;let final_text=received["texto_original"].as_str().ok_or("Sin texto final")?;
        put(&out.join("FINAL.txt"),final_text.as_bytes())?;save(&out.join("ENTREGA-PROVEEDOR.json"),&received)?;
        let pages=read(&out.join("PAGINAS.json"))?;let pages=pages.as_array().ok_or("Páginas")?.iter().map(|p|p["texto"].as_str().map(String::from).ok_or("Texto".to_string())).collect::<R<Vec<_>>>()?;
        let parsed=estricto::parse(final_text.as_bytes());
        let formal=parsed.as_ref().map_err(e).and_then(|v|contrato::check(v,&format!("D{case}"),"S1",&pages,case,0).map_err(e));
        let audit=json!({"caso":case,"capa":0,"integridad_conforme":true,"contrato_formal_conforme":formal.is_ok(),"defecto_formal":formal.err(),"decision_observada":parsed.as_ref().ok().map(|v|v["decision"].clone()),"final_sha256":sha(final_text.as_bytes()),"sustantivo":"pendiente de lectura y adjudicación externa al candidato","premisas_externas":"pendiente de contraste sustantivo; las citas literales no bastan para probarlo","fuente_sse_sha256":sha(&fs::read(out.join("SALIDA-SSE.txt")).map_err(e)?),"solicitud_sha256":p["solicitud_sha256"]});
        save(&out.join("AUDITORIA-FORMAL.json"),&audit)?;
        Ok(json!({"caso":case,"capa":0,"completa":true,"estado":"entrega_recibida","duracion_ms":elapsed,"primer_evento_ms":first,"primer_texto_ms":text,"eventos":events.len(),"bytes_sse":bytes,"uso_proveedor":received["uso_proveedor"],"modelo_solicitado":"gpt-6-astra","modelo_declarado":events.last().unwrap()["response"]["model"],"terminacion":events.last().unwrap()["type"],"resumen_recibido":received["resumen_proveedor"].as_array().is_some_and(|a|!a.is_empty()),"herramientas_habilitadas":0,"herramientas_ejecutadas":0,"contrato_formal_conforme":audit["contrato_formal_conforme"],"adjudicado":false,"creditos_atribuibles":null,"coste_monetario_atribuible":null}))
    })();
    monitor.event("fase",json!({"fase":"despues","operacion_concluida":operation.is_ok()}))?;thread::sleep(Duration::from_secs(1));
    let telemetry=monitor.finish();
    let telemetry_ok=telemetry.as_ref().is_ok_and(|v|v["fallos_medicion"]==0&&v["intervalo_maximo_ms"].as_u64().is_some_and(|n|n<=750));
    let mut result=operation.clone().unwrap_or_else(|cause|json!({"caso":case,"capa":0,"completa":false,"estado":"impedimento_instrumental","causa":cause,"fuera_de_terna":true,"reintento_automatico":false,"envio_iniciado_o_incierto":out.join("ADMISION-UNICA.json").exists(),"uso_proveedor":null,"creditos_atribuibles":null,"coste_monetario_atribuible":null}));
    result["telemetria"]=telemetry.clone().unwrap_or_else(|s|json!({"error":s}));result["telemetria_conforme"]=json!(telemetry_ok);
    save(&out.join("RESULTADO.json"),&result)?;
    operation?;check(telemetry_ok,"Telemetría no conforme; siguiente solicitud detenida")?;Ok(result)
}
pub fn infer(token:&str)->R<Value>{
    prepare()?;let base=path(RUN);let lock=OpenOptions::new().read(true).write(true).create(true).truncate(false).open(base.join("CONTROL.lock")).map_err(e)?;lock.try_lock().map_err(e)?;
    save(&base.join("envio-unico.json"),&json!({"utc_unix_ms":sv_instrumentacion::utc_ms(),"bloque":"A","capa":0,"maximo_solicitudes":9,"autorizado":true,"creditos_adicionales_modificados":false}))?;
    let mut results=vec![];let start=Instant::now();
    for n in 1..=9{let id=format!("A{n:02}");let out=base.join(format!("originales/A/{id}/A0"));
        match one(token,&out,&id){Ok(v)=>{println!("CATALOGO_CASO={}",json!({"caso":id,"ms":v["duracion_ms"],"tokens":v["uso_proveedor"]["total_tokens"],"forma":v["contrato_formal_conforme"]}));std::io::stdout().flush().map_err(e)?;results.push(v);},Err(cause)=>{save(&base.join("DETENCION.json"),&json!({"caso":id,"causa":cause,"resultados_previos":results,"sin_reintentos":true}))?;return Err(cause);}}
    }
    Ok(json!({"estado":"A0_entregas_recibidas","casos":results,"duracion_bloque_con_observacion_ms":start.elapsed().as_millis(),"adjudicado":false,"continuacion_automatica":false,"creditos_atribuibles":null,"coste_monetario_atribuible":null}))
}

#[cfg(test)]mod tests{
    use super::*;
    fn pages()->Vec<Value>{(0..2).map(|n|json!({"documento":"DA01","seccion":"S1","pagina":n,"paginas":2,"texto":format!("texto completo {n} ñ")})).collect()}
    fn case()->Value{json!({"id":"A01","documento":"DA01","afirmacion":"afirmación sintética"})}
    #[test]fn ambas_paginas_y_prohibicion(){let p=source_policy().unwrap();let r=request(&case(),&pages(),&p).unwrap();assert_eq!(r["tools"],json!([]));assert_eq!(r["tool_choice"],"none");assert!(p.contains("No añada hechos recordados del entrenamiento"));let s=r["input"][0]["content"].as_str().unwrap();for p in pages(){assert!(s.contains(p["texto"].as_str().unwrap()));}assert!(r.get("previous_response_id").is_none());assert!(r.get("text").is_none());}
    #[test]fn rechaza_omision_y_desorden(){assert!(request(&case(),&pages()[..1],"p").is_err());let mut p=pages();p.swap(0,1);assert!(request(&case(),&p,"p").is_err());}
    #[test]fn no_despacha_herramientas(){for t in ["web_search_call","function_call","mcp_call","code_interpreter_call"]{assert!(allowed_event(&json!({"type":"response.output_item.added","item":{"type":t}})).is_err());}assert!(allowed_event(&json!({"type":"response.web_search_call.in_progress"})).is_err());}
    #[test]fn no_admite_citas_externas_ni_recepcion_parcial(){let mut v=json!({"decision":"RESPALDADA","reglas":["D1"],"evidencias":[{"documento":"DA01","seccion":"S1","pagina":0,"fragmento":"texto"}],"justificacion_breve":"Según el texto","recepcion_documental":[{"documento":"DA01","seccion":"S1","paginas":[0,1]}],"revision":null});let p=vec!["texto".into(),"otro".into()];assert!(contrato::check(&v,"DA01","S1",&p,"A01",0).is_ok());v["evidencias"][0]["documento"]=json!("web");assert!(contrato::check(&v,"DA01","S1",&p,"A01",0).is_err());v["evidencias"][0]["documento"]=json!("DA01");v["recepcion_documental"][0]["paginas"]=json!([1]);assert!(contrato::check(&v,"DA01","S1",&p,"A01",0).is_err());}
    #[test]fn preserva_recepcion_sse_y_rechaza_alteracion(){let b=include_str!("../../../../gpt-6-astra/prueba-entrega-20261006/respuesta.sse");let mut ev=b.lines().filter_map(|s|s.strip_prefix("data:")).filter(|s|s.trim()!="[DONE]").map(|s|serde_json::from_str::<Value>(s).unwrap()).collect::<Vec<_>>();assert!(recepcion::extract(&ev).is_ok());let v=ev.iter_mut().find(|e|e["type"]=="response.output_text.delta").unwrap();v["delta"]=json!("corrupto");assert!(recepcion::extract(&ev).is_err());}
}
