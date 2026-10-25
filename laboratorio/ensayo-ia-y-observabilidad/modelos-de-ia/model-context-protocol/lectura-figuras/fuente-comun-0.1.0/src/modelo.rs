use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const MAX_ENTRADA: usize = 2 * 1024 * 1024;
pub(crate) const MAX_NODOS: usize = 8192;
pub(crate) const MAX_PROFUNDIDAD: usize = 48;
pub(crate) const MAX_TEXTO: usize = 128 * 1024;
pub(crate) const MAX_TEXTOS: usize = 4096;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Formato,
    Limite,
    ElementoNoAdmitido,
    AtributoNoAdmitido,
    Referencia,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fuente {
    pub esquema: String,
    pub original_sha256: String,
    pub relaciones: String,
    pub textos: Vec<Texto>,
    pub estructura_visual: Elemento,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Texto {
    pub id: String,
    pub valor: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Elemento {
    pub nombre: String,
    pub atributos: BTreeMap<String, String>,
    pub hijos: Vec<Nodo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "tipo", deny_unknown_fields)]
pub enum Nodo {
    #[serde(rename = "elemento")]
    Elemento { elemento: Elemento },
    #[serde(rename = "texto")]
    Texto { id: String },
}
