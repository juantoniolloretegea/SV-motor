# Compilación y consulta del visor 0.2.0

Rust 1.98.0, eframe/egui 0.33.3, wasm-bindgen 0.2.129 y Cargo.lock fijado. Código propio con prohibición de unsafe. CAPA.json está incorporado en el módulo y su huella se comprueba al iniciar. Las dependencias de terceros conservan sus licencias; el documento no modifica sus términos.

Desde este directorio, con los destinos y dependencias correspondientes disponibles:

```text
cargo +1.98.0 test --locked --lib
cargo +1.98.0 build --locked --release --lib --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/sv_visor_catalogo.wasm --target web --out-dir web
cargo +1.98.0 run --locked --bin empaquetar -- web
```

Si CARGO_TARGET_DIR está definido, ajustar únicamente la ruta del módulo de entrada a wasm-bindgen. No mezclar versiones de su biblioteca y su herramienta: ambas deben ser 0.2.129. Herramienta Windows oficial cotejada: archivo wasm-bindgen-0.2.129-x86_64-pc-windows-msvc.tar.gz, SHA-256 79e348c169d0c10c0b287647f1e753d0985353ca96800bc29be8f4261baf13e6. No se requiere instalar esa herramienta para abrir el HTML resultante.

La salida web/POLIGONO-EGUI.html incluye el módulo y el enlace técnico generado. La consulta exige WebAssembly y WebGL. La implementación funcional y el empaquetado se realizan en Rust. La variante nativa se abre con `cargo run --locked --bin sv-visor-catalogo`; no requiere argumentos ni fuentes descargadas durante su uso. La verificación visual de esta edición se realizó sobre la variante de navegador, no sobre una nueva ventana nativa.

El auxiliar opcional `vista_local` sirve exclusivamente los bytes del HTML indicado, en una dirección de bucle local y puerto asignado por el sistema, sin exponer un directorio. Su vida está acotada a 120 minutos. No forma parte del documento autónomo ni realiza inferencia. `VISTA-LOCAL.json` identifica su dirección y proceso para la administración local; no se publica.

La huella de la fuente debe seguir siendo 1be4b387d9fb44fe4a6e9d3bfae0aa2db24cd93caf3aa389722a2a964dc766be. Cambiar una adjudicación exige un expediente nuevo; no se edita para modificar la apariencia.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
