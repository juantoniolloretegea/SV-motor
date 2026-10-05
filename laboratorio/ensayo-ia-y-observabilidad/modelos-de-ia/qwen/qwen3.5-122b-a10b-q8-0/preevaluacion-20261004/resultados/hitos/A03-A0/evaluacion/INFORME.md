# Preevaluación Qwen3.5-122B-A10B Q8_0: resultado A03/A0

Encargo QWEN35-PRE-20261004/r1. Una única solicitud, terminación normal `stop` y respuesta completa. La adjudicación externa es **0: contenido correcto y cumplimiento formal completo**. A03 es no crítico y tiene dificultad prefijada 2. La recepción independiente permanece pendiente. Tres casos adjudicados no permiten declarar κ ni puntuación del bloque completo.

La afirmación atribuye quince piezas al contenedor Arven. La página 0 de DA03/S1 documenta doce piezas y precisa que el recuento abarcó el contenedor completo y cerrado. La continuación confirma que no se añadieron ni retiraron piezas después y que el expediente no incluye otros contenedores. La conclusión CONTRADICHA conserva esa incompatibilidad numérica, con cita literal y localizador correctos. Declara las páginas 0 y 1 y conserva `revision: null`. Se han leído íntegramente entrada, fuentes, razonamiento emitido y final. El fundamento completo consta en [ADJUDICACION-EXTERNA.json](ADJUDICACION-EXTERNA.json); el razonamiento se mantiene separado de los antecedentes futuros del candidato.

## Medición observada

| Magnitud | Valor |
| --- | ---: |
| Entrada efectiva, concordante con tokenización nativa | 2109 tokens |
| Salida nativa total | 1747 tokens |
| Duración de la solicitud | 8654,705140274 s |
| Preparación nativa de entrada | 387,989 s |
| Generación nativa | 8265,584 s |
| Primera emisión observable, según RESULTADO.json | 388,053247203 s |
| Primera emisión del canal final, según RESULTADO.json | 7754,775303676 s |
| Velocidad nativa informada de generación | 0,21135834 tokens/s |
| Razonamiento emitido | 5498 bytes |
| Respuesta final | 526 bytes |
| Memoria máxima muestreada | 225119432704 bytes |
| Intercambio máximo muestreado | 0 bytes |
| Incremento de eventos del límite suave durante A03 | 0; 1582 iniciales y finales |
| Agotamientos de memoria y reinicios observados | 0 |

El diario localiza la primera emisión a 388,051597057 s y la primera emisión final a 7754,773583927 s, secuencia 1555. Esta última comienza con separación en blanco: identifica la apertura observable del canal, no la primera palabra sustantiva. El cierre emitido figura a 8654,701923827 s. La comprobación Rust verifica la concordancia entre registros dentro de la resolución prevista. El cambio observado del contador de entrada se sitúa a 391,000222699 s, con muestreo nominal cada cinco segundos; no sustituye la duración nativa de 387,989 s. El mayor intervalo observado sin progreso nativo fue 385,990592996 s, inferior a la cota de 1800 s.

La recodificación externa del texto con el tokenizador efectivo produce 1554 identificadores para razonamiento y 191 para el final: 1745 en conjunto, frente a 1747 de salida nativa. Se conservan completos los identificadores recodificados y los 2109 de entrada reproducidos. Los recuentos por canal no son identificadores nativos generados; la diferencia de dos no se atribuye a una causa sin evidencia.

## Integridad y realización efectiva

Recuperados y cotejados íntegramente en Rust **29 originales, 60721625 bytes**, y **12 complementos, 1243423 bytes**. La auditoría verifica canales frente a la emisión original, huellas, entrada, páginas, citas y forma, con nueve intercambios MCP, 1748 registros de salida y 1727 muestras de telemetría. Constancias: [AUDITORIA-RUST.json](AUDITORIA-RUST.json), [cotejo de originales](COTEJO-RECUPERACION-ORIGINALES-RUST.json) y [cotejo de complementos](COTEJO-RECUPERACION-COMPLEMENTOS-RUST.json).

