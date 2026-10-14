//! Autocomprobación del medidor CPU, sin modelos ni operaciones del SV.
#![forbid(unsafe_code)]
use serde_json::json;
use std::{
    fs::OpenOptions,
    io::Write,
    path::Path,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use sv_telemetria_cpu_tpu::{intervalo_cpu, sha256, ObservadorCpu};

fn conservar(path: &Path, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = OpenOptions::new().write(true).create_new(true).open(path)?;
    f.write_all(bytes)?;
    f.sync_all()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 || args[1] != "autocomprobacion" {
        return Err("Uso: sv-telemetria-cpu-tpu autocomprobacion <carpeta existente vacía>".into());
    }
    let dir = Path::new(&args[2]);
    if !dir.is_dir() || dir.read_dir()?.next().is_some() {
        return Err("Carpeta inexistente o no vacía".into());
    }
    let inicio = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let mut obs = ObservadorCpu::propio()?;
    let mut muestras = Vec::new();
    let a = obs.muestra()?;
    muestras.push(json!({"fase":"carga_cpu", "muestra":a}));
    let mut cuenta = 0u64;
    let mut carga = [7u8; 4096];
    let limite = Instant::now();
    let mut siguiente = Duration::from_millis(100);
    while limite.elapsed() < Duration::from_millis(650) {
        let h = sha256(std::hint::black_box(&carga));
        carga[..64].copy_from_slice(h.as_bytes());
        cuenta += 1;
        if limite.elapsed() >= siguiente {
            muestras.push(json!({"fase":"carga_cpu", "muestra":obs.muestra()?}));
            siguiente += Duration::from_millis(100);
        }
    }
    let b = obs.muestra()?;
    muestras.push(json!({"fase":"carga_cpu", "muestra":b}));
    let activa = intervalo_cpu(&a, &b)?;
    let c = obs.muestra()?;
    muestras.push(json!({"fase":"espera", "muestra":c}));
    for _ in 0..7 {
        std::thread::sleep(Duration::from_millis(100));
        muestras.push(json!({"fase":"espera", "muestra":obs.muestra()?}));
    }
    let d = obs.muestra()?;
    muestras.push(json!({"fase":"espera", "muestra":d}));
    let espera = intervalo_cpu(&c, &d)?;
    // Diferencia amplia, no umbral de rendimiento del equipo. No es calibración metrológica.
    let control = activa.cpu_proceso_ms > 0 && activa.cpu_proceso_ms > espera.cpu_proceso_ms;
    let value = json!({
        "tipo":"autocomprobacion_medidor_cpu", "revision":env!("CARGO_PKG_VERSION"),
        "inicio_unix_ms":inicio, "sistema":std::env::consts::OS, "arquitectura":std::env::consts::ARCH,
        "fuente_cpu":"sysinfo 0.39.6; contador acumulado del sistema operativo",
        "carga_cpu":activa, "espera":espera, "muestras":muestras,
        "iteraciones_hash":cuenta, "carga_final_sha256":sha256(&carga),
        "contraste_carga_espera_conforme":control,
        "tpu":null, "tpu_estado":"no observada; no existe ejecución TPU en esta comprobación",
        "energia_julios":null, "coste_atribuido":null,
        "limites":["CPU agregada del proceso, incluidos todos sus hilos y el medidor",
          "Memoria muestreada; no garantiza máximo entre muestras",
          "Resolución del tiempo CPU: milisegundos informados por sysinfo; granularidad del SO no calibrada",
          "Es autocomprobación instrumental local, no prueba celular SV ni integración del Árbitro-Director"]
    });
    let bytes = serde_json::to_vec_pretty(&value)?;
    conservar(&dir.join("MUESTREO-CPU.json"), &bytes)?;
    let recovered = std::fs::read(dir.join("MUESTREO-CPU.json"))?;
    if recovered != bytes {
        return Err("Recuperación local discordante".into());
    }
    let manifest = format!("{}  MUESTREO-CPU.json\n", sha256(&recovered));
    conservar(&dir.join("SHA256SUMS"), manifest.as_bytes())?;
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "control_cpu_conforme":control,"cpu_carga_ms":activa.cpu_proceso_ms,
            "cpu_espera_ms":espera.cpu_proceso_ms,"tpu":null,"sha256":sha256(&recovered)
        }))?
    );
    if !control {
        return Err("El medidor no distingue carga de espera; conservar y revisar".into());
    }
    Ok(())
}
