# OpenAI · gpt-oss-20b

**Estado vigente · 24/09/2026:** [Recuperación y evidencia](resultados/recuperacion-onecloud-2026-09-24/INFORME.md). Candidata MXFP4 conservada y trasladada a OneCloud; una prueba de regresión aprobada y arranque básico comprobado. Inferencia con el parche pendiente; pesos y controlador operativo por restituir y verificar. Los apartados siguientes conservan sus cortes históricos.

**Estado al 23 de septiembre de 2026:** carga completa en CPU y peticiones reales con generación; respuesta útil todavía pendiente.

La [continuación GGUF](resultados/gguf-2026-09-23/RESULTADO.md) documenta la conversión MXFP4 de ggml-org, su revisión y huella, conservando mistral.rs 0.9.3 y Harmony 0.0.8. La asignación CPU explícita permite cargar las 24 capas. Se corrige además el rechazo del alias `default` por el controlador. Las peticiones de 96 y 256 tokens devuelven HTTP 200 y alcanzan su límite sin contenido final visible. La muestra posterior de ocho tokens acredita emisión de texto; el diagnóstico de entrega y fidelidad permanece abierto.

Los [intentos anteriores](resultados/mxfp4-2026-09-23/RESULTADO.md), sin servicio ni inferencia, conservan su resultado histórico. La [continuación inicial](resultados/continuacion-2026-09-23/INFORME.md), la [rectificación instrumental](resultados/continuacion-2026-09-23/RECTIFICACION_CONTROLADOR.md) y el [primer intento](resultados/2026-09-23/RESULTADO.md) mantienen sus fuentes y registros. La nueva carga no determina retrospectivamente la causa de sus señales.

## Controlador y capacidad

Las candidatas GGUF 0.1.12–0.1.18 se identifican en el [informe de recepción](resultados/gguf-2026-09-23/RESULTADO.md). La corrección del selector supera nueve pruebas declaradas con Rust 1.98.0 en el entorno remoto. BF16 alcanza generación, sin entrega final útil. Las comparaciones F32, con caché automática y con caché F32 explícita, cierran la conexión de inferencia sin respuesta HTTP y no se adoptan como solución. El pico RSS muestreado máximo de esta serie es 14 674 833 408 B; persisten límites de cobertura y lecturas no disponibles declaradas.

El [controlador de referencia 0.1.10](controlador-nativo/README.md) conserva las correcciones instrumentales anteriores. La [candidata 0.1.11](resultados/ventana-acotada-2026-09-23/RESULTADO.md) añade la configuración de cuantización y fue ejecutada en los intentos 13 y 14. Su banco local secuencial supera 17 registros, incluidos dos auxiliares; esa comprobación no equivale a inferencia conseguida ni sustituye por sí sola a la referencia.

Los 13 123 MiB anunciados previamente por el cargador son un inventario, no un máximo RSS. Los nuevos resultados conservan memoria anónima, memoria respaldada por archivos, disponibilidad y señales. Las guardas observan el proceso hijo y el ámbito visible; no acreditan una cuota agregada ni todos los límites del anfitrión. El informe declara las medidas y las incertidumbres, sin atribuir retrospectivamente el SIGTERM original a falta de memoria.

## Antecedente documental del 22 de septiembre de 2026

El contenido siguiente conserva el estado previo a este intento; sus referencias a falta de instalación y de selección de motor corresponden a aquel corte histórico.

**Estado:** evaluación documental; sin instalación ni ejecución en este ensayo.  
**Destino propuesto:** vía A, condicionado a viabilidad real.  
**Corte documental:** 22 de septiembre de 2026.

| Elemento | Condición conocida o pendiente |
|---|---|
| Modelo | gpt-oss-20b; pesos abiertos. La distribución concreta y su huella se fijarán si se admite un ensayo. |
| Memoria | OpenAI indica funcionamiento en configuraciones de 16 GB con la representación prevista. No constituye una garantía de memoria total ni de viabilidad en un navegador. |
| Formato de conversación | Harmony. Debe respetarse mediante un adaptador compatible. |
| Motor numérico | No seleccionado. El registro de modelos de la revisión de Candle utilizada por Qwen no contiene una implementación gpt-oss. |
| Vía A | Pendientes el motor compatible con navegador/WebAssembly, los requisitos efectivos de memoria y, cuando proceda, WebGPU, y el alcance real de las guardas. |
| Recursos | Sin aprovisionamiento ni descarga de pesos en aquel corte documental. |
| Licencias | Los pesos oficiales se publican bajo Apache-2.0; cada componente adicional deberá conservar su aviso correspondiente. |

La ruta se determina por el lugar donde se ejecuta la inferencia. Si el navegador envía peticiones a un proceso nativo remoto, ese funcionamiento no acredita la vía A documentada. Si WebAssembly controla un proceso exterior, la inferencia exterior requiere sus propios permisos, aislamiento y supervisión.

El consumo de la vía A recae en el equipo que ejecuta realmente el navegador y el modelo, que puede ser local o remoto. Antes de instalar se necesita identificar una implementación compatible, el equipo disponible, el presupuesto de memoria y tiempo y la condición de parada. La referencia general de 16 GB no basta para admitir el experimento.

Comparar esta configuración con Qwen/B permitiría valorar alternativas completas. Para atribuir una diferencia a WebAssembly habría que mantener el modelo y controlar las restantes condiciones. No se presume que un cambio de soporte mejore la calidad de las respuestas ni que conserve su identidad exacta.

## Fuentes

- [Modelo y requisitos oficiales de gpt-oss](https://github.com/openai/gpt-oss).
- [Ficha oficial de gpt-oss-20b](https://developers.openai.com/api/docs/models/gpt-oss-20b).
- [Registro de modelos de Candle en la revisión instalada](https://github.com/huggingface/candle/blob/ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a/candle-transformers/src/models/mod.rs).
- [Modelo de seguridad de WebAssembly](https://webassembly.org/docs/security/) y [operaciones numéricas](https://webassembly.github.io/spec/core/exec/numerics.html).

---

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es) · [Aviso del SV y licencias de terceros](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/controlador-nativo/AVISO_LICENCIAS.json).
