//! Control instrumental del ejemplo público de SC-XF-002; no caso bibliográfico.
use serde_json::json;
use speccheck_core::{parser::{MarkdownParser,SpecParser},rules::{Rule,cross_file::CrossFileContradiction}};
use std::{path::Path,fs::OpenOptions,io::Write};
fn main()->Result<(),Box<dyn std::error::Error>>{
 let out=std::env::args().nth(1).ok_or("Falta archivo de salida")?;
 let p=MarkdownParser::new();
 let positive="# Security\n\nThe system shall log all authentication events.\n";
 let negative="# Privacy\n\nThe system shall not log authentication events.\n";
 let d1=p.parse(Path::new("security.md"),positive)?;
 let d2=p.parse(Path::new("privacy.md"),negative)?;
 let findings=CrossFileContradiction::new().check_corpus(&[&d1,&d2]);
 let ok=findings.len()==1;
 let data=json!({"tipo":"control_instrumental_positivo","origen":"ejemplo público de la explicación de SC-XF-002 en speccheck-core 0.4.1","texto_positivo":positive,"texto_negativo":negative,"avisos":findings.iter().map(|d|json!({"regla":d.rule_id.to_string(),"mensaje":d.message,"localizacion":d.span})).collect::<Vec<_>>(),"conforme":ok,"incluido_en_los_doce_casos":false});
 let mut f=OpenOptions::new().write(true).create_new(true).open(out)?;f.write_all(&serde_json::to_vec_pretty(&data)?)?;
 if !ok{return Err("Control instrumental no conforme".into())} println!("Control positivo conforme: una alerta SC-XF-002");Ok(())
}
