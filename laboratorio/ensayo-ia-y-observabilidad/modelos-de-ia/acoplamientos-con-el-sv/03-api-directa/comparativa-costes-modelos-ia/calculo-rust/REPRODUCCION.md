# Reproducción del ranquin en Rust

Versión 1.0.0. La herramienta no se conecta a proveedores, no ejecuta inferencia y no modifica resultados científicos del SV. Lee únicamente `DATOS.json`, ordena magnitudes públicas y genera tablas y un gráfico.

Desde la carpeta de la comparativa:

```text
cargo test --locked --manifest-path calculo-rust/Cargo.toml
cargo run --locked --manifest-path calculo-rust/Cargo.toml -- .
```

Con dependencias ya disponibles se puede añadir `--offline`. Las salidas quedan en `resultados/`: `RANQUIN.json`, `RANQUIN.csv` y `COSTE-Y-CAPACIDAD.svg`. Las copias publicadas se encuentran en la raíz de la comparativa. La ejecución de comprobación se realizó sin nuevas instalaciones.

El programa utiliza enteros para precios, puestos y tiempos. Calcula el puesto como uno más el número de valores estrictamente mejores: dos valores iguales conservan el mismo puesto. Valida cuatro identificadores distintos y la inclusión del razonamiento en las salidas aproximadas. La ausencia de precio oficial de caché no se convierte en gratuidad. Los precios ponderados de la referencia externa y los calculables con fuentes oficiales permanecen en campos distintos.

Se ejecutaron cuatro pruebas: conservación de empates; ausencia frente a cero; fórmula común 7:2:1; separación entre orden económico y orden de capacidad. La figura utiliza los mismos datos y ejes con origen cero. No es un polígono de adjudicación ni una célula del SV.

`RANQUIN.json` conserva SHA-256 de los datos de entrada. La comprobación reproduce el orden y la aritmética, no repite ni certifica las evaluaciones de Artificial Analysis. Las dependencias `serde_json` y `sha2`, fijadas mediante Cargo.lock, conservan sus propias licencias.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
