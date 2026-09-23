# OpenAI · gpt-oss-20b

**Estado al 23 de septiembre de 2026:** instalado en una instancia separada; no operativo. La carga nativa terminó por `SIGTERM` antes de atender una petición. Causa de la terminación no identificada.

El [resultado y las evidencias del intento del 23 de septiembre](resultados/2026-09-23/RESULTADO.md) documentan la adquisición de los pesos oficiales MXFP4, Rust/Cargo 1.98.0, mistral.rs 0.9.3 para CPU y Harmony 0.0.8. Se utilizó una máquina de 4 núcleos y 16 GB, con BF16. La instancia quedó detenida.

No se generaron respuestas ni se evaluó calidad. Este intento nativo no acredita la vía A ni demuestra imposibilidad general del modelo. Las rutas cruzadas entre modelos y soportes siguen abiertas a trabajo posterior cuando exista motivo concreto.

La [revisión del controlador propio y su contraste con los esquemas A/B](resultados/2026-09-23/REVISION_CONTROLADOR_RUST_20260923.md) reproduce localmente la pérdida del registro de memoria y el cierre al reutilizar una ventana caducada. Identifica además carencias de trazabilidad de señales y de limpieza ante errores. La [corrección del controlador nativo 0.1.0](controlador-nativo/README.md) supera diez comprobaciones funcionales locales con auxiliares Rust. Queda pendiente su ejecución con el motor y los pesos instalados; el emisor del SIGTERM original sigue sin identificar.

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
