# gpt-oss-20b: carga GGUF y recepción del servicio nativo

**23 de septiembre de 2026. Recepción técnica de carga y generación; respuesta útil pendiente.**

## Configuración y modificación concreta

Se conserva el ejecutable nativo Rust de mistral.rs 0.9.3, revisión `24dbf5c256f232176ee5949485ba264049407fbe`, SHA-256 `0bd54bd0d17eb79be22b21cccaac0a3f32add5918f93ea162a888feccd7a2158`. La referencia de Harmony permanece en 0.0.8. No se utiliza el SDK de Python ni Ollama.

Se adquiere la conversión `ggml-org/gpt-oss-20b-GGUF`, revisión `b97cbb20d1995efd41dce8c4dd1ddf86e8db375b`, archivo `gpt-oss-20b-MXFP4.gguf`: **12 109 566 624 bytes**, SHA-256 `27cd6c432c7672cb812a92f611cf3ba7bbc35928262bb1e1253ff4ee6ae35901`. Es una conversión publicada por ggml-org; no se presenta como un GGUF distribuido directamente por OpenAI. El descargador Rust comprueba tamaño y huella antes de admitir el archivo; el controlador vuelve a comprobar la huella antes de ejecutar el motor. Los archivos oficiales previamente instalados se conservan.

El anfitrión utiliza CPU, cuatro núcleos, 16 GB nominales y carece de intercambio observado. Se mantienen límite de direcciones virtuales de 32 GiB por proceso, reserva instrumental de 512 MiB, observación de memoria anónima y compartida, identificación del proceso y del puerto, custodia durante la ejecución y cierre del hijo. No constituyen una cuota agregada ni una guarda exterior.

## Secuencia comprobada

| Intento | Controlador | Resultado |
|---|---|---|
| 15 | 0.1.12 | El estimador automático del motor rechaza la carga; salida con código 1. Pico RSS muestreado: 518 246 400 B. Sin servicio ni petición de inferencia. |
| 16 | 0.1.13 | La asignación explícita `--cpu --device-layers 0` carga las 24 capas y habilita el servicio. El controlador rechaza indebidamente la lista formada por el modelo `gguf` y su alias `default`; envía SIGTERM de cierre. Sin petición de inferencia. |
| 17 | 0.1.14 | HTTP 200 y 96 tokens de finalización contabilizados; `finish_reason: length`, `message.content: null`. Generación efectiva, sin respuesta visible útil. |
| 18 | 0.1.15 | HTTP 200 y 256 tokens de finalización; se repiten `length` y `content: null`. La ampliación no resuelve la entrega. |
| 19 | 0.1.16 | Muestra de ocho tokens con probabilidades: hay texto emitido, pero `content: null`. No se obtiene la solución solicitada. |
| 20 | 0.1.17 | F32, misma entrada y ocho tokens: carga completa, servicio comprobado y conexión de la petición cerrada sin respuesta HTTP. El controlador registra `http_incompleto`. |
| 21 | 0.1.18 | F32 con caché F32 explícita: repite el cierre de conexión sin respuesta HTTP. No corrige el resultado de 20. |

En el intento 15, el motor declara `cpu (avail: 13803MB)` y un exceso de `13491MB` en su cálculo de asignación. Es un diagnóstico del estimador, no una medición de memoria consumida ni una demostración de agotamiento. El [código de asignación](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-core/src/pipeline/loaders/auto_device_map.rs) calcula el mensaje a partir de los pesos que permanecen sin asignar. No se interpreta como una medición exacta de memoria adicional necesaria.

El intento 16 sustituye la asignación automática por la [opción explícita admitida por el motor](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-server-core/src/mistralrs_for_server_builder.rs). El registro anuncia `Model loaded` a las 20:41:31.809 UTC y el servicio a las 20:41:33.666 UTC. La respuesta HTTP 200 de `/v1/models` contiene un modelo real, `gguf`, con estado `loaded`, y el alias `default`. Exigir un único elemento era un defecto del controlador, no una multiplicidad real de modelos. El [manejador de esa revisión](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-server-core/src/handlers.rs) incorpora expresamente ese alias.

La candidata 0.1.14 admite ese alias y exige un único modelo real `gguf`, cargado, con raíz concordante y sin modelo padre. Su comprobación con Rust 1.98.0 supera nueve pruebas declaradas: cinco del selector y cuatro del módulo de recursos, una de ellas auxiliar. No se presenta como repetición del banco completo de candidatas anteriores. La 0.1.15 conserva esta corrección y amplía únicamente el presupuesto de salida de la petición; se compila antes de ejecutar.

En 17, el servicio devuelve HTTP 200 tras 164,053 s. La respuesta contabiliza 117 tokens de entrada y 96 de finalización; declara 0,785 tokens de finalización por segundo. El pico RSS muestreado del hijo es 14 598 774 784 B. El límite de salida se alcanza sin texto final visible. El campo técnico `error: null` del controlador no constituye una validación del contenido: no se admite esta respuesta como resolución de la pregunta.

En 18, la petición dura 373,386 s y contabiliza 256 tokens de finalización, con 0,772 tokens/s declarados. En 19 se limita la muestra a ocho tokens y se solicitan sus probabilidades: el texto concatenado es `: No es un orfao‑`. No contiene la solución solicitada y el campo final permanece nulo. Ocho tokens no bastan para evaluar la calidad general del modelo, pero sí distinguen emisión de texto de ausencia total de generación.

