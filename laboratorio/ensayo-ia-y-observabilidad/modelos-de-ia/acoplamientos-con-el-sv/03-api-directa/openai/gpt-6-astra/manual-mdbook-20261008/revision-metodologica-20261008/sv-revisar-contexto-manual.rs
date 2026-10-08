//! Revisión documental local. No abre red, no llama al candidato ni modifica originales.
//! © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

type R<T> = Result<T, Box<dyn std::error::Error>>;
const BASE: &str = "ejecucion/astra-manual-20261008";
const OUT: &str = "seguimiento/revision-md01-20261008/COTEJO-RUST.json";
fn sha(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
fn load(p: &Path) -> R<Value> { Ok(serde_json::from_slice(&fs::read(p)?)?) }
fn check(ok: bool, msg: &str) -> R<()> { if ok { Ok(()) } else { Err(msg.into()) } }
fn text(v: &Value) -> R<&str> { v.as_str().ok_or_else(|| "Cadena ausente".into()) }
fn row<'a>(receipt: &'a Value, n: usize) -> R<&'a Value> {
    receipt["casos"].as_array().ok_or("Recepción sin casos")?.iter()
        .find(|v| v["entrega"] == n).ok_or_else(|| "Entrega ausente".into())
}
fn same_history(q: &Value, originals: &[String]) -> bool {
    let Some(input) = q["input"].as_array() else { return false };
    let got = input.iter().filter(|v| v["role"] == "assistant")
        .map(|v| v["content"].as_str()).collect::<Vec<_>>();
    got == originals.iter().map(|s| Some(s.as_str())).collect::<Vec<_>>()
}
fn main() -> R<()> {
    let base = Path::new(BASE);
    let receipt = load(&base.join("RECEPCION-RUST.json"))?;
    let previa = load(&base.join("PREVIA.json"))?;
    let mut fixed = vec![];
    for entry in previa["archivos_fijados"].as_array().ok_or("Fijación ausente")? {
        let p = text(&entry["ruta"])?;
        let bytes = fs::read(p)?;
        let ok = entry["sha256"] == sha(&bytes) && entry["bytes"] == bytes.len();
        check(ok, "Un archivo fijado ha cambiado")?;
        fixed.push(json!({"ruta":p.replace('\\',"/"),"sha256":sha(&bytes),"bytes":bytes.len(),"identico":ok}));
    }
    let mut cases = vec![];
    for case in 1..=9 {
        let mut requests = vec![];
        let mut originals = vec![];
        let mut stages = vec![];
        for stage in 0..3 {
            let n = (case - 1) * 3 + stage + 1;
            let path = base.join(format!("intentos/ENVIO-{n:03}/MD{case:02}/R{stage}"));
            let request_bytes = fs::read(path.join("SOLICITUD.json"))?;
            let final_bytes = fs::read(path.join("FINAL.txt"))?;
            let q: Value = serde_json::from_slice(&request_bytes)?;
            let f: Value = serde_json::from_slice(&final_bytes)?;
            let audit = load(&path.join("AUDITORIA-FORMAL.json"))?;
            let r = row(&receipt,n)?;
            check(r["solicitud_sha256"] == sha(&request_bytes) && r["final_sha256"] == sha(&final_bytes), "Original y recepción difieren")?;
            check(r["auditoria_sha256"].is_null() || r["auditoria_sha256"] == sha(&fs::read(path.join("AUDITORIA-FORMAL.json"))?), "Huella de auditoría discordante")?;
            check(f["etapa"] == stage && f["caso"] == format!("MD{case:02}"), "Identidad o etapa incorrecta")?;
            check(audit["conforme"] == true && r["formal_conforme"] == true && r["http"] == 200 && r["completa"] == true, "Entrega sin conformidad registrada")?;
            check(q["store"] == false && q.get("previous_response_id").is_none() && q["tools"] == json!([]) && q["tool_choice"] == "none", "Estado o herramientas inesperados")?;
            check(same_history(&q,&originals), "Historia de respuestas alterada")?;
            check(text(&q["instructions"])?.contains(&format!("etapa debe ser {stage}.")), "Regla de etapa ausente")?;
            stages.push(json!({"etapa":stage,"solicitud_sha256":sha(&request_bytes),"final_sha256":sha(&final_bytes),"auditoria_formal_sha256":sha(&fs::read(path.join("AUDITORIA-FORMAL.json"))?),"etapa_conforme":true,"historia_respuestas_exacta":true,"formal_conforme_registrado":true,"revision":f["revision"],"razonamiento_solicitado":q["reasoning"]}));
            requests.push(q);
            originals.push(String::from_utf8(final_bytes)?);
        }
        check(requests[0]["input"][0] == requests[1]["input"][0] && requests[0]["input"][0] == requests[2]["input"][0], "Fuente o pregunta alteradas")?;
        let real_r1 = &requests[1]["input"][2]["content"];
        let replay_r1 = &requests[2]["input"][2]["content"];
        check(real_r1 != replay_r1, "El historial no presenta la sustitución esperada; revisar hallazgo")?;
        let r2_string = serde_json::to_string(&requests[2])?;
        let historical_rules_absent = !r2_string.contains("etapa debe ser 0.") && !r2_string.contains("etapa debe ser 1.");
        check(historical_rules_absent, "Hay instrucciones históricas no previstas; revisar hallazgo")?;
        cases.push(json!({"caso":format!("MD{case:02}"),"corpus_pregunta_identicos":true,"etapas":stages,"encargo_r1_original":real_r1,"encargo_r1_representado_en_r2":replay_r1,"encargo_r1_sustituido":true,"reglas_historicas_etapa_ausentes_en_r2":true,"store":false,"previous_response_id":null}));
    }
    let key = load(&base.join("reservado/CLAVE.json"))?;
    let out = json!({"version":"1.0.0","fecha":"2026-10-08","alcance":"Cotejo documental retrospectivo local; sin nueva inferencia ni adjudicación automática", "conforme":true,"archivos_prefijados_identicos":fixed,"entregas_cotejadas":27,"casos":cases,"criterio_prefijado_md01":key["preguntas"][0],"fuentes_instrumentales":[{"ruta":"PREVIA.json","sha256":sha(&fs::read(base.join("PREVIA.json"))?)},{"ruta":"RECEPCION-RUST.json","sha256":sha(&fs::read(base.join("RECEPCION-RUST.json"))?)}],"limites":["La conformidad formal y de transporte se coteja con la recepción conservada; este programa no vuelve a interpretar SSE ni prueba la infraestructura del proveedor.","La interpretación de la ambigüedad y del alcance de la criticidad requiere revisión sustantiva, no queda demostrada por la coincidencia de bytes.","No se demuestra causalidad exclusiva ni se cambia ningún valor del vector histórico."],"inferencias_nuevas":0,"tokens_candidato_nuevos":0});
    let dest = Path::new(OUT);
    fs::create_dir_all(dest.parent().ok_or("Destino sin padre")?)?;
    let mut bytes = serde_json::to_vec_pretty(&out)?; bytes.push(b'\n');
    if dest.exists() { check(fs::read(dest)? == bytes,"Resultado preexistente distinto; no se sobrescribe")?; }
    else { use std::io::Write; fs::OpenOptions::new().write(true).create_new(true).open(dest)?.write_all(&bytes)?; }
    println!("{}",json!({"conforme":true,"archivo":OUT,"sha256":sha(&bytes),"entregas":27,"historiales_r2_con_encargo_r1_sustituido":9,"inferencias_nuevas":0}));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn detecta_alteracion_y_reordenacion_del_historial() {
        let q = json!({"input":[{"role":"user","content":"fuente"},{"role":"assistant","content":"R0"},{"role":"user","content":"revision"},{"role":"assistant","content":"R1"}]});
        assert!(same_history(&q,&["R0".into(),"R1".into()]));
        assert!(!same_history(&q,&["R1".into(),"R0".into()]));
        assert!(!same_history(&q,&["R0 modificado".into(),"R1".into()]));
        assert!(!same_history(&q,&["R0".into()]));
    }
}
