//! Instrumento experimental aislado del Buscador-Semántico - Diferencial.
//! Las referencias de evaluación sólo se abren en el comando evaluar.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use speccheck_core::{parser::{MarkdownParser, SpecParser}, rules::{cross_file::CrossFileContradiction, Rule}, Section};
use std::{collections::BTreeSet, error::Error, fs::{self, OpenOptions}, io::Write, path::{Component, Path}, time::{Instant, SystemTime, UNIX_EPOCH}};
type R<T> = Result<T, Box<dyn Error>>;
fn hash(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
fn read(p: impl AsRef<Path>) -> R<Value> { Ok(serde_json::from_slice(&fs::read(p)?)?) }
fn save(p: impl AsRef<Path>, v: &Value) -> R<()> { let mut f=OpenOptions::new().write(true).create_new(true).open(p)?; f.write_all(&serde_json::to_vec_pretty(v)?)?; f.sync_all()?; Ok(()) }
fn arr(v: &Value) -> R<&Vec<Value>> { v.as_array().ok_or_else(||"Lista ausente".into()) }
fn s(v: &Value) -> R<&str> { v.as_str().ok_or_else(||"Texto ausente".into()) }
fn safe(root: &Path, name: &str) -> R<std::path::PathBuf> {
    if name.is_empty() || !Path::new(name).components().all(|c| matches!(c,Component::Normal(_))) { return Err("Ruta ajena".into()); }
    let p=root.join(name); if !p.canonicalize()?.starts_with(root.canonicalize()?) { return Err("Salida del directorio".into()); } Ok(p)
}
fn recognized(ss: &[Section], out: &mut Vec<Value>) {
    for sec in ss { for req in &sec.requirements { out.push(json!({"texto":req.text,"localizacion":req.span})); } recognized(&sec.subsections,out); }
}
fn detect(a: &[String]) -> R<()> {
    let adv=read(&a[2])?; let oldroot=Path::new(&a[3]); let dest=Path::new(&a[4]);
    if dest.exists() { return Err("Se conserva la ejecución anterior".into()); }
    let old=read(oldroot.join("ENTRADAS-EN.json"))?; let mut all=Vec::new(); let mut ids=BTreeSet::new();
    if arr(&adv["casos"])?.len()!=16 || arr(&old["casos"])?.len()!=12 { return Err("Cantidades distintas del protocolo".into()); }
    for c in arr(&adv["casos"])? {
        if !ids.insert(s(&c["id"])?.to_owned()) || !["inmunologia","ciberseguridad"].contains(&s(&c["dominio"])?) || arr(&c["documentos"])?.len()!=2 { return Err("Control inválido".into()); }
        let mut docs=Vec::new();
        for d in arr(&c["documentos"])? { let name=s(&d["ruta"])?; let text=s(&d["texto"])?; if text.is_empty() { return Err("Texto vacío".into()); } docs.push(json!({"ruta":name,"texto":text,"sha256":hash(text.as_bytes())})); }
        if docs[0]["ruta"]==docs[1]["ruta"] { return Err("Documentos no distintos".into()); }
        all.push(json!({"id":c["id"],"dominio":c["dominio"],"conjunto":"adversarial","documentos":docs}));
    }
    for c in arr(&old["casos"])? {
        if !ids.insert(s(&c["id"])?.to_owned()) || !["inmunologia","ciberseguridad"].contains(&s(&c["dominio"])?) || arr(&c["referencias_documentales"])?.len()!=1 { return Err("Caso anterior no previsto".into()); }
        let r=&c["referencias_documentales"][0]; if r["dominio"]!=c["dominio"] { return Err("Referencia de otro dominio".into()); }
        let mut docs=Vec::new();
        for f in [&c["propuesto"],r] { let name=s(&f["ruta"])?; let raw=fs::read(safe(oldroot,name)?)?; if hash(&raw)!=s(&f["sha256"])? { return Err("Texto anterior modificado".into()); } let text=String::from_utf8(raw)?; docs.push(json!({"ruta":name,"texto":text,"sha256":f["sha256"]})); }
        all.push(json!({"id":c["id"],"dominio":c["dominio"],"conjunto":"documental_conocido","documentos":docs}));
    }
    fs::create_dir(dest)?;
    save(dest.join("ENTRADAS-COTEJADAS.json"),&json!({"casos":all,"controles_sha256":hash(&fs::read(&a[2])?),"entradas_anteriores_sha256":hash(&fs::read(oldroot.join("ENTRADAS-EN.json"))?)}))?;
    let mut stream=OpenOptions::new().write(true).create_new(true).open(dest.join("REGISTRO.jsonl"))?;
    let timer=Instant::now(); let start=SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let parser=MarkdownParser::new(); let profiles=[("original",CrossFileContradiction::new()),("should",CrossFileContradiction::experimental_should(false)),("should_coincidencia",CrossFileContradiction::experimental_should(true))];
    let mut cases=Vec::new();
    for (profile,rule) in &profiles { for c in &all {
        if *profile=="original" && c["conjunto"]!="adversarial" { continue; }
        let t=Instant::now(); let mut parsed=Vec::new(); let mut inputs=Vec::new();
        for d in arr(&c["documentos"])? { let doc=parser.parse(Path::new(s(&d["ruta"])?),s(&d["texto"])?)?; let mut reqs=Vec::new(); recognized(&doc.sections,&mut reqs); inputs.push(json!({"ruta":d["ruta"],"sha256":d["sha256"],"afirmaciones_reconocidas":reqs})); parsed.push(doc); }
        let findings=rule.check_corpus(&[&parsed[0],&parsed[1]]);
        let out:Vec<_>=findings.iter().map(|d| json!({"regla":d.rule_id.to_string(),"mensaje_original":d.message,"localizacion":d.span,"gravedad_original":format!("{:?}",d.severity),"confianza_declarada_no_calibrada":d.confidence})).collect();
        let row=json!({"perfil":profile,"id":c["id"],"dominio":c["dominio"],"conjunto":c["conjunto"],"documentos":inputs,"alerta":!out.is_empty(),"hallazgos":out,"duracion_microsegundos":t.elapsed().as_micros()});
        stream.write_all(&serde_json::to_vec(&row)?)?; stream.write_all(b"\n")?; stream.sync_all()?; cases.push(row);
    }}
    if cases.len()!=72 { return Err("Número de llamadas distinto del protocolo".into()); }
    save(dest.join("RESULTADO.json"),&json!({"elemento":"Buscador-Semántico - Diferencial","candidato":"speccheck-core 0.4.1; original y dos variantes experimentales de SC-XF-002","ejecutable_sha256":hash(&fs::read(std::env::current_exe()?)?),"entradas_cotejadas_sha256":hash(&fs::read(dest.join("ENTRADAS-COTEJADAS.json"))?),"inicio_unix_ms":start,"duracion_microsegundos":timer.elapsed().as_micros(),"llamadas":cases.len(),"referencias_de_evaluacion_recibidas":false,"ia_detector":false,"reescritura_entradas":false,"integracion_operativa":false,"casos":cases}))?;
    println!("72 llamadas conservadas; sin evaluación todavía"); Ok(())
}
fn evaluate(a: &[String]) -> R<()> {
    let actual=read(&a[2])?; let adv=read(&a[3])?; let old=read(&a[4])?;
    if arr(&actual["casos"])?.len()!=72 { return Err("Resultado incompleto".into()); }
    let mut groups=Vec::new(); let mut seen=BTreeSet::new();
    for profile in ["original","should","should_coincidencia"] { for (group,refs,n) in [("adversarial",&adv,16),("documental_conocido",&old,12)] {
        if profile=="original" && group!="adversarial" { continue; }
        if arr(&refs["casos"])?.len()!=n { return Err("Referencias incompletas".into()); }
        let mut totals=[0u32;6]; let mut rows=Vec::new(); let mut refids=BTreeSet::new();
        for r in arr(&refs["casos"])? {
            let id=s(&r["id"])?; if !refids.insert(id) { return Err("Referencia duplicada".into()); }
            let matches:Vec<_>=arr(&actual["casos"])?.iter().filter(|c| c["perfil"]==profile && c["conjunto"]==group && c["id"]==id).collect(); if matches.len()!=1 { return Err("Resultado ausente o duplicado".into()); }
            let c=matches[0]; if c["dominio"]!=r["dominio"] { return Err("Dominio distinto".into()); }
            let alert=c["alerta"].as_bool().ok_or("Alerta ausente")?;
            let i=match (s(&r["referencia"])?,alert) { ("incompatibilidad",true)=>0,("incompatibilidad",false)=>1,("compatibilidad",true)=>2,("compatibilidad",false)=>3,("contexto_insuficiente",true)=>4,("contexto_insuficiente",false)=>5,_=>return Err("Referencia no reconocida".into()) }; totals[i]+=1;
            seen.insert((profile.to_string(),group.to_string(),id.to_string())); rows.push(json!({"id":id,"dominio":r["dominio"],"referencia":r["referencia"],"alerta":alert}));
        }
        groups.push(json!({"perfil":profile,"conjunto":group,"detecciones":totals[0],"omisiones":totals[1],"avisos_en_compatibles":totals[2],"silencios_en_compatibles":totals[3],"avisos_en_contexto_insuficiente":totals[4],"silencios_en_contexto_insuficiente":totals[5],"casos":rows}));
    }}
    if seen.len()!=72 { return Err("Cobertura de evaluación incompleta".into()); }
    save(&a[5],&json!({"validacion_independiente":false,"resultado_sha256":hash(&fs::read(&a[2])?),"referencias_adversariales_sha256":hash(&fs::read(&a[3])?),"referencias_anteriores_sha256":hash(&fs::read(&a[4])?),"grupos":groups}))?;
    for g in groups { println!("{} / {}: detectadas {}, omitidas {}, avisos compatibles {}, avisos insuficientes {}",g["perfil"],g["conjunto"],g["detecciones"],g["omisiones"],g["avisos_en_compatibles"],g["avisos_en_contexto_insuficiente"]); } Ok(())
}
fn main() -> R<()> { let a:Vec<String>=std::env::args().collect(); match a.get(1).map(String::as_str) { Some("detectar") if a.len()==5=>detect(&a), Some("evaluar") if a.len()==6=>evaluate(&a), _=>Err("Uso: detectar controles.json traducciones destino | evaluar resultado.json referencias_nuevas.json referencias_previas.json salida.json".into()) } }
