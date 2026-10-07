//! Orden de preguntas: la interrupción del proveedor aplaza; nunca repite entregas recibidas.
use std::collections::VecDeque;
pub struct Agenda{pending:VecDeque<usize>}
pub fn remaining_ms(elapsed:u128)->u64{300000u64.saturating_sub(elapsed.min(u64::MAX as u128) as u64)}
impl Agenda{
 pub fn new()->Self{Self{pending:(16..=25).chain([15]).collect()}}
 pub fn next(&mut self)->Option<usize>{self.pending.pop_front()}
 pub fn defer(&mut self,q:usize){assert!((15..=25).contains(&q));assert!(!self.pending.contains(&q));self.pending.push_back(q);}
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn comienza_por_siguiente_y_deja_15_al_final(){let mut a=Agenda::new();let mut v=vec![];while let Some(q)=a.next(){v.push(q);}assert_eq!(v,(16..=25).chain([15]).collect::<Vec<_>>());}
 #[test]fn fallo_no_repite_antes_de_siguientes(){let mut a=Agenda::new();assert_eq!(a.next(),Some(16));a.defer(16);let mut v=vec![];while let Some(q)=a.next(){v.push(q);}assert_eq!(v,(17..=25).chain([15,16]).collect::<Vec<_>>());}
 #[test]fn dos_fallos_conservan_orden(){let mut a=Agenda::new();let q=a.next().unwrap();a.defer(q);let q=a.next().unwrap();a.defer(q);let v:Vec<_>=std::iter::from_fn(||a.next()).collect();assert_eq!(&v[v.len()-3..],&[15,16,17]);}
 #[test]#[should_panic]fn no_repite_ya_encolada(){Agenda::new().defer(16);}
 #[test]fn cinco_minutos_son_un_limite_real(){assert_eq!(remaining_ms(0),300000);assert_eq!(remaining_ms(299999),1);assert_eq!(remaining_ms(300000),0);assert_eq!(remaining_ms(300001),0);assert_eq!(remaining_ms(u128::MAX),0);}
}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
