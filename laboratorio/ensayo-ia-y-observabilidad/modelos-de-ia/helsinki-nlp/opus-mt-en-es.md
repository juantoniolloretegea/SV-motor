# Helsinki-NLP/opus-mt-en-es: traducción en Rust y preparación del contraste CPU/TPU

14 de octubre de 2026.

## Objeto y estado

Este expediente identifica el motor de traducción inglés–español que ya funciona en el Sistema Vectorial SV y prepara su contraste mediante el **Selector del Árbitro - Director CPU/GPU/TPU/NPU**. Se reutiliza la implementación existente; no se sustituye el modelo ni se modifica el procedimiento de generación para mejorar artificialmente sus resultados.

**Estado de esta edición:** funcionamiento CPU e integración documental comprobados en las actuaciones anteriores; fuentes canónicas recuperadas y cotejadas; conexión del modelo con el Selector y ejecución del modelo en TPU pendientes. No se han realizado nuevas inferencias en esta preparación. La comprobación del servicio encontró el gestor activo y el mismo ejecutable identificado en los resultados conservados.

La ejecución de traducción pertenece a la infraestructura administrada por el SV. Esta función puede requerir selección de dispositivo aunque otras consultas se dirijan a modelos externos. El ensayo estudia esa capacidad para entornos administrados por el sistema; no declara validado el nodo 1 ni modifica la arquitectura de los tres nodos.

## Identidad y funcionamiento existente

