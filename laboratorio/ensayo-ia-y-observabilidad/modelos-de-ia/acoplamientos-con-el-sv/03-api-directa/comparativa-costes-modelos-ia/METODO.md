# Método de comparación

**Edición 1.4 · 08/10/2026.**

## Resultados propios y referencias externas

Las respuestas y duraciones proceden de ensayos del SV. Los contadores de tokens e importes por solicitud son los comunicados por cada proveedor. Las tarifas publicadas permiten cálculos económicos identificados como estimaciones. Artificial Analysis es una referencia externa, separada de esos ensayos.

## Unidad de comparación y clasificación

PDQ25 compara Astra y Grok sobre las mismas 25 preguntas. PDQ16 restringe ambos a P01–P16 y añade Qwen sobre sus 16 preguntas efectivamente completadas. Se cotejaron banco, catálogo, fuente y suministros; no se combinan con la réplica posterior de P13. CYB16 compara Astra y GLM sobre 16 preguntas, corpus y criterios coincidentes.

R0, R1 y R2 permanecen separadas. R2 es siempre final; no se escoge la mejor respuesta. El puesto ordena la cantidad de posiciones conformes dentro de una prueba y etapa. Los empates comparten puesto y el siguiente refleja cuántos candidatos lo preceden. Un incumplimiento crítico impide admisión aunque el número de conformidades alcance el umbral.

La reserva metodológica de Qwen acompaña a su clasificación: diferencias de marcación literal y el alcance de P13 no se reinterpretan como falsedades clínicas. Los puestos contractuales no sustituyen una revisión sustantiva independiente. El subconjunto PDQ16 no constituye otra repetición experimental.

Los dictámenes son los publicados, exteriores al candidato y asistidos por IA. La recepción científica independiente permanece pendiente. Tres etapas dependientes no son tres evaluadores. Una ejecución no demuestra estabilidad estadística ni superioridad general.

## Consumo y duración

La tabla de exámenes contiene 297 intentos científicos de cinco campañas, con pregunta, etapa, estado y fuente. Los tres intentos incompletos conservan sus tiempos y contadores desconocidos. No se duplican las respuestas recuperadas desde su flujo original.

La suma temporal usa la operación observada: en Astra PDQ, `duracion_observada_ms`; en los demás, `duracion_operacion_ms`. Incluye los intentos incompletos. Preparación, cierre y espera instrumental no son idénticos entre receptores; el orden temporal describe las ejecuciones conservadas y no una medición controlada de latencia interna. No incluye las pausas entre operaciones. Mediana inferior y percentil 95 de CYB16 se calculan sobre las 48 entregas completas; el percentil usa rango más próximo.

Los tokens de caché son parte de la entrada y los de razonamiento parte de la salida. No se suman dos veces ni se interpretan como una unidad semántica universal. Un dato no comunicado se conserva como desconocido.

El inventario de 412 registros incluye otros ensayos y comprobaciones. Sirve para agregación de consumo; las comparaciones por tarea se calculan desde la tabla de exámenes identificados, no desde esos totales heterogéneos.

## Economía

El coste de Grok PDQ25 suma importes por solicitud comunicados por el proveedor. GLM CYB16 usa la fórmula de entrada ordinaria, lectura de caché y salida publicada con sus tarifas fechadas. Ninguno equivale por sí solo a una factura. Para Astra y Qwen no hay importe atribuible comunicado.

El ejemplo comercial de un millón de tokens de entrada y cien mil de salida, sin caché, aplica idéntico volumen a los cuatro precios. Es una comparación de tarifas, no de trabajo resuelto. El coste por tarea de Artificial Analysis conserva su propio procedimiento y configuración. No se divide su índice por cien para calcular una tasa de acierto.

## Reproducción

[Datos y sumas](evidencia-sv/PROCEDENCIA.md) · [Código y comandos](calculo-rust/REPRODUCCION.md) · [Vectores](RANQUIN-CALIDAD-SV.json) · [Manifiesto de integridad](MANIFIESTO.json). Rust reproduce agregados, estimaciones y puestos desde los datos publicados; no vuelve a adjudicar semánticamente las respuestas.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
