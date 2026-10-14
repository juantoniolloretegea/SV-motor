//! Observación del ensayo; no modifica operaciones matemáticas del motor.
use std::{fs::{File, OpenOptions}, io::Write, sync::{Mutex, OnceLock, atomic::{AtomicU64, Ordering}}, time::SystemTime};
static LOG: OnceLock<Mutex<Option<File>>> = OnceLock::new();
static PTX_ID: AtomicU64 = AtomicU64::new(0);
static LANZAMIENTOS: AtomicU64 = AtomicU64::new(0);
pub fn activa() -> bool { std::env::var_os("SV_GPU_TRAZA").is_some() }
pub fn total() -> u64 { LANZAMIENTOS.load(Ordering::SeqCst) }
pub fn anotar(tipo: &str, datos: &str) {
    let log = LOG.get_or_init(|| Mutex::new(std::env::var_os("SV_GPU_TRAZA").map(|p| OpenOptions::new().create_new(true).write(true).open(p).expect("No se puede conservar la traza GPU"))));
    if let Some(f) = log.lock().expect("Traza bloqueada").as_mut() {
        let ns=SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Reloj").as_nanos();
        let fase=std::env::var("SV_FASE").unwrap_or_else(|_| "preparacion".into());
        writeln!(f, "{{\"tipo\":{tipo:?},\"fase\":{fase:?},\"pid\":{},\"unix_ns\":{ns},\"datos\":{datos}}}", std::process::id()).expect("Pérdida de traza GPU");
        f.flush().expect("Pérdida de traza GPU");
    }
}
pub fn conservar_ptx(ptx: &str) {
    if !activa() { return; }
    let dir=std::path::PathBuf::from(std::env::var_os("SV_GPU_PTX").expect("Directorio PTX ausente"));
    let n=PTX_ID.fetch_add(1,Ordering::SeqCst);
    let nombre=format!("programa-{n:04}.ptx");
    let mut f=OpenOptions::new().create_new(true).write(true).open(dir.join(&nombre)).expect("PTX no conservado");
    f.write_all(ptx.as_bytes()).expect("PTX incompleto");
    anotar("ptx",&format!("{{\"archivo\":{nombre:?},\"bytes\":{}}}",ptx.len()));
}
pub fn funcion(nombre: &str, funcion: usize, modulo: usize) {
    anotar("funcion",&format!("{{\"nombre\":{nombre:?},\"funcion\":{funcion},\"modulo\":{modulo}}}"));
}
pub fn lanzamiento(funcion: usize, flujo: usize, grid: (u32,u32,u32), bloque:(u32,u32,u32), resultado:i32) {
    LANZAMIENTOS.fetch_add(1,Ordering::SeqCst);
    anotar("lanzamiento",&format!("{{\"funcion\":{funcion},\"flujo\":{flujo},\"grid\":[{},{},{}],\"bloque\":[{},{},{}],\"resultado\":{resultado}}}",grid.0,grid.1,grid.2,bloque.0,bloque.1,bloque.2));
}
pub fn denegar_blas(nombre: &str) {
    if activa() {
        anotar("recorrido_rechazado", &format!("{{\"biblioteca\":{nombre:?}}}"));
        eprintln!("Recorrido ajeno al cálculo Rust rechazado: {nombre}");
        std::process::exit(71);
    }
}
