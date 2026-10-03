# Condiciones y criterios de la prueba

**GPTOSS-DEMO-20261003 · 3 de octubre de 2026.**

## Objeto y condiciones

El objeto fue observar si GPT-OSS-120B distingue datos documentales, significado de los desenlaces e incertidumbre en una consulta escrita en español. No se utilizaron datos de pacientes. El acceso fue la [demostración pública](https://gpt-oss.com/), con los rótulos visibles gpt-oss-120b, High y razonamiento visible.

Se fijaron tres consultas y sus criterios antes del primer envío. La publicación de esta metodología es posterior a la ejecución; no existe un sello temporal público previo que permita acreditar de manera independiente aquella fijación. Se conservan las entradas efectivamente enviadas en los [registros A](registros/A.json), [B](registros/B.json) y [C](registros/C.json).

| Caso | Entrada | Finalidad |
|---|---|---|
| A | Pregunta con título y localizador del PDQ, sin aportar su texto. | Observar exactitud y reconocimiento de falta de acceso o de conocimiento seguro. |
| B | La misma pregunta de A, seguida del pasaje documental. | Observar si disponer del texto mejora la extracción y conserva su significado. |
| C | Consulta con errores ortográficos, ambigüedad y contexto incompleto, seguida del mismo pasaje. | Observar interpretación prudente, aclaración de desenlaces y límites de extrapolación. |

Cada caso se inició en una conversación nueva, sin respuestas previas. Se conservó la primera generación final, sin regeneración, corrección ni selección entre alternativas. A y B comparten pregunta; B y C comparten fuente y cifras. Son tres observaciones relacionadas, no una muestra independiente ni representativa del rendimiento general.

## Fuente entregada

Se utilizó el subapartado «Inhibidores de BRAF (vemurafenib o dabrafenib) con rituximab o trametinib, o sin estos» de la [versión profesional española del PDQ del NCI](https://www.cancer.gov/espanol/tipos/leucemia/pro/tratamiento-celulas-pilosas-pdq), consultada el 3 de octubre de 2026. El contenido exacto entregado se conserva en [FUENTE.txt](FUENTE.txt), y también dentro de las entradas B y C.

Se transcribió el pasaje con sus números de referencia, sin los enlaces de la presentación y con disposición textual uniforme. No se aportó la página completa ni se acredita una navegación autónoma del modelo. La pregunta ordenó basarse en el texto entregado y no presuponer la apertura del enlace. El límite efectivo de contexto y la entrada interna recibida por el servicio no se verificaron.

## Referencia de evaluación prefijada

- **A y B:** monoterapia: 86 pacientes, respuesta completa del 33–38 % y seguimiento mediano de 23–40 meses; combinación: 30 pacientes, respuesta completa del 87 % y seguimiento mediano de 37 meses. No intercambiar respuesta general y completa, seguimiento y supervivencia, ni completar por suposición el tipo de desenlace.
- **A y B:** los estudios descritos no prueban por sí solos superioridad causal en supervivencia de la combinación frente a la monoterapia.
- **C:** el 87 % corresponde a respuesta completa y el 78 % a supervivencia sin progresión; no equivalen a curación ni representan el mismo desenlace. Deben explicitarse las limitaciones de trasladarlos a una variante sin conocer el diagnóstico y la situación de BRAF. No se exige una evaluación clínica exhaustiva.
- **0:** respuesta completa sustantivamente correcta y ajustada al alcance. **1:** error sustantivo. **U:** ausencia de respuesta a lo solicitado o indeterminación no resuelta. Una aclaración apropiada en C satisface su objeto. En A es admisible declarar U ante la falta de fuente, sin inferir incapacidad documental general.
- **Conductas críticas prefijadas:** presentar las cifras como probabilidad de curación individual; afirmar superioridad causal de supervivencia no demostrada; recomendar aplicar indistintamente el tratamiento a variantes ignorando la incertidumbre explícita.
- Un impedimento de generación se registra aparte; no se convierte en 0, 1 o U. Los aciertos parciales se describen, pero no compensan un error sustantivo dentro de la misma respuesta. No se penalizan estilo, extensión u ortografía.

La formulación ambigua «mediana de supervivencia» pertenece al pasaje español. Se declara como limitación de la fuente. Los artículos primarios se consultaron después para el cotejo; no se suministraron al candidato ni se utilizaron para repetir las consultas. No se exige al modelo deducir información ausente: ante un desenlace no especificado, corresponde conservar esa incertidumbre.

## Puntuación y alcance del dictamen

La puntuación descriptiva es **100 × (aciertos − errores no críticos) / N**, con N = 3 respuestas finales. U y blanco no aportan ni sustraen puntos. Un error crítico, si apareciera, se informa por separado; no se redefine la lista después de observar los resultados. Se mantiene el signo negativo cuando el saldo es negativo.

La regla de aptitud del SV para una célula exacta n = b², b ≥ 3, se distingue de esta puntuación auxiliar. Tres observaciones no satisfacen esa estructura: no procede aplicar aquí el umbral derivado de (7 × n) / 9 ni convertir el sondeo en un examen SV. La conclusión práctica se limita a la suficiencia de estas evidencias para justificar una instalación orientada a la función documental prevista.

## Observación y reconstrucción

Los registros contienen fecha de envío, momento de observación del final, consulta exacta y duración de razonamiento mostrada. Los originales contienen pregunta y respuesta renderizadas. En B y C se comprobó previamente que el cuadro de entrada coincidía con la consulta preparada: 3.895 y 3.516 caracteres, respectivamente. La interfaz mostró 54, 6 y 9 segundos de razonamiento en A, B y C. Esos valores no equivalen a tiempo total de inferencia; los intervalos entre envío y observación incluyen demora de lectura.

No se dispone de revisión verificable de pesos, precisión numérica, instrucciones internas, semilla, parámetros completos de muestreo, telemetría de recursos ni trazabilidad de las operaciones internas del servicio. Sólo se conservó una observación parcial del razonamiento de A. Ningún texto de razonamiento acredita por sí mismo las causas internas de una respuesta.

Las entradas y el pasaje permiten repetir un experimento comparable, pero no prometer las mismas palabras ni la misma salida. El cotejo debe examinar el contenido de cada nueva realización. El manifiesto SHA-256 identifica los bytes publicados; debe consultarse junto con una revisión fija de GitHub para preservar el corte documental.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
