# Resultados documentados de las ejecuciones del SV

**Versión 1.1 · 08/10/2026.** Se presentan cuatro ensayos conservados. Sus preguntas, etapas y cobertura son diferentes; esta tabla no constituye una competición común.

## Resultados científicos y volumen observado

| Modelo y fuente pública fija | Ensayo y cobertura | Resultado documentado | Tokens de entrada / salida / total |
|---|---|---|---|
| [GPT-6 Astra](https://github.com/juantoniolloretegea/SV-motor/blob/0246a7df1e9436f4bff4f0d94ac6d36e1f29c1e0/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/replica-md01-20261008/INFORME.md) | Réplica MD01; una pregunta, R0, R1 y R2 | Conformidad de la réplica en sus tres etapas | 61.110 / 4.435 / **65.545** |
| [Grok 4.7](https://github.com/juantoniolloretegea/SV-motor/blob/091b50a884c893fd4322bbf211b0feb0132bb258/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/xai/grok-4.7/md01-diagnostica-20261008/INFORME.md) | MD01 diagnóstica; R0 y R1 | R0 y R1 correctas y completas; R2 no enviada; cobertura parcial | 46.763 / 20.026 / **66.789** |
| [Qwen3.8-Max-0902](https://github.com/juantoniolloretegea/SV-motor/blob/fba54670fbb45727b69566af5ea985204d96ed05/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/qwen/qwen3.8-max-0902/replica-p13-20261008/INFORME.md) | Réplica P13; R0, R1 y R2 | R2 conforme; no sustituye el dictamen histórico del examen general | 64.347 / 10.672 / **75.019** |
| [GLM-5.3](https://github.com/juantoniolloretegea/SV-motor/blob/76dea7a3b6279e3bd3da62680059c860776a13fa/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/zai/glm-5.3/cyb16-20261008/INFORME.md) | CYB16; 16 preguntas, tres etapas, 48 solicitudes | R0: 11 correctas y 5 errores críticos; R1: 15 y 1; R2: 16 y 0. Aptitud de R2 limitada al contrato; recepción científica independiente pendiente en el corte | 1.226.311 / 157.944 / **1.384.255** |

Los contadores proceden de las respuestas de los proveedores a ejecuciones del SV. Los dictámenes proceden de los informes enlazados y conservan su alcance. En los cuatro casos permanece pendiente la recepción científica independiente en el corte documental. Las síntesis públicas no equivalen a la publicación íntegra de todas las respuestas originales; el presente conjunto permite auditar las cifras publicadas, no reconstruir por sí solo toda la adjudicación científica. No se transforma el resultado de una pregunta o una réplica en aptitud universal del modelo.

| Ensayo | Duración total registrada del banco | Muestras temporales | Mayor intervalo registrado |
|---|---:|---:|---:|
| Astra MD01 | 126.191 ms | 452 | 289 ms |
| Grok MD01, dos etapas | 243.560 ms | 908 | 367 ms |
| Qwen P13 | 243.295 ms | 896 | 550 ms |
| GLM CYB16 | 2.461.600 ms | 8.976 | 495 ms |

Las duraciones totales incluyen operaciones del instrumento. Los intervalos de muestreo describen su seguimiento; no son latencias del modelo. No se ordenan estos totales, porque la cantidad y naturaleza del trabajo son distintas.

## Observaciones temporales de MD01-R0

| Modelo | Primer evento de texto, ms | Resultado R0 |
|---|---:|---|
| GPT-6 Astra | 5.579 | Correcta y completa |
| Grok 4.7 | 64.239 | Correcta y completa |

La pregunta y el corpus coinciden. Las solicitudes completas y el punto inicial del reloj no son idénticos. Se conservan ambas cifras sin asignar puestos: **no constituyen una comparación controlada de latencia intrínseca**. Una sola ejecución por candidato no permite afirmar significación estadística. Véanse [cotejo](COTEJO-MD01.md), [datos](DATOS-SV.json) y [tabla reproducible](OBSERVACIONES-MD01.json).

## Evidencia económica publicable

| Conjunto | Dato disponible | Naturaleza y límite |
|---|---|---|
| Grok, 78 de sus 79 registros publicados | 3,5991300000 USD | Importe comunicado; conciliación parcial. No equivale a factura ni sólo a MD01. |
| GLM, CYB16: 48 solicitudes | 2,35145108 USD | Estimación mediante contadores y tarifas; liquidación individual desconocida. |
| Astra, ensayos publicados | Importe atribuible no comunicado | No aplicar retrospectivamente una tarifa API a otra modalidad de acceso. |
| Qwen, ensayos publicados | Importe atribuible no comunicado | La cobertura por cuota no acredita coste monetario comparable igual a cero. |

El cálculo de CYB16 resta de la entrada total los tokens de caché antes de aplicar el precio ordinario; la caché se valora separadamente. El razonamiento está incluido en la salida y no se suma por segunda vez. [Desglose y fórmula reproducible](evidencia-sv/PROCEDENCIA.md).

## Inventario de contadores por modelo

| Modelo | Registros publicados | Con contadores completos | Entrada conocida | Salida conocida | Total conocido |
|---|---:|---:|---:|---:|---:|
| GPT-6 Astra | 156 | 154 | 1.574.576 | 103.884 | 1.678.460 |
| Grok 4.7 | 79 | 78 | 1.078.935 | 263.250 | 1.342.185 |
| Qwen3.8-Max-0902 | 52 | 52 | 672.829 | 90.540 | 763.369 |
| GLM-5.3 | 76 | 76 | 1.868.420 | 220.432 | 2.088.852 |

Los cuatro ensayos anteriores están contenidos en este inventario: **no se suman nuevamente**. No se interpreta el número de registros como tareas equivalentes, ni el volumen agregado como eficiencia. Las diferencias de segmentación impiden tratar tokens de modelos distintos como unidades lingüísticas uniformes.

Se cotejaron las 363 filas contra el índice de origen. Para CYB16 también se recuperaron y cotejaron los 48 registros individuales. No se atribuye a esa comprobación el alcance de una auditoría integral de los 363 expedientes. El lector dispone de [todas las filas depuradas](evidencia-sv/CONSUMOS-POR-REGISTRO.csv), [agregados](evidencia-sv/CONSUMOS-POR-MODELO.json), [CYB16 por solicitud](evidencia-sv/CYB16-POR-SOLICITUD.csv) y [comprobador Rust](evidencia-sv/calculo-consumos-rust/src/main.rs).

**Conclusión de alcance:** existen mediciones y resultados propios del SV. Su evidencia permite la exposición de las observaciones MD01-R0 y los cálculos publicados; todavía no permite un ranquin monetario conjunto de cuatro candidatos bajo un mismo encargo.

[Comparación general](readme.md) · [Método](METODO.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
