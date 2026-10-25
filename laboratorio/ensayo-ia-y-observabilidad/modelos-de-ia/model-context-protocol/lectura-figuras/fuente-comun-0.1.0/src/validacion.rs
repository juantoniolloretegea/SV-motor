use crate::modelo::*;
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn identificador(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
        && s.as_bytes()[0].is_ascii_alphabetic()
}

fn numero(s: &str, positivo: bool) -> bool {
    !s.is_empty()
        && s.len() <= 32
        && s.parse::<f64>()
            .is_ok_and(|n| n.is_finite() && n.abs() <= 100_000.0 && (!positivo || n >= 0.0))
}

fn color(s: &str) -> bool {
    matches!(s, "none" | "white" | "black" | "transparent")
        || (s.starts_with('#')
            && matches!(s.len(), 4 | 7 | 9)
            && s[1..].bytes().all(|c| c.is_ascii_hexdigit()))
}

fn atributo(nombre: &str, valor: &str, elemento: &str) -> bool {
    if valor.len() > 4096 || valor.chars().any(|c| c.is_control()) {
        return false;
    }
    match nombre {
        "xmlns" => elemento == "svg" && valor == "http://www.w3.org/2000/svg",
        "viewBox" => {
            let partes: Vec<_> = valor.split_whitespace().collect();
            partes.len() == 4
                && partes.iter().all(|s| numero(s, false))
                && partes[2]
                    .parse::<f64>()
                    .is_ok_and(|n| n > 0.0 && n <= 8192.0)
                && partes[3]
                    .parse::<f64>()
                    .is_ok_and(|n| n > 0.0 && n <= 8192.0)
        }
        "id" => identificador(valor),
        "role" => valor == "img",
        "aria-labelledby" => {
            valor.split_whitespace().count() <= 4
                && !valor.is_empty()
                && valor.split_whitespace().all(identificador)
        }
        "fill" | "stroke" => color(valor),
        "x" | "y" | "cx" | "cy" | "refX" | "refY" | "dx" | "dy" => numero(valor, false),
        "width" | "height" | "r" | "rx" | "ry" | "stroke-width" | "font-size" | "markerWidth"
        | "markerHeight" => numero(valor, true),
        "font-family" => valor == "Arial, sans-serif",
        "font-weight" => matches!(valor, "normal" | "bold" | "400" | "600" | "700"),
        "text-anchor" => matches!(valor, "start" | "middle" | "end"),
        "orient" => valor == "auto",
        "stroke-dasharray" => {
            !valor.is_empty()
                && valor.split_whitespace().count() <= 8
                && valor.split_whitespace().all(|s| numero(s, true))
        }
        "marker-end" => valor
            .strip_prefix("url(#")
            .and_then(|s| s.strip_suffix(')'))
            .is_some_and(identificador),
        "d" => {
            !valor.is_empty()
                && valor.len() <= 4096
                && valor
                    .bytes()
                    .all(|c| c.is_ascii_digit() || b"MmLlHhVvCcSsQqTtAaZzEe.,-+ \t".contains(&c))
        }
        _ => false,
    }
}

struct Estado<'a> {
    textos: BTreeMap<&'a str, &'a str>,
    usados: BTreeSet<&'a str>,
    ids: BTreeMap<&'a str, &'a str>,
    referencias: Vec<(&'a str, bool)>,
    nodos: usize,
}

fn recorrer<'a>(
    e: &'a Elemento,
    nivel: usize,
    contexto_texto: bool,
    s: &mut Estado<'a>,
) -> Result<(), Error> {
    if nivel > MAX_PROFUNDIDAD || e.atributos.len() > 32 {
        return Err(Error::Limite);
    }
    s.nodos += 1;
    if s.nodos > MAX_NODOS {
        return Err(Error::Limite);
    }
    if !matches!(
        e.nombre.as_str(),
        "svg"
            | "g"
            | "defs"
            | "marker"
            | "rect"
            | "path"
            | "circle"
            | "text"
            | "tspan"
            | "title"
            | "desc"
            | "metadata"
    ) || (nivel > 0 && e.nombre == "svg")
    {
        return Err(Error::ElementoNoAdmitido);
    }
    for (k, v) in &e.atributos {
        if !atributo(k, v, &e.nombre) {
            return Err(Error::AtributoNoAdmitido);
        }
        if k == "id" && s.ids.insert(v, &e.nombre).is_some() {
            return Err(Error::Referencia);
        }
        if k == "marker-end" {
            s.referencias.push((&v[5..v.len() - 1], true));
        }
        if k == "aria-labelledby" {
            for id in v.split_whitespace() {
                s.referencias.push((id, false));
            }
        }
    }
    let texto_admitido =
        contexto_texto || matches!(e.nombre.as_str(), "text" | "title" | "desc" | "metadata");
    if e.nombre == "tspan" && !contexto_texto {
        return Err(Error::Formato);
    }
    for hijo in &e.hijos {
        match hijo {
            Nodo::Elemento { elemento } => {
                if texto_admitido
                    && (elemento.nombre != "tspan"
                        || !matches!(e.nombre.as_str(), "text" | "tspan"))
                {
                    return Err(Error::Formato);
                }
                recorrer(elemento, nivel + 1, texto_admitido, s)?;
            }
            Nodo::Texto { id } => {
                s.nodos += 1;
                if s.nodos > MAX_NODOS {
                    return Err(Error::Limite);
                }
                if !texto_admitido || !s.textos.contains_key(id.as_str()) || !s.usados.insert(id) {
                    return Err(Error::Referencia);
                }
            }
        }
    }
    Ok(())
}

/// Admite sólo una figura estática; los datos no pueden introducir código, red o marcado.
/// Accepts a static figure only; data cannot introduce code, network access or markup.
pub fn validar(f: &Fuente) -> Result<(), Error> {
    if f.esquema != "sv-figura-fuente-1"
        || f.relaciones != "no_inferidas"
        || f.original_sha256.len() != 64
        || !f.original_sha256.bytes().all(|c| c.is_ascii_hexdigit())
        || f.estructura_visual.nombre != "svg"
        || f.estructura_visual
            .atributos
            .get("xmlns")
            .map(String::as_str)
            != Some("http://www.w3.org/2000/svg")
        || !f.estructura_visual.atributos.contains_key("viewBox")
    {
        return Err(Error::Formato);
    }
    if f.textos.len() > MAX_TEXTOS {
        return Err(Error::Limite);
    }
    let mut s = Estado {
        textos: BTreeMap::new(),
        usados: BTreeSet::new(),
        ids: BTreeMap::new(),
        referencias: Vec::new(),
        nodos: 0,
    };
    let mut bytes = 0usize;
    for t in &f.textos {
        bytes = bytes.checked_add(t.valor.len()).ok_or(Error::Limite)?;
        if bytes > MAX_TEXTO {
            return Err(Error::Limite);
        }
        if !identificador(&t.id)
            || s.textos.insert(&t.id, &t.valor).is_some()
            || t.valor
                .chars()
                .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
        {
            return Err(Error::Formato);
        }
    }
    recorrer(&f.estructura_visual, 0, false, &mut s)?;
    if s.usados.len() != s.textos.len()
        || s.referencias
            .iter()
            .any(|(id, marker)| match s.ids.get(id) {
                Some(nombre) => *marker && *nombre != "marker",
                None => true,
            })
    {
        return Err(Error::Referencia);
    }
    Ok(())
}
