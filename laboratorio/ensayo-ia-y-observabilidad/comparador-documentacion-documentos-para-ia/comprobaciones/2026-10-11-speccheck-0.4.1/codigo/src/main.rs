//! Comprobación aislada de un candidato para el Buscador-Semántico - Diferencial.
//! No modifica el SV ni aplica decisiones documentales. No recibe referencias.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use speccheck_core::parser::{MarkdownParser, SpecParser};
use speccheck_core::rules::contradiction::{DirectNegation, NumericConflict, TemporalConflict, RequirementTension, SatContradiction};
use speccheck_core::rules::cross_file::CrossFileContradiction;
use speccheck_core::rules::{AnalysisContext, Rule};
use speccheck_core::Section;
use std::{error::Error, fs, path::{Path, Component}, time::{Instant, SystemTime, UNIX_EPOCH}};

fn hash(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }
fn count(sections: &[Section]) -> usize {
    sections.iter().map(|s| s.requirements.len() + count(&s.subsections)).sum()
}
fn relative(s: &str) -> bool {
    !s.is_empty() && Path::new(s).components().all(|c| matches!(c, Component::Normal(_)))
}
fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 { return Err("Uso: comprobacion ENTRADAS.json DIRECTORIO SALIDA.json".into()); }
    if Path::new(&args[3]).exists() { return Err("La salida ya existe; se preserva".into()); }
    let plan_bytes = fs::read(&args[1])?;
    let plan: Value = serde_json::from_slice(&plan_bytes)?;
    if plan.get("referencias").is_some() { return Err("El detector no admite referencias".into()); }
    let cases = plan["casos"].as_array().ok_or("casos ausentes")?;
    let rules: Vec<Box<dyn Rule>> = vec![Box::new(DirectNegation::new()), Box::new(NumericConflict::new()),
        Box::new(TemporalConflict::new()), Box::new(RequirementTension::new()),
        Box::new(SatContradiction::new()), Box::new(CrossFileContradiction::new())];
    let rule_ids: Vec<String> = rules.iter().map(|r| r.id().to_string()).collect();
    let parser = MarkdownParser::new();
    let mut results = Vec::new();
    let start = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let total = Instant::now();
    for case in cases {
        let timer = Instant::now();
        let files = case["archivos"].as_array().ok_or("archivos ausentes")?;
        let mut docs = Vec::new();
        let mut readings = Vec::new();
        for f in files {
            let name = f["ruta"].as_str().ok_or("ruta ausente")?;
            if !relative(name) { return Err("Ruta no relativa o con ascenso".into()); }
            let bytes = fs::read(Path::new(&args[2]).join(name))?;
            let digest = hash(&bytes);
            if f["sha256"].as_str() != Some(&digest) { return Err(format!("Huella distinta: {name}").into()); }
            let content = String::from_utf8(bytes)?;
            let doc = parser.parse(Path::new(name), &content)?;
            readings.push(json!({"ruta":name,"sha256":digest,"bytes":content.len(),"requisitos_reconocidos":count(&doc.sections)}));
            docs.push(doc);
        }
        let mut diagnostics = Vec::new();
        for doc in &docs {
            let ctx = AnalysisContext::new(&doc.path, &doc.raw_text, false);
            assert!(ctx.nli_detector.is_none() && ctx.grammar_checker.is_none());
            for rule in &rules { diagnostics.extend(rule.check_document(doc, &ctx)); }
        }
        let refs: Vec<_> = docs.iter().collect();
        for rule in &rules { diagnostics.extend(rule.check_corpus(&refs)); }
        let findings: Vec<Value> = diagnostics.iter().map(|d| json!({
            "regla":d.rule_id.to_string(),"mensaje_original":d.message,"gravedad_original":format!("{:?}",d.severity),
            "confianza_declarada_no_calibrada":d.confidence,"localizacion":d.span,
            "sugerencia_original":d.fix.as_ref().map(|f|json!({"mensaje":f.message,"sustitucion":f.replacement,"localizacion":f.span}))
        })).collect();
        results.push(json!({"id":case["id"],"archivos":readings,"alerta":!findings.is_empty(),"hallazgos":findings,"duracion_microsegundos":timer.elapsed().as_micros()}));
    }
    let exe = std::env::current_exe()?;
    let report = json!({"elemento":"Buscador-Semántico - Diferencial","estado":"comprobacion_aislada_de_candidato",
        "candidato":"speccheck-core 0.4.1","reglas":rule_ids,"ia_activada":false,"referencias_recibidas":false,
        "correcciones_aplicadas":false,"autorizacion_corpus":false,"entradas_sha256":hash(&plan_bytes),
        "ejecutable_sha256":hash(&fs::read(exe)?),"inicio_unix_ms":start,"duracion_microsegundos":total.elapsed().as_micros(),
        "casos":results});
    fs::write(&args[3], serde_json::to_vec_pretty(&report)?)?;
    println!("{} casos comprobados; salida conservada; sin decisión sobre el corpus", cases.len());
    Ok(())
}
fn main(){ if let Err(e)=run(){ eprintln!("INCIDENCIA: {e}"); std::process::exit(1); } }
