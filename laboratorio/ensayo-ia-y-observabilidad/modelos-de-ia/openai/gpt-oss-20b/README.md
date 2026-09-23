# OpenAI · gpt-oss-20b

**Estado al 23 de septiembre de 2026:** instalado en una instancia separada; no operativo. La continuación nativa conserva ocho intentos sin servicio HTTP ni respuesta de inferencia. Se observaron señales externas y paradas instrumentales por tiempo o RSS, diferenciadas en la evidencia. Instancia detenida.

El [primer intento](resultados/2026-09-23/RESULTADO.md) documenta la adquisición de los pesos oficiales MXFP4, Rust/Cargo 1.98.0, mistral.rs 0.9.3 para CPU y Harmony 0.0.8. La [continuación y su diagnóstico](resultados/continuacion-2026-09-23/INFORME.md) conservan medidas durante la carga, variantes de precisión y trazas de señales. Se utilizó la misma instalación de 4 núcleos y 16 GB, con BF16; las variantes no modificaron los pesos originales en disco.

No se generaron respuestas ni se evaluó calidad. Este intento nativo no acredita la vía A ni demuestra imposibilidad general del modelo. Las rutas cruzadas entre modelos y soportes siguen abiertas a trabajo posterior cuando exista motivo concreto.

La [revisión del controlador propio](resultados/2026-09-23/REVISION_CONTROLADOR_RUST_20260923.md) conserva los defectos instrumentales originales. El [controlador de referencia 0.1.3](controlador-nativo/README.md) conserva sus correcciones posteriores y ha sido ejecutado con el motor instalado; supera 16 registros de prueba locales (14 funcionales y dos auxiliares). Ninguna de estas comprobaciones equivale a una inferencia conseguida.

## Corrección posterior y capacidad

Desde [0.1.1](controlador-nativo/verificacion-0.1.1/INFORME.md) se conservan muestras durante la carga, disponibilidad observada, límite explícito de direcciones virtuales y terminación del hijo por Linux cuando desaparece el hilo padre que lo creó. 0.1.3 añade reserva explícita y diagnóstico de observación. Son controles instrumentales, sin contención agregada acreditada.

Los 13 123 MiB anunciados por el cargador representan un inventario, no un máximo RSS. La continuación ya conserva medidas reales del proceso. Dos trazas muestran SIGTERM externo antes de la parada propia; no identifican el servicio emisor ni su motivo. Las variantes Q3K, incluida la cuantización con un trabajador, alcanzaron el límite observado de RSS. El siguiente trabajo debe resolver el consumo de carga mediante una modificación fundamentada del cargador o de la representación, conservando Harmony y la identidad de la configuración. No se atribuye retrospectivamente el SIGTERM original a falta de memoria.

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
