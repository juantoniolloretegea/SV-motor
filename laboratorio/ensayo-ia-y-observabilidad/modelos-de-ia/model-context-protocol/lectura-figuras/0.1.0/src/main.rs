//! Entrada administrativa local por flujo; no admite direcciones ni carga recursos externos.
#![forbid(unsafe_code)]
use std::io::{self, Read};
use sv_lectura_figuras::{MAX_ENTRADA, extraer_svg};

fn ejecutar() -> Result<(), &'static str> {
    let mut bytes = Vec::new();
    io::stdin()
        .take((MAX_ENTRADA + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| "No se pudo leer la entrada")?;
    let lectura =
        extraer_svg(&bytes).map_err(|_| "SVG no admitido o límite de lectura excedido")?;
    serde_json::to_writer(io::stdout().lock(), &lectura)
        .map_err(|_| "No se pudo entregar la lectura")
}
fn main() {
    if let Err(causa) = ejecutar() {
        eprintln!("{causa}");
        std::process::exit(1);
    }
}
