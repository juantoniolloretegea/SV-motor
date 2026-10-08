# OpenAI · GPT-6 Astra

Ficha justificativa · Versión 1.1 · 08/10/2026.

## Evidencia de las ejecuciones del SV

La réplica MD01 conserva R0, R1 y R2 y 65.545 tokens. En MD01-R0 se registran 5,579 s hasta el primer evento de texto, sin asignar puesto comparativo. Las diferencias de reloj y configuración impiden interpretarlo como comparación controlada de latencia intrínseca. El importe atribuible permanece desconocido.

[Informe científico público fijado](https://github.com/juantoniolloretegea/SV-motor/blob/0246a7df1e9436f4bff4f0d94ac6d36e1f29c1e0/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/replica-md01-20261008/INFORME.md) · [Resultados propios](../RESULTADOS-SV.md) · [Datos de consumo](../evidencia-sv/CONSUMOS-POR-REGISTRO.csv) · [Cotejo MD01](../COTEJO-MD01.md).

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

El [expediente del modelo en el nodo 03](../../openai/gpt-6-astra) conserva sus propias pruebas y límites. El ranquin no los sustituye ni los recalifica. Para admitir un uso, el cumplimiento de parámetros críticos prevalece sobre la economía.

[Volver al ranquin](../readme.md) · [Método común](../METODO.md) · [Datos y reservas](../DATOS.json).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
