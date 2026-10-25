//! Fuente común para la representación y lectura de figuras documentales.
//! Common source for documentary figure rendering and accessible reading.
#![forbid(unsafe_code)]

mod modelo;
mod salida;
mod svg;
mod validacion;

pub use modelo::{Elemento, Error, Fuente, MAX_ENTRADA, Nodo, Texto};
pub use salida::{html, markdown, representar_svg};
pub use svg::importar;
pub use validacion::validar;

/// JSON estricto y acotado; no ejecuta instrucciones ni obtiene recursos.
/// Strict bounded JSON; no instruction execution or resource retrieval.
pub fn leer_json(bytes: &[u8]) -> Result<Fuente, Error> {
    if bytes.len() > MAX_ENTRADA {
        return Err(Error::Limite);
    }
    let fuente: Fuente = serde_json::from_slice(bytes).map_err(|_| Error::Formato)?;
    validar(&fuente)?;
    Ok(fuente)
}
