# Preevaluación Qwen3.5-122B-A10B Q8_0: resultado A06/A0

Encargo QWEN35-PRE-20261004/r1. Una única solicitud, terminación normal stop y respuesta completa. Adjudicación externa **1 por error sustantivo crítico**, con forma conforme. A06 tiene dificultad prefijada 4. Su recepción independiente está pendiente; A01–A03 tienen recepción favorable separada y A04–A05 permanecen pendientes al último corte competente. El vector parcial contiene cinco ceros, un uno y tres posiciones pendientes: no permite declarar κ ni puntuación global.

## Contraste documental

La afirmación atribuye a las muestras juveniles de Faril una permanencia de la señal durante una semana. DA06/S1, página 0, sólo describe diez muestras adultas y siete con señal durante dos días. La continuación, página 1, declara que el informe no incluye muestras juveniles ni observaciones posteriores al segundo día. Esto delimita lo observado, pero no niega que las muestras juveniles puedan conservar la señal una semana.

El final clasifica CONTRADICHA y justifica esa decisión por la ausencia de datos de juveniles y de observación posterior. Cambia así el objeto de la afirmación: de la persistencia de una señal a la existencia de datos sobre esa persistencia en el informe. La decisión correcta es EVIDENCIA_INSUFICIENTE, porque faltan observaciones de la población y del intervalo afirmados. La ausencia documental no acredita una contradicción del fenómeno. El error afecta población, temporalidad e incertidumbre y contraviene D3 y D5.

Se han leído íntegramente entrada efectiva, las dos páginas, razonamiento emitido y final. El razonamiento observable mantiene esa misma inferencia incorrecta; se conserva sin reescribir, incluida su terminación textual. No se reconstruye un proceso que no haya sido emitido. Ambas citas finales son literales y pertinentes, la recepción [0,1] coincide con la incorporación cotejada y el único objeto JSON cumple los seis campos y revision null. La forma correcta no subsana el contenido. [ADJUDICACION-EXTERNA.json](ADJUDICACION-EXTERNA.json) conserva el argumento externo, que no deriva únicamente de comparar etiquetas. Este resultado válido incorrecto no es U ni incidencia instrumental y no autoriza un reintento.

## Medición observada

| Magnitud | Valor |
| --- | ---: |
| Entrada efectiva concordante con tokenización nativa | 2108 tokens |
| Salida nativa total | 1754 tokens |
| Duración de la solicitud | 8812,145835575 s |
| Preparación nativa de entrada | 398,958 s |
| Generación nativa | 8412,067 s |
| Primera emisión observable según RESULTADO.json | 399,021909784 s |
| Primera emisión del canal final según RESULTADO.json | 7459,002730999 s |
| Velocidad nativa de generación | 0,20850998 tokens/s |
| Razonamiento emitido | 5816 bytes |
| Respuesta final | 919 bytes |
| Memoria máxima muestreada | 225177513984 bytes |
| Intercambio máximo muestreado | 0 bytes |
| Incremento de eventos del límite suave | 0; 1582 iniciales y finales |
| Agotamientos de memoria y reinicios observados | 0 |

El diario sitúa la primera emisión a 399,020375164 s, la apertura del canal final a 7459,000851451 s y el cierre a 8812,143656535 s. La separación inicial en blanco no fija la primera palabra sustantiva. La actualización observada del contador de entrada se sitúa a 401,063512941 s, con muestreo nominal cada cinco segundos; no sustituye la temporización nativa interna de 398,958 s. El mayor intervalo sin progreso nativo observado fue 396,053049709 s, inferior a 1800 s.

La recodificación externa produce 1467 identificadores de razonamiento y 284 del final: 1751 en conjunto frente a 1754 tokens nativos de salida. Los identificadores completos y los 2108 de entrada reproducidos quedan conservados. No son un desglose de identificadores nativos emitidos; la diferencia de tres permanece sin atribución causal.

## Integridad y realización efectiva

Recuperados y cotejados íntegramente en Rust **29 originales, 61634698 bytes**, y **9 complementos, 2424724 bytes**. La auditoría de integridad y forma es conforme: nueve intercambios MCP, 1755 registros de salida y 1758 muestras de telemetría. Constancias: [AUDITORIA-RUST.json](AUDITORIA-RUST.json), [cotejo de originales](COTEJO-RECUPERACION-ORIGINALES-RUST.json) y [cotejo de complementos](COTEJO-RECUPERACION-COMPLEMENTOS-RUST.json).

Se utiliza la medición ya compilada y contrastada tras A02, sin nuevas compilaciones ni sustituciones. Las huellas efectivas del conductor, medidor, manifestador y motor coinciden con las recibidas. Tras el cierre se comprobaron control inactivo con salida cero, cero secuencias activas y en espera, modelo cargado, arranque automático deshabilitado y reinicio automático desactivado. No se consumió reparación ni reintento. Se conservan registros nativos del control y del modelo; su salida al archivo propio se distingue de un diario journald vacío.

| Evidencia | SHA-256 |
| --- | --- |
| Final A06 | 749b41f702bd03397f6409e46d0d6d0b55923a4e46e241ee8a305b4ce4a8bb80 |
| Razonamiento A06 | 638de9dfa58d943a46219af328650b6d74b86d9478dede73484736a0db17073a |
| Emisión SSE A06 | 74e34a5e38db3c96dae6eefaefadeba898a73752066f16f962b18fdb708d56d8 |
| RESULTADO.json A06 | 5076b67d4787c1549bbbee8a82770d9b6d4e49a06568c59a7645f9c8b28abf07 |
| Utilidad de medición | babd9c5f5962c9fba464134d2689403c72a3fb0f06118ed915689cf32e7dd3bb |
| Conductor recibido | 9cbb789d3fc5b664307ba668449866e37acaf2268981113e4967285d95483c16 |
| Motor recibido | badd26375ed6378777c60c834d8527bfcc742bdb3adb912bf7a591875eea311b |

## Alcance y continuación

El presupuesto acumulado del control tras seis solicitudes es **61479,52822988 s**, con incidencia nula y cero reparaciones. Restan **24920,47177012 s** de las 86400 autorizadas, suficientes para la cota prospectiva de otra solicitud de 18000 s. La duración del control se mantiene separada de la medición interna de cada resultado; preparación y campaña se registran por separado.

A06 demuestra un error crítico de esta respuesta inicial, con transmisión completa y lectura disponible de ambas páginas. No demuestra incapacidad semántica universal ni el resultado de una revisión futura. Se conserva el vector parcial de cinco ceros, un uno crítico y tres posiciones pendientes, con correspondencia al frame verificada en Rust. El error crítico impide la conformidad de esta capa si se mantiene; no se sustituye por un acierto posterior ni se mezcla con otra capa. A0 debe continuar en su orden dentro de las cotas; no procede adelantar A1, B ni R-PDF-01.

A07 requiere preparación íntegra, contador nativo concordante, ausencia de otra secuencia y comprobación del presupuesto antes de una admisión única; este informe no acredita su inicio. Publicación del hito A06 y consolidación de fase pendientes en este corte. A01–A05 mantienen sus hitos publicados, recuperados y cotejados. Los originales completos siguen en el servidor y en la recuperación local cotejada. Las recepciones independientes se mantienen separadas. No constituye aptitud clínica ni ejecución del examen de 25 preguntas. Núcleo, semántica V0.2, IR 0.3, pesos y accesos intactos.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).