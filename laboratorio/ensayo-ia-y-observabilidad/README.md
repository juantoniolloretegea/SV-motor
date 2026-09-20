# Ensayo de inteligencia artificial y observabilidad

**Versión documental:** 0.2  
**Fecha:** 18 de septiembre de 2026  
**Estado:** preparación experimental; implementación y mediciones pendientes.

## Objeto y adscripción

Este espacio reúne el diseño de un ensayo reproducible de inferencia y observabilidad dentro del dominio de trabajo del Sistema Vectorial SV. Se adscribe al bloque (p1+P3)-Bis del Lenguaje SV y aporta evidencia técnica para sus decisiones de diseño. Su apertura no constituye el motor de inteligencia artificial definitivo ni acredita los resultados de pilotos anteriores.

La pregunta experimental es si una implementación Rust puede ejecutar un modelo auxiliar acotado y registrar sus operaciones instrumentadas con un coste medible, preservando la separación entre propuesta probabilística, autorización y efecto. La calidad de la inferencia y la corrección de la frontera se evaluarán por separado.

El [contrato experimental](contrato/README.md) vincula este ensayo con el [acta de rutas de conocimiento del 14 de septiembre](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3d362a01c05644362a8bf77f456b4ecfedd8fd57/docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/ACTA_EVALUACION_Y_RECEPCION_DOCUMENTAL_RUTAS_CONOCIMIENTO_SV_2026_09_14.md) y con el [Acta 001 de continuidad](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3d362a01c05644362a8bf77f456b4ecfedd8fd57/docs/calidad/tuberias-ia/continuacion-15-09-2026/ACTA_001_CONTINUIDAD_Y_RUMBO_2026_09_15.md). La primera aporta obligaciones y reservas; la segunda conserva la secuencia y las sedes de autoridad. Este laboratorio produce evidencia para su recepción posterior: no establece un estado canónico paralelo.

## Selección experimental

| Función | Selección | Estatuto |
|---|---|---|
| Inferencia | Candle | Motor seleccionado para este ensayo; adopción definitiva no resuelta. |
| Modelo inicial | Qwen3-0.6B, cuantización Q4_K_M | Referencia de pequeña escala para comprobar el acoplamiento. |
| Observabilidad | OpenTelemetry Rust | Único sistema de instrumentación y exportación de telemetría seleccionado para el ensayo. |
| Implementación y verificación | Rust 1.98.0 | Versión que deberá comprobarse y registrarse antes de compilar. |
| Entorno inicial | Ejecución nativa de referencia en GitHub Actions y WebAssembly en navegador | Compatibilidad, reproducibilidad y costes pendientes de comprobación. |

La selección de OpenTelemetry Rust es una decisión experimental efectiva. No constituye una afirmación de suficiencia universal ni incorpora un servicio externo de recolección. La elección de una biblioteca no implica ausencia de dependencias transitivas; estas deberán identificarse en el manifiesto de construcción.

## Organización

| Directorio | Contenido |
|---|---|
| [contrato/](contrato/README.md) | Perímetro, fuentes rectoras y correspondencia entre obligaciones y pruebas. |
| [inferencia/](inferencia/README.md) | Identidad del modelo, adaptación del motor y límites de generación. |
| [observabilidad/](observabilidad/README.md) | Integración de OpenTelemetry Rust, señales, cobertura y límites. |
| [pruebas/](pruebas/README.md) | Protocolo experimental, controles negativos y criterios de aceptación. |
| [resultados/](resultados/README.md) | Evidencias de ejecución, mediciones y límites de interpretación. |

## Perímetro

El ensayo se limita a las entradas, operaciones y efectos expuestos por su propia interfaz. No introduce semántica probabilística en el núcleo del SV ni otorga autoridad a la salida del modelo. Tampoco constituye un antivirus, un sistema de detección de intrusiones o un monitor general del sistema operativo.

La instrumentación del programa no acredita por sí sola todas las escrituras, conexiones o actividades de procesos ajenos. La cobertura observada, los puntos instrumentados y las dependencias del entorno deben declararse. La variante WASI y otros sistemas operativos requieren comprobación específica; una ejecución satisfactoria en navegador no los acredita.

## Presupuesto inicial

Las cantidades siguientes son restricciones del protocolo y objetivos de aceptación; no representan consumos medidos ni controles ya implementados.

| Magnitud | Presupuesto |
|---|---|
| Pesos | Un único fichero, hasta 500 MB; sin descarga de colecciones de variantes. |
| Conversaciones simultáneas | Una. |
| Contexto retenido | Hasta 2.048 tokens, incluida la salida generada. |
| Generación | Hasta 128 tokens por caso. |
| Memoria del conjunto de procesos del ensayo | Objetivo de aceptación: hasta 4 GiB; método y cobertura de medición por declarar. |
| Paquete local conservado | Hasta 2 GiB. |
| Ocupación local adicional máxima | 5 GiB, incluidos temporales y copias atribuibles al ensayo. |
| Reserva del volumen del sistema | Al menos 30 GiB libres; comprobación previa y durante las operaciones que escriban en él. |
| Evidencia exportada | Hasta 20 MiB por ejecución, con registro explícito de truncamientos y pérdidas. |
| Ejecuciones iniciales en Actions | Hasta tres, secuenciales, con límite de 40 minutos por ejecución y sin reintentos automáticos. |

