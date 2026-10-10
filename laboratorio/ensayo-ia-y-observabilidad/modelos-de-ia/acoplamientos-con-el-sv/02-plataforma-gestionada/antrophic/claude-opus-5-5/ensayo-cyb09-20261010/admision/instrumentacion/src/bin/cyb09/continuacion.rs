use super::*;
pub(super) const SELECCION: [&str; 9] = ["C01", "C02", "C03", "C05", "C06", "C08", "C10", "C11", "C16"];
pub(super) const CONSERVADAS: [(&str, usize); 4] = [("C01", 0), ("C01", 1), ("C01", 2), ("C02", 0)];

pub(super) fn leer(p: &Path) -> R<Value> { parse(&fs::read(p).map_err(|e| e.to_string())?) }

pub(super) fn cotejar_original(root: &Path, manifest: &Value, id: &str, stage: usize, q: &Value, w: &Value) -> R<Value> {
    let directory = format!("campana/cyb16/{id}-R{stage}");
    let entries = manifest.as_array().ok_or("Manifiesto de recuperación ausente")?;
    let selected: Vec<_> = entries.iter().filter(|v| v["ruta"].as_str().is_some_and(|r| r.starts_with(&format!("{directory}/")))).collect();
    need(selected.len() >= 12, "Originales de una entrega incompletos")?;
    for entry in selected {
        let name = entry["ruta"].as_str().ok_or("Ruta ausente")?;
        need(!name.contains("..") && !name.contains('\\') && !name.starts_with('/'), "Ruta fuera del perímetro")?;
        let path = root.join(name);
        let meta = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
        need(meta.is_file() && !meta.file_type().is_symlink(), "Original no regular")?;
        let bytes = fs::read(path).map_err(|e| e.to_string())?;
        need(entry["bytes"].as_u64() == Some(bytes.len() as u64) && entry["sha256"] == sha(&bytes), "Original distinto de la custodia fijada")?;
    }
    let d = root.join(directory);
    need(leer(&d.join("SOLICITUD-DOCUMENTAL.json"))? == *q && leer(&d.join("SOLICITUD.json"))? == *w, "Solicitud o historial original diferentes")?;
    let raw = fs::read(d.join("RESPUESTA-HTTP.json")).map_err(|e| e.to_string())?;
    let body = parse(&raw)?;
    admitir(&body)?;
    need(leer(&d.join("HTTP.json"))?["status"] == 200, "HTTP original no conforme")?;
    let original = body["choices"][0]["message"]["content"].as_str().ok_or("Texto original ausente")?;
    let (final_text, reasoning, segmentation) = segmentar(original)?;
    for (name, expected) in [("CONTENIDO-ORIGINAL.txt", original), ("FINAL.txt", final_text.as_str()), ("RAZONAMIENTO-THINK.txt", reasoning.as_str())] {
        need(fs::read(d.join(name)).map_err(|e| e.to_string())? == expected.as_bytes(), "Texto original alterado")?;
    }
    need(leer(&d.join("SEGMENTACION.json"))? == segmentation, "Segmentación distinta")?;
    need(leer(&d.join("RAZONAMIENTO-CANALES.json"))? == json!({"reasoning_content":body["choices"][0]["message"]["reasoning_content"],"reasoning":body["choices"][0]["message"]["reasoning"]}), "Canal de razonamiento perdido")?;
    let row = leer(&d.join("RESULTADO.json"))?;
    let telemetry = sv::instrumentacion::verify(&d.join("instrumentacion/telemetria.jsonl"))?;
    need(row["completa"] == true && row["error"].is_null() && row["uso_proveedor"] == body["usage"] && row["coste_nanodolares"] == coste(&body)?, "Resultado original discordante")?;
    need(row["telemetria"] == telemetry && telemetry["fallos_medicion"] == 0 && telemetry["intervalo_maximo_ms"].as_u64().is_some_and(|v| v <= 750), "Observación original incompleta")?;
    let events = sv::instrumentacion::records(&d.join("instrumentacion/telemetria.jsonl"))?;
    need(events.iter().any(|v| v["tipo"] == "recepcion" && v["datos"]["respuesta_http_sha256"] == sha(&raw)), "Recepción no correlacionada")?;
    let wire_sha = sha(&serde_json::to_vec(w).map_err(|e| e.to_string())?);
    need(events.iter().any(|v| v["tipo"] == "envio" && v["datos"]["solicitud_sha256"] == wire_sha), "Envío no correlacionado")?;
    let formal = parse(final_text.as_bytes()).and_then(|a| sv::formal(&a, q)).unwrap_or_else(|e| json!({"conforme":false,"defecto":e}));
    need(formal == row["auditoria_formal"], "Auditoría formal discordante")?;
    Ok(row)
}

