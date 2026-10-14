//! Cotejo posterior del archivo recuperado; no carga modelos ni inicializa dispositivos.
use flate2::read::GzDecoder;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, error::Error, fs, io::Read, path::{Component, Path}};
type R<T> = Result<T, Box<dyn Error>>;
fn sha(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
fn main() -> R<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 4 { return Err("Uso: cotejar archivo sha256 informe".into()); }
    let raw = fs::read(&a[1])?;
    if sha(&raw) != a[2] { return Err("SHA-256 del archivo distinto".into()); }
    let mut tar = tar::Archive::new(GzDecoder::new(raw.as_slice()));
    let mut files = BTreeMap::new();
    for entry in tar.entries()? {
        let mut e = entry?;
        let p = e.path()?.into_owned();
        if !p.components().all(|c| matches!(c, Component::Normal(_))) || !p.starts_with("resultado") {
            return Err("Ruta de archivo no admitida".into());
        }
        if e.header().entry_type().is_dir() { continue; }
        if !e.header().entry_type().is_file() || e.size() > 128 * 1024 * 1024 { return Err("Entrada no admitida".into()); }
        let key = p.strip_prefix("resultado")?.to_string_lossy().replace('\\', "/");
        let mut b = Vec::new(); e.read_to_end(&mut b)?;
        if files.insert(key, b).is_some() { return Err("Archivo duplicado".into()); }
    }
    let parse = |n: &str| -> R<Value> { Ok(serde_json::from_slice(files.get(n).ok_or("Archivo ausente")?)?) };
    let manifest = parse("INTEGRIDAD.json")?;
    let mut checked = BTreeMap::new();
    for f in manifest["archivos"].as_array().ok_or("Manifiesto incorrecto")? {
        let name = f["archivo"].as_str().ok_or("Nombre ausente")?;
        let bytes = files.get(name).ok_or("Archivo del manifiesto ausente")?;
        if f["bytes"].as_u64() != Some(bytes.len() as u64) || f["sha256"].as_str() != Some(sha(bytes).as_str()) {
            return Err(format!("Identidad discrepante: {name}").into());
        }
        if checked.insert(name, bytes.len()).is_some() { return Err("Manifiesto duplicado".into()); }
    }
    if checked.len() + 1 != files.len() { return Err("Archivos adicionales sin cotejar".into()); }
    let lines = |n: &str| -> R<Vec<Value>> { std::str::from_utf8(files.get(n).ok_or("Registro ausente")?)?.lines().map(|s| Ok(serde_json::from_str(s)?)).collect() };
    let trace = lines("TRAZA-GPU.jsonl")?;
    let tele = lines("TELEMETRIA.jsonl")?;
    let mut kinds = BTreeMap::<String, usize>::new();
    let mut phases = BTreeMap::<String, usize>::new();
    let mut transfer_bytes = BTreeMap::<String, u64>::new();
    for t in &trace {
        *kinds.entry(t["tipo"].as_str().unwrap_or("?").into()).or_default() += 1;
        *phases.entry(t["fase"].as_str().unwrap_or("?").into()).or_default() += 1;
        if t["tipo"] == "transferencia" { *transfer_bytes.entry(t["datos"]["operacion"].as_str().unwrap_or("?").into()).or_default() += t["datos"]["bytes"].as_u64().unwrap_or(0); }
    }
    let mut telemetry_phases = BTreeMap::<String, usize>::new();
    let mut errors = 0; let mut own = 0; let mut cpu_peak = 0; let mut gpu_peak = BTreeMap::<String,u64>::new();
    for t in &tele {
        *telemetry_phases.entry(t["fase"].as_str().unwrap_or("?").into()).or_default() += 1;
        if t["cpu"].get("error").is_some() { errors += 1; }
        cpu_peak = cpu_peak.max(t["cpu"]["rss_bytes"].as_u64().unwrap_or(0));
        for g in t["gpu"].as_array().ok_or("Muestra GPU incorrecta")? {
            if g.get("error").is_some() { errors += 1; }
            if g["procesos_propios"].as_array().is_some_and(|a| !a.is_empty()) { own += 1; }
            let p = gpu_peak.entry(g["indice"].to_string()).or_default();
            *p = (*p).max(g["memoria_usada"].as_u64().unwrap_or(0));
        }
    }
    let mut vectors = Vec::new();
    for (n,b) in &files {
        if n.ends_with(".f32") {
            if b.len()%4 != 0 { return Err("Vector incompleto".into()); }
            let v: Vec<f32> = b.chunks_exact(4).map(|x| f32::from_le_bytes(x.try_into().unwrap())).collect();
            vectors.push(json!({"archivo":n,"valores":v.len(),"finitos":v.iter().all(|x| x.is_finite())}));
        }
    }
    let max_interval = tele.windows(2).map(|w| w[1]["monotono_ms"].as_u64().unwrap_or(0).saturating_sub(w[0]["monotono_ms"].as_u64().unwrap_or(0))).max();
    let cpu = if files.contains_key("CPU.json") { parse("CPU.json")? } else { parse("solicitud-cpu.json")? };
    let us: u64 = cpu["posiciones"].as_array().ok_or("Posiciones CPU ausentes")?.iter().map(|p|p["pared_us"].as_u64().unwrap_or(0)).sum();
    let out = json!({"alcance":"Cotejo posterior en Rust, sin inferencia", "archivo_sha256":a[2],"archivo_bytes":raw.len(),"integridad":"conforme","archivos_cotejados":checked.len(),"archivos_totales":files.len(),"supervisor":parse("SUPERVISOR.json")?,"cpu":cpu,"cpu_posiciones_total_us":us,"vectores":vectors,"traza":{"registros":trace.len(),"tipos":kinds,"fases":phases,"transferencias_bytes":transfer_bytes,"ultimo":trace.last()},"telemetria":{"muestras":tele.len(),"fases":telemetry_phases,"errores_basicos":errors,"muestras_gpu_pid_propio":own,"rss_maximo_bytes":cpu_peak,"memoria_gpu_maxima_bytes":gpu_peak,"intervalo_maximo_ms":max_interval,"primera":tele.first(),"ultima":tele.last()},"inventario":parse("INVENTARIO.json")?,"entrada":parse("ENTRADA.json")?});
    if Path::new(&a[3]).exists() { return Err("El informe ya existe".into()); }
    fs::write(&a[3], serde_json::to_vec_pretty(&out)?)?;
    println!("Conforme: {} archivos cotejados; {} registros de traza; {} muestras; {} vectores. Informe: {}",checked.len(),trace.len(),tele.len(),out["vectores"].as_array().unwrap().len(),a[3]);
    Ok(())
}
