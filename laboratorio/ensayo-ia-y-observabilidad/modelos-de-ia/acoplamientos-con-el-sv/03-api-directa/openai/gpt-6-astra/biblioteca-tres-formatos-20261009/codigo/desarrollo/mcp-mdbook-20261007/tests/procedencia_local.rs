use std::{fs,path::PathBuf};
use sv_mcp_documental::{libro::{Archivo,Plan,preparar},sha256};
#[test]
fn origen_local_exige_huella_sin_simular_publicacion() {
    let root=PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target").join(format!("local-{}",std::process::id()));
    fs::create_dir_all(&root).unwrap();
    let mk=|id:&str,path:&str,text:&str|{fs::write(root.join(path),text).unwrap();let h=sha256(text.as_bytes());Archivo{id:id.into(),ruta:path.into(),titulo:id.into(),url:format!("urn:sv:documento:sha256:{h}"),sha256:h}};
    let index=mk("INDICE","SUMMARY.md","# Índice\n\n- [Capítulo](capitulo.md)\n");
    let chapter=mk("CAPITULO","capitulo.md","# Texto\n\nAño y negación conservados.\n");
    let mut p=Plan{version:2,libro:"LIBRO_LOCAL".into(),revision:"a".repeat(64),summary:index,capitulos:vec![chapter]};
    let ready=preparar(&root,&p).unwrap();assert_eq!(ready.catalogo.documents.len(),2);
    assert!(!ready.catalogo.documents[1].synthetic);
    p.capitulos[0].url=format!("urn:sv:documento:sha256:{}","b".repeat(64));assert!(preparar(&root,&p).is_err());
    p.capitulos[0].url=format!("urn:sv:documento:sha256:{}",p.capitulos[0].sha256);p.version=1;p.revision="a".repeat(40);assert!(preparar(&root,&p).is_err());
}
