//! Ensayo de una herramienta OCR auxiliar, sin acceso al Gestor ni al núcleo.
//! Auxiliary OCR trial without access to the application or kernel.
#![forbid(unsafe_code)]
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
use rustix::process::{Resource, Rlimit, setrlimit};
use serde_json::json;
use std::{io::Read, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for (r, v) in [
        (Resource::As, 768 * 1024 * 1024),
        (Resource::Cpu, 20),
        (Resource::Fsize, 1024 * 1024),
        (Resource::Core, 0),
    ] {
        setrlimit(
            r,
            Rlimit {
                current: Some(v),
                maximum: Some(v),
            },
        )?;
    }
    let mut bytes = Vec::new();
    std::io::stdin()
        .take(4 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() > 4 * 1024 * 1024 {
        return Err("Imagen superior a la cota".into());
    }
    let formato = image::guess_format(&bytes)?;
    if !matches!(formato, image::ImageFormat::Png | image::ImageFormat::Jpeg) {
        return Err("Formato no admitido".into());
    }
    let dimensiones =
        image::ImageReader::with_format(std::io::Cursor::new(&bytes), formato).into_dimensions()?;
    if u64::from(dimensiones.0) * u64::from(dimensiones.1) > 2_000_000 {
        return Err("Demasiados píxeles".into());
    }
    let img = image::load_from_memory_with_format(&bytes, formato)?.into_rgb8();
    let inicio = Instant::now();
    let detector = Model::load_file("/datos/text-detection-ssfbcj81.rten")?;
    let reconocedor = Model::load_file("/datos/text-rec-checkpoint-s52qdbqt.rten")?;
    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detector),
        recognition_model: Some(reconocedor),
        ..Default::default()
    })?;
    let carga_ms = inicio.elapsed().as_millis();
    let mut resultados = Vec::new();
    for repeticion in 0..2 {
        let instante = Instant::now();
        let origen = ImageSource::from_bytes(img.as_raw(), img.dimensions())?;
        let entrada = engine.prepare_input(origen)?;
        let texto = engine.get_text(&entrada)?;
        resultados.push(json!({"repeticion":repeticion+1,"milisegundos":instante.elapsed().as_millis(),"texto":texto}));
    }
    let estado = std::fs::read_to_string("/proc/self/status")?;
    let recursos: Vec<_> = estado
        .lines()
        .filter(|l| l.starts_with("VmHWM:") || l.starts_with("VmRSS:") || l.starts_with("Threads:"))
        .collect();
    let rutas = std::fs::read_to_string("/proc/net/route")?;
    let salidas_red = rutas
        .lines()
        .skip(1)
        .filter(|l| !l.trim().is_empty())
        .count();
    let host_visible =
        std::path::Path::new("/mnt/c").exists() || std::path::Path::new("/home").exists();
    if salidas_red != 0 || host_visible {
        return Err("Aislamiento del ensayo no conforme".into());
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &json!({"ocrs":"0.13.1","rten":"0.26.0","dimensiones":dimensiones,"carga_modelos_ms":carga_ms,"resultados":resultados,"limite_memoria_virtual_bytes":768*1024*1024,"limite_cpu_segundos":20,"recursos_proceso":recursos,"rutas_red":salidas_red,"host_visible":host_visible,"modelo_ia_auxiliar":true,"nodos_sv":false,"integrado_en_gestor":false})
        )?
    );
    Ok(())
}
