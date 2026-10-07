#![forbid(unsafe_code)]
#[path="../catalogo/mod.rs"] mod catalogo;
type E=Box<dyn std::error::Error+Send+Sync>;
fn main(){match catalogo::preflight(){Ok(v)=>println!("{}",serde_json::json!({"conforme":v["conforme"],"casos":v["casos"].as_array().map(Vec::len),"inferencia":false})),Err(e)=>{eprintln!("Recepción detenida: {e}");std::process::exit(1);}}}
