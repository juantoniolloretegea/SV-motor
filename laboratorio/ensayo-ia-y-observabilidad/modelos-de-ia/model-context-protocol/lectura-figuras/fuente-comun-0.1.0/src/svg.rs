use crate::modelo::*;
use crate::validacion::validar;
use quick_xml::{
    Reader,
    events::{BytesStart, Event},
};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

fn elemento(e: &BytesStart<'_>) -> Result<Elemento, Error> {
    let nombre = std::str::from_utf8(e.name().as_ref())
        .map_err(|_| Error::Formato)?
        .to_owned();
    let mut atributos = BTreeMap::new();
    for a in e.attributes() {
        let a = a.map_err(|_| Error::Formato)?;
        let k = std::str::from_utf8(a.key.as_ref())
            .map_err(|_| Error::Formato)?
            .to_owned();
        let v = a.unescape_value().map_err(|_| Error::Formato)?.into_owned();
        if atributos.insert(k, v).is_some() {
            return Err(Error::Formato);
        }
    }
    Ok(Elemento {
        nombre,
        atributos,
        hijos: Vec::new(),
    })
}

fn insertar(e: Elemento, pila: &mut [Elemento], raiz: &mut Option<Elemento>) -> Result<(), Error> {
    if let Some(padre) = pila.last_mut() {
        padre.hijos.push(Nodo::Elemento { elemento: e });
    } else if raiz.replace(e).is_some() {
        return Err(Error::Formato);
    }
    Ok(())
}

fn texto(
    s: String,
    pila: &mut [Elemento],
    textos: &mut Vec<Texto>,
    total: &mut usize,
) -> Result<(), Error> {
    let dentro_texto = pila
        .iter()
        .any(|e| matches!(e.nombre.as_str(), "text" | "title" | "desc" | "metadata"));
    if !dentro_texto {
        return if s.trim().is_empty() {
            Ok(())
        } else {
            Err(Error::Formato)
        };
    }
    *total = total.checked_add(s.len()).ok_or(Error::Limite)?;
    if *total > MAX_TEXTO || textos.len() >= MAX_TEXTOS {
        return Err(Error::Limite);
    }
    let padre = pila.last_mut().ok_or(Error::Formato)?;
    let id = format!("t{:04}", textos.len() + 1);
    padre.hijos.push(Nodo::Texto { id: id.clone() });
    textos.push(Texto { id, valor: s });
    Ok(())
}

/// Importa sólo estructura explícita. Las flechas no se convierten en relaciones semánticas.
/// Imports explicit structure only. Arrows are not converted into semantic relations.
pub fn importar(bytes: &[u8]) -> Result<Fuente, Error> {
    if bytes.len() > MAX_ENTRADA {
        return Err(Error::Limite);
    }
    let xml = std::str::from_utf8(bytes).map_err(|_| Error::Formato)?;
    let mut lector = Reader::from_str(xml);
    let mut pila: Vec<Elemento> = Vec::new();
    let mut raiz = None;
    let mut textos = Vec::new();
    let mut total = 0usize;
    let mut nodos = 0usize;
    let mut declaracion = false;
    loop {
        match lector.read_event().map_err(|_| Error::Formato)? {
            Event::Start(e) => {
                nodos += 1;
                if nodos > MAX_NODOS || pila.len() >= MAX_PROFUNDIDAD {
                    return Err(Error::Limite);
                }
                pila.push(elemento(&e)?);
            }
            Event::Empty(e) => {
                nodos += 1;
                if nodos > MAX_NODOS {
                    return Err(Error::Limite);
                }
                insertar(elemento(&e)?, &mut pila, &mut raiz)?;
            }
            Event::End(_) => {
                let e = pila.pop().ok_or(Error::Formato)?;
                insertar(e, &mut pila, &mut raiz)?;
            }
            Event::Text(e) => texto(
                e.unescape().map_err(|_| Error::Formato)?.into_owned(),
                &mut pila,
                &mut textos,
                &mut total,
            )?,
            Event::CData(e) => {
                if pila.is_empty() {
                    return Err(Error::Formato);
                }
                texto(
                    std::str::from_utf8(e.as_ref())
                        .map_err(|_| Error::Formato)?
                        .to_owned(),
                    &mut pila,
                    &mut textos,
                    &mut total,
                )?;
            }
            Event::Decl(e) => {
                if declaracion
                    || nodos != 0
                    || e.version().map_err(|_| Error::Formato)?.as_ref() != b"1.0"
                {
                    return Err(Error::Formato);
                }
                if let Some(enc) = e.encoding()
                    && !enc
                        .map_err(|_| Error::Formato)?
                        .as_ref()
                        .eq_ignore_ascii_case(b"UTF-8")
                {
                    return Err(Error::Formato);
                }
                declaracion = true;
            }
            Event::DocType(_) | Event::PI(_) => return Err(Error::ElementoNoAdmitido),
            Event::Eof => break,
            _ => {}
        }
    }
    if !pila.is_empty() {
        return Err(Error::Formato);
    }
    let f = Fuente {
        esquema: "sv-figura-fuente-1".into(),
        original_sha256: format!("{:x}", Sha256::digest(bytes)),
        relaciones: "no_inferidas".into(),
        textos,
        estructura_visual: raiz.ok_or(Error::Formato)?,
    };
    validar(&f)?;
    Ok(f)
}
