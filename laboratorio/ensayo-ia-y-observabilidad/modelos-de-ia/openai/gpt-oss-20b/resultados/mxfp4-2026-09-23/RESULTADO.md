# gpt-oss-20b: carga con expertos MXFP4 y continuación acotada

**23 de septiembre de 2026. Resultado: no se completó la carga ni se obtuvo respuesta de inferencia. Instancia detenida.** Los intentos 13 y 14 utilizaron la misma configuración numérica; el segundo amplió el tiempo permitido de carga. Se conserva el resultado adverso y se suspende esta línea de ejecución para evaluación.

## Configuración identificada

| Elemento | Identificación |
|---|---|
| Anfitrión | GitHub Codespaces, CPU, 4 núcleos, 16 GB de RAM y 32 GB de almacenamiento; sin intercambio disponible. |
| Modelo | gpt-oss-20b, revisión `6cee5e81ee83917806bbde320786a8fb61efebee`; archivos oficiales conservados. |
| Motor | mistral.rs 0.9.3, revisión `24dbf5c256f232176ee5949485ba264049407fbe`; ejecutable SHA-256 `0bd54bd0d17eb79be22b21cccaac0a3f32add5918f93ea162a888feccd7a2158`. |
| Controlador | Candidata 0.1.11, compilada con Rust 1.98.0; SHA-256 `57d226cca53ed9b30ebd47bf1bfdc28d14b2f875b100da217b322a7f84fba6a5`. |
| Representación solicitada | Expertos MXFP4 mediante topología explícita; Q8_0 para las restantes capas; un trabajador de cuantización. Precisión base BF16. |
| Servicio previsto | `127.0.0.1:8089`, contexto máximo 1024, una secuencia, sin interfaz del motor. No llegó a habilitarse. |
| Formato de conversación | Harmony 0.0.8, dependencia Rust del motor; no se modificó. |
| Supervisión | Límite de direcciones virtuales de 32 GiB, reserva instrumental de 512 MiB, observación de memoria anónima y compartida, plazos, registro durante la ejecución y terminación del hijo. No acredita contención agregada ni guarda exterior. |

La [preparación de la candidata](../ventana-acotada-2026-09-23/RESULTADO.md) conserva fuentes, fundamento de la selección MXFP4, compilación y banco secuencial de 17 registros, incluidos dos auxiliares. Las pruebas locales no acreditan la viabilidad de la carga. El motor anunció la lectura de pesos para cuantización, pero no completó esa fase: no se certifica una representación final íntegramente cargada.

## Resultados observados

Tiempos UTC. Las duraciones incluyen la preparación interna del controlador. Los máximos de memoria son valores muestreados, no máximos exactos.

| Medida | Intento 13 | Intento 14 |
|---|---:|---:|
| Inicio del registro | 17:47:08.205 | 17:59:51.545 |
| Final del registro | 17:58:57.199 | 18:05:46.903 |
| Duración | 708,994 s | 355,359 s |
| Plazo de carga | 700 s | 1100 s |
| Disponibilidad inicial | 14 679 425 024 B | 14 636 974 080 B |
| Pico RSS muestreado | 13 125 398 528 B | 13 940 629 504 B |
| Máximo de memoria anónima en muestras persistidas | 8 608 251 904 B | 13 451 169 792 B |
| Mínima disponibilidad en muestras persistidas | 5 981 696 000 B | 1 204 113 408 B |
| Muestras RSS válidas / no disponibles | 11 334 / 0 | 5694 / 7 |
| Muestras de recursos persistidas | 660 | 334 |
| Servicio comprobado / petición HTTP / respuesta | No / no / no | No / no / no |
| Estado final del hijo | SIGTERM; cierre confirmado | SIGKILL; cierre confirmado |

**Intento 13:** venció el plazo de carga. La traza registra el SIGTERM enviado por el controlador y la terminación posterior. No activó el corte instrumental por memoria. Una observación durante la carga muestra al motor en estado `Dl`, con 19 segundos de CPU tras 8 minutos y 51 segundos transcurridos y 7 258 099 712 bytes leídos del almacenamiento. Es compatible con una demora de entrada/salida; una sola observación no identifica todos sus factores.

**Intento 14:** varios hilos aparecen terminados por SIGKILL desde las **18:05:46.233919**. El controlador registra después un fallo al enumerar `/proc/10867/fd` y envía SIGTERM de limpieza a las **18:05:46.600909**. La traza no contiene una llamada propia que envíe SIGKILL. Por tanto, el SIGTERM de limpieza no explica el comienzo de esta terminación. La identidad del emisor de SIGKILL y su motivo no quedan establecidos. El error de permisos es el error observado por el controlador, no una causa raíz demostrada.

La última muestra persistida del intento 14 conserva 1 204 113 408 B disponibles. Los contadores del grupo visible mantienen `oom=0` y `oom_kill=0`; la lectura del registro del núcleo fue denegada y su error se conserva. Esto **no confirma ni descarta agotamiento de memoria en ámbitos no observados**. No existe evidencia de un desbordamiento de memoria como defecto de programación. Tampoco se demuestra que el modelo completo pueda funcionar dentro de este entorno.

El segundo intento avanzó inicialmente con mayor rapidez. La reutilización de páginas en caché es una explicación plausible, no una comparación controlada ni una medida concluyente del rendimiento del cargador.

## Cierre y alcance

Ambos procesos hijos terminaron. La comprobación posterior no encontró los PID del segundo controlador y motor ni un servicio escuchando en el puerto 8089. El paquete se recuperó y su SHA-256 coincidió con el calculado en origen. La interfaz de Codespaces confirmó **«Codespace is stopped» a las 18:09:33 UTC**, antes del límite exterior de la ventana, 18:24:56 UTC.

La configuración no resolvió la carga en este ensayo. No se realizó otra ejecución. TT-0012 conserva pendiente la obtención de una respuesta real, con esta continuación detenida para evaluación. El fallo ocurrió antes del servicio y de procesar una conversación: **estos resultados no fundamentan atribuirlo a Harmony ni sustituir esa biblioteca como solución del fallo**. Una continuación requeriría una modificación concreta del recorrido de carga o una explicación contrastada de la terminación; ampliar plazos por sí solo no resuelve el resultado del intento 14.

Qwen/B conserva su cierre como realización parcial. Este ensayo nativo no acredita la vía A, calidad del contenido ni integración conforme con el SV.

## Evidencias reproducibles

- [Paquete original](EVIDENCIAS_MXFP4_20260923.tar.gz), SHA-256 `35f6cb8bf4e111c4366e9382de745997ce82b2f0784d5750e3f2a6e76333fdf5`. Contiene registros JSONL, trazas, salidas, topología, observaciones de procesos y memoria y comprobaciones finales.
- [Resumen 13](RESUMEN_13.json) y [resumen 14](RESUMEN_14.json), derivados con este [lector Rust](resumir-intento.rs). Conservan huellas de los registros, identidad de ejecutables, configuración, límites y resultado original.
- [Confirmación visual de parada](PARADA.jpg). La captura confirma el estado final observado; no mide por sí sola la hora de la transición interna.
- [Manifiesto SHA-256](SHA256SUMS.txt).

## Licencias

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es). Los componentes de terceros conservan sus licencias y atribuciones: [aviso de la candidata](../ventana-acotada-2026-09-23/candidata-0.1.11/AVISO_LICENCIAS.json). No se distribuyen pesos ni ejecutables del motor en este depósito.
