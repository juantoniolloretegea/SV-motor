//! Preparación documental Markdown. Sin navegación, ejecución de ejemplos ni macros.
#![forbid(unsafe_code)]
use crate::{parse_strict, read_bounded, sha256, Catalog, Document, FuenteAutorizada, Section};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::HashSet,
    fs,
    io::Write,
    path::{Component, Path},
    time::Instant,
};

pub const MAX_TEXT: usize = 1024 * 1024;
pub const MAX_CHAPTER: usize = 256 * 1024;
pub const MAX_CHAPTERS: usize = 63;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Archivo {
    pub id: String,
    pub ruta: String,
    pub titulo: String,
    pub url: String,
    pub sha256: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    pub version: u32,
    pub libro: String,
    pub revision: String,
    pub summary: Archivo,
    pub capitulos: Vec<Archivo>,
}
pub struct Preparado {
    pub catalogo: Catalog,
    pub fuentes: Vec<FuenteAutorizada>,
    pub mapa: Value,
}

fn relative(s: &str) -> Result<(), String> {
    if s.is_empty()
        || s.len() > 240
        || s.contains(['\\', ':', '%', '?', '#', '\0'])
        || !s.ends_with(".md")
        || s.split('/').any(|p| p.is_empty() || p == "." || p == "..")
        || Path::new(s)
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err("RUTA_MARKDOWN_NO_ADMITIDA".into());
    }
    Ok(())
}
fn source(root: &Path, f: &Archivo) -> Result<String, String> {
    relative(&f.ruta)?;
    if !crate::valid_id(&f.id)
        || f.titulo.is_empty()
        || f.titulo.chars().count() > 200
        || !crate::hash_ok(&f.sha256)
        || !f.url.starts_with("https://")
        || !f.url.ends_with(".md")
        || f.url.len() > 1200
    {
        return Err("METADATOS_MARKDOWN_INVALIDOS".into());
    }
    // Se rechazan enlaces en cada componente, incluidos los que apuntan dentro del corpus.
    let mut path = root.to_path_buf();
    for c in Path::new(&f.ruta).components() {
        path.push(c);
        let m = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
        if m.file_type().is_symlink() {
            return Err("ENLACE_SIMBOLICO_NO_ADMITIDO".into());
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            if m.file_attributes() & 0x400 != 0 {
                return Err("REPARSE_POINT_NO_ADMITIDO".into());
            }
        }
    }
    let canonical = fs::canonicalize(&path).map_err(|e| e.to_string())?;
    if !canonical.starts_with(root) {
        return Err("FUERA_DEL_CORPUS".into());
    }
    if !fs::metadata(&canonical)
        .map_err(|e| e.to_string())?
        .is_file()
    {
        return Err("FUENTE_NO_REGULAR".into());
    }
    let bytes = read_bounded(&canonical, MAX_CHAPTER).map_err(|e| e.to_string())?;
    if sha256(&bytes) != f.sha256 {
        return Err("HUELLA_ORIGINAL_NO_COINCIDE".into());
    }
    let text = String::from_utf8(bytes).map_err(|_| "MARKDOWN_NO_UTF8")?;
    if text.trim().is_empty() || text.contains('\0') {
        return Err("TEXTO_VACIO_O_NULO".into());
    }
    if text.contains("{{#") || text.contains("{{ #") {
        return Err("MACRO_NO_ADMITIDA_EN_ESTA_EDICION".into());
    }
    for event in Parser::new(&text) {
        if matches!(
            event,
            Event::Html(_) | Event::InlineHtml(_) | Event::Start(Tag::Image { .. })
        ) {
            return Err("CONTENIDO_VISUAL_O_HTML_REQUIERE_OTRA_RECEPCION".into());
        }
    }
    Ok(text)
}

fn sections(text: &str) -> Result<Vec<Section>, String> {
    let mut headings = Vec::<(usize, String)>::new();
    let mut heading = None;
    for (event, range) in Parser::new(text).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { .. }) => heading = Some((range.start, String::new())),
            Event::Text(t) | Event::Code(t) => {
                if let Some((_, s)) = &mut heading {
                    s.push_str(&t);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(h) = heading.take() {
                    headings.push(h);
                }
            }
            _ => {}
        }
    }
    if headings.first().is_none_or(|h| h.0 != 0) {
        headings.insert(0, (0, "Preámbulo".into()));
    }
    if headings.len() > 64 {
        return Err("EXCESO_DE_SECCIONES".into());
    }
    let mut output = Vec::new();
    for (i, (start, title)) in headings.iter().enumerate() {
        if title.is_empty() || title.chars().count() > 200 {
            return Err("TITULO_DE_SECCION_INVALIDO".into());
        }
        let end = headings.get(i + 1).map(|h| h.0).unwrap_or(text.len());
        let part = &text[*start..end];
        let first = text[..*start].bytes().filter(|b| *b == b'\n').count() + 1;
        let last = first + part.bytes().filter(|b| *b == b'\n').count()
            - usize::from(part.ends_with('\n'));
        output.push(Section {
            id: format!("MD-L{first:06}-L{last:06}"),
            title: title.clone(),
            text: part.into(),
            sha256: sha256(part.as_bytes()),
        });
    }
    if output.iter().map(|s| s.text.as_str()).collect::<String>() != text {
        return Err("RECONSTRUCCION_NO_IDENTICA".into());
    }
    Ok(output)
}

