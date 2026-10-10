#![forbid(unsafe_code)]
use serde_json::{json,Value};
use std::{fs,path::Path,collections::HashSet};
use sv_claude_kaggle::{need,parse,R,LICENCIA};

fn bytes(p:&Path)->R<Vec<u8>>{fs::read(p).map_err(|e|e.to_string())}
fn texto(p:&Path)->R<String>{fs::read_to_string(p).map_err(|e|e.to_string())}
fn s(v:&Value,k:&str)->R<String>{v[k].as_str().map(str::to_owned).ok_or(format!("Falta {k}"))}
fn archivos(root:&Path,p:&Path,set:&mut HashSet<String>)->R<()>{for e in fs::read_dir(p).map_err(|e|e.to_string())?{let e=e.map_err(|e|e.to_string())?;let t=e.file_type().map_err(|e|e.to_string())?;need(!t.is_symlink(),"Enlace físico no permitido")?;if t.is_dir(){archivos(root,&e.path(),set)?}else{set.insert(e.path().strip_prefix(root).map_err(|e|e.to_string())?.to_string_lossy().replace('\\',"/"));}}Ok(())}
fn enlaces(ruta:&str,t:&str,rutas:&HashSet<String>)->R<usize>{let mut n=0;for part in t.split("](").skip(1){let fin=part.find(')').ok_or("Enlace incompleto")?;let raw=&part[..fin];if raw.starts_with("https://")||raw.starts_with("http://")||raw.starts_with('#'){continue}let target=raw.split('#').next().unwrap();let mut v:Vec<&str>=ruta.split('/').collect();v.pop();for x in target.split('/'){match x{""|"."=>(),".."=>{need(v.pop().is_some(),"Enlace fuera de sede")?},_=>v.push(x)}}let path=v.join("/");need(rutas.contains(&path)||(target.ends_with('/')&&rutas.iter().any(|x|x.starts_with(&(path.clone()+"/")))),&format!("Destino ausente: {ruta} → {path}"))?;n+=1;}Ok(n)}
fn run()->R<()>{
    let a:Vec<_>=std::env::args().collect();need(a.len()==7,"ORIGINALES PREPARADOS CAMBIOS ADJUDICACION PUNTUACION INFORME")?;
    let old=Path::new(&a[1]);let new=Path::new(&a[2]);let cambios=parse(&bytes(Path::new(&a[3]))?)?;
    let base="laboratorio/ensayo-ia-y-observabilidad";
    let mut rutas:HashSet<String>=cambios["rutas_base"].as_array().ok_or("Inventario de rutas")?.iter().map(|x|x.as_str().unwrap_or("").to_owned()).collect();archivos(new,new,&mut rutas)?;let mut links=0;
    for documento in cambios["documentos"].as_array().ok_or("Documentos")?{
        let ruta=s(documento,"ruta")?;let mut esperado=texto(&old.join(&ruta))?;
        for op in documento["operaciones"].as_array().ok_or("Operaciones")?{
            let antes=s(op,"antes")?;let despues=s(op,"despues")?;
            need(!antes.is_empty()&&esperado.matches(&antes).count()==1,"Ancla ausente o ambigua")?;
            need(!antes.contains("```")&&!antes.contains("<a id="),"Cambio en diagrama o ancla histórica")?;
            links+=enlaces(&ruta,&despues,&rutas)?;
            esperado=esperado.replacen(&antes,&despues,1);
        }
        need(esperado==texto(&new.join(&ruta))?,"Cambio fuera de las incorporaciones delimitadas")?;
    }
    let matrix=format!("{base}/modelos-de-ia/acoplamientos-con-el-sv/02-plataforma-gestionada/antrophic/claude-opus-5-5/ensayo-cyb09-20261010/entrega/MATRIZ-RECEPCION-NODO02.md");links+=enlaces(&matrix,&texto(&new.join(&matrix))?,&rutas)?;
    let ruta=format!("{base}/VERSIONES.json");let x=parse(&bytes(&old.join(&ruta))?)?;let y=parse(&bytes(&new.join(&ruta))?)?;
    let alterables=["edicion_documental","fecha_corte","commit_base_consultado","revision_actual","candidatos","componentes"];
    for(k,v)in x.as_object().ok_or("Registro anterior")?{if !alterables.contains(&k.as_str()){need(y[k]==*v,"Antecedente estructurado alterado")?;}}
    need(y["revision_2_30_conservada"]==x["revision_actual"]&&y["edicion_documental"]=="2.31","Edición o antecedente discordante")?;
    for k in ["candidatos","componentes"]{let aa=x[k].as_array().ok_or("Colección anterior")?;let bb=y[k].as_array().ok_or("Colección vigente")?;need(bb.len()==aa.len()+1&&bb[..aa.len()]==aa[..],"Colección histórica reescrita")?;}
    let c=y["candidatos"].as_array().unwrap().last().unwrap();need(c["nodo"]=="02"&&c["modelo"]=="anthropic/claude-opus-5-5@default"&&c["recepcion_cientifica_independiente"]=="pendiente"&&c["aceptacion_general_nodo"]==false,"Identidad o recepción de nodo")?;
    let j=parse(&bytes(Path::new(&a[4]))?)?;need(j["conforme"]==true&&j["recepcion_independiente"]=="pendiente"&&j["n"]==9,"Juicio no recibido")?;
    let mut resultados=vec![];
    for r in j["resultados"].as_array().ok_or("Resultados")?{
        let v=r["vector"].as_array().ok_or("Vector")?;need(v.len()==9,"Longitud")?;
        let correctas=v.iter().filter(|x|x.as_str()==Some("0")).count();let errores=v.iter().filter(|x|x.as_str()==Some("1")).count();let u=v.iter().filter(|x|x.as_str()==Some("U")).count();
        need(correctas+errores+u==9&&r["correctas"]==correctas&&r["total"]==9,"Alfabeto o recuento")?;
        let score=100.0*correctas as f64/9.0;
        resultados.push(json!({"etapa":r["etapa"],"correctas":correctas,"errores_criticos":errores,"errores_no_criticos":0,"indeterminaciones":u,"n":9,"puntuacion_sobre_100":score,"clasificacion_auxiliar":if errores>=7{"No apto"}else if correctas>=7{"Apto"}else{"Indeterminado"},"dictamen_con_criticidad":if errores>0{"No apto"}else if u>0{"U"}else{"Apto"}}));
    }
    need(resultados[0]["correctas"]==8&&resultados[1]["correctas"]==9&&resultados[2]["correctas"]==8,"Resultado distinto del conservado")?;
    fs::write(&a[5],serde_json::to_vec_pretty(&json!({"conforme":true,"formula":"100 × (aciertos − errores no críticos) / N","n_fijado":9,"criticas":9,"etapa_final":"R2","autoridad":"adjudicación asistida exterior al candidato","recepcion_independiente":"pendiente","resultados":resultados,"licencia":LICENCIA})).unwrap()).map_err(|e|e.to_string())?;
    fs::write(&a[6],serde_json::to_vec_pretty(&json!({"conforme":true,"edicion_documental":"2.31","documentos_con_cambios_delimitados":cambios["documentos"].as_array().unwrap().len(),"enlaces_relativos_cotejados":links,"antecedentes_estructurados_preservados":true,"diagramas_y_anclas_historicos_preservados":true,"nodos":"01 → 02 → 03, orden conservado","resultado_final":"88,89/100; No apto por C02 crítico, juicio asistido","recepcion_independiente":"pendiente","inferencias":0,"licencia":LICENCIA})).unwrap()).map_err(|e|e.to_string())?;
    println!("Integración documental y puntuación cotejadas en Rust; arquitectura e historia conservadas");Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("NO CONFORME: {e}");std::process::exit(1)}}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
