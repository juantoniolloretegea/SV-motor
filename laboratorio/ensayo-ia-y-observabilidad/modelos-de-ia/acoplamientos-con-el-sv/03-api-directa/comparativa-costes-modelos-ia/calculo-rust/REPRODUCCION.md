# Reproducción de datos, clasificaciones e integridad en Rust

Versión documental 1.1. Los programas trabajan sobre los archivos publicados: no se conectan a los proveedores ni realizan inferencias. Las dependencias están fijadas en Cargo.lock; conservan sus propias licencias.

Desde la raíz de esta comparativa:

```text
cargo test --locked --manifest-path calculo-rust/Cargo.toml
cargo run --locked --manifest-path calculo-rust/Cargo.toml --bin sv-ranquin-costes -- .
cargo run --locked --manifest-path evidencia-sv/calculo-consumos-rust/Cargo.toml -- evidencia-sv
cargo run --locked --manifest-path calculo-rust/Cargo.toml --bin auditar_publicacion -- verificar .
```

Con las dependencias ya disponibles puede añadirse `--offline`. La verificación de esta edición se realiza sin nuevas instalaciones.

## Datos externos

El programa `sv-ranquin-costes` lee DATOS.json, reproduce el orden de Artificial Analysis, conserva empates y ausencias, calcula la mezcla de precios 7:2:1 y genera la figura. Sus salidas quedan en resultados/: RANQUIN.json, RANQUIN.csv y COSTE-Y-CAPACIDAD.svg. Deben coincidir por bytes con las tres copias publicadas en la raíz.

El cálculo utiliza enteros; separa el precio externo de caché de Qwen de su tarifa oficial ausente. El JSON incluye la huella de DATOS.json. Las pruebas cubren empates, ausencia frente a cero, fórmula de precios y distinción entre orden económico y capacidad. Esto reproduce aritmética y atribución documental, no el experimento del evaluador externo.

## Consumos de ejecuciones del SV

El [segundo programa](../evidencia-sv/calculo-consumos-rust/src/main.rs) lee únicamente los dos CSV de evidencia-sv. Reproduce los agregados por modelo, el detalle económico CYB16 y VERIFICACION.json. Comprueba identidades públicas, ausencias, sumas, correspondencias y las 16 preguntas por tres etapas. Usa enteros de 10⁻¹⁰ USD para el cálculo monetario.

Su ejecución vuelve a escribir los tres JSON derivados dentro de evidencia-sv. Una reproducción fiel debe conservar su contenido exacto. El [alcance documental](../evidencia-sv/PROCEDENCIA.md) distingue la reproducción pública de la comprobación contra las fuentes de origen.

## Observaciones MD01 e integridad

`auditar_publicacion verificar .` no escribe. Extrae las observaciones R0 de DATOS-SV.json sin asignar puestos y compara bytes de OBSERVACIONES-MD01.json y OBSERVACIONES-MD01.csv y coteja el inventario, tamaños y SHA-256 contra MANIFIESTO.json. No acepta datos ausentes como cero, sumas incoherentes, duplicados ni modelos ajenos a MD01. Preserva las limitaciones instrumentales.

Las pruebas cubren extracción exclusiva R0 sin puestos, ausencias, duplicados, sumas, desbordamiento y alteración de contenido de igual tamaño. Se rechazan enlaces simbólicos; se excluyen target, resultados, .git y el manifiesto raíz de su propio inventario.

Para elaborar una nueva revisión, una vez fijados sus documentos y resultados:

```text
cargo run --locked --manifest-path calculo-rust/Cargo.toml --bin auditar_publicacion -- generar .
```

Este modo sustituye las dos salidas MD01 y el manifiesto; no debe utilizarse para ocultar discrepancias de una edición recibida. La recepción remota se comprueba siempre con `verificar`, contra el manifiesto previamente fijado.

La integridad de archivos y la reproducción de cálculos no certifican liquidación, exactitud interna de los proveedores, igualdad experimental o calidad científica. Esas condiciones se documentan por separado.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
