#![forbid(unsafe_code)]
use base64::{engine::general_purpose::STANDARD, Engine};
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};
fn main() {
    let dir = PathBuf::from(std::env::args().nth(1).expect("Directorio de wasm-bindgen"));
    let js = fs::read_to_string(dir.join("sv_visor_pdf.js")).unwrap();
    let wasm = fs::read(dir.join("sv_visor_pdf_bg.wasm")).unwrap();
    assert!(!js.to_lowercase().contains("</script"));
    let datos = sv_visor_pdf::Visor::default();
    assert_eq!(datos.datos["casos"].as_array().unwrap().len(), 9);
    let html = format!(
        r#"<!doctype html>
<html lang="es"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'unsafe-inline' 'wasm-unsafe-eval'; style-src 'unsafe-inline'; img-src data: blob:; connect-src 'none'; worker-src 'none'; font-src data:; object-src 'none'; base-uri 'none'; form-action 'none'">
<title>SV · Polígono interactivo egui · Astra · MD · Verificación final R2 · 0.6.0</title>
<style>html,body{{margin:0;background:#fafafa;font:14px system-ui;color:#1b3e63;height:100%;overflow:hidden}}canvas{{display:block;width:100%;height:calc(100% - 75px);touch-action:none}}#seleccion{{display:block;height:65px;padding:5px 14px;box-sizing:border-box;overflow:auto;border-top:1px solid #ddd;font-size:13px}}#estado{{position:absolute;top:30%;left:10%;right:10%;background:white;padding:25px;border:1px solid #999}}noscript{{position:absolute;inset:20px;background:white;padding:20px}}</style></head>
<body><canvas id="visor" aria-label="Polígono SV interactivo; seleccione MD01 a MD09 para consultar evidencias"></canvas>
<output id="seleccion" aria-live="polite">Preparando la representación local…</output>
<p id="estado">Cargando Rust / egui incluido en este archivo. No se consulta ningún servidor.</p>
<noscript>Este visor requiere JavaScript para iniciar el módulo Rust WebAssembly y WebGL para dibujar. No descarga módulos externos.</noscript>
<script type="module">
{js}
try {{
  const bytes = Uint8Array.from(atob('{b64}'), c => c.charCodeAt(0));
  await __wbg_init({{module_or_path: bytes}});
  await iniciar(document.getElementById('visor'));
  document.getElementById('estado').remove();
}} catch (error) {{
  document.getElementById('estado').textContent = 'No se pudo iniciar el visor Rust/egui. Este navegador necesita WebAssembly y WebGL. Detalle: ' + String(error);
}}
</script></body></html>"#,
        b64 = STANDARD.encode(&wasm)
    );
    let path = dir.join("POLIGONO-EGUI.html");
    fs::write(&path, &html).unwrap();
    let prueba = serde_json::json!({"version":"0.6.0","fuente_sha256":sv_visor_pdf::HUELLA,"dictamen_sha256":sv_visor_pdf::contrato::huella(sv_visor_pdf::DICTAMEN),"contrato":sv_visor_pdf::contrato::CONTRATO,"html_sha256":format!("{:x}",Sha256::digest(html.as_bytes())),"html_bytes":html.len(),"wasm_sha256":format!("{:x}",Sha256::digest(&wasm)),"wasm_bytes":wasm.len(),"javascript_sha256":format!("{:x}",Sha256::digest(js.as_bytes())),"red":"connect-src none; recursos incluidos en HTML","adjudicacion":"CAPA.json inmutable, comprobada por Rust antes de representar","limite":"El empaquetado no acredita por sí mismo interacción; prueba de navegador separada"});
    fs::write(
        dir.join("MANIFIESTO.json"),
        serde_json::to_vec_pretty(&prueba).unwrap(),
    )
    .unwrap();
    println!("{}; {} bytes", path.display(), html.len());
}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