pub(super) fn comprobar_paquete(p: &Value, original: &Path) -> R<Value> {
    need(p["modelo"] == sv::MODELO && p["version"] == "claude-cyb09-1.0" && p["original_paquete_sha256"] == "758b9bac230b81410df73795c7c74ae3089987bfb8aecc7d53917c50896f749d", "Identidad de contrato discordante")?;
    need(p["max_tokens"] == 16384 && p["reintentos_automaticos"] == 0 && p["nuevas_entregas"] == 23 && p["limite_nanodolares"] == 9_990_000_000u64 && p["limite_banco_s"] == 5400, "Límites no autorizados")?;
    let cases = p["casos"].as_array().ok_or("Casos ausentes")?;
    need(cases.len() == 9, "Selección incompleta")?;
    let mut recovered = Vec::new();
    for (case, id) in cases.iter().zip(SELECCION) {
        need(case["id"] == id, "Orden o identidad alterados")?;
        let mut history = Vec::new();
        for stage in 0..3 {
            let (q, w) = sv::compose(&case["base"], stage, &history)?;
            sv::observabilidad_modelo::comprobar_solicitud(&q)?;
            need(w["model"] == sv::MODELO && w["stream"] == false && w["reasoning_effort"] == "high" && w.get("tools").is_none() && w.get("response_format").is_none(), "Frontera de transporte cambiada")?;
            reserva(&w)?;
            if CONSERVADAS.contains(&(id, stage)) {
                let row = cotejar_original(original, &p["originales"], id, stage, &q, &w)?;
                recovered.push(json!({"caso":id,"etapa":stage,"resultado":row,"nueva_inferencia":false}));
                history.push(fs::read_to_string(original.join(format!("campana/cyb16/{id}-R{stage}/FINAL.txt"))).map_err(|e| e.to_string())?);
            } else { history.push("Antecedente sintético de comprobación; no se utiliza para inferir".into()); }
        }
    }
    need(recovered.len() == 4, "Recuperación incompleta")?;
    Ok(json!({"conforme":true,"composiciones":27,"originales_recuperados":recovered,"nuevas_inferencias":0,"nuevas_entregas_previstas":23,"peticion_operativa_presente":true,"nucleo_semantica_ir_modificados":false,"licencia":sv::LICENCIA}))
}

pub(super) fn cuota_permite(spent: u64, reserved: u64, limit: u64) -> bool { spent.checked_add(reserved).is_some_and(|v| v <= limit) }

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn excluye_repeticiones_y_conserva_orden() {
        let new: Vec<_> = SELECCION.iter().flat_map(|id| (0..3).map(move |stage| (*id, stage))).filter(|x| !CONSERVADAS.contains(x)).collect();
        assert_eq!(new.len(), 23); assert_eq!(new[0], ("C02", 1)); assert_eq!(new.last(), Some(&("C16", 2)));
        assert!(!new.iter().any(|(id, _)| ["C04","C07","C09","C12","C13","C14","C15"].contains(id)));
    }
    #[test] fn cuota_separa_historia_y_rechaza_desbordamiento() {
        assert!(cuota_permite(7_864_776_000, 1_000_000_000, 9_990_000_000));
        assert!(!cuota_permite(9_900_000_000, 100_000_001, 10_000_000_000));
        assert!(!cuota_permite(u64::MAX, 1, u64::MAX));
    }
}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
