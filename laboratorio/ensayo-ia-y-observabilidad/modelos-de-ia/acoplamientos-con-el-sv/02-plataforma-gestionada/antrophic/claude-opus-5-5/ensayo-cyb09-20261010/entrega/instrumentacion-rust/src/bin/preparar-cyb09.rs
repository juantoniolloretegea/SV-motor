#![forbid(unsafe_code)]
use sv_claude_kaggle::{self as sv, need, parse, sha, R};
use serde_json::{json, Value};
use std::{fs, path::Path, io::{Read, Write}, time::{Duration, Instant}};
const BASE: &str = "https://mp-staging.kaggle.net/models/openapi";
#[path="cyb09/recepcion.rs"] mod recepcion;
use recepcion::*;
#[path="cyb09/continuacion.rs"] mod continuacion;
use continuacion::*;

fn run() -> R<()> {
    let args: Vec<_> = std::env::args().collect();
    need(args.len() == 6, "Uso: preparar-cyb09 PAQUETE-ORIGINAL RAIZ-ORIGINALES MANIFIESTO CLAVE DESTINO")?;
    let bytes = fs::read(&args[1]).map_err(|e| e.to_string())?;
    need(sha(&bytes) == "758b9bac230b81410df73795c7c74ae3089987bfb8aecc7d53917c50896f749d", "Paquete original diferente")?;
    let p = parse(&bytes)?;
    let manifest_bytes = fs::read(&args[3]).map_err(|e| e.to_string())?;
    need(sha(&manifest_bytes) == "83d7ff96be26d8626dbdde555102565a90a440456a8ff4231719bf209e10594f", "Manifiesto original diferente")?;
    let manifest = parse(&manifest_bytes)?;
    let key_bytes = fs::read(&args[4]).map_err(|e| e.to_string())?;
    need(sha(&key_bytes) == "214c94df2521569d9349836fb13f439c87a0af01e36ce4581b6cce37824d4807", "Criticidad o clave original diferentes")?;
    let key = parse(&key_bytes)?;
    let cases = p["bancos"][1]["casos"].as_array().ok_or("Banco original ausente")?;
    let mut selection = Vec::new(); let mut criteria = Vec::new();
    for id in SELECCION {
        selection.push(cases.iter().find(|v| v["id"] == id).ok_or("Caso ausente")?.clone());
        let k = key["preguntas"].as_array().ok_or("Clave ausente")?.iter().find(|v| v["id"] == id).ok_or("Criterio ausente")?;
        need(k["critica"] == true, "Criticidad cambiada")?; criteria.push(k.clone());
    }
    let entries: Vec<_> = manifest["archivos"].as_array().ok_or("Manifiesto sin archivos")?.iter().filter(|v| v["ruta"].as_str().is_some_and(|r| CONSERVADAS.iter().any(|(id,s)| r.starts_with(&format!("campana/cyb16/{id}-R{s}/"))))).cloned().collect();
    let packet = json!({"version":"claude-cyb09-1.0","modelo":sv::MODELO,"original_paquete_sha256":sha(&bytes),"casos":selection,"originales":entries,"max_tokens":16384,"reintentos_automaticos":0,"nuevas_entregas":23,"limite_nanodolares":9_990_000_000u64,"limite_banco_s":5400,"licencia":sv::LICENCIA});
    let report = comprobar_paquete(&packet, Path::new(&args[2]))?;
    let out = Path::new(&args[5]); need(!out.exists(), "Preparación ya presente")?;
    fs::create_dir_all(out).map_err(|e| e.to_string())?;
    save(&out.join("PAQUETE-CANDIDATO.json"), &packet)?;
    save(&out.join("CLAVE-FIJADA.json"), &json!({"version":"CYB09-1.0","origen_clave_sha256":sha(&key_bytes),"umbral_auxiliar":7,"b":3,"n":9,"preguntas":criteria,"seleccion_posterior_a_cuatro_entregas":true,"recepcion_independiente":"pendiente","licencia":sv::LICENCIA}))?;
    save(&out.join("ADMISION-LOCAL-RUST.json"), &report)?;
    println!("Preparación Rust conforme: 9 casos, 4 originales recuperados, 23 entregas nuevas; ninguna inferencia");
    Ok(())
}
fn main() { if let Err(e) = run() { eprintln!("IMPEDIMENTO: {e}"); std::process::exit(1) } }

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
