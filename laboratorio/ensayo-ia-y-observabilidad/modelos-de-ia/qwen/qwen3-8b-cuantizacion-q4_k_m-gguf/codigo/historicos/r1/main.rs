mod observacion;
use observacion::{cpu, Monitor, RelojGpu};
use realizar::gguf::{
    byte_level_bpe::ByteLevelBpe, MappedGGUFModel, OwnedQuantizedKVCache, OwnedQuantizedModel,
    OwnedQuantizedModelCuda,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};
type R<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
use trueno_gpu::driver::sv_observacion as traza;
const MODELO: &str = "d98cdcbd03e17ce47681435b5150e34c1417f50b5c0019dd560e4882c5745785";
const URL: &str = "https://huggingface.co/Qwen/Qwen3-8B-GGUF/resolve/7c41481f57cb95916b40956ab2f0b139b296d974/Qwen3-8B-Q4_K_M.gguf";
const BYTES: u64 = 5_027_783_488;
const SELECTOR: &str = include_str!("../../src/lib.rs");
const PROMPT: &str = "El resultado de sumar dos y dos es";
const CONT: &str = " cuatro. Esta operación permite comparar el cálculo.";
fn nuevo(p: impl AsRef<Path>) -> R<File> {
    Ok(OpenOptions::new().write(true).create_new(true).open(p)?)
}
fn guardar(d: &Path, nombre: &str, v: &Value) -> R<()> {
    let mut f = nuevo(d.join(nombre))?;
    serde_json::to_writer_pretty(&mut f, v)?;
    f.sync_all()?;
    Ok(())
}
fn hash(p: &Path) -> R<String> {
    let mut f = File::open(p)?;
    let mut h = Sha256::new();
    let mut b = vec![0; 1024 * 1024];
    loop {
        let n = f.read(&mut b)?;
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    Ok(format!("{:x}", h.finalize()))
}
fn fase(s: &str) {
    std::env::set_var("SV_FASE", s);
    traza::anotar("fase", &json!({"nombre":s}).to_string());
    println!("Fase: {s}");
}
fn descargar(p: &Path) -> R<()> {
    if p.exists() {
        return Err("No se reemplaza un archivo de pesos existente".into());
    }
    let t = Instant::now();
    let a = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(600))
        .build();
    let r = a.get(URL).call()?;
    let mut src = r.into_reader();
    let mut f = nuevo(p)?;
    let mut h = Sha256::new();
    let mut b = vec![0; 1024 * 1024];
    let mut n = 0u64;
    loop {
        if t.elapsed() > Duration::from_secs(600) {
            return Err("Descarga fuera de plazo".into());
        }
        let k = src.read(&mut b)?;
        if k == 0 {
            break;
        }
        n += k as u64;
        if n > BYTES {
            return Err("Tamaño excedido".into());
        }
        f.write_all(&b[..k])?;
        h.update(&b[..k]);
    }
    f.sync_all()?;
    let digest = format!("{:x}", h.finalize());
    if n != BYTES || digest != MODELO {
        return Err("Identidad de los pesos no conforme".into());
    }
    println!(
        "{}",
        json!({"pesos_bytes":n,"sha256":digest,"descarga_ms":t.elapsed().as_millis()})
    );
    Ok(())
}
fn entorno(c: &mut Command) {
    for (k, _) in std::env::vars() {
        if k.starts_with("APR_")
            || k.starts_with("SV_GPU")
            || k.contains("Q4K")
            || k.contains("Q6K")
            || k.contains("PARITY")
            || k.contains("CUBLAS")
            || k.starts_with("CUDA_GRAPH")
            || k.starts_with("FP8_")
            || k.starts_with("HGEMM_")
            || k.starts_with("W4A16_")
            || k.starts_with("FUSED_")
            || k.starts_with("TRUENO_")
        {
            c.env_remove(k);
        }
    }
    for k in [
        "BATCHED_PREFILL",
        "HGEMM_DECODE",
        "FP8_PREFILL",
        "FP8_DECODE",
        "W4A16_INTERLEAVED",
        "FUSED_GATE_UP",
        "CUDA_GRAPH_ENABLE",
    ] {
        c.env(k, "0");
    }
    c.env("SKIP_CUDA_GRAPH", "1")
        .env("CUDA_DEVICE_ORDER", "PCI_BUS_ID")
        .env("RAYON_NUM_THREADS", "4");
}
fn empaquetar(dir: &Path) -> R<PathBuf> {
    fn visitar(p: &Path, base: &Path, v: &mut Vec<Value>) -> R<()> {
        let mut entradas = fs::read_dir(p)?.collect::<Result<Vec<_>, _>>()?;
        entradas.sort_by_key(|e| e.file_name());
        for e in entradas {
            let x = e.path();
            if e.file_type()?.is_symlink() {
                return Err("Enlace inesperado en resultados".into());
            }
            if x.is_dir() {
                visitar(&x, base, v)?
            } else {
                v.push(json!({"archivo":x.strip_prefix(base)?.to_string_lossy(),"bytes":e.metadata()?.len(),"sha256":hash(&x)?}));
            }
        }
        Ok(())
    }
    let mut v = vec![];
    visitar(dir, dir, &mut v)?;
    guardar(dir, "INTEGRIDAD.json", &json!({"archivos":v}))?;
    let archivo = dir.with_extension("tar.gz");
    let f = nuevo(&archivo)?;
    let enc = flate2::write::GzEncoder::new(f, flate2::Compression::default());
    let mut t = tar::Builder::new(enc);
    t.append_dir_all("resultado", dir)?;
    t.into_inner()?.finish()?.sync_all()?;
    println!(
        "Archivo: {} · SHA-256 {}",
        archivo.display(),
        hash(&archivo)?
    );
    Ok(archivo)
}
fn supervisar(modelo: &Path, dir: &Path) -> R<()> {
    fs::create_dir(dir)?;
    fs::create_dir(dir.join("ptx"))?;
    guardar(
        dir,
        "PROCEDENCIA.json",
        &json!({"fuente":"paiml/aprender","revision":"d0a59c1882ddf43656d1067cc6d5c1943aba22d3","modelo_sha256":MODELO,"selector_sha256":format!("{:x}",Sha256::digest(SELECTOR.as_bytes())),"ejecutable_sha256":hash(&std::env::current_exe()?)?,"limite_s":600,"reintentos":0,"alcance":"Compatibilidad instrumental de inferencia; no acredita integración productiva, aptitud del modelo ni validación del nodo"}),
    )?;
    let mut c = Command::new(std::env::current_exe()?);
    entorno(&mut c);
    c.arg("trabajador")
        .arg(modelo)
        .arg(dir)
        .env("SV_GPU_TRAZA", dir.join("TRAZA-GPU.jsonl"))
        .env("SV_GPU_PTX", dir.join("ptx"))
        .stdout(Stdio::from(nuevo(dir.join("SALIDA.txt"))?))
        .stderr(Stdio::from(nuevo(dir.join("ERRORES.txt"))?));
    let t = Instant::now();
    let mut h = c.spawn()?;
    let mut vencido = false;
    let estado = loop {
        if let Some(s) = h.try_wait()? {
            break s;
        }
        if t.elapsed() > Duration::from_secs(600) {
            vencido = true;
            h.kill()?;
            break h.wait()?;
        }
        std::thread::sleep(Duration::from_millis(250));
    };
    guardar(
        dir,
        "SUPERVISOR.json",
        &json!({"pid":h.id(),"exit_code":estado.code(),"exito":estado.success(),"limite_superado":vencido,"total_ms":t.elapsed().as_millis()}),
    )?;
    empaquetar(dir)?;
    if !estado.success() {
        return Err("Prueba detenida; registros conservados, sin reintento".into());
    }
    Ok(())
}
fn vector(d: &Path, nombre: &str, v: &[f32]) -> R<()> {
    let mut f = std::io::BufWriter::new(nuevo(d.join(nombre))?);
    for x in v {
        f.write_all(&x.to_le_bytes())?
    }
    f.flush()?;
    f.get_ref().sync_all()?;
    Ok(())
}
fn comparar(a: &[f32], b: &[f32]) -> R<Value> {
    if a.is_empty() || a.len() != b.len() || a.iter().chain(b).any(|v| !v.is_finite()) {
        return Err("Dimensiones o valores no conformes".into());
    }
    let (mut aa, mut bb, mut ab, mut dd, mut max) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for (&x, &y) in a.iter().zip(b) {
        let (x, y) = (x as f64, y as f64);
        aa += x * x;
        bb += y * y;
        ab += x * y;
        dd += (x - y) * (x - y);
        max = max.max((x - y).abs());
    }
    if aa == 0.0 || bb == 0.0 {
        return Err("Vector nulo no comparable".into());
    }
    fn mejores(x: &[f32]) -> (usize, f64) {
        let mut ids: Vec<usize> = (0..x.len()).collect();
        ids.sort_unstable_by(|&i, &j| x[j].total_cmp(&x[i]).then(i.cmp(&j)));
        (
            ids[0],
            if ids.len() > 1 {
                x[ids[0]] as f64 - x[ids[1]] as f64
            } else {
                0.0
            },
        )
    }
    let cos = ab / (aa * bb).sqrt();
    let rel = (dd / aa).sqrt();
    let (ai, am) = mejores(a);
    let (bi, bm) = mejores(b);
    Ok(
        json!({"dimensiones":a.len(),"coseno":cos,"error_relativo_l2":rel,"diferencia_absoluta_maxima":max,"argmax_cpu":ai,"argmax_gpu":bi,"margen_cpu":am,"margen_gpu":bm,"conforme":cos>=0.99 && rel<=0.05 && ai==bi}),
    )
}
fn ensayo(modelo: &Path, d: &Path) -> R<Value> {
    let nv_pre = nvml_wrapper::Nvml::init()?;
    let mut inventario = vec![];
    for i in 0..nv_pre.device_count()? {
        let dev = nv_pre.device_by_index(i)?;
        let nombre = dev.name()?;
        let mem = dev.memory_info()?;
        if !nombre.contains("T4") || mem.free < 8 * 1024 * 1024 * 1024 {
            return Err("Inventario GPU fuera del protocolo o memoria insuficiente".into());
        }
        inventario.push(json!({"indice":i,"nombre":nombre,"uuid":dev.uuid()?,"vram_total":mem.total,"vram_libre":mem.free,"pci":dev.pci_info()?.bus_id}));
    }
    if inventario.is_empty() {
        return Err("No se observa GPU".into());
    }
    let mut sistema = sysinfo::System::new();
    sistema.refresh_memory();
    if sistema.available_memory() < 12 * 1024 * 1024 * 1024 {
        return Err("RAM disponible insuficiente para la prueba".into());
    }
    guardar(
        d,
        "INVENTARIO.json",
        &json!({"gpu":inventario,"controlador":nv_pre.sys_driver_version()?,"ram_total":sistema.total_memory(),"ram_disponible":sistema.available_memory(),"cpu_logicas":std::thread::available_parallelism()?.get(),"cpu_descripcion":fs::read_to_string("/proc/cpuinfo")?}),
    )?;
    fase("identidad");
    if fs::metadata(modelo)?.len() != BYTES || hash(modelo)? != MODELO {
        return Err("Pesos no conformes".into());
    }
    let inicio = Instant::now();
    fase("carga_cpu");
    let mapped = MappedGGUFModel::from_path(modelo)?;
    let vocab = mapped.model.vocabulary().ok_or("Vocabulario ausente")?;
    let bpe = ByteLevelBpe::from_gguf(&mapped.model.metadata, &vocab)
        .map_err(|e| format!("Tokenización canónica no disponible: {e:?}"))?;
    let p = bpe.encode(PROMPT);
    let siguientes = bpe.encode(CONT);
    if p.is_empty() || p.len() > 32 || siguientes.len() < 7 {
        return Err("Secuencia fuera del protocolo".into());
    }
    let mut entrada = p.clone();
    entrada.extend(&siguientes[..7]);
    let comienzo = p.len() - 1;
    let m = OwnedQuantizedModel::from_mapped(&mapped)?;
    let (capas, kv) = (m.config().num_layers, m.config().kv_dim());
    if capas != 36
        || m.config().hidden_dim != 4096
        || m.config().num_heads != 32
        || m.config().num_kv_heads != 8
    {
        return Err("Configuración distinta de Qwen3-8B".into());
    }
    guardar(
        d,
        "ENTRADA.json",
        &json!({"texto":PROMPT,"continuacion":CONT,"tokens_prompt":p,"secuencia":entrada,"primera_salida":comienzo,"salidas":8,"formato_vectores":"f32 little-endian, sin cabecera, orden de vocabulario GGUF","vocabulario":vocab.len(),"configuracion":format!("{:?}",m.config())}),
    )?;
    drop(bpe);
    drop(vocab);
    drop(mapped);
    let carga_cpu = inicio.elapsed().as_millis();
    let mut cache = OwnedQuantizedKVCache::new(capas, kv, 64);
    let mut cpu_vectores = vec![];
    let mut tiempos_cpu = vec![];
    let contador = traza::total();
    fase("inferencia_cpu");
    let ini_cpu = Instant::now();
    for (i, &token) in entrada.iter().enumerate() {
        let antes = cpu()?;
        let t = Instant::now();
        let v = m.forward_single_with_cache(token, &mut cache, i)?;
        let pared = t.elapsed().as_micros();
        let despues = cpu()?;
        if v.iter().any(|x| !x.is_finite()) {
            return Err("CPU produjo valores no finitos".into());
        }
        tiempos_cpu.push(json!({"posicion":i,"token":token,"pared_us":pared,"cpu_antes":antes,"cpu_despues":despues}));
        if i >= comienzo {
            vector(d, &format!("cpu-{:02}.f32", i - comienzo), &v)?;
            cpu_vectores.push(v);
        }
    }
    let cpu_total = ini_cpu.elapsed().as_micros();
    if traza::total() != contador {
        return Err("Lanzamiento GPU durante recorrido CPU".into());
    }
    drop(cache);
    guardar(
        d,
        "CPU.json",
        &json!({"carga_ms":carga_cpu,"recorrido_total_us":cpu_total,"posiciones":tiempos_cpu,"lanzamientos_gpu":0}),
    )?;
    fase("carga_gpu_y_paridad");
    let t = Instant::now();
    let mut gpu = OwnedQuantizedModelCuda::with_max_seq_len(m, 0, 64)
        .map_err(|e| format!("Carga GPU rechazada sin alternativa CPU: {e}"))?;
    if !gpu.supports_gpu_resident() {
        return Err("Modelo no admite este recorrido GPU".into());
    }
    let carga_gpu = t.elapsed().as_millis();
    let reloj = RelojGpu::nuevo()?;
    let identidad = reloj.identidad()?;
    let perfil = serde_json::to_value(gpu.executor().gpu_profile())?;
    if perfil["q4k"] != "mwv"
        || perfil["q6k"] != "hw_dp4a"
        || [
            "hgemm_decode",
            "fp8_prefill",
            "fp8_decode",
            "w4a16_interleaved",
            "fused_gate_up",
        ]
        .iter()
        .any(|k| perfil[*k] != false)
    {
        return Err("Perfil efectivo distinto del fijado".into());
    }
    guardar(
        d,
        "GPU-CARGA.json",
        &json!({"nombre":gpu.device_name(),"identidad":identidad,"perfil":gpu.executor().gpu_profile(),"paridad":{"estado":gpu.parity.status,"coseno":gpu.parity.cosine,"posiciones":gpu.parity.positions,"umbral":gpu.parity.threshold,"fundamento":gpu.parity.basis},"vram":gpu.vram_report(),"carga_y_paridad_ms":carga_gpu}),
    )?;
    if identidad["cc"] != 75 || !gpu.device_name().contains("T4") {
        return Err("Dispositivo distinto del protocolo T4".into());
    }
    let nv = nvml_wrapper::Nvml::init()?;
    let uuid = identidad["uuid"].as_str().ok_or("UUID ausente")?;
    let dev = nv.device_by_uuid(uuid)?;
    if dev
        .running_compute_processes()?
        .iter()
        .all(|p| p.pid != std::process::id())
    {
        return Err("NVML no identifica el proceso de cálculo".into());
    }
    gpu.executor_mut().reset_kv_cache_gpu();
    let mut cache = OwnedQuantizedKVCache::new(capas, kv, 64);
    let mut tiempos_gpu = vec![];
    let mut comparaciones = vec![];
    fase("inferencia_gpu");
    let ini_gpu = Instant::now();
    let lanzamientos = traza::total();
    for (i, &token) in entrada.iter().enumerate() {
        gpu.executor().synchronize()?;
        let stream = gpu.executor().sv_execution_stream().raw() as usize;
        let antes = cpu()?;
        let n = traza::total();
        let t = Instant::now();
        let eventos = reloj.iniciar(stream)?;
        let v = gpu.forward_gpu_resident(token, &mut cache, i)?;
        let gpu_ms = eventos.terminar()?;
        gpu.executor().synchronize()?;
        let pared = t.elapsed().as_micros();
        let despues = cpu()?;
        if traza::total() == n {
            return Err("Posición GPU sin lanzamientos observados".into());
        }
        tiempos_gpu.push(json!({"posicion":i,"token":token,"pared_us":pared,"intervalo_dispositivo_ms":gpu_ms,"cpu_antes":antes,"cpu_despues":despues,"lanzamientos":traza::total()-n,"flujo":stream}));
        if i >= comienzo {
            vector(d, &format!("gpu-{:02}.f32", i - comienzo), &v)?;
            let c = comparar(&cpu_vectores[i - comienzo], &v)?;
            guardar(d, &format!("comparacion-{:02}.json", i - comienzo), &c)?;
            comparaciones.push(c);
        }
    }
    let gpu_total = ini_gpu.elapsed().as_micros();
    guardar(
        d,
        "GPU.json",
        &json!({"recorrido_total_us":gpu_total,"posiciones":tiempos_gpu,"lanzamientos":traza::total()-lanzamientos,"cpu_residual":"Representaciones de tokens, control, transferencias, sesgo opcional y observación; medida del proceso completa","intervalo_dispositivo":"Incluye esperas entre operaciones; no es suma exclusiva de tiempos de los programas GPU"}),
    )?;
    fs::write(
        d.join("BIBLIOTECAS-CARGADAS.txt"),
        fs::read("/proc/self/maps")?,
    )?;
    let conforme = comparaciones.len() == 8 && comparaciones.iter().all(|v| v["conforme"] == true);
    Ok(
        json!({"comparaciones":comparaciones,"conformidad_numerica":conforme,"cpu_total_us":cpu_total,"gpu_total_us":gpu_total,"identidad_gpu":identidad,"lanzamientos_gpu":traza::total(),"alcance":"Contraste acotado del motor. No acredita por sí solo selección productiva u optimalidad; el Selector permanece intacto."}),
    )
}
fn trabajar(m: &Path, d: &Path) -> R<()> {
    let monitor = Monitor::iniciar(d)?;
    let r = ensayo(m, d);
    let obs = monitor.terminar()?;
    match r {
        Ok(v) => {
            let conforme = v["conformidad_numerica"] == true && obs["fallos"] == 0;
            guardar(
                d,
                "RESULTADO.json",
                &json!({"conforme":conforme,"ensayo":v,"telemetria":obs}),
            )?;
            if !conforme {
                return Err("Contraste no conforme; no se repite".into());
            }
        }
        Err(e) => {
            guardar(
                d,
                "RESULTADO.json",
                &json!({"conforme":false,"error":e.to_string(),"telemetria":obs}),
            )?;
            return Err(e);
        }
    }
    Ok(())
}
fn run() -> R<()> {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("descargar") if a.len() == 3 => descargar(Path::new(&a[2])),
        Some("ejecutar") if a.len() == 4 => supervisar(Path::new(&a[2]), Path::new(&a[3])),
        Some("trabajador") if a.len() == 4 => trabajar(Path::new(&a[2]), Path::new(&a[3])),
        Some("sha256") if a.len() == 3 => {
            println!("{}", hash(Path::new(&a[2]))?);
            Ok(())
        }
        _ => Err("Uso: descargar archivo | ejecutar modelo resultados | sha256 archivo".into()),
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
    #[test]
    fn identidad() {
        assert_eq!(comparar(&[1., 2.], &[1., 2.]).unwrap()["conforme"], true)
    }
    #[test]
    fn rechazo_nan_y_forma() {
        assert!(comparar(&[f32::NAN], &[1.]).is_err());
        assert!(comparar(&[1., 2.], &[1.]).is_err())
    }
    #[test]
    fn rechazo_argmax_y_error() {
        assert_eq!(
            comparar(&[1., 1.001], &[1.001, 1.]).unwrap()["conforme"],
            false
        );
        assert_eq!(comparar(&[1., 2.], &[2., 4.]).unwrap()["conforme"], false)
    }
}
