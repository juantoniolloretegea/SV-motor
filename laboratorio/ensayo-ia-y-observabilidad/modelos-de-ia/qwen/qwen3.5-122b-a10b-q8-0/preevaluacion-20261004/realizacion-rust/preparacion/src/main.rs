use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use sha1::Sha1;
use std::{fs,path::Path};
type E=Box<dyn std::error::Error>;
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn save(p:&Path,v:&Value)->Result<(),E>{fs::write(p,serde_json::to_vec_pretty(v)?)?;Ok(())}
fn text<'a>(v:&'a Value,k:&str)->Result<&'a str,E>{v[k].as_str().filter(|s|!s.trim().is_empty()).ok_or_else(||format!("Campo textual {k}").into())}
fn check(v:&Value,doc:&str,section:&str,pages:&[String],case:&str,layer:u64)->Result<(),E>{
 let o=v.as_object().ok_or("Se exige objeto")?;
 let fields=["decision","reglas","evidencias","justificacion_breve","recepcion_documental","revision"];
 if o.len()!=fields.len()||fields.iter().any(|k|!o.contains_key(*k)){return Err("Campos finales no conformes".into())}
 if !["RESPALDADA","CONTRADICHA","EVIDENCIA_INSUFICIENTE"].contains(&text(v,"decision")?){return Err("Decisión".into())}
 let r=v["reglas"].as_array().ok_or("Reglas")?;
 if r.is_empty()||r.iter().any(|r|!matches!(r.as_str(),Some("D1"|"D2"|"D3"|"D4"|"D5"))){return Err("Reglas".into())}
 text(v,"justificacion_breve")?;
 let ev=v["evidencias"].as_array().ok_or("Evidencias")?;if ev.is_empty(){return Err("Sin evidencias".into())}
 for e in ev {
  if e.as_object().map(|o|o.len())!=Some(4)||e["documento"]!=doc||e["seccion"]!=section{return Err("Localizador".into())}
  let p=e["pagina"].as_u64().ok_or("Página")? as usize;
  if !pages.get(p).is_some_and(|t|t.contains(text(e,"fragmento").unwrap_or("\0"))){return Err("Cita no literal o página inexistente".into())}
 }
 let rec=v["recepcion_documental"].as_array().ok_or("Recepción")?;
 if rec.len()!=1||rec[0].as_object().map(|o|o.len())!=Some(3)||rec[0]["documento"]!=doc||rec[0]["seccion"]!=section||rec[0]["paginas"]!=json!((0..pages.len()).collect::<Vec<_>>()){return Err("Recepción incompleta o inexistente".into())}
 if layer==0 {if !v["revision"].is_null(){return Err("Revisión inicial".into())}}
 else {
  let rev=&v["revision"];
  if rev.as_object().map(|o|o.len())!=Some(3){return Err("Campos revisión".into())}
  let prior=rev["antecedentes"].as_array().ok_or("Antecedentes")?;
  if prior.len()!=layer as usize||prior.iter().enumerate().any(|(i,a)|a!=&json!({"caso":case,"capa":i})){return Err("Antecedentes".into())}
  let adv=rev["adversarial"].as_array().ok_or("Adversarial")?;
  if adv.is_empty(){return Err("Adversarial vacía".into())}
  for a in adv {if a.as_object().map(|o|o.len())!=Some(3){return Err("Campos adversarial".into())}text(a,"objecion")?;text(a,"contraste_documental")?;if !["mantener","corregir","retirar","indeterminar"].contains(&text(a,"conclusion")?){return Err("Conclusión".into())}}
  text(rev,"fundamento_del_cambio_o_mantenimiento")?;
 }
 Ok(())
}
fn obj(properties:Value)->Value {let required:Vec<_>=properties.as_object().unwrap().keys().cloned().collect();json!({"type":"object","properties":properties,"required":required,"additionalProperties":false})}
fn schema()->Value{
 let s=json!({"type":"string","minLength":1});let n=json!({"type":"integer","minimum":0});
 let evidence=obj(json!({"documento":s,"seccion":s,"pagina":n,"fragmento":s}));
 let reception=obj(json!({"documento":s,"seccion":s,"paginas":{"type":"array","minItems":1,"uniqueItems":true,"items":n}}));
 let rev=obj(json!({"antecedentes":{"type":"array","minItems":1,"items":obj(json!({"caso":s,"capa":n}))},"adversarial":{"type":"array","minItems":1,"items":obj(json!({"objecion":s,"contraste_documental":s,"conclusion":{"enum":["mantener","corregir","retirar","indeterminar"]}}))},"fundamento_del_cambio_o_mantenimiento":s}));
 let mut result=obj(json!({"decision":{"enum":["RESPALDADA","CONTRADICHA","EVIDENCIA_INSUFICIENTE"]},"reglas":{"type":"array","minItems":1,"items":{"enum":["D1","D2","D3","D4","D5"]}},"evidencias":{"type":"array","minItems":1,"items":evidence},"justificacion_breve":s,"recepcion_documental":{"type":"array","minItems":1,"items":reception},"revision":{"anyOf":[{"type":"null"},rev]}}));
 result["$schema"]=json!("https://json-schema.org/draft/2020-12/schema");result
}
fn prepare(root:&Path)->Result<(),E>{
 let src=root.join("preparacion/fuentes");let out=root.join("preparacion/protocolo");fs::create_dir_all(&out)?;
 let list:Value=serde_json::from_slice(&fs::read(root.join("preparacion/FUENTES-RECUPERADAS.json"))?)?;let mut checked=vec![];
 for item in list.as_array().ok_or("Lista de fuentes")?{let name=text(item,"ruta")?;let bytes=fs::read(src.join(name))?;if bytes.len() as u64!=item["bytes"].as_u64().ok_or("Tamaño")?{return Err("Tamaño de fuente".into())}if let Some(oid)=item["objeto_git"].as_str(){let mut h=Sha1::new();h.update(format!("blob {}\0",bytes.len()).as_bytes());h.update(&bytes);if format!("{:x}",h.finalize())!=oid{return Err("Objeto Git no coincidente".into())}}checked.push(json!({"ruta":name,"bytes":bytes.len(),"sha256":hash(&bytes),"objeto_git":item["objeto_git"]}));}
 let enc=fs::read(src.join("ENCARGO-REMOTO.md"))?;let local=fs::read(root.join("../../encargos/qwen-preeevaluacion-20261004/ENCARGO.md")).or_else(|_|fs::read(root.join("../../encargos/qwen-preevaluacion-20261004/ENCARGO.md")))?;
 if enc!=local||hash(&enc)!="87e718a484e01e90b34d82598eb089b3d41f792cdf4ce83e977d65653ba55034"{return Err("Encargo no conforme".into())}
 let key=fs::read(root.join("../safeguard-retroalimentacion-20261003/reservado/CLAVE.json"))?;
 if key.len()!=8007||hash(&key)!="0cfefe66e3b27e314414d3e21f39de9c434f710813904e8764e9b76d798b1c8b"{return Err("Clave no conforme".into())}
 save(&out.join("COTEJO-FUENTES-RUST.json"),&json!({"conforme":true,"archivos":checked,"clave":{"bytes":key.len(),"sha256":hash(&key),"copiada_al_candidato":false},"encargo_identico":true}))?;
 let old=fs::read_to_string(src.join("POLITICA.txt"))?;
 let start=old.find("## Definiciones").ok_or("Definiciones")?;let end=old.find("Formato:").ok_or("Formato")?;
 let exstart=old.find("## Ejemplos sintéticos").ok_or("Ejemplos")?;let exend=old.find("Ejemplo de salida").ok_or("Ejemplo final")?;
 let enc=String::from_utf8(enc)?;let examples:Vec<Value>=enc.split("```json\n").skip(1).take(2).map(|s|serde_json::from_str(s.split("```").next().unwrap().trim()).unwrap()).collect();
 if examples.len()!=2{return Err("Ejemplos del encargo")?}
 let page=vec!["El archivo abre de lunes a viernes, de 09:00 a 14:00".to_string()];for (i,e) in examples.iter().enumerate(){check(e,"EJ","S1",&page,"EJ-01",i as u64)?;save(&out.join(format!("EJEMPLO-CAPA-{i}.json")),e)?;}
 let intro=old.split("En el canal de respuesta final,").next().unwrap();
 let contract="En el canal final devuelva un único objeto JSON, sin cercas ni texto exterior. Campos obligatorios: decision, reglas, evidencias, justificacion_breve, recepcion_documental y revision. Las evidencias incluyen documento, seccion, pagina lógica de base cero y fragmento literal. recepcion_documental contiene documento, seccion y paginas recibidas. revision es null en la capa inicial. En las posteriores contiene antecedentes (caso y capa), adversarial (lista no vacía de objecion, contraste_documental y conclusion) y fundamento_del_cambio_o_mantenimiento. Cada conclusión será mantener, corregir, retirar o indeterminar. El contraste debe argumentar y referirse a las evidencias. Entregue siempre la respuesta revisada completa y una justificación documental verificable. No se solicita revelar procesos internos no observables.\n\n";
 let reading="\nLea todas las páginas suministradas por el Árbitro-Director; no las sustituya por búsqueda selectiva. Las páginas comienzan en 0; siguiente_pagina=null no acredita haber recibido las anteriores. No acceda a Internet ni añada premisas externas. Los antecedentes propios son hipótesis, no fuentes. La recepción declarada se contrasta externamente. No calcule huellas ni su propia puntuación.\n";
 let policy=format!("{intro}{contract}{}{}{}\nEjemplo inicial completo EJ/S1, página 0:\n{}\nEjemplo completo de revisión del mismo supuesto EJ-01:\n{}\n",&old[start..end],&old[exstart..exend],reading,serde_json::to_string(&examples[0])?,serde_json::to_string(&examples[1])?);
 fs::write(out.join("POLITICA-QWEN-r1.txt"),&policy)?;save(&out.join("ESQUEMA-QWEN-r1.json"),&schema())?;
 let mut plan:Value=serde_json::from_slice(&fs::read(src.join("PLAN-GENERAL.json"))?)?;plan["campana"]=json!("QWEN35-PRE-20261004");plan["fuente_original"]=json!("db1395ef883da403fcec86f8b002d0ce86bf42ed");save(&out.join("PLAN-QWEN-r1.json"),&plan)?;
 let sources:Value=serde_json::from_slice(&fs::read(src.join("FUENTES-SINTETICAS.json"))?)?;
 for doc in sources["documents"].as_array().ok_or("Documentos")? {for s in doc["sections"].as_array().ok_or("Secciones")?{if hash(text(s,"text")?.as_bytes())!=text(s,"sha256")?{return Err("Fuente sintética alterada".into())}}}
 let license="© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
 fs::write(out.join("DIFERENCIAS-DE-PRESENTACION.md"),format!("# Adaptación prospectiva de presentación\n\nSe conservan literalmente las definiciones, reglas D1–D5, contradicción e insuficiencia y los seis ejemplos didácticos originales. Se sustituye el formato abreviado y la búsqueda selectiva por un contrato completo y la lectura de todas las páginas; se incorporan los dos ejemplos completos del encargo. Las huellas corresponden al instrumento. La clave permanece en su sede reservada local y no se copia al servidor. El catálogo MCP de cada caso adaptará sólo su metadato URL al identificador sintético admitido; textos y huellas permanecen idénticos. Esta adaptación no modifica resultados históricos ni acredita una causa de error.\n\n{license}\n"))?;
 save(&out.join("PARAMETROS-PREVISTOS.json"),&json!({"enable_thinking":true,"seed":42,"temperature":1.0,"top_p":0.95,"top_k":20,"min_p":0.0,"presence_penalty":1.5,"repetition_penalty":1.0,"max_tokens":4096,"contexto":32768,"reserva":2048,"segundos_solicitud":18000,"segundos_sin_progreso":1800,"segundos_campana_carga_inferencia":86400,"reparaciones_maximas":2,"estado":"Previstos; comprobación efectiva e integración pendientes"}))?;
 println!("Fuentes y clave cotejadas en Rust; política, esquema y ejemplos preparados; sin inferencia.");Ok(())
}
fn manifest(root:&Path,out:&Path)->Result<(),E>{fn visit(base:&Path,p:&Path,rows:&mut Vec<Value>)->Result<(),E>{let mut entries:Vec<_>=fs::read_dir(p)?.collect::<Result<_,_>>()?;entries.sort_by_key(|e|e.file_name());for e in entries{let meta=e.file_type()?;if meta.is_symlink(){return Err("Enlace no admisible".into())}let p=e.path();if meta.is_dir(){visit(base,&p,rows)?}else if meta.is_file(){let b=fs::read(&p)?;rows.push(json!({"ruta":p.strip_prefix(base)?.to_string_lossy().replace('\\',"/"),"bytes":b.len(),"sha256":hash(&b)}));}}Ok(())}let mut rows=vec![];visit(root,root,&mut rows)?;save(out,&json!({"archivos":rows,"realizacion":"Rust","encargo":"QWEN35-PRE-20261004/r1"}))?;println!("Manifiesto conservado: {} archivos",rows.len());Ok(())}
fn main()->Result<(),E>{let args:Vec<_>=std::env::args().collect();match args.get(1).map(String::as_str){Some("preparar")=>prepare(Path::new(args.get(2).ok_or("Raíz requerida")?)),Some("manifestar")=>manifest(Path::new(args.get(2).ok_or("Raíz requerida")?),Path::new(args.get(3).ok_or("Destino")?)),_=>Err("Uso: preparar RAIZ | manifestar RAIZ SALIDA".into())}}
#[cfg(test)]mod tests{
 use super::*;
 fn example()->Value{json!({"decision":"RESPALDADA","reglas":["D1"],"evidencias":[{"documento":"EJ","seccion":"S1","pagina":0,"fragmento":"abre los martes"}],"justificacion_breve":"La fuente fija el día.","recepcion_documental":[{"documento":"EJ","seccion":"S1","paginas":[0]}],"revision":null})}
 fn valid(v:&Value,layer:u64)->bool{check(v,"EJ","S1",&["El archivo abre los martes.".into()],"EJ-01",layer).is_ok()}
 #[test]fn tres_decisiones(){for d in ["RESPALDADA","CONTRADICHA","EVIDENCIA_INSUFICIENTE"]{let mut v=example();v["decision"]=json!(d);assert!(valid(&v,0));}}
 #[test]fn campos_obligatorios(){for k in ["decision","reglas","evidencias","justificacion_breve","recepcion_documental","revision"]{let mut v=example();v.as_object_mut().unwrap().remove(k);assert!(!valid(&v,0));}}
 #[test]fn cita_y_localizadores(){for (k,x) in [("pagina",json!(1)),("fragmento",json!("abre los lunes")),("documento",json!("OTRO")),("seccion",json!("S2"))]{let mut v=example();v["evidencias"][0][k]=x;assert!(!valid(&v,0));}}
 #[test]fn recepcion_real(){let mut v=example();v["recepcion_documental"][0]["paginas"]=json!([0,1]);assert!(!valid(&v,0));}
 #[test]fn revision_y_historial(){let mut v=example();v["revision"]=json!({"antecedentes":[{"caso":"EJ-01","capa":0}],"adversarial":[{"objecion":"Podría ser otro día","contraste_documental":"EJ/S1 página 0 dice martes","conclusion":"mantener"}],"fundamento_del_cambio_o_mantenimiento":"La cita conserva el día"});assert!(valid(&v,1));assert!(!valid(&v,0));assert!(!valid(&v,2));v["revision"]["adversarial"]=json!([]);assert!(!valid(&v,1));}
 #[test]fn esquema_obligatorio(){let s=schema();assert_eq!(s["required"].as_array().unwrap().len(),6);assert_eq!(s["additionalProperties"],false);}
}
