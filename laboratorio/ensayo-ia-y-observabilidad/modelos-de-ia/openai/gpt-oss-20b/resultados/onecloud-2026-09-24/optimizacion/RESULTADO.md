# Optimización CPU MXFP4 y ejecución residente: resultado experimental

Registro documental: 2026-09-24T12:36:09.219Z. Unidad de ejecución experimental. Seguimiento S39, revisión 18; Acta004 §18; RETP-2026-271; PTA-2026-013 y PTA-SVM-004. Verificación acotada de una investigación lateral; sin modificación del núcleo del lenguaje.

## Dictamen

Se cumple el criterio fijado antes de ejecutar: cinco casos correctos, incluidas tres repeticiones correctas del caso aritmético; cierre sin errores; mediana candidata no superior al 50 % de la referencia. La mediana pasa de **94,002 s a 18,168 s**: factor **5,174**, reducción **80,673 %**. La aceptación es local a este banco, configuración y anfitrión.

La condición de paso a llama.cpp no se activa. No se compila ni ejecuta esa alternativa en esta campaña. No se adquieren recursos adicionales ni se crean ramas distintas de main.

## Diseño y precedencia del protocolo

La ventana autorizada comienza el 24/09/2026 a las 11:58 UTC (13:58 Europe/Madrid). Primera fase hasta 12:58 UTC; alternativa condicional hasta 13:58 UTC. Preparación y compilación cuentan dentro del plazo. [Protocolo](PROTOCOLO.md) publicado en el commit 7a3a353f55f8fddf5898eae9c82feb2caeac4677, a las 12:03 UTC, antes de inferencia. Una ruta documental errónea se corrigió en 388933be4ac622e2f773939413ce0f4609f6b8f4 sin cambiar contenido, casos, oráculos ni umbral; se conserva el historial y el [registro de incidencias](REGISTRO_INCIDENCIAS.md).

Comparación secuencial de dos modos del **mismo ejecutable**, con idénticos pesos, tokenizador, petición C01, parámetros y número declarado de trabajadores. Referencia: tres peticiones en una carga. Candidata: tres repeticiones C01 y cuatro casos adicionales en una carga. Una sola inferencia activa. Ambas sesiones residentes se detienen al acabar su banco.

Los límites efectivos son CPU, BF16 solicitado, contexto 1024, una secuencia, caché de prefijos 0, atención paginada desactivada, temperatura 0, C01 máximo 16 tokens y casos adicionales máximo 32. Entrada Harmony literal con canal final abierto, mediante /v1/completions. Doce trabajadores declarados para Candle y Rayon. BF16 solicitado no implica uniformidad de todos los cálculos internos: se conserva la ruta de atención del motor y su limitación previamente observada.

Anfitrión: máquina virtual Ubuntu 26.04, KVM, 12 CPU virtuales expuestas sobre AMD EPYC 7502, 64 GB nominales de RAM y almacenamiento nominal de 640 GB; ejecución CPU. No se equiparan estas CPU virtuales con doce núcleos físicos dedicados ni se certifica el rendimiento del soporte físico del disco.

## Intervención e identidades

Motor mistral.rs 0.9.3, revisión 24dbf5c256f232176ee5949485ba264049407fbe. Se conserva la corrección de difusión de entrada compartida entre los expertos seleccionados. La optimización sustituye copias completas de bloques y escalas contiguos en CPU por referencias al almacenamiento; reparte las celdas de salida independientes con Rayon y mantiene el orden de acumulación de cada resultado. No se cambian pesos ni cuantización. La comparación conjunta no separa el efecto de retirar copias del efecto de paralelizar.

- GGUF MXFP4, SHA-256: 27cd6c432c7672cb812a92f611cf3ba7bbc35928262bb1e1253ff4ee6ae35901.
- Motor comparado, SHA-256: f6ec0bb908e18ced633d66d6a762a915349281bdbb1397991b268fe487bcc418.
- Fuente MXFP4 instrumentada, SHA-256: ebddd367e2cc348da56001fd3f1aac5cb271739bb01e2bc61371881aca3ecb3e.
- Controlador residente, SHA-256: a5f66a13eabed2f548c81c82872e4c34b20650fd4c506c3df57512c6d867d8e1.
- Archivo de fuentes del motor, SHA-256: aadf426b6489599b8dfff09c05b3fa8637f19ccc228a020dd0a33d65d761bcc0.

Rust 1.98.0 y Cargo 1.98.0. Compilación --offline --locked --release, sin características predeterminadas, ocho trabajos. El [registro de compilación](evidencias/COMPILACION_MOTOR.log) conserva identidades y comandos; [COMPILAR.sh](COMPILAR.sh) permite repetir el procedimiento sobre el archivo de fuentes y la caché identificados. El análisis experimental se realiza con instrumentos Rust; los scripts CJS se emplean para edición y formato documental. No se usa Python.

## Resultados observados

| Caso | Referencia, s | Paralelo, s | Respuesta de la candidata |
|---|---:|---:|---|
| C01-R1 | 94,434 | 18,436 | Hay cinco elementos en total. |
| C01-R2 | 94,002 | 18,168 | Hay cinco elementos en total. |
| C01-R3 | 93,126 | 17,826 | Hay cinco elementos en total. |
| C02 | — | 14,986 | 3 |
| C03 | — | 14,403 | AX-17 |
| C04 | — | 18,253 | 2, 5, 9 |
| C05 | — | 23,513 | No, sin conocer la cantidad inicial no se puede determinar el total exacto. |

