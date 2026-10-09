//! Correspondencia literal de una edición Markdown pública. No altera el controlador.
#![forbid(unsafe_code)]
use serde_json::{json, Value};
use std::{fs, path::Path};
use sv_biblioteca_documental::{Biblioteca, Politica, Tipo, Visibilidad, leer_json, leer_identidad, preparar, raiz_segura, ruta_segura};
use sv_mcp_documental::{sha256, Section};

fn intervalos(original: &str, secciones: &[Section]) -> Result<Vec<Value>, String> {
    let mut offset = 0;
    let mut salida = Vec::new();
    for s in secciones {
        let fin = offset + s.text.len();
        if original.get(offset..fin) != Some(s.text.as_str()) || sha256(s.text.as_bytes()) != s.sha256 {
            return Err("CORRESPONDENCIA_LITERAL_NO_CONFORME".into());
        }
        let primera = original[..offset].bytes().filter(|b| *b == b'\n').count() + 1;
        let ultima = primera + s.text.bytes().filter(|b| *b == b'\n').count() - usize::from(s.text.ends_with('\n'));
        if s.id != format!("MD-L{primera:06}-L{ultima:06}") { return Err("LOCALIZADOR_NO_CONFORME".into()); }
        salida.push(json!({"seccion":s.id,"titulo":s.title,"primera_linea":primera,"ultima_linea":ultima,
            "inicio_byte":offset,"fin_byte_exclusivo":fin,"sha256":s.sha256}));
        offset = fin;
    }
    if offset != original.len() { return Err("ORIGINAL_INCOMPLETO".into()); }
    Ok(salida)
}

fn run() -> Result<(), String> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 6 { return Err("Uso: cotejar-edicion RAIZ POLITICA SHA256_POLITICA BIBLIOTECA SALIDA_NUEVA".into()); }
    sv_mcp_documental::aislamiento::no_network().map_err(|e| e.to_string())?;
    let root = raiz_segura(Path::new(&a[1]))?;
    let raw = sv_mcp_documental::read_bounded(&ruta_segura(&root, &a[2])?, 65536).map_err(|e| e.to_string())?;
    if sha256(&raw) != a[3] { return Err("POLITICA_NO_AUTORIZADA".into()); }
    let policy: Politica = leer_json(&raw)?;
    let manifest = sv_mcp_documental::read_bounded(&ruta_segura(&root, &a[4])?, 256 * 1024).map_err(|e| e.to_string())?;
    let library: Biblioteca = leer_json(&manifest)?;
    if policy.permitir_restringidos || policy.permitir_sinteticos || library.libros.len() != policy.libros_permitidos.len() {
        return Err("PERFIL_NO_PUBLICABLE".into());
    }
    let ready = preparar(&root, &manifest, &policy)?;
    let mut documents = Vec::new();
    for book in &library.libros {
        for meta in &book.metadatos {
            if meta.tipo_original != Tipo::Markdown || meta.visibilidad != Visibilidad::Publica || meta.sintetico {
                return Err("FUENTE_NO_PUBLICABLE_EN_ESTE_PERFIL".into());
            }
            let original_raw = leer_identidad(&root, &meta.original, 1024 * 1024)?;
            let original = std::str::from_utf8(&original_raw).map_err(|e| e.to_string())?;
            let document = ready.catalogo.documents.iter().find(|d| d.id == meta.documento).ok_or("DOCUMENTO_AUSENTE")?;
            let sections = intervalos(original, &document.sections)?;
            documents.push(json!({"libro":book.id,"documento":document.id,"original":meta.original,
                "procedencia":meta.procedencia,"licencia":meta.licencia,"alcance":meta.alcance,
                "identificador_editorial_r0":meta.identificador_editorial_r0,"correspondencia":"identidad_literal",
                "cobertura_bytes":original_raw.len(),"secciones":sections}));
        }
    }
    let report = json!({"version":1,"conforme":true,"edicion":library.edicion,"biblioteca_sha256":sha256(&manifest),
        "politica_sha256":a[3],"documentos":documents,"consulta_proveedor":false,"conversion_general_recibida":false,
        "navegacion_autonoma_modelo":false,"alcance":"Correspondencia completa y literal de los originales Markdown seleccionados; no recepción de traducción ni producción"});
    let output = Path::new(&a[5]);
    raiz_segura(output.parent().ok_or("SIN_PADRE")?)?;
    sv_mcp_documental::libro::guardar_nuevo(output, &serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?)?;
    println!("{}", json!({"conforme":true,"documentos":documents.len(),"sha256":sha256(&fs::read(output).map_err(|e|e.to_string())?)}));
    Ok(())
}
fn main() { if let Err(e) = run() { eprintln!("Cotejo detenido: {e}"); std::process::exit(1); } }

#[cfg(test)]
mod tests {
    use super::*;
    fn section(text: &str, id: &str) -> Section { Section { id:id.into(),title:"Título".into(),text:text.into(),sha256:sha256(text.as_bytes()) } }
    #[test] fn utf8_y_linea_final_sin_salto() {
        let text = "# Índice\n\nÁrbitro y sección.";
        let r = intervalos(text, &[section("# Índice\n\n", "MD-L000001-L000002"),section("Árbitro y sección.","MD-L000003-L000003")]).unwrap();
        assert_eq!(r[1]["inicio_byte"],11); assert_eq!(r[1]["fin_byte_exclusivo"],text.len());
    }
    #[test] fn omision_del_anexo_no_es_cobertura_completa() {
        assert_eq!(intervalos("Texto\nAnexo", &[section("Texto\n","MD-L000001-L000001")]).unwrap_err(), "ORIGINAL_INCOMPLETO");
    }
    #[test] fn cita_de_linea_distinta_rechazada() {
        assert_eq!(intervalos("A\nB", &[section("A\nB","MD-L000002-L000003")]).unwrap_err(), "LOCALIZADOR_NO_CONFORME");
    }
    #[test] fn texto_o_huella_modificados_rechazados() {
        assert!(intervalos("Original", &[section("Alterado","MD-L000001-L000001")]).is_err());
        let mut s = section("Original","MD-L000001-L000001"); s.sha256 = "0".repeat(64);
        assert!(intervalos("Original", &[s]).is_err());
    }
}
