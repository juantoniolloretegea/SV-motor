//! Recepción acotada de texto convertido. No acredita validez clínica ni recepción universal.
use crate::{Identidad, Metadato, leer_identidad, leer_json};
use serde::{Deserialize, Serialize};
use std::path::Path;
use sv_mcp_documental::sha256;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recepcion {
    pub version: u32,
    pub documento: String,
    pub original_sha256: String,
    pub markdown_sha256: String,
    pub evidencia: Identidad,
    pub metodo: String,
    pub alcance: String,
    pub admitido_ensayo: bool,
    pub produccion: bool,
}
pub fn recibir(root: &Path, id: &Identidad, meta: &Metadato, raw: &[u8], text: &str) -> Result<(), String> {
    let r: Recepcion = leer_json(&leer_identidad(root, id, 16384)?)?;
    if r.version != 1 || r.documento != meta.documento || r.original_sha256 != sha256(raw)
        || r.markdown_sha256 != sha256(text.as_bytes()) || !r.admitido_ensayo || r.produccion
        || r.metodo.trim().is_empty() || r.alcance.trim().is_empty()
        || meta.correspondencia != "conversion_recibida_para_ensayo" {
        return Err("RECEPCION_CONVERSION_DISCORDANTE".into());
    }
    // La huella del recibo se fija fuera del corpus. Una declaración del candidato no lo modifica.
    leer_identidad(root, &r.evidencia, 1024 * 1024)?;
    Ok(())
}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
