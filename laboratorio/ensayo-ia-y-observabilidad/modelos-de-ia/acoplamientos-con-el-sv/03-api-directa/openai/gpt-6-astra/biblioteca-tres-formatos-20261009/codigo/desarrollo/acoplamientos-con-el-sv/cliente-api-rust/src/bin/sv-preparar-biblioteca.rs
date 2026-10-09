#![forbid(unsafe_code)]
//! Entrada local del controlador común. No crea perfiles ni llama a proveedores.
use std::path::Path;
fn main() {
    let a:Vec<_>=std::env::args().collect();
    if a.len()!=4 {eprintln!("Uso: sv-preparar-biblioteca CONTRATO SHA256_CONTRATO SALIDA_NUEVA");std::process::exit(2);}
    match sv_cliente_api::biblioteca_documental::preparar(Path::new(&a[1]),&a[2],Path::new(&a[3])) {
        Ok(v)=>println!("{v}"),Err(e)=>{eprintln!("Control detenido: {e}");std::process::exit(1);}
    }
}
