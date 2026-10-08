#![forbid(unsafe_code)]
use std::{fs, path::PathBuf};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = PathBuf::from(
        std::env::args()
            .nth(1)
            .ok_or("Directorio de salida requerido")?,
    );
    fs::create_dir_all(&dir)?;
    let d = sv_visor_pdf::contrato::generar(sv_visor_pdf::FUENTE, include_bytes!("../BANCO.json"))?;
    sv_visor_pdf::contrato::cotejar(&d)?;
    let bytes = serde_json::to_vec_pretty(&d)?;
    fs::write(dir.join("DICTAMEN.json"), &bytes)?;
    println!(
        "Derivación Rust conforme; SHA-256 {}",
        sv_visor_pdf::contrato::huella(&bytes)
    );
    Ok(())
}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
