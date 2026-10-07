//! Recepción verificable de una revisión documental exterior; no contiene inferencia.
#![forbid(unsafe_code)]
#[path="../catalogo/estricto.rs"]mod estricto;
use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs::{self,OpenOptions},io::Write,path::Path};
type R<T>=Result<T,String>;
const ROOT:&str=".";
const RUN:&str="ejecucion/astra-examen25-r2-20261007";
const KEY:&str="9ffde6937234bed15afa9d154c5cc3f3ca40b1926540fbc12d95aa5e1cbf1dec";
const PIE:&str="© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
fn e(v:impl std::fmt::Display)->String{v.to_string()}
fn need(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn load(p:&Path)->R<Value>{estricto::parse(&fs::read(p).map_err(e)?).map_err(e)}
fn save(p:&Path,v:&Value)->R<()>{if p.exists(){return need(fs::read(p).map_err(e)?==serde_json::to_vec_pretty(v).map_err(e)?,"Derivado previo distinto");}fs::create_dir_all(p.parent().ok_or("Padre")?).map_err(e)?;let mut f=OpenOptions::new().write(true).create_new(true).open(p).map_err(e)?;f.write_all(&serde_json::to_vec_pretty(v).map_err(e)?).and_then(|_|f.sync_all()).map_err(e)}
fn rule(value:&str,formal:bool,content:&str,reason:Option<&str>)->R<()>{need(matches!(value,"0"|"1"|"U"),"Valor no ternario")?;if value=="0"{need(formal&&content=="correcto"&&reason.is_none(),"Cero incompatible")?;}else{need(reason.is_some_and(|s|!s.trim().is_empty()),"Incidencia sin fundamento")?;}Ok(())}
fn dictamen(v:&[String])->R<Value>{
 need(v.len()==25,"Vector incompleto")?;for s in v{need(matches!(s.as_str(),"0"|"1"|"U"),"Fuera de terna")?;}
 let a=v.iter().filter(|s|s.as_str()=="0").count();let b=v.iter().filter(|s|s.as_str()=="1").count();let u=25-a-b;
 let critical=(1..=25).map(|n|n%5!=0).collect::<Vec<_>>();let bc=v.iter().zip(&critical).filter(|(s,c)|s.as_str()=="1"&&**c).count();let uc=v.iter().zip(&critical).filter(|(s,c)|s.as_str()=="U"&&**c).count();
 Ok(json!({"terna_completa":true,"correctos":a,"errores":b,"indeterminados":u,"umbral":19,"kappa":if a>=19{"Apto"}else if b>=19{"No apto"}else{"Indeterminado"},"admision":if bc>0{"No apto"}else if uc>0{"Indeterminada"}else if a>=19{"Apto para el contrato documental"}else{"No acreditada"},"criticidades":critical,"errores_criticos":bc,"indeterminaciones_criticas":uc,"errores_no_criticos":b-bc,"puntuacion":null,"aptitud_clinica":false}))
}
fn norm(s:&str)->String{s.split_whitespace().collect::<Vec<_>>().join(" ")}
fn fuente_limite<'a>(fs:&'a [Value],literal:&str)->Option<&'a Value>{
 for (i,f) in fs.iter().enumerate(){let mut text=String::new();
  for q in &fs[i..]{if q["documento"]!=f["documento"]||q["seccion"]!=f["seccion"]{break;}
   text.push_str(q["texto"].as_str()?);
   if norm(&text).contains(&norm(literal)){return Some(f);}
  }
 }None
}
#[test]fn limite_entre_fragmentos_sin_saltar_pagina(){let f=vec![json!({"documento":"d","pagina_pdf_indice":6,"seccion":"p6","texto":"Aún queda por determinar, en los ensayos clínicos, la dosis y l"}),json!({"documento":"d","pagina_pdf_indice":6,"seccion":"p6","texto":"a duración adecuadas del tratamiento."})];let s="Aún queda por determinar, en los ensayos clínicos, la dosis y la duración adecuadas del tratamiento.";assert!(fuente_limite(&f,s).is_some());let mut g=f.clone();g[1]["seccion"]=json!("otra");assert!(fuente_limite(&g,s).is_none());}
fn run()->R<Value>{let root=Path::new(ROOT);let base=root.join(RUN);let key=base.join("fuentes/CLAVE-CORRECCION.md");need(sha(&fs::read(key).map_err(e)?)==KEY,"Clave distinta")?;
 let review=load(&base.join("REVISION-SUSTANTIVA.json"))?;let proof=load(&base.join("COTEJO-BANCO-RUST.json"))?;let met=load(&base.join("METRICAS-RUST.json"))?;
 need(review["clave_sha256"]==KEY&&proof["conforme"]==true&&met["conforme"]==true,"Recepción previa ausente")?;
 for v in [&review,&proof,&met]{need(v["casos"].as_array().is_some_and(|a|a.len()==75),"Se exigen 75 entregas")?;}
 let mut capas=vec![];
 for stage in 0..3{let mut rows=vec![];
  for n in 1..=25{let i=(n-1)*3+stage;let id=format!("P{n:02}");let p=base.join(format!("originales/{id}/R{stage}"));let ad=&review["casos"][i];let pr=&proof["casos"][i];let m=&met["casos"][i];let r=load(&p.join("RESULTADO.json"))?;let raw=fs::read(p.join("FINAL.txt")).map_err(e)?;let h=sha(&raw);let a=estricto::parse(&raw).unwrap_or(Value::Null);
   need(ad["caso"]==id&&ad["etapa"]==stage&&ad["original_sha256"]==h&&m["final_sha256"]==h&&pr["conforme"]==true&&pr["etapa"]==stage&&pr["caso"]==id&&r["completa"]==true&&r["telemetria_conforme"]==true,"Orden o identidad incorrectos")?;
   need(pr["original_resultado_sha256"]==sha(&fs::read(p.join("RESULTADO.json")).map_err(e)?),"Resultado alterado")?;
   let formal=load(&p.join("AUDITORIA-FORMAL.json"))?["conforme"]==true;let val=ad["valor"].as_str().ok_or("Valor")?;rule(val,formal,ad["contenido"].as_str().ok_or("Contenido")?,ad["naturaleza_defecto"].as_str())?;
   let mut passages=vec![];let mut cites=vec![];
   for q in a["evidencias"].as_array().into_iter().flatten(){passages.push(json!({"documento":q["documento"],"pagina":q["fragmentos"][0].as_u64().ok_or("Fragmento MCP")?,"seccion":q["seccion"],"fragmento":q["cita_literal_breve"]}));cites.push(json!({"citas":[{"conforme":formal,"segmentos":[{"texto":q["cita_literal_breve"]}]}]}));}
   if ad["alerta_limite"]==true{let literal=ad["cita_limite"].as_str().filter(|s|!s.trim().is_empty()).ok_or("Alerta sin evidencia del evaluador")?;let req=load(&p.join("SOLICITUD.json"))?;let supplied=estricto::parse(req["input"][0]["content"].as_str().ok_or("Suministro")?.as_bytes()).map_err(e)?;let fs=supplied["fragmentos_documentales_completos"].as_array().ok_or("Fragmentos")?;let f=fuente_limite(fs,literal).ok_or("Límite no comprobado en fuente")?;passages.push(json!({"documento":f["documento"],"pagina":f["pagina"],"seccion":f["seccion"],"fragmento":literal,"origen":"cotejo adicional del evaluador contra el suministro; no cita atribuida al candidato"}));cites.push(json!({"origen":"cotejo del evaluador","citas":[{"conforme":true,"segmentos":[{"texto":literal}]}]}));}
   let out=json!({"caso":id,"etapa":stage,"valor":val,"critico":n%5!=0,"sustantivo":ad["contenido"],"naturaleza_defecto":ad["naturaleza_defecto"],"fundamento_sustantivo":ad["fundamento"],"respuesta_original":String::from_utf8(raw).map_err(e)?,"estado_documental_candidato":a["estado_documental"],"alerta_limite":ad["alerta_limite"],"formal_conforme":formal,"limite_documental":ad["explicacion_limite"],"observaciones_revision":ad["observaciones_revision"],"revision_citas":{"citas":cites},"final_sha256":h,"auditoria_sha256":sha(&fs::read(p.join("AUDITORIA-FORMAL.json")).map_err(e)?),"solicitud_sha256":m["solicitud_sha256"],"pasajes_contrastados":passages,"medicion":m,"recepcion_independiente":"pendiente"});save(&base.join(format!("hitos/ENTREGA-{:02}/evaluacion/ADJUDICACION.json",i+1)),&out)?;rows.push(out);
  }
  let vector=rows.iter().map(|v|v["valor"].as_str().unwrap().to_string()).collect::<Vec<_>>();let c=json!({"encargo":"ASTRA-EXAMEN25-20261007","fase":format!("R{stage}"),"casos":rows,"no_adjudicados":0,"medidas":dictamen(&vector)?,"vector":vector,"clave_sha256":KEY,"revision_sustantiva_sha256":sha(&fs::read(base.join("REVISION-SUSTANTIVA.json")).map_err(e)?),"cotejo_instrumental":true,"recepcion_independiente":"pendiente","aptitud_clinica":false,"licencia":PIE});save(&base.join(format!("adjudicacion/CAPA-R{stage}.json")),&c)?;capas.push(c);
 }
 let transitions=(0..25).map(|i|json!({"caso":format!("P{:02}",i+1),"R0":capas[0]["vector"][i],"R1":capas[1]["vector"][i],"R2":capas[2]["vector"][i],"estable":capas[0]["vector"][i]==capas[1]["vector"][i]&&capas[1]["vector"][i]==capas[2]["vector"][i]})).collect::<Vec<_>>();
 let out=json!({"conforme":true,"capas":capas.iter().map(|c|json!({"fase":c["fase"],"vector":c["vector"],"medidas":c["medidas"]})).collect::<Vec<_>>(),"transiciones":transitions,"respuesta_final":"R2, sin selección retrospectiva","independencia":"Autorrevisiones del mismo candidato; no evaluadores independientes","causalidad":"Contrato cambiado frente a la prueba anterior; no atribuir efecto aislado a las revisiones","inferencias_nuevas":0,"autoria_licencia":PIE});save(&base.join("COMPARACION-RUST.json"),&out)?;Ok(out)
}
fn main(){match run(){Ok(v)=>println!("{}",v["capas"]),Err(e)=>{eprintln!("{e}");std::process::exit(1)}}}
#[cfg(test)]mod tests{use super::*;#[test]fn veto_y_u(){let mut v=vec!["0".to_string();25];assert_eq!(dictamen(&v).unwrap()["admision"],"Apto para el contrato documental");v[8]="1".into();let d=dictamen(&v).unwrap();assert_eq!(d["kappa"],"Apto");assert_eq!(d["admision"],"No apto");v[8]="U".into();assert_eq!(dictamen(&v).unwrap()["admision"],"Indeterminada");assert!(dictamen(&v[..24]).is_err());}#[test]fn no_encubre_forma(){assert!(rule("0",false,"correcto",None).is_err());assert!(rule("1",false,"correcto",None).is_err());assert!(rule("U",true,"indeterminado",None).is_err());}}

#[test]fn no_criticos_no_imponen_veto(){let mut v=vec!["0".to_string();25];for i in [4,9,14,19,24]{v[i]="1".into();}let d=dictamen(&v).unwrap();assert_eq!(d["admision"],"Apto para el contrato documental");assert_eq!(d["errores_no_criticos"],5);assert_eq!(d["errores_criticos"],0);v[0]="1".into();assert_eq!(dictamen(&v).unwrap()["admision"],"No apto");}