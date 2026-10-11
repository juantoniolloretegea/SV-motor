use serde_json::json;
use std::{fs,io::Write};
use sv_administracion_documental::huella;
fn main()->Result<(),Box<dyn std::error::Error>>{
 let a:Vec<_>=std::env::args().collect();if a.len()!=4{return Err("original modificado salida".into())}
 let old=fs::read_to_string(&a[1])?.replace("\r\n","\n");let new=fs::read_to_string(&a[2])?.replace("\r\n","\n");
 let rule=old.split_once("impl Rule for CrossFileContradiction {").ok_or("Regla ausente")?.1;
 let from=rule.find("    fn check_corpus(").ok_or("Método ausente")?;
 let to=rule[from..].find("    fn explanation(").ok_or("Final ausente")?+from;
 let original=&rule[from..to];
 let start=new.find("    fn check_corpus_observed(").ok_or("Observador ausente")?;
 let end=new[start..].find("\n}\n\nimpl Default for CrossFileContradiction").ok_or("Final auxiliar ausente")?+start;
 let recovered=new[start..end].replace("fn check_corpus_observed(&self, docs: &[&SpecDocument], observer: &mut dyn FnMut(&SourceSpan))","fn check_corpus(&self, docs: &[&SpecDocument])").replace("                    observer(&pos_req.span);\n","");
 let ok=original.trim()==recovered.trim();
 let result=json!({"conforme":ok,"metodo":"Igualdad textual del cuerpo de comparación tras retirar únicamente la llamada de observación y restaurar la firma","cuerpo_original_sha256":huella(original.trim().as_bytes()),"cuerpo_reconstituido_sha256":huella(recovered.trim().as_bytes()),"localizacion_conservada":"pos_req.span en el punto de emisión; neg_req.span permanece en el diagnóstico","etiquetas":"floor_char_boundary evita cortar UTF-8; no participa en la comparación"});
 let mut f=fs::OpenOptions::new().write(true).create_new(true).open(&a[3])?;f.write_all(&serde_json::to_vec_pretty(&result)?)?;
 if !ok{return Err("Criterio discordante".into())}println!("Cuerpo de comparación conservado; cotejo Rust conforme");Ok(())
}
