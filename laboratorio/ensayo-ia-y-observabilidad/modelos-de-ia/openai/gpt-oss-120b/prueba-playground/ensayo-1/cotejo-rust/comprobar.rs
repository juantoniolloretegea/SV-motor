// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{env, fs, path::{Component, Path, PathBuf}};

fn hash(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
fn child(root: &Path, name: &str) -> PathBuf {
    assert!(!name.is_empty());
    let mut p = root.to_path_buf();
    for part in Path::new(name).components() {
        match part { Component::Normal(n) => p.push(n), _ => panic!("Ruta no relativa") }
        let m = fs::symlink_metadata(&p).expect("Archivo ausente");
        assert!(!m.file_type().is_symlink(), "Enlace no admitido");
        #[cfg(windows)] { use std::os::windows::fs::MetadataExt; assert_eq!(m.file_attributes() & 0x400, 0); }
        assert!(fs::canonicalize(&p).unwrap().starts_with(root));
    }
    p
}
fn read_json(root: &Path, name: &str) -> Value {
    serde_json::from_slice(&fs::read(child(root, name)).unwrap()).unwrap()
}
fn main() {
    let root = fs::canonicalize(env::args().nth(1).expect("Indique la carpeta del ensayo")).unwrap();
    let manifest = read_json(&root, "MANIFIESTO.json");
    let files = manifest["archivos"].as_array().unwrap();
    for entry in files {
        let bytes = fs::read(child(&root, entry["ruta"].as_str().unwrap())).unwrap();
        assert_eq!(bytes.len() as u64, entry["bytes"].as_u64().unwrap());
        assert_eq!(hash(&bytes), entry["sha256"].as_str().unwrap());
    }
    let expected = read_json(&root, "FUENTE-SEGMENTADA.json");
    let output = read_json(&root, "SALIDA-01.json");
    let exp = expected.as_array().unwrap();
    let got = output["fragmentos"].as_array().unwrap();
    assert_eq!(output["ensayo"], "GPTOSS-PLAYGROUND-E1-20261003");
    assert_eq!(exp.len(), got.len());
    assert_eq!(output["calculo_hash_realizado"], false);
    assert!(output.get("sha256_modelo").unwrap().is_null());
    let mut rows = vec![];
    for (x, y) in exp.iter().zip(got) {
        assert_eq!(x["id"], y["id"]);
        assert!(!y["sintesis"].as_str().unwrap().trim().is_empty());
        let a = x["texto"].as_str().unwrap();
        let b = y["texto"].as_str().unwrap();
        rows.push(json!({"id": x["id"], "igualdad_estricta": a == b,
            "igualdad_interpretando_saltos": a == b.replace("\\n", "\n")}));
    }
    println!("{}", serde_json::to_string_pretty(&json!({
        "archivos_verificados": files.len(),
        "segmentos": rows,
        "segmentos_exactos": rows.iter().filter(|r| r["igualdad_estricta"] == true).count(),
        "segmentos_coincidentes_tras_interpretar_saltos": rows.iter().filter(|r| r["igualdad_interpretando_saltos"] == true).count(),
        "alcance": "Identidad documental; no demuestra lectura interna ni corrección científica"
    })).unwrap());
}
