#![forbid(unsafe_code)]
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, env, fs, path::Path};
type R<T> = Result<T, Box<dyn std::error::Error>>;
fn num(v: &Value, k: &str) -> R<u64> {
    v[k].as_u64()
        .ok_or_else(|| format!("Falta entero: {k}").into())
}
fn text<'a>(v: &'a Value, k: &str) -> R<&'a str> {
    v[k].as_str()
        .ok_or_else(|| format!("Falta texto: {k}").into())
}
fn rank(values: &[u64], value: u64, ascending: bool) -> usize {
    1 + values
        .iter()
        .filter(|&&n| if ascending { n < value } else { n > value })
        .count()
}
// Milésimas de USD por millón, con mezcla 7:2:1 de caché, entrada ordinaria y salida.
fn blended(c: Option<u64>, i: u64, o: u64) -> Option<u64> {
    c.and_then(|c| {
        c.checked_mul(7)?
            .checked_add(i.checked_mul(2)?)?
            .checked_add(o)
    })
}
fn money_cents(c: u64) -> String {
    format!("{}.{:02}", c / 100, c % 100)
}
fn main() -> R<()> {
    let args: Vec<_> = env::args().collect();
    let root = Path::new(args.get(1).ok_or("Indicar directorio de estudio")?);
    let input = fs::read(root.join("DATOS.json"))?;
    let data: Value = serde_json::from_slice(&input)?;
    let models = data["modelos"].as_array().ok_or("Modelos ausentes")?;
    if models.len() != 4 {
        return Err("Se requieren cuatro modelos".into());
    }
    let mut ids = BTreeSet::new();
    let mut costs = vec![];
    let mut quality = vec![];
    let mut speed = vec![];
    let mut latency = vec![];
    for m in models {
        if !ids.insert(text(m, "id")?) {
            return Err("Modelo duplicado".into());
        }
        let q = num(m, "indice_aa")?;
        if q > 100
            || num(m, "razonamiento_aproximado_tokens_tarea")?
                > num(m, "salida_aproximada_tokens_tarea")?
        {
            return Err("Magnitudes incoherentes".into());
        }
        costs.push(num(m, "coste_tarea_centavos")?);
        quality.push(q);
        speed.push(num(m, "velocidad_decimas_tokens_s")?);
        latency.push(num(m, "primera_respuesta_ms")?);
    }
    let mut rows = vec![];
    for m in models {
        let c = num(m, "coste_tarea_centavos")?;
        let q = num(m, "indice_aa")?;
        let i = num(m, "entrada_centavos_millon")?;
        let o = num(m, "salida_centavos_millon")?;
        rows.push(json!({"id":m["id"],"nombre":m["nombre"],"proveedor":m["proveedor"],"configuracion_aa":m["configuracion_aa"],
            "puesto_coste_tarea":rank(&costs,c,true),"coste_tarea_usd":money_cents(c),
            "puesto_indice_capacidad":rank(&quality,q,false),"indice_aa":q,
            "puesto_velocidad":rank(&speed,num(m,"velocidad_decimas_tokens_s")?,false),
            "puesto_primera_respuesta":rank(&latency,num(m,"primera_respuesta_ms")?,true),
            "precio_ponderado_aa_721_milesimas_usd_millon":blended(m["cache_aa_centavos_millon"].as_u64(),i,o),
            "precio_ponderado_cache_oficial_721_milesimas_usd_millon":blended(m["cache_oficial_centavos_millon"].as_u64(),i,o),
            "fuente":m["fuente_aa"]}));
    }
    rows.sort_by_key(|r| r["puesto_coste_tarea"].as_u64().unwrap());
    let out = root.join("resultados");
    fs::create_dir_all(&out)?;
    let result = json!({"version":"1.1.0","naturaleza":"evaluacion_externa","datos_sha256":format!("{:x}",Sha256::digest(&input)),
        "referencia":data["referencia_externa"],"fecha":data["fecha_consulta"],
        "regla":"Orden externo de Artificial Analysis: menor coste medio por tarea primero; no ranquin propio SV ni coste por acierto.",
        "modelos":rows,"licencia_documento":data["licencia_documento"]});
    fs::write(
        out.join("RANQUIN.json"),
        serde_json::to_vec_pretty(&result)?,
    )?;
    let mut csv=String::from("puesto_economico_externo,proveedor,modelo,configuracion_aa,usd_por_tarea_aa,indice_aa,puesto_capacidad_aa,puesto_velocidad_aa,puesto_primera_respuesta_aa\n");
    for r in result["modelos"].as_array().unwrap() {
        csv += &format!(
            "{},{},{},{},{},{},{},{},{}\n",
            r["puesto_coste_tarea"],
            text(r, "proveedor")?,
            text(r, "id")?,
            text(r, "configuracion_aa")?,
            text(r, "coste_tarea_usd")?,
            r["indice_aa"],
            r["puesto_indice_capacidad"],
            r["puesto_velocidad"],
            r["puesto_primera_respuesta"]
        );
    }
    fs::write(out.join("RANQUIN.csv"), csv)?;
    // Gráfico documental; los ejes comienzan en cero. No representa una célula SV.
    let mut svg = String::from(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="630" viewBox="0 0 1000 630" role="img" aria-labelledby="titulo descripcion"><title id="titulo">Artificial Analysis: coste por tarea y capacidad externa</title><desc id="descripcion">Cuatro modelos de Artificial Analysis v4.3.2; menor coste hacia la izquierda y mayor índice hacia arriba. No es una clasificación de aptitud SV.</desc><rect width="1000" height="630" fill="white"/><g font-family="Arial,sans-serif" fill="#23394a"><text x="65" y="35" font-size="23">Artificial Analysis · coste por tarea y capacidad</text><text x="65" y="60" font-size="14">Artificial Analysis v4.3.2 · configuraciones declaradas · referencia externa al SV</text>"##,
    );
    for tick in 0..=6 {
        let x = 80 + tick * 140;
        svg += &format!(
            r##"<path d="M{x} 90 V450" stroke="#dde3e8"/><text x="{x}" y="473" text-anchor="middle" font-size="14">{tick}</text>"##
        );
    }
    for tick in 0..=6 {
        let y = 450 - tick * 60;
        svg += &format!(
            r##"<path d="M80 {y} H920" stroke="#dde3e8"/><text x="66" y="{}" text-anchor="end" font-size="14">{}</text>"##,
            y + 5,
            tick * 10
        );
    }
    for m in models {
        let x = 80.0 + num(m, "coste_tarea_centavos")? as f64 * 1.4;
        let y = 450 - num(m, "indice_aa")? * 6;
        let (dx, dy) = if text(m, "id")? == "gpt-6-astra" {
            (12.0, -12)
        } else {
            (12.0, 22)
        };
        svg += &format!(
            r##"<circle cx="{x:.2}" cy="{y}" r="6" fill="#245575"/><text x="{:.2}" y="{}" font-size="15">{}</text>"##,
            x + dx,
            y as i64 + dy,
            text(m, "nombre")?
        );
    }
    svg += r##"<text x="500" y="504" text-anchor="middle" font-size="16">Coste medio ponderado por tarea (USD)</text><text x="19" y="270" transform="rotate(-90 19 270)" text-anchor="middle" font-size="16">Índice de capacidad AA</text><text x="65" y="541" font-size="12">Fuente: artificialanalysis.ai · Cálculo y representación en Rust · Sin equivalencia con aptitud clínica o de ciberseguridad</text></g></svg>"##;
    let licence = text(&data, "licencia_documento")?;
    let footer = format!(
        r##"<metadata>{licence}</metadata><text x="65" y="570" font-family="Arial,sans-serif" font-size="9" fill="#23394a"><tspan x="65">© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 |</tspan><tspan x="65" dy="14">Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ |</tspan><tspan x="65" dy="14">ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).</tspan></text></svg>"##
    );
    svg = svg.replace("</svg>", &footer);
    fs::write(out.join("COSTE-Y-CAPACIDAD.svg"), svg)?;
    println!(
        "Conforme: cuatro modelos; orden económico EXTERNO, empates, referencias y gráfico generados."
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conserva_empates() {
        assert_eq!(rank(&[45, 53, 46, 45], 45, false), 3);
        assert_eq!(rank(&[45, 53, 46, 45], 53, false), 1);
    }
    #[test]
    fn ausencia_no_es_gratis() {
        assert_eq!(blended(None, 200, 600), None);
        assert_eq!(blended(Some(0), 200, 600), Some(1000));
    }
    #[test]
    fn mezcla_comun() {
        assert_eq!(blended(Some(26), 140, 440), Some(902));
        assert_eq!(blended(Some(25), 200, 600), Some(1175));
        assert_eq!(blended(Some(100), 1000, 5000), Some(7700));
    }
    #[test]
    fn coste_no_es_calidad() {
        assert_eq!(rank(&[201, 326, 374, 541], 326, true), 2);
        assert_eq!(rank(&[45, 53, 46, 45], 53, false), 1);
    }
}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
