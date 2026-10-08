# Procedencia y reproducción de los consumos observados

**Edición: 8 de octubre de 2026.** Esta colección publica los datos cuantitativos de las ejecuciones del SV necesarios para reproducir sus agregados. No contiene datos de cuentas, medios de pago, credenciales, identificadores de acceso ni información personal operativa.

## Alcance documental

Se incluyen **363 registros de cuatro modelos**, correspondientes al 6, 7 y 8 de octubre de 2026. Se han contrastado todas las filas contra el índice de origen fijado para esta edición. Los 13 registros restantes de aquel índice quedan fuera del ámbito de estos cuatro modelos.

La comprobación de 363 filas acredita la fidelidad de esta extracción al índice: **no equivale a una auditoría exhaustiva de los 363 expedientes originales**. Para el subconjunto CYB16 sí se recuperaron los 48 registros individuales y se comprobaron mediante Rust sus huellas SHA-256, identificadores, pregunta, etapa, contadores, fechas, tiempos e importes estimados. No se ejecutaron nuevas inferencias.

Los identificadores públicos permiten relacionar las tablas de esta colección. Sus correspondencias de custodia se conservan separadamente. El lector puede examinar y recalcular todos los datos cuantitativos publicados sin acceso a otra sede.

## Archivos y unidades

| Archivo | Contenido |
|---|---|
| [CONSUMOS-POR-REGISTRO.csv](CONSUMOS-POR-REGISTRO.csv) | 363 filas, fecha, modelo, intentos declarados, contadores y estado económico. |
| [CONSUMOS-POR-MODELO.json](CONSUMOS-POR-MODELO.json) | Agregados calculados por Rust, con cobertura y ausencias explícitas. |
| [CYB16-POR-SOLICITUD.csv](CYB16-POR-SOLICITUD.csv) | 48 solicitudes de GLM-5.3: 16 preguntas y tres etapas, contadores, tiempos y cálculo económico. |
| [CYB16-RESUMEN.json](CYB16-RESUMEN.json) | Totales y agregados por etapa del mismo subconjunto. |
| [VERIFICACION.json](VERIFICACION.json) | Resultado del comprobador e integridad de las dos tablas de entrada. |
| [Código Rust](calculo-consumos-rust/src/main.rs) | Comprobación y reproducción utilizando exclusivamente las tablas públicas. |

Los tokens de entrada, salida y total son contadores comunicados por los proveedores durante ejecuciones del SV, conservados en sus registros. No son resultados de evaluaciones externas. La segmentación puede variar entre modelos: sumar tokens de proveedores distintos produce un inventario de contadores, no una medida uniforme del volumen lingüístico.

Los tiempos de CYB16 están medidos desde el cliente en milisegundos. `duracion_operacion_ms` incluye preparación y cierre instrumental, además de una espera explícita de un segundo. `primer_evento_ms` corresponde al primer evento reconocido por el adaptador; `primer_texto_ms`, a la primera cadena no vacía recibida en delta.content. Esta cadena puede contener sólo espacios: no acredita primer carácter visible ni primera palabra útil. Estas definiciones corresponden al adaptador Z.ai y no se trasladan automáticamente a otros instrumentos. No se equiparan a tiempos internos del proveedor. Las fechas de solicitud conservan UTC explícito.

Un registro puede reunir más de un intento. Por ello, 363 registros no significan 363 tareas homogéneas ni 363 respuestas completadas. Las celdas vacías del CSV representan datos no comunicados; los agregados JSON conservan `null` donde no hay importe. No se convierte la falta de datos en cero.

## Alcance económico

En 78 registros de Grok 4.7 consta un importe comunicado: suman **3,5991300000 USD**, con conciliación parcial. Existe otro registro sin importe. Esta suma no acredita una factura, un gasto completo de cuenta ni el coste de un examen común a los cuatro modelos.

CYB16 constituye un subconjunto de los 76 registros de GLM-5.3; **no debe sumarse nuevamente** al conjunto. Sus 48 solicitudes suman 1.226.311 tokens de entrada, 157.944 de salida y 1.384.255 totales. Los 52.928 tokens de caché están incluidos en la entrada y los 34.699 de razonamiento están incluidos en la salida.

La estimación de CYB16 asciende a **2,35145108 USD**. Se calcula por solicitud con las tarifas documentadas el 08/10/2026: 1,40 USD por millón de entrada ordinaria, 0,26 por millón de lectura desde caché y 4,40 por millón de salida. [Fuente tarifaria](https://docs.z.ai/guides/overview/pricing).

La fórmula es:

```text
[(entrada_total − entrada_cache) × 1,40
 + entrada_cache × 0,26
 + salida_total × 4,40] / 1.000.000
```

La liquidación individual de CYB16 permanece desconocida. La precisión decimal del cálculo no acredita la misma precisión de una futura factura. No se incorporan saldos, compras, cuotas ni promociones de cuentas.

Estos agregados reúnen pruebas diferentes. **No ordenan a los modelos por eficiencia ni por coste de resolución de un mismo encargo.** Una clasificación experimental requiere identificar las pruebas comparables y sus criterios de calidad y cobertura.

## Reproducción

Desde esta carpeta, con Rust y las dependencias fijadas en `Cargo.lock`:

```text
cargo run --locked --manifest-path calculo-consumos-rust/Cargo.toml -- .
```

Si las dependencias ya están disponibles localmente, puede añadirse `--offline`. El programa lee los dos CSV, comprueba identificadores, sumas, ausencias y correspondencias entre tablas; reproduce los agregados JSON y el resultado de verificación. Valida las 16 preguntas por las tres etapas y calcula los importes con enteros de 10⁻¹⁰ USD.

Esta reproducción permite cotejar la coherencia de los datos públicos. No certifica la exactitud de los contadores internos de los proveedores, las facturas ni la calidad de las respuestas. Los dictámenes científicos conservan sus propios criterios y evidencias.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

El licenciamiento del texto propio no modifica los derechos de las fuentes externas.

