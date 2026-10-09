//! Preparación acotada de una biblioteca para el MCP existente.
//! La política es una entrada del controlador, nunca una herramienta del candidato.
#![forbid(unsafe_code)]
pub mod conversion;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashSet, fs, path::{Component, Path, PathBuf}, time::Instant};
use sv_mcp_documental::{libro::{self, Plan}, parse_strict, read_bounded, sha256,
    Catalog, Document, FuenteAutorizada, Section};

pub const MAX_MANIFEST: usize = 256 * 1024;
const MAX_ORIGINAL: usize = 8 * 1024 * 1024;
pub const INDICE: &str = "BIBLIOTECA_INDICE";
pub const LICENCIA: &str = "© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Identidad { pub ruta: String, pub sha256: String }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Tipo { Markdown, Html, Pdf, Docx }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Visibilidad { Publica, Restringida }
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadato {
    pub documento: String,
    pub tipo_original: Tipo,
    pub original: Identidad,
    pub procedencia: String,
    pub licencia: String,
    pub sintetico: bool,
    pub visibilidad: Visibilidad,
    pub alcance: String,
    pub correspondencia: String,
    pub identificador_editorial_r0: Option<String>,
    /// Recibo exterior de una conversión revisada. Su identidad queda ligada a la política.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recepcion_conversion: Option<Identidad>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Libro {
    pub id: String,
    pub titulo: String,
    pub raiz: String,
    pub plan: Identidad,
    pub metadatos: Vec<Metadato>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Biblioteca {
    pub version: u32,
    pub edicion: String,
    pub estado: String,
    pub libros: Vec<Libro>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Politica {
    pub version: u32,
    pub biblioteca_sha256: String,
    pub nodo: u8,
    pub finalidad: String,
    pub libros_permitidos: Vec<String>,
    pub permitir_sinteticos: bool,
    pub permitir_restringidos: bool,
}
pub struct Preparacion {
    pub catalogo: Catalog,
    pub fuentes: Vec<FuenteAutorizada>,
    pub mapa: Value,
}
/// El hash de este recibo lo fija el controlador fuera del corpus del candidato.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recibo {
    pub version: u32,
    pub politica: Identidad,
    pub catalogo: Identidad,
    pub fuentes: Identidad,
    pub mapa: Identidad,
    pub documentos: usize,
    pub secciones: usize,
}
pub fn raiz_segura(path: &Path) -> Result<PathBuf, String> {
    let absolute = std::path::absolute(path).map_err(|e| e.to_string())?;
    for ancestor in absolute.ancestors() {
        let m = fs::symlink_metadata(ancestor).map_err(|e| e.to_string())?;
        if m.file_type().is_symlink() { return Err("RAIZ_SIMBOLICA".into()); }
        #[cfg(windows)] {
            use std::os::windows::fs::MetadataExt;
            if m.file_attributes() & 0x400 != 0 { return Err("RAIZ_REPARSE".into()); }
        }
    }
    fs::canonicalize(absolute).map_err(|e| e.to_string())
}
pub fn recibir_autorizacion(dir: &Path, hash: &str) -> Result<(Recibo, Politica, Catalog), String> {
    let root = raiz_segura(dir)?;
    let raw = leer_identidad(&root, &Identidad { ruta:"RECIBO.json".into(), sha256:hash.into() }, 65536)?;
    let r: Recibo = leer_json(&raw)?;
    if r.version != 1 || r.politica.ruta != "POLITICA.json" || r.catalogo.ruta != "CATALOGO.json"
        || r.fuentes.ruta != "FUENTES.json" || r.mapa.ruta != "MAPA.json" {
        return Err("RECIBO_INVALIDO".into());
    }
    let p: Politica = leer_json(&leer_identidad(&root, &r.politica, 65536)?)?;
    if p.version != 1 || !(1..=3).contains(&p.nodo) || p.finalidad != "verificacion_local" {
        return Err("POLITICA_FUERA_DE_RECEPCION".into());
    }
    let c: Catalog = leer_json(&leer_identidad(&root, &r.catalogo, sv_mcp_documental::MAX_CATALOG)?)?;
    let sources: Vec<FuenteAutorizada> = leer_json(&leer_identidad(&root, &r.fuentes, 65536)?)?;
    let mapa: Value = leer_json(&leer_identidad(&root, &r.mapa, sv_mcp_documental::MAX_CATALOG)?)?;
    if mapa["biblioteca_sha256"] != p.biblioteca_sha256 || mapa["nodo_declarado"] != p.nodo
        || c.documents.len() != r.documentos || c.documents.iter().map(|d| d.sections.len()).sum::<usize>() != r.secciones {
        return Err("RECIBO_DISCORDANTE".into());
    }
    c.validate_with_sources(p.permitir_sinteticos, &sources)?;
    Ok((r, p, c))
}
fn id_ok(s: &str) -> bool {
    !s.is_empty() && s.len() <= 60 && s.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
}
fn hash_ok(s: &str) -> bool { s.len() == 64 && s.bytes().all(|c| c.is_ascii_hexdigit()) }
fn text_ok(s: &str, max: usize) -> bool {
    !s.trim().is_empty() && s.len() <= max && !s.chars().any(|c| c.is_control())
}
pub fn ruta_segura(root: &Path, relative: &str) -> Result<PathBuf, String> {
    if relative.is_empty() || relative.len() > 240 || relative.contains(['\\', ':', '%', '?', '#', '\0'])
        || relative.split('/').any(|c| c.is_empty() || c == "." || c == "..")
        || Path::new(relative).components().any(|c| !matches!(c, Component::Normal(_))) {
        return Err("RUTA_NO_ADMITIDA".into());
    }
    let mut path = root.to_path_buf();
    for part in Path::new(relative).components() {
        path.push(part);
        let meta = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
        if meta.file_type().is_symlink() { return Err("ENLACE_NO_ADMITIDO".into()); }
        #[cfg(windows)] {
            use std::os::windows::fs::MetadataExt;
            if meta.file_attributes() & 0x400 != 0 { return Err("REPARSE_NO_ADMITIDO".into()); }
        }
    }
    let path = fs::canonicalize(path).map_err(|e| e.to_string())?;
    if !path.starts_with(root) { return Err("FUERA_DEL_PERIMETRO".into()); }
    Ok(path)
}
pub fn leer_identidad(root: &Path, file: &Identidad, max: usize) -> Result<Vec<u8>, String> {
    if !hash_ok(&file.sha256) { return Err("HUELLA_INVALIDA".into()); }
    let raw = read_bounded(&ruta_segura(root, &file.ruta)?, max).map_err(|e| e.to_string())?;
    if sha256(&raw) != file.sha256 { return Err("HUELLA_NO_COINCIDE".into()); }
    Ok(raw)
}
pub fn leer_json<T: serde::de::DeserializeOwned>(raw: &[u8]) -> Result<T, String> {
    serde_json::from_value(parse_strict(raw).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}
fn comprobar_tipo(root: &Path, meta: &Metadato, raw: &[u8], text: &str) -> Result<(), String> {
    let ok = match meta.tipo_original {
        Tipo::Markdown => meta.original.ruta.ends_with(".md") && raw == text.as_bytes()
            && meta.correspondencia == "identidad_literal",
        Tipo::Pdf => meta.original.ruta.ends_with(".pdf") && raw.starts_with(b"%PDF-"),
        Tipo::Docx => meta.original.ruta.ends_with(".docx") && raw.starts_with(b"PK\x03\x04"),
        Tipo::Html => meta.original.ruta.ends_with(".html") && std::str::from_utf8(raw).is_ok_and(|s|
            s.to_lowercase().contains("<html") && s.to_lowercase().contains("</html>")),
    };
    if !ok { return Err("TIPO_O_CORRESPONDENCIA_INCOMPATIBLE".into()); }
    // Las firmas son comprobaciones preliminares: no sustituyen al lector del formato.
    if meta.tipo_original != Tipo::Markdown {
        if let Some(identity) = &meta.recepcion_conversion {
            conversion::recibir(root, identity, meta, raw, text)?;
        } else if !meta.sintetico || meta.correspondencia != "documento_completo_sin_mapa_fino" {
            return Err("CONVERSION_FUERA_DE_RECEPCION_INICIAL".into());
        }
    } else if meta.recepcion_conversion.is_some() {
        return Err("RECEPCION_INNECESARIA_PARA_IDENTIDAD_LITERAL".into());
    }
    Ok(())
}

pub fn preparar(root: &Path, manifest_raw: &[u8], politica: &Politica) -> Result<Preparacion, String> {
    let start = Instant::now();
    if manifest_raw.len() > MAX_MANIFEST || sha256(manifest_raw) != politica.biblioteca_sha256 {
        return Err("EDICION_NO_AUTORIZADA".into());
    }
    let library: Biblioteca = leer_json(manifest_raw)?;
    if library.version != 1 || !id_ok(&library.edicion) || library.estado != "preparacion_local"
        || library.libros.is_empty() || library.libros.len() > 8 {
        return Err("BIBLIOTECA_INVALIDA".into());
    }
    if politica.version != 1 || !(1..=3).contains(&politica.nodo)
        || politica.finalidad != "verificacion_local" || politica.libros_permitidos.is_empty()
        || politica.libros_permitidos.len() > 8 {
        return Err("POLITICA_FUERA_DE_RECEPCION".into());
    }
    let allowed: HashSet<_> = politica.libros_permitidos.iter().collect();
    let books: HashSet<_> = library.libros.iter().map(|b| &b.id).collect();
    if books.len() != library.libros.len() || allowed.len() != politica.libros_permitidos.len()
        || !allowed.is_subset(&books) { return Err("LISTA_DE_LIBROS_INVALIDA".into()); }
    let root = raiz_segura(root)?;
    let mut documents = Vec::new();
    let mut sources = Vec::new();
    let mut maps = Vec::new();
    let mut identities = HashSet::from([INDICE.to_string()]);
    let mut nav = Vec::new();
    let mut total_bytes = 0;
    for book in &library.libros {
        if !allowed.contains(&book.id) { continue; }
        if start.elapsed().as_secs() >= 30 { return Err("PLAZO_DE_PREPARACION".into()); }
        if !id_ok(&book.id) || !text_ok(&book.titulo, 180) { return Err("LIBRO_INVALIDO".into()); }
        // Toda autorización se comprueba antes de abrir el plan o su contenido.
        for meta in &book.metadatos {
            if meta.visibilidad == Visibilidad::Restringida && !politica.permitir_restringidos {
                return Err("DOCUMENTO_RESTRINGIDO_NO_AUTORIZADO".into());
            }
            if meta.sintetico && !politica.permitir_sinteticos { return Err("SINTETICO_NO_AUTORIZADO".into()); }
        }
        let plan_raw = leer_identidad(&root, &book.plan, 64 * 1024)?;
        let plan: Plan = leer_json(&plan_raw)?;
        let expected: HashSet<_> = std::iter::once(&plan.summary).chain(plan.capitulos.iter()).map(|f| f.id.as_str()).collect();
        let actual: HashSet<_> = book.metadatos.iter().map(|m| m.documento.as_str()).collect();
        if actual != expected || actual.len() != book.metadatos.len() {
            return Err("METADATOS_INCOMPLETOS_O_DUPLICADOS".into());
        }
        let corpus = ruta_segura(&root, &book.raiz)?;
        let mut prepared = libro::preparar(&corpus, &plan)?;
        if book.metadatos.len() != prepared.catalogo.documents.len() { return Err("METADATOS_INCOMPLETOS".into()); }
        let meta_ids: HashSet<_> = book.metadatos.iter().map(|m| &m.documento).collect();
        if meta_ids.len() != book.metadatos.len() { return Err("METADATOS_DUPLICADOS".into()); }
        let mut navigation = format!("# {}\n\nLibro: {}. Edición: {}.\n\n", book.titulo, book.id, library.edicion);
        let mut book_map = Vec::new();
        for document in &mut prepared.catalogo.documents {
            let meta = book.metadatos.iter().find(|m| m.documento == document.id).ok_or("METADATO_AUSENTE")?;
            if !identities.insert(document.id.clone()) { return Err("DOCUMENTO_DUPLICADO".into()); }
            if !text_ok(&meta.licencia, 1200) || !text_ok(&meta.procedencia, 1800) || !text_ok(&meta.alcance, 1600)
                || meta.identificador_editorial_r0.as_ref().is_some_and(|s| !text_ok(s, 200)) {
                return Err("PROCEDENCIA_INCOMPLETA".into());
            }
            if meta.visibilidad == Visibilidad::Restringida && !politica.permitir_restringidos {
                return Err("DOCUMENTO_RESTRINGIDO_NO_AUTORIZADO".into());
            }
            if meta.sintetico && !politica.permitir_sinteticos { return Err("SINTETICO_NO_AUTORIZADO".into()); }
            let raw = leer_identidad(&root, &meta.original, MAX_ORIGINAL)?;
            let text = document.sections.iter().map(|s| s.text.as_str()).collect::<String>();
            if sha256(text.as_bytes()) != document.raw_sha256 { return Err("RECONSTRUCCION_DISCORDANTE".into()); }
            comprobar_tipo(&root, meta, &raw, &text)?;
            total_bytes += text.len();
            if total_bytes > libro::MAX_TEXT { return Err("BIBLIOTECA_SUPERA_COTA".into()); }
            document.synthetic = meta.sintetico;
            if meta.sintetico { document.url = "urn:sv:prueba-sintetica".into(); }
            // La edición es inmutable; las horas de acceso pertenecen al diario, no al corpus.
            document.retrieved_utc = "edicion_fijada_por_sha256".into();
            document.updated_source = Some(format!("edicion={}; libro={}", library.edicion, book.id));
            navigation.push_str(&format!("## {}\n\nDocumento: {}. SHA-256: {}. Sintético: {}.\n\nLicencia: {}\n\nAlcance: {}\n\n",
                document.title, document.id, document.raw_sha256, meta.sintetico, meta.licencia, meta.alcance));
            for section in &document.sections {
                navigation.push_str(&format!("- {}: {}\n", section.id, section.title));
            }
            navigation.push('\n');
            book_map.push(json!({"documento":document.id,"metadatos":meta,"markdown_sha256":document.raw_sha256,
                "secciones":document.sections.iter().map(|s|json!({"id":s.id,"titulo":s.title,"sha256":s.sha256,"bytes":s.text.len()})).collect::<Vec<_>>()}));
        }
        nav.push(Section { id: book.id.clone(), title: book.titulo.clone(), sha256: sha256(navigation.as_bytes()), text: navigation });
        maps.push(json!({"libro":book.id,"plan_sha256":book.plan.sha256,"documentos":book_map}));
        documents.extend(prepared.catalogo.documents);
        sources.extend(prepared.fuentes);
    }
    let overview = format!("# Índice de la biblioteca documental del SV\n\nEdición {}. Finalidad: verificación local.\n\nCada entrada indica una sección de BIBLIOTECA_INDICE; allí constan los documentos y todas sus secciones. Use leer_documento y recorra siguiente_pagina hasta completar cada sección necesaria. Los enlaces de procedencia no autorizan acceso a Internet.\n\n{}\n{}\n",
        library.edicion, nav.iter().map(|s|format!("- {}: {}\n",s.id,s.title)).collect::<String>(), LICENCIA);
    nav.insert(0, Section { id:"inicio".into(), title:"Índice general".into(), sha256:sha256(overview.as_bytes()), text:overview });
    let nav_text = nav.iter().map(|s|s.text.as_str()).collect::<String>();
    let nav_hash = sha256(nav_text.as_bytes());
    let nav_url = format!("urn:sv:documento:sha256:{nav_hash}");
    sources.push(FuenteAutorizada { documento: INDICE.into(), url: nav_url.clone(), sha256: nav_hash.clone() });
    documents.insert(0, Document { id: INDICE.into(), title:"Índice de la biblioteca documental del SV".into(),
        url:nav_url, retrieved_utc:"edicion_fijada_por_sha256".into(), updated_source:Some(library.edicion.clone()),
        raw_sha256:nav_hash, synthetic:false, sections:nav });
    let catalog = Catalog { version:1, documents };
    catalog.validate_with_sources(politica.permitir_sinteticos, &sources)?;
    if serde_json::to_vec(&catalog).map_err(|e|e.to_string())?.len() > sv_mcp_documental::MAX_CATALOG {
        return Err("CATALOGO_SUPERA_COTA".into());
    }
    Ok(Preparacion { catalogo:catalog, fuentes:sources,
        mapa:json!({"edicion":library.edicion,"biblioteca_sha256":politica.biblioteca_sha256,
            "finalidad":politica.finalidad,"nodo_declarado":politica.nodo,"libros":maps,
            "entrada":{"documento":INDICE,"seccion":"inicio"},"autoridad":"politica externa fijada por el controlador; no autenticacion del decisor",
            "produccion_admitida":false,"inferencia":false,"arbitro_integral_ejecutado":false}) })
}
