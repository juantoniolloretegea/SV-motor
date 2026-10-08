# Resultados del SV y comparación de costes de modelos de IA

**Versión 1.2 · Corte documental: 8 de octubre de 2026.** Esta edición presenta una clasificación propia de calidad, los consumos y observaciones del SV y la referencia económica externa. Los resultados externos quedan identificados como tales. La calidad, la demora y el coste son magnitudes separadas.

## Ranquin propio de calidad: MD01, respuesta inicial R0

Dos comprobaciones comunes están documentadas: **contenido correcto y completo** y **estructura conforme**. Ambas forman parte del dictamen conjunto existente. Su resultado permite esta clasificación propia:

| Puesto en conformidad MD01-R0 | Modelo | Contenido adjudicado | Estructura | Dictamen conjunto |
|---:|---|---|---|---|
| **1, compartido** | GPT-6 Astra | 0: correcto y completo | Conforme | 0 |
| **1, compartido** | Grok 4.7 | 0: correcto y completo | Conforme | 0 |

**El empate es un resultado, no una clasificación pendiente.** Se limita a esta pregunta y etapa, con corpus y criterios sustantivos comunes. Corrección y completitud no se cuentan como dos preguntas ni dos puntos independientes; las columnas tampoco se suman. Se conserva la revisión contextual conforme en ambos.

