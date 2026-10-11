use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    fs,
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

type R<T> = Result<T, String>;
const ATTRIBUTION:&str="© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(p: &Path) -> R<Value> {
    serde_json::from_slice(&fs::read(p).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}
fn write(p: &Path, v: &Value) -> R<()> {
    fs::create_dir_all(p.parent().ok_or("Ruta sin directorio")?).map_err(|e| e.to_string())?;
    fs::write(p, serde_json::to_vec_pretty(v).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}
fn text<'a>(v: &'a Value, k: &str) -> R<&'a str> {
    v[k].as_str().ok_or(format!("Falta texto: {k}"))
}
fn array<'a>(v: &'a Value, k: &str) -> R<&'a Vec<Value>> {
    v[k].as_array().ok_or(format!("Falta lista: {k}"))
}
fn find<'a>(a: &'a [Value], id: &str) -> R<&'a Value> {
    let found: Vec<_> = a.iter().filter(|v| v["id"] == id).collect();
    if found.len() != 1 {
        return Err(format!("Identificador ausente o duplicado: {id}"));
    }
    Ok(found[0])
}
fn number(v: &Value, k: &str) -> R<Option<u64>> {
    let x = v.get(k).ok_or(format!("Falta campo: {k}"))?;
    if x.is_null() {
        Ok(None)
    } else {
        x.as_u64().map(Some).ok_or(format!("Valor inválido: {k}"))
    }
}
fn prepare(root: &Path, out: &Path) -> R<()> {
    if out.exists() {
        return Err("La preparación ya existe; no se sobrescribe".into());
    }
    let prior = root.join("seguimiento/buscador-semantico-diferencial-20261011-paso3");
    let cp = root.join("seguimiento/bsd-correspondencias-20261011/resultado/CORRESPONDENCIAS.json");
    let rp = root.join(
        "seguimiento/buscador-semantico-diferencial-20261011-oposicion/ejecucion/RESULTADO.json",
    );
    let corr = read(&cp)?;
    let result = read(&rp)?;
    let entries = read(&prior.join("ENTRADAS.json"))?;
    let expected = read(&prior.join("REFERENCIAS.json"))?;
    let cases = array(&entries, "casos")?;
    if cases.len() != 12 {
        return Err("Se requieren los doce casos conservados".into());
    }
    // Asignación deliberada y fijada antes de la revisión. No es aleatorización.
    // Incluye el único aviso documental existente en el grupo con ayuda.
    let order = [
        "I02", "C02", "I03", "C03", "I01", "C01", "I04", "C05", "I05", "C04", "I06", "C06",
    ];
    let aided = ["I01", "I04", "I06", "C02", "C03", "C04"];
    let mut plan = Vec::new();
    let mut refs = Vec::new();
    let mut identities = Vec::new();
    let mut summary = String::from("# Resumen\n\n");
    for (i, id) in order.iter().enumerate() {
        let entry = find(cases, id)?;
        let c = find(array(&corr, "casos")?, id)?;
        let reference = find(array(&expected, "casos")?, id)?;
        if c["dominio"] != entry["dominio"] || reference["dominio"] != c["dominio"] {
            return Err("Dominio discordante".into());
        }
        let r = result["casos"]
            .as_array()
            .ok_or("Resultados ausentes")?
            .iter()
            .find(|v| {
                v["id"] == *id
                    && v["perfil"] == "should_coincidencia"
                    && v["conjunto"] == "documental_conocido"
            })
            .ok_or("Resultado previo ausente")?;
        let proposed_path = prior.join(text(&entry["propuesto"], "ruta")?);
        let proposed = fs::read_to_string(proposed_path).map_err(|e| e.to_string())?;
        let docs = array(c, "documentos")?;
        if docs.len() != 2 || docs[0]["original_documento"]["sha256"] != hash(proposed.as_bytes()) {
            return Err("Original propuesto discordante".into());
        }
        let ref_id = entry["seleccion"]["ids"][0]
            .as_str()
            .ok_or("Referencia ausente")?;
        let reference_text = fs::read_to_string(prior.join(format!("referencias/{ref_id}.md")))
            .map_err(|e| e.to_string())?;
        for (d, original) in docs.iter().zip([&proposed, &reference_text]) {
            if d["ingles_sha256"] != hash(text(d, "ingles_completo")?.as_bytes()) {
                return Err("Traducción discordante".into());
            }
            for b in array(d, "bloques")? {
                let s = text(b, "original")?;
                if !original.contains(s) || b["original_sha256"] != hash(s.as_bytes()) {
                    return Err("Pasaje original discordante".into());
                }
            }
        }
        let condition = if aided.contains(id) {
            "con_ayuda"
        } else {
            "ordinaria"
        };
        let material = json!({"id":id,"orden":i+1,"dominio":entry["dominio"],"condicion":condition,
            "naturaleza":"caso experimental conservado; no constituye recomendación clínica ni de seguridad",
            "documentos":[{"funcion":"propuesto","original_es":proposed,"traduccion_en":docs[0]["ingles_completo"],"procedencia":docs[0]["bloques"]},
                           {"funcion":"referencia","id":ref_id,"original_es":reference_text,"traduccion_en":docs[1]["ingles_completo"],"procedencia":docs[1]["bloques"]}],
            "ayuda": if condition == "con_ayuda" {json!({"perfil":"should_coincidencia","avisos":r["hallazgos"],"localizaciones":c["localizaciones"],"nota":"La ausencia de aviso no acredita compatibilidad; la otra localización no se deduce del mensaje abreviado."})} else {Value::Null}});
        let path = format!("participante/{id}.json");
        write(&out.join(&path), &material)?;
        let mut page=format!("# Caso {id}\n\nDominio: {}. Modalidad: {condition}.\n\nMaterial experimental: no constituye una recomendación clínica ni de seguridad.\n\n",text(entry,"dominio")?);
        for d in array(&material, "documentos")? {
            page.push_str(&format!("## Documento {}\n\n### Original español\n\n~~~~\n{}\n~~~~\n\n### Traducción conservada\n\n~~~~\n{}\n~~~~\n\n",text(d,"funcion")?,text(d,"original_es")?,text(d,"traduccion_en")?));
            for b in array(d, "procedencia")? {
                page.push_str(&format!(
                    "Procedencia: {}. {}\n\n",
                    b["fuente"]["titulo"]
                        .as_str()
                        .unwrap_or("Fuente conservada"),
                    text(b, "derechos")?
                ));
            }
        }
        if condition == "con_ayuda" {
            page.push_str(&format!(
                "## Sugerencias conservadas\n\n~~~~json\n{}\n~~~~\n",
                serde_json::to_string_pretty(&material["ayuda"]).map_err(|e| e.to_string())?
            ));
        }
        page.push_str(&format!("\nLa evaluación se limita al contexto íntegro suministrado en este caso.\n\nEl pie corresponde a la presentación propia; los textos citados conservan los derechos indicados.\n\n{ATTRIBUTION}\n"));
        let md = format!("visor/src/{id}.md");
        fs::create_dir_all(out.join("visor/src")).map_err(|e| e.to_string())?;
        fs::write(out.join(&md), &page).map_err(|e| e.to_string())?;
        identities.push(json!({"ruta":md,"bytes":page.len(),"sha256":hash(page.as_bytes())}));
        summary.push_str(&format!("- [Caso {id}]({id}.md)\n"));
        let bytes = fs::read(out.join(&path)).map_err(|e| e.to_string())?;
        identities.push(json!({"ruta":path,"bytes":bytes.len(),"sha256":hash(&bytes)}));
        plan.push(json!({"id":id,"orden":i+1,"dominio":entry["dominio"],"condicion":condition,"archivo":path,
            "avisos_entregados":if condition=="con_ayuda" {array(r,"hallazgos")?.len()} else {0}}));
        refs.push(json!({"id":id,"dominio":entry["dominio"],"referencia":reference["referencia"],"fundamento":reference["fundamento"],
            "criticidad":if reference["referencia"]=="incompatibilidad" {"alta_en_el_ensayo"} else {"no_aplica"}}));
    }
    let p = json!({"esquema":"bsd-utilidad/1","casos":plan,"tipo":"exploratorio; asignación deliberada, sin repetición de casos","registro":[],"atribucion":ATTRIBUTION});
    let key = json!({"referencias_preliminares":true,"casos":refs,"atribucion":ATTRIBUTION});
    for condition in ["ordinaria", "con_ayuda"] {
        let selected: Vec<_> = array(&p, "casos")?
            .iter()
            .filter(|v| v["condicion"] == condition)
            .collect();
        for (class, count) in [
            ("incompatibilidad", 3),
            ("compatibilidad", 2),
            ("contexto_insuficiente", 1),
        ] {
            if selected
                .iter()
                .filter(|v| {
                    find(array(&key, "casos").unwrap(), v["id"].as_str().unwrap()).unwrap()
                        ["referencia"]
                        == class
                })
                .count()
                != count
            {
                return Err("Asignación desequilibrada".into());
            }
        }
        for domain in ["inmunologia", "ciberseguridad"] {
            if selected.iter().filter(|v| v["dominio"] == domain).count() != 3 {
                return Err("Dominios desequilibrados".into());
            }
        }
    }
    write(&out.join("PLAN.json"), &p)?;
    write(&out.join("evaluacion/REFERENCIAS.json"), &key)?;
    write(&out.join("REGISTRO-VACIO.json"), &json!({"sesiones":[]}))?;
    fs::write(out.join("visor/src/SUMMARY.md"), summary).map_err(|e| e.to_string())?;
    fs::write(out.join("visor/book.toml"),"[book]\ntitle = \"Buscador-Semántico - Diferencial: revisión experimental\"\nlanguage = \"es\"\nsrc = \"src\"\n[output.html]\n").map_err(|e|e.to_string())?;
    for name in [
        "PLAN.json",
        "evaluacion/REFERENCIAS.json",
        "REGISTRO-VACIO.json",
        "visor/book.toml",
        "visor/src/SUMMARY.md",
    ] {
        let b = fs::read(out.join(name)).map_err(|e| e.to_string())?;
        identities.push(json!({"ruta":name,"bytes":b.len(),"sha256":hash(&b)}));
    }
    write(
        &out.join("IDENTIDADES.json"),
        &json!({"archivos":identities,
        "fuentes_fijadas":{"correspondencias_sha256":hash(&fs::read(cp).map_err(|e|e.to_string())?),"resultados_sha256":hash(&fs::read(rp).map_err(|e|e.to_string())?)},
        "comprobaciones":{"casos":12,"por_condicion":6,"dominios_por_condicion":2,"avisos_entregados":1,"inferencias":0,"nuevas_comparaciones":0}}),
    )
}

