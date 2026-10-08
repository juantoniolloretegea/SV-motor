use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Number, Value};
use std::fmt;

struct Unico(Value);
impl<'de> Deserialize<'de> for Unico {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Unico;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("JSON sin claves repetidas") }
            fn visit_bool<E: de::Error>(self, v: bool) -> Result<Unico,E> { Ok(Unico(Value::Bool(v))) }
            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Unico,E> { Ok(Unico(Value::Number(v.into()))) }
            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Unico,E> { Ok(Unico(Value::Number(v.into()))) }
            fn visit_f64<E: de::Error>(self, v: f64) -> Result<Unico,E> { Number::from_f64(v).map(|v|Unico(Value::Number(v))).ok_or_else(||E::custom("Número no finito")) }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<Unico,E> { Ok(Unico(Value::String(v.into()))) }
            fn visit_string<E: de::Error>(self, v: String) -> Result<Unico,E> { Ok(Unico(Value::String(v))) }
            fn visit_none<E: de::Error>(self) -> Result<Unico,E> { Ok(Unico(Value::Null)) }
            fn visit_unit<E: de::Error>(self) -> Result<Unico,E> { Ok(Unico(Value::Null)) }
            fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Unico,A::Error> {
                let mut v=Vec::new(); while let Some(e)=a.next_element::<Unico>()? { v.push(e.0); }
                Ok(Unico(Value::Array(v)))
            }
            fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> Result<Unico,A::Error> {
                let mut v=Map::new();
                while let Some(k)=a.next_key::<String>()? {
                    if v.contains_key(&k) { return Err(de::Error::custom(format!("Clave repetida: {k}"))); }
                    v.insert(k,a.next_value::<Unico>()?.0);
                }
                Ok(Unico(Value::Object(v)))
            }
        }
        d.deserialize_any(V)
    }
}
pub fn parse(b: &[u8]) -> Result<Value, serde_json::Error> {
    let mut d=serde_json::Deserializer::from_slice(b);
    let v=Unico::deserialize(&mut d)?; d.end()?; Ok(v.0)
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn repeticion_anidada() { assert!(parse(br#"{"e":[{"p":0,"p":1}]}"#).is_err()); }
    #[test] fn exterior() { assert!(parse(b"{} {}").is_err()); assert!(parse(b"```json\n{}\n```").is_err()); assert!(parse(b" \n{}\t").is_ok()); }
    #[test] fn conserva_tipos() { let s=br#"{"a":[null,true,-1,2,0.5,"x"]}"#; assert_eq!(parse(s).unwrap(),serde_json::from_slice::<Value>(s).unwrap()); }
}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
