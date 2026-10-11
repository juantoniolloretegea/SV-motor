# Reproducción de la comprobación de admisión

Se requiere Rust 1.98.1 y acceso inicial al registro para recuperar las dependencias exactas de Cargo.lock. Las ediciones de terceros conservan sus respectivas licencias; las bibliotecas Logicaffeine utilizan BUSL-1.1. Este procedimiento no instala servicios.

Desde esta carpeta de evidencias:

```text
cargo build --locked --manifest-path codigo/Cargo.toml
cargo run --locked --offline --manifest-path codigo/Cargo.toml --bin comprobacion-admision-documental -- . codigo/Cargo.lock codigo/src/main.rs EJECUCION-01
cargo run --locked --offline --manifest-path codigo/Cargo.toml --bin comprobacion-admision-documental -- . codigo/Cargo.lock codigo/src/main.rs EJECUCION-02
cargo run --locked --offline --manifest-path codigo/Cargo.toml --bin cotejar -- . codigo/Cargo.lock
```

La reproducción se hará en una copia de trabajo para conservar las salidas originales. El cotejo exige las catorce entradas previstas, fijaciones idénticas y salidas sustantivas idénticas. No se comparan duraciones exactas ni se considera que una compilación o cotejo conforme demuestra conservación semántica.

Se conserva el código propio del instrumento como evidencia exacta de lo ejecutado. La atribución y licencia de los documentos propios se indican al pie; no sustituyen las licencias de dependencias de terceros.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
