# OpenAI · GPT-6 Astra

**Edición 1.4 · 08/10/2026.**

## Resultados medidos por el SV

PDQ25: 25/25 conformes en R0, R1 y R2; primer puesto inicial y empate final con Grok. CYB16: 16/16 en las tres etapas; primer puesto inicial y empate final con GLM.

PDQ25: 842.698 tokens conocidos, 75 entregas en 77 intentos y 1.804,348 s de operaciones observadas. CYB16: 1.139.600 tokens conocidos, 48 entregas en 49 intentos y 2.595,289 s. Los intentos incompletos carecen de contadores. Importe atribuible: no comunicado.

[Fuente PDQ](https://github.com/juantoniolloretegea/SV-motor/blob/3370da26c1d19ebab9e742c4a0f085910c5b1bd4/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/examen25-20261007/resultado/INFORME.md) · [Fuente CYB16](https://github.com/juantoniolloretegea/SV-motor/blob/3370da26c1d19ebab9e742c4a0f085910c5b1bd4/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/cyb16-20261008/INFORME.md).

La duración corresponde a operaciones del cliente; sus límites instrumentales se explican en el [método común](../METODO.md). La recepción científica independiente permanece pendiente. Los puestos describen las pruebas indicadas.

## Posición en la evaluación externa de Artificial Analysis

**Puesto económico externo AA: 2 de 4, por coste medio ponderado por tarea.** Ocupa el segundo puesto económico y el primero en el índice de capacidad. Aunque su tarifa por token es mayor, su coste por tarea publicado resulta inferior al de Grok y Qwen. Su demora hasta la primera respuesta es la mayor en este corte y configuración; esta cifra externa no se atribuye al cliente del SV.

| Magnitud externa o tarifaria | Dato conservado |
|---|---|
| Configuración de la ficha externa | max |
| Coste por tarea AA | 3,26 USD |
| Índice de capacidad AA v4.3.2 | 53 |
| Puesto por capacidad AA | 1; empate conservado cuando corresponde |
| Generación externa AA | 44,5 tokens normalizados/s |
| Demora externa AA hasta la primera respuesta | 410,56 s |
| Precio ordinario de entrada / salida | 10,00 / 50,00 USD por millón |

[Fuente externa del modelo](https://artificialanalysis.ai/models/gpt-6-astra) · [Tarifa del proveedor](https://developers.openai.com/api/docs/models/gpt-6-astra). Las cifras externas no son mediciones del SV, una factura, una probabilidad de acierto ni una garantía de servicio. Caché y razonamiento se contabilizan según su condición de subconjuntos, sin sumarlos otra vez a la entrada o salida total.


[Comparación completa de los cuatro proveedores](../INFORME.md) · [Datos por solicitud](../evidencia-sv/ENSAYOS-POR-SOLICITUD.json) · [Resultados complementarios](../RESULTADOS-SV.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
