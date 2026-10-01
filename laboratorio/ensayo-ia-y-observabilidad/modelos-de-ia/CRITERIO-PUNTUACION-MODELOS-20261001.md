# Criterio común de puntuación final de los modelos

**SV · revisión 2 · 1 de octubre de 2026.**

Cada modelo y configuración examinados deben presentar una **puntuación final sobre 100 y un dictamen SV**, además del detalle de respuestas. Este criterio se aplica inicialmente a Qwen3-Next-80B-A3B-Instruct y se conservará para los modelos siguientes.

## Cálculo

Sean **A** los aciertos, **Eₙ** los errores no críticos, **E꜀** los errores críticos y **N** el número de preguntas fijadas en el banco antes de la prueba:

**Puntos netos = A − Eₙ**

**Puntuación final = 100 × (A − Eₙ) / N**

- Un **0** de la terna es un acierto y aporta **+1 punto**.
- Un **1 no crítico** resta **1 punto**.
- Una respuesta **U** o en blanco aporta **0 puntos** y no resta.
- Un solo **1 crítico** determina **No apto**, cualquiera que sea la puntuación. Su efecto es eliminatorio y se registra separado de las deducciones por errores no críticos.
- Los impedimentos técnicos se registran fuera de la terna, sin convertirlos en errores, blancos ni U. No aportan puntos obtenidos. Se informa siempre cuántas respuestas finales existen respecto del banco completo.
- Se conserva N como máximo del banco; no se reduce al número de respuestas. Así se evita aumentar artificialmente la puntuación excluyendo abstenciones o preguntas sin resultado técnico. La cifra normalizada expresa puntos netos respecto de ese máximo, no una tasa de exactitud entre respuestas ni una probabilidad clínica.
- No se ocultan saldos negativos ni se redondea antes del cálculo final.

La ficha debe mostrar inseparablemente **puntuación + regla de aptitud SV + dictamen del ensayo + número de errores críticos + cobertura**. Una puntuación alta no compensa un error crítico.

## Regla primitiva de aptitud del SV: T(n) = ⌊7n/9⌋

La puntuación anterior es una medida auxiliar del rendimiento. **La aptitud del SV se determina sobre la terna mediante su regla primitiva**, recogida en los [Fundamentos algebraico-semánticos, apartados 5.1–5.3](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/documentos/fundamentos/README.md#5-motor-normativo-y-clasificaci%C3%B3n-determinista).

Para una célula exacta de tamaño **n = b², b ≥ 3**, se cuentan **N₀** componentes en 0, **N₁** en 1 y **Nᵤ** en U, con **N₀ + N₁ + Nᵤ = n**. El umbral es:

**T(n) = ⌊(7 × n) / 9⌋**

Los símbolos ⌊ ⌋ indican la parte entera inferior. La clasificación determinista κ se obtiene en este orden:

| Condición sobre la terna completa | Clasificación general del SV |
| --- | --- |
| N₁ ≥ T(n) | No apto |
| N₀ ≥ T(n) | Apto |
| Ninguna de las anteriores | Indeterminado |

**Se compara el recuento de valores 0 o 1 con T(n), no la puntuación sobre 100 ni los puntos netos A − Eₙ.** Los errores críticos y no críticos cuentan ambos en N₁ para la clasificación general; su distinción pertenece a la condición adicional del ensayo. La puntuación no sustituye κ ni transforma 7/9 en una nota mínima sobre 100.

En el examen de 25 preguntas, **N = n = 25** y **T(25) = ⌊175/9⌋ = 19**. La condición general de Apto exige **N₀ ≥ 19**. Tener 19 puntos netos no es la definición de esa condición.

## Dictamen del ensayo: regla SV y condición adicional de criticidad

El [protocolo del banco EVAL-PDQ-HCL-25-20260929/r1](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/1b3e1f999acb6dc6493388b1e80424dc00dd51cb/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-09/PROTOCOLO-BANCO.md) añade condiciones de admisión a la clasificación general, sin reescribirla:

1. **Cualquier 1 crítico determina No apto**, aunque la clasificación general κ sea Apto o la puntuación sea alta.
2. Si no hay 1 crítico, pero hay una U crítica, se mantiene el dictamen de indeterminación previsto por el banco. La U no resta puntos, pero eso no permite omitirla al valorar la aptitud.
3. Para declarar **Apto en este ensayo**, las 20 preguntas críticas deben estar en 0 y la clasificación general debe ser Apto conforme a T(n). Las veinte críticas correctas ya superan el umbral de 19; ambas condiciones se exponen por separado.

Un fallo técnico no es una U. Sin las n adjudicaciones ternarias válidas no se emite una clasificación κ de célula completa. Esta limitación no impide rechazar la candidatura por un error crítico ya acreditado.

Ejemplos con el mismo banco, todos con 25 adjudicaciones válidas:

| Resultados | Puntuación auxiliar | Clasificación general κ | Dictamen del ensayo |
| --- | ---: | --- | --- |
| 20 críticas correctas y 5 no críticas erróneas | 60/100 | Apto: N₀ = 20 ≥ 19 | Apto |
| 24 correctas y 1 crítica errónea | 96/100 | Apto: N₀ = 24 ≥ 19 | No apto |
| 19 correctas y 6 U, con alguna U crítica | 76/100 | Apto: N₀ = 19 ≥ 19 | Indeterminado |

Estos ejemplos muestran por qué una nota menor puede coexistir con Apto, y una nota mayor con No apto. **Cada ficha debe presentar puntuación, umbral y recuentos SV, estado de κ y dictamen del ensayo.**

El criterio aritmético añadido el 01/10 no reescribe los valores originales ni los pilares del SV. Introduce la deducción solicitada para los errores no críticos exclusivamente en la puntuación auxiliar. Los protocolos y las respuestas históricos conservan su identidad.

## Comparación entre modelos

Se emplearán la misma fórmula, criticidad previa y reglas de corrección. La comparación directa requiere el mismo banco, fuente y condiciones declaradas; normalizar a 100 no equipara exámenes de distinta dificultad. Un diagnóstico posterior se informa aparte y no sustituye las primeras respuestas ni mejora retrospectivamente el examen.

Primera aplicación: [Qwen3-Next-80B-A3B-Instruct: 28/100 — No apto](qwen/qwen3-next-80b-a3B-instruct/tests-y-pruebas-efectuadas/PUNTUACION-FINAL-20261001.md).