Las tres respuestas C01 de referencia son también «Hay cinco elementos en total.». Todas las respuestas terminan con stop; no hay truncamiento por límite de generación. La latencia se mide alrededor de cada petición HTTP y excluye carga e inicialización. No es tiempo total desde el arranque ni rendimiento de una conversación larga.

Referencia: finalización 2026-09-24T12:24:00.800Z, duración total del controlador 337,169 s. Candidata: finalización 2026-09-24T12:27:20.066Z, duración total 160,163 s. El cierre del banco se produce dentro de la primera ventana. Los tiempos totales incluyen preparación, comprobaciones, carga, banco y cierre; no se presentan como mediciones puras de carga. La residencia evita recargar entre peticiones de la misma sesión; su ventaja frente a cargar por cada petición no se cuantifica mediante un ensayo separado.

La [validación semántica](VALIDACION_SEMANTICA.json) explicita el juicio de cada caso. El campo nativo content_validation conserva el valor no_realizada: describe la función del controlador, que no evalúa semántica. El juicio posterior no modifica los originales.

## Controles y recursos

Tres pruebas numéricas específicas aprobadas: equivalencia bit a bit de referencia y paralelo con entradas compartidas, bidimensionales y expandidas, con y sin sesgo; rechazo de índice de experto fuera de rango; regresión de la entrada compartida entre cuatro expertos. Nueve guardas del controlador aprobadas. Cargo check del paquete mistralrs-quant finaliza correctamente a las 12:27:57 UTC. No se ejecuta ni se declara aprobada la suite integral del motor.

La [auditoría instrumental Rust](evidencias/AUDITORIA_INSTRUMENTAL.json) verifica 26 condiciones sobre los originales: cargas, procesos, solicitudes, respuestas, identidad de ejecutables, igualdad de peticiones comparables, HTTP 200, parámetros, orden temporal, ausencia de truncamiento, muestreo y cierre. Todas aprobadas. Esta auditoría no añade nuevos ensayos del modelo.

Cuota exterior por sesión: 32 GiB, swap 0, 256 tareas, red privada y escucha local. RLIMIT_AS interior: 32 GiB por proceso; no equivale a RSS agregada. Pico RSS muestreado del hijo: referencia 25194369024 B; candidata 24807313408 B. Muestras: 6245 y 2875, sin muestras ausentes. No se equipara RSS con memory.peak del cgroup: el ámbito y la imputación de páginas compartidas y caché difieren.

Se conservan contadores del cgroup real antes y después del controlador, antes de retirar el servicio. No registran eventos max, oom u oom_kill. Son observaciones de estas ejecuciones; no constituyen una prueba de saturación de la cuota. [Referencia](evidencias/REFERENCIA_CONTENCION_FINAL.txt) y [candidata](evidencias/PARALELO_CONTENCION_FINAL.txt). El controlador mantiene la limitación conocida de resolución interna del cgroup; la cuota y lectura exterior documentan el grupo efectivo.

Ambos hijos se detienen mediante la señal prevista de cierre, sin errores. Los servicios y temporizadores del ensayo se verifican detenidos en [CIERRE_SERVICIOS.txt](evidencias/CIERRE_SERVICIOS.txt). La máquina virtual permanece disponible; no queda un modelo residente ni una inferencia de esta campaña en marcha.

## Límites y continuidad

Banco pequeño y sintético, tres repeticiones por modo para una única tarea de rendimiento, orden fijo referencia–candidata, sin aleatorización ni control exhaustivo de carga externa y cachés. La precisión bit a bit se acredita para las entradas de las pruebas específicas, no para cualquier tensor posible. El factor observado no se extrapola a otros modelos, contextos, máquinas ni concurrencia.

El contraste corrige la limitación de comparar ejecutables diferentes en OC01/OC02: aquí solo se selecciona el modo de la rutina dentro del mismo binario. No separa ambas optimizaciones ni constituye una certificación causal universal. Los perfiles por operación se conservan en motor.log y en ambos resúmenes, en nanosegundos de tiempo transcurrido; no son consumo acumulado de CPU. Para comparar prellenado C01, la categoría tokens=60 comprende las mismas 144 llamadas en cada modo. Los totales tokens=1 abarcan bancos de distinto tamaño y no deben compararse como si fueran equivalentes.

Se acepta esta revisión como candidata para una fase posterior de tareas representativas y contexto progresivo, con nuevos oráculos y presupuestos. Esa fase no se ejecuta aquí. S39 permanece abierto; TT-0012 conserva su cierre material anterior. Se conservan datos adversos históricos y las licencias y atribuciones originales obligatorias; la documentación nueva usa denominaciones funcionales.

## Custodia y recuperación

[ESTADO.json](ESTADO.json) fija decisión, plazos y siguiente acción. [MANIFIESTO_SHA256.txt](MANIFIESTO_SHA256.txt) permite cotejar los archivos de este directorio. Los originales HTTP, JSONL, perfiles, compilaciones, fuentes y cierres permanecen versionados en main. No se han creado ramas temporales. Las copias de trabajo anteriores del entorno no se han restablecido ni sobrescrito.

Para recuperar la instalación se conservan fuentes, compilador, caché, ejecutables y pesos en la máquina adquirida, bajo /opt/sv-lab/optimizacion-20260924 y /opt/sv-lab/onecloud-20260924. Antes de cualquier nueva inferencia se debe comprobar el estado real de los servicios, las huellas y establecer un plazo nuevo: el límite de SESION.sh corresponde exclusivamente a esta campaña y no debe reutilizarse sin actualizarlo explícitamente. GitHub conserva fuentes y evidencia; los pesos no se incorporan al repositorio. La recuperación no requiere comprar otro recurso.
