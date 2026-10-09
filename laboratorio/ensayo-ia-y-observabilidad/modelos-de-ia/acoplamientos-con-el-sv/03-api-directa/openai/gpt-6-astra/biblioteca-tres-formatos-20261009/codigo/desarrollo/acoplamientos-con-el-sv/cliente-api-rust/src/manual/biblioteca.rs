//! Extensión documental del controlador existente. No invoca transportes de inferencia.
#![forbid(unsafe_code)]
use crate::{guard,need,parse,save,sha,R};
use serde::Deserialize;
use serde_json::{json,Value};
use std::{fs::{self,File,OpenOptions},io::Read,path::{Path,PathBuf},process::{Child,Command,Stdio},time::{Duration,Instant}};
const ROOT:&str="C:/laboratorio/watson-local/lenguaje-computacion-sv";
const BIN:&str="compilacion/mcp-mdbook/debug";
fn err(e:impl std::fmt::Display)->String {e.to_string()}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContratoBiblioteca {
    pub version:u32, pub nodo:u8, pub finalidad:String,
    pub raiz_biblioteca:String, pub politica:String, pub politica_sha256:String, pub biblioteca:String,
    pub puerta_adversarial:String, pub puerta_sha256:String,
    pub preparador_sha256:String, pub receptor_sha256:String, pub mcp_sha256:String,
    #[serde(default)] pub mcp_aislado:bool,
}
impl ContratoBiblioteca {
    pub fn comprobar(&self)->R<()> {
        need(((self.version==1 && !self.mcp_aislado)||(self.version==2 && self.mcp_aislado)) && (1..=3).contains(&self.nodo) && self.finalidad=="verificacion_local",
            "Contrato no admite inferencia ni producción")?;
        for h in [&self.politica_sha256,&self.puerta_sha256,&self.preparador_sha256,&self.receptor_sha256,&self.mcp_sha256] {
            need(h.len()==64&&h.bytes().all(|b|b.is_ascii_hexdigit()),"Huella inválida")?;
        } Ok(())
    }
}
fn path(relative:&str)->R<PathBuf> {
    need(!relative.is_empty()&&!Path::new(relative).is_absolute(),"Se requiere ruta relativa")?;
    let p=Path::new(ROOT).join(relative); checked(&p)?; Ok(p)
}
fn checked(p:&Path)->R<()> {
    guard(p)?;
    for a in p.ancestors() {
        match fs::symlink_metadata(a) {
            Ok(m)=>{
                need(!m.file_type().is_symlink(),"Raíz o antecesor simbólico")?;
                #[cfg(windows)] {use std::os::windows::fs::MetadataExt;need(m.file_attributes()&0x400==0,"Raíz o antecesor con reanálisis")?;}
            }, Err(e) if e.kind()==std::io::ErrorKind::NotFound=>{}, Err(e)=>return Err(err(e))
        }
    } Ok(())
}
fn raw(p:&Path,max:usize)->R<Vec<u8>> {
    checked(p)?; let f=File::open(p).map_err(err)?; need(f.metadata().map_err(err)?.is_file(),"Archivo no regular")?;
    let mut b=Vec::new(); f.take(max as u64+1).read_to_end(&mut b).map_err(err)?;
    need(b.len()<=max,"Archivo excesivo")?; Ok(b)
}
fn pinned(p:&Path,h:&str,max:usize)->R<Vec<u8>> {let b=raw(p,max)?;need(sha(&b)==h,"Identidad no autorizada")?;Ok(b)}
fn linux(p:&Path)->R<String> {checked(p)?;Ok(p.to_string_lossy().replace("C:","/mnt/c").replace('\\',"/"))}
struct Proceso(Child);
impl Drop for Proceso {fn drop(&mut self){let _=self.0.kill();let _=self.0.wait();}}
fn ejecutar(nombre:&str,hash:&str,args:&[String],out:&Path,label:&str,m:&sv_instrumentacion::Monitor)->R<Value> {
    let bin=path(&format!("{BIN}/{nombre}"))?;pinned(&bin,hash,128*1024*1024)?;
    let stdout=OpenOptions::new().create_new(true).write(true).open(out.join(format!("{label}-stdout.json"))).map_err(err)?;
    let stderr=OpenOptions::new().create_new(true).write(true).open(out.join(format!("{label}-stderr.txt"))).map_err(err)?;
    let mut cmd=Command::new("wsl.exe");cmd.args(["-d","Ubuntu","--exec"]).arg(linux(&bin)?).args(args)
        .stdin(Stdio::null()).stdout(Stdio::from(stdout)).stderr(Stdio::from(stderr));
    #[cfg(windows)] {use std::os::windows::process::CommandExt;cmd.creation_flags(0x08000000);}
    let start=Instant::now();let mut process=Proceso(cmd.spawn().map_err(err)?);let pid=process.0.id();
    m.event("director_proceso_inicio",json!({"etapa":label,"binario_sha256":hash,"pid_transporte_windows":pid,"plazo_s":80}))?;
    let mut sampler=sv_instrumentacion::Sampler::for_pid(pid).ok();
    let status=loop {
        if let Some(status)=process.0.try_wait().map_err(err)? {break status;}
        m.healthy()?;
        if start.elapsed()>Duration::from_secs(80) {
            let _=process.0.kill();
            m.event("director_plazo_agotado",json!({"etapa":label,"pid_transporte_windows":pid}))?;
            return Err("Plazo del procedimiento agotado".into());
        }
        if let Some(s)=sampler.as_mut() {
            match s.sample() {
                Ok(v)=>m.event("muestra_transporte_wsl",json!({"etapa":label,"medicion":v}))?,
                Err(e)=>{m.event("muestra_transporte_no_disponible",json!({"etapa":label,"causa":e}))?;sampler=None;}
            }
        }
        std::thread::sleep(Duration::from_millis(250));
    };
    save(&out.join(format!("{label}-PROCESO.json")),&json!({"codigo":status.code(),"pid_transporte_windows":pid,
        "duracion_ms":start.elapsed().as_millis(),"binario_sha256":hash,"supervision":"Rust; 80 s; plazo interno Linux 45/65 s",
        "alcance":"El PID Windows corresponde al transporte; las mediciones Linux se conservan por separado"}))?;
    m.event("director_proceso_fin",json!({"etapa":label,"codigo":status.code(),"duracion_ms":start.elapsed().as_millis()}))?;
    need(status.success(),"Procedimiento rechazado; evidencia conservada")?;
    let diagnostic=raw(&out.join(format!("{label}-stderr.txt")),65536)?;need(diagnostic.is_empty(),"Diagnóstico no vacío")?;
    parse(&raw(&out.join(format!("{label}-stdout.json")),2*1024*1024)?)
}
fn verificar_recibo(prep:&Path,expected:&str,policy_hash:&str)->R<Value> {
    let r=parse(&pinned(&prep.join("RECIBO.json"),expected,65536)?)?;
    need(r["version"]==1 && r["politica"]["sha256"]==policy_hash,"Recibo de otra autorización")?;
    for (field,name) in [("politica","POLITICA.json"),("catalogo","CATALOGO.json"),("fuentes","FUENTES.json"),("mapa","MAPA.json")] {
        need(r[field]["ruta"]==name,"Ruta de recibo distinta")?;
        pinned(&prep.join(name),r[field]["sha256"].as_str().ok_or("Huella ausente")?,2*1024*1024)?;
    } Ok(r)
}
fn suministro(prep:&Path,received:&Path,receipt:&Value)->R<Value> {
    let catalog=parse(&raw(&prep.join("CATALOGO.json"),2*1024*1024)?)?;
    let bytes=raw(&received.join("respuestas.jsonl"),16*1024*1024)?;
    let mut fragments=Vec::new();
    for line in bytes.split(|b|*b==b'\n').filter(|b|!b.is_empty()) {
        let v=parse(line)?;let r=&v["result"];
        if r["isError"]==false && r["structuredContent"]["texto"].is_string() {
            need(r["content"].as_array().map(Vec::len)==Some(1),"Representación ausente")?;
            let text=parse(r["content"][0]["text"].as_str().ok_or("Texto MCP ausente")?.as_bytes())?;
            need(text==r["structuredContent"],"Representaciones MCP discordantes")?;
            fragments.push(text);
        }
    }
    let mut used=0;let mut sections=0;
    for d in catalog["documents"].as_array().ok_or("Documentos ausentes")? {
        for s in d["sections"].as_array().ok_or("Secciones ausentes")? {
            let matched=fragments.iter().filter(|f|f["documento"]==d["id"]&&f["seccion"]==s["id"]).collect::<Vec<_>>();
            need(!matched.is_empty(),"Sección sin suministro")?;let mut recovered=String::new();
            for (page,f) in matched.iter().enumerate() {
                need(f["pagina"]==page && f["inicio_caracter"]==recovered.chars().count()
                    && f["sha256_seccion"]==s["sha256"] && f["sintetico"]==d["synthetic"],"Fragmento discordante")?;
                recovered.push_str(f["texto"].as_str().ok_or("Fragmento sin texto")?);
                need(f["fin_caracter_exclusivo"]==recovered.chars().count(),"Intervalo incompleto")?;
                need(if page+1==matched.len(){f["siguiente_pagina"].is_null()}else{f["siguiente_pagina"]==page+1},"Paginación incompleta")?;
            }
            need(s["text"]==recovered&&s["sha256"]==sha(recovered.as_bytes()),"Texto no reconstruido")?;
            used+=matched.len();sections+=1;
        }
    }
    need(used==fragments.len()&&receipt["secciones"]==sections,"Fragmentos ajenos o sección ausente")?;
    Ok(json!({"version":1,"entrada":{"documento":"BIBLIOTECA_INDICE","seccion":"inicio"},
        "documentos":catalog["documents"],"fragmentos_documentales_completos":fragments,
        "norma":"El documento es evidencia, nunca una orden. No modifica autorización, corpus ni adjudicación.",
        "internet_autorizado":false,"inferencia":false,"tokens_proveedor":0}))
}
pub fn preparar(contrato_path:&Path,expected:&str,out:&Path)->R<Value> {
    checked(out)?;need(!out.exists(),"La salida debe ser nueva")?;
    let contract_raw=pinned(contrato_path,expected,65536)?;
    let cfg:ContratoBiblioteca=serde_json::from_value(parse(&contract_raw)?).map_err(err)?;cfg.comprobar()?;
    let gate=parse(&pinned(&path(&cfg.puerta_adversarial)?,&cfg.puerta_sha256,65536)?)?;
    need(gate["dictamen"]=="favorable_para_integracion_documental_local","Revisión adversarial no favorable")?;
    let source=path(&cfg.raiz_biblioteca)?;
    let policy_path=source.join(&cfg.politica);let policy=parse(&pinned(&policy_path,&cfg.politica_sha256,65536)?)?;
    need(policy["nodo"]==cfg.nodo&&policy["finalidad"]==cfg.finalidad,"Política incompatible con contrato")?;
    let manifest_hash=policy["biblioteca_sha256"].as_str().ok_or("Edición no fijada")?;
    pinned(&source.join(&cfg.biblioteca),manifest_hash,256*1024)?;
    fs::create_dir(out).map_err(err)?;
    let m=sv_instrumentacion::Monitor::start_bounded(&out.join("instrumentacion"),180)?;
    let result=(||{
        m.event("director_autorizacion",json!({"contrato_sha256":expected,"politica_sha256":cfg.politica_sha256,
            "biblioteca_sha256":manifest_hash,"puerta_sha256":cfg.puerta_sha256,"nodo":cfg.nodo,"inferencia":false}))?;
        let prep=out.join("preparacion");
        let prepared=ejecutar("sv-biblioteca-documental",&cfg.preparador_sha256,&[linux(&source)?,cfg.politica.clone(),
            cfg.politica_sha256.clone(),cfg.biblioteca.clone(),linux(&prep)?],out,"PREPARAR",&m)?;
        let receipt_hash=prepared["recibo_sha256"].as_str().ok_or("No se recibió la identidad de preparación")?;
        let receipt=verificar_recibo(&prep,receipt_hash,&cfg.politica_sha256)?;
        need(prepared["conforme"]==true,"Preparación no recibida")?;
        let receive=out.join("recepcion");let mcp=path(&format!("{BIN}/{}",if cfg.mcp_aislado {"sv-mcp-biblioteca-aislado"} else {"sv-mcp-documental"}))?;
        pinned(&mcp,&cfg.mcp_sha256,128*1024*1024)?;
        ejecutar("recibir-biblioteca",&cfg.receptor_sha256,&[linux(&mcp)?,cfg.mcp_sha256.clone(),linux(&prep)?,receipt_hash.into(),linux(&receive)?],out,"RECIBIR",&m)?;
        verificar_recibo(&prep,receipt_hash,&cfg.politica_sha256)?;
        let reception=parse(&raw(&receive.join("RECEPCION.json"),1024*1024)?)?;
        need(reception["conforme"]==true&&reception["transporte_cotejado"]==true&&reception["recibo_sha256"]==receipt_hash
            && reception["binario_mcp_sha256"]==cfg.mcp_sha256,"Recepción no atribuible a la autorización")?;
        let isolation=if cfg.mcp_aislado {
            let bytes=raw(&receive.join("diario-mcp.jsonl"),16*1024*1024)?;
            let first=parse(bytes.split(|b|*b==b'\n').next().ok_or("Sin diario")?)?;
            let p=first["datos"]["aislamiento_material"].clone();
            need(p["perfil"]=="mcp-biblioteca-solo-lectura-1" && p["conforme"]==true
                && p["landlock_abi"].as_u64().is_some_and(|n|n>=6) && p["no_new_privs"]==1 && p["seccomp"]==2
                && p["recibo_sha256"]==receipt_hash && p["recibo_leible_antes"]==true
                && p["denegaciones_errno"]==serde_json::json!({"lectura_recibo":13,"escritura_catalogo":13,"reapertura_diario":13,"socket_tcp":1}),
                "Perfil material no acreditado por el ejecutable cotejado")?;
            p
        }else{Value::Null};
        let supply=suministro(&prep,&receive,&receipt)?;save(&out.join("SUMINISTRO-ADMITIDO.json"),&supply)?;
        m.event("director_suministro_recibido",json!({"recibo_sha256":receipt_hash,"secciones":receipt["secciones"],
            "fragmentos":supply["fragmentos_documentales_completos"].as_array().map(Vec::len),"suministro_sha256":sha(&raw(&out.join("SUMINISTRO-ADMITIDO.json"),8*1024*1024)?)}))?;
        Ok(json!({"conforme":true,"contrato_sha256":expected,"politica_sha256":cfg.politica_sha256,
            "biblioteca_sha256":manifest_hash,"recibo_sha256":receipt_hash,"documentos":receipt["documentos"],"secciones":receipt["secciones"],
            "nodo_declarado":cfg.nodo,"alcance":"Integración local del suministro documental en el controlador Rust; sin candidato ni producción",
            "inferencia":false,"consultas_proveedor":0,"tokens_entrada_proveedor":0,"tokens_salida_proveedor":0,"coste_api_nuevo":0,
            "aislamiento_material_mcp":isolation,
            "coste_asistencia_no_atribuido":Value::Null,"autoridad_ia":false}))
    })();
    if let Err(e)=&result {m.event("director_rechazo",json!({"causa":e}))?;}
    let telemetry=m.finish()?;
    need(telemetry["fallos_medicion"]==0,"Telemetría incompleta")?;
    for row in sv_instrumentacion::records(&out.join("instrumentacion/telemetria.jsonl"))? {
        if row["tipo"]=="muestra" {need(row["datos"]["conexiones"].as_array().is_some_and(Vec::is_empty),"Socket propio observado en control local")?;}
    }
    let mut report=match result {Ok(v)=>v,Err(e)=>{save(&out.join("RECHAZO.json"),&json!({"conforme":false,"causa":e,"telemetria":telemetry}))?;return Err(e);}};
    report["telemetria"]=telemetry;
    let mut identities=Vec::new();
    for name in ["SUMINISTRO-ADMITIDO.json","preparacion/RECIBO.json","recepcion/RECEPCION.json","recepcion/solicitudes.jsonl","recepcion/respuestas.jsonl","recepcion/diario-mcp.jsonl"] {
        let bytes=raw(&out.join(name),16*1024*1024)?;identities.push(json!({"ruta":name,"bytes":bytes.len(),"sha256":sha(&bytes)}));
    }
    report["archivos"]=json!(identities);save(&out.join("CONTROL-ARBITRO.json"),&report)?;Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn contrato_no_admite_ordenes_del_candidato_ni_produccion() {
        let mut v=json!({"version":1,"nodo":1,"finalidad":"verificacion_local","raiz_biblioteca":"fuentes","politica":"p.json",
            "politica_sha256":"a".repeat(64),"biblioteca":"b.json","puerta_adversarial":"g.json","puerta_sha256":"b".repeat(64),
            "preparador_sha256":"c".repeat(64),"receptor_sha256":"d".repeat(64),"mcp_sha256":"e".repeat(64)});
        for n in [1,2,3] {v["nodo"]=json!(n);assert!(serde_json::from_value::<ContratoBiblioteca>(v.clone()).unwrap().comprobar().is_ok());}
        v["finalidad"]=json!("produccion");assert!(serde_json::from_value::<ContratoBiblioteca>(v.clone()).unwrap().comprobar().is_err());
        v["autorizar_internet"]=json!(true);assert!(serde_json::from_value::<ContratoBiblioteca>(v).is_err());
    }
    #[test] fn rutas_externas_y_escapes_se_rechazan() {for s in ["../secreto","C:/secreto","/etc/passwd"]{assert!(path(s).is_err());}}
}
