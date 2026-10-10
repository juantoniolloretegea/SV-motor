#![forbid(unsafe_code)]
use serde_json::{json,Value};
use std::{fs::{self,OpenOptions,File},io::{Read,Write,BufRead,BufReader},path::{Path,PathBuf,Component},process::{Command,Stdio,Child,ChildStdin},sync::mpsc,thread,time::{Instant,Duration}};
use sv_suministro_pdf_astra::{*,DireccionPdf};
const ROOT:&str="C:/SV-LABORATORIO";
const ANNEX:&str="desarrollo/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/anexo-pdf-20261007";
const PDF_PATH:&str="desarrollo/mcp-pdf-preparacion-20261003/lector/tests/fixtures/hairy-cell-leukemia.pdf";
const BIN:&str="compilacion/mcp-pdf-linux/debug";
fn err(e:impl std::fmt::Display)->String{e.to_string()}
fn guard(p:&Path)->R<()> {
    let rel=p.strip_prefix(ROOT).map_err(|_|"Ruta fuera del perímetro")?;let mut current=PathBuf::from(ROOT);
    for c in rel.components(){need(matches!(c,Component::Normal(_)),"Ruta no normal")?;current.push(c);
        if current.exists(){let m=fs::symlink_metadata(&current).map_err(err)?;
            #[cfg(windows)]{use std::os::windows::fs::MetadataExt;need(m.file_attributes()&0x400==0,"Reanálisis no permitido")?;}
            need(!m.file_type().is_symlink(),"Enlace no permitido")?;
        }
    }Ok(())
}
fn path(s:&str)->R<PathBuf>{let p=Path::new(ROOT).join(s);guard(&p)?;Ok(p)}
fn raw(p:&Path,max:usize)->R<Vec<u8>>{guard(p)?;let f=File::open(p).map_err(err)?;need(f.metadata().map_err(err)?.is_file(),"Archivo no regular")?;let mut b=vec![];f.take(max as u64+1).read_to_end(&mut b).map_err(err)?;need(b.len()<=max,"Archivo excede cota")?;Ok(b)}
fn load(p:&Path)->R<Value>{parse(&raw(p,64*1024*1024)?)}
fn put(p:&Path,b:&[u8])->R<()>{guard(p)?;let mut f=OpenOptions::new().create_new(true).write(true).open(p).map_err(err)?;f.write_all(b).and_then(|_|f.sync_all()).map_err(err)}
fn save(p:&Path,v:&Value)->R<()>{put(p,&serde_json::to_vec_pretty(v).map_err(err)?)}
fn linux(p:&Path)->R<String>{guard(p)?;Ok(p.to_string_lossy().replace("C:","/mnt/c").replace('\\',"/"))}
fn command(bin:&str,args:&[String])->R<Command>{
    let mut c=Command::new("wsl.exe");c.args(["-d","Ubuntu","--exec","/usr/bin/timeout","--signal=KILL","30s"]);
    c.arg(linux(&path(&format!("{BIN}/{bin}"))?)?).args(args);
    #[cfg(windows)]{use std::os::windows::process::CommandExt;c.creation_flags(0x08000000);}
    Ok(c)
}
struct Process{child:Child,input:Option<ChildStdin>,rx:mpsc::Receiver<R<Vec<u8>>>,stdout:Vec<u8>,stderr:Option<thread::JoinHandle<std::io::Result<Vec<u8>>>>,reader:Option<thread::JoinHandle<()>>,started:Instant,pid:u32,requests:Vec<u8>}
impl Process{
    fn start(mut cmd:Command)->R<Self>{
        let mut child=cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().map_err(err)?;
        let pid=child.id();let input=child.stdin.take();let stdout=child.stdout.take().ok_or("stdout")?;let stderr=child.stderr.take().ok_or("stderr")?;
        let(tx,rx)=mpsc::channel();
        let reader=thread::spawn(move||{let mut rd=BufReader::new(stdout);let mut total=0;loop{let mut line=vec![];let result=rd.by_ref().take(65537).read_until(b'\n',&mut line);match result{Err(e)=>{let _=tx.send(Err(e.to_string()));break;},Ok(0)=>break,Ok(_)=>{total+=line.len();if line.len()>65536||total>16*1024*1024||!line.ends_with(b"\n"){let _=tx.send(Err("Trama incompleta o excesiva".into()));break;}if tx.send(Ok(line)).is_err(){break;}}}}});
        let stderr=thread::spawn(move||{let mut b=vec![];stderr.take(65537).read_to_end(&mut b).map(|_|b)});
        Ok(Self{child,input,rx,stdout:vec![],stderr:Some(stderr),reader:Some(reader),started:Instant::now(),pid,requests:vec![]})
    }
    fn write(&mut self,v:&Value)->R<()>{let mut b=serde_json::to_vec(v).map_err(err)?;b.push(b'\n');need(b.len()<=16384,"Solicitud MCP demasiado grande")?;self.input.as_mut().ok_or("Entrada cerrada")?.write_all(&b).map_err(err)?;self.input.as_mut().unwrap().flush().map_err(err)?;self.requests.extend(b);Ok(())}
    fn response(&mut self)->R<Value>{let left=Duration::from_secs(35).checked_sub(self.started.elapsed()).ok_or("Plazo auxiliar agotado")?;let b=self.rx.recv_timeout(left).map_err(err)??;let v=parse(&b)?;self.stdout.extend(b);Ok(v)}
    fn finish(mut self,out:&Path,label:&str)->R<Vec<u8>>{
        self.input.take();loop{if self.child.try_wait().map_err(err)?.is_some(){break;}need(self.started.elapsed()<Duration::from_secs(35),"Proceso auxiliar no termina")?;thread::sleep(Duration::from_millis(20));}
        self.reader.take().unwrap().join().map_err(|_|"Lector fallido")?;
        for b in self.rx.try_iter(){self.stdout.extend(b?);}
        let stderr=self.stderr.take().unwrap().join().map_err(|_|"stderr fallido")?.map_err(err)?;
        let status=self.child.wait().map_err(err)?;
        put(&out.join(format!("{label}-SOLICITUDES.jsonl")),&self.requests)?;put(&out.join(format!("{label}-RESPUESTAS.jsonl")),&self.stdout)?;put(&out.join(format!("{label}-stderr.txt")),&stderr)?;
        save(&out.join(format!("{label}-PROCESO.json")),&json!({"pid_windows":self.pid,"duracion_ms":self.started.elapsed().as_millis(),"codigo":status.code(),"alcance":"PID del transporte WSL, no consumo del proceso Linux","limite_linux_segundos":30,"stdin_cerrado":true}))?;
        need(status.success()&&stderr.len()<=65536,"Proceso auxiliar no conforme; véanse originales")?;Ok(self.stdout.clone())
    }
}
impl Drop for Process{fn drop(&mut self){self.input.take();let _=self.child.kill();let _=self.child.wait();}}
fn once(bin:&str,args:&[String],out:&Path,label:&str)->R<Value>{let p=Process::start(command(bin,args)?)?;let b=p.finish(out,label)?;parse(&b)}
fn from_hex(s:&str)->R<Vec<u8>>{need(s.len()%2==0,"Hexadecimal incompleto")?;s.as_bytes().chunks(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).map_err(err)?,16).map_err(err)).collect()}
fn cotejar_diario(base:&Path,bin_hash:&str,source_hash:&str)->R<()> {
    let mut req=vec![];let mut rep=vec![];
    let raw_log=raw(&base.join("MCP-DIARIO.jsonl"),64*1024*1024)?;
    for(i,line)in raw_log.split(|b|*b==b'\n').filter(|b|!b.is_empty()).enumerate(){let v=parse(line)?;let d=&v["datos"];if i==0{need(d["binario_sha256"]==bin_hash&&d["fuentes_autorizadas_sha256"]==source_hash,"Identidad efectiva del MCP distinta")?;}
        match d["evento"].as_str(){Some("solicitud")=>req.extend(from_hex(d["bytes_hex"].as_str().ok_or("Bytes de petición ausentes")?)?),Some("resultado")=>rep.extend(from_hex(d["bytes_hex"].as_str().ok_or("Bytes de respuesta ausentes")?)?),_=>{}}
    }
    need(req==raw(&base.join("MCP-SOLICITUDES.jsonl"),16*1024*1024)?&&rep==raw(&base.join("MCP-RESPUESTAS.jsonl"),16*1024*1024)?,"Transporte y diario discordantes")
}
fn inventory()->R<Vec<Value>>{
    let mut out=vec![];
    for(name,role)in[("preparar-pdf","extracción Rust aislada"),("sv-mcp-documental","servicio documental Rust aislado"),("verificar-diario","reproducción Rust del protocolo")]{let p=path(&format!("{BIN}/{name}"))?;let b=raw(&p,128*1024*1024)?;out.push(json!({"ruta":format!("{BIN}/{name}"),"funcion":role,"bytes":b.len(),"sha256":sha(&b)}));}
    let executable=std::env::current_exe().map_err(err)?;guard(&executable)?;let b=raw(&executable,128*1024*1024)?;
    out.push(json!({"ruta":executable.strip_prefix(ROOT).map_err(err)?.to_string_lossy(),"funcion":"auxiliar de admisión documental del Árbitro-Director; composición sin envío y telemetría Rust","bytes":b.len(),"sha256":sha(&b)}));Ok(out)
}
fn prepare(base:&Path)->R<Value>{
    let bank=raw(&path(&format!("{ANNEX}/CANDIDATO.json"))?,128*1024)?;banco(&bank)?;
    let key=raw(&path(&format!("{ANNEX}/CRITERIOS-EVALUADOR.md"))?,128*1024)?;need(sha(&key)==CLAVE,"Clave recibida alterada")?;
    let pdf=path(PDF_PATH)?;need(sha(&raw(&pdf,20*1024*1024)?)==PDF,"Original PDF alterado")?;
    let components=inventory()?;
    let monitor=sv_instrumentacion::Monitor::start_bounded(&base.join("instrumentacion"),180)?;
    monitor.event("director_inicio",json!({"suceso":"S39","tique":"TT-0021","banco_sha256":BANCO,"clave_sha256":CLAVE,"inferencia_habilitada":false,"componentes":components}))?;
    let source=base.join("documento");
    let prepared=once("preparar-pdf",&[linux(&pdf)?,PDF.into(),DOC.into(),URL.into(),linux(&source)?],base,"EXTRACCION")?;
    need(prepared["preparado"]==true&&prepared["red_bloqueada"]==true&&prepared["paginas"]==10,"Preparación PDF no conforme")?;
    let cat=load(&source.join("CATALOGO.json"))?;let extraction=load(&source.join("EXTRACCION.json"))?;let sources=load(&source.join("FUENTES.json"))?;
    let cat_hash=sha(&raw(&source.join("CATALOGO.json"),2*1024*1024)?);let sources_hash=sha(&raw(&source.join("FUENTES.json"),65536)?);
    need(prepared["catalogo_sha256"]==cat_hash&&prepared["fuentes_sha256"]==sources_hash&&prepared["binario_sha256"]==components[0]["sha256"],"Recibo del preparador no corresponde")?;
    let mut director=DireccionPdf::nueva(&bank,cat.clone(),&extraction,&sources)?;
    let isolation=once("sv-mcp-documental",&["--probe-isolation".into(),linux(&source.join("CATALOGO.json"))?],base,"AISLAMIENTO")?;
    need(isolation["red_externa_error"]==1&&isolation["red_local_error"]==1,"Aislamiento MCP no conforme")?;
    let journal=base.join("MCP-DIARIO.jsonl");let mut p=Process::start(command("sv-mcp-documental",&[linux(&source.join("CATALOGO.json"))?,cat_hash.clone(),linux(&journal)?,"256".into(),"--fuentes-autorizadas".into(),linux(&source.join("FUENTES.json"))?,sources_hash.clone()])?)?;
    p.write(&json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"sv-suministro-pdf-astra","version":"0.1.0"}}}))?;
    let init=p.response()?;need(init["id"]==0&&init["result"]["protocolVersion"]=="2025-06-18"&&init.get("error").is_none(),"Inicialización MCP inválida")?;
    p.write(&json!({"jsonrpc":"2.0","method":"notifications/initialized"}))?;
    p.write(&json!({"jsonrpc":"2.0","id":1,"method":"tools/list"}))?;
    let list=p.response()?;need(list["id"]==1&&list["result"]["tools"].as_array().is_some_and(|t|t.len()==2&&t.iter().any(|v|v["name"]=="leer_documento")&&t.iter().any(|v|v["name"]=="buscar_documentos")),"Herramientas MCP distintas")?;
    let mut pages=vec![];let mut id=2;
    for ordinal in 1..=10{
        let mut fragments=vec![];let mut page=0;
        loop{monitor.healthy()?;need(id<130,"Presupuesto de llamadas MCP agotado")?;
            let t=Instant::now();p.write(&json!({"jsonrpc":"2.0","id":id,"method":"tools/call","params":{"name":"leer_documento","arguments":{"documento":DOC,"seccion":format!("PDF-P{:04}",ordinal-1),"pagina":page}}}))?;
            let f=contenido(&p.response()?,id)?;
            need(f["pagina"]==page&&f["pagina_pdf_ordinal"]==ordinal,"Secuencia MCP incorrecta")?;
            let next=f["siguiente_pagina"].clone();monitor.event("fragmento_recibido",json!({"pagina_fisica":ordinal,"fragmento":page,"duracion_us":t.elapsed().as_micros(),"bytes":f["texto"].as_str().ok_or("Sin texto")?.len(),"rpc_id":id}))?;
            fragments.push(f);id+=1;
            if next.is_null(){break;}let n=num(&next)?;need(n==page+1,"Cursor omitido o circular")?;page=n;
        }
        let data=Pagina{ordinal,fragmentos:fragments};data.comprobar(&cat["documents"][0]["sections"][ordinal-1])?;
        monitor.event("pagina_completa",json!({"ordinal":ordinal,"fragmentos":data.fragmentos.len(),"sha256":cat["documents"][0]["sections"][ordinal-1]["sha256"]}))?;pages.push(data);
    }
    p.finish(base,"MCP")?;
    let audited=once("verificar-diario",&[linux(&source.join("CATALOGO.json"))?,cat_hash.clone(),linux(&journal)?,"--oficial".into(),"--fuentes-autorizadas".into(),linux(&source.join("FUENTES.json"))?,sources_hash.clone()],base,"MCP-COTEJO")?;
    cotejar_diario(base,components[1]["sha256"].as_str().unwrap(),&sources_hash)?;
    director.recibir(pages.clone(),&isolation,&audited,&sha(&raw(&journal,64*1024*1024)?))?;
    monitor.event("director_recepcion_documental",json!({"paginas":10,"fragmentos":id-2,"diario_cotejado":true,"inferencia":false}))?;
    let measured=monitor.finish()?;director.admitir(&measured,&sha(&key))?;
    let mut receipts=vec![];let requests=base.join("solicitudes-previstas");fs::create_dir(&requests).map_err(err)?;
    for n in 0..9{let req=director.solicitud(n)?;director.cotejar_solicitud(n,&req)?;let bytes=serde_json::to_vec_pretty(&req).map_err(err)?;put(&requests.join(format!("PDF{:02}.json",n+1)),&bytes)?;
        receipts.push(json!({"caso":format!("PDF{:02}",n+1),"paginas_fisicas":PAGINAS[n],"fragmentos":PAGINAS[n].iter().map(|p|pages[p-1].fragmentos.len()).sum::<usize>(),"solicitud_bytes":bytes.len(),"solicitud_sha256":sha(&bytes),"suministro_admitido":true,"envio_autorizado_por_este_instrumento":false}));
    }
    let proof=json!({"esquema":"sv-admision-documental-pdf/1","estado":"suministro_documental_admitido_sin_inferencia","suceso":"S39","tique":"TT-0021","arbitro_director":{"funcion_ejercida":"control de fuente, recorrido, aislamiento, instrumentación y composición por lista cerrada","adjudicacion_ejercida":false,"clave_en_contexto":false,"continuacion_automatica":false},"original_sha256":PDF,"banco_sha256":BANCO,"clave_sha256":CLAVE,"catalogo_sha256":cat_hash,"fuentes_sha256":sources_hash,"paginas_fisicas":10,"fragmentos":id-2,"diario":audited,"aislamiento":isolation,"telemetria":measured,"componentes":components,"casos":receipts,"inferencias":0,"herramientas_candidato":0,"proveedor_consultado":false,"tokens_candidato":0,"creditos_atribuibles":null,"coste_asistencia":null,"recepcion_cientifica":false});
    save(&base.join("CONTROL-ARBITRO.json"),&proof)?;Ok(proof)
}
fn verify(base:&Path)->R<Value>{
    let control=load(&base.join("CONTROL-ARBITRO.json"))?;
    need(control["banco_sha256"]==BANCO&&control["clave_sha256"]==CLAVE&&control["inferencias"]==0,"Control ajeno")?;
    let source=base.join("documento");let cat=load(&source.join("CATALOGO.json"))?;let ext=load(&source.join("EXTRACCION.json"))?;let sources=load(&source.join("FUENTES.json"))?;
    need(sha(&raw(&source.join("CATALOGO.json"),2*1024*1024)?)==control["catalogo_sha256"]&&sha(&raw(&source.join("FUENTES.json"),65536)?)==control["fuentes_sha256"],"Catálogo o fuentes alterados")?;
    let bank=raw(&path(&format!("{ANNEX}/CANDIDATO.json"))?,128*1024)?;let mut director=DireccionPdf::nueva(&bank,cat.clone(),&ext,&sources)?;
    let rows=raw(&base.join("MCP-RESPUESTAS.jsonl"),16*1024*1024)?;let mut pages:Vec<Pagina>=(1..=10).map(|ordinal|Pagina{ordinal,fragmentos:vec![]}).collect();let mut id=2;
    for row in rows.split(|b|*b==b'\n').filter(|s|!s.is_empty()).skip(2){let f=contenido(&parse(row)?,id)?;let ordinal=num(&f["pagina_pdf_ordinal"])?;need((1..=10).contains(&ordinal),"Página ajena")?;pages[ordinal-1].fragmentos.push(f);id+=1;}
    let replay=once("verificar-diario",&[linux(&source.join("CATALOGO.json"))?,control["catalogo_sha256"].as_str().ok_or("Huella catálogo")?.into(),linux(&base.join("MCP-DIARIO.jsonl"))?,"--oficial".into(),"--fuentes-autorizadas".into(),linux(&source.join("FUENTES.json"))?,control["fuentes_sha256"].as_str().ok_or("Huella fuentes")?.into()],base,"VERIFICACION-DIARIO")?;
    let components=inventory()?;need(json!(components)==control["componentes"],"Ejecutables distintos")?;
    cotejar_diario(base,components[1]["sha256"].as_str().unwrap(),control["fuentes_sha256"].as_str().unwrap())?;
    let measured=sv_instrumentacion::verify(&base.join("instrumentacion/telemetria.jsonl"))?;need(measured==control["telemetria"],"Recibo de telemetría alterado")?;
    let isolation=parse(&raw(&base.join("AISLAMIENTO-RESPUESTAS.jsonl"),65536)?)?;need(isolation==control["aislamiento"],"Recibo de aislamiento distinto")?;
    director.recibir(pages,&isolation,&replay,&sha(&raw(&base.join("MCP-DIARIO.jsonl"),64*1024*1024)?))?;
    let key_hash=sha(&raw(&path(&format!("{ANNEX}/CRITERIOS-EVALUADOR.md"))?,128*1024)?);director.admitir(&measured,&key_hash)?;
    need(control["casos"].as_array().map(Vec::len)==Some(9),"Recibos incompletos")?;
    for n in 0..9{let b=raw(&base.join(format!("solicitudes-previstas/PDF{:02}.json",n+1)),1024*1024)?;director.cotejar_solicitud(n,&parse(&b)?)?;need(sha(&b)==control["casos"][n]["solicitud_sha256"],"Huella de solicitud distinta")?;}
    Ok(json!({"conforme":true,"casos":9,"paginas":10,"fragmentos":id-2,"diario_reproducido":true,"solicitudes_recompuestas_identicas":true,"telemetria_cotejada":true,"inferencias":0,"control_sha256":sha(&raw(&base.join("CONTROL-ARBITRO.json"),1024*1024)?)}))
}
fn main(){let a:Vec<_>=std::env::args().collect();let result=(||->R<Value>{need(a.len()==3,"Uso: sv-suministro-pdf-astra preparar|verificar ejecucion/astra-pdf-20261007/NUEVO")?;need(a[2].starts_with("ejecucion/astra-pdf-20261007/"),"Destino fuera del ensayo")?;let base=path(&a[2])?;
    match a[1].as_str(){"preparar"=>{need(!base.exists(),"Destino existente: no sobrescribir")?;fs::create_dir_all(&base).map_err(err)?;match prepare(&base){Ok(v)=>Ok(v),Err(e)=>{let _=save(&base.join("DETENCION.json"),&json!({"causa":e,"fuera_de_terna":true,"inferencia_habilitada":false}));Err(e)}}},"verificar"=>{let v=verify(&base)?;save(&base.join("COTEJO-SUMINISTRO-RUST.json"),&v)?;Ok(v)},_=>Err("Operación no admitida".into())}
})();match result{Ok(v)=>println!("{}",json!({"estado":v["estado"],"conforme":v.get("conforme"),"casos":v["casos"].as_array().map(Vec::len).map(|n|json!(n)).unwrap_or_else(||v["casos"].clone()),"paginas":v.get("paginas_fisicas").unwrap_or(&v["paginas"]),"fragmentos":v["fragmentos"],"inferencias":0})),Err(e)=>{eprintln!("Detención del suministro: {e}");std::process::exit(1)}}}
