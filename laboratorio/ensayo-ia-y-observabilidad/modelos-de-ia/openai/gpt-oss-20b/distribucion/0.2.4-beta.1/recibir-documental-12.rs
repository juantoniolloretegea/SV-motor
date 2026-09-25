use std::{env,fs,path::PathBuf,collections::{BTreeMap,BTreeSet}};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn rows(p:&std::path::Path)->Result<Vec<Value>,Box<dyn std::error::Error>>{let s=fs::read_to_string(p)?;if !s.is_empty()&&!s.ends_with('\n'){return Err("Registro incompleto".into())}Ok(s.lines().map(serde_json::from_str).collect::<Result<_,_>>()?)}
fn main()->Result<(),Box<dyn std::error::Error>>{
 let a:Vec<_>=env::args().collect();let dir=PathBuf::from(&a[1]);let baseline=PathBuf::from(&a[2]);
 let ib=fs::read(dir.join("ENTRADAS_DOC05_DOC12.json"))?;let input:Value=serde_json::from_slice(&ib)?;
 let specs:BTreeMap<String,Value>=input["cases"].as_array().unwrap().iter().map(|v|(v["id"].as_str().unwrap().to_string(),v.clone())).collect();
 assert_eq!(specs.len(),8);
 let mut seen=BTreeSet::new();let mut result=Vec::new();
 for(is_new,path)in[(false,baseline.clone()),(true,dir.join("ejecucion-01/RESULTADOS.jsonl"))]{
  for r in rows(&path)?{
   let id=r["case"].as_str().ok_or("Falta identificador")?;assert!(seen.insert(id.to_string()),"Duplicado");
   let expected=if is_new{specs.get(id).ok_or("Condición nueva desconocida")?["expected"].clone()}
   else{assert!(["DOC01","DOC02","DOC03","DOC04"].contains(&id));if id=="DOC01"||id=="DOC04"{json!({"estado":"documentado","fuente":"OP-IMM-001-P10@1.0","cita":"Compara un recuento absoluto de neutrófilos válido con el intervalo de referencia aplicable."})}else{json!({"estado":"sin_respaldo","fuente":"","cita":""})}};
   let mut request_hash=Value::Null;
   if is_new{
    let pb=fs::read(dir.join(format!("ejecucion-01/{id}-PETICION.json")))?;let p:Value=serde_json::from_slice(&pb)?;
    assert_eq!(&p["original"],specs.get(id).unwrap());
    let spec=&specs[id];let prompt=format!("<|start|>system<|message|>{}<|end|><|start|>user<|message|>{}<|end|><|start|>assistant<|channel|>final<|message|>",spec["system"].as_str().unwrap(),spec["user"].as_str().unwrap());
    assert_eq!(p["request"]["prompt"],prompt);assert_eq!(p["request"]["max_tokens"],192);assert_eq!(p["request"]["temperature"],0);
    assert!(p["ids"].as_array().unwrap().len()+192<=4096);
    if !r["response"].is_null(){assert_eq!(r["input_tokens"],p["ids"].as_array().unwrap().len());assert_eq!(r["response"]["usage"]["prompt_tokens"],r["input_tokens"]);}
    request_hash=json!(sha(&pb));
   }
   let raw=r["response"]["choices"][0]["text"].as_str().unwrap_or("");
   let parsed=serde_json::from_str::<Value>(raw).ok();
   let exact=parsed.as_ref().is_some_and(|p|p==&expected);
   let object=parsed.as_ref().and_then(Value::as_object);
   let format_ok=object.is_some_and(|o|o.len()==3&&["estado","fuente","cita"].iter().all(|k|o.get(*k).is_some_and(Value::is_string)));
   let field=|name:&str|parsed.as_ref().is_some_and(|p|p[name]==expected[name]);
   let normal=r["response"]["choices"][0]["finish_reason"]=="stop"&&r["error"].is_null();
   let accepted=exact&&normal;
   if !r["accepted"].is_null(){assert_eq!(r["accepted"],accepted,"Discordancia con el ejecutor");assert_eq!(r["expected"],expected);}
   result.push(json!({"id":id,"origin":if is_new{"ampliacion"}else{"antecedente"},"answer":raw,"expected":expected,"decision":field("estado"),"source":field("fuente"),"literal_quote":field("cita"),"strict_format":format_ok,"normal_finish":normal,"accepted":accepted,"input_tokens":r["input_tokens"],"output_tokens":r["response"]["usage"]["completion_tokens"],"seconds":r["seconds"],"error":r["error"],"request_sha256":request_hash}));
  }
 }
 for id in ["DOC01","DOC02","DOC03","DOC04"]{assert!(seen.contains(id));}
 let new:Vec<_>=result.iter().filter(|v|v["origin"]=="ampliacion").collect();
 let missing:Vec<_>=specs.keys().filter(|id|!seen.contains(*id)).collect();
 let ob=fs::read(dir.join("OBSERVADOR_CORTE/trazas.jsonl"))?;let observations=rows(&dir.join("OBSERVADOR_CORTE/trazas.jsonl"))?;let observation=json!({"records":observations.len(),"bytes":ob.len(),"sha256":sha(&ob),"scope":"Copia de observación acumulada de la instancia; no son muestras exclusivamente atribuibles a las ocho peticiones."});
 let report=json!({"observation":observation,"schema":"SV-DOC-12-AUDITORIA-1","source_commit":input["source"]["source_commit"],"inputs_sha256":sha(&ib),"baseline_sha256":sha(&fs::read(&baseline)?),"new_results_sha256":sha(&fs::read(dir.join("ejecucion-01/RESULTADOS.jsonl"))?),"received":result.len(),"new_received":new.len(),"accepted":result.iter().filter(|v|v["accepted"]==true).count(),"new_accepted":new.iter().filter(|v|v["accepted"]==true).count(),"not_executed":missing,"conditions":result,"scope":"Cuatro antecedentes y ocho condiciones previstas. Cinco parámetros de un corte histórico; no suficiencia clínica ni validación del chat libre."});
 fs::write(dir.join("AUDITORIA_RUST.json"),serde_json::to_vec_pretty(&report)?)?;
 println!("Recibidas={} nuevas={} conformes={} nuevas_conformes={}",report["received"],report["new_received"],report["accepted"],report["new_accepted"]);
 for r in report["conditions"].as_array().unwrap(){println!("{} {} entrada={} salida={} tiempo={}s {}",r["id"],r["accepted"],r["input_tokens"],r["output_tokens"],r["seconds"],r["answer"]);}
 Ok(())
}