pub fn preparar(root: &Path, plan: &Plan) -> Result<Preparado, String> {
    let t = Instant::now();
    if plan.version != 1
        || !crate::valid_id(&plan.libro)
        || plan.revision.len() != 40
        || !plan.revision.bytes().all(|c| c.is_ascii_hexdigit())
        || plan.summary.ruta != "SUMMARY.md"
        || plan.capitulos.is_empty()
        || plan.capitulos.len() > MAX_CHAPTERS
    {
        return Err("PLAN_DE_LIBRO_INVALIDO".into());
    }
    if fs::symlink_metadata(root)
        .map_err(|e| e.to_string())?
        .file_type()
        .is_symlink()
    {
        return Err("RAIZ_SIMBOLICA".into());
    }
    let root = fs::canonicalize(root).map_err(|e| e.to_string())?;
    let mut ids = HashSet::new();
    let mut paths = HashSet::new();
    for f in std::iter::once(&plan.summary).chain(&plan.capitulos) {
        if !ids.insert(&f.id) || !paths.insert(&f.ruta) {
            return Err("IDENTIDAD_DUPLICADA".into());
        }
    }
    let summary = source(&root, &plan.summary)?;
    let mut links = Vec::new();
    let mut drafts = 0;
    for e in Parser::new(&summary) {
        if let Event::Start(Tag::Link { dest_url, .. }) = e {
            if dest_url.is_empty() {
                drafts += 1;
                continue;
            }
            relative(&dest_url)?;
            links.push(dest_url.to_string());
        }
    }
    if links
        != plan
            .capitulos
            .iter()
            .map(|f| f.ruta.clone())
            .collect::<Vec<_>>()
    {
        return Err("INDICE_Y_PLAN_NO_CONCUERDAN".into());
    }
    let mut docs = Vec::new();
    let mut fuentes = Vec::new();
    let mut mapping = Vec::new();
    let mut total = 0;
    for f in std::iter::once(&plan.summary).chain(&plan.capitulos) {
        if t.elapsed().as_secs() >= 30 {
            return Err("PLAZO_DE_PREPARACION_AGOTADO".into());
        }
        let text = if f.ruta == "SUMMARY.md" {
            summary.clone()
        } else {
            source(&root, f)?
        };
        total += text.len();
        if total > MAX_TEXT {
            return Err("CORPUS_SUPERA_COTA".into());
        }
        let section = sections(&text)?;
        mapping.push(json!({"documento":f.id,"archivo":f.ruta,"url":f.url,"sha256":f.sha256,"bytes":text.len(),"secciones":section.iter().map(|s|json!({"id":s.id,"titulo":s.title,"sha256":s.sha256,"bytes":s.text.len()})).collect::<Vec<_>>() }));
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs();
        docs.push(Document {
            id: f.id.clone(),
            title: f.titulo.clone(),
            url: f.url.clone(),
            retrieved_utc: format!("unix:{now}"),
            updated_source: Some(format!(
                "libro={}; base_documental={}; identidad=huellas_del_plan",
                plan.libro, plan.revision
            )),
            raw_sha256: f.sha256.clone(),
            synthetic: false,
            sections: section,
        });
        fuentes.push(FuenteAutorizada {
            documento: f.id.clone(),
            url: f.url.clone(),
            sha256: f.sha256.clone(),
        });
    }
    let catalogo = Catalog {
        version: 1,
        documents: docs,
    };
    catalogo.validate_with_sources(false, &fuentes)?;
    if serde_json::to_vec(&catalogo)
        .map_err(|e| e.to_string())?
        .len()
        > crate::MAX_CATALOG
    {
        return Err("CATALOGO_SUPERA_COTA".into());
    }
    let mapa = json!({"version":1,"libro":plan.libro,"revision_base_documental":plan.revision,"capitulos":plan.capitulos.len(),"capitulos_sin_contenido":drafts,"bytes_originales":total,"archivos":mapping,"reconstruccion_identica":true,"normalizacion":false,"macros":"rechazadas","imagenes_html":"rechazados","red":"sin operaciones de red en el lector"});
    Ok(Preparado {
        catalogo,
        fuentes,
        mapa,
    })
}
pub fn guardar_nuevo(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut f = options.open(path).map_err(|e| e.to_string())?;
    f.write_all(bytes)
        .and_then(|_| f.sync_all())
        .map_err(|e| e.to_string())
}
pub fn preparar_archivo(
    root: &Path,
    plan_path: &Path,
    hash: &str,
    out: &Path,
) -> Result<Value, String> {
    let t = Instant::now();
    let raw = read_bounded(plan_path, 64 * 1024).map_err(|e| e.to_string())?;
    if sha256(&raw) != hash {
        return Err("PLAN_NO_AUTORIZADO".into());
    }
    let plan: Plan = serde_json::from_value(parse_strict(&raw).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let p = preparar(root, &plan)?;
    fs::create_dir(out).map_err(|e| e.to_string())?;
    let mut files = Vec::new();
    for (name, value) in [
        ("CATALOGO.json", serde_json::to_value(&p.catalogo).unwrap()),
        ("FUENTES.json", serde_json::to_value(&p.fuentes).unwrap()),
        ("MAPA.json", p.mapa),
    ] {
        let bytes = serde_json::to_vec_pretty(&value).map_err(|e| e.to_string())?;
        guardar_nuevo(&out.join(name), &bytes)?;
        files.push(json!({"archivo":name,"bytes":bytes.len(),"sha256":sha256(&bytes)}));
    }
    let proof = json!({"preparado":true,"version":env!("CARGO_PKG_VERSION"),"plan_sha256":hash,"duracion_us":t.elapsed().as_micros(),"archivos":files,"inferencia":false,"capturado_unix_s":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e|e.to_string())?.as_secs()});
    guardar_nuevo(
        &out.join("PREPARACION.json"),
        &serde_json::to_vec_pretty(&proof).unwrap(),
    )?;
    Ok(proof)
}
