#![forbid(unsafe_code)]
#[path="../catalogo/estricto.rs"] mod estricto;
#[path="../catalogo/puntuacion.rs"] mod puntuacion;
use std::{fs,io::Write,path::Path};
use serde_json::Value;
use sha2::{Digest,Sha256};
type E=Box<dyn std::error::Error+Send+Sync>;
const FOOTER:&str="© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn read(p:&Path)->Result<Value,E>{Ok(estricto::parse(&fs::read(p)?)?)}
fn save(p:&Path,v:&Value)->Result<(),E>{let mut f=fs::OpenOptions::new().write(true).create_new(true).open(p)?;f.write_all(&serde_json::to_vec_pretty(v)?)?;f.sync_all()?;Ok(())}
fn main()->Result<(),E>{let a:Vec<_>=std::env::args().collect();if a.len()!=3{return Err("Uso: ADJUDICACIONES.json CARPETA_NUEVA".into())}puntuacion::capa(Path::new(&a[1]),Path::new(&a[2]))}
