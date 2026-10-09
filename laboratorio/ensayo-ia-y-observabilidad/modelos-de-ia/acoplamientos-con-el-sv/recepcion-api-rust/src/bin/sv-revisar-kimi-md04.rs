//! Diagnóstico local de suministro y recepción de MD04; no ejecuta inferencias.
#![forbid(unsafe_code)]
use serde_json::{json, Value};
use std::{fs, path::Path};
use sv_cliente_api::{need, parse, sha, R};
mod suministro_pdf {
    pub use sv_cliente_api::{need, parse, sha, R};
    pub fn num(v: &serde_json::Value) -> R<usize> {
        v.as_u64().and_then(|n| usize::try_from(n).ok()).ok_or("Entero requerido".into())
    }
}
#[path = "../../../cliente-api-rust/src/manual/localizadores.rs"] mod localizadores;
#[path = "../../../cliente-api-rust/src/manual/contrato.rs"] mod contrato;
fn raw(p: &Path) -> R<Vec<u8>> { sv_cliente_api::guard(p)?; fs::read(p).map_err(|e| e.to_string()) }
fn load(p: &Path) -> R<Value> { parse(&raw(p)?) }
fn main() {
    let result = (|| -> R<()> {
        let args = std::env::args().collect::<Vec<_>>();
        need(args.len() == 3, "Raíz original y salida diagnóstica")?;
        let root = Path::new(&args[1]); let out = Path::new(&args[2]);
        let receipt = load(&root.join("RECEPCION-RUST.json"))?;
        let base = load(&root.join("fuentes-admitidas/MD04.json"))?;
        let previa = load(&root.join("PREVIA.json"))?;
        for name in ["CRITERIOS.md", "ADMISION.md"] {
            let frozen = previa["archivos"].as_array().ok_or("Inventario previo")?.iter()
                .find(|v| v["ruta"].as_str().is_some_and(|p| p.ends_with(name))).ok_or("Contrato no fijado")?;
            need(frozen["sha256"] == sha(&raw(&root.join(name))?), "Contrato modificado")?;
        }
        let mut originals = vec![]; let mut rows = vec![]; let mut history = vec![];
        for stage in 0..3 {
            let dir = root.join(format!("intentos/I{:03}-MD04-R{stage}", 10 + stage));
            for name in ["SOLICITUD.json", "SALIDA-SSE.txt", "FINAL.txt", "AUDITORIA-FORMAL.json", "HTTP.json", "RESULTADO.json", "ENVIO.json", "ENTREGA-PROVEEDOR.json"] {
                let path = dir.join(name); originals.push((path.clone(), sha(&raw(&path)?)));
            }
            let request_bytes = raw(&dir.join("SOLICITUD.json"))?;
            let req = parse(&request_bytes)?; sv_cliente_api::kimi::validar(&req)?;
            need(load(&dir.join("ENVIO.json"))?["solicitud_sha256"] == sha(&request_bytes), "Solicitud distinta de la registrada al enviar")?;
            need(req["messages"][1]["content"] == base["input"][0]["content"], "Suministro modificado")?;
            let messages = req["messages"].as_array().ok_or("Mensajes")?;
            let prior = messages.iter().filter(|m| m["role"] == "assistant").map(|m| m["content"].as_str().unwrap().to_owned()).collect::<Vec<_>>();
            need(prior == history, "Antecedentes textuales alterados")?;
            let source = parse(messages[1]["content"].as_str().unwrap().as_bytes())?;
            need(source["pregunta"] == "¿Qué doce campos debe contener la ficha de un tramo? Distinga las obligaciones qué fija y qué no fija.", "Enunciado discordante")?;
            let fragments = source["fragmentos_documentales_completos"].as_array().ok_or("Fragmentos")?;
            need(fragments.len() == 31, "Suministro incompleto")?;
            let section = fragments.iter().find(|s| s["documento"] == "MANUAL_CONSTRUCTOR" && s["seccion"] == "MD-L000109-L000118").ok_or("Criterio de cierre ausente")?;
            let text = section["texto"].as_str().ok_or("Texto de cierre")?;
            need(section["inicio_caracter"] == 0 && section["siguiente_pagina"].is_null() && section["fin_caracter_exclusivo"] == text.chars().count(), "Sección incompleta")?;
            need(text.lines().filter(|s| s.starts_with("- ")).count() == 5 && text.contains("Un tramo se considerará cerrable cuando reúna, como mínimo:"), "Cinco condiciones no suministradas")?;
            let bytes = raw(&dir.join("FINAL.txt"))?; let answer = parse(&bytes)?;
            let sse = raw(&dir.join("SALIDA-SSE.txt"))?;
            let mut stream = sv_cliente_api::chat::Flujo::default(); stream.feed(&sse)?;
            let got = stream.recibir("kimi-k3")?;
            need(got["texto_original"].as_str().unwrap().as_bytes() == bytes, "Reconstrucción discordante")?;
            need(load(&dir.join("ENTREGA-PROVEEDOR.json"))?["texto_original"] == got["texto_original"], "Entrega archivada discordante")?;
            need(load(&dir.join("HTTP.json"))?["status"] == 200, "HTTP incompleto")?;
            let old = receipt["casos"].as_array().unwrap().iter().find(|c| c["caso"] == "MD04" && c["etapa"] == stage).ok_or("Recepción ausente")?;
            need(old["final_sha256"] == sha(&bytes), "Respuesta diferente de la recibida")?;
            let q = json!({"input": &messages[1..]}); contrato::formal(&answer, &q)?;
            let mut variant = answer.clone(); variant["insuficiencias"] = json!([]);
            contrato::formal(&variant, &q)?;
            let insuff = answer["insuficiencias"].as_array().unwrap().iter().find_map(Value::as_str).ok_or("No hay afirmación de insuficiencia")?;
            need(insuff.contains("Criterio de cierre") && insuff.contains("más allá de su denominación"), "Afirmación examinada ausente")?;
            let cites_closure = answer["evidencias"].as_array().unwrap().iter().any(|c| c["seccion"] == "MD-L000109-L000118");
            let capa = load(&root.join(format!("adjudicacion/CAPA-R{stage}.json")))?;
            let judged = capa["casos"].as_array().unwrap().iter().find(|c| c["caso"] == "MD04").ok_or("Adjudicación ausente")?;
            need(judged["final_sha256"] == sha(&bytes) && judged["critico"] == false && capa["vector"][3] == "1", "Correspondencia con el vector discordante")?;
            rows.push(json!({"etapa":stage,"solicitud_sha256":sha(&request_bytes),"sse_sha256":sha(&sse),"final_sha256":sha(&bytes),"suministro_identico_al_admitido":true,"fragmentos":fragments.len(),"enunciado":source["pregunta"],"seccion_completa":section,"http":200,"reconstruccion_sse_identica":true,"terminacion":"stop y DONE comprobados","antecedentes_textuales_identicos":true,"formal_conforme":true,"insuficiencias_vacias_admitidas_en_variante_solo_en_memoria":true,"afirmacion_objeto_revision":insuff,"respuesta_cita_el_apartado_de_cierre":cites_closure,"vector_corresponde_a_respuesta_original":true,"critico":false}));
            history.push(String::from_utf8(bytes).map_err(|e|e.to_string())?);
        }
        for (path, hash) in &originals { need(sha(&raw(path)?) == *hash, "Original alterado durante comprobación")?; }
        sv_cliente_api::save(out, &json!({"conforme":true,"objeto":"Diagnóstico MD04: suministro, transporte, contrato y correspondencia de adjudicación","etapas":rows,"originales_cotejados_antes_y_despues":originals.len(),"originales_modificados":0,"llamadas_proveedor":0,"limites":"El cotejo no observa procesos internos remotos ni determina causalidad psicológica; la interpretación sustantiva se conserva en informe exterior separado. La variante vacía prueba únicamente admisibilidad estructural; no es respuesta nueva ni recalificación.","licencia":sv_cliente_api::LICENCIA}))?;
        println!("Conforme: tres entregas, 24 originales intactos; suministro completo, reconstrucción idéntica y ausencia de insuficiencias admisible. Cero llamadas."); Ok(())
    })();
    if let Err(e) = result { eprintln!("{e}"); std::process::exit(1); }
}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