Fuentes: [adjudicación de Astra](https://github.com/juantoniolloretegea/SV-motor/blob/0246a7df1e9436f4bff4f0d94ac6d36e1f29c1e0/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/replica-md01-20261008/hitos/INTENTO-001/ADJUDICACION.json) y [adjudicación de Grok](https://github.com/juantoniolloretegea/SV-motor/blob/091b50a884c893fd4322bbf211b0feb0132bb258/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/xai/grok-4.7/md01-diagnostica-20261008/adjudicacion/MD01-R0.json). [Clasificación propia descargable](RANQUIN-CALIDAD-SV.json).

La precisión terminológica señalada en Grok permanece conservada, sin convertirla retrospectivamente en un error para romper el empate. La recepción científica independiente sigue pendiente. Esta clasificación no acredita igualdad general entre modelos ni admisión del contrato completo. Las diferencias de reloj afectan a la comparación temporal, no al valor de las adjudicaciones sustantivas conservadas.

## Comparación Astra–GLM mediante CYB16

La misma prueba de dieciséis preguntas permite una **clasificación propia de calidad para ese contrato**: mismas fuentes, preguntas, posiciones críticas, criterios y etapas R0/R1/R2. La comparación final utiliza R2 de ambos; R0 y R1 muestran por separado la calidad inicial y la evolución. Los intentos repetidos e interrupciones permanecen visibles.

El resultado final podrá ser una diferencia o un empate. Si ambos alcanzan dieciséis respuestas correctas, sin errores críticos y con conformidad documental, compartirán puesto en calidad final. No se inventará un desempate. La fiabilidad de entrega y el tiempo necesario para completar el mismo encargo podrán generar clasificaciones separadas si sus definiciones y observaciones son comparables.

No es necesario que ambos proveedores utilicen el mismo segmentador, infraestructura o formato de API. Sí deben conservarse los criterios de evaluación y declararse las adaptaciones, configuraciones y límites efectivos. Una interrupción sin respuesta completa no se transforma en error de contenido; tampoco se oculta al comparar entrega, tiempo o consumo.

La clasificación de calidad no queda bloqueada por falta de costes monetarios. Para ordenar por coste deben existir importes atribuibles y comparables; una valoración mediante tarifas publicadas se identificaría como estimación, no como gasto liquidado. La información de una sola campaña permite describir ese ensayo, sin acreditar estabilidad general.

## Observaciones propias: demora registrada en MD01-R0

Se presenta la **demora hasta el primer evento de texto registrada por el cliente** en las dos ejecuciones de la misma pregunta MD01, con el mismo corpus y contrato sustantivo.

| Modelo | Demora registrada | Resultado de R0 |
|---|---:|---|
| [GPT-6 Astra](gpt-6-astra/readme.md) | **5,579 s** | Correcta y completa |
| [Grok 4.7](grok-4.7/readme.md) | **64,239 s** | Correcta y completa |

**Estas observaciones no establecen puestos ni una comparación controlada de latencia.** Hay una ejecución por modelo; las solicitudes tienen adaptaciones diferentes y el reloj de Grok incluye preparación local adicional. El primer evento de texto puede contener un fragmento vacío. Estas limitaciones no se han corregido mediante supuestos.

El [cotejo MD01](COTEJO-MD01.md) publica las huellas, configuraciones y diferencias instrumentales. Los [datos propios](DATOS-SV.json), las [observaciones en JSON](OBSERVACIONES-MD01.json) y su [tabla CSV](OBSERVACIONES-MD01.csv) permiten comprobar estas cifras. R1 se conserva como complemento; R2 de Grok no fue enviado. Qwen y GLM tienen aquí ensayos de otro contenido y no se incorporan artificialmente a MD01.

## Consumos y resultados propios de los cuatro modelos

Se publican **363 registros** de las ejecuciones del SV, con **5.872.866 tokens conocidos**. Son contadores comunicados por los proveedores: tres registros carecen de contadores completos y un registro puede reunir varios intentos. La suma no representa una cantidad uniforme de lenguaje ni 363 tareas comparables.

- [Resultados por ensayo, tiempos, dictámenes y alcance económico](RESULTADOS-SV.md).
- [Datos de consumo por registro](evidencia-sv/CONSUMOS-POR-REGISTRO.csv) y [agregados por modelo](evidencia-sv/CONSUMOS-POR-MODELO.json).
- [Las 48 solicitudes de CYB16](evidencia-sv/CYB16-POR-SOLICITUD.csv), con contadores, tiempos y cálculo de **2,35145108 USD estimados**.
- [Procedencia, cobertura de la comprobación y reproducción pública en Rust](evidencia-sv/PROCEDENCIA.md).

**Ranquin económico propio de los cuatro modelos: no establecido.** Los registros reúnen encargos diferentes y no existe un coste monetario atribuible y comparable para todos. Los 3,59913 USD comunicados en 78 registros de Grok y la estimación de CYB16 no pueden ordenarse como si correspondieran al mismo trabajo. Un importe desconocido conserva esa condición.

## Ranquin económico externo: Artificial Analysis

Este orden utiliza el **coste medio ponderado por tarea del Artificial Analysis Intelligence Index v4.3.2**. No procede de las ejecuciones del SV. Es coste por tarea evaluada, no por respuesta correcta.

| Puesto externo | Modelo | Configuración externa | USD por tarea AA | Índice AA |
|---:|---|---|---:|---:|
| **1** | [Z.ai · GLM-5.3](glm-5.3/readme.md) | max | **2,01** | 45 |
| **2** | [OpenAI · GPT-6 Astra](gpt-6-astra/readme.md) | max | **3,26** | 53 |
| **3** | [xAI · Grok 4.7](grok-4.7/readme.md) | xhigh | **3,74** | 46 |
| **4** | [Alibaba Cloud · Qwen3.8-Max-0902](qwen3.8-max-0902/readme.md) | 0902, con razonamiento | **5,41** | 45 |

El índice no es un porcentaje de aciertos. Las etiquetas de esfuerzo no acreditan el mismo cómputo entre proveedores; Qwen no tiene una etiqueta equivalente en su ficha. Fuentes: [GLM](https://artificialanalysis.ai/models/glm-5-3), [Astra](https://artificialanalysis.ai/models/gpt-6-astra), [Grok](https://artificialanalysis.ai/models/grok-4-7), [Qwen](https://artificialanalysis.ai/models/qwen3-8-max) y [metodología](https://artificialanalysis.ai/methodology). Se conserva el corte de la edición anterior, sin presentarlo como una nueva medición.

![Artificial Analysis: coste por tarea e índice de capacidad externos](COSTE-Y-CAPACIDAD.svg)

## Documentación y reproducción

- [Informe: resultados propios, tarifas y evaluaciones externas](INFORME.md).
- [Método y clases de evidencia](METODO.md).
- [Datos externos](DATOS.json), [ranquin externo JSON](RANQUIN.json) y [CSV](RANQUIN.csv).
- [Reproducción en Rust](calculo-rust/REPRODUCCION.md), [manifiesto](MANIFIESTO.json) e [historial de revisiones](HISTORIAL.md).

Los datos necesarios para estas tablas se encuentran en esta carpeta pública. Se han excluido datos de cuentas, medios de pago y secretos. Los antecedentes científicos permanecen enlazados a revisiones públicas fijas.

La admisión a un contrato del SV exige sus criterios de calidad: un error crítico no se compensa con menor coste o demora. Esta revisión no recalifica los ensayos ni acredita una aptitud general.

[Volver al nodo 03](../readme.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