fn now_ms() -> R<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis()
        .try_into()
        .map_err(|_| "Reloj fuera de rango".into())
}
fn timing(args: &[String]) -> R<()> {
    match args.get(1).map(String::as_str) {
        Some("iniciar") if args.len() == 5 => {
            let p = read(Path::new(&args[2]))?;
            let c = find(array(&p, "casos")?, &args[3])?;
            let data = json!({"id":c["id"],"dominio":c["dominio"],"condicion":c["condicion"],"inicio_unix_ms":now_ms()?,"fin_unix_ms":null,"fuente_reloj":"SystemTime Rust; tiempo exterior de la sesión"});
            let mut f = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&args[4])
                .map_err(|e| e.to_string())?;
            f.write_all(&serde_json::to_vec_pretty(&data).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())
        }
        Some("cerrar") if args.len() == 4 => {
            let mut v = read(Path::new(&args[2]))?;
            let start = number(&v, "inicio_unix_ms")?.ok_or("Inicio ausente")?;
            let end = now_ms()?;
            if !v["fin_unix_ms"].is_null() || end < start {
                return Err("Registro ya cerrado o reloj invertido".into());
            }
            v["fin_unix_ms"] = json!(end);
            let mut f = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&args[3])
                .map_err(|e| e.to_string())?;
            f.write_all(&serde_json::to_vec_pretty(&v).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())
        }
        _ => Err("Uso temporal: iniciar PLAN ID INICIO | cerrar INICIO FINAL".into()),
    }
}

