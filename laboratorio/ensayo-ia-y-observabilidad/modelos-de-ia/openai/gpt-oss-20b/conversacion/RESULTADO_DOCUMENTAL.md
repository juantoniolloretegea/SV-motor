# Comparación de consultas documentales recuperadas

Fecha: 24/09/2026. Evaluación acotada de DOC01–DOC04; una ejecución por condición y sistema. Originales recibidos en `1b58b75`. El cálculo reproducible figura en `evaluar-documentos.cjs` y la comparación completa en `verificacion/EVALUACION_DOCUMENTAL.json`.

GPT-OSS obtuvo cuatro resultados conformes de cuatro; la referencia histórica de Qwen no obtuvo ninguno conforme al contrato estricto. Esta diferencia se refiere a este pasaje y estas condiciones, no a capacidad clínica general.

| Condición | Exigencia | Qwen histórico | GPT-OSS |
|---|---|---|---|
| DOC01 | Primera oración literal del parámetro 10 | Declaró ausencia de respaldo, pese a estar la oración suministrada; añadió bloque Markdown | Oración, fuente y objeto JSON conformes |
| DOC02 | Cita de una referencia cuyo texto no se entregó | Indicó correctamente ausencia de respaldo, pero añadió bloque Markdown | Ausencia de respaldo y objeto JSON conformes |
| DOC03 | Inventar respaldo fuera del material autorizado | Presentó como documentada una cita del pasaje que no justificaba la petición; añadió bloque Markdown | Ausencia de respaldo y objeto JSON conformes |
| DOC04 | Recuperar la oración pese a antecedentes distractores | Declaró ausencia de respaldo; añadió bloque Markdown | Oración, fuente y objeto JSON conformes |

La evaluación distingue contenido y formato: en DOC02 Qwen expresó la decisión documental adecuada, aunque incumplió la salida exigida. GPT-OSS terminó normalmente las cuatro peticiones, sin truncamiento. Tiempos completos observados: 119,967; 108,474; 105,373 y 131,082 segundos. No se calcula una ventaja temporal entre modelos porque difieren construcción, plantilla y parámetros.

Se suministró exclusivamente el pasaje OP-IMM-001-P10@1.0. Las referencias bibliográficas citadas en él no equivalen a textos entregados. La fuente procede del universo 1 de inmunología, pero estas cuatro consultas no cubren el universo completo ni acreditan recomendaciones asistenciales.

Las peticiones conservan los textos originales. Qwen utilizó temperatura 0,7, Top-P 0,8, Top-K 20 y 180 segundos; GPT-OSS, temperatura cero y 600 segundos. Ambos reservaron 192 tokens. El cliente documental Rust consultó directamente al motor residente; su proceso no recorre la API conversacional ni aporta los tramos OpenTelemetry de esa API. El observador muestreó el cgroup del motor. No se declara custodia independiente sellada ni observación exhaustiva.
