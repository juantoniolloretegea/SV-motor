use serde_json::{json, Value};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};
use sv_mcp_documental::{
    auditoria::{unhex, verify},
    libro::{preparar, preparar_archivo, Archivo, Plan},
    sha256, Catalog, Session,
};
fn root() -> PathBuf {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/evidencias-libros")
        .join(format!(
            "{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&p).unwrap();
    p
}
fn file(p: &Path, id: &str, path: &str, text: &str) -> Archivo {
    fs::write(p.join(path), text).unwrap();
    Archivo {
        id: id.into(),
        ruta: path.into(),
        titulo: format!("SVP {id}"),
        url: format!("https://example.invalid/edicion/{path}"),
        sha256: sha256(text.as_bytes()),
    }
}
fn fixture(text: &str) -> (PathBuf, Plan) {
    let p = root();
    let summary = file(
        &p,
        "INDICE",
        "SUMMARY.md",
        "# Índice\n\n- [Contenido](capitulo.md)\n- [Futuro]()\n",
    );
    let c = file(&p, "CAPITULO", "capitulo.md", text);
    (
        p,
        Plan {
            version: 1,
            libro: "LIBRO_PRUEBA".into(),
            revision: "a".repeat(40),
            summary,
            capitulos: vec![c],
        },
    )
}
fn ready(c: &Catalog) -> Session {
    let mut s = Session::default();
    s.handle(c,json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":sv_mcp_documental::PROTOCOL,"capabilities":{},"clientInfo":{"name":"instrumento","version":"1"}}}));
    s.handle(
        c,
        json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
    );
    s
}
#[test]
fn preserva_bytes_unicode_crlf_y_encabezados_fuera_de_bloques() {
    let text="Preámbulo\r\n\r\n# Título á 漢字\r\nAfirmación.\r\n\r\n```rust\r\n# no es sección\r\n```\r\n\r\n## Otra\r\nFin sin salto";
    let (p, plan) = fixture(text);
    let b = preparar(&p, &plan).unwrap();
    let d = &b.catalogo.documents[1];
    assert_eq!(d.sections.len(), 3);
    assert_eq!(
        d.sections
            .iter()
            .map(|s| s.text.as_str())
            .collect::<String>(),
        text
    );
    assert_eq!(b.mapa["capitulos_sin_contenido"], 1);
    assert!(d.sections[1].id.starts_with("MD-L000003"));
}
#[test]
fn rechaza_huella_cambiada() {
    let (p, plan) = fixture("# Texto\nOriginal\n");
    fs::write(p.join("capitulo.md"), "# Modificado\n").unwrap();
    assert!(preparar(&p, &plan).unwrap_err_contains("HUELLA"));
}
trait ErrorText {
    fn unwrap_err_contains(self, s: &str) -> bool;
}
impl<T> ErrorText for Result<T, String> {
    fn unwrap_err_contains(self, s: &str) -> bool {
        match self {
            Err(e) => e.contains(s),
            Ok(_) => false,
        }
    }
}
#[test]
fn rechaza_rutas_escapadas_url_absoluta_y_codificada() {
    for path in [
        "../secreto.md",
        "/etc/secreto.md",
        "C:/secreto.md",
        "a\\b.md",
        "https://x/a.md",
        "%2e%2e/secreto.md",
        "./capitulo.md",
    ] {
        let (p, mut plan) = fixture("# Texto\nHola\n");
        plan.capitulos[0].ruta = path.into();
        plan.summary = file(
            &p,
            "INDICE",
            "SUMMARY.md",
            &format!("# Índice\n- [Capítulo]({path})\n"),
        );
        assert!(preparar(&p, &plan).is_err(), "{path}");
    }
}
#[test]
fn rechaza_macros_html_imagenes_y_nulos() {
    for text in [
        "# A\n{{#include secreto.md}}",
        "# A\n{{ #include secreto.md}}",
        "# A\n<script>algo()</script>",
        "# A\n![imagen](otra.png)",
        "# A\n\0",
    ] {
        let (p, plan) = fixture(text);
        assert!(preparar(&p, &plan).is_err(), "{text}");
    }
}
#[test]
fn rechaza_indice_distinto_archivo_faltante_y_duplicado() {
    let (p, mut plan) = fixture("# A\nTexto\n");
    plan.summary = file(&p, "INDICE", "SUMMARY.md", "# Índice\n- [Ajeno](otro.md)\n");
    assert!(preparar(&p, &plan).is_err());
    let (p, plan) = fixture("# A\nTexto\n");
    fs::remove_file(p.join("capitulo.md")).unwrap();
    assert!(preparar(&p, &plan).is_err());
    let (p, mut plan) = fixture("# A\nTexto\n");
    plan.capitulos.push(plan.capitulos[0].clone());
    assert!(preparar(&p, &plan).is_err());
}
#[cfg(unix)]
#[test]
fn rechaza_enlaces_incluso_dentro_del_corpus() {
    use std::os::unix::fs::symlink;
    let (p, plan) = fixture("# A\nTexto\n");
    fs::rename(p.join("capitulo.md"), p.join("original.md")).unwrap();
    symlink("original.md", p.join("capitulo.md")).unwrap();
    assert!(preparar(&p, &plan).unwrap_err_contains("ENLACE"));
}
#[test]
fn rechaza_exceso_de_bytes_y_secciones() {
    let (p, plan) = fixture(&"a".repeat(sv_mcp_documental::libro::MAX_CHAPTER + 1));
    assert!(preparar(&p, &plan).is_err());
    let text = (0..65)
        .map(|i| format!("# Sección {i}\nTexto\n"))
        .collect::<String>();
    let (p, plan) = fixture(&text);
    assert!(preparar(&p, &plan).unwrap_err_contains("SECCIONES"));
}
#[test]
fn no_reescribe_y_exige_plan_exacto() {
    let (p, plan) = fixture("# A\nTexto\n");
    let path = p.join("PLAN.json");
    let bytes = serde_json::to_vec(&plan).unwrap();
    fs::write(&path, &bytes).unwrap();
    let out = p.join("salida");
    assert!(preparar_archivo(&p, &path, &"0".repeat(64), &out).is_err());
    assert!(!out.exists());
    preparar_archivo(&p, &path, &sha256(&bytes), &out).unwrap();
    let before = fs::read(out.join("PREPARACION.json")).unwrap();
    assert!(preparar_archivo(&p, &path, &sha256(&bytes), &out).is_err());
    assert_eq!(before, fs::read(out.join("PREPARACION.json")).unwrap());
}
#[test]
fn texto_extenso_se_reconstruye_por_fragmentos_y_citas() {
    let text = format!("# Larga\n{}", "Premisas áé 漢字. ".repeat(800));
    let (p, plan) = fixture(&text);
    let b = preparar(&p, &plan).unwrap();
    let c = b.catalogo;
    let mut s = ready(&c);
    let mut page = 0;
    let mut reconstructed = String::new();
    loop {
        let r=s.handle(&c,json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"leer_documento","arguments":{"documento":"CAPITULO","seccion":c.documents[1].sections[0].id,"pagina":page}}})).unwrap();
        assert_eq!(r["result"]["isError"], false);
        let d = &r["result"]["structuredContent"];
        assert_eq!(d["linea_inicial"], 1);
        assert_eq!(d["sha256_original"], plan.capitulos[0].sha256);
        assert_eq!(d["inicio_caracter"], reconstructed.chars().count());
        reconstructed.push_str(d["texto"].as_str().unwrap());
        if d["siguiente_pagina"].is_null() {
            break;
        }
        page = d["siguiente_pagina"].as_u64().unwrap();
    }
    assert!(page > 1);
    assert_eq!(reconstructed, text);
}
#[test]
fn citas_no_falsifican_lineas_pdf() {
    let (p, plan) = fixture("# A\nTexto\n");
    let mut b = preparar(&p, &plan).unwrap();
    b.catalogo.documents[1].url = "https://example.invalid/test.pdf".into();
    b.catalogo.documents[1].sections[0].id = "PDF-P0003".into();
    let mut s = ready(&b.catalogo);
    let r=s.handle(&b.catalogo,json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"leer_documento","arguments":{"documento":"CAPITULO","seccion":"PDF-P0003"}}})).unwrap();
    assert_eq!(r["result"]["structuredContent"]["pagina_pdf_ordinal"], 4);
    assert!(r["result"]["structuredContent"]
        .get("linea_inicial")
        .is_none());
}

