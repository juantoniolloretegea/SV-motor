//! Reutilización de tres originales conservados para un ensayo instrumental cerrado.
use serde_json::{json, Value};
use std::{fs, path::Path};
use sv_biblioteca_documental::{Politica, preparar, raiz_segura, ruta_segura, LICENCIA};
use sv_mcp_documental::{sha256, parse_strict, libro::guardar_nuevo};
type R<T> = Result<T, Box<dyn std::error::Error>>;
fn save(p: &Path, v: &Value) -> R<()> { guardar_nuevo(p, &serde_json::to_vec_pretty(v)?)?; Ok(()) }
fn identity(root: &Path, rel: &str) -> R<Value> { Ok(json!({"ruta":rel,"sha256":sha256(&fs::read(ruta_segura(root,rel)?)?)})) }
fn normalized(t: &str) -> String { t.split_whitespace().collect::<Vec<_>>().join(" ") }
fn run() -> R<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len()!=2 { return Err("Uso: preparar-tres-formatos RAIZ_AUTORIZADA".into()); }
    sv_mcp_documental::aislamiento::no_network()?;
    let root=raiz_segura(Path::new(&args[1]))?;
    let base="ejecucion/astra-biblioteca-tres-formatos-20261009";
    let out=root.join(base).join("libro-r3"); fs::create_dir(&out)?;
    let html_rel="ejecucion/astra-examen25-r2-20261007/fuentes/pdq.html";
    let pdf_rel="desarrollo/mcp-pdf-preparacion-20261003/lector/tests/fixtures/hairy-cell-leukemia.pdf";
    let extracted_rel="desarrollo/mcp-pdf-preparacion-20261003/EXTRACCION-FINAL.json";
    let md_rel="ejecucion/zai-cyb25-preparacion-20261008/fuentes/BANCO-CYB25.md";
    let html=fs::read(ruta_segura(&root,html_rel)?)?;
    let pdf=fs::read(ruta_segura(&root,pdf_rel)?)?;
    if sha256(&html)!="00018ac31108eecc4709f0ce80439d4ca2d56b02262c5c334a184b9c0944e04d"
        || sha256(&pdf)!="21322ed388c999bd0713ba5ec5c7ab34402d4bb4cd6100903aa5ea6d8bf35d9c" {return Err("ORIGINAL_NO_ES_EL_RECIBIDO".into());}
    let hpath=format!("{base}/conversion-r1/HTML01.extraccion.json");
    let h:Value=parse_strict(&fs::read(ruta_segura(&root,&hpath)?)?)?;
    let html_raw=h["markdown"].as_str().ok_or("SIN_MARKDOWN")?;
    if sha256(html_raw.as_bytes())!=h["markdown_sha256"] {return Err("CONVERSION_ALTERADA".into());}
    // Conserva el texto alternativo y la referencia. No interpreta ni descarga figuras.
    let html_md=html_raw.replace("![", "[Recurso visual no interpretado: ");
    let check_html=["Tratamiento de la leucemia de células pilosas", "Información general", "CD20", "cladribina"];
    let htext=std::str::from_utf8(&html)?;
    for s in check_html {if !htext.contains(s)||!html_md.contains(s){return Err(format!("FALTA_TESTIGO_HTML: {s}").into());}}
    let e:Value=parse_strict(&fs::read(ruta_segura(&root,extracted_rel)?)?)?;
    let pages=e["paginas"].as_array().ok_or("SIN_PAGINAS")?;
    if e["original_sha256"]!=sha256(&pdf)||pages.len()!=10 {return Err("EXTRACCION_AJENA".into());}
    let mut pdf_md=String::new(); let mut mapping=Vec::new();
    for (i,p) in pages.iter().enumerate() {
        let t=p["texto"].as_str().ok_or("SIN_TEXTO_PDF")?;
        if p["indice"]!=i||p["pagina_impresa_ordinal"]!=i+1||p["sha256"]!=sha256(t.as_bytes())||p["caracteres"]!=t.chars().count(){return Err("PAGINA_NO_COTEJADA".into());}
        let heading=format!("# Página física {}\n\n",i+1); pdf_md.push_str(&heading);
        let start=pdf_md.len(); pdf_md.push_str(t); let end=pdf_md.len(); pdf_md.push_str("\n\n");
        mapping.push(json!({"pagina_fisica":i+1,"inicio_byte_markdown":start,"fin_byte_markdown":end,"texto_sha256":p["sha256"],"texto_literal":true}));
    }
    let phrase="La enfermedad también se denomina leucemia de células pilosas y tricoleucemia.";
    let first=pages[0]["texto"].as_str().unwrap();
    let failed:Value=parse_strict(&fs::read(root.join(base).join("conversion-r1/PDF01.extraccion.json"))?)?;
    if !normalized(first).contains(phrase)||normalized(failed["markdown"].as_str().ok_or("SIN_CONTRASTE")?).contains(phrase){return Err("DEFECTO_PDF_NO_REPRODUCIDO".into());}
    let md=fs::read(ruta_segura(&root,md_rel)?)?; let md_text=std::str::from_utf8(&md)?;
    if !md_text.contains("CYB25")||!md_text.contains("C25"){return Err("BANCO_INCORRECTO".into());}
    for (file,bytes) in [("HTML01.md",html_md.as_bytes()),("PDF01.md",pdf_md.as_bytes()),("MD01.md",&md)] {guardar_nuevo(&out.join(file),bytes)?;}
    let summary="# Índice\n\n- [Caché NCI PDQ](HTML01.md)\n- [PDF LLS de diez páginas](PDF01.md)\n- [Banco documental de ciberseguridad CYB25](MD01.md)\n";
    guardar_nuevo(&out.join("SUMMARY.md"),summary.as_bytes())?;
    let evidence=json!({"version":1,"original_html":identity(&root,html_rel)?,"original_pdf":identity(&root,pdf_rel)?,
        "conversion_html":identity(&root,&hpath)?,"extraccion_pdf_recibida":identity(&root,extracted_rel)?,"original_markdown":identity(&root,md_rel)?,
        "html_testigos_preservados":check_html,"html_referencias_visuales_convertidas_a_enlace":html_raw.matches("![").count(),"html_figuras_no_interpretadas":true,"pdf_paginas":mapping,"pdf_defecto_xberg_reproducido":true,
        "pdf_metodo_admitido":e["metodo"],"conversion_pdf_xberg_admitida":false,
        "alcance":"Cotejo instrumental de identidades, testigos y totalidad de páginas de una extracción ya recibida. No certifica lectura universal de PDF ni validez clínica.",
        "markdown_alcance":"Banco de 25 preguntas, sin clave reservada ni respuestas inventadas. Documento de consulta; no se realiza ese examen.",
        "advertencias_pdf":e["advertencias"],"inferencia":false,"produccion":false,"licencia_documento_propio":LICENCIA});
    save(&out.join("COTEJO-FUENTES.json"),&evidence)?;
    let mut meta=Vec::new(); let mut chapters=Vec::new();
    for (id,file,title,kind,original,method,license,scope) in [
        ("HTML01","HTML01.md","Caché NCI PDQ profesional","html",html_rel,"Xberg 1.3.6 sin acceso a red; testigos cotejados","NCI; atribución y condiciones del original, sin extensión de la licencia del SV","Caché histórica profesional conservada; no es una descarga actual ni la versión para pacientes"),
        ("PDF01","PDF01.md","PDF LLS FS16S8/18","pdf",pdf_rel,"Extracción Rust histórica recibida, orden de operadores; texto literal de diez páginas","LLS © 2018; derechos y condiciones del original","Edición histórica; cotejo textual, no dictamen clínico actual"),
        ("MD01","MD01.md","Banco de preguntas CYB25","markdown",md_rel,"Identidad literal",LICENCIA,"Casos sintéticos; preguntas sin clave reservada; prueba de consulta, no examen CYB25")
    ] {
        let bytes=fs::read(out.join(file))?; let hash=sha256(&bytes);
        chapters.push(json!({"id":id,"ruta":file,"titulo":title,"sha256":hash,"url":format!("urn:sv:documento:sha256:{hash}")}));
        let mut m=json!({"documento":id,"tipo_original":kind,"original":identity(&root,original)?,"procedencia":method,"licencia":license,
            "sintetico":id=="MD01","visibilidad":"publica","alcance":scope,"correspondencia":if kind=="markdown"{"identidad_literal"}else{"conversion_recibida_para_ensayo"},"identificador_editorial_r0":null});
        if id=="HTML01" {m["procedencia"]=json!("https://www.cancer.gov/espanol/tipos/leucemia/pro/tratamiento-celulas-pilosas-pdq; caché histórica fijada por SHA-256");}
        if id=="PDF01" {m["procedencia"]=json!("LLS FS16S8/18; SVperitus-dataset@488d597fa1999a1fc2eb6538fd610a36046f6f01/dominios/inmunologia/literatura-tricoleucemia/lls-hoja-informativa-2018/original/hairy-cell-leukemia.pdf");}
        if kind!="markdown" {
            let receipt=json!({"version":1,"documento":id,"original_sha256":m["original"]["sha256"],"markdown_sha256":hash,
                "evidencia":identity(&root,&format!("{base}/libro-r3/COTEJO-FUENTES.json"))?,"metodo":method,"alcance":scope,"admitido_ensayo":true,"produccion":false});
            save(&out.join(format!("RECEPCION-{id}.json")),&receipt)?;
            m["recepcion_conversion"]=identity(&root,&format!("{base}/libro-r3/RECEPCION-{id}.json"))?;
        }
        meta.push(m);
    }
    let sh=sha256(summary.as_bytes());
    let plan=json!({"version":2,"libro":"tres-formatos","revision":sha256(&serde_json::to_vec(&evidence)?),"summary":{"id":"SUMARIO","ruta":"SUMMARY.md","titulo":"Índice de tres documentos","sha256":sh,"url":format!("urn:sv:documento:sha256:{sh}")},"capitulos":chapters});
    save(&out.join("PLAN.json"),&plan)?;
    meta.push(json!({"documento":"SUMARIO","tipo_original":"markdown","original":identity(&root,&format!("{base}/libro-r3/SUMMARY.md"))?,
        "procedencia":"Índice del ensayo","licencia":LICENCIA,"sintetico":false,"visibilidad":"publica","alcance":"Navegación documental", "correspondencia":"identidad_literal","identificador_editorial_r0":null}));
    let library=json!({"version":1,"edicion":"tres-formatos-20261009-r3","estado":"preparacion_local","libros":[{"id":"tres-formatos","titulo":"Caché, PDF y Markdown conservados","raiz":format!("{base}/libro-r3"),"plan":identity(&root,&format!("{base}/libro-r3/PLAN.json"))?,"metadatos":meta}]});
    save(&out.join("BIBLIOTECA.json"),&library)?;
    let raw=fs::read(out.join("BIBLIOTECA.json"))?;
    let policy=Politica{version:1,biblioteca_sha256:sha256(&raw),nodo:3,finalidad:"verificacion_local".into(),libros_permitidos:vec!["tres-formatos".into()],permitir_sinteticos:true,permitir_restringidos:false};
    save(&out.join("POLITICA.json"),&serde_json::to_value(&policy)?)?;
    let result=preparar(&root,&raw,&policy)?;
    // Ensayo adversarial: una conversión no recibida y una huella alterada se rechazan antes de servirla.
    let mut bad=library.clone();bad["libros"][0]["metadatos"][0].as_object_mut().unwrap().remove("recepcion_conversion");
    let badraw=serde_json::to_vec(&bad)?;let mut bp=policy.clone();bp.biblioteca_sha256=sha256(&badraw);
    if preparar(&root,&badraw,&bp).is_ok(){return Err("CONVERSION_NO_RECIBIDA_ACEPTADA".into());}
    bad=library.clone();bad["libros"][0]["metadatos"][1]["recepcion_conversion"]["sha256"]=json!("0".repeat(64));
    let badraw=serde_json::to_vec(&bad)?;bp.biblioteca_sha256=sha256(&badraw);
    if preparar(&root,&badraw,&bp).is_ok(){return Err("RECEPCION_ALTERADA_ACEPTADA".into());}
    save(&out.join("CONTROL-PREPARACION.json"),&json!({"conforme":true,"documentos":result.catalogo.documents.len(),"secciones":result.catalogo.documents.iter().map(|d|d.sections.len()).sum::<usize>(),"rechazos_adversariales":2,"inferencia":false,"muestra":sv_mcp_documental::instrumentacion::muestra(),"licencia":LICENCIA}))?;
    println!("{}",json!({"conforme":true,"politica_sha256":sha256(&fs::read(out.join("POLITICA.json"))?),"documentos":result.catalogo.documents.len(),"inferencia":false}));
    Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("Preparación detenida: {e}");std::process::exit(1)}}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
