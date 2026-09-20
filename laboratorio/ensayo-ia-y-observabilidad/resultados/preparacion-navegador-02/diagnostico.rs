//! Marcas instrumentales; sin cambios de contrato o algoritmo.
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch="wasm32")]
#[wasm_bindgen(inline_js = "export function eio_marca(nombre){globalThis.eioMarcaRust?.(nombre);}")]
extern "C" {fn eio_marca(nombre:&str);}
pub fn marca(nombre:&str){
 #[cfg(target_arch="wasm32")] eio_marca(nombre);
 #[cfg(not(target_arch="wasm32"))] {let _=nombre;}
}
