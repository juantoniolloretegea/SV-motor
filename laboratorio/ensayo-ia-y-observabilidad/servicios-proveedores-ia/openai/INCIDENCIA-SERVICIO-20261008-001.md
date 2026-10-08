# OpenAI · Recepción interrumpida de C08/R2 y recuperación

**SV-SERVICIO-OPENAI-20261008-001 · 08/10/2026.** Examen documental CYB16 de GPT-6 Astra, nodo 03. S39 r74; TT-0016 y TT-0021. C08/R2 quedó aplazada y se recibió completa al final del examen. La evaluación de la calidad del servicio permanece pendiente; no se atribuye una causa interna sin evidencia del proveedor.

| Hecho observado | Resultado |
|---|---|
| I024, C08/R2 | HTTP 200; lectura interrumpida, sin cierre ni uso completo |
| Inicio del envío | 19:59:34.900 UTC |
| Recepción interrumpida observada | 20:02:46.210 UTC |
| Duración instrumental de la operación | 192.508 ms, incluida su preparación instrumental; no equivale a duración de una caída |
| Última lectura HTTPS | 20:00:01.140 UTC |
| Argumento o código causal del proveedor | No recibido |
| I025, C09/R0 | Entrega completa a las 20:03:30.690 UTC; 44.480 ms después de la recepción interrumpida |
| I049, C08/R2, al final | Entrega completa a las 20:24:23.158 UTC; operación de 53.248 ms |

El receptor registró literalmente: «Lectura interrumpida; original parcial conservado». Es un diagnóstico local de recepción, no una explicación causal emitida por OpenAI. El HTTP 200 acredita el comienzo satisfactorio de la respuesta HTTP, no la terminación del contenido solicitado. La causa puede estar en distintos puntos del recorrido; esta evidencia no permite aislarla.

El Árbitro conservó las respuestas anteriores, prosiguió con C09–C16 y aplazó exclusivamente C08/R2. Al terminar C16/R2 realizó I049 con los mismos antecedentes e instrucciones de C08/R2, sin transmitir una corrección del evaluador. I024 no recibió valor 1 ni U ni se convirtió en respuesta científica. La interrupción no invalida las 48 entregas completas finalmente recibidas y adjudicadas.

La condición de suspensión tras cinco minutos continuos sin prestación suficiente no se alcanzó: C09/R0 llegó completa 44.480 ms después del fallo observado. El tiempo hasta el reintento deliberadamente aplazado no mide indisponibilidad. **Duración total de una eventual caída: desconocida.** La duración conocida de I024 es 192,508 segundos; no se sustituye la magnitud desconocida por ésta.

Se conservan el original SSE parcial, la solicitud, el resultado y 712 muestras Rust del intento interrumpido, con intervalo máximo de 292 ms y cero fallos de medición. Sus tokens de entrada, salida y razonamiento, así como sus créditos o importe atribuibles, son desconocidos. El posible procesamiento remoto no se considera inexistente. I049 tiene mediciones y registro de consumo propios.

El rechazo inicial de C01/R0 fue otro hecho: OpenAI había entregado una respuesta completa que el receptor común no admitió. Se corrigió y recuperó localmente sin nueva inferencia. Ese defecto del SV se conserva separado y no se imputa a la calidad del proveedor.

[Registro estructurado, huellas y tiempos cotejados en Rust](REGISTRO-SERVICIO-20261008-001.json) · [Expediente científico](../../modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/cyb16-20261008/INFORME.md) · [Código de derivación del registro](../../modelos-de-ia/acoplamientos-con-el-sv/recepcion-api-rust/src/bin/sv-incidencia-cyb16.rs).

Se conserva el marco de gestión del registro existente, ISO/IEC 42001:2023 e ISO 9001:2026, para la valoración posterior. Este documento no declara certificación, incumplimiento de cláusulas ni una puntuación de servicio todavía no establecida. Retorno: revisión competente de calidad del servicio y conciliación económica independiente de la aptitud documental del candidato.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
