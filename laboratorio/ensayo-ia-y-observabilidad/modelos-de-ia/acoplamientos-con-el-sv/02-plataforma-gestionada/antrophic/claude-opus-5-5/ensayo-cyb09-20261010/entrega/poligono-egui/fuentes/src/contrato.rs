use serde_json::{json,Value};
use sha2::{Digest,Sha256};
pub fn huella(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tri {
    Cero,
    Uno,
    U,
}
impl Tri {
    pub fn leer(s: &str) -> Result<Self, String> {
        match s {
            "0" => Ok(Self::Cero),
            "1" => Ok(Self::Uno),
            "U" => Ok(Self::U),
            _ => Err("Símbolo no admisible".into()),
        }
    }
    pub fn simbolo(self) -> &'static str {
        match self {
            Self::Cero => "0",
            Self::Uno => "1",
            Self::U => "U",
        }
    }
    pub fn radio(self) -> u8 {
        match self {
            Self::Cero => 1,
            Self::Uno => 2,
            Self::U => 3,
        }
    }
    pub fn nombre_color(self) -> &'static str {
        match self {
            Self::Cero => "rojo",
            Self::Uno => "verde",
            Self::U => "azul",
        }
    }
    // Tonos de realización accesibles; la fuente prescribe los nombres, no estos valores RGB.
    pub fn rgb(self) -> [u8; 3] {
        match self {
            Self::Cero => [181, 42, 45],
            Self::Uno => [21, 119, 80],
            Self::U => [36, 87, 181],
        }
    }
}

pub fn clasificar(v: &[Tri]) -> Result<([usize; 3], usize, &'static str), String> {
    let n = v.len();
    let b = (3..=n)
        .take_while(|b| b.checked_mul(*b).is_some_and(|s| s <= n))
        .find(|b| b * b == n);
    if b.is_none() {
        return Err("Se exige n=b² y b≥3; no se clasifica un vector parcial".into());
    }
    let mut rec = [0; 3];
    for x in v {
        rec[match x {
            Tri::Cero => 0,
            Tri::Uno => 1,
            Tri::U => 2,
        }] += 1;
    }
    let t = n.checked_mul(7).ok_or("Desbordamiento del umbral")? / 9;
    let k = if rec[1] >= t {
        "No apto"
    } else if rec[0] >= t {
        "Apto"
    } else {
        "Indeterminado"
    };
    Ok((rec, t, k))
}

pub fn control_criticidad(v: &[Tri], criticos: &[Option<bool>]) -> Result<&'static str, String> {
    if v.len() != criticos.len() {
        return Err("Criticidades y posiciones no corresponden".into());
    }
    if v.iter()
        .zip(criticos)
        .any(|(x, c)| *x == Tri::Uno && *c == Some(true))
    {
        return Ok("No apto");
    }
    if v.iter().zip(criticos).any(|(x,c)| *x==Tri::U && *c==Some(true)) { return Ok("Indeterminada"); }
    if criticos.iter().any(Option::is_none) {
        return Ok("No acreditada");
    }
    // Satisfacer esta comprobación no acredita las restantes condiciones del contrato.
    Ok("Sin error crítico acreditado; aplicar las demás condiciones del ensayo")
}


pub const CONTRATO:&str="SV-CLAUDE-CYB09-1.0";

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
