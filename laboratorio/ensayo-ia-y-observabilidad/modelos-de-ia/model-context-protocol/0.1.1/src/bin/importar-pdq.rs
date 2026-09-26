//! Importación administrativa sin red. Conserva todas las secciones principales del PDQ.
use std::{io::{self, Write}, fs::OpenOptions, path::Path};
use scraper::{Html, Selector};
use sv_mcp_documental::{Catalog, Document, Section, sha256, read_bounded, NCI_URL};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len()!=4 { return Err("Uso: importar-pdq ORIGINAL_HTML FECHA_UTC CATALOGO_NUEVO".into()); }
    let raw = read_bounded(Path::new(&args[1]), 2*1024*1024)?;
    let text = std::str::from_utf8(&raw)?;
    let page = Html::parse_document(text);
    let canonical = Selector::parse("link[rel='canonical']")?;
    if page.select(&canonical).next().and_then(|x|x.value().attr("href"))!=Some(NCI_URL) { return Err("URL canónica inesperada".into()); }
    let title_selector = Selector::parse("article h1")?;
    let title = page.select(&title_selector).next().ok_or("Falta el título")?.text().collect::<String>().trim().to_owned();
    let section_selector = Selector::parse("#cgvBody .accordion > section")?;
    let heading = Selector::parse("h2")?;
    let link_selector = Selector::parse("a[href]")?;
    let mut sections = Vec::new(); let mut updated = None;
    for el in page.select(&section_selector) {
        let id = el.value().attr("id").ok_or("Sección sin identificador")?.to_string();
        let name = el.select(&heading).next().ok_or("Sección sin título")?.text().collect::<String>();
        if name.starts_with("Actualizaciones más recientes") { updated = Some(name.clone()); }
        // Resolver vínculos relativos antes de la conversión. No seguir ni ejecutar enlaces.
        let mut html = el.html();
        for a in el.select(&link_selector) {
            let href = a.value().attr("href").unwrap();
            let absolute = if href.starts_with('#') { Some(format!("{NCI_URL}{href}")) }
                else if href.starts_with('/') && !href.starts_with("//") { Some(format!("https://www.cancer.gov{href}")) } else { None };
            if let Some(abs) = absolute {
                // Las referencias bibliográficas simples del PDQ no requieren sustitución de entidades.
                html = html.replace(&format!("href=\"{href}\""), &format!("href=\"{abs}\""));
            }
        }
        let extracted = html2text::from_read(html.as_bytes(), 100)?;
        sections.push(Section {id,title:name,text:extracted.clone(),sha256:sha256(extracted.as_bytes())});
    }
    if sections.len()!=5 { return Err("Estructura PDQ distinta: revisar extracción antes de continuar".into()); }
    let c = Catalog { version:1, documents:vec![Document {id:"pdq-nci-hcl-es".into(),title,url:NCI_URL.into(),
        retrieved_utc:args[2].clone(),updated_source:updated,raw_sha256:sha256(&raw),synthetic:false,sections}] };
    c.validate(false).map_err(io::Error::other)?;
    let bytes = serde_json::to_vec_pretty(&c)?;
    let mut f = OpenOptions::new().write(true).create_new(true).open(&args[3])?;
    f.write_all(&bytes)?; f.sync_all()?;
    println!("{}",sha256(&bytes));
    Ok(())
}
fn main() { if let Err(e) = run() { eprintln!("Importación detenida: {e}"); std::process::exit(1); } }
