//! Instrumento experimental; no se incorpora al SV ni altera su semántica.
//! Reutiliza exclusivamente CrossFileContradiction de speccheck-core 0.4.1.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use speccheck_core::{parser::{MarkdownParser, SpecParser}, rules::{cross_file::CrossFileContradiction, Rule}, Section};
use std::{collections::{BTreeMap, BTreeSet}, error::Error, fs::{self, OpenOptions}, io::Write, path::{Path, Component}, time::{Instant, SystemTime, UNIX_EPOCH}};
type R<T> = Result<T, Box<dyn Error>>;
fn hash(b:&[u8])->String {format!("{:x}",Sha256::digest(b))}
fn read(p:impl AsRef<Path>)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn save(p:impl AsRef<Path>,v:&Value)->R<()> {let mut f=OpenOptions::new().write(true).create_new(true).open(p)?; f.write_all(&serde_json::to_vec_pretty(v)?)?; Ok(())}
fn arr(v:&Value)->R<&Vec<Value>>{v.as_array().ok_or_else(||"Se esperaba una lista".into())}
fn s(v:&Value)->R<&str>{v.as_str().ok_or_else(||"Se esperaba texto".into())}
fn path(root:&Path,name:&str)->R<std::path::PathBuf>{
 if name.is_empty() || !Path::new(name).components().all(|c|matches!(c,Component::Normal(_))){return Err("Ruta ajena o ascendente".into())}
 let p=root.join(name); if !p.canonicalize()?.starts_with(root.canonicalize()?){return Err("Salida del directorio".into())} Ok(p)
}
fn texto_propuesto(root:&Path,c:&Value)->R<String>{
 let t=fs::read_to_string(path(root,s(&c["propuesto"]["ruta"])?)?)?.replace("\r\n","\n");
 let cab=format!("# Documento propuesto {}\n\n",s(&c["id"])?);
 Ok(t.strip_prefix(&cab).ok_or("Cabecera no prevista")?.trim_end_matches('\n').to_string())
}
fn split_markers(t:&str)->R<Vec<(String,String)>>{
 let mut out=Vec::new(); let mut cursor=0;
 while cursor<t.len(){
  let rest=t[cursor..].trim_start(); if rest.is_empty(){break}
  if !rest.starts_with('['){return Err("Contenido sin identificador".into())}
  let end=rest.find(']').ok_or("Identificador incompleto")?;
  let id=&rest[1..end]; if id.is_empty() || !id.chars().all(|c|c.is_ascii_alphanumeric()||c=='-'){return Err("Identificador no previsto".into())}
  let body=&rest[end+1..]; let next=body.find("\n\n[").unwrap_or(body.len());
  out.push((id.to_string(),body[..next].trim().to_string()));
  cursor=t.len()-body.len()+next;
 }
 Ok(out)
}
fn select(corpus:&Value,q:&Value)->Result<Vec<String>,String>{
 if q["ayuda_solicitada"]==false{return Ok(vec![])}
 let domain=q["dominio"].as_str().ok_or("dominio_ausente")?;
 if let Some(proposed)=q["dominio_propuesto"].as_str(){if proposed!=domain{return Err("dominio_propuesto_distinto".into())}}
 let docs=corpus["documentos"].as_array().ok_or("corpus_invalido")?;
 let mode=q["modo"].as_str().ok_or("modo_ausente")?;
 let accessible:BTreeSet<&str>=q["accesibles"].as_array().ok_or("permisos_ausentes")?.iter().map(|v|v.as_str().ok_or("permiso_invalido")).collect::<Result<_,_>>()?;
 let ids:Vec<String>=match mode {
  "documento"=>{let a=q["ids"].as_array().ok_or("ids_ausentes")?;if a.len()>1{return Err("seleccion_no_individual".into())} a.iter().map(|v|v.as_str().map(str::to_string).ok_or("id_invalido".to_string())).collect::<Result<_,_>>()?},
  "grupo"=>{let values=if let Some(a)=q["grupo_experimental"].as_array(){a}else{
   let g=corpus["grupos"].as_array().ok_or("grupos_ausentes")?.iter().find(|g|g["id"]==q["grupo"]).ok_or("grupo_inexistente")?;
   if g["dominio"]!=domain{return Err("grupo_con_referencia_ajena".into())} g["documentos"].as_array().ok_or("grupo_invalido")?
  };values.iter().map(|v|v.as_str().map(str::to_string).ok_or("id_invalido".to_string())).collect::<Result<_,_>>()?},
  "todos"=>docs.iter().filter(|d|d["dominio"]==domain && accessible.contains(d["id"].as_str().unwrap_or(""))).map(|d|d["id"].as_str().unwrap().to_string()).collect(),
  _=>return Err("modo_desconocido".into())};
 let mut seen=BTreeSet::new();
 for id in &ids {
  if !seen.insert(id){return Err("id_duplicado".into())}
  let d=docs.iter().find(|d|d["id"]==*id).ok_or("documento_inexistente")?;
  if d["dominio"]!=domain{return Err(if mode=="grupo"{"grupo_con_referencia_ajena"}else{"dominio_ajeno"}.into())}
  if !accessible.contains(id.as_str()){return Err("sin_permiso".into())}
  if d["estado_en_ensayo"]!="referencia_simulada"{return Err("estado_no_incorporado_en_ensayo".into())}
 }
 Ok(ids)
}
fn prepare(a:&[String])->R<()>{
 let root=Path::new(&a[2]);let lang=Path::new(&a[3]);let dest=Path::new(&a[4]);
 if dest.exists(){return Err("Se conserva la preparación anterior".into())}
 let corpus=read(root.join("CORPUS.json"))?;let entries=read(root.join("ENTRADAS.json"))?;
 let captures=read(lang.join("CAPTURAS-GOOGLE.json"))?;
 let mut originals=BTreeMap::<String,String>::new();let mut batches=vec![Vec::<String>::new();3];
 for p in arr(&corpus["pasajes"])? {let id=s(&p["id"])?;let text=s(&p["texto"])?;originals.insert(id.into(),text.into());batches[0].push(format!("[{id}]\n{text}"));}
 for c in arr(&entries["casos"])? {let id=s(&c["id"])?;let text=texto_propuesto(root,c)?;batches[if id.starts_with('I'){1}else{2}].push(format!("[{id}]\n{text}"));originals.insert(id.into(),text);}
 if originals.len()!=20 || arr(&captures["capturas"])?.len()!=3{return Err("Cantidad no prevista".into())}
 let mut translations=BTreeMap::<String,Value>::new();
 for (i,cap) in arr(&captures["capturas"])?.iter().enumerate(){
  let input=s(&cap["entrada"])?;if input!=batches[i].join("\n\n"){return Err(format!("Entrada distinta del original en lote {}",i+1).into())}
  let visible=s(&cap["salida_visible"])?;
  let payload=visible.strip_prefix("Resultados de traducción\n").ok_or("Cabecera de salida distinta")?.strip_suffix("\nstar_border").ok_or("Final de salida distinto")?;
  let parsed=split_markers(payload)?;
  let wanted:Vec<_>=split_markers(input)?.into_iter().map(|x|x.0).collect();
  if parsed.iter().map(|x|x.0.clone()).collect::<Vec<_>>()!=wanted{return Err("Identificadores omitidos o modificados".into())}
  for (id,en) in parsed {if en.is_empty()||translations.contains_key(&id){return Err("Texto vacío o duplicado".into())}
   let es=originals.get(&id).ok_or("Identificador ajeno")?;
   translations.insert(id.clone(),json!({"id":id,"es":es,"en":en,"lote":i+1,"fecha_utc":cap["fecha_utc"],"es_sha256":hash(es.as_bytes()),"en_sha256":hash(en.as_bytes())}));
  }
 }
 if translations.len()!=20{return Err("Cobertura incompleta".into())}
 fs::create_dir_all(dest)?;
 let mut reference_docs=BTreeMap::<String,Value>::new();
 for d in arr(&corpus["documentos"])?{
  let id=s(&d["id"])?;let mut content=format!("# {id}\n\n");
  for p in arr(&d["pasajes"])?{let pid=s(p)?;content.push_str(&format!("## {pid}\n\n{}\n\n",s(&translations[pid]["en"])?));}
  let name=format!("{id}.md");fs::write(dest.join(&name),&content)?;
  reference_docs.insert(id.into(),json!({"id":id,"ruta":name,"dominio":d["dominio"],"sha256":hash(content.as_bytes())}));
 }
 let accessible:Vec<_>=reference_docs.keys().cloned().collect();let mut cases=Vec::new();
 for c in arr(&entries["casos"])? {
  let id=s(&c["id"])?;let mut q=c["seleccion"].clone();q["dominio"]=c["dominio"].clone();q["dominio_propuesto"]=c["dominio"].clone();q["accesibles"]=json!(accessible);
  let ids=select(&corpus,&q).map_err(|e|format!("Selección {id}: {e}"))?;
  let content=format!("# {id}\n\n{}\n",s(&translations[id]["en"])?);let name=format!("{id}.md");fs::write(dest.join(&name),&content)?;
  cases.push(json!({"id":id,"dominio":c["dominio"],"propuesto":{"id":id,"ruta":name,"sha256":hash(content.as_bytes())},"referencias_documentales":ids.iter().map(|id|reference_docs[id].clone()).collect::<Vec<_>>()}));
 }
 save(dest.join("TRADUCCIONES.json"),&json!({"traducciones":translations.values().collect::<Vec<_>>(),"identidad_entradas_verificada":true,"equivalencia_semantica_automatica":false,"capturas_sha256":hash(&fs::read(lang.join("CAPTURAS-GOOGLE.json"))?)}))?;
 save(dest.join("ENTRADAS-EN.json"),&json!({"perfil":"SC-XF002 exclusivamente entre propuesto y cada referencia seleccionada","casos":cases,"idioma":"en","referencias_de_evaluacion_recibidas":false,"corpus_es_sha256":hash(&fs::read(root.join("CORPUS.json"))?),"entradas_es_sha256":hash(&fs::read(root.join("ENTRADAS.json"))?)}))?;
 let tests=read(root.join("SELECCIONES-PREVISTAS.json"))?;let mut checked=Vec::new();
 for q in arr(&tests["casos"])?{
  let result=select(&corpus,q);let obtained=match &result{Ok(ids)=>json!({"seleccion":ids}),Err(e)=>json!({"rechazo":e})};
  let ok=match result {Ok(ids)=>json!(ids)==q["esperado"],Err(e)=>json!(e)==q["rechazo"]};
  checked.push(json!({"id":q["id"],"obtenido":obtained,"conforme":ok,"llamadas_candidato":0}));
 }
 let all=checked.iter().all(|x|x["conforme"]==true);
 save(dest.join("SELECCIONES-RESULTADO.json"),&json!({"naturaleza":"comprobación aislada de selección; sin conexión a la administración real","todos_conformes":all,"casos":checked}))?;
 if !all{return Err("Selecciones no conformes".into())}
 println!("20 textos cotejados; 12 casos preparados; 12 selecciones conformes; candidato no ejecutado");Ok(())
}
fn count(ss:&[Section])->usize{ss.iter().map(|s|s.requirements.len()+count(&s.subsections)).sum()}
fn detect(a:&[String])->R<()>{
 let root=Path::new(&a[2]);let plan=read(root.join("ENTRADAS-EN.json"))?;
 if Path::new(&a[3]).exists(){return Err("Se conserva el resultado anterior".into())}
 let timer=Instant::now();let start=SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
 let parser=MarkdownParser::new();let rule=CrossFileContradiction::new();let mut cases=Vec::new();
 for c in arr(&plan["casos"])? {
  let t=Instant::now();let mut pairs=Vec::new();
  for r in arr(&c["referencias_documentales"])?{
   if r["dominio"]!=c["dominio"]{return Err("Referencia de otro dominio".into())}
   let mut docs=Vec::new();let mut inputs=Vec::new();
   for f in [&c["propuesto"],r]{
    let name=s(&f["ruta"])?;let raw=fs::read(path(root,name)?)?;if hash(&raw)!=s(&f["sha256"])?{return Err("Huella de entrada no conforme".into())}
    let text=String::from_utf8(raw)?;let doc=parser.parse(Path::new(name),&text)?;
    inputs.push(json!({"id":f["id"],"ruta":name,"sha256":f["sha256"],"bytes":text.len(),"requisitos_reconocidos":count(&doc.sections)}));docs.push(doc);
   }
   let findings=rule.check_corpus(&[&docs[0],&docs[1]]);
   let out:Vec<_>=findings.iter().map(|d|json!({"regla":d.rule_id.to_string(),"mensaje_original":d.message,"localizacion":d.span,"gravedad_original":format!("{:?}",d.severity),"confianza_declarada_no_calibrada":d.confidence})).collect();
   pairs.push(json!({"archivos":inputs,"hallazgos":out,"alerta":!out.is_empty()}));
  }
  cases.push(json!({"id":c["id"],"dominio":c["dominio"],"alerta":pairs.iter().any(|p|p["alerta"]==true),"pares":pairs,"duracion_microsegundos":t.elapsed().as_micros()}));
 }
 save(&a[3],&json!({"elemento":"Buscador-Semántico - Diferencial","candidato":"speccheck-core 0.4.1","reglas":[rule.id().to_string()],"idioma":"en","ia_detector":false,"traduccion":"Google Translate conservada previamente","referencias_de_evaluacion_recibidas":false,"reescritura_de_textos":false,"decisiones_documentales":false,"entradas_sha256":hash(&fs::read(root.join("ENTRADAS-EN.json"))?),"ejecutable_sha256":hash(&fs::read(std::env::current_exe()?)?),"inicio_unix_ms":start,"duracion_microsegundos":timer.elapsed().as_micros(),"casos":cases}))?;
 println!("12 casos ejecutados con SC-XF002; resultado conservado");Ok(())
}
fn evaluate(a:&[String])->R<()>{
 let actual=read(&a[2])?;let refs=read(&a[3])?;let mut rows=Vec::new();let mut tp=0;let mut fnn=0;let mut fp=0;let mut tn=0;let mut ua=0;let mut un=0;
 if arr(&actual["casos"])?.len()!=12||arr(&refs["casos"])?.len()!=12{return Err("Cantidad no conforme".into())}
 for r in arr(&refs["casos"])?{let c=arr(&actual["casos"])?.iter().find(|c|c["id"]==r["id"]).ok_or("Caso ausente")?;let alert=c["alerta"]==true;
  match(s(&r["referencia"])?,alert){("incompatibilidad",true)=>tp+=1,("incompatibilidad",false)=>fnn+=1,("compatibilidad",true)=>fp+=1,("compatibilidad",false)=>tn+=1,("contexto_insuficiente",true)=>ua+=1,("contexto_insuficiente",false)=>un+=1,_=>return Err("Referencia no reconocida".into())};
  rows.push(json!({"id":r["id"],"referencia":r["referencia"],"alerta":alert}));
 }
 save(&a[4],&json!({"resultado_sha256":hash(&fs::read(&a[2])?),"referencias_sha256":hash(&fs::read(&a[3])?),"detecciones":tp,"omisiones":fnn,"avisos_en_compatibles":fp,"silencios_en_compatibles":tn,"avisos_en_contexto_insuficiente":ua,"silencios_en_contexto_insuficiente":un,"casos":rows,"validacion_independiente":false,"silencio_equivale_a_compatibilidad":false}))?;
 println!("Detecciones {tp}; omisiones {fnn}; avisos compatibles {fp}; contexto insuficiente con aviso {ua}");Ok(())
}
fn main(){let a:Vec<String>=std::env::args().collect();let r=match a.get(1).map(String::as_str){Some("preparar")if a.len()==5=>prepare(&a),Some("detectar")if a.len()==4=>detect(&a),Some("evaluar")if a.len()==5=>evaluate(&a),_=>Err("Uso: preparar PASO3 IDIOMAS DESTINO | detectar PREPARACION SALIDA | evaluar RESULTADO REFERENCIAS SALIDA".into())};if let Err(e)=r{eprintln!("INCIDENCIA: {e}");std::process::exit(1)}}
