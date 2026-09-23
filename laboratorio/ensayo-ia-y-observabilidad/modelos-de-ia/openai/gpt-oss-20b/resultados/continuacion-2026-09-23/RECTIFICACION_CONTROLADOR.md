# Rectificación del controlador y de la interpretación de cuantización

**23 de septiembre de 2026 · Inferencia pendiente; instancia detenida.**

Los intentos 09–12 no alcanzaron el servicio ni emitieron una petición de inferencia. Los cuatro hijos terminaron. La ausencia de los procesos examinados y del puerto 8089 quedó registrada a las 13:25:35 UTC. La parada de Codespaces se confirmó visualmente a las 13:28 UTC. Esta revisión complementa el [corte 01–08](INFORME.md); no declara la instalación operativa.

## Correcciones instrumentales

1. **RSS total interpretada como consumo no recuperable.** Los intentos 07–08 se detuvieron por RSS aunque las muestras anteriores conservaban disponibilidad. La RSS comprende memoria anónima, de archivos y compartida. La corrección separa estos componentes, mantiene la reserva global y aplica el umbral del hijo a `RssAnon + RssShmem`. No presupone que toda memoria de archivos sea recuperable ni convierte la observación en una cuota agregada. [Definiciones del núcleo Linux](https://docs.kernel.org/filesystems/proc.html).
2. **Acuse de escritura excesivamente breve.** En 09 venció la espera de 250 ms del registro. Esto demuestra ausencia de confirmación dentro del plazo; no identifica por sí solo un fallo permanente del disco. La espera pasa a dos segundos. Tras un fallo declarado, los siguientes registros fallan inmediatamente para no retrasar repetidamente el cierre. La pérdida de custodia sigue provocando la parada y se declara.

La [referencia 0.1.10](../../controlador-nativo/README.md) incorpora ambas correcciones, conserva la configuración numérica original y supera 17 registros de prueba locales con Rust 1.98.0, incluida una escritura correcta demorada 400 ms. Las variantes reales 0.1.7–0.1.9 permanecen en el paquete. La referencia 0.1.10 no se presenta como una nueva ejecución del modelo.

## Cuantización solicitada y selección del motor

Las denominaciones Q3K y Q2K utilizadas anteriormente identificaban argumentos solicitados. **No acreditaban que los expertos hubieran adquirido esos formatos.** Debía haberse comprobado esa selección antes de atribuirles un ahorro de memoria.

La configuración oficial fija `hidden_size = intermediate_size = 2880`. En el motor fijado, `GgufMatMul::quantize_expert_stack` consulta `get_quantization_behaviour` para cada matriz de experto. Los bloques K requieren 256 elementos; 2880 no es divisible por 256. La función deriva Q2K y Q3K a Q4_0, de bloque 32, compatible con 2880. Por tanto, el recorrido de conversión de estos expertos selecciona Q4_0, no la reducción a dos o tres bits prevista. Es una deducción del código y las dimensiones fijados; no un inventario de una instancia completamente cargada, que no llegó a existir.

Fuentes exactas: [conversión de expertos](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-quant/src/gguf/mod.rs), [selección y sustitución de tipos](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-quant/src/utils/isq.rs), [tamaños de bloque en Candle](https://github.com/huggingface/candle/blob/35d7ae7ca5c93e17c77359c3617376b8a72e96a4/candle-core/src/quantized/k_quants.rs) y [configuración del modelo](https://huggingface.co/openai/gpt-oss-20b/blob/6cee5e81ee83917806bbde320786a8fb61efebee/config.json).

Esta rectificación prevalece sobre las descripciones previas de Q3K/Q2K como reducciones efectivas. Tampoco se acredita con estos intentos una pérdida de calidad específica de esos dos formatos: no hubo respuesta que evaluar.

## Resultados adicionales

| Intento | Condición solicitada | Duración | Resultado |
|---|---|---:|---|
| 09 | Corrección de memoria; Q3K; un trabajador ISQ | 105,557 s | Parada por vencimiento del acuse del registro; custodia declarada no conforme. |
| 10 | Correcciones de memoria y escritura; Q3K | 663,070 s | SIGTERM exterior anterior a la parada propia; sin servicio. |
| 11 | Q2K; mismos controles | 83,945 s | SIGTERM exterior anterior a la parada propia; sin servicio. |
| 12 | Q2K; devolución inmediata de páginas libres por mimalloc | 116,381 s | Opciones reconocidas por el motor; SIGTERM exterior, sin servicio. |

El intento 10 superó la antigua cota descriptiva de RSS y continuó. Su máximo muestreado fue 14 432 149 504 B. En 10–12 aumentó la memoria anónima antes de la terminación. Las trazas muestran `SI_USER` y PID no visible desde el espacio de nombres del contenedor, antes de la llamada de parada del controlador. Esto identifica una procedencia exterior; **el servicio emisor y su motivo no están identificados**. La proximidad temporal del consumo de memoria no demuestra por sí sola la causa de su decisión. Los errores posteriores al enumerar `/proc/PID/fd` son compatibles con la terminación concurrente y no prueban falta de permisos durante una ejecución estable.

El intento 12 fijó `MIMALLOC_PURGE_DELAY=0` y `MIMALLOC_PURGE_DECOMMITS=1`, con diagnóstico del asignador. El registro confirma ambas opciones. Su uso no completó la carga. [Documentación del asignador](https://github.com/microsoft/mimalloc#environment-options).

## Evidencia y continuidad

- [Paquete 09–12](EVIDENCIAS_CORRECCION.tar.gz): 132 272 bytes; SHA-256 `5f66e3ba62d5c78e1232f4090a15ad318fd844bb7976a549df5d46060e61cdd4`; **103 archivos interiores cotejados**.
- [Resumen derivado](RESUMEN_CORRECCION.json), [lector Rust](analizar-correccion.rs) y [parada de la instancia](PARADA_CORRECCION.jpg). Los originales prevalecen sobre el resumen.
- Las observaciones auxiliares intentadas después de la muerte del proceso 11 no aportan datos: los archivos vacíos o ausentes no se presentan como observaciones válidas. La memoria durante la ejecución sí consta en `SUCESOS.jsonl`.

[TT-0012](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0012.md) continúa abierto. La siguiente intervención necesita resolver el consumo transitorio del cargador o disponer de una representación efectivamente compatible y de menor consumo, comprobando también las operaciones CPU de expertos. Cambiar únicamente la etiqueta Q2K/Q3K no satisface esa condición. Se conservan los pesos originales, Harmony, Qwen/B y el mapa de continuidad.

Sistema Vectorial SV · [Aviso y licencias](../../controlador-nativo/AVISO_LICENCIAS.json).
