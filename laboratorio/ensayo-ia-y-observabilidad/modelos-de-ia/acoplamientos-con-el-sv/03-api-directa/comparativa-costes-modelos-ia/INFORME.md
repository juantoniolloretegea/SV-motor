# Informe comparativo de costes, capacidad y respuesta

Versión 1.1 · 08/10/2026 · Cuatro modelos del nodo 03.

Este informe separa las observaciones del SV, los contadores e importes comunicados por proveedores sobre nuestras ejecuciones, los cálculos derivados, las tarifas comerciales y las evaluaciones de terceros. El [resumen general](readme.md) presenta las observaciones propias y el ranquin externo con su alcance.

## Evidencia propia del SV

Se publican [363 filas de consumo](evidencia-sv/CONSUMOS-POR-REGISTRO.csv), [agregados por modelo](evidencia-sv/CONSUMOS-POR-MODELO.json) y [48 solicitudes CYB16](evidencia-sv/CYB16-POR-SOLICITUD.csv). La [procedencia y reproducción](evidencia-sv/PROCEDENCIA.md) permiten recalcularlos sin consultar documentación privada. La verificación de 363 filas se refiere al índice; la recuperación y cotejo de expedientes individuales cubre las 48 solicitudes CYB16.

El [informe de resultados propios](RESULTADOS-SV.md) resume los resultados, tiempos y alcance por ensayo: Astra MD01 en tres etapas; Grok MD01 en dos; Qwen P13 en tres; GLM CYB16 en 48 solicitudes. No son cuatro exámenes de idéntica composición.

| Modelo en MD01-R0 | Primer evento de texto registrado |
|---|---:|
| GPT-6 Astra | 5,579 s |
| Grok 4.7 | 64,239 s |

Ambas respuestas R0 son correctas y completas. Hay una ejecución por modelo, solicitudes adaptadas y un origen del reloj diferente: Grok incluye preparación local adicional. El primer evento puede llevar texto vacío. Se publican ambas cifras sin puestos: **no constituyen una comparación controlada de la latencia intrínseca**. El [cotejo](COTEJO-MD01.md) explicita estas diferencias y publica las fuentes.

**No hay un ranquin económico propio conjunto de cuatro modelos acreditado.** Se conocen 3,59913 USD comunicados en 78 registros de Grok, con conciliación parcial, y 2,35145108 USD estimados para CYB16 de GLM; son trabajos distintos. Los importes no comunicados de Astra y Qwen no se convierten en cero ni se rellenan con una tarifa ajena a su modalidad de acceso.

## Referencias comerciales y evaluaciones externas

Los apartados siguientes conservan el corte externo de la edición 1.0. No sustituyen las ejecuciones propias ni afirman una actualización de las páginas del proveedor.

## 1. Precio de entrada, salida y caché

USD por millón de tokens nativos, servicio ordinario de texto en tiempo real y tramo corto de contexto. No se incluyen impuestos, promociones, procesamiento por lotes, herramientas o escritura y conservación explícitas de caché.

