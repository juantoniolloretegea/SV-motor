# Reproducción de tablas y clasificaciones

**Edición 1.4 · 08/10/2026.** Desde la raíz de esta comparación, con las dependencias fijadas:

```text
cargo run --locked --manifest-path calculo-rust/Cargo.toml --bin sv-ranquin-costes -- .
cargo run --locked --manifest-path evidencia-sv/calculo-consumos-rust/Cargo.toml -- evidencia-sv
cargo run --locked --manifest-path calculo-rust/Cargo.toml --bin comparar_ensayos -- .
cargo run --locked --manifest-path calculo-rust/Cargo.toml --bin auditar_publicacion -- verificar .
```

El primer programa conserva la referencia externa de tarifas y Artificial Analysis; no genera resultados experimentales. El segundo reproduce el inventario de consumo y la estimación GLM. El tercero contrasta sumas, cobertura, ausencias, importes y puestos de los ensayos, y genera ENSAYOS-RESUMEN.json. El último comprueba integridad y las observaciones instrumentales MD01. Puede añadirse `--offline` si las dependencias ya están disponibles.

Para actualizar el manifiesto tras una edición autorizada: `auditar_publicacion generar .`. No ejecutar generación antes del cotejo de una copia recibida: sustituiría el manifiesto que se pretende verificar.

Los programas no ejecutan inferencias ni adjudican de nuevo el contenido de las respuestas. [Método y alcance](../METODO.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