Se distinguen MB decimales, de 10⁶ bytes, y MiB/GiB binarios, de 2²⁰/2³⁰ bytes. La ocupación temporal del ejecutor de Actions y la del equipo local se contabilizan por separado. La disponibilidad efectiva del ejecutor debe medirse antes de construir. No se habilitan recursos de pago, servicios persistentes ni almacenamiento masivo.

### Estimación de memoria

El proveedor de la cuantización publica aproximadamente 397 MB para el fichero Q4_K_M. Esta cifra no equivale al consumo de memoria de la aplicación.

Para una caché convencional de claves y valores:

M_KV = 2 × L × H_KV × d × T × B × s

Con L = 28 capas, H_KV = 8 cabezas, d = 128 componentes, T = 2.048 tokens, B = 1 conversación y s = 2 bytes, el resultado es 234.881.024 bytes, equivalentes a 224 MiB. Con s = 4 bytes asciende a 448 MiB. Estas estimaciones excluyen pesos, reservas, copias y tensores temporales. El tipo numérico real de la caché debe comprobarse; cuantizar los pesos no acredita la cuantización de la caché.

## Condiciones de ejecución

Antes de ejecutar deben estar identificados el código, los pesos, el tokenizador, las dependencias, las opciones de compilación y los mecanismos de limitación y parada. Deben corregirse o descartarse de forma fundada las incompatibilidades del ejemplo de referencia relativas a finalización, formato conversacional y registro del contenido de las consultas.

La primera fase utilizará datos de prueba sin información personal ni credenciales. La inferencia no requiere acceso general a archivos, servicios ni dispositivos del equipo. Las dependencias y los pesos se obtendrán de fuentes identificadas; cualquier comunicación durante el ensayo tendrá una finalidad declarada.

Una interrupción, una pérdida de evidencia o un exceso de recursos no se presentarán como resultado satisfactorio. La imposibilidad de medir una magnitud se declarará expresamente.

## Estado de la evidencia

Esta versión contiene exclusivamente documentación preparatoria. No incluye pesos, ejecutables, dependencias instaladas ni resultados de inferencia. No acredita rendimiento, paridad entre plataformas, integridad del equipo anfitrión o suficiencia del modelo para una operación del SV.

## Referencias

- [Continuidad del bloque de diseño en el Lenguaje SV](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/tree/main/docs/calidad/tuberias-ia/continuacion-15-09-2026).
- [Candle: revisión de referencia](https://github.com/huggingface/candle/tree/ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a).
- [Configuración de Qwen3-0.6B](https://huggingface.co/Qwen/Qwen3-0.6B/blob/main/config.json).
- [Distribución de la cuantización Q4_K_M](https://huggingface.co/unsloth/Qwen3-0.6B-GGUF/blob/main/Qwen3-0.6B-Q4_K_M.gguf).
- [OpenTelemetry Rust](https://github.com/open-telemetry/opentelemetry-rust).
- [Recursos de los ejecutores de GitHub Actions](https://docs.github.com/en/actions/reference/runners/github-hosted-runners).

Las referencias móviles sirven para identificar las fuentes. Antes del ensayo deberán sustituirse en el manifiesto de ejecución por revisiones concretas y huellas verificadas de los artefactos.


## Actualización de continuidad · 20 de septiembre de 2026

La versión preparatoria anterior se conserva como antecedente. Existen ya resultados de EIO-05, EIO-06 y EIO-JSON-01. La [revisión receptora de JSON-01](resultados/json-01/RECEPCION.md) contrasta 24 controles de regresión y 35 complementarios, con una reserva explícita sobre la identidad de los archivos efímeros de evidencia. El rechazo estructural de las salidas del modelo permanece como resultado adverso. La comparación con/sin telemetría ya tiene tres pares en EIO-05; la comprobación WASM de EIO-06 no acredita ejecución en navegador. El siguiente trabajo es preparar esa ejecución, dentro del perímetro original, sin nuevas inferencias o ejecuciones autorizadas por esta nota. No se constituye una solución definitiva ni se cierra la aceptación científica.


## EIO-NAV-01 · preparación autorizada · 20/09/2026

[Paquete derivado y precompromiso](resultados/preparacion-navegador-01/README.md): una ejecución remota adicional número 8, intento 1, sin reintentos. Incluye matriz NAV-01…05, custodia original y cierre; aún no acredita ejecución en navegador. Los antecedentes y sus reservas se conservan. S38 sigue pendiente en su sede propia.


## EIO-NAV-01 · ejecución consumida · 20/09/2026

[Resultado y custodia](resultados/navegador-01/README.md): run35503596075, número8/intento1, failure92. Construcción y controles NAV01–04 conformes; NAV05 interrumpido por umbral RSS, sin salida contractual. Recuperación de39/39 archivos emitidos conforme, con laguna explícita de captura al cierre. Guardas cerradas y sin reintento. Pendiente revisión receptora.


## EIO-NAV-02 · preparación · 20/09/2026

[Diagnóstico acotado y precompromiso](resultados/preparacion-navegador-02/README.md): campaña9/intento1, umbral RSS conservado, métricas complementarias y captura incremental. No optimización del modelo ni reintento de NAV01. Preparación pendiente de observación.


## EIO-NAV-02 · diagnóstico entregado · 20/09/2026

[Resultado observado y custodia](resultados/navegador-02/README.md): run35507386447, número9/intento1, failure92 por umbral RSS conservado. NAV01–04 conformes; ModelWeights completado y marca previa al primer forward recibida, sin primer token ni salida contractual. Captura incremental conservada,49/49 identidades cotejadas; reservas de PSS final y coste de escritura explícitas. Guardas cerradas, sin reintento. Pendiente revisión receptora.