| Proveedor / modelo | Entrada ordinaria | Lectura de caché | Salida | Fuente oficial |
|---|---:|---:|---:|---|
| Z.ai / GLM-5.3 | 1,40 | 0,26 | 4,40 | [Z.ai](https://docs.z.ai/guides/overview/pricing) |
| xAI / Grok 4.7 | 2,00 | 0,50 | 6,00 | [xAI](https://docs.x.ai/developers/models/grok-4.7) |
| Alibaba Cloud / Qwen3.8-Max-0902 | 2,00 | Por confirmar para la cuenta y región específicas | 6,00 | [Alibaba Cloud](https://www.alibabacloud.com/help/en/model-studio/model-pricing) |
| OpenAI / GPT-6 Astra | 10,00 | 1,00 | 50,00 | [OpenAI](https://developers.openai.com/api/docs/models/gpt-6-astra) |

La entrada desde caché es un subconjunto de la entrada; el razonamiento facturable incluido en la salida no se suma otra vez. Las tarifas de contexto extenso y otras modalidades requieren un cálculo distinto.

### Precio ponderado con una mezcla idéntica

La definición actual de precio ponderado de Artificial Analysis utiliza **7 partes de entrada desde caché, 2 de entrada ordinaria y 1 de salida**. Con precios por millón, `P = (7 × Pc + 2 × Pi + Po) / 10`. Es una mezcla convencional para comparar tarifas; no afirma que una carga real consiga un 70 % de tokens servidos desde caché. [Definición](https://artificialanalysis.ai/methodology).

| Orden por precio ponderado | Modelo | USD por millón combinado, referencia AA |
|---:|---|---:|
| 1 | GLM-5.3 | 0,902 |
| 2 | Qwen3.8-Max-0902 | 1,175 |
| 3 | Grok 4.7 | 1,350 |
| 4 | GPT-6 Astra | 7,700 |

Qwen utiliza aquí **0,25 USD/M de lectura de caché publicado por Artificial Analysis**, no una tarifa confirmada en la cuenta del SV. Alibaba excluye este modelo de su regla genérica de descuento y remite al precio específico; esa reserva continúa abierta. Por ello, el cálculo con tarifa oficial de caché queda `null` para Qwen en el fichero de resultados. [Comparación externa](https://artificialanalysis.ai/models/comparisons/gpt-6-astra-vs-qwen3-8-max) · [Condiciones oficiales de caché](https://www.alibabacloud.com/help/en/model-studio/context-cache).

El ejemplo anterior de 1.000.000 tokens de entrada ordinaria y 100.000 de salida sigue dando 1,84 USD para GLM; 2,60 para Grok y Qwen; 15,00 para Astra. Es otro reparto de volumen, sin caché, y no se confunde con la mezcla 7:2:1.

## 2. Por qué el coste por tarea cambia el orden

Artificial Analysis publica costes medios ponderados de 2,01 USD para GLM, 3,26 para Astra, 3,74 para Grok y 5,41 para Qwen en su índice v4.3.2. Este es exclusivamente el orden económico externo de esta edición. Su cálculo incorpora el consumo que la fuente atribuye al conjunto de evaluaciones y sus precios; no se obtiene aplicando una cantidad idéntica de tokens a todos. [GLM/Grok](https://artificialanalysis.ai/models/comparisons/grok-4-7-vs-glm-5-3) · [Astra/Qwen](https://artificialanalysis.ai/models/comparisons/gpt-6-astra-vs-qwen3-8-max).

La fuente también publica magnitudes aproximadas de salida por tarea: Astra 27.000, GLM 71.000, Grok 81.000 y Qwen 108.000 tokens. El razonamiento comunicado forma parte de esas salidas. No se puede reconstruir el coste completo multiplicando sólo estas cifras redondeadas por la tarifa de salida: faltan entrada, caché y ponderaciones. El precio de una unidad y la cantidad consumida explican por qué pueden invertirse posiciones.

**Una tarea evaluada puede haberse resuelto incorrectamente.** Por tanto, estas cantidades no son «dólares por respuesta correcta». Tampoco se divide el coste por 0,53, 0,46 o 0,45: el Intelligence Index es un índice compuesto, no esas tasas de acierto.

## 3. Capacidad y tiempo de respuesta publicados por Artificial Analysis

Se conserva la entrada principal de cada ficha: Astra `max`, Grok `xhigh`, GLM `max` y Qwen 0902 con razonamiento, sin etiqueta de esfuerzo en la ficha. El protocolo externo es común; las configuraciones y recursos de cómputo no son idénticos. Los nombres de los ajustes no prueban equivalencia con los usados en un examen SV.

| Modelo | Índice AA v4.3.2 | Generación, tokens/s¹ | Primer token, s | Primera respuesta, s |
|---|---:|---:|---:|---:|
| GLM-5.3 | 45 | 82,5 | 2,74 | 26,98 |
| GPT-6 Astra | 53 | 44,5 | 410,56 | 410,56 |
| Grok 4.7 | 46 | 68,1 | 91,39 | 91,39 |
| Qwen3.8-Max-0902 | 45 | 35,4 | 2,90 | 59,38 |

¹ Artificial Analysis normaliza la velocidad con un segmentador común; sus precios y costes utilizan los tokens propios comunicados por cada API. La velocidad se mide durante la generación y no incluye la espera inicial. El primer token puede ser de razonamiento; el tiempo hasta la primera respuesta es más pertinente para interpretar lo que espera una persona. Estas mediciones de servicio proceden de un conjunto distinto del índice de capacidad. [Método de rendimiento](https://artificialanalysis.ai/methodology/performance-benchmarking).

Las lecturas de fichas abiertas utilizadas son las fechadas en [DATOS.json](DATOS.json); durante la consulta se observaron fragmentos de buscador con otras cifras de velocidad y demora. No se mezclan con este corte ni se presenta una garantía de latencia. En particular, la demora externa publicada para Astra `max` no predice la demora de un cliente SV con otra configuración.

Por velocidad de generación: GLM, Grok, Astra, Qwen. Por menor demora hasta la primera respuesta: GLM, Qwen, Grok, Astra. Por índice de capacidad: Astra, Grok, GLM y Qwen empatados a la precisión publicada. Ninguno de estos órdenes sustituye al otro.

## 4. Interpretación para el laboratorio

La documentación de mercado permite una preferencia económica por GLM y una preferencia por el índice de capacidad de Astra en las configuraciones observadas. Para relacionar ambas, el gráfico muestra directamente coste y capacidad; no crea una puntuación combinada con pesos arbitrarios. Un índice alto tampoco garantiza fidelidad documental, seguridad o idoneidad médica en un caso concreto.

La decisión de uso del SV requiere primero la conformidad con el contrato pertinente, sus parámetros críticos, privacidad, licencia y límites operativos. Después pueden compararse coste y tiempo entre candidatos admisibles. **Un error crítico no se compensa con una ventaja económica.**

El acceso por autorización ChatGPT utilizado en Astra no equivale a liquidación mediante la tarifa API estándar. Los créditos del plan, las cuotas de bienvenida y los saldos disponibles son condiciones administrativas distintas; quedan fuera de este ranquin de mercado. [Distinción de modalidades de OpenAI](https://learn.chatgpt.com/docs/pricing).

La comparación propia comienza por cotejar los ensayos conservados. MD01 permite publicar las observaciones con las reservas expuestas arriba; no permite un coste por resultado conforme común a los cuatro modelos. Una ampliación posterior requeriría condiciones y relojes homogéneos, cobertura de todos los intentos y costes atribuibles. No se prescribe repetir pruebas existentes ni se autoriza nuevo consumo con esta publicación. El [método](METODO.md) conserva estas condiciones.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
