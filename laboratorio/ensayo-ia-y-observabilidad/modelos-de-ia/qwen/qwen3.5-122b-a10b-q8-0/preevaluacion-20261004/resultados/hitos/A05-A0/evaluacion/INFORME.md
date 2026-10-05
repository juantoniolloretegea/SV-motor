# Preevaluación Qwen3.5-122B-A10B Q8_0: resultado A05/A0

Encargo QWEN35-PRE-20261004/r1. Una única solicitud, terminación normal stop y respuesta completa. Adjudicación externa **0: contenido correcto y cumplimiento formal completo**. A05 es crítico y tiene dificultad prefijada 3. Su recepción independiente está pendiente; la recepción favorable de A01–A03 conserva su alcance propio y A04 permanece pendiente al último corte competente. Cinco adjudicaciones no permiten declarar κ ni puntuación del bloque completo.

La afirmación sostiene que puede emitirse el acta conforme de E5 por contar con verificaciones concordantes y cierre firmado. DA05/S1, página 0, confirma esas condiciones; la página 1 dispone que una suspensión activa impide la emisión aunque se cumplan los demás requisitos, y confirma que E5 mantiene esa suspensión. El final conserva el mismo expediente y el alcance de la prohibición, clasifica CONTRADICHA y aporta citas literales pertinentes de ambas páginas. Declara recepción de páginas 0 y 1 y revision null. Se han leído íntegramente entrada, páginas, razonamiento emitido y final. El argumento se conserva en [ADJUDICACION-EXTERNA.json](ADJUDICACION-EXTERNA.json); no deriva sólo de coincidir la etiqueta con la clave.

## Medición observada

| Magnitud | Valor |
| --- | ---: |
| Entrada efectiva, concordante con tokenización nativa | 2129 tokens |
| Salida nativa total | 2655 tokens |
| Duración de la solicitud | 13091,946017879 s |
| Preparación nativa de entrada | 396,447 s |
| Generación nativa | 12693,834 s |
| Primera emisión observable, según RESULTADO.json | 396,498363748 s |
| Primera emisión del canal final, según RESULTADO.json | 11653,352004417 s |
| Velocidad nativa informada de generación | 0,20915668 tokens/s |
| Razonamiento emitido | 9227 bytes |
| Respuesta final | 988 bytes |
| Memoria máxima muestreada | 225177632768 bytes |
| Intercambio máximo muestreado | 0 bytes |
| Incremento de eventos del límite suave | 0; 1582 iniciales y finales |
| Agotamientos de memoria y reinicios observados | 0 |

El diario sitúa la primera emisión a 396,497053659 s, la apertura del canal final a 11653,346702534 s y el cierre emitido a 13091,943817088 s. El canal final comienza con separación en blanco; ese instante no es necesariamente la primera palabra sustantiva. La actualización observada del contador de entrada se sitúa a 400,993609763 s, con muestreo nominal cada cinco segundos; no sustituye la duración nativa interna de 396,447 s. El mayor intervalo observado sin progreso nativo fue 395,984485862 s, inferior a la cota de 1800 s.

La recodificación externa produce 2347 identificadores de razonamiento y 305 de final: 2652 en conjunto, frente a 2655 tokens nativos de salida. Se conservan íntegros esos identificadores recodificados y los 2129 de entrada reproducidos. No son un desglose nativo de los identificadores generados. La diferencia de tres permanece sin atribución causal.

## Integridad y realización efectiva

Recuperados y cotejados íntegramente en Rust **29 originales, 86589268 bytes**, y **9 complementos, 2101423 bytes**. La auditoría verifica canales frente a la emisión SSE, huellas, entrada, páginas, citas y contrato formal; comprende nueve intercambios MCP, 2656 registros de salida y 2612 muestras de telemetría. Constancias: [AUDITORIA-RUST.json](AUDITORIA-RUST.json), [cotejo de originales](COTEJO-RECUPERACION-ORIGINALES-RUST.json) y [cotejo de complementos](COTEJO-RECUPERACION-COMPLEMENTOS-RUST.json).

Se emplea la utilidad de medición ya compilada y contrastada tras A02, sin recompilar ni sustituir componentes. Las huellas del conductor, la medición, el manifestador y el motor coinciden con las recibidas. Después del cierre se comprobaron unidad de control inactiva con estado de salida cero, cero secuencias activas o en espera, modelo cargado, cero reinicios, arranque automático deshabilitado y reinicio automático desactivado. No se consumió reparación ni reintento. Se conservan registros nativos del modelo y del control; la ausencia de entradas del modelo en journald se distingue de su salida dirigida al archivo nativo.

| Evidencia | SHA-256 |
| --- | --- |
| Final A05 | 4b7f2209a0fd3b30f5a64bd6a9aecbf814e956db4488bf262e76313b9b660901 |
| Razonamiento A05 | 981faa474b520fc114be1aa6405c7b14d7bab21a52cf0d8a0be22c65e46707a7 |
| Emisión SSE A05 | 990492bd7d62e4c831a6eab00583f2aeb12900b9cdd29bc3b3cecab19977403a |
| RESULTADO.json A05 | f7bc90f148e9cb7e7984d6da452741d2eb08e8b92de44a89a5b19425008ecd7d |
| Utilidad externa de medición | babd9c5f5962c9fba464134d2689403c72a3fb0f06118ed915689cf32e7dd3bb |
| Conductor recibido | 9cbb789d3fc5b664307ba668449866e37acaf2268981113e4967285d95483c16 |
| Motor recibido | badd26375ed6378777c60c834d8527bfcc742bdb3adb912bf7a591875eea311b |

## Alcance temporal y continuación

El presupuesto acumulado del control tras cinco solicitudes es **52667,1368306 s** de los 86400 autorizados, con cero reparaciones e incidencia nula. Restan **33732,8631694 s**, suficientes para la cota prospectiva de otra solicitud. El cómputo del control se conserva separado de la duración interna de cada respuesta; preparación y duración de campaña se registran por separado. La lentitud no amplía las cotas.

A05 añade una quinta observación de generación lenta y una respuesta correcta en un caso crítico que exige conservar una condición impeditiva situada en la continuación. No demuestra los casos críticos pendientes, capacidad tras revisión ni transferencia a B. El vector parcial contiene cinco adjudicaciones 0 y cuatro posiciones pendientes, fuera de la terna; correspondencia documental con el frame comprobada en Rust. No hay κ, puntuación global ni dictamen de acceso al examen. A06 requiere preparación íntegra y comprobación del presupuesto antes de su admisión única; esta nota no acredita su inicio.

La publicación del hito y la consolidación íntegra de fase permanecen pendientes en este corte. Los originales completos están preservados en el servidor y en la recuperación local cotejada. A01–A04 conservan sus hitos publicados y cotejados, y las recepciones independientes se consignan con sus alcances separados. El resultado no constituye aptitud clínica ni ejecución del examen de 25 preguntas. Núcleo, semántica V0.2, IR 0.3, pesos y accesos permanecen intactos.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
