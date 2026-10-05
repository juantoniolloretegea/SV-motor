# Preevaluación Qwen3.5-122B-A10B Q8_0: resultado A04/A0

Encargo QWEN35-PRE-20261004/r1. Una única solicitud, terminación normal `stop` y respuesta completa. Adjudicación externa **0: contenido correcto y cumplimiento formal completo**. A04 es crítico y tiene dificultad prefijada 3. Su recepción independiente está pendiente; la recepción documental favorable de A01–A03 se conserva separadamente y no reescribe sus cortes históricos. Cuatro casos adjudicados no permiten declarar κ ni puntuación del bloque completo.

La afirmación atribuye a D4 el cumplimiento del requisito de duración. DA04/S1, página 0, exige más de ocho minutos y, como máximo, trece; D4 registra exactamente ocho. El extremo inferior es exclusivo. La continuación confirma que cumplir los demás requisitos no sustituye el de duración. El final identifica correctamente la contradicción, conserva expediente, unidades y condición estricta, y aporta dos citas literales pertinentes de la página 0. Declara la recepción de páginas 0 y 1 y mantiene `revision: null`. Se han leído íntegramente entrada, ambas páginas, razonamiento emitido y final. El argumento detallado consta en [ADJUDICACION-EXTERNA.json](ADJUDICACION-EXTERNA.json).

## Medición observada

| Magnitud | Valor |
| --- | ---: |
| Entrada efectiva, concordante con tokenización nativa | 2123 tokens |
| Salida nativa total | 1916 tokens |
| Duración de la solicitud | 9511,126912435 s |
| Preparación nativa de entrada | 393,529 s |
| Generación nativa | 9116,369 s |
| Primera emisión observable, según RESULTADO.json | 393,59408617 s |
| Primera emisión del canal final, según RESULTADO.json | 8179,441370019 s |
| Velocidad nativa informada de generación | 0,21017139 tokens/s |
| Razonamiento emitido | 6066 bytes |
| Respuesta final | 876 bytes |
| Memoria máxima muestreada | 225182179328 bytes |
| Intercambio máximo muestreado | 0 bytes |
| Incremento de eventos del límite suave durante A04 | 0; 1582 iniciales y finales |
| Agotamientos de memoria y reinicios observados | 0 |

El diario sitúa la primera emisión a 393,592564294 s y la apertura del canal final a 8179,439411513 s, secuencia 1630. El canal final comienza con separación en blanco; ese instante no es necesariamente la primera palabra sustantiva. El cierre emitido figura a 9511,124509024 s. La actualización observada del contador de entrada se sitúa a 396,026784157 s, con muestreo nominal cada cinco segundos; no sustituye la duración interna nativa de 393,529 s. El mayor intervalo observado sin progreso nativo fue 391,017656053 s, inferior a la cota de 1800 s.

La recodificación externa del texto produce 1629 identificadores para razonamiento y 285 para el final: 1914 en conjunto, frente a 1916 tokens de salida nativa. Se conservan íntegros esos identificadores recodificados y los 2123 de entrada reproducidos. No constituyen un desglose nativo de los identificadores generados. La diferencia de dos queda sin atribución causal.

## Integridad y realización efectiva

Recuperados y cotejados íntegramente en Rust **29 originales, 65733223 bytes**, y **9 complementos, 1585178 bytes**. La auditoría verifica canales frente a la emisión SSE, huellas, entrada, páginas, citas y contrato formal; comprende nueve intercambios MCP, 1917 registros de salida y 1898 muestras de telemetría. Constancias: [AUDITORIA-RUST.json](AUDITORIA-RUST.json), [cotejo de originales](COTEJO-RECUPERACION-ORIGINALES-RUST.json) y [cotejo de complementos](COTEJO-RECUPERACION-COMPLEMENTOS-RUST.json).

Se utiliza la utilidad de medición ya compilada y contrastada tras A02. No se recompila ni sustituye ningún componente para A04. Las huellas del conductor, la medición, el manifestador y el motor coinciden con las recibidas. Tras el cierre se comprobaron cero secuencias activas y en espera, modelo cargado, cero reinicios, arranque automático deshabilitado y reinicio automático desactivado. No se consumió reparación ni reintento. Los registros nativos del modelo y del control están conservados; la consulta sin entradas del modelo en journald se distingue de su salida dirigida al archivo nativo.

| Evidencia | SHA-256 |
| --- | --- |
| Final A04 | `b8bdba0c97c43c3a9066e36ca1026f473df6afa90207e2ce70d4c0cecd8ac9f8` |
| Razonamiento A04 | `70ad921d14e42af9cd589205946c9a82a89eb9bb46780c21f4c33760268f84f6` |
| Emisión SSE A04 | `246a1f878cf6f3079c7f02fc1c432710de0569409c1deef6072a78ae5b155e38` |
| RESULTADO.json A04 | `b2ee6970164067e98f8c503e807cba59475c88cda16f57c7dc1d7d9afbd80c33` |
| Utilidad externa de medición | `babd9c5f5962c9fba464134d2689403c72a3fb0f06118ed915689cf32e7dd3bb` |
| Conductor recibido | `9cbb789d3fc5b664307ba668449866e37acaf2268981113e4967285d95483c16` |
| Motor recibido | `badd26375ed6378777c60c834d8527bfcc742bdb3adb912bf7a591875eea311b` |

## Alcance temporal y continuación

El presupuesto acumulado del control tras cuatro solicitudes es **39574,54266022 s** de los 86400 autorizados, con cero reparaciones e incidencia nula. Restan **46825,45733978 s**, suficientes para la cota prospectiva de otra solicitud. El cómputo del control se conserva separado de la duración interna de cada respuesta; preparación y duración total de campaña se registran por separado. Las cotas no se amplían por la lentitud observada.

A04 aporta una cuarta observación de generación lenta y una respuesta correcta en un supuesto crítico de límite exclusivo. No prueba corrección en los demás casos críticos, capacidad tras revisión ni transferencia al bloque B. El vector parcial contiene cuatro adjudicaciones 0 y cinco posiciones pendientes, fuera de la terna. No hay κ, puntuación global ni dictamen de acceso al examen. El siguiente caso requiere preparación íntegra y comprobación del presupuesto antes de su admisión única; esta nota no acredita que ya se haya iniciado.

La publicación del hito y la consolidación completa de fase están pendientes en este corte. Los originales íntegros están preservados en el servidor y en la recuperación local cotejada. A01–A03 están publicados y cotejados; la recepción independiente recibida para esos casos no se extiende automáticamente a A04. El resultado no constituye aptitud clínica ni ejecución del examen de 25 preguntas. Núcleo, semántica V0.2, IR 0.3, pesos y accesos permanecen intactos.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
