//! Correspondencia de evidencia ya conservada; no ejecuta el comparador.
use serde_json::{json, Value};
use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},error::Error,fs::{self,OpenOptions},io::Write,path::{Path,Component}};
type R<T>=Result<T,Box<dyn Error>>;
const FOOT:&str="© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
fn must(ok:bool,e:&str)->R<()> {if ok {Ok(())}else{Err(e.into())}}
fn s(v:&Value)->R<&str>{v.as_str().ok_or_else(||"TEXTO_AUSENTE".into())}
fn ar(v:&Value)->R<&Vec<Value>>{v.as_array().ok_or_else(||"LISTA_AUSENTE".into())}
fn num(v:&Value)->R<usize>{Ok(v.as_u64().ok_or("ENTERO_AUSENTE")?.try_into()?)}
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn read(p:impl AsRef<Path>)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn save(p:impl AsRef<Path>,bytes:&[u8])->R<()>{let mut f=OpenOptions::new().write(true).create_new(true).open(p)?;f.write_all(bytes)?;f.sync_all()?;Ok(())}
fn jsave(p:impl AsRef<Path>,v:&Value)->R<()>{save(p,&serde_json::to_vec_pretty(v)?)}
fn one<'a>(xs:&'a[Value],field:&str,id:&str)->R<&'a Value>{let m:Vec<_>=xs.iter().filter(|v|v[field]==id).collect();must(m.len()==1,"IDENTIDAD_NO_UNICA")?;Ok(m[0])}
fn unique(xs:&[Value],field:&str,err:&str)->R<()>{let mut ids=BTreeSet::new();for v in xs{must(ids.insert(s(&v[field])?),err)?}Ok(())}
fn esc(t:&str)->String{t.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;").replace('"',"&quot;").replace('\'',"&#39;").replace('{',"&#123;").replace('}',"&#125;")}
fn pre(t:&str)->String{format!("<pre class=\"texto\">{}</pre>",esc(t))}
#[derive(Clone)] struct Inputs {data:BTreeMap<String,Value>, originals:BTreeMap<String,String>, receipts:Vec<Value>}
fn load(plan:&Path,root:&Path)->R<Inputs>{
 let p=read(plan)?;let mut data=BTreeMap::new();let mut originals=BTreeMap::new();let mut receipts=Vec::new();let base=root.canonicalize()?;
 for f in ar(&p["archivos"])?{let name=s(&f["ruta"])?;must(!name.is_empty()&&Path::new(name).components().all(|c|matches!(c,Component::Normal(_))),"RUTA_INVALIDA")?;let path=root.join(name).canonicalize()?;must(path.starts_with(&base),"RUTA_AJENA")?;let raw=fs::read(path)?;must(raw.len()==num(&f["bytes"])?&&hash(&raw)==s(&f["sha256"])?,"FIJACION_DISCORDANTE")?;let alias=s(&f["alias"])?;
  if f["tipo"]=="texto"{must(originals.insert(alias.into(),String::from_utf8(raw)?).is_none(),"ALIAS_DUPLICADO")?;}else{must(data.insert(alias.into(),serde_json::from_slice(&raw)?).is_none(),"ALIAS_DUPLICADO")?;}
  receipts.push(json!({"alias":alias,"bytes":f["bytes"],"sha256":f["sha256"]}));
 }Ok(Inputs{data,originals,receipts})
}
fn inspect(input:&Inputs)->R<Value>{
 let d=&input.data;let entries=ar(&d["entradas"]["casos"])?;must(entries.len()==28,"ENTRADAS_INCOMPLETAS")?;unique(entries,"id","CASOS_DUPLICADOS")?;
 let allresults=ar(&d["resultado"]["casos"])?;must(allresults.len()==72,"RESULTADO_INCOMPLETO")?;
 let results:Vec<Value>=allresults.iter().filter(|r|r["perfil"]=="should_coincidencia").cloned().collect();must(results.len()==28,"RESULTADO_INCOMPLETO")?;unique(&results,"id","RESULTADOS_DUPLICADOS")?;
 let translations=ar(&d["traducciones"]["traducciones"])?;must(translations.len()==20,"TRADUCCIONES_INCOMPLETAS")?;unique(translations,"id","TRADUCCION_DUPLICADA")?;
 let passages=ar(&d["corpus"]["pasajes"])?;must(passages.len()==8,"PASAJES_INCOMPLETOS")?;unique(passages,"id","PASAJE_DUPLICADO")?;
 for t in translations{must(hash(s(&t["es"])?.as_bytes())==s(&t["es_sha256"])?,"ES_HUELLA")?;must(hash(s(&t["en"])?.as_bytes())==s(&t["en_sha256"])?,"EN_HUELLA")?;
  let id=s(&t["id"])?;let es=if id.starts_with("P-"){s(&one(passages,"id",id)?["texto"])?.to_string()}else{let raw=input.originals.get(id).ok_or("ORIGINAL_AUSENTE")?.replace("\r\n","\n");raw.strip_prefix(&format!("# Documento propuesto {id}\n\n")).ok_or("CABECERA_ORIGINAL")?.trim_end_matches('\n').into()};must(es==s(&t["es"])?,"ORIGINAL_DISCORDANTE")?;
 }
 let mut rows=Vec::new();let mut alerts=0;let mut mapped=0;
 for c in entries{let id=s(&c["id"])?;let domain=s(&c["dominio"])?;must(["inmunologia","ciberseguridad"].contains(&domain),"DOMINIO_INVALIDO")?;let r=one(&results,"id",id)?;must(r["dominio"]==c["dominio"]&&r["conjunto"]==c["conjunto"],"DOMINIO_DISCORDANTE")?;
  let docs=ar(&c["documentos"])?;must(docs.len()==2,"PAR_INCOMPLETO")?;unique(docs,"ruta","DOCUMENTO_DUPLICADO")?;must(ar(&r["documentos"])?.len()==2,"PAR_RESULTADO_INCOMPLETO")?;
  let mut display=Vec::new();
  for (side,doc) in docs.iter().enumerate(){let name=s(&doc["ruta"])?;let en=s(&doc["texto"])?;must(hash(en.as_bytes())==s(&doc["sha256"])?,"HUELLA_DOCUMENTO")?;let recognized=one(ar(&r["documentos"])?,"ruta",name)?;must(recognized["sha256"]==doc["sha256"],"DOCUMENTO_RESULTADO_DISCORDANTE")?;
   let mut blocks=Vec::new();let mut source_es=Value::Null;
   if c["conjunto"]=="documental_conocido"{
    let prev=one(ar(&d["plan_anterior"]["casos"])?,"id",id)?;must(prev["dominio"]==domain,"DOMINIO_DISCORDANTE")?;let f=if side==0{&prev["propuesto"]}else{must(ar(&prev["referencias_documentales"])?.len()==1,"REFERENCIA_NO_UNICA")?;&prev["referencias_documentales"][0]};must(f["ruta"]==name&&f["sha256"]==doc["sha256"],"SELECCION_DISCORDANTE")?;
    let mut expected=String::new();
    if side==0{let t=one(translations,"id",id)?;expected.push_str(&format!("# {id}\n\n"));let start=expected.len();expected.push_str(s(&t["en"])?);expected.push('\n');source_es=json!({"sha256":hash(input.originals[id].as_bytes()),"naturaleza":"documento experimental; no atribuir su contenido alterado a la fuente de referencia"});blocks.push(json!({"id":id,"idioma_original":"es","original":t["es"],"ingles":t["en"],"original_sha256":t["es_sha256"],"ingles_sha256":t["en_sha256"],"inicio_en":start,"fin_en":expected.len(),"fuente":{"titulo":"Documento propuesto de ensayo","formato":"Markdown"},"localizador_fuente":null,"derechos":"Documento experimental; conservar las atribuciones y condiciones del paso 3. Los derivados de INCIBE mantienen CC BY-NC-SA 4.0."}));
    }else{let reference=one(ar(&d["corpus"]["documentos"])?,"id",s(&f["id"])?)?;must(reference["dominio"]==domain&&f["dominio"]==domain,"DOMINIO_DISCORDANTE")?;expected.push_str(&format!("# {}\n\n",s(&reference["id"])?));
     for pid in ar(&reference["pasajes"])?{let p=one(passages,"id",s(pid)?)?;must(p["dominio"]==domain&&p["documento_referencia"]==reference["id"],"DOMINIO_DISCORDANTE")?;let t=one(translations,"id",s(pid)?)?;let source=one(ar(&d["fuentes"]["fuentes"])?,"id",s(&p["fuente"])?)?;must(source["dominio"]==domain,"DOMINIO_DISCORDANTE")?;expected.push_str(&format!("## {}\n\n",s(pid)?));let start=expected.len();expected.push_str(s(&t["en"])?);expected.push_str("\n\n");blocks.push(json!({"id":pid,"idioma_original":"es","original":t["es"],"ingles":t["en"],"original_sha256":t["es_sha256"],"ingles_sha256":t["en_sha256"],"inicio_en":start,"fin_en":expected.len(),"fuente":source,"localizador_fuente":p["localizador"],"derechos":p["derechos"]}));}
    }must(expected==en,"RECONSTRUCCION_DISCORDANTE")?;
   }else{must(c["conjunto"]=="adversarial","CONJUNTO_DESCONOCIDO")?;blocks.push(json!({"id":name,"idioma_original":"en","original":en,"ingles":en,"original_sha256":doc["sha256"],"ingles_sha256":doc["sha256"],"inicio_en":0,"fin_en":en.len(),"fuente":{"titulo":"Control sintético original en inglés","formato":"Markdown"},"localizador_fuente":null,"derechos":"Control experimental propio; no constituye una recomendación clínica ni de actuación sobre sistemas."}));}
   for req in ar(&recognized["afirmaciones_reconocidas"])?{validate_span(en,name,&req["localizacion"])?;}
   display.push(json!({"ruta":name,"lado":side,"ingles_completo":en,"ingles_sha256":doc["sha256"],"original_documento":source_es,"bloques":blocks,"afirmaciones_reconocidas":recognized["afirmaciones_reconocidas"]}));
  }
  let findings=ar(&r["hallazgos"])?;must(r["alerta"].as_bool().ok_or("ALERTA_AUSENTE")?==!findings.is_empty(),"ALERTA_DISCORDANTE")?;let mut locations=Vec::new();
  for finding in findings{let loc=&finding["localizacion"];let doc=one(&display,"ruta",s(&loc["file"])?)?;let en=s(&doc["ingles_completo"])?;let (start,end)=validate_span(en,s(&doc["ruta"])?,loc)?;
   let candidates:Vec<_>=ar(&doc["bloques"])?.iter().filter(|b|num(&b["inicio_en"]).map(|x|x<=start).unwrap_or(false)&&num(&b["fin_en"]).map(|x|x>=end).unwrap_or(false)).collect();must(candidates.len()==1,"CORRESPONDENCIA_AMBIGUA")?;
   let recognized=ar(&doc["afirmaciones_reconocidas"])?.iter().filter(|q|q["localizacion"]==*loc).count();must(recognized==1,"LOCALIZACION_NO_ACREDITADA")?;
   locations.push(json!({"localizacion_declarada":loc,"bloque_correspondiente":candidates[0]["id"],"fragmento_ingles_localizado":&en[start..end],"otro_pasaje_localizado":false,"regla":finding["regla"]}));mapped+=1;
  }
  if !findings.is_empty(){alerts+=1;}
  rows.push(json!({"id":id,"dominio":domain,"conjunto":c["conjunto"],"aviso_conservado":!findings.is_empty(),"localizaciones":locations,"documentos":display,"vigencia_documental":"no acreditada por el comparador; consultar encabezamientos y contexto conservados"}));
 }
 must(alerts==7&&mapped==7,"AVISOS_NO_PREVISTOS")?;
 Ok(json!({"elemento":"Buscador-Semántico - Diferencial","paso":"6 / 8","perfil_conservado":"should_coincidencia","pares":28,"avisos_conservados":alerts,"casos_sin_aviso":28-alerts,"localizaciones_directas_cotejadas":mapped,"segundas_localizaciones_no_declaradas":mapped,"traducciones_cotejadas":20,"equivalencia_semantica_acreditada":false,"detector_ejecutado":false,"integracion_operativa":false,"fuentes_fijadas":input.receipts,"casos":rows}))
}
fn validate_span(text:&str,name:&str,span:&Value)->R<(usize,usize)>{must(span["file"]==name,"LOCALIZACION_ARCHIVO")?;let a=num(&span["byte_start"])?;let b=num(&span["byte_end"])?;must(a<b&&b<=text.len(),"LOCALIZACION_RANGO")?;must(text.is_char_boundary(a)&&text.is_char_boundary(b),"LOCALIZACION_UTF8")?;Ok((a,b))}
fn checks(input:&Inputs)->R<Value>{
 let mut rows=vec![json!({"id":"BASE","conforme":inspect(input).is_ok(),"esperado":"28 pares y siete avisos conservados"})];
 let tests:[(&str,&str,fn(&mut Inputs));10]=[
  ("FALTA_TRADUCCION","TRADUCCIONES_INCOMPLETAS",|x|{x.data.get_mut("traducciones").unwrap()["traducciones"].as_array_mut().unwrap().pop();}),
  ("ID_DUPLICADO","TRADUCCION_DUPLICADA",|x|{let t=&mut x.data.get_mut("traducciones").unwrap()["traducciones"];t[1]["id"]=t[0]["id"].clone();}),
  ("INGLES_ALTERADO","EN_HUELLA",|x|x.data.get_mut("traducciones").unwrap()["traducciones"][0]["en"]=json!("alterado")),
  ("ORIGINAL_ALTERADO","ORIGINAL_DISCORDANTE",|x|x.data.get_mut("corpus").unwrap()["pasajes"][0]["texto"]=json!("alterado")),
  ("DOMINIO_AJENO","DOMINIO_DISCORDANTE",|x|x.data.get_mut("corpus").unwrap()["documentos"][0]["dominio"]=json!("ciberseguridad")),
  ("DOCUMENTO_ALTERADO","HUELLA_DOCUMENTO",|x|x.data.get_mut("entradas").unwrap()["casos"][0]["documentos"][0]["texto"]=json!("alterado")),
  ("RESULTADO_INCOMPLETO","RESULTADO_INCOMPLETO",|x|{x.data.get_mut("resultado").unwrap()["casos"].as_array_mut().unwrap().pop();}),
  ("CORTE_UTF8","LOCALIZACION_UTF8",|x|{let rows=x.data.get_mut("resultado").unwrap()["casos"].as_array_mut().unwrap();let c=rows.iter_mut().find(|r|r["perfil"]=="should_coincidencia"&&r["id"]=="C03").unwrap();c["hallazgos"][0]["localizacion"]["byte_start"]=json!(8);}),
  ("FUERA_DE_RANGO","LOCALIZACION_RANGO",|x|{let rows=x.data.get_mut("resultado").unwrap()["casos"].as_array_mut().unwrap();let c=rows.iter_mut().find(|r|r["perfil"]=="should_coincidencia"&&r["id"]=="C03").unwrap();c["hallazgos"][0]["localizacion"]["byte_end"]=json!(999999);}),
  ("CASO_DUPLICADO","CASOS_DUPLICADOS",|x|{let c=&mut x.data.get_mut("entradas").unwrap()["casos"];c[1]["id"]=c[0]["id"].clone();})];
 for (id,expected,mutate) in tests{let mut changed=input.clone();mutate(&mut changed);let error=inspect(&changed).err().map(|e|e.to_string());rows.push(json!({"id":id,"esperado":expected,"obtenido":error,"conforme":error.as_deref()==Some(expected)}));}
 let active="</pre><script>alert(1)</script><img src=\"https://invalid.example/x\">{{#include fuera.md}}";let inert=pre(active);rows.push(json!({"id":"TEXTO_INERTE","conforme":!inert.contains("<script")&&!inert.contains("<img")&&!inert.contains("{{#include")&&inert.contains("&lt;script&gt;")}));
 let ok=rows.iter().all(|r|r["conforme"]==true);Ok(json!({"todos_conformes":ok,"detector_ejecutado":false,"pruebas":rows}))
}
fn render_case(c:&Value)->R<String>{
 let mut out=format!("# Caso {} · {}\n\n",s(&c["id"])?,s(&c["dominio"])?);
 out.push_str(if c["aviso_conservado"]==true{"**Aviso conservado de la comparación anterior.** Se localiza directamente un solo pasaje. El otro documento se muestra completo dentro de la cobertura del ensayo; no se le atribuye una localización ausente.\n\n"}else{"**La comparación anterior no emitió aviso.** Este silencio no acredita coherencia documental.\n\n"});
 out.push_str("Muestra experimental de consulta opcional. Los textos propuestos pueden contener alteraciones deliberadas. No son recomendaciones clínicas ni de actuación sobre sistemas. La presentación no modifica documentos, permisos o su suministro.\n\n");
 for (i,d) in ar(&c["documentos"])?.iter().enumerate(){out.push_str(&format!("## {}\n\n",if i==0{"Documento propuesto"}else{"Documento de referencia utilizado"}));
  out.push_str("### Documento inglés completo utilizado en este par\n\nLos encabezamientos se conservan literalmente. La vigencia y la autoridad no se deducen del aviso.\n\n");out.push_str(&pre(s(&d["ingles_completo"])?));out.push_str("\n\n");
  for loc in ar(&c["localizaciones"])?{if loc["localizacion_declarada"]["file"]==d["ruta"]{out.push_str("### Pasaje localizado directamente por el diagnóstico\n\n");out.push_str(&pre(s(&loc["fragmento_ingles_localizado"])?));out.push_str("\n\n");}}
  for b in ar(&d["bloques"])?{out.push_str(&format!("### Correspondencia {}\n\n",s(&b["id"])?));out.push_str(if b["idioma_original"]=="es"{"**Original español conservado**\n\n"}else{"**Original inglés; no se ha generado traducción española**\n\n"});out.push_str(&pre(s(&b["original"])?));out.push_str("\n\n");if b["idioma_original"]=="es"{out.push_str("**Traducción instrumental inglesa conservada de Google**\n\n");out.push_str(&pre(s(&b["ingles"])?));out.push_str("\n\nLa correspondencia se verifica por identificador y huellas; no certifica equivalencia semántica.\n\n");}
   out.push_str("**Procedencia y alcance**\n\n");
   let source=&b["fuente"];let locator=&b["localizador_fuente"];
   out.push_str(&format!("<p>{}. Formato de origen: {}.</p>\n\n",esc(s(&source["titulo"])?),esc(s(&source["formato"])?)));
   if let Some(edition)=source["edicion"].as_str(){out.push_str(&format!("<p>Edición: {}.</p>\n\n",esc(edition)));}
   if let Some(page)=locator["pagina_ordinal"].as_u64(){out.push_str(&format!("<p>Página física {page}; índice {}.</p>\n\n",num(&locator["pagina_indice"])?));}
   if let Some(heading)=locator["epigrafe"].as_str(){out.push_str(&format!("<p>Epígrafe: {}.</p>\n\n",esc(heading)));}
   out.push_str(&format!("<p>{}</p>\n\n",esc(s(&b["derechos"])?)));}
 }
 out.push_str("Se conserva el documento parcial usado en la prueba; no equivale a la totalidad del PDF, HTML o libro de origen. Las páginas indicadas por índice comienzan en 0; el ordinal físico comienza en 1.\n\n");out.push_str(FOOT);out.push_str("\n\nEste aviso corresponde a la exposición propia. Los originales y sus derivados conservan las licencias indicadas.\n");Ok(out)
}
fn book(out:&Path,v:&Value)->R<()>{
 let root=out.join("consulta-mdbook");fs::create_dir_all(root.join("src"))?;fs::create_dir(root.join("theme"))?;
 let cfg="[book]\ntitle = \"Buscador-Semántico - Diferencial · Consulta experimental\"\nlanguage = \"es\"\nsrc = \"src\"\n[build]\nbuild-dir = \"book\"\ncreate-missing = false\nuse-default-preprocessors = false\n[output.html]\ntheme = \"theme\"\nsmart-punctuation = false\nhash-files = false\n[output.html.search]\nenable = false\n[output.html.print]\nenable = false\n[output.html.playground]\nrunnable = false\ncopyable = false\ncopy-js = false\n";
 save(root.join("book.toml"),cfg.as_bytes())?;
 let template="<!doctype html><html lang=es><head><meta charset=utf-8><meta name=viewport content='width=device-width,initial-scale=1'><meta http-equiv=Content-Security-Policy content=\"default-src 'none'; style-src 'unsafe-inline'; base-uri 'none'; form-action 'none'\"><title>{{title}}</title><style>body{font:18px/1.6 Georgia,serif;max-width:1050px;margin:auto;padding:2rem;color:#172c3a;background:#fafaf7}h1,h2,h3{font-family:system-ui;line-height:1.3}h2{border-top:2px solid #48766e;padding-top:1.2rem;margin-top:2rem}a{color:#075d70}pre.texto{white-space:pre-wrap;overflow-wrap:anywhere;background:#edf2f2;border-left:4px solid #48766e;padding:1rem;font:16px/1.6 Georgia,serif}nav{font-family:system-ui;font-size:16px}article{min-width:0}</style></head><body><nav><a href='indice.html'>Índice por dominio</a> · Consulta experimental de evidencia conservada</nav><article>{{{content}}}</article></body></html>";
 save(root.join("theme/index.hbs"),template.as_bytes())?;
 let mut summary=String::from("# Índice\n\n- [Consulta de correspondencias](indice.md)\n");let mut index=String::from("# Consulta de correspondencias\n\nMuestra del paso 6 / 8 del Buscador-Semántico - Diferencial. Reúne 28 pares ya comparados: siete avisos conservados y veintiún casos sin aviso. No se ha ejecutado de nuevo el comparador. La consulta es opcional y no modifica decisiones documentales.\n\nEl diagnóstico anterior localiza directamente un solo lado. Se conservan ambos documentos y sus encabezamientos, sin inventar el otro pasaje. Los originales españoles y sus traducciones se muestran juntos cuando existen. Los controles originales ingleses se identifican expresamente.\n\nEsta muestra no es un nuevo libro de la Biblioteca ni una integración operativa.\n\n");
 for domain in ["inmunologia","ciberseguridad"]{index.push_str(&format!("## {}\n\n",if domain=="inmunologia"{"Inmunología"}else{"Ciberseguridad"}));for c in ar(&v["casos"])?{if c["dominio"]!=domain{continue}let id=s(&c["id"])?;must(id.chars().all(|x|x.is_ascii_alphanumeric()),"ID_NO_SEGURO")?;let name=format!("{id}.md");summary.push_str(&format!("- [{id} · {domain}]({name})\n"));index.push_str(&format!("- [{id}]({name}): {}.\n",if c["aviso_conservado"]==true{"aviso conservado"}else{"sin aviso; no acredita coherencia"}));save(root.join("src").join(name),render_case(c)?.as_bytes())?;}}
 index.push_str(&format!("\n{FOOT}\n"));save(root.join("src/SUMMARY.md"),summary.as_bytes())?;save(root.join("src/indice.md"),index.as_bytes())?;Ok(())
}
fn verify_html(v:&Value,root:&Path)->R<Value>{let mut rows=Vec::new();for c in ar(&v["casos"])?{let name=format!("{}.html",s(&c["id"])?);let html=fs::read_to_string(root.join(&name))?;let mut expected=Vec::new();for d in ar(&c["documentos"])?{expected.push(pre(s(&d["ingles_completo"])?));for b in ar(&d["bloques"])?{expected.push(pre(s(&b["original"])?));if b["idioma_original"]=="es"{expected.push(pre(s(&b["ingles"])?));}}}for loc in ar(&c["localizaciones"])?{expected.push(pre(s(&loc["fragmento_ingles_localizado"])?));}let decoded=html_escape::decode_html_entities(&html);let ok=expected.iter().all(|p|decoded.contains(html_escape::decode_html_entities(p).as_ref()))&&!html.contains("<script")&&!html.contains("<iframe")&&!html.contains("<form")&&!html.contains("<img");rows.push(json!({"caso":c["id"],"html_sha256":hash(html.as_bytes()),"fragmentos_completos_comprobados":expected.len(),"conforme":ok}));}
 let ok=rows.iter().all(|r|r["conforme"]==true);Ok(json!({"todos_conformes":ok,"paginas_de_casos":rows.len(),"casos":rows,"inspeccion_visual":false}))}
