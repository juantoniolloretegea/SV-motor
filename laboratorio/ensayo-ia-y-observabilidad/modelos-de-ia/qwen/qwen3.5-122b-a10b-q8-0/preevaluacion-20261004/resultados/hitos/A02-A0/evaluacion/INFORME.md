# Preevaluación Qwen3.5-122B-A10B Q8_0: resultado A02/A0

Encargo QWEN35-PRE-20261004/r1. Una única solicitud, terminación normal `stop` y cierre completo. La adjudicación externa es **0: contenido correcto y cumplimiento formal completo**. A02 es no crítico, con dificultad prefijada 2. La recepción independiente sigue pendiente. Dos casos adjudicados no permiten declarar κ ni puntuación del bloque completo.

La afirmación contrapone la consulta gratuita del índice de Narel y la reproducción certificada sujeta a tarifa. La respuesta cita literalmente la gratuidad en DA02/S1, página 0, y el precio de reproducción en la página 1. Ambas páginas distinguen los servicios y la continuación excluye aplicar la tarifa al índice. Su conclusión RESPALDADA y su justificación son concordantes con el conjunto documental. Declara correctamente las dos páginas y conserva `revision: null`. Se han leído completos el final y el razonamiento emitido; este último permanece separado de cualquier antecedente futuro del candidato. El fundamento detallado figura en [ADJUDICACION-EXTERNA.json](ADJUDICACION-EXTERNA.json).

## Medición observada

| Magnitud | Valor |
| --- | ---: |
| Entrada efectiva, concordante con tokenización nativa | 2117 tokens |
| Salida nativa total | 2784 tokens |
| Duración de la solicitud | 13164,299115189 s |
| Preparación nativa de entrada | 392,123 s |
| Generación nativa | 12770,411 s |
| Primera emisión observable, según RESULTADO.json | 392,186833291 s |
| Primer contenido final, según RESULTADO.json | 11957,187943465 s |
| Velocidad nativa informada de generación | 0,21800394 tokens/s |
| Razonamiento emitido | 9988 bytes |
| Respuesta final | 836 bytes |
| Memoria máxima muestreada | 225137139712 bytes |
| Intercambio máximo muestreado | 0 bytes |
| Incremento de eventos del límite suave durante A02 | 0; 1582 iniciales y finales |
| Agotamientos de memoria y reinicios observados | 0 |

El diario localiza la primera emisión a 392,184928676 s y el primer contenido final a 11957,186275626 s. Son observaciones de escritura distintas de las temporizaciones internas; la comprobación Rust verifica su concordancia dentro de la resolución prevista. El cambio observado del contador de entrada se sitúa a 396,032126732 s, con muestreo nominal cada cinco segundos; no sustituye la duración nativa exacta de 392,123 s. El mayor intervalo observado sin progreso nativo fue 391,019835664 s, inferior a la cota de 1800 s.

La recodificación externa del texto, con el tokenizador efectivo fijado, produce 2520 identificadores para razonamiento y 262 para el final. Su suma, 2782, difiere en dos del total nativo 2784. Los identificadores recodificados se conservan completos; no son identificadores originales de generación ni un desglose nativo por canal. La diferencia no se atribuye a una causa sin evidencia. Se reproducen íntegramente los 2117 identificadores de entrada.

## Integridad y realización efectiva

Recuperados y cotejados en Rust **29 originales, 87121388 bytes**, y **20 complementos, 964076 bytes**. La auditoría verifica los canales contra la emisión original, las huellas y cadenas de 2785 registros de salida, nueve intercambios MCP y 2627 muestras de telemetría, además de entrada, fuentes, páginas, citas y formato. Constancias: [AUDITORIA-RUST.json](AUDITORIA-RUST.json), [cotejo de originales](COTEJO-RECUPERACION-ORIGINALES-RUST.json) y [cotejo de complementos](COTEJO-RECUPERACION-COMPLEMENTOS-RUST.json).

La utilidad externa `medir_salida` se compiló únicamente después de comprobar cero secuencias activas y en espera. Su contraste positivo reproduce todos los identificadores recodificados de la medición previa de A01; el control negativo altera deliberadamente un identificador y obtiene rechazo en Rust, sin informe de éxito. No hubo inferencia en esas comprobaciones. Se conservan fuente, registros, referencia alterada y constancia de realización. El conductor de inferencia mantiene su huella original; no se sustituyó ni se modificaron motor, perfil, política, Núcleo, semántica, IR o pesos. Esta medición auxiliar no constituye una reparación ni un reintento del candidato.

Se conservan los registros nativos de control y una copia completa del registro del modelo hasta el cierre de A02. El diario del modelo consultado en journald está vacío porque su salida se dirige al archivo nativo conservado; no se interpreta como ausencia de registro. Los originales A01 permanecen intactos.

| Evidencia | SHA-256 |
| --- | --- |
| Final A02 | `efe10b53487889857c74693f8902a723219311c65bc5261887d88a7b3a88c107` |
| Razonamiento A02 | `0bf41797af5aa2c121f07ca9a2d3f629a5f55cdfe7fa6003f58431d6274bd51e` |
| Emisión SSE A02 | `07846d15eec4aa1667f539307733ed005bd70dc92e9f12e9e163c3845367c006` |
| RESULTADO.json A02 | `e0808f92a9e02546d810fb9fb4e6b64d08f02498e9966060257bf57d350eacda` |
| Utilidad externa de medición compilada | `babd9c5f5962c9fba464134d2689403c72a3fb0f06118ed915689cf32e7dd3bb` |
| Fuente de esa utilidad | `c2ce2eb6585aef1f37728431efd617d2485d0c9fec4b6ba81fa842ce1e3bdcdd` |
| Conductor recibido, sin sustitución | `9cbb789d3fc5b664307ba668449866e37acaf2268981113e4967285d95483c16` |

## Alcance temporal y continuación

Tras dos solicitudes, el control registra **21404,141780441 s** acumulados de los 86400 autorizados y cero reparaciones. Restan **64995,858219559 s**, con margen para otra solicitud de cota 18000 s. El cómputo de control incluye el intervalo de supervisión hasta detectar el cierre y difiere de la duración interna de cada petición. La preparación y la campaña se mantienen separadas.

A02 tardó más que A01 bajo la misma configuración. Son dos observaciones particulares; no acreditan un pronóstico validado ni incapacidad semántica general. Se mantienen los escenarios prospectivos documentados tras A01 y las cotas originales. La continuación con A03 requiere preparación íntegra y comprobación de admisión; esta nota no prueba por sí sola que A03 se haya iniciado. A y B, sus revisiones condicionadas, la recepción PDF y el acceso al examen conservan las condiciones del encargo. No existe todavía dictamen de admisión ni aptitud clínica.

La publicación de este hito y la consolidación final de fase están pendientes en este corte. Los originales siguen conservados en el servidor y en su recuperación local cotejada. La conformidad de recuperación no sustituye la custodia posterior en GitHub ni la recepción independiente.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