fn evaluate(plan: &Value, refs: &Value, records: &Value) -> R<Value> {
    let cases = array(plan, "casos")?;
    let keys = array(refs, "casos")?;
    let sessions = array(records, "sesiones")?;
    let mut planned = HashSet::new();
    for c in cases {
        if !planned.insert(text(c, "id")?) {
            return Err("Caso duplicado en el plan".into());
        }
        let k = find(keys, text(c, "id")?)?;
        if c["dominio"] != k["dominio"] {
            return Err("Dominio discordante en la referencia".into());
        }
        if !["ordinaria", "con_ayuda"].contains(&text(c, "condicion")?) {
            return Err("Condición inválida".into());
        }
        if !["inmunologia", "ciberseguridad"].contains(&text(c, "dominio")?) {
            return Err("Dominio inválido".into());
        }
    }
    if keys.len() != cases.len() {
        return Err("Referencias adicionales o ausentes".into());
    }
    let mut seen = HashSet::new();
    let mut rows = Vec::new();
    let mut previous_end = None;
    for s in sessions {
        let id = text(s, "id")?;
        if !seen.insert(id) {
            return Err("Sesión duplicada; conservar intentos aparte".into());
        }
        let c = find(cases, id)?;
        let k = find(keys, id)?;
        if s["condicion"] != c["condicion"] || s["dominio"] != c["dominio"] {
            return Err("Condición o dominio discordantes".into());
        }
        let status = text(s, "estado")?;
        if !["completa", "fallida"].contains(&status) {
            return Err("Estado inválido".into());
        }
        let elapsed = match (number(s, "inicio_unix_ms")?, number(s, "fin_unix_ms")?) {
            (Some(a), Some(b)) if b >= a => Some(b - a),
            (None, None) => None,
            _ => return Err("Relojes incompletos o invertidos".into()),
        };
        if let Some(start) = number(s, "inicio_unix_ms")? {
            if previous_end.map_or(false, |end| start < end) {
                return Err("Sesiones solapadas o fuera de secuencia temporal".into());
            }
            previous_end = number(s, "fin_unix_ms")?;
        }
        if elapsed.is_some() && text(s, "fuente_reloj")?.trim().is_empty() {
            return Err("Falta fuente temporal".into());
        }
        let wait = number(s, "espera_instrumental_ms")?;
        let alert_time = number(s, "revision_avisos_ms")?;
        if wait.is_some() && (elapsed.is_none() || wait > elapsed) {
            return Err("Espera inválida".into());
        }
        if alert_time.is_some() && (elapsed.is_none() || alert_time > elapsed) {
            return Err("Duración de avisos inválida".into());
        }
        let notices = c["avisos_entregados"]
            .as_u64()
            .ok_or("Falta recuento de avisos")?;
        if notices == 0 && alert_time.unwrap_or(0) != 0 {
            return Err("Tiempo de avisos sin avisos".into());
        }
        let tokens = number(s, "tokens_observados")?;
        if tokens.is_some() && text(s, "fuente_tokens")?.trim().is_empty() {
            return Err("Tokens sin fuente".into());
        }
        let decision = s.get("decision").ok_or("Falta decisión")?;
        let foundation = s
            .get("fundamento_contrastado")
            .ok_or("Falta contraste del fundamento")?;
        if !foundation.is_null() && !foundation.is_boolean() {
            return Err("Contraste inválido".into());
        }
        let complete = status == "completa";
        if complete {
            if ![
                "incompatibilidad",
                "compatibilidad",
                "contexto_insuficiente",
            ]
            .contains(&decision.as_str().ok_or("Decisión inválida")?)
            {
                return Err("Decisión inválida".into());
            }
            if text(s, "justificacion")?.trim().is_empty()
                || array(s, "citas")?.len() != 2
                || array(s, "citas")?
                    .iter()
                    .any(|v| v.as_str().map_or(true, |x| x.trim().is_empty()))
            {
                return Err("Falta justificación o citas de ambos documentos".into());
            }
        } else if !decision.is_null() || !foundation.is_null() {
            return Err("Una sesión fallida no produce dictamen evaluable".into());
        }
        if foundation.as_bool() == Some(true) && text(s, "nota_contraste")?.trim().is_empty() {
            return Err("Contraste sin explicación".into());
        }
        rows.push(json!({"id":id,"condicion":c["condicion"],"dominio":c["dominio"],"completa":complete,
            "duracion_ms":elapsed,"espera_ms":wait,"revision_avisos_ms":alert_time,"tokens_observados":tokens,
            "coincide_referencia":if complete {json!(decision==&k["referencia"])} else {Value::Null},
            "fundamento_contrastado":foundation,"resultado_fundado":complete&&decision==&k["referencia"]&&foundation==true,
            "omision_incompatibilidad":if complete {json!(k["referencia"]=="incompatibilidad"&&decision!="incompatibilidad")} else {Value::Null},
            "falsa_incompatibilidad":if complete {json!(k["referencia"]=="compatibilidad"&&decision=="incompatibilidad")} else {Value::Null}}));
    }
    let mut groups = Vec::new();
    for domain in ["todos", "inmunologia", "ciberseguridad"] {
        for condition in ["ordinaria", "con_ayuda"] {
            let rs: Vec<_> = rows
                .iter()
                .filter(|v| {
                    v["condicion"] == condition && (domain == "todos" || v["dominio"] == domain)
                })
                .collect();
            let n = cases
                .iter()
                .filter(|v| {
                    v["condicion"] == condition && (domain == "todos" || v["dominio"] == domain)
                })
                .count();
            let complete = rs.iter().filter(|v| v["completa"] == true).count();
            let known: Vec<u64> = rs
                .iter()
                .filter_map(|v| v["duracion_ms"].as_u64())
                .collect();
            let sum = known.iter().try_fold(0u64, |a, b| {
                a.checked_add(*b).ok_or("Desbordamiento temporal")
            })?;
            let all = n > 0 && complete == n && known.len() == n;
            let founded = rs.iter().filter(|v| v["resultado_fundado"] == true).count();
            let tokens: Option<Vec<u64>> =
                rs.iter().map(|v| v["tokens_observados"].as_u64()).collect();
            let token_total = if rs.len() == n && n > 0 {
                tokens
                    .map(|v| {
                        v.iter().try_fold(0u64, |a, b| {
                            a.checked_add(*b).ok_or("Desbordamiento de tokens")
                        })
                    })
                    .transpose()?
            } else {
                None
            };
            groups.push(json!({"dominio":domain,"condicion":condition,"previstos":n,"registrados":rs.len(),"completos":complete,"fallidos":rs.len()-complete,
            "coincidencias":rs.iter().filter(|v|v["coincide_referencia"]==true).count(),"resultados_fundados":founded,
            "fundamentos_pendientes":rs.iter().filter(|v|v["completa"]==true&&v["fundamento_contrastado"].is_null()).count(),
            "omisiones_incompatibilidad":rs.iter().filter(|v|v["omision_incompatibilidad"]==true).count(),
            "falsas_incompatibilidades":rs.iter().filter(|v|v["falsa_incompatibilidad"]==true).count(),
            "duraciones_observadas":known.len(),"suma_duraciones_observadas_ms":if known.is_empty(){Value::Null}else{json!(sum)},
            "media_ms":if all{json!(sum as f64/n as f64)}else{Value::Null},
            "resultados_fundados_por_hora":if all&&sum>0&&rs.iter().all(|v|!v["fundamento_contrastado"].is_null()){json!(founded as f64*3_600_000.0/sum as f64)}else{Value::Null},
            "tokens_total":token_total}));
        }
    }
    let mut differences = Vec::new();
    for domain in ["todos", "inmunologia", "ciberseguridad"] {
        let a = groups
            .iter()
            .find(|v| v["dominio"] == domain && v["condicion"] == "ordinaria")
            .unwrap();
        let b = groups
            .iter()
            .find(|v| v["dominio"] == domain && v["condicion"] == "con_ayuda")
            .unwrap();
        let delta = match (a["media_ms"].as_f64(), b["media_ms"].as_f64()) {
            (Some(x), Some(y)) if x > 0.0 => json!(100.0 * (x - y) / x),
            _ => Value::Null,
        };
        differences.push(json!({"dominio":domain,"reduccion_descriptiva_porcentaje":delta,
            "interpretacion":"Comparación exploratoria entre casos distintos; no acredita por sí sola un efecto causal ni ahorro en uso real."}));
    }
    Ok(
        json!({"esquema":"bsd-utilidad-resultado/1","estado":if sessions.is_empty(){"sin_sesiones"}else if rows.len()==cases.len()&&rows.iter().all(|v|v["completa"]==true){"sesiones_completas"}else{"incompleto"},"casos":rows,"grupos":groups,"diferencias":differences,"dictamen_automatico":null}),
    )
}

