# gpt-oss-20b — intento nativo del 23 de septiembre de 2026

**Resultado: adquisición e instalación completadas; ejecución no conseguida.** La carga de pesos terminó por `SIGTERM` antes de habilitar el servicio. No se emitió ninguna petición de inferencia ni se generó una respuesta. La causa de la señal no está identificada.

Watson realizó este intento por autorización directa de Juan Antonio Lloret Egea en el chat Lenguaje Prog. Astra XXI. No fue una ejecución encargada a Holmes. Se respetó una ventana máxima de treinta minutos y el presupuesto de gasto adicional cero, con bloqueo de gasto de Codespaces activado. La comprobación de parada exterior se efectuó a los 18 minutos y 25 segundos de la solicitud de creación.

## Entorno y componentes efectivamente instalados

| Elemento | Valor observado |
|---|---|
| Instancia independiente | Nombre visible `openai-gpt-oss-20b`; identificador permanente `didactic-chainsaw-p49vp5w7qg62r7j7` |
| Máquina | 4 núcleos; 16 GB de RAM; 32 GB de almacenamiento; Europe West |
| Repositorio de partida | SV-motor, `main`, `2389ac77887dbe7a19ea7867bc834f018f666622` |
| Rust / Cargo | 1.98.0 / 1.98.0, comprobados mediante sus salidas de versión |
| Motor | Ejecutable oficial para CPU de mistral.rs 0.9.3; no compilación local completa del motor |
| Etiqueta del motor | `v0.9.3`, commit `24dbf5c256f232176ee5949485ba264049407fbe` |
| Integridad del archivo del motor | SHA-256 `8d192e2efb0714a6e3cd10374700b3c79142bacba8f23f2d5d5ea465771aac46`, cotejado con el activo publicado |
| Modelo | `openai/gpt-oss-20b`, revisión `6cee5e81ee83917806bbde320786a8fb61efebee`, distribución oficial MXFP4 |
| Pesos | Tres fragmentos, 13.761.316.904 bytes; sus tres SHA-256 concordaron |
| Harmony | Dependencia oficial `openai-harmony` 0.0.8 en Cargo.lock de la etiqueta del motor; vocabulario local cotejado |
| Configuración final | CPU, BF16; una secuencia; caché de prefijos desactivada; PagedAttention desactivada; servicio en 127.0.0.1:8089 |
| Contexto | `max-seq-len=1024` para la asignación automática de dispositivos; no constituye un límite duro del contexto del modelo |
| Petición prevista | Un caso sintético aritmético en español, máximo 96 tokens y esfuerzo bajo; no llegó a emitirse |
| Ubicación | `laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/instalacion-local/`, excluida localmente de Git |

La instalación empleó un ejecutable Rust distribuido por el proyecto del motor y un programa de control propio en Rust. No se emplearon Python, Batch ni PowerShell. No se habilitaron herramientas de búsqueda, ejecución de código o ejecución de órdenes para el modelo.

## Secuencia observada — horas UTC

| Hora | Hecho |
|---|---|
| 06:08:11 | Solicitud de creación de la instancia separada. |
| 06:13:07 | Inicio del programa Rust de instalación y registro. |
| 06:18:51 | Adquisición terminada, con cotejo de pesos, tokenizador y vocabulario. |
| 06:18:51 | Primer arranque rechazado: `cannot seed the CPU rng with set_seed`. |
| 06:19:59–06:20:00 | Segundo arranque rechazado: `max_model_len=1024 is not supported by this model loader`. |
| 06:20:56 | Arranque corregido: BF16, asignación de las 24 capas a CPU e inicio de carga de pesos. |
| 06:21:45 | Terminación del proceso durante la carga: señal 15, `SIGTERM`, a los 49 segundos. |
| 06:26:07 | Solicitud exterior de parada de la instancia en GitHub. |
| 06:26:35 | GitHub dejó de mostrar la instancia como activa; el menú dejó de ofrecer «Stop codespace». |

Los dos primeros arranques corrigieron errores de la invocación preparada por Watson. Se conservaron sus salidas originales y no se repitió la descarga. La frase «primera carga efectiva» que aparece en el registro previo al segundo arranque expresa la intención de esa orden: el error de argumento acredita que tampoco llegó entonces a cargar pesos. La carga efectiva comenzó en el tercero.

El motor informó un inventario de 13.123 MiB para la asignación. Esa cifra es una estimación del cargador, no una medición del máximo de memoria. El programa de control no volcó el máximo muestreado al terminar anticipadamente; por tanto, no se declara un pico de RSS. El contador `memory.events` consultado después presentaba valores `oom=0` y `oom_kill=0`; no demuestra ausencia de presión de memoria en toda la máquina. La lectura de `dmesg` fue denegada. No se atribuye el `SIGTERM` al agotamiento de memoria, al modelo ni a un mecanismo concreto sin evidencia adicional.

## Cierre y alcance

La instancia quedó detenida y conserva la instalación. No se efectuaron nuevas cargas tras el `SIGTERM`. La ausencia de procesos `mistralrs` se comprobó antes de la parada exterior. Qwen no se inició, detuvo ni modificó durante este intento. No se crearon ramas ni se modificó el mapa de continuidad.

Este resultado cierra el tanteo práctico de esta configuración como **no operativo y sin inferencia evaluada**. No acredita la imposibilidad general de gpt-oss-20b en 16 GB ni permite juzgar la calidad de sus respuestas. Tampoco verifica la vía A: se intentó ejecución nativa, correspondiente a la familia de la vía B. La interfaz de Qwen, OpenTelemetry, DuckDB y NCBI no se integraron en este intento.

No se alteró el presupuesto de gasto cero. Detener una instancia finaliza su cómputo, pero los archivos conservados siguen ocupando la cuota de almacenamiento. Se mantiene el bloqueo económico existente.

## Evidencias

El [paquete original descargado de la instancia](EVIDENCIAS_GPT_OSS_20260923.tar.gz) tiene SHA-256 `8521ed8ecedc6567c1ae802607b49f83d0669c8bfc7b5515f14c2338feaadee1`; la huella obtenida localmente coincide con la de la instancia. Contiene las órdenes y sus salidas, las tres versiones del programa Rust, las versiones instaladas, las comprobaciones de integridad y los metadatos del modelo. Los pesos y el ejecutable del motor no se incluyen en el paquete de evidencias. Se ofrecen también el [registro cronológico](SUCESOS.txt) y la [salida de la carga efectiva](motor-ejecucion.log) para lectura directa.

- `evidencias/SUCESOS.txt`: registro cronológico original.
- `evidencias/arranque-argumento-semilla.log` y `arranque-argumento-contexto.log`: errores de invocación.
- `evidencias/motor-ejecucion.log`: última salida del motor durante la carga efectiva.
- `evidencias/procesos-final.txt`: consulta final sin procesos `mistralrs`.
- `evidencias/git-final.txt`: el único archivo ajeno al ensayo que figuraba sin seguimiento era `poetry.lock`, ya presente antes de la instalación del modelo; se conservó intacto.

## Referencias de distribución

- [mistral.rs 0.9.3](https://github.com/EricLBuehler/mistral.rs/releases/tag/v0.9.3).
- [Cargo.lock de la etiqueta usada](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/Cargo.lock).
- [Revisión oficial de los pesos](https://huggingface.co/openai/gpt-oss-20b/tree/6cee5e81ee83917806bbde320786a8fb61efebee).

Documento técnico del ensayo del Sistema Vectorial SV. Dirección: Juan Antonio Lloret Egea. Los componentes de terceros conservan sus licencias y autorías; su instalación no modifica el licenciamiento del SV.