El [adaptador Harmony de la revisión ejecutada](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-core/src/reasoning_parsers/harmony.rs) separa los canales y omite los errores de procesamiento de tokens. Este comportamiento es pertinente para investigar la entrega; no demuestra por sí solo que Harmony sea la causa de la generación observada. La inspección de metadatos mediante Rust conserva los identificadores y nombres de los símbolos especiales del GGUF, concordantes con los originales examinados.

En 20 se cambia únicamente la precisión solicitada de BF16 a F32, además de la versión del controlador. El motor activa automáticamente caché F16 en CPU. La comparación 21 fija `MISTRALRS_CPU_KV_F32=1`, opción [prevista por ese motor](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-core/src/kv_cache/mod.rs), para distinguir ese cambio adicional. Se conservan las mismas guardas. La conexión vuelve a cerrarse sin cabeceras ni cuerpo HTTP. No se adopta F32 como corrección ni se atribuye ese cierre a agotamiento de memoria.

## Límites de configuración y alcance

| Intento | Pico RSS muestreado del hijo, B | Lecturas RSS válidas | Lecturas RSS no disponibles |
|---|---:|---:|---:|
| 15 | 518 246 400 | 63 | 2 |
| 16 | 14 593 359 872 | 853 | 0 |
| 17 | 14 598 774 784 | 3974 | 0 |
| 18 | 14 590 693 376 | 8047 | 0 |
| 19 | 14 643 302 400 | 1781 | 0 |
| 20 | 14 674 833 408 | 1270 | 13 |
| 21 | 14 669 312 000 | 1433 | 13 |

Las lecturas no disponibles se conservan como huecos, no como consumo cero. El muestreo del hijo y del grupo visible no permite excluir agotamiento de memoria en ámbitos no observados. Los resultados 20 y 21 registran cierre HTTP incompleto, sin activación declarada de una guarda por memoria; no identifican su causa. Todos los hijos terminan y se confirma su cierre. La observación final, a las 21:20:53 UTC, no encuentra los procesos propios examinados ni escucha en el puerto 8089. La instancia se conserva activa para continuar el diagnóstico.

El motor utiliza el tokenizador incorporado al GGUF: anuncia 201 088 entradas y rechaza como incompatible el tokenizador externo de 200 019 entradas. También declara que ignora el archivo externo de configuración de generación. Se conserva este comportamiento; no se presume equivalencia completa entre ambas representaciones.

El parámetro `--max-seq-len 1024` pertenece al cálculo automático de asignación. Su presencia en una invocación con asignación explícita no acredita un límite efectivo de contexto de 1024. La petición concreta es breve, sintética y solicita temperatura 0 y esfuerzo de razonamiento bajo. El presupuesto de salida es 96 tokens en 17 y 256 en 18. El servicio escucha exclusivamente en `127.0.0.1:8089`.

Los pesos GGUF se alojan en almacenamiento temporal separado del volumen del proyecto. No se garantiza su conservación al detener o reconstruir la instancia. Las fuentes, registros y evidencias se conservan aparte; no se distribuyen pesos ni ejecutables del motor en este depósito.

Esta ejecución corresponde a la vía B. La carga conseguida no acredita calidad clínica, aptitud para ciberseguridad, integración contractual completa ni ejecución en navegador. Qwen/B conserva su cierre como realización parcial. Los nuevos hechos no atribuyen retrospectivamente una causa a las señales de los intentos anteriores.

## Evidencias y licencias

- [Paquete original](EVIDENCIAS_GGUF_RECEPCION_20260923.tar.gz): 228 833 bytes; SHA-256 `0c24a3ff967b95871a1885c793fa1187c3aa7872bab9d6ef144d24fd12bcf86d`, coincidente entre origen y recepción. Contiene 115 archivos: fuentes, compilaciones, pruebas y registros de los intentos 15–21.
- [Resumen derivado](RESUMEN_RECEPCION.json), con registros finales, identidad de fuentes y las tres respuestas HTTP originales. Los JSONL del paquete prevalecen sobre este resumen.
- [Controlador BF16 0.1.16](controlador-bf16/Cargo.toml), [comparación F32 0.1.18](controlador-f32/Cargo.toml) y [descargador Rust](descarga/Cargo.toml). Las restantes variantes se conservan en el paquete. Estas candidatas no sustituyen automáticamente la referencia instrumental publicada.
- [Inventario del controlador](DEPENDENCIAS_CONTROLADOR.json), [inventario del descargador](DEPENDENCIAS_DESCARGA.json), [inspector de metadatos Rust](gguf-inspector.rs) y [huellas](SHA256SUMS).

El objetivo pendiente es localizar la discrepancia entre entrada, operaciones del motor, secuencia de tokens y extracción del canal final. Aumentar únicamente el límite de generación ya produjo un resultado adverso. No se concede conformidad de contenido ni se adopta una variante numérica que no entrega respuesta.

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es). mistral.rs conserva MIT; Harmony y los pesos mantienen sus condiciones propias. La conversión GGUF se atribuye a [ggml-org](https://huggingface.co/ggml-org/gpt-oss-20b-GGUF/tree/b97cbb20d1995efd41dce8c4dd1ddf86e8db375b). Los inventarios de dependencias distinguen el controlador, el descargador y el motor externo.
