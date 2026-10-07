//! Comprueba que la conciliación preserva los registros ajenos y los antecedentes.
use std::fs;
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn read(root:&str,k:&str)->R<String>{Ok(fs::read_to_string(format!("{root}/{k}.txt"))?)}
fn sin_s39(s:&str)->String{s.lines().filter(|l|!l.starts_with("\"S39\",")).collect::<Vec<_>>().join("\n")}
fn sin_tt16(s:&str)->String{s.lines().filter(|l|!l.contains(",\"TT-0016\",")).collect::<Vec<_>>().join("\n")}
fn sin_seccion(s:&str)->String{let a=s.find("## S39 ").unwrap();let b=s[a..].find("\n## S40").map(|n|a+n).unwrap_or(s.len());format!("{}{}",&s[..a],&s[b..]).trim_end().to_owned()}
fn bloques(s:&str)->Vec<String>{s.split("```").enumerate().filter(|(n,_)|n%2==1).map(|(_,t)|t.to_owned()).collect()}
fn main()->R<()>{let old=read("calidad-base","eventcsv")?;let new=read("calidad-final","eventcsv")?;assert_eq!(sin_s39(&old),sin_s39(&new));println!("CONFORME: otros sucesos CSV intactos");
assert_eq!(sin_tt16(&read("calidad-base","ticketscsv")?),sin_tt16(&read("calidad-final","ticketscsv")?));println!("CONFORME: otros tiques CSV intactos");
assert_eq!(sin_seccion(&read("calidad-base","events")?),sin_seccion(&read("calidad-final","events")?));println!("CONFORME: otros sucesos Markdown intactos");
let o=read("calidad-base","eventhistory")?;let n=read("calidad-final","eventhistory")?;assert!(n.starts_with(o.trim_end()));assert_eq!(n.lines().count(),o.lines().count()+1);assert!(n.lines().last().unwrap().starts_with("\"27\",\"S39\","));println!("CONFORME: historial conservado y única revisión 27 añadida");
for k in ["acta1","acta3","quality-index"]{let o=read("calidad-base",k)?;let n=read("calidad-final",k)?;let body=o.split_once("\n\n").unwrap().1.trim_end();assert!(n.contains(body),"Antecedente alterado: {k}");assert_eq!(bloques(&o),bloques(&n));}println!("CONFORME: tres remisiones conservan el texto y los diagramas previos");
let o=read("calidad-base","acta4")?;let n=read("calidad-final","acta4")?;assert_eq!(bloques(&o),bloques(&n));assert!(n.contains(o.split_once("## 1.").unwrap().1.trim_end()));assert!(n.contains("## 24."));println!("CONFORME: Acta 004 conserva apartados 1 a 23 y añade 24");
for k in ["acta1","acta3","acta4","quality-index","events","eventcsv","eventhistory","ticketscsv","tt16","tt16json"]{let n=read("calidad-final",k)?;assert!(!n.contains("__ENTREGA_COMMIT__")&&!n.contains("__MODELO_COMMIT__"),"Enlace sin resolver");}
println!("CONFORME: referencias fijadas sin marcadores pendientes");
let t=read("calidad-final","tt16json")?;assert!(t.contains("\"frame_completo\": false")&&t.contains("\"dictamen_final\": null")&&t.contains("\"revision_suceso\": 27"));println!("CONFORME: recepción humana pendiente y frame incompleto explícitos");Ok(())}
