# Informe comparativo de los cuatro proveedores de IA

**Edición 1.4 · 8 de octubre de 2026.** Comparación de OpenAI/GPT-6 Astra, xAI/Grok 4.7, Alibaba Cloud/Qwen3.8-Max-0902 y Z.ai/GLM-5.3 mediante los ensayos realizados por el SV. Se presentan calidad documental, tokens, duración y coste, conservando sus unidades y procedencia.

## Clasificación propia: mismas preguntas y misma etapa

R0 es la respuesta inicial; R1, la autocrítica; R2, la entrega final. Se reproduce la conformidad adjudicada en los ensayos del SV. Los puestos se ordenan por número de respuestas conformes dentro de cada prueba; los empates se conservan. Los errores críticos y las reservas se presentan aparte y prevalecen para la admisión.

| Prueba común | Modelo | Conformes R0 → R1 → R2 | Incumplimientos críticos R0 → R1 → R2 | Puesto inicial | Puesto final |
|---|---|---|---|---:|---:|
| PDQ, 25 preguntas | GPT-6 Astra | 25 → 25 → 25 | 0 → 0 → 0 | **1** | **1, compartido** |
| PDQ, 25 preguntas | Grok 4.7 | 24 → 25 → 25 | 1 → 0 → 0 | **2** | **1, compartido** |
| PDQ, primeras 16¹ | GPT-6 Astra | 16 → 16 → 16 | 0 → 0 → 0 | **1** | **1, compartido** |
| PDQ, primeras 16¹ | Grok 4.7 | 15 → 16 → 16 | 1 → 0 → 0 | **2** | **1, compartido** |
| PDQ, primeras 16¹ | Qwen3.8-Max-0902 | 13 → 12 → 12 | 2 → 3 → 3 | **3¹** | **3¹** |
| CYB16, 16 preguntas | GPT-6 Astra | 16 → 16 → 16 | 0 → 0 → 0 | **1** | **1, compartido** |
| CYB16, 16 preguntas | GLM-5.3 | 11 → 15 → 16 | 5 → 1 → 0 | **2** | **1, compartido** |

¹ El tramo PDQ de dieciséis preguntas extrae P01–P16 de Astra y Grok y utiliza el examen efectivo de Qwen. Es un subconjunto de los ensayos anteriores, no otras ejecuciones. **La posición de Qwen es de conformidad contractual y está bajo reserva metodológica:** sus incumplimientos incluyen marcación literal y la interpretación de P13; no equivalen a cuatro falsedades médicas ni acreditan inferioridad clínica. Su réplica posterior de P13 no sustituye el vector histórico.

**Lectura del ranquin:** Astra obtiene la mejor respuesta inicial en las pruebas comunes disponibles. Grok iguala a Astra en la entrega final del examen PDQ; GLM lo iguala en la entrega final de CYB16. Qwen conserva el dictamen contractual desfavorable con la reserva indicada. No se construye un puesto global entre los cuatro mezclando pruebas de temas diferentes.

