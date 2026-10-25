#![forbid(unsafe_code)]
use std::io::{self, Read, Write};
use sv_figuras_fuente::{MAX_ENTRADA, html, importar, leer_json, markdown, representar_svg};

fn ejecutar() -> Result<(), String> {
    let accion = std::env::args()
        .nth(1)
        .ok_or("Indique importar, svg, html o markdown")?;
    if std::env::args().count() != 2 {
        return Err("La entrada se recibe únicamente por flujo".into());
    }
    let mut bytes = Vec::new();
    io::stdin()
        .take((MAX_ENTRADA + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| "Entrada no disponible")?;
    let salida = if accion == "importar" {
        let f = importar(&bytes).map_err(|e| format!("Figura no admitida: {e:?}"))?;
        serde_json::to_string_pretty(&f).map_err(|_| "Fuente no representable")?
    } else {
        let f = leer_json(&bytes).map_err(|e| format!("Fuente no admitida: {e:?}"))?;
        match accion.as_str() {
            "svg" => representar_svg(&f),
            "html" => html(&f),
            "markdown" => markdown(&f),
            _ => return Err("Operación no admitida".into()),
        }
        .map_err(|e| format!("Representación no admitida: {e:?}"))?
    };
    if salida.len() > MAX_ENTRADA {
        return Err("Representación superior al límite de entrada".into());
    }
    io::stdout()
        .lock()
        .write_all(salida.as_bytes())
        .map_err(|_| "Salida no disponible".into())
}
fn main() {
    if let Err(e) = ejecutar() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
