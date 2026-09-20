//! Recepción acotada de dos salidas literales; no evalúa calidad semántica general.
use std::{env, fs, process};
use serde_json::Value;
use eio_candidato::{comprobar, Condiciones, Referencia};

fn leer(ruta: &str) -> Result<Value, Box<dyn std::error::Error>> {
    if fs::metadata(ruta)?.len() > 1_048_576 { return Err("EVIDENCIA_EXCESIVA".into()); }
    Ok(serde_json::from_str(&fs::read_to_string(ruta)?)?)
}
fn evaluar(v: &Value, variante: &str) -> Result<String, Box<dyn std::error::Error>> {
    if v["tipo"] != "modelo_real" || v["variante_peticion"] != variante || v["telemetria"] != "on" {
        return Err("IDENTIDAD_DE_SALIDA".into());
    }
    let texto = v["salida"]["texto"].as_str().ok_or("TEXTO_AUSENTE")?;
    let entrada = usize::try_from(v["salida"]["entrada"].as_u64().ok_or("ENTRADA_AUSENTE")?)?;
    let refs = vec![Referencia { id: "A".into(), version: 1 }, Referencia { id: "B".into(), version: 1 }];
    let c = Condiciones { id: "sintetica-01".into(), permiso: false, activa: false, veto: false,
        tokens_entrada: entrada, exportacion_correcta: true };
    // Estas referencias fijan el contrato del caso; no prueban consultas internas del modelo.
    let resultado = comprobar(&c, texto, &refs, &refs).err().unwrap_or("OK");
    if v["contrato"].as_str() != Some(resultado) { return Err("CLASIFICACION_INCONSISTENTE".into()); }
    Ok(resultado.into())
}
fn ejecutar() -> Result<bool, Box<dyn std::error::Error>> {
    let a: Vec<String> = env::args().collect();
    if a.len() != 3 { return Err("uso: recepcion06 SALIDA_BASE SALIDA_ESTRUCTURADA".into()); }
    let base = leer(&a[1])?;
    let nueva = leer(&a[2])?;
    let anterior: Value = serde_json::from_str(include_str!("referencia-eio05.json"))?;
    let cb = evaluar(&base, "EIO-05")?;
    let cn = evaluar(&nueva, "EIO-06")?;
    let referencia = base["salida"]["texto"] == anterior["salida"]["texto"]
        && base["salida"]["tokens"] == anterior["salida"]["tokens"]
        && base["salida"]["entrada"] == anterior["salida"]["entrada"]
        && base["salida"]["generados"] == anterior["salida"]["generados"]
        && base["salida"]["fin"] == "EOS" && cb == "ESTRUCTURA";
    let conforme = referencia && cn == "OK" && nueva["salida"]["fin"] == "EOS";
    println!("{}", serde_json::json!({
        "tipo": "recepcion_acotada_06", "referencia_reproducida": referencia,
        "contrato_base": cb, "contrato_estructurado": cn,
        "fin_estructurado": nueva["salida"]["fin"], "conforme": conforme,
        "alcance": "regresión literal y contrato de salida; no suficiencia semántica ni tasa de fiabilidad"
    }));
    Ok(conforme)
}
fn main() {
    match ejecutar() {
        Ok(true) => (),
        Ok(false) => process::exit(2),
        Err(e) => { eprintln!("ERROR_RECEPCION: {e}"); process::exit(1); }
    }
}