La medición utiliza la utilidad ya compilada, contrastada y recibida tras A02. No se ha recompilado ni sustituido ningún componente para A03. Las huellas del conductor, de la utilidad de medición, del manifestador y del motor recibido coinciden con las anteriores. Se comprobó cero secuencias activas y en espera tras el cierre, modelo cargado sin reinicios, arranque automático deshabilitado y reinicio automático desactivado. No se consumió reparación ni reintento.

Los complementos conservan el registro nativo completo del modelo hasta este cierre y el registro del control. La consulta del diario del modelo a través de journald carece de entradas del intervalo porque su salida se dirige al archivo nativo conservado; no equivale a ausencia de registro. Los originales A01 y A02 permanecen intactos.

Se incorporan como antecedentes tres registros de la compilación r2 recibida: terminación del perfil optimizado, código cero y fecha 04/10/2026 a las 17:26:03 UTC. Son anteriores a esta preevaluación. Su lectura permite descartar que la ausencia de un archivo externo de parámetros de generación demuestre ausencia de optimización de compilación. No acredita rendimiento óptimo ni determina qué operaciones explican la velocidad medida.

| Evidencia | SHA-256 |
| --- | --- |
| Final A03 | `34a4d1eab5fcd376d2f365ea2a5e1d5a0ae1aa99177c27f9924dac4a8d64f7e0` |
| Razonamiento A03 | `aaee82edb434ea3aaf9308e56a8388da068af0727bd33dd3b29974d0b8f9eca7` |
| Emisión SSE A03 | `aed2cc6cf2fc176177b7cd4684709db32087669064c89dab97ebfdb9e5150939` |
| RESULTADO.json A03 | `b16937d845d947d2814474516c4134ac2213eecc0a4638f525c3601afb10372c` |
| Utilidad externa de medición, sin sustitución | `babd9c5f5962c9fba464134d2689403c72a3fb0f06118ed915689cf32e7dd3bb` |
| Conductor recibido, sin sustitución | `9cbb789d3fc5b664307ba668449866e37acaf2268981113e4967285d95483c16` |
| Motor recibido, sin sustitución | `badd26375ed6378777c60c834d8527bfcc742bdb3adb912bf7a591875eea311b` |

## Alcance temporal y continuación

Tras tres solicitudes, el control registra **30060,638852654003 s** acumulados de los 86400 autorizados y cero reparaciones. Restan aproximadamente **56339,361147346 s**, con margen para otra solicitud de cota 18000 s. El cómputo incluye el intervalo de control hasta detectar el cierre y se distingue de los tiempos internos; preparación y duración de campaña se conservan por separado.

A03 aporta una tercera observación de generación lenta, aproximadamente 0,211 tokens/s, junto a 0,216 en A01 y 0,218 en A02. Las diferencias de longitud de salida contribuyen a las diferencias de duración. No se deduce de tres casos una explicación causal completa del rendimiento, un techo físico del procesador ni una mejora garantizada por cambiar de cuantización. La ausencia de nuevos eventos del límite suave durante A02 y A03 impide atribuir toda su demora al recuento acumulado de esos eventos. Memoria ocupada, tráfico de memoria y entrada efectiva son magnitudes distintas.

El vector parcial contiene tres adjudicaciones 0 y seis posiciones pendientes, fuera de la terna. No hay κ, puntuación global ni dictamen de admisión. La continuación con A04 exige preparación íntegra y nueva comprobación del presupuesto; esta nota no acredita por sí sola su inicio. A y B, las revisiones condicionadas y la recepción PDF mantienen las condiciones del encargo. El examen y la aptitud clínica siguen fuera de este resultado.

La publicación del hito y la consolidación de fase están pendientes en este corte. Los originales íntegros se conservan en el servidor y en la recuperación local cotejada. La recuperación conforme no sustituye la custodia posterior en GitHub ni la recepción independiente.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
