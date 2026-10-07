use super::*;
pub const MODEL:&str="gpt-6-astra";
pub fn pdf()->bool {true}
pub fn dir()->String {super::pdf::base().to_string_lossy().into_owned()}
pub fn prepare()->R<()> {super::pdf::prepare()}
pub fn infer(_http:&Client,token:&str)->R<Value>{super::pdf::infer(token)}
pub fn page(v:&Value)->String {super::pdf::page(v)}
pub fn start_page()->String {super::pdf::start_page()}
pub fn result_page(message:Option<&str>)->String {
    fs::read_to_string(super::pdf::base().join("resultado.html")).unwrap_or_else(|_|format!("<!doctype html><meta charset='utf-8'><p>{}</p>",sv_instrumentacion::escape(message.unwrap_or("Comprobación en curso"))))
}
