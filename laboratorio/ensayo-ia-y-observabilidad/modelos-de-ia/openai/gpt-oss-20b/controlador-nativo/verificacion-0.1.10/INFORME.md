# Verificación instrumental 0.1.10

23 de septiembre de 2026. Rust/Cargo 1.98.0; compilación y pruebas locales con dependencias fijadas, sin acceso a red.

`cargo test --offline --locked -- --test-threads=1`: 17 registros superados, 15 pruebas funcionales y dos auxiliares. La nueva regresión conserva una escritura correcta demorada 400 ms. Se mantienen las comprobaciones de señales reales, muerte del padre, plazos, custodia fallida, atribución del puerto y cierre del hijo. `cargo build --release --offline --locked`: satisfactorio.

[Pruebas](PRUEBAS.log), [compilación](COMPILACION.log) y [huellas](SHA256SUMS.txt).

La referencia incorpora las correcciones de contabilización de memoria y acuse de escritura verificadas en la base 0.1.8; conserva los argumentos numéricos originales de 0.1.3. No constituye una ejecución de gpt-oss, una prueba de capacidad del entorno, una cuota agregada ni una guarda exterior. La evidencia real y sus limitaciones figuran en la [rectificación](../../resultados/continuacion-2026-09-23/RECTIFICACION_CONTROLADOR.md).

Sistema Vectorial SV · [Aviso y licencias](../AVISO_LICENCIAS.json).
