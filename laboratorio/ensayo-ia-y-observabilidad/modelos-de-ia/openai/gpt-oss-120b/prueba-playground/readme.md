# GPT-OSS-120B: prueba documental en la demostración pública

**Fecha: 3 de octubre de 2026. Identificador: GPTOSS-DEMO-20261003.**

Se examinó la fidelidad de tres respuestas sobre un pasaje del resumen profesional del Instituto Nacional del Cáncer (NCI). Se utilizó la [demostración pública de GPT-OSS](https://gpt-oss.com/), con la selección visible **gpt-oss-120b** y razonamiento **High**. Cada condición se realizó en una conversación nueva y recibió una sola generación.

**Resultado: ninguna respuesta fue íntegramente correcta.** Al aportar el texto documental mejoró la extracción de cifras y se mantuvieron cautelas pertinentes, pero persistieron explicaciones que alteraban el significado de los resultados clínicos. La unidad evaluada fue la respuesta completa, incluidos sus resúmenes.

| Condición | Resultado principal | Terna |
|---|---|:---:|
| A · Pregunta sin el pasaje | Cifras discordantes y atribución bibliográfica no sustentada por el documento indicado. | 1 |
| B · La misma pregunta con el pasaje | Seis magnitudes solicitadas correctas; identificación indebida de una supervivencia no especificada como supervivencia global. | 1 |
| C · Consulta ambigua con el pasaje | Cautelas y aclaraciones pertinentes; confusión entre progresión, reaparición y momento de evaluación de la respuesta. | 1 |

**Balance: 0 aciertos completos, 3 errores no críticos según la lista prefijada, 0 indeterminaciones y 0 blancos. Cobertura: 3/3. Puntuación descriptiva: −100/100**, calculada como 100 × (0 − 3) / 3. El saldo negativo no expresa un porcentaje de conocimiento. Los aciertos parciales y el fundamento de cada error se detallan en [RESULTADOS.md](RESULTADOS.md).

La puntuación y la aptitud son cuestiones distintas. Las tres observaciones relacionadas no constituyen una célula exacta del SV ni permiten aplicar su regla de aptitud. Tampoco acreditan aptitud clínica. La ausencia de las conductas críticas prefijadas no elimina la relevancia de los errores encontrados.

## Alcance para una posible instalación

La demostración aporta evidencia desfavorable para adoptar directamente el modelo como asistente documental fiable. **No justifica por sí sola su incorporación a un examen de aptitud ni una instalación orientada al uso previsto.** Una eventual instalación experimental requeriría un objetivo adicional concreto que justificara qué incertidumbre se pretende resolver.

Esta prueba no mide la viabilidad técnica de instalación, la memoria necesaria ni el rendimiento de una realización propia. La identidad y la configuración se conocen por los rótulos de la interfaz; no se verificaron los pesos, la precisión numérica, las instrucciones internas ni la infraestructura de inferencia. Por tanto, el resultado tampoco demuestra incapacidad universal del modelo o equivalencia con cualquier instalación futura.

## Evidencias y reproducción

| Contenido | Acceso |
|---|---|
| Condiciones, criterios y limitaciones | [METODOLOGIA.md](METODOLOGIA.md) |
| Evaluación razonada y fuentes de contraste | [RESULTADOS.md](RESULTADOS.md) |
| Resultado estructurado | [RESULTADOS.json](RESULTADOS.json) |
| Pasaje exacto suministrado en B y C | [FUENTE.txt](FUENTE.txt) |
| Preguntas exactas y marcas de observación | [A](registros/A.json) · [B](registros/B.json) · [C](registros/C.json) |
| Primeras respuestas completas conservadas | [A](respuestas/A.txt) · [B](respuestas/B.txt) · [C](respuestas/C.txt) |
| Observaciones parciales anteriores al final A | [01](observaciones/A-01.txt) · [02](observaciones/A-02.txt) · [03](observaciones/A-03.txt) |
| Identidad de los archivos | [MANIFIESTO-SHA256.tsv](MANIFIESTO-SHA256.tsv) |

Los originales conservan el texto renderizado, incluida la pregunta, sin corregir las afirmaciones del modelo. Las observaciones parciales no son nuevas generaciones ni una transcripción íntegra del razonamiento. Las huellas permiten comprobar identidad documental; no certifican verdad científica. La metodología explica qué puede reconstruirse y qué queda fuera de observación.

El pasaje del NCI y los originales del modelo se conservan como materiales de evaluación; su publicación no constituye una recomendación clínica. Las fuentes de terceros conservan sus derechos y condiciones de uso.

## Autoría, licencia y alcance

El régimen de autoría y licencia indicado a continuación se aplica al trabajo propio contenido en esta carpeta y a todos los ensayos, documentos y subcarpetas que se incorporen a ella, incluido `ensayo-1`. Comprende su diseño experimental, documentación, análisis y presentación de resultados. La atribución corresponde al titular identificado; el Sistema Vectorial SV no sustituye a dicho titular.

Las fuentes, los componentes y los demás materiales de terceros conservan sus derechos y licencias. Su incorporación como evidencia no supone atribuir su autoría al titular de este trabajo ni modificar sus condiciones de uso.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

