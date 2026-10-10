//! Recepción de la revisión documental exterior al candidato y formación del vector completo.
#![forbid(unsafe_code)]
#[path="../catalogo/estricto.rs"]mod estricto;
use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs::{self,OpenOptions},io::Write,path::Path};
type R<T>=Result<T,String>;
const ROOT:&str="C:/SV-LABORATORIO";
const RUN:&str="ejecucion/astra-pdf-banco-20261007-r2";
const KEY:&str="9e9f31008fcdcf6bb1a05db2286c5bdda88f4cf43d45a7e74dd46757c6dcdbe0";
const PIE:&str="© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
fn e(v:impl std::fmt::Display)->String{v.to_string()}
fn check(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn load(p:&Path)->R<Value>{estricto::parse(&fs::read(p).map_err(e)?).map_err(e)}
fn save(p:&Path,v:&Value)->R<()>{fs::create_dir_all(p.parent().ok_or("Padre")?).map_err(e)?;let mut f=OpenOptions::new().write(true).create_new(true).open(p).map_err(e)?;f.write_all(&serde_json::to_vec_pretty(v).map_err(e)?).and_then(|_|f.sync_all()).map_err(e)}
fn rule(value:&str,formal:bool,content:&str,reason:Option<&str>)->R<()>{
    check(matches!(value,"0"|"1"|"U"),"Fuera de la terna")?;
    if value=="0" {check(formal&&content=="correcto"&&reason.is_none(),"Cero incompatible con revisión")?;}
    if value=="1" {check(reason.is_some_and(|s|!s.trim().is_empty()),"Error sin fundamento de naturaleza")?;}Ok(())
}
fn run()->R<Value>{
    let root=Path::new(ROOT);let base=root.join(RUN);let key=root.join("desarrollo/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/anexo-pdf-20261007/CRITERIOS-EVALUADOR.md");check(sha(&fs::read(key).map_err(e)?)==KEY,"Clave alterada")?;
    let review=load(&base.join("REVISION-SUSTANTIVA.json"))?;check(review["clave_sha256"]==KEY,"Revisión de otra clave")?;
    let bank=load(&base.join("COTEJO-BANCO-RUST.json"))?;let cites=load(&base.join("REVISION-CITAS-RUST.json"))?;let met=load(&base.join("METRICAS-RUST.json"))?;
    for v in [&review,&bank,&cites,&met]{check(v["casos"].as_array().is_some_and(|a|a.len()==9),"Nueve casos requeridos")?;}
    let mut rows=vec![];
    for n in 1..=9 {let id=format!("PDF{n:02}");let p=base.join("originales").join(&id);let ad=&review["casos"][n-1];let cit=&cites["casos"][n-1];let proof=&bank["casos"][n-1];let m=&met["casos"][n-1];let r=load(&p.join("RESULTADO.json"))?;let bytes=fs::read(p.join("FINAL.txt")).map_err(e)?;let h=sha(&bytes);
        for v in [ad,cit,proof,m,&r]{check(v["caso"]==id,"Orden distinto")?;}
        check(r["completa"]==true&&r["telemetria_conforme"]==true&&proof["conforme"]==true&&cit["original_sha256"]==h&&m["final_sha256"]==h,"Fuente o instrumental no conformes")?;
        check(proof["original_resultado_sha256"]==sha(&fs::read(p.join("RESULTADO.json")).map_err(e)?),"Resultado no corresponde al cotejo")?;
        let formal=cit["revision"]["conforme"]==true;let val=ad["valor"].as_str().ok_or("Valor")?;rule(val,formal,ad["contenido"].as_str().ok_or("Contenido")?,ad["naturaleza_defecto"].as_str())?;
        let mut passages=vec![];for q in cit["revision"]["citas"].as_array().into_iter().flatten(){for c in q["citas"].as_array().into_iter().flatten(){for s in c["segmentos"].as_array().into_iter().flatten(){passages.push(json!({"documento":"LLS-HCL-2018","pagina":q["pagina_fisica"].as_u64().ok_or("Página")?-1,"seccion":q["identificador_cotejado"],"fragmento":s["texto"]}));}}}
        let out=json!({"caso":id,"valor":val,"sustantivo":ad["contenido"],"naturaleza_defecto":ad["naturaleza_defecto"],"fundamento_sustantivo":ad["fundamento"],"respuesta_original":String::from_utf8(bytes).map_err(e)?,"formal_original_conforme":r["contrato_formal_conforme"],"formal_conforme":formal,"revision_citas":cit["revision"],"final_sha256":h,"auditoria_sha256":sha(&fs::read(base.join("REVISION-CITAS-RUST.json")).map_err(e)?),"solicitud_sha256":m["solicitud_sha256"],"pasajes_contrastados":passages,"medicion":m,"recepcion_independiente":"pendiente"});
        save(&base.join(format!("hitos/{id}/evaluacion/ADJUDICACION.json")),&out)?;rows.push(out);
    }
    let counts=|s:&str|rows.iter().filter(|r|r["valor"]==s).count();
    let v=json!({"encargo":"ASTRA-ANEXO-PDF-20261007","fase":"anexo PDF, distinto de A0 y examen","casos":rows,"no_adjudicados":0,"medidas":{"terna_completa":true,"correctos":counts("0"),"errores":counts("1"),"indeterminados":counts("U"),"puntuacion":null,"kappa":null,"criticidades":null,"nota":"Distribución descriptiva; no se hereda escala ni criticidad de A0"},"clave_sha256":KEY,"revision_sustantiva_sha256":sha(&fs::read(base.join("REVISION-SUSTANTIVA.json")).map_err(e)?),"cotejo_instrumental":true,"recepcion_independiente":"pendiente","aptitud_clinica":false,"licencia":PIE});
    save(&base.join("adjudicacion/CAPA.json"),&v)?;Ok(v)
}
fn main(){match run(){Ok(v)=>println!("{}",v["medidas"]),Err(e)=>{eprintln!("{e}");std::process::exit(1)}}}
#[cfg(test)]mod tests{use super::*;
#[test]fn fallo_formal_no_recibe_cero(){assert!(rule("0",false,"correcto",None).is_err());}
#[test]fn error_exige_naturaleza(){assert!(rule("1",false,"no evaluado",None).is_err());rule("1",false,"núcleo concordante",Some("formal")).unwrap();}
#[test]fn pendientes_fuera_de_terna(){assert!(rule("NE",true,"correcto",None).is_err());}
}