Fuentes de adjudicación: [Astra PDQ](https://github.com/juantoniolloretegea/SV-motor/blob/3370da26c1d19ebab9e742c4a0f085910c5b1bd4/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/examen25-20261007/resultado/INFORME.md), [Grok PDQ](https://github.com/juantoniolloretegea/SV-motor/blob/3370da26c1d19ebab9e742c4a0f085910c5b1bd4/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/xai/grok-4.7/examen25-20261008/INFORME.md), [Qwen PDQ](https://github.com/juantoniolloretegea/SV-motor/blob/3370da26c1d19ebab9e742c4a0f085910c5b1bd4/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/qwen/qwen3.8-max-0902/examen25-20261008/INFORME.md), [Astra CYB16](https://github.com/juantoniolloretegea/SV-motor/blob/3370da26c1d19ebab9e742c4a0f085910c5b1bd4/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/cyb16-20261008/INFORME.md) y [GLM CYB16](https://github.com/juantoniolloretegea/SV-motor/blob/3370da26c1d19ebab9e742c4a0f085910c5b1bd4/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/zai/glm-5.3/cyb16-20261008/INFORME.md). [Vectores y clasificación descargables](RANQUIN-CALIDAD-SV.json).

## Tokens, tiempos e importes de los exámenes

Se contabilizan las tres etapas y todos los intentos científicos identificados. Entrada y salida son contadores comunicados por el proveedor. Los tiempos son mediciones del cliente del SV.

| Proveedor · modelo | Examen | Entregas / intentos | Entrada | Salida | Total conocido | Suma de operaciones² | Importe del examen, USD |
|---|---|---:|---:|---:|---:|---:|---|
| OpenAI · GPT-6 Astra | PDQ25 | 75 / 77 | 807.043 | 35.655 | **842.698** | **1.804,348 s** | No comunicado |
| xAI · Grok 4.7 | PDQ25 | 75 / 75 | 1.030.606 | 243.081 | **1.273.687** | **3.712,666 s** | **3,386642**, comunicado |
| Alibaba Cloud · Qwen3.8-Max-0902 | PDQ16 | 48 / 48 | 608.113 | 79.805 | **687.918** | **2.140,166 s** | No comunicado |
| OpenAI · GPT-6 Astra | CYB16 | 48 / 49 | 1.056.285 | 83.315 | **1.139.600** | **2.595,289 s** | No comunicado |
| Z.ai · GLM-5.3 | CYB16 | 48 / 48 | 1.226.311 | 157.944 | **1.384.255** | **2.392,304 s** | **2,35145108**, estimado |

² Suma de la duración observada de las operaciones, incluidos los intentos incompletos. No es tiempo de calendario ni velocidad interna de generación. Para Astra PDQ se usa `duracion_observada_ms`, más las dos interrupciones; para los otros ensayos, `duracion_operacion_ms`. La preparación y cierre incluidos difieren entre instrumentos: el orden temporal es descriptivo de estas ejecuciones, no una comparación controlada de latencia.

Los dos intentos incompletos de Astra PDQ y el de Astra CYB16 carecen de contadores completos: el total conocido no incorpora una estimación de ellos. Caché y razonamiento ya están incluidos en entrada y salida. **No comunicado significa desconocido, no cero.**

En PDQ25, Astra registró menor duración acumulada que Grok. En CYB16, GLM registró menor duración acumulada que Astra: 2.392,304 frente a 2.595,289 segundos, incluyendo la interrupción de Astra. Sobre las 48 entregas completas, las medianas inferiores fueron 41,468 s para GLM y 48,682 s para Astra; el percentil 95 fue 112,952 y 77,727 s, respectivamente. Una mediana menor no implica una cola de demoras mejor.

El importe de Grok procede de los costes de sus 75 solicitudes científicas; no se confunde con los 3,59913 USD de todos sus registros. La estimación de GLM aplica las tarifas documentadas del 08/10/2026 a las 48 solicitudes. No se dispone de un gasto monetario comparable de los cuatro para la misma prueba, por lo que no se atribuye un ganador económico experimental conjunto.

[Detalle de 297 intentos por modelo, prueba, pregunta y etapa](evidencia-sv/ENSAYOS-POR-SOLICITUD.json) · [Resumen reproducible](evidencia-sv/ENSAYOS-RESUMEN.json). Cada fila conserva la fuente científica y los datos desconocidos.

## Clasificación comercial: igual volumen de tokens

Este cálculo aplica las tarifas documentadas el 08/10/2026 a **1.000.000 de tokens de entrada ordinaria y 100.000 de salida, sin caché**. Es una referencia tarifaria calculada; no un gasto medido ni un encargo resuelto por los modelos.

| Puesto por menor precio | Proveedor · modelo | Entrada, USD/M | Salida, USD/M | Coste del volumen convencional |
|---:|---|---:|---:|---:|
| **1** | Z.ai · GLM-5.3 | 1,40 | 4,40 | **1,84 USD** |
| **2, compartido** | xAI · Grok 4.7 | 2,00 | 6,00 | **2,60 USD** |
| **2, compartido** | Alibaba Cloud · Qwen3.8-Max-0902 | 2,00 | 6,00 | **2,60 USD** |
| **4** | OpenAI · GPT-6 Astra | 10,00 | 50,00 | **15,00 USD** |

Fórmula: entrada × precio de entrada / 1.000.000 + salida × precio de salida / 1.000.000. Los segmentadores difieren: un volumen idéntico de tokens no garantiza idéntico contenido. Se conserva el supuesto de servicio de texto ordinario y contexto corto, sin impuestos, herramientas ni otras modalidades.

Fuentes tarifarias: [Z.ai](https://docs.z.ai/guides/overview/pricing), [xAI](https://docs.x.ai/developers/models/grok-4.7), [Alibaba Cloud](https://www.alibabacloud.com/help/en/model-studio/model-pricing) y [OpenAI](https://developers.openai.com/api/docs/models/gpt-6-astra). Son las referencias fechadas del informe; no se presentan como nuevas lecturas en tiempo real.

## Referencia externa: Artificial Analysis

Estos resultados proceden de **Artificial Analysis Intelligence Index v4.3.2**, corte documental del 08/10/2026. Se mantienen separados de las mediciones del SV.

| Puesto económico externo | Modelo | Configuración de la fuente | USD por tarea evaluada | Índice AA |
|---:|---|---|---:|---:|
| 1 | GLM-5.3 | max | 2,01 | 45 |
| 2 | GPT-6 Astra | max | 3,26 | 53 |
| 3 | Grok 4.7 | xhigh | 3,74 | 46 |
| 4 | Qwen3.8-Max-0902 | 0902 con razonamiento | 5,41 | 45 |

Es coste por tarea evaluada, no por respuesta correcta. El índice AA no es una tasa de acierto ni una puntuación del SV. Las configuraciones externas no equivalen a las de nuestros ensayos.

Fuentes: [GLM](https://artificialanalysis.ai/models/glm-5-3), [Astra](https://artificialanalysis.ai/models/gpt-6-astra), [Grok](https://artificialanalysis.ai/models/grok-4-7), [Qwen](https://artificialanalysis.ai/models/qwen3-8-max) y [método](https://artificialanalysis.ai/methodology). [Datos externos conservados](DATOS.json).

![Coste e índice de Artificial Analysis, referencia externa](COSTE-Y-CAPACIDAD.svg)

## Alcance de las conclusiones

Los resultados describen las ejecuciones conservadas. La evaluación documental es exterior al candidato y asistida por IA; la recepción científica independiente continúa pendiente. No hay repeticiones suficientes para inferir estabilidad estadística o superioridad general. El coste y la duración no compensan incumplimientos críticos.

La comparación PDQ16 conserva la identidad de banco, catálogo, fuente y suministros de P01–P16; expresa por separado la reserva de aplicación formal en Qwen. CYB16 conserva identidad de preguntas, corpus y criterios entre Astra y GLM. MD01 y las réplicas diagnósticas siguen disponibles como antecedentes en [resultados complementarios](RESULTADOS-SV.md).

[Resumen y clasificaciones](readme.md) · [Método](METODO.md) · [Datos por solicitud y reproducción](evidencia-sv/PROCEDENCIA.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
