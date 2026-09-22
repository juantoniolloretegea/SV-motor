# Integración y verificación de la observación del servicio

22 de septiembre de 2026. Vinculación: S39 y TT-0005.

**OpenTelemetry Rust 0.31.0 está integrado y activo en la conversación 0.1.3.** La versión anterior conservaba sucesos propios, pero no incorporaba este SDK. Se han añadido trazas del supervisor y observación periódica de procesos Linux, con exportación local, límites explícitos y señalización de pérdidas. No se ha activado un recolector externo.

## Objeto y alcance

Esta intervención pertenece al ensayo lateral que prepara evidencias para las necesidades del núcleo, la semántica y la representación intermedia del Lenguaje SV. Su resultado es una observación contrastable de ejecución, conservación y consumo de recursos. No declara completadas las rutas de consulta acotada, la integración doctrinal de la IA ni las condiciones de ciberseguridad del encargo original. La revisión de la adenda queda para el paso siguiente.

La traza de cada inferencia relaciona su identificador de petición con la creación del proceso hijo, las fases que este comunica y su terminación. El observador separado relaciona sus muestras con el PID y el tiempo de creación del proceso. La narración producida por el modelo permanece diferenciada de estas evidencias.

Se utiliza el canal de trazas del SDK. Las medidas Linux se conservan como atributos estructurados de esos registros; no se ha habilitado un exportador independiente de métricas ni un servicio Prometheus.

## Condiciones y comprobaciones

Se utilizó Rust/Cargo 1.98.0, Qwen3-0.6B Q4_K_M en CPU y la revisión fijada de Candle. El cotejo de `Cargo.lock` confirma que todas las identidades anteriores de dependencias permanecen presentes; se añaden OpenTelemetry 0.31.0 y sus dependencias necesarias. Los [criterios previos](../OBSERVABILIDAD_0_1_3.md) se conservaron antes de la ejecución.

Las once pruebas nativas finalizaron correctamente. Incluyen parentesco de trazas, límite del exportador, fallo real de escritura, exclusión de identidad de proceso incorrecta, archivo borrado que permanece abierto, socket de escucha atribuible al proceso y detección de la terminación de un proceso sintético. Se conservaron asimismo los controles anteriores de integridad y exclusión de instancias. Véanse [primera ejecución](PRUEBAS_01.log) y [verificación final](PRUEBAS_02.log).

Una única inferencia sintética, con datos separados y HTTP local en el puerto 3001, permitió comprobar la correspondencia entre el suceso final, la traza y el proceso observado. La pregunta solicitó la palabra «HOLA» y la salida fue «HOLA.». Se reservaron 32 tokens, 120 segundos y semilla 299792458, en modo sin razonamiento, con 117 tokens de entrada. El criterio de esta prueba era técnico; no se realizó una evaluación general de calidad ni de determinismo.

| Magnitud | Resultado observado |
| --- | ---: |
| Duración de la inferencia en el supervisor | 18,746 s |
| Primera salida comunicada | 18,098 s |
| Carga del modelo declarada por el hijo | 2,603 s |
| Procesamiento inicial del contexto declarado por el hijo | 15,124 s |
| Generación declarada por el hijo | 0,550 s |
| Tokens generados | 3 |
| Máxima RSS entre las muestras del supervisor | 1.298.771.968 bytes |
| Muestras del observador durante la prueba | 4 |
| Mayor duración observada del recorrido de procesos | 10,058 ms |
| Memoria residente observada del observador | 5.738.496–6.131.712 bytes |
| Trazas del supervisor, incluido el cierre | 7.622 bytes |
| Trazas del observador, incluido el cierre | 10.617 bytes |
| Errores y descartes de exportación en la prueba | 0 |

El proceso de inferencia apareció en tres muestras con cinco hilos y dos sockets Unix. No se observaron sockets TCP/UDP de ese proceso en esas muestras. Esto describe descriptores locales observados; no demuestra ausencia de comunicaciones entre muestras ni identifica por sí solo el origen de cada descriptor. Los contadores de CPU y entrada/salida se conservan sin atribuirlos exclusivamente a una fase interna.

