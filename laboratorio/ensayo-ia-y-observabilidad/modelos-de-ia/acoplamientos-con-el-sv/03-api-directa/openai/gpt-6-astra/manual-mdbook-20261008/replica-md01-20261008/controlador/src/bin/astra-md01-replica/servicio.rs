//! Aplazamiento de preguntas interrumpidas, sin repetir entregas completas.
use std::collections::VecDeque;
pub struct Agenda{pending:VecDeque<usize>}
pub fn remaining_ms(elapsed:u128)->u64{300000u64.saturating_sub(elapsed.min(u64::MAX as u128) as u64)}
impl Agenda{
 pub fn new()->Self{Self{pending:(1..=1).collect()}}
 pub fn next(&mut self)->Option<usize>{self.pending.pop_front()}
 pub fn defer(&mut self,q:usize){assert!((1..=1).contains(&q));assert!(!self.pending.contains(&q));self.pending.push_back(q);}
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn orden_y_aplazamiento(){let mut a=Agenda::new();assert_eq!(a.next(),Some(1));a.defer(1);assert_eq!(std::iter::from_fn(||a.next()).collect::<Vec<_>>(),vec![1]);}
 #[test]fn limite_cinco_minutos(){assert_eq!(remaining_ms(299999),1);assert_eq!(remaining_ms(300000),0);assert_eq!(remaining_ms(u128::MAX),0);}
 #[test]#[should_panic]fn no_duplica(){Agenda::new().defer(1);}
}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
