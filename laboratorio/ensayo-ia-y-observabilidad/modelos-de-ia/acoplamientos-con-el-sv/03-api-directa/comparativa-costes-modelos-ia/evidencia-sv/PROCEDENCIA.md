# Datos de consumo y reproducción

**Edición 1.4 · 08/10/2026.**

| Datos | Alcance |
|---|---|
| [ENSAYOS-POR-SOLICITUD.json](ENSAYOS-POR-SOLICITUD.json) | 297 intentos científicos: Astra PDQ25, Grok PDQ25, Qwen PDQ16, Astra CYB16 y GLM CYB16. Identificación por prueba, modelo, pregunta, etapa y estado; fuente pública por fila. |
| [ENSAYOS-RESUMEN.json](ENSAYOS-RESUMEN.json) | Sumas de tokens, duración e importes por examen y etapa; tabla del subconjunto común P01–P16. |
| [CONSUMOS-POR-REGISTRO.csv](CONSUMOS-POR-REGISTRO.csv) | 412 registros de consumo de los cuatro modelos, incluidas otras pruebas y comprobaciones. No son 412 tareas equivalentes. |
| [CONSUMOS-POR-MODELO.json](CONSUMOS-POR-MODELO.json) | Agregados del inventario general. Los exámenes ya están incluidos; no se suman otra vez. |
| [CYB16-POR-SOLICITUD.csv](CYB16-POR-SOLICITUD.csv) | Detalle de las 48 solicitudes de GLM, con desglose de su estimación monetaria. |
| [CYB16-RESUMEN.json](CYB16-RESUMEN.json) | Reproducción de la estimación GLM por etapas. |

Las fuentes científicas y mediciones públicas están fijadas por revisión en cada fila. Para Grok se cotejaron también las 75 declaraciones individuales del proveedor y sus importes; se publica sólo su medición por solicitud. El inventario general se cotejó con su índice; no se afirma una auditoría de todos los originales operativos de sus 412 registros. Las 297 filas permiten reconstruir las cinco comparaciones sin depender del inventario general.

## Unidades y datos ausentes

Los contadores nativos de entrada y salida proceden de respuestas a ejecuciones del SV. Caché y razonamiento son subconjuntos, no consumos adicionales. Los tres intentos científicos incompletos conservan sus tiempos y valores de uso desconocidos. Una celda vacía o `null` no representa cero. Los identificadores E sólo distinguen filas y no expresan orden cronológico.

La duración de operación suma preparación, espera y cierre medidos por el cliente. En Astra PDQ se recupera `duracion_observada_ms`; en los demás, `duracion_operacion_ms`. No se homologa la latencia interna de los proveedores. Las definiciones de primer texto varían según receptor y no se usan para establecer el ranquin temporal. Las medianas inferiores y percentiles de CYB16 se calculan sobre entregas completas.

## Costes

Grok PDQ25: suma de importes individuales comunicados en unidades de 10⁻¹⁰ USD: **3,3866420000 USD**. El conjunto de sus 78 registros con importe suma **3,5991300000 USD**. Son alcances distintos.

GLM CYB16: **2,35145108 USD estimados** con las tarifas fechadas del 08/10/2026: entrada ordinaria 1,40 USD/M, lectura de caché 0,26 USD/M y salida 4,40 USD/M. [Fuente tarifaria](https://docs.z.ai/guides/overview/pricing).

```text
coste = [(entrada − caché) × 1,40 + caché × 0,26 + salida × 4,40] / 1.000.000
```

No se dispone de importe monetario comunicado para Astra y Qwen. Un importe comunicado o estimado no equivale por sí solo a factura. La comparación comercial con igual volumen de tokens es otro cálculo identificado en el informe.

## Reproducción en Rust

Desde esta carpeta:

```text
cargo run --locked --manifest-path calculo-consumos-rust/Cargo.toml -- .
cargo run --locked --manifest-path ../calculo-rust/Cargo.toml --bin comparar_ensayos -- ..
```

Puede añadirse `--offline` si las dependencias fijadas ya están disponibles. El primer programa reproduce los 412 registros y GLM CYB16; el segundo verifica sumas, cobertura por pregunta y etapa, desconocidos, importes y puestos contractuales, y reproduce los resúmenes de los cinco exámenes. No vuelve a adjudicar las respuestas ni certifica sistemas internos del proveedor.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
