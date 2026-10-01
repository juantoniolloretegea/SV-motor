# Puntuación final · Qwen3-Next-80B-A3B-Instruct

**28/100 — No apto**

**1 de octubre de 2026 · Examen v7 · EVAL-PDQ-HCL-25-20260929/r1 · S39 / TT-0016.**

## Cálculo y resultado

**7 aciertos − 0 errores no críticos = 7 puntos netos de un máximo de 25.**

**Puntuación final: 100 × 7 / 25 = 28/100.**

**Dictamen: No apto por dos errores críticos, P07 y P19.** Un solo error crítico basta para la exclusión; los aciertos no lo compensan. Estos dos errores tienen efecto eliminatorio y no se confunden con la deducción aritmética de errores no críticos.

| Concepto | Resultado | Efecto |
| --- | ---: | --- |
| Aciertos, valor 0 | 7 | +7 puntos |
| Errores no críticos, valor 1 | 0 | 0 puntos deducidos |
| Errores críticos, valor 1 | 2 | No apto |
| Indeterminaciones, U | 10 | 0 puntos; no penalizan |
| Respuestas en blanco | 0 | 0 puntos |
| Impedimentos técnicos | 6 | Fuera de la terna; sin puntos obtenidos |
| Respuestas finales conservadas | 19 de 25 | Cobertura declarada |

La puntuación expresa los puntos netos obtenidos respecto del máximo del banco. No es una tasa de exactitud entre las 19 respuestas, no atribuye valor ternario a los seis impedimentos y no constituye un vector completo de 25 adjudicaciones.

## Aptitud SV y dictamen del ensayo

La [regla primitiva del SV](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/documentos/fundamentos/README.md#5-motor-normativo-y-clasificaci%C3%B3n-determinista) establece **T(n) = ⌊7n/9⌋**. Para este banco, **n = 25 y T(25) = 19**.

| Magnitud | Resultado |
| --- | --- |
| Valores 0 conservados, N₀ | 7 |
| Valores 1 conservados, N₁ | 2 |
| Valores U conservados, Nᵤ | 10 |
| Posiciones sin adjudicación ternaria por impedimento técnico | 6 |
| Umbral general T(25) | 19 |
| Condición general N₀ ≥ 19 | No alcanzada |
| Clasificación κ de célula completa | No emitida: sólo hay 19 adjudicaciones válidas de 25 |
| Dictamen del ensayo | **No apto**, por P07 y P19 críticas con valor 1 |

El **28/100** procede del cálculo auxiliar de puntos. El **No apto** procede del veto por errores críticos del [protocolo del ensayo](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/1b3e1f999acb6dc6493388b1e80424dc00dd51cb/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-09/PROTOCOLO-BANCO.md); no se obtiene comparando 28 con una nota mínima. La insuficiencia documental de las seis posiciones restantes no se rellena con U ni se presenta como una clasificación general completa.

## Errores críticos determinantes

- **P07:** la corrección conservada identifica una inferencia falsa de ausencia de contenido en la fuente a partir de búsquedas sin resultados.
- **P19:** la corrección conservada identifica una generalización absoluta que elimina el matiz de la fuente sobre cambios medulares.

Se conservan las adjudicaciones originales y sus fundamentos; el cálculo no realiza una nueva corrección médica. Incluso si se revisara favorablemente P19, el error crítico de P07 seguiría determinando No apto.

## Regla común y trazabilidad

Se aplica el [criterio común de puntuación](../../../CRITERIO-PUNTUACION-MODELOS-20261001.md): +1 por acierto, −1 por error no crítico, 0 por U o blanco y exclusión por cualquier error crítico. La denominación del dictamen SV es **No apto**.

Fuente fijada: [resultados originales del examen](https://github.com/juantoniolloretegea/SV-motor/blob/2d77c181312cb467465a4e9a9bc501b840cdbd41/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3-next-80b-a3B-instruct/tests-y-pruebas-efectuadas/RESULTADOS-PDQ-HCL-25-20260930.md), con [adjudicaciones JSON](https://github.com/juantoniolloretegea/SV-motor/blob/2d77c181312cb467465a4e9a9bc501b840cdbd41/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3-next-80b-a3B-instruct/tests-y-pruebas-efectuadas/RESULTADOS-PDQ-HCL-25-20260930.json); commit `2d77c181312cb467465a4e9a9bc501b840cdbd41`, objeto JSON `c599eebedda33227cb2eb209efecfa196d43110f`.

La corrección original estaba identificada como documental para revisión humana. Esta ficha añade la puntuación y aplica el criterio eliminatorio solicitado, sin declarar una recepción institucional ni validación clínica. La fórmula del 01/10 se distingue del protocolo aritmético histórico. El diagnóstico v8 de cuatro condiciones se conserva separado y **no sustituye ni mejora esta puntuación**.

[Datos estructurados de la puntuación](PUNTUACION-FINAL-20261001.json).