#[test]
fn manual_real_recorrido_mcp_y_diario_cotejados() {
    let project = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = project.join("manual/PLAN.json");
    let raw = fs::read(&path).unwrap();
    let p = root();
    let prepared = p.join("preparado");
    preparar_archivo(&project.join("manual/src"), &path, &sha256(&raw), &prepared).unwrap();
    let catraw = fs::read(prepared.join("CATALOGO.json")).unwrap();
    let hash = sha256(&catraw);
    let source_hash = sha256(&fs::read(prepared.join("FUENTES.json")).unwrap());
    let sources =
        sv_mcp_documental::fuentes_autorizadas(&prepared.join("FUENTES.json"), &source_hash)
            .unwrap();
    let c = Catalog::load_with_sources(&prepared.join("CATALOGO.json"), &hash, false, &sources)
        .unwrap();
    let mut requests = vec![
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":sv_mcp_documental::PROTOCOL,"capabilities":{},"clientInfo":{"name":"director-instrumental-libros","version":"1"}}}),
        json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
        json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}),
    ];
    let mut expected = Vec::new();
    let mut id = 3;
    let mut simulated = ready(&c);
    for d in &c.documents {
        for section in &d.sections {
            let mut page = 0;
            loop {
                let req = json!({"jsonrpc":"2.0","id":id,"method":"tools/call","params":{"name":"leer_documento","arguments":{"documento":d.id,"seccion":section.id,"pagina":page}}});
                let answer = simulated.handle(&c, req.clone()).unwrap();
                assert_eq!(answer["result"]["isError"], false);
                let next = answer["result"]["structuredContent"]["siguiente_pagina"].as_u64();
                requests.push(req);
                expected.push((id, d.id.clone(), section.id.clone()));
                id += 1;
                if let Some(n) = next {
                    page = n;
                } else {
                    break;
                }
            }
        }
    }
    // Dos consultas adversas prueban límites de la interfaz sin introducir contenido secreto.
    for args in [
        json!({"documento":"../../secreto","seccion":"S1"}),
        json!({"documento":"MANUAL_CONSTRUCTOR","seccion":"S1","url":"https://example.invalid"}),
    ] {
        requests.push(json!({"jsonrpc":"2.0","id":id,"method":"tools/call","params":{"name":"leer_documento","arguments":args}}));
        id += 1;
    }
    let mut input = Vec::new();
    for request in &requests {
        input.extend(serde_json::to_vec(request).unwrap());
        input.push(b'\n');
    }
    fs::write(p.join("solicitudes.jsonl"), &input).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_sv-mcp-documental"))
        .arg(prepared.join("CATALOGO.json"))
        .arg(&hash)
        .arg(p.join("diario.jsonl"))
        .args(["256", "--fuentes-autorizadas"])
        .arg(prepared.join("FUENTES.json"))
        .arg(&source_hash)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut pipe = child.stdin.take().unwrap();
    let sent = input.clone();
    let writer = std::thread::spawn(move || pipe.write_all(&sent).unwrap());
    let output = child.wait_with_output().unwrap();
    writer.join().unwrap();
    fs::write(p.join("respuestas.jsonl"), &output.stdout).unwrap();
    fs::write(p.join("stderr.txt"), &output.stderr).unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let replies: Vec<Value> = output
        .stdout
        .split(|b| *b == b'\n')
        .filter(|b| !b.is_empty())
        .map(|b| serde_json::from_slice(b).unwrap())
        .collect();
    let mut recovered = std::collections::BTreeMap::<(String, String), String>::new();
    for (id, doc, section) in &expected {
        let r = replies.iter().find(|v| v["id"] == *id).unwrap();
        assert_eq!(r["result"]["isError"], false);
        assert!(r.to_string().chars().count() <= sv_mcp_documental::MAX_RESPONSE);
        recovered
            .entry((doc.clone(), section.clone()))
            .or_default()
            .push_str(r["result"]["structuredContent"]["texto"].as_str().unwrap());
    }
    for d in &c.documents {
        for section in &d.sections {
            assert_eq!(recovered[&(d.id.clone(), section.id.clone())], section.text);
        }
    }
    assert!(replies
        .iter()
        .rev()
        .take(2)
        .all(|r| r["result"]["isError"] == true));
    let journal = fs::read(p.join("diario.jsonl")).unwrap();
    let replay = verify(&c, &hash, &journal).unwrap();
    let (mut received, mut emitted) = (Vec::new(), Vec::new());
    let mut observations = 0;
    for line in journal.split(|b| *b == b'\n').filter(|b| !b.is_empty()) {
        let v: Value = serde_json::from_slice(line).unwrap();
        let d = &v["datos"];
        if d["evento"] == "solicitud" {
            received.extend(unhex(d["bytes_hex"].as_str().unwrap()).unwrap());
        }
        if d["evento"] == "resultado" {
            emitted.extend(unhex(d["bytes_hex"].as_str().unwrap()).unwrap());
            assert!(d["proceso"]["rss_kib"].as_u64().is_some());
            assert_eq!(d["proceso"]["sockets_propios"], 0);
            observations += 1;
        }
    }
    assert_eq!(received, input);
    assert_eq!(emitted, output.stdout);
    let proof = json!({"conforme":true,"inferencia":false,"documentos":c.documents.len(),"secciones":c.documents.iter().map(|d|d.sections.len()).sum::<usize>(),"fragmentos":expected.len(),"solicitudes":requests.len(),"bytes_solicitados":input.len(),"bytes_recibidos":output.stdout.len(),"muestras_proceso":observations,"reconstruccion_texto_exacta":true,"diario":replay,"fuentes_sha256":source_hash,"plan_sha256":sha256(&raw),"consultas_ajenas_rechazadas":2});
    fs::write(
        p.join("COMPROBACION.json"),
        serde_json::to_vec_pretty(&proof).unwrap(),
    )
    .unwrap();
    println!("EVIDENCIA_MANUAL={}", p.display());
}
