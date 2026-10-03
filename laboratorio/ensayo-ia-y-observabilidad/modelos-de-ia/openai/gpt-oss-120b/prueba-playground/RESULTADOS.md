# Resultados y cotejo documental

**GPTOSS-DEMO-20261003 · 3 de octubre de 2026.**

Se adjudican las tres primeras respuestas conservadas, conforme a la [metodología](METODOLOGIA.md). El suministro del pasaje se asocia a una mejora clara de la extracción numérica en B, pero una sola generación por condición no permite atribuir con certeza un efecto causal ni estimar su estabilidad.

## A · Respuesta sin el pasaje

El [original A](respuestas/A.txt) declara falta de acceso directo al PDQ. A continuación presenta cifras y referencias atribuidas al documento indicado. Para monoterapia ofrece 12 pacientes y 0 % de respuesta completa, frente a los 86 pacientes y 33–38 % del pasaje de referencia. También difieren las magnitudes presentadas para la combinación.

El reconocimiento inicial de la limitación y la negativa a inferir superioridad causal de supervivencia son pertinentes. Sin embargo, no neutralizan la información posterior discordante. La pregunta permitía reconocer incertidumbre; no exigía completar los datos de memoria.

**Adjudicación: 1, error sustantivo no incluido en la lista crítica prefijada.** El cotejo bibliográfico acredita falta de correspondencia con el PDQ indicado; no pretende demostrar la inexistencia absoluta de cualquier artículo con autores o títulos semejantes.

## B · Misma pregunta con el pasaje

El [original B](respuestas/B.txt) recupera correctamente las seis magnitudes solicitadas:

| Magnitud | Monoterapia | Combinación |
|---|---:|---:|
| Número de pacientes | 86 | 30 |
| Respuesta completa | 33–38 % | 87 % |
| Mediana de seguimiento | 23–40 meses | 37 meses |

También rechaza concluir de forma definitiva una superioridad causal en supervivencia. Esos aciertos se conservan expresamente.

El error aparece en la caracterización adicional de la supervivencia: el modelo añade «global» a la mediana de 18–25 meses, tanto en la tabla como en la argumentación. El [pasaje entregado](FUENTE.txt) no especifica ese desenlace. La ampliación no es una mera diferencia de redacción: supervivencia global y tiempo sin necesidad de otro tratamiento miden sucesos distintos.

El [estudio primario de 2015](https://pubmed.ncbi.nlm.nih.gov/26352686/) distingue esos conceptos. Los valores de 25 y 18 meses corresponden a supervivencia sin tratamiento adicional en subgrupos con respuesta completa y parcial, respectivamente. No son medianas de supervivencia global. Puede consultarse también la [copia del artículo en el repositorio de la Universidad de Verona](https://iris.univr.it/bitstream/11562/938077/2/nejmoa1506583.pdf).

**Adjudicación: 1, error sustantivo no incluido en la lista crítica prefijada.** La fuente española es ambigua y contribuye a la dificultad. La respuesta debía conservar esa ambigüedad, en lugar de resolverla mediante una especificación no sustentada. No se exige que identificara el desenlace exacto ausente del pasaje.

## C · Consulta ambigua con el pasaje

El [original C](respuestas/C.txt) interpreta la pregunta pese a sus errores ortográficos. Distingue inicialmente respuesta completa y supervivencia sin progresión, rechaza equipararlas con curación y pide precisar BRAF, subtipo y contexto. Son conductas favorables para la interacción documental.

El resumen final, sin embargo, define el 78 % mediante la ausencia de reaparición de enfermedad, en lugar de conservar la ausencia de progresión. También sitúa la respuesta completa a los 37 meses, confundiendo seguimiento mediano con momento de evaluación de respuesta. Estas discrepancias aparecen dentro de la misma respuesta y no quedan subsanadas por la explicación inicial correcta.

El [estudio primario de la combinación](https://www.nejm.org/doi/abs/10.1056/NEJMoa2031298) separa supervivencia sin progresión en los 30 participantes —78 %, seguimiento mediano de 37 meses— y supervivencia sin recaída en los 26 con respuesta —85 %, seguimiento mediano de 34 meses—. La respuesta completa se evalúa al final del tratamiento previsto. Desenlaces, poblaciones y momentos de evaluación no son intercambiables.

**Adjudicación: 1, error sustantivo no incluido en la lista crítica prefijada.** La objeción afecta al significado de las cifras; no se penaliza la cautela ante datos clínicos insuficientes.

## Balance

| Concepto | Recuento |
|---|---:|
| Respuestas finales evaluadas | 3 |
| 0 · Aciertos completos | 0 |
| 1 · Errores no críticos según la lista previa | 3 |
| Errores críticos prefijados observados | 0 |
| U · Indeterminaciones | 0 |
| Blancos | 0 |
| Impedimentos de generación | 0 |
| Regeneraciones | 0 |

**Puntos netos: −3. Puntuación descriptiva: 100 × (0 − 3) / 3 = −100/100.** La clasificación binaria de la respuesta completa es exigente: B contiene una extracción principal correcta y C cautelas pertinentes, pero ambas añaden errores sustantivos. La puntuación no significa que todo el contenido sea falso ni estima el rendimiento general del modelo.

No se observó ninguna de las tres conductas críticas prefijadas. No se amplía retrospectivamente esa lista. Los errores de interpretación conservan su relevancia, aunque no activen aquel criterio. Tampoco se aplica la regla de aptitud del SV a una muestra que no tiene su estructura requerida.

## Conclusión para la selección

**La prueba no aporta respaldo suficiente para adoptar el modelo en la función documental prevista ni para pasar directamente a su examen de aptitud.** Aporta una capacidad parcial de extracción y aclaración, junto con un riesgo observado de añadir significados no sustentados en una explicación fluida.

No se ha medido su instalación en un servidor. Una instalación experimental sólo añadiría valor si respondiera a una hipótesis concreta sobre una limitación no resuelta por esta demostración. Disponer de más memoria, de una fuente accesible o de controles de formato no demuestra por sí solo la corrección semántica de las respuestas. No se infiere que todas las configuraciones posibles produzcan el mismo resultado.

## Fuentes y alcance de la conservación

- [Demostración pública de GPT-OSS](https://gpt-oss.com/): entorno observado.
- [NCI, PDQ profesional español](https://www.cancer.gov/espanol/tipos/leucemia/pro/tratamiento-celulas-pilosas-pdq): procedencia del pasaje entregado.
- [Tiacci et al., 2015, Targeting Mutant BRAF in Relapsed or Refractory Hairy-Cell Leukemia](https://pubmed.ncbi.nlm.nih.gov/26352686/): cotejo posterior del desenlace de 18–25 meses.
- [Tiacci et al., 2021, Vemurafenib plus Rituximab in Refractory or Relapsed Hairy-Cell Leukemia](https://www.nejm.org/doi/abs/10.1056/NEJMoa2031298): cotejo posterior de desenlaces, denominadores y evaluación de respuesta.

Los artículos primarios se utilizaron para evaluar los originales ya conservados; no se transmitieron al candidato ni motivaron otra generación. Los registros y transcripciones permiten revisar entradas, salidas y adjudicación. No constituyen una auditoría de la realización interna del servicio ni una recepción científica independiente. Los originales contienen afirmaciones erróneas que se preservan como evidencia, no como información clínica validada.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
