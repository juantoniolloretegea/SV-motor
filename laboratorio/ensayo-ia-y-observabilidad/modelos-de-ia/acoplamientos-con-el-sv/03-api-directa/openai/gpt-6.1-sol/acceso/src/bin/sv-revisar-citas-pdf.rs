//! Revisión documentada de representaciones no tipadas por el banco. Sin inferencia.
#![forbid(unsafe_code)]
#[path="../catalogo/estricto.rs"]mod estricto;
use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs::{self,OpenOptions},io::Write,path::Path};
type R<T>=Result<T,String>;
const ROOT:&str="./ejecucion/astra-pdf-banco-20261007-r2";
fn e(v:impl std::fmt::Display)->String{v.to_string()}
fn check(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn parse(b:&[u8])->R<Value>{estricto::parse(b).map_err(e)}
fn load(p:&Path)->R<Value>{parse(&fs::read(p).map_err(e)?)}
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn norm(s:&str)->String{s.split_whitespace().collect::<Vec<_>>().join(" ")}
fn quote(text:&str,q:&str)->R<Value>{
    let text=norm(text);let parts=q.split("[...]").map(norm).collect::<Vec<_>>();check(parts.iter().all(|s|!s.is_empty()),"Cita vacía o elipsis sin extremos")?;
    let mut cursor=0;let mut spans=vec![];
    for s in &parts{let relative=text[cursor..].find(s.as_str()).ok_or("Segmento de cita no literal o desordenado")?;let start=cursor+relative;cursor=start+s.len();spans.push(json!({"inicio_utf8_texto_normalizado":start,"fin_utf8_texto_normalizado":cursor,"texto":s}));}
    Ok(json!({"conforme":true,"elipsis_explicitas":parts.len()-1,"segmentos":spans,"normalizacion":"sólo espacios; omisiones [...] comprobadas por segmentos literales ordenados; no semejanza"}))
}
fn audit(a:&Value,r:&Value)->R<Value>{
    for k in ["respuesta","fundamentos_verificables","evidencias","insuficiencias"]{check(a.get(k).is_some(),"Campo ausente")?;}
    let i=parse(r["input"][0]["content"].as_str().ok_or("Contenido")?.as_bytes())?;let f=i["fragmentos_documentales_completos"].as_array().ok_or("Fragmentos")?;
    let mut receipts=vec![];
    for q in a["evidencias"].as_array().filter(|a|!a.is_empty()).ok_or("Evidencias")?{
        let p=q["pagina_fisica"].as_u64().ok_or("Página")?;check(p>0&&i["paginas_fisicas"].as_array().ok_or("Páginas")?.contains(&json!(p))&&q["documento"]=="LLS-HCL-2018","Página o documento ajeno")?;
        let expected=format!("PDF-P{:04}",p-1);let section=q["seccion"].as_str().ok_or("Sección")?;
        let mut full=String::new();for t in f.iter().filter(|f|f["pagina_pdf_ordinal"]==p){full.push_str(t["texto"].as_str().ok_or("Texto")?);}
        let title=if section==expected{None}else{let title=section.strip_prefix(&format!("{expected} — ")).ok_or("Identificador de sección distinto")?;check(!title.is_empty()&&norm(&full).contains(&norm(title)),"Título añadido no pertenece a la página")?;Some(title)};
        let mut last=None;let mut end=None;let mut text=String::new();let mut refs=vec![];
        for r in q["fragmentos"].as_array().filter(|a|!a.is_empty()).ok_or("Localizadores")?{
            let m=f.iter().filter(|f|f["pagina_pdf_ordinal"]==p&&if r.is_u64(){f["pagina"]==*r}else{r.is_object()&&r["inicio_caracter"].is_u64()&&r["fin_caracter_exclusivo"].is_u64()&&r["inicio_caracter"]==f["inicio_caracter"]&&r["fin_caracter_exclusivo"]==f["fin_caracter_exclusivo"]}).collect::<Vec<_>>();
            check(m.len()==1,"Fragmento no identificado exactamente")?;let z=m[0];let id=z["pagina"].as_u64().ok_or("Índice")?;
            check(last.is_none_or(|n|id==n+1)&&end.as_ref().is_none_or(|v|*v==z["inicio_caracter"]),"Fragmentos discontinuos o duplicados")?;
            last=Some(id);end=Some(z["fin_caracter_exclusivo"].clone());text.push_str(z["texto"].as_str().ok_or("Texto")?);refs.push(json!({"indice":id,"inicio_caracter":z["inicio_caracter"],"fin_caracter_exclusivo":z["fin_caracter_exclusivo"]}));
        }
        let quotes=match &q["cita_literal_breve"]{Value::String(s)=>vec![s.as_str()],Value::Array(v)=>v.iter().map(|x|x.as_str().ok_or("Cita no textual".into())).collect::<R<Vec<_>>>()?,_=>return Err("Citas ausentes".into())};check(!quotes.is_empty(),"Sin citas")?;
        let checks=quotes.iter().map(|s|quote(&text,s)).collect::<R<Vec<_>>>()?;
        receipts.push(json!({"pagina_fisica":p,"seccion_recibida":section,"identificador_cotejado":expected,"titulo_documental_cotejado":title,"fragmentos":refs,"citas":checks,"lista_de_citas":q["cita_literal_breve"].is_array()}));
    }
    Ok(json!({"conforme":true,"citas":receipts,"adjudicacion_sustantiva":false}))
}
fn run()->R<Value>{let base=Path::new(ROOT);let bank=load(&base.join("COTEJO-BANCO-RUST.json"))?;check(bank["conforme"]==true,"Falta cotejo original")?;let mut cases=vec![];
    for n in 1..=9 {let id=format!("PDF{n:02}");let p=base.join("originales").join(&id);let b=fs::read(p.join("FINAL.txt")).map_err(e)?;
        let request=load(&p.join("SOLICITUD.json"))?;
        let review=match parse(&b).and_then(|a|audit(&a,&request)){Ok(v)=>v,Err(e)=>json!({"conforme":false,"defecto":e})};cases.push(json!({"caso":id,"original_sha256":sha(&b),"original_auditoria":load(&p.join("AUDITORIA-FORMAL.json"))?,"revision":review}));}
    let v=json!({"version":"0.3.0","naturaleza":"revisión posterior de representación documental; sin modificar solicitudes, respuestas ni criterio semántico","fundamento":"El banco no exige una cadena escalar ni prohíbe títulos documentales añadidos o elipsis explícitas. Se coteja cada segmento literal y localizador exacto; no se permite completar citas por parecido.","casos":cases,"conforme":cases.iter().all(|c|c["revision"]["conforme"]==true),"inferencias_nuevas":0,"fecha_revision_ms":sv_instrumentacion::utc_ms(),"ejecutable_sha256":sha(&fs::read(std::env::current_exe().map_err(e)?).map_err(e)?)});
    let mut f=OpenOptions::new().write(true).create_new(true).open(base.join("REVISION-CITAS-RUST.json")).map_err(e)?;f.write_all(&serde_json::to_vec_pretty(&v).map_err(e)?).and_then(|_|f.sync_all()).map_err(e)?;Ok(v)}
fn main(){match run(){Ok(v)=>{println!("Conformidad documental: {}",v["conforme"]);for c in v["casos"].as_array().unwrap(){println!("{}: {} {}",c["caso"],c["revision"]["conforme"],c["revision"]["defecto"]);}},Err(e)=>{eprintln!("{e}");std::process::exit(1)}}}
#[cfg(test)]mod tests{use super::*;
#[test]fn elipsis_ordenadas(){quote("Uno. Dos. Tres.","Uno. [...] Tres.").unwrap();assert!(quote("Uno. Dos. Tres.","Tres. [...] Uno.").is_err());}
#[test]fn sin_semejanza_ni_extremos_vacios(){assert!(quote("Uno dos.","Uno tres.").is_err());assert!(quote("Uno dos.","Uno [...]").is_err());assert!(quote("Uno dos.","[...]").is_err());}
#[test]fn espacios_no_palabras(){quote("Uno\n dos.","Uno dos.").unwrap();assert!(quote("Unodos.","Uno dos.").is_err());}
}
