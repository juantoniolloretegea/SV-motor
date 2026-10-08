use std::{collections::{BTreeMap,BTreeSet},env,fs,path::Path};
use serde_json::{Value,json};
fn n(v:&Value,k:&str)->u64{v[k].as_u64().expect(k)}
fn decimal(s:&str)->u64{let mut it=s.split('.');let a:u64=it.next().unwrap().parse().unwrap();let b=it.next().unwrap_or("");assert!(it.next().is_none()&&b.len()<=10);a*10_000_000_000+if b.is_empty(){0}else{b.parse::<u64>().unwrap()*10u64.pow(10-b.len() as u32)}}
fn money(n:u64)->String{format!("{}.{:010}",n/10_000_000_000,n%10_000_000_000)}
fn sum(rs:&[&Value],k:&str)->u64{rs.iter().filter_map(|v|v[k].as_u64()).sum()}
fn summarize(rs:&[&Value])->Value{
 let complete=rs.iter().filter(|r|r["completa"]==true).count();
 let mut times:Vec<_>=rs.iter().filter(|r|r["completa"]==true).map(|r|n(r,"duracion_operacion_ms")).collect();times.sort();
 let cost=|k:&str|{let vs:Vec<_>=rs.iter().filter_map(|r|r[k].as_str()).collect();if vs.is_empty(){Value::Null}else{json!(money(vs.iter().map(|v|decimal(v)).sum()))}};
 json!({"solicitudes":rs.len(),"entregas_completas":complete,"incompletas":rs.len()-complete,
 "entrada":sum(rs,"entrada_tokens"),"salida":sum(rs,"salida_tokens"),"total":sum(rs,"total_tokens"),"cache_incluida":sum(rs,"cache_incluida"),
 "uso_desconocido":rs.iter().filter(|r|r["total_tokens"].is_null()).count(),
 "suma_operaciones_ms":sum(rs,"duracion_operacion_ms"),
 "mediana_inferior_completas_ms":times[(times.len()-1)/2],
 "p95_completas_ms":times[(95*times.len()).div_ceil(100)-1],
 "importe_comunicado_usd":cost("importe_comunicado_usd"),"coste_estimado_usd":cost("coste_estimado_usd")})
}
fn main(){
 let a:Vec<_>=env::args().collect();let root=Path::new(a.get(1).map(String::as_str).unwrap_or("."));
 let source:Value=serde_json::from_slice(&fs::read(root.join("evidencia-sv/ENSAYOS-POR-SOLICITUD.json")).unwrap()).unwrap();
 let rows=source["filas"].as_array().unwrap();assert_eq!(rows.len(),297);
 let mut ids=BTreeSet::new();let mut groups:BTreeMap<(String,String),Vec<&Value>>=BTreeMap::new();
 for r in rows{
  assert!(ids.insert(r["id"].as_str().unwrap()));
  assert!(r["fuente"].as_str().unwrap().starts_with("https://github.com/juantoniolloretegea/SV-motor/blob/"));
  assert!(["R0","R1","R2"].contains(&r["etapa"].as_str().unwrap()));
  assert!(n(r,"duracion_operacion_ms")>0);
  if r["completa"]==true{assert_eq!(n(r,"entrada_tokens")+n(r,"salida_tokens"),n(r,"total_tokens"));assert!(n(r,"cache_incluida")<=n(r,"entrada_tokens"));}
  else{for k in ["entrada_tokens","salida_tokens","total_tokens","cache_incluida"]{assert!(r[k].is_null());}}
  if let Some(t)=r["primer_texto_ms"].as_u64(){assert!(t<=n(r,"duracion_operacion_ms"));}
  if r["modelo"]=="grok-4.7"{let c=decimal(r["importe_comunicado_usd"].as_str().unwrap());assert_eq!(c,n(r,"coste_ticks_proveedor"));assert_eq!(c,(n(r,"entrada_tokens")-n(r,"cache_incluida"))*20_000+n(r,"cache_incluida")*5_000+n(r,"salida_tokens")*60_000);}
  else{assert!(r["importe_comunicado_usd"].is_null());}
  if r["modelo"]=="glm-5.3"{assert_eq!(decimal(r["coste_estimado_usd"].as_str().unwrap()),(n(r,"entrada_tokens")-n(r,"cache_incluida"))*14_000+n(r,"cache_incluida")*2_600+n(r,"salida_tokens")*44_000);}
  else{assert!(r["coste_estimado_usd"].is_null());}
  groups.entry((r["campana"].as_str().unwrap().to_string(),r["modelo"].as_str().unwrap().to_string())).or_default().push(r);
 }
 assert_eq!(groups.len(),5);
 let mut exams=Vec::new();
 for ((campaign,model),rs) in &groups{
  let questions=if campaign=="PDQ25"{25}else{16};let prefix=if campaign=="CYB16"{"C"}else{"P"};
  let mut pairs=BTreeSet::new();for r in rs.iter().filter(|r|r["completa"]==true){assert!(pairs.insert((r["caso"].as_str().unwrap().to_string(),r["etapa"].as_str().unwrap().to_string())));}
  for i in 1..=questions{for s in ["R0","R1","R2"]{assert!(pairs.contains(&(format!("{prefix}{i:02}"),s.to_string())));}}
  assert_eq!(pairs.len(),questions*3);
  let stages:Vec<_>=["R0","R1","R2"].iter().map(|stage|{let subset:Vec<_>=rs.iter().copied().filter(|r|r["etapa"]==*stage).collect();json!({"etapa":stage,"medidas":summarize(&subset)})}).collect();
  exams.push(json!({"prueba":campaign,"modelo":model,"medidas":summarize(rs),"etapas":stages}));
 }
 let mut subset=Vec::new();
 for ((campaign,model),rs) in &groups{
  if campaign.starts_with("PDQ"){
   let common:Vec<_>=rs.iter().copied().filter(|r|r["caso"].as_str().unwrap()[1..].parse::<u32>().unwrap()<=16).collect();
   subset.push(json!({"modelo":model,"medidas":summarize(&common)}));
  }
 }
 let ranking:Value=serde_json::from_slice(&fs::read(root.join("RANQUIN-CALIDAD-SV.json")).unwrap()).unwrap();
 for comp in ranking["comparaciones"].as_array().unwrap(){
  let size=n(comp,"preguntas") as usize;let fs=comp["filas"].as_array().unwrap();let mut seen=BTreeSet::new();
  for f in fs{
   assert!(seen.insert((f["modelo"].as_str().unwrap(),f["etapa"].as_str().unwrap())));
   let vs=f["vector"].as_array().unwrap();let critical=f["criticidades"].as_array().unwrap();assert_eq!(vs.len(),size);assert_eq!(critical.len(),size);
   assert!(vs.iter().all(|v|["0","1","U"].contains(&v.as_str().unwrap())));
   assert_eq!(n(f,"conformes"),vs.iter().filter(|v|**v=="0").count() as u64);
   assert_eq!(n(f,"indeterminadas"),vs.iter().filter(|v|**v=="U").count() as u64);
   assert_eq!(n(f,"incumplimientos_criticos"),vs.iter().zip(critical).filter(|(v,c)|**v=="1"&&**c==true).count() as u64);
   let place=1+fs.iter().filter(|x|x["etapa"]==f["etapa"]&&n(x,"conformes")>n(f,"conformes")).count() as u64;assert_eq!(n(f,"puesto"),place);
  }
 }
 let out=json!({"edicion":"1.4","fecha":"2026-10-08","resultado":"CONFORME para recuentos y clasificación contractual publicada",
 "examenes":exams,"subconjunto_pdq16":subset,"licencia":source["licencia"],
 "alcance":"Reproducción de observaciones y adjudicaciones publicadas; no nueva evaluación semántica ni certificación de facturas."});
 fs::write(root.join("evidencia-sv/ENSAYOS-RESUMEN.json"),format!("{}\n",serde_json::to_string_pretty(&out).unwrap())).unwrap();
 println!("CONFORME: 297 intentos, cinco exámenes, subconjunto PDQ16 y 21 filas de clasificación.");
}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
