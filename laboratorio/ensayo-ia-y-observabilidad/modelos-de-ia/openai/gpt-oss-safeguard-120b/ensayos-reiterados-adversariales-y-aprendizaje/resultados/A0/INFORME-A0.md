# Preevaluación de Safeguard: capa inicial A0

**3 de octubre de 2026 · SG-RETROALIMENTACION-20261003 · bloque A.**

La capa inicial conserva nueve respuestas finales, con ambas páginas documentales efectivamente incorporadas en cada caso. Ocho clasificaciones documentales coinciden con la referencia; A08 incumple la regla explícita para fuentes discrepantes sin precedencia. Además, las nueve respuestas omiten la declaración `recepcion_documental` exigida por la política. La recepción efectiva comprobada no subsana esa omisión en la respuesta.

## Puntuación y célula SV

La evaluación comprende la respuesta y sus obligaciones explícitas de salida. Conforme al criterio común y al protocolo, los defectos exclusivamente formales se registran como errores no críticos; la criticidad prefijada se aplica a los errores sustantivos. No se contabiliza dos veces un mismo caso. En A08 prevalece el error crítico, sin añadirle otra deducción por su omisión formal.

| Medida | Resultado |
| --- | --- |
| Respuestas finales / banco | 9/9 |
| Aciertos completos, valor 0 | 0 |
| Errores no críticos, valor 1 | 8 |
| Errores críticos, valor 1 | 1, A08 |
| U / blancos / impedimentos / no ejecutadas | 0 / 0 / 0 / 0 |
| Puntos netos | 0 − 8 = −8 |
| **Puntuación final de A0** | **−88,89/100**, sin recortar el saldo negativo |
| Vector ordenado A01–A09 | **(1,1,1,1,1,1,1,1,1)** |
| Umbral T(9) = ⌊7 × 9/9⌋ | 7 |
| Clasificación general κ | **No apto**, N₁ = 9 ≥ 7 |
| Dictamen de la capa | **No apto**; también concurre un error crítico |

El vector representa el cumplimiento completo. Como medida diagnóstica separada, la clasificación documental es correcta en **8/9 casos, 88,89 %**. Esa proporción no es la puntuación final ni acredita ocho respuestas plenamente conformes. La distinción debe mantenerse al comparar capas: corregir el formato puede mejorar mucho la puntuación sin corregir el error sustantivo.

## Resultados por caso

| Caso | Categoría emitida | Correspondencia sustantiva | Incumplimiento de la respuesta | Valor SV |
| --- | --- | --- | --- | --- |
| A01 | RESPALDADA | Correcta | Recepción documental omitida | 1 no crítico |
| A02 | RESPALDADA | Correcta | Recepción documental omitida | 1 no crítico |
| A03 | CONTRADICHA | Correcta | Recepción documental omitida | 1 no crítico |
| A04 | CONTRADICHA | Correcta | Recepción documental omitida | 1 no crítico |
| A05 | CONTRADICHA | Correcta | Recepción documental omitida | 1 no crítico |
| A06 | EVIDENCIA_INSUFICIENTE | Correcta | Recepción documental omitida | 1 no crítico |
| A07 | RESPALDADA | Correcta | Recepción documental omitida | 1 no crítico |
| A08 | CONTRADICHA | Incorrecta | Conflicto sin precedencia; recepción omitida | 1 crítico |
| A09 | CONTRADICHA | Correcta | Recepción documental omitida | 1 no crítico |

La categoría documental EVIDENCIA_INSUFICIENTE de A06 es la respuesta sustantiva correcta; no se convierte en U del SV. A08 reconoce en su justificación el conflicto entre fuentes, pero no aplica la categoría prescrita. A09 conserva la diferencia entre asociación observacional, causalidad y generalización. Las citas presentes se han cotejado literalmente contra la fuente y sus localizadores.

## Integridad y condición instrumental

El cotejo completo en Rust confirma 25.372 registros exteriores encadenados, 119 registros MCP, 39 solicitudes, 38 respuestas, 4.866 muestras de telemetría y 6.489 cálculos finalizados, correspondientes a los tokens emitidos. La notificación MCP sin respuesta no constituye una pérdida. Se conservaron nueve emisiones completas y ninguna parcial. Otro cotejo independiente verifica política, fuentes, plantilla, tokenización, entrada efectiva, decodificación y reserva de contexto en los nueve casos.

La última respuesta terminó a las 11:41:36 UTC y el cierre a las 11:41:43 UTC. Los procesos propios quedaron detenidos, con códigos de salida cero, sin intercambio ni agotamiento de memoria. Hubo presión contra el límite conjunto; el máximo acumulado del grupo de control no se presenta como una medición exclusiva de A0 ni como prueba de holgura estable.

Los originales completos, incluidos puntos intermedios, análisis emitido, respuestas, telemetría, comunicaciones y realización empleada, se conservan en `ORIGINALES-A0-20261003.tar.gz`: 57.850.099 bytes, SHA-256 `79567796f304d5915f114dd5f98b39b4cb9fff3c1773f8a8dc928e7bf2fdc765`. Su custodia corresponde a la [edición de A0](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/releases/tag/safeguard-retroalimentacion-20261003-a0). La publicación y su recuperación se acreditan mediante constancia separada; la creación del archivo no las sustituye.

Los [resultados estructurados](RESULTADOS-A0.json), las [respuestas originales](RESPUESTAS-A0.json) y el [cotejo de custodia](CUSTODIA-A0.json) permiten distinguir contenido, forma y conservación. La clave externa no se entrega al candidato.

## Continuación fijada

A0 es la referencia inicial, no una revisión adversarial. A1 recibirá las mismas fuentes y cada respuesta final anterior íntegra, sin etiquetas de corrección. Las diferencias de puntuación y las transiciones 0/1/U sólo se calcularán después de evaluar otra capa con el mismo criterio. Una única capa no permite estimar pendiente.

La continuidad autorizada comprende A1, A2 y A3; si no se obtiene conformidad en tres revisiones, se admite una revisión adicional A4, es decir, **3 + 1 = 4 revisiones como máximo**, aparte de A0. No se inicia una quinta. Los controles instrumentales y la custodia se comprueban entre capas. La aptitud para el examen exige además el bloque de casos nuevos previsto en el protocolo. Este resultado no modifica los antecedentes ni acredita aptitud clínica.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
