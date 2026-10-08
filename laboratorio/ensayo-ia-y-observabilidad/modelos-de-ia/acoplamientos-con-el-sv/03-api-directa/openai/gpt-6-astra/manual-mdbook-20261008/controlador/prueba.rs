use super::*;
pub const MODEL:&str="gpt-6-astra";
pub fn pdf()->bool {true}
pub fn dir()->String {super::manual::base().to_string_lossy().into_owned()}
pub fn prepare()->R<()> {super::manual::prepare()}
pub fn infer(_http:&Client,token:&str)->R<Value>{super::manual::infer(token)}
pub fn page(v:&Value)->String {super::manual::page(v)}
pub fn start_page()->String {super::manual::start_page()}
pub fn result_page(message:Option<&str>)->String {
    fs::read_to_string(super::manual::base().join("resultado.html")).unwrap_or_else(|_|format!("<!doctype html><meta charset='utf-8'><p>{}</p>",sv_instrumentacion::escape(message.unwrap_or("Comprobación en curso"))))
}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