fn run() -> R<()> {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("preparar") if args.len() == 4 => prepare(Path::new(&args[2]), Path::new(&args[3])),
        Some("calcular") if args.len() == 6 => write(
            Path::new(&args[5]),
            &evaluate(
                &read(Path::new(&args[2]))?,
                &read(Path::new(&args[3]))?,
                &read(Path::new(&args[4]))?,
            )?,
        ),
        Some("iniciar" | "cerrar") => timing(&args),
        _ => Err("Uso: preparar RAIZ SALIDA | calcular PLAN REFERENCIAS REGISTRO SALIDA".into()),
    }
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn data() -> (Value, Value, Value) {
        (
            json!({"casos":[{"id":"X","dominio":"inmunologia","condicion":"ordinaria","avisos_entregados":0},{"id":"Y","dominio":"inmunologia","condicion":"con_ayuda","avisos_entregados":1}]}),
            json!({"casos":[{"id":"X","dominio":"inmunologia","referencia":"incompatibilidad"},{"id":"Y","dominio":"inmunologia","referencia":"incompatibilidad"}]}),
            json!({"sesiones":[record("X","ordinaria",1000),record("Y","con_ayuda",500)]}),
        )
    }
    fn record(id: &str, c: &str, end: u64) -> Value {
        let start = if id == "Y" { 2000 } else { 0 };
        json!({"id":id,"dominio":"inmunologia","condicion":c,"estado":"completa","inicio_unix_ms":start,"fin_unix_ms":end+start,"fuente_reloj":"control sintético","espera_instrumental_ms":null,"revision_avisos_ms":null,"tokens_observados":null,"fuente_tokens":null,"decision":"incompatibilidad","justificacion":"Ejemplo sintético","citas":["a","b"],"fundamento_contrastado":true,"nota_contraste":"Ejemplo controlado"})
    }
    #[test]
    fn reduccion_y_rendimiento() {
        let (p, k, s) = data();
        let r = evaluate(&p, &k, &s).unwrap();
        assert_eq!(
            r["diferencias"][0]["reduccion_descriptiva_porcentaje"],
            50.0
        );
        assert_eq!(r["grupos"][0]["resultados_fundados_por_hora"], 3600.0);
    }
    #[test]
    fn vacio_no_es_cero() {
        let (p, k, _) = data();
        let r = evaluate(&p, &k, &json!({"sesiones":[]})).unwrap();
        assert_eq!(r["estado"], "sin_sesiones");
        assert!(r["diferencias"][0]["reduccion_descriptiva_porcentaje"].is_null());
        assert!(r["grupos"][0]["tokens_total"].is_null());
    }
    #[test]
    fn falta_tiempo_no_es_cero() {
        let (p, k, mut s) = data();
        s["sesiones"][0]["inicio_unix_ms"] = Value::Null;
        s["sesiones"][0]["fin_unix_ms"] = Value::Null;
        assert!(evaluate(&p, &k, &s).unwrap()["diferencias"][0]
            ["reduccion_descriptiva_porcentaje"]
            .is_null());
    }
    #[test]
    fn duplicado_rechazado() {
        let (p, k, mut s) = data();
        let x = s["sesiones"][0].clone();
        s["sesiones"].as_array_mut().unwrap().push(x);
        assert!(evaluate(&p, &k, &s).is_err());
    }
    #[test]
    fn dominio_rechazado() {
        let (p, k, mut s) = data();
        s["sesiones"][0]["dominio"] = json!("ciberseguridad");
        assert!(evaluate(&p, &k, &s).is_err());
    }
    #[test]
    fn condicion_rechazada() {
        let (p, k, mut s) = data();
        s["sesiones"][0]["condicion"] = json!("con_ayuda");
        assert!(evaluate(&p, &k, &s).is_err());
    }
    #[test]
    fn reloj_invertido_rechazado() {
        let (p, k, mut s) = data();
        s["sesiones"][0]["inicio_unix_ms"] = json!(2000);
        assert!(evaluate(&p, &k, &s).is_err());
    }
    #[test]
    fn espera_excesiva_rechazada() {
        let (p, k, mut s) = data();
        s["sesiones"][0]["espera_instrumental_ms"] = json!(2000);
        assert!(evaluate(&p, &k, &s).is_err());
    }
    #[test]
    fn fundamento_no_inferido() {
        let (p, k, mut s) = data();
        s["sesiones"][0]["fundamento_contrastado"] = Value::Null;
        let r = evaluate(&p, &k, &s).unwrap();
        assert_eq!(r["grupos"][0]["resultados_fundados"], 0);
        assert_eq!(r["grupos"][0]["coincidencias"], 1);
        assert!(r["grupos"][0]["resultados_fundados_por_hora"].is_null());
    }
    #[test]
    fn denominador_cero() {
        let (p, k, mut s) = data();
        s["sesiones"][0]["fin_unix_ms"] = json!(0);
        let r = evaluate(&p, &k, &s).unwrap();
        assert!(r["diferencias"][0]["reduccion_descriptiva_porcentaje"].is_null());
    }
    #[test]
    fn caso_desconocido_rechazado() {
        let (p, k, mut s) = data();
        s["sesiones"][0]["id"] = json!("Z");
        assert!(evaluate(&p, &k, &s).is_err());
    }
    #[test]
    fn tokens_sin_fuente_rechazados() {
        let (p, k, mut s) = data();
        s["sesiones"][0]["tokens_observados"] = json!(99);
        assert!(evaluate(&p, &k, &s).is_err());
    }
    #[test]
    fn fallido_no_es_resultado() {
        let (p, k, mut s) = data();
        s["sesiones"][0]["estado"] = json!("fallida");
        assert!(evaluate(&p, &k, &s).is_err());
        s["sesiones"][0]["decision"] = Value::Null;
        s["sesiones"][0]["fundamento_contrastado"] = Value::Null;
        let r = evaluate(&p, &k, &s).unwrap();
        assert_eq!(r["estado"], "incompleto");
        assert!(r["diferencias"][0]["reduccion_descriptiva_porcentaje"].is_null());
    }
    #[test]
    fn omision_separada() {
        let (p, k, mut s) = data();
        s["sesiones"][0]["decision"] = json!("compatibilidad");
        let r = evaluate(&p, &k, &s).unwrap();
        assert_eq!(r["grupos"][0]["omisiones_incompatibilidad"], 1);
        assert_eq!(r["grupos"][0]["resultados_fundados"], 0);
    }
    #[test]
    fn solapamiento_rechazado() {
        let (p, k, mut s) = data();
        s["sesiones"][1]["inicio_unix_ms"] = json!(500);
        assert!(evaluate(&p, &k, &s).is_err());
    }
    #[test]
    fn tiempo_negativo_rechazado() {
        let (p, k, mut s) = data();
        s["sesiones"][0]["fin_unix_ms"] = json!(-1);
        assert!(evaluate(&p, &k, &s).is_err());
    }
    #[test]
    fn falsa_incompatibilidad() {
        let (p, mut k, s) = data();
        k["casos"][0]["referencia"] = json!("compatibilidad");
        assert_eq!(
            evaluate(&p, &k, &s).unwrap()["grupos"][0]["falsas_incompatibilidades"],
            1
        );
    }
    #[test]
    fn detalle_no_se_confunde_con_cero() {
        let (p, k, mut s) = data();
        s["sesiones"][0]["citas"] = json!([]);
        assert!(evaluate(&p, &k, &s).is_err());
    }
}
