use crate::{modelo::*, validacion::validar};
use std::collections::BTreeMap;

fn escapar(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn mapa(f: &Fuente) -> BTreeMap<&str, &str> {
    f.textos
        .iter()
        .map(|t| (t.id.as_str(), t.valor.as_str()))
        .collect()
}

fn svg(e: &Elemento, textos: &BTreeMap<&str, &str>, salida: &mut String) {
    salida.push('<');
    salida.push_str(&e.nombre);
    for (k, v) in &e.atributos {
        salida.push(' ');
        salida.push_str(k);
        salida.push_str("=\"");
        salida.push_str(&escapar(v));
        salida.push('"');
    }
    salida.push('>');
    for hijo in &e.hijos {
        match hijo {
            Nodo::Elemento { elemento } => svg(elemento, textos, salida),
            Nodo::Texto { id } => salida.push_str(&escapar(textos[id.as_str()])),
        }
    }
    salida.push_str("</");
    salida.push_str(&e.nombre);
    salida.push('>');
}

pub fn representar_svg(f: &Fuente) -> Result<String, Error> {
    validar(f)?;
    let mut s = String::new();
    svg(&f.estructura_visual, &mapa(f), &mut s);
    s.push('\n');
    Ok(s)
}

fn contenido(e: &Elemento, textos: &BTreeMap<&str, &str>, salida: &mut String) {
    for n in &e.hijos {
        match n {
            Nodo::Texto { id } => salida.push_str(textos[id.as_str()]),
            Nodo::Elemento { elemento } => contenido(elemento, textos, salida),
        }
    }
}

fn leer(e: &Elemento, textos: &BTreeMap<&str, &str>, salida: &mut Vec<(String, String)>) {
    if matches!(e.nombre.as_str(), "text" | "title" | "desc") {
        let mut s = String::new();
        contenido(e, textos, &mut s);
        let s = s.split_whitespace().collect::<Vec<_>>().join(" ");
        if !s.is_empty() {
            salida.push((e.nombre.clone(), s));
        }
    } else if !matches!(e.nombre.as_str(), "metadata" | "defs") {
        for n in &e.hijos {
            if let Nodo::Elemento { elemento } = n {
                leer(elemento, textos, salida);
            }
        }
    }
}

/// Lectura HTML codificada, traducible por la interfaz de publicación.
/// Escaped HTML reading, translatable by the publishing interface.
pub fn html(f: &Fuente) -> Result<String, Error> {
    validar(f)?;
    let mut grupos = Vec::new();
    leer(&f.estructura_visual, &mapa(f), &mut grupos);
    let mut s = String::from(
        "<div class=\"sv-figura-fuente-lectura\"><p>La figura y esta lectura proceden de una misma fuente estructurada. No se ha utilizado OCR. Las etiquetas conservan su orden documental; las relaciones científicas no se deducen de la composición gráfica.</p><ol>",
    );
    for (_, t) in grupos {
        s.push_str("<li>");
        s.push_str(&escapar(&t));
        s.push_str("</li>");
    }
    s.push_str("</ol></div>");
    Ok(s)
}

pub fn markdown(f: &Fuente) -> Result<String, Error> {
    validar(f)?;
    let mut grupos = Vec::new();
    leer(&f.estructura_visual, &mapa(f), &mut grupos);
    let mut s = String::from(
        "# Lectura de la figura\n\nFuente estructurada común; sin OCR. El orden documental no acredita relaciones espaciales.\n\n",
    );
    for (i, (_, t)) in grupos.iter().enumerate() {
        let literal = t
            .chars()
            .flat_map(|c| {
                if "\\`*_{}[]()#+-.!|<>".contains(c) {
                    vec!['\\', c]
                } else {
                    vec![c]
                }
            })
            .collect::<String>();
        s.push_str(&format!("{}. {}\n", i + 1, literal));
    }
    s.push('\n');
    s.push_str(include_str!("../LICENCIA.md").trim());
    s.push('\n');
    Ok(s)
}
