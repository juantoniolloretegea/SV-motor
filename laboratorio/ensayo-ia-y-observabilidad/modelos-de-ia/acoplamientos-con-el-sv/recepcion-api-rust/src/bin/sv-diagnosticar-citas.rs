//! Diagnóstico exterior de citas. No cambia la auditoría, el contrato ni el dictamen.
#![forbid(unsafe_code)]
use serde_json::{json, Value};
use std::{fs, path::PathBuf};
use sv_cliente_api::{guard, need, parse, save, sha, R, LICENCIA};
fn norm(s: &str) -> String { s.split_whitespace().collect::<Vec<_>>().join(" ") }
fn compare(source: &str, quote: &str) -> (bool, bool) {
    let exact = norm(source).contains(&norm(quote));
    let without_stars = !exact && norm(&source.replace('*', "")).contains(&norm(&quote.replace('*', "")));
    (exact, without_stars)
}
fn load(p: &std::path::Path) -> R<Value> { guard(p)?; parse(&fs::read(p).map_err(|e| e.to_string())?) }
fn run() -> R<Value> {
    let a: Vec<_> = std::env::args().collect();
    need(a.len() == 3, "Uso: sv-diagnosticar-citas DIRECTORIO SALIDA")?;
    let base = PathBuf::from(&a[1]); let output = PathBuf::from(&a[2]); guard(&base)?; guard(&output)?;
    let mut dirs = fs::read_dir(base.join("intentos")).map_err(|e| e.to_string())?.map(|e| e.map(|x|x.path()).map_err(|e|e.to_string())).collect::<R<Vec<_>>>()?;
    dirs.sort(); let mut rows = vec![];
    for d in dirs {
        let ap = d.join("AUDITORIA-FORMAL.json"); if !ap.exists() {continue;}
        let formal = load(&ap)?; if formal["conforme"] == true {continue;}
        let raw = fs::read(d.join("FINAL.txt")).map_err(|e|e.to_string())?;
        let answer = parse(&raw)?; let request = load(&d.join("SOLICITUD.json"))?;
        let source = parse(request["input"][0]["content"].as_str().ok_or("Fuente ausente")?.as_bytes())?;
        let fragments = source["fragmentos_documentales_completos"].as_array().ok_or("Fragmentos ausentes")?;
        let mut discrepancies = vec![];
        for (index, cite) in answer["evidencias"].as_array().ok_or("Citas ausentes")?.iter().enumerate() {
            let ids = cite["fragmentos"].as_array().ok_or("Indices ausentes")?;
            let mut text = String::new(); let mut locators = !ids.is_empty(); let mut previous = None;
            for id in ids {
                let n = id.as_u64().ok_or("Indice no entero")?;
                if previous.is_some_and(|p| n != p + 1) {locators = false;}
                previous = Some(n);
                let found: Vec<_> = fragments.iter().filter(|f| f["seccion"] == cite["seccion"] && f["pagina"] == *id).collect();
                if found.len() != 1 {locators = false; continue;}
                text.push_str(found[0]["texto"].as_str().ok_or("Texto ausente")?);
            }
            let quote = cite["cita_literal_breve"].as_str().filter(|q| !q.trim().is_empty()).ok_or("Cita vacía")?;
            let (exact, stars) = compare(&text, quote);
            if !locators || !exact {discrepancies.push(json!({"indice_cita":index,"localizadores_consecutivos_encontrados":locators,"literal":exact,"coincide_tras_retirar_solo_asteriscos":stars,"cita":quote,"cita_sha256":sha(quote.as_bytes()),"texto_localizado_sha256":sha(text.as_bytes())}));}
        }
        rows.push(json!({"intento":d.file_name().and_then(|s|s.to_str()),"caso":answer["caso"],"etapa":answer["etapa"],"final_sha256":sha(&raw),"auditoria_original":formal,"discrepancias":discrepancies}));
    }
    let result = json!({"alcance":"Diagnóstico auxiliar exterior en Rust. La retirada de asteriscos no es una normalización admitida ni una adjudicación: requiere examinar el pasaje. Auditorías y originales intactos.","candidatos_con_incidencia":rows,"inferencias_nuevas":0,"autoria_licencia":LICENCIA});
    save(&output,&result)?; Ok(result)
}
fn main() { match run() {Ok(v)=>println!("{}",v), Err(e)=>{eprintln!("{e}");std::process::exit(1)}} }
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn distingue_tipografia_y_sentido() {assert_eq!(compare("Carecen de *BRAF*.","Carecen de BRAF."),(false,true)); assert_eq!(compare("No se recomienda.","Se recomienda."),(false,false));}
    #[test] fn solo_espacios_permitidos_en_cotejo_literal() {assert_eq!(compare("A  B\nC","A B C"),(true,false));assert_eq!(compare("abc","a bc"),(false,false));}
}
