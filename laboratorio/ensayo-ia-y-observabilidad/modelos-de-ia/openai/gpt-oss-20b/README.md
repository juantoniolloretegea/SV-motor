# OpenAI · gpt-oss-20b

**Estado al 23 de septiembre de 2026:** instalado en una instancia separada; no operativo. Los últimos intentos, 13 y 14, no completaron la carga ni habilitaron el servicio. Instancia detenida; continuación suspendida para evaluación.

El [primer intento](resultados/2026-09-23/RESULTADO.md) documenta los pesos oficiales MXFP4, Rust/Cargo 1.98.0, mistral.rs 0.9.3 para CPU y Harmony 0.0.8. La [continuación inicial](resultados/continuacion-2026-09-23/INFORME.md) conserva variantes y trazas; la [rectificación instrumental](resultados/continuacion-2026-09-23/RECTIFICACION_CONTROLADOR.md) distingue memoria anónima y respaldada por archivos, corrige la confirmación de escritura y documenta la derivación de solicitudes Q2K/Q3K a Q4_0.

La [continuación con expertos MXFP4](resultados/mxfp4-2026-09-23/RESULTADO.md) ensayó la candidata 0.1.11, conservando expertos MXFP4 mediante topología y solicitando Q8_0 para el resto. El intento 13 finalizó por el plazo de carga; el 14, por SIGKILL observado antes del SIGTERM de limpieza del controlador. No se identificó el emisor ni se demostró su causa. Los registros no permiten concluir agotamiento de memoria ni viabilidad del modelo completo en este entorno.

No hubo petición de inferencia ni evaluación de contenido. La terminación durante la carga no aporta fundamento para atribuir el fallo a Harmony. El objetivo de obtener una respuesta real sigue pendiente; no se prolonga automáticamente esta serie de ejecuciones. Este recorrido nativo no acredita la vía A ni imposibilidad general del modelo.

## Controlador y capacidad

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
