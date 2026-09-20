//! Duraciones monotonicas y tiempo civil medido; std preservado en nativo.
#[cfg(not(target_arch="wasm32"))]
pub use std::time::Instant;
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch="wasm32")]
#[wasm_bindgen(inline_js = "export function eio_mono(){return performance.now();} export function eio_civil(){return Date.now();}")]
extern "C" {
    fn eio_mono() -> f64;
    fn eio_civil() -> f64;
}
#[cfg(target_arch="wasm32")]
pub struct Instant(f64);
#[cfg(target_arch="wasm32")]
impl Instant {
 pub fn now()->Self{Self(eio_mono())}
 pub fn elapsed(&self)->std::time::Duration{
  let ms=eio_mono()-self.0; assert!(ms.is_finite() && ms>=0.0,"RELOJ_MONOTONICO");
  std::time::Duration::from_secs_f64(ms/1000.0)
 }
}
pub fn civil()->std::time::SystemTime {
 #[cfg(target_arch="wasm32")] {
  let ms=eio_civil(); assert!(ms.is_finite() && ms>0.0,"RELOJ_CIVIL");
  std::time::UNIX_EPOCH+std::time::Duration::from_secs_f64(ms/1000.0)
 }
 #[cfg(not(target_arch="wasm32"))] {std::time::SystemTime::now()}
}