| Elemento | Identificación |
|---|---|
| Modelo | [Helsinki-NLP/opus-mt-en-es](https://huggingface.co/Helsinki-NLP/opus-mt-en-es) |
| Revisión de los archivos | `fdaddf76f50fcc1583ba42f95965862a7ab30f97` |
| Arquitectura | Marian, codificador y decodificador de seis capas cada uno |
| Inferencia | Rust, Candle 0.11.0, CPU, precisión F32 |
| Pesos | Safetensors, 312 062 468 bytes |
| SHA-256 de los pesos | `b3ecbf954573c2fd95d05d3ad4618baf961793db0da07c2add2e8a3a6cd78d0b` |
| SHA-256 del ejecutable utilizado | `9d1e430db85541190fb86607834042627b493079ce5174563746b87f522e6447` |
| Procedimiento vigente | Selección de la continuación más probable; sin muestreo |
| Preparación documental | `lectura-estructurada-2`, delimitación por oraciones y conservación de contenido protegido |
| Sistema | Linux x86_64; procesos con red denegada y lectura restringida mediante seccomp y Landlock |

El gestor prepara los fragmentos, solicita la traducción al proceso Rust, comprueba correspondencia e integridad y recompone la presentación conservando los elementos protegidos. El motor admite hasta 384 tokens de entrada antes del final de secuencia y hasta 480 pasos de generación; rechaza excesos y salidas incompletas, sin truncamiento silencioso. El proceso de traducción dispone de un límite de 60 segundos por fragmento; el gestor aplica además 45 segundos al conjunto de su petición. Son límites distintos que debe conservar y medir cualquier ensayo posterior.

Las traducciones facilitan la lectura de personas. Los modelos examinados o consultados reciben los documentos originales, completos y en su idioma nativo. Esta función no modifica esa regla.

## Código vinculado al modelo

La carpeta [codigo](codigo) contiene:

- [Motor completo](codigo/motor): manifiestos Cargo, código Rust de inferencia, tokenización y aislamiento, con su licencia conservada.
- [Integración documental](codigo/integracion/traduccion.rs): copia exacta del módulo del gestor. Es una referencia de integración, no una aplicación autónoma.
- [Selector](codigo/selector): copia exacta del componente de selección. Su presencia en esta carpeta no implica que el motor de traducción ya lo invoque.
- [Comprobación en Rust](codigo/comprobacion): verificación de huellas, correspondencia entre entradas y salidas y recuperación de los archivos publicados.

[PROCEDENCIA.json](codigo/PROCEDENCIA.json) identifica cada archivo y su origen. Las siete fuentes del motor y de su integración proceden de la revisión `c60f5ab9d017715045ea7e3b189bc5943651b750` de [gestion-usuarios-y-documentacion-sv](https://github.com/IA-en/gestion-usuarios-y-documentacion-sv/tree/c60f5ab9d017715045ea7e3b189bc5943651b750/codigo/desarrollo/traduccion-documental-rust-20261012) y coinciden byte por byte con las copias locales examinadas. No se modifican el núcleo, la semántica ni la representación intermedia del SV.

## Resultados anteriores que se reutilizan

Se conservan [las trece entradas](evidencias/ENTRADAS-CPU-ANTERIORES.jsonl), [sus salidas](evidencias/SALIDAS-CPU-ANTERIORES.jsonl) y [las mediciones](evidencias/MEDICIONES-ANTERIORES.json). Son antecedentes identificados, no una nueva prueba CPU/TPU.

| Medida de la ejecución CPU anterior | Valor |
|---|---:|
| Fragmentos traducidos | 13 |
| Tokens de entrada / salida | 239 / 270 |
| Generación acumulada | 10 127 ms |
| Carga del modelo, separada | 2 179 ms |
| Memoria máxima del proceso | 676 488 KiB |

Una comparación posterior conservó las trece salidas del procedimiento vigente y estudió cuatro secuencias alternativas. La duración pasó de 8,324 a 44,672 segundos; se observaron dos correcciones gramaticales, sin mejora semántica material. Se mantuvo el motor vigente. Estas duraciones pertenecen a otra ejecución y no deben mezclarse con la tabla anterior. [Resultado de aquella comparación](https://github.com/IA-en/gestion-usuarios-y-documentacion-sv/blob/c60f5ab9d017715045ea7e3b189bc5943651b750/verificaciones/traduccion-cuatro-alternativas-20261014/RESULTADO.md).

Persisten errores de concordancia y terminología. Funcionamiento técnico y aceleración no acreditan por sí mismos calidad científica general de las traducciones. Los trece fragmentos proceden de pasajes médicos y de información normativa; no representan la cobertura completa de inmunología ni ciberseguridad.

## Procedimiento del contraste de dispositivos

Se sigue la secuencia empleada con Qwen: identidad y compatibilidad, conexión del Selector, calibración, solicitudes con destino obligatorio y selección por rendimiento, observación del dispositivo efectivo y conservación verificable. Los pesos, tokenizadores y generación del traductor vigente constituyen la referencia.

1. Fijar la implementación existente y conservar sus entradas y salidas. **Realizado en esta edición.**
2. Preparar la conexión del motor con el Selector y la realización TPU. Debe conservarse la topología Marian, los límites y el aislamiento; el servicio activo permanece separado del ensayo.
3. Cotejar la realización adaptada con la referencia CPU, fijando antes de inferir precisión, tolerancias numéricas y reglas de comparación de las secuencias.
4. Medir CPU y TPU con la misma carga: inicialización y compilación por separado, después peticiones comparables. No se presupone ventaja de TPU.
5. Ejecutar CPU obligatoria, TPU obligatoria y selección por rendimiento. Una TPU incompatible o no disponible debe impedir su uso; no cabe sustituirla silenciosamente por CPU.
6. Conservar entradas, salidas, decisiones, tiempos completos, memoria, transferencias y trazas originales. Recuperar y cotejar los resultados antes de retirar los recursos utilizados.

La telemetría distinguirá preparación y control en CPU, cálculo del modelo, transferencias, espera y recuperación. Si algunas operaciones permanecen en CPU durante una solicitud TPU, se identificarán expresamente; no se declarará ejecución íntegra del modelo en TPU basándose únicamente en productos matriciales aislados.

## Compatibilidad pendiente

El motor conservado fija `Device::Cpu`. La interfaz de dispositivos de [Candle](https://github.com/huggingface/candle/blob/main/candle-core/src/device.rs) contempla CPU, CUDA y Metal; no ofrece una selección TPU directa. Su aislamiento actual permite acceder a los archivos del modelo y a las métricas propias, no a un dispositivo TPU.

El adaptador Rust/PJRT del contraste previo de Google Cloud ya ejecutó y observó productos matriciales en CPU/TPU. Ese resultado es reutilizable para la comunicación y la instrumentación, pero todavía no acredita la ejecución del codificador y decodificador Marian. La conexión pendiente pertenece al recorrido del modelo hacia TPU; no supone que falten un traductor Rust o su integración documental.

La vía TPU examinada utiliza además la biblioteca oficial nativa `libtpu`, que incorpora servicios y compilación del fabricante. No se presenta toda esa infraestructura como código Rust. La identificación del dispositivo, por sí sola, tampoco acredita ejecución del modelo.

[ESTADO.json](evidencias/ESTADO.json) separa lo comprobado de lo pendiente. Esta edición no atribuye rendimiento TPU al traductor, no declara concluido ese ensayo y no comunica nuevos consumos de Google Cloud. Energía e importe monetario atribuible permanecen desconocidos cuando no hay una medición propia.

## Integridad y licencias

[MANIFIESTO.json](MANIFIESTO.json) identifica los archivos de esta edición, salvo el propio manifiesto. Los pesos se identifican mediante [su manifiesto](evidencias/IDENTIDAD-MODELO.json), sin duplicar su distribución. El código de terceros y el modelo conservan sus licencias; el archivo Apache 2.0 se incluye en el motor. El [procedimiento de comprobación](codigo/LEAME.md) permite recuperar y verificar los contenidos.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

