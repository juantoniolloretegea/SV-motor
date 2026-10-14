use anyhow::{Context, Result, bail, ensure};
use candle::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::{generation::LogitsProcessor, models::marian};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{io::{self, BufRead, Read, Write}, path::Path, time::Instant};
mod spm;
mod aislamiento;

#[derive(Deserialize)]
struct Entrada { id: String, texto: String }
#[derive(Serialize)]
struct Salida {
    id: String, origen: &'static str, destino: &'static str,
    texto: String, entrada_sha256: String, salida_sha256: String,
    tokens_entrada: usize, tokens_salida: usize, duracion_ms: u128,
    memoria_residente_kib: Option<u64>, memoria_maxima_kib: Option<u64>,
}
fn huella(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
fn memoria(campo: &str)->Option<u64> {
    std::fs::read_to_string("/proc/self/status").ok()?.lines().find_map(|l|l.strip_prefix(campo).and_then(|v|v.split_whitespace().next()?.parse().ok()))
}
fn main() -> Result<()> {
    aislamiento::sin_red()?;
    let dir = std::env::args().nth(1).context("Falta la carpeta local del modelo")?;
    let p = Path::new(&dir);
    let _anclas = aislamiento::solo_modelo(p)?;
    let manifiesto: serde_json::Value = serde_json::from_slice(&std::fs::read(p.join("MANIFIESTO.json"))?)?;
    let archivos = manifiesto["archivos"].as_array().context("Manifiesto incompleto")?;
    ensure!(archivos.len() == 6, "Manifiesto incompleto");
    ensure!(manifiesto["modelo"] == "Helsinki-NLP/opus-mt-en-es" && manifiesto["revision"] == "fdaddf76f50fcc1583ba42f95965862a7ab30f97", "Modelo no admitido");
    let mut recibidos = std::collections::BTreeSet::new();
    for a in archivos {
        let n = a["ruta"].as_str().context("Ruta ausente")?;
        ensure!(["config.json","vocab.json","source.spm","target.spm","model.safetensors","ficha-modelo.md"].contains(&n), "Ruta no admitida");
        ensure!(recibidos.insert(n), "Archivo repetido en el manifiesto");
        let b = std::fs::read(p.join(n))?;
        ensure!(Some(b.len() as u64) == a["bytes"].as_u64() && Some(huella(&b).as_str()) == a["sha256"].as_str(), "Archivo no conforme");
        if n == "model.safetensors" { ensure!(huella(&b) == "b3ecbf954573c2fd95d05d3ad4618baf961793db0da07c2add2e8a3a6cd78d0b", "Pesos no admitidos"); }
    }
    let carga = Instant::now();
    let config: marian::Config = serde_json::from_slice(&std::fs::read(p.join("config.json"))?)?;
    ensure!(config.max_position_embeddings == 512 && config.vocab_size == 65001, "Modelo no admitido");
    let enc = spm::tokenizer(p.join("source.spm"), p.join("vocab.json"))?;
    let dec = spm::tokenizer(p.join("target.spm"), p.join("vocab.json"))?;
    let device = Device::Cpu;
    let vb = VarBuilder::from_buffered_safetensors(std::fs::read(p.join("model.safetensors"))?, DType::F32, &device)?;
    let mut model = marian::MTModel::new(&config, vb)?;
    eprintln!("Carga local completada: {} ms", carga.elapsed().as_millis());
    let mut stdin = io::stdin().lock();
    let mut salida = io::stdout().lock();
    loop {
        let mut linea = String::new();
        let mut limite = (&mut stdin).take(32769);
        if limite.read_line(&mut linea)? == 0 { break }
        ensure!(linea.len() <= 32768 && linea.ends_with('\n'), "Entrada excesiva o incompleta");
        let e: Entrada = serde_json::from_str(&linea)?;
        ensure!(!e.id.is_empty() && e.id.len() <= 100 && !e.texto.trim().is_empty() && e.texto.len() <= 16000, "Entrada no admitida");
        let inicio = Instant::now();
        let mut ids = enc.encode(e.texto.as_str(), false).map_err(anyhow::Error::msg)?.get_ids().to_vec();
        ensure!(ids.len() <= 384, "El fragmento excede la capacidad; no se trunca");
        ids.push(config.eos_token_id);
        let tokens_entrada = ids.len();
        model.reset_kv_cache();
        let xs = model.encoder().forward(&Tensor::new(ids.as_slice(), &device)?.unsqueeze(0)?, 0)?;
        let mut generados = vec![config.decoder_start_token_id];
        let mut processor = LogitsProcessor::new(0, None, None);
        let mut terminada = false;
        for i in 0..480 {
            ensure!(inicio.elapsed().as_secs() < 60, "Tiempo de traducción excedido");
            let logits = model.decode(&Tensor::new(&generados[i..], &device)?.unsqueeze(0)?, &xs, i)?.squeeze(0)?;
            let token = processor.sample(&logits.get(logits.dim(0)? - 1)?)?;
            if token == config.eos_token_id { terminada = true; break; }
            ensure!(token != config.pad_token_id, "Token no admitido en la salida");
            generados.push(token);
        }
        if !terminada { bail!("Salida incompleta; no se presenta como traducción íntegra") }
        let texto = dec.decode(&generados[1..], true).map_err(anyhow::Error::msg)?;
        ensure!(!texto.trim().is_empty(), "Salida vacía");
        let s = Salida { id:e.id, origen:"en", destino:"es", entrada_sha256:huella(e.texto.as_bytes()), salida_sha256:huella(texto.as_bytes()), texto, tokens_entrada, tokens_salida:generados.len()-1, duracion_ms:inicio.elapsed().as_millis(), memoria_residente_kib:memoria("VmRSS:"), memoria_maxima_kib:memoria("VmHWM:") };
        serde_json::to_writer(&mut salida, &s)?; writeln!(salida)?; salida.flush()?;
    }
    Ok(())
}