El coste indicado corresponde a las lecturas del árbol observadas. No incluye por sí solo toda la exportación, la programación de tareas ni el efecto sobre el tiempo de respuesta. No se extrapola a sesiones largas ni sustituye una comparación con y sin instrumentación. Los límites y la información incompleta forman parte de cada muestra.

La inferencia se ejecutó con el binario `ae021c7ac49295464a557fadebe6580c0ca1304a771971514d1b23506e0f07cb`. Después se precisó la distinción entre proceso desaparecido y lectura de identidad no disponible, se repitieron las once pruebas y se compiló el binario instalado `3b29ef59d82145575d5efaba1ec76e50c70bf4a576e2d13008fa3baf6de70e61`. No se realizó una segunda inferencia. Las fuentes de esta última versión y el cierre de dependencias están conservados en la revisión `9dbd003f6c52a4605a947e07a3567d5f1957d3e1`.

## Activación y conservación

La sustitución se realizó sin una petición activa. Se confirmó la parada del servicio anterior y el inicio del proceso 14407, observado por el proceso 14428. El servicio quedó disponible a las 14:47:11 UTC. La comprobación posterior registró dieciocho muestras, exportación íntegra y ausencia de descartes. Las huellas de los dos expedientes se conservaron antes y después de la sustitución; seguían disponibles las seis conversaciones.

La interfaz informa separadamente de conexión y observación. La exportación propia incompleta o un observador sin confirmación reciente impiden admitir nuevas inferencias. No se elimina el historial ni se reenvía automáticamente una petición. Se conserva el ejecutable anterior para recuperación.

El acceso HTTP local está comprobado. Esta intervención no acredita el acceso externo desde el navegador del usuario. El puerto debe permanecer privado. El Codespace estaba detenido al comenzar la inspección y se reanudó; esa observación no permite atribuir la parada a una causa concreta.

## Ensayos recuperados y trabajo posterior

Del medidor histórico se recuperó el contraejemplo del archivo borrado mientras permanece abierto. Del observador preparado para controles nativos se recuperó la separación entre la declaración de cierre y la desaparición observada del proceso. Ambas ideas se han comprobado en este entorno con ensayos nuevos y pequeños.

No se repitió el recorrido masivo de archivos pequeños: su límite temporal ya estaba documentado y el observador nuevo no utiliza ese recorrido. Tampoco se repitieron la comparación de doce conversaciones ni los controles históricos de HTTP, grupos de control o restitución de permisos. Sus resultados y pendientes conservan su ámbito original.

El trabajo posterior puede utilizar identidades, semillas, contextos conservados, tramos de ejecución y observaciones de recursos. Aún quedan por estudiar el determinismo y el coste comparado de la instrumentación en sesiones representativas. La revisión de la adenda deberá determinar qué exigencias de la tubería original están cubiertas por estas evidencias y cuáles permanecen pendientes.

## Evidencias

- [Inferencia y comprobaciones de correlación](INFERENCIA.json).
- [Trazas del supervisor](TRAZAS_SUPERVISOR.jsonl) y [muestras del observador](TRAZAS_OBSERVADOR.jsonl).
- [Sustitución y conservación de expedientes](DESPLIEGUE.json) y [estado posterior](ESTADO_POSTERIOR.json).
- [Compilación final](COMPILACION_FINAL.log), [Rust](RUST.txt), [Cargo](CARGO.txt) y [prueba de integración](INFERENCIA_01.log).
- [Código del exportador](../src/telemetry.rs), [observador Linux](../src/observation.rs) y [comprobación de activación](../examples/comprobar_observabilidad.rs).

Los datos publicados proceden de la prueba sintética y de comprobaciones técnicas de la instalación. No se publican conversaciones privadas ni credenciales.
