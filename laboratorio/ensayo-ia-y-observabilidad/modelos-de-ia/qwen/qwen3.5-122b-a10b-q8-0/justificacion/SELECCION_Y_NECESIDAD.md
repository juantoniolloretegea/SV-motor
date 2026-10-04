# Justificación de selección y necesidad documental

**Versión 1.0 · 4 de octubre de 2026**

## 1. Necesidad que motiva la selección

La función buscada es interpretar conocimiento documental delimitado sin introducir hechos ajenos, conservar las condiciones de las fuentes y aplicar una política expresa de decisión. La respuesta debe vincular sus afirmaciones con evidencia localizable, reconocer información insuficiente o contradictoria y cumplir todas las obligaciones de salida.

El corpus profesional de tricoleucemia constituye un caso acotado de comprobación. No agota el Universo 1 de inmunología ni permite extrapolar resultados a otros dominios. El objetivo arquitectónico más amplio es una IA auxiliar gobernada por el Lenguaje SV, con separación entre propuesta probabilística, validación y autoridad. [Guía del sistema conjunto, revisión 2](https://github.com/juantoniolloretegea/SV-motor/blob/7e2bcb2ac9d5b7803c8dba76df91757fa749c458/laboratorio/sistema-conjunto-lenguaje-computacion-ia-gobernada/README.md).

La selección de Qwen responde a tres razones: mejores puntuaciones publicadas en varios indicadores de instrucciones y contexto; una distribución numérica susceptible de estudio con los recursos previstos; y necesidad de contrastar un candidato diferente ante incumplimientos documentados. Ninguna de ellas demuestra por adelantado conformidad.

## 2. Dos comparaciones diferentes

| Objeto | Evidencia utilizada | Interpretación admisible |
|---|---|---|
| GPT-OSS-120B general frente a Qwen3.5-122B-A10B | Evaluaciones externas publicadas por Qwen | Orienta la selección; no mide nuestra instalación. |
| GPT-OSS-Safeguard-120B en el procedimiento documental SV | Informes A0 y A2, originales y política publicados | Describe incumplimientos de una configuración y un banco concretos. |

Safeguard está especializado en clasificar contenido conforme a políticas suministradas. Esa especialización no acredita automáticamente fidelidad a documentos, corrección profesional ni equivalencia con el modelo general. [Guía oficial de Safeguard](https://developers.openai.com/cookbook/articles/gpt-oss-safeguard-guide).

## 3. Qué no quedó resuelto en los antecedentes recibidos

La referencia inicial A0 y las revisiones A1–A2 conservaron **ocho clasificaciones documentales correctas de nueve**, pero ninguna respuesta plenamente conforme. Persistieron un error sustantivo crítico y omisiones formales. No debe confundirse la proporción 8/9 con la puntuación integral o el dictamen. [A0](https://github.com/juantoniolloretegea/SV-motor/blob/7e2bcb2ac9d5b7803c8dba76df91757fa749c458/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-safeguard-120b/ensayos-reiterados-adversariales-y-aprendizaje/resultados/A0/INFORME-A0.md) · [A2](https://github.com/juantoniolloretegea/SV-motor/blob/7e2bcb2ac9d5b7803c8dba76df91757fa749c458/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-safeguard-120b/ensayos-reiterados-adversariales-y-aprendizaje/resultados/A2/INFORME-A2.md).

| Necesidad | Observación documentada | Hipótesis que deberá contrastarse con Qwen |
|---|---|---|
| Resolver conflictos conforme a la política | A08 reconoce fuentes discrepantes sin precedencia, pero emite CONTRADICHA donde se exige EVIDENCIA_INSUFICIENTE. | Aplicación de la regla explícita sin convertir indecidibilidad documental en una conclusión de falsedad. |
| Cumplimiento completo de la respuesta | A0–A2 omiten la recepción documental; A1–A2 omiten además la revisión exigida. | Inclusión de todos los campos necesarios, con contenido sustantivo verificable. |
| Revisión de una respuesta previa | A0–A2 no muestran mejora en el resultado integral. | Corrección fundamentada cuando proceda y mantenimiento justificado cuando la respuesta ya sea correcta. |
| Fidelidad al corpus | Se requiere preservar negaciones, cantidades, alcance, excepciones, causalidad e incertidumbre. | Interpretación ceñida a las fuentes y citas literales cuando se soliciten. |
| Viabilidad temporal | La calidad de una respuesta no determina por sí sola su utilidad práctica. | Demora y memoria medidas bajo límites prefijados; no se presume una aceleración. |

El informe A2 adjudica **No apto para esa revisión**, con una puntuación de −88,89/100 conforme a su regla específica. No es una medida de capacidad general de la familia GPT ni un resultado transferible a otra configuración. Este expediente utiliza A0–A2 como antecedentes cerrados y publicados; no adjudica diagnósticos posteriores todavía pendientes de recepción documental en esta selección.

## 4. Reservas que impiden una conclusión simplista

El propio informe A2 identifica incoherencia de presentación entre un formato inicial de cuatro campos y exigencias adicionales al final. También señala la distinción entre una autorización y la posibilidad de concluir esa autorización. Ambas reservas limitan la interpretación causal del fallo; no se ha demostrado que expliquen por sí solas las salidas.

La recepción efectiva de documentos y la integridad de archivos tampoco acreditan comprensión ni equivalencia numérica entre motores. El procedimiento de revisiones conserva antecedentes en el contexto; no entrena los pesos ni entrega una corrección externa. [Condiciones y reservas de A2](https://github.com/juantoniolloretegea/SV-motor/blob/7e2bcb2ac9d5b7803c8dba76df91757fa749c458/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-safeguard-120b/ensayos-reiterados-adversariales-y-aprendizaje/resultados/A2/INFORME-A2.md).

Antes de evaluar Qwen debe formularse una política coherente, con una única definición de campos y ejemplos compatibles. Si se modifica el contrato o el banco, el nuevo ensayo tendrá identidad propia y no se presentará como una comparación en condiciones idénticas. Los resultados previos permanecen intactos.

## 5. Qué se espera y qué no puede prometerse

Se espera comprobar una mejor combinación de cumplimiento, fidelidad documental, manejo de insuficiencia y tiempo de respuesta. Se trata de una expectativa experimental. Las ventajas publicadas de Qwen permiten priorizarlo, pero no demuestran la resolución de A08, ausencia de alucinaciones, aptitud clínica o conformidad general con SV.

La categoría EVIDENCIA_INSUFICIENTE puede ser la respuesta correcta y recibir una adjudicación favorable. **No equivale automáticamente a U.** La terna (0,1,U) pertenece a la evaluación constituida; los errores de transmisión y las preguntas no ejecutadas conservan su naturaleza instrumental.

Las posiciones de una célula SV deben tener significado definido. No se añaden parámetros vacíos para acomodar una respuesta, y la célula mínima de nueve posiciones no es una matriz. La elección de dimensiones o de células combinadas corresponde al dominio y su contrato, no al candidato. [Marco de representación](https://github.com/juantoniolloretegea/SV-motor/blob/7e2bcb2ac9d5b7803c8dba76df91757fa749c458/laboratorio/sistema-conjunto-lenguaje-computacion-ia-gobernada/README.md).

## 6. Resultado exigible

La selección sólo podrá convertirse en recepción favorable cuando existan una realización identificada, respuestas originales conservadas y adjudicación independiente bajo condiciones prefijadas. Una eventual conclusión negativa deberá describir el incumplimiento y su alcance. Las necesidades nuevas para el Lenguaje se registrarán separadamente, sin modificar automáticamente el Núcleo.

[Volver al expediente](../readme.md) · [Comparativa](../comparativa/COMPARATIVA_GPT_OSS.md) · [Recepción](../evaluacion/CRITERIOS_DE_RECEPCION.md).