fn main()->R<()>{let a:Vec<_>=std::env::args().collect();match a.get(1).map(String::as_str){
 Some("preparar") if a.len()==5=>{let input=load(Path::new(&a[2]),Path::new(&a[3]))?;let v=inspect(&input)?;let tests=checks(&input)?;let out=Path::new(&a[4]);must(!out.exists(),"SALIDA_EXISTENTE")?;fs::create_dir(out)?;jsave(out.join("COMPROBACIONES-RUST.json"),&tests)?;must(tests["todos_conformes"]==true,"COMPROBACIONES_NO_CONFORMES")?;jsave(out.join("CORRESPONDENCIAS.json"),&v)?;book(out,&v)?;println!("28 pares, 20 traducciones y 7 localizaciones cotejados; 12 comprobaciones conformes; detector no ejecutado");Ok(())},
 Some("verificar-html") if a.len()==5=>{let v=read(&a[2])?;let r=verify_html(&v,Path::new(&a[3]))?;jsave(&a[4],&r)?;must(r["todos_conformes"]==true,"HTML_DISCORDANTE")?;println!("28 páginas HTML con textos completos cotejados en Rust");Ok(())},
 _=>Err("Uso: preparar plan.json raiz salida_nueva | verificar-html correspondencias.json directorio_html salida.json".into())}}
