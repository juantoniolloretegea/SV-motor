# Ficha técnica · EIO conversación 0.1.3-beta.1

## Identificación y finalidad

Entrega Beta del ejecutable EIO conversación 0.1.3 para Linux x86-64. Su finalidad es la evaluación técnica de una interfaz conversacional con inferencia local al anfitrión, conservación de expedientes y observación de procesos. La aplicación forma parte de una investigación lateral del Lenguaje SV; no es el núcleo, el intérprete de operaciones clínicas ni una versión del universo de conocimiento.

La entrega fija fuentes y archivos. El estado de una instalación y el de sus procesos deben comprobarse en el anfitrión correspondiente; no se deducen de la existencia de esta publicación.

## Composición principal

| Elemento | Versión o identidad | Función |
|---|---|---|
| Aplicación | EIO conversación 0.1.3 | Servicio, supervisión y conservación. |
| Rust / Cargo | 1.98.0 / 1.98.0 | Herramientas de construcción identificadas. |
| Candle | `ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a` | Inferencia numérica en CPU. |
| Qwen | Qwen3-0.6B, Q4_K_M | Modelo de 0,6 mil millones de parámetros; recurso externo identificado. |
| Tokenizers | 0.23.1 | Codificación y recuento de las unidades de texto. |
| Axum / Hyper / Tokio | 0.8.9 / 1.11.1 / 1.53.1 | Capa HTTP y ejecución asíncrona; bibliotecas del programa. |
| OpenTelemetry Rust | API y SDK 0.31.0 | Trazas exportadas localmente. |
| Interfaz | HTML, CSS y JavaScript | Interacción en navegador; incluida en el ejecutable. |

`COMPOSICION.json` amplía este cuadro con las dependencias resueltas. No se cuentan dependencias como servicios. egui, el custodio independiente y la guarda exterior no forman parte de este ejecutable.

## Condiciones operativas

| Característica | Valor o condición |
|---|---|
| Entorno de referencia | Linux x86-64, dos núcleos y 8 GB de memoria en el anfitrión ensayado. No constituye certificación de otras plataformas. |
| Inferencias simultáneas | Una. |
| Contexto operativo | 16 384 unidades, incluidas entrada y reserva de salida. No se ha acreditado rendimiento suficiente en toda esa extensión. |
| Reserva de salida | 2 048 por defecto; admisión entre 32 y 4 096. |
| Tiempo de petición | 600 s por defecto; admisión entre 30 y 1 800 s. |
| Semilla predeterminada | 299792458; no implica determinismo acreditado. |
| Modo con razonamiento | Temperatura 0,6; Top-P 0,95; Top-K 20. |
| Modo directo | Temperatura 0,7; Top-P 0,8; Top-K 20. |
| Preparación del contexto | Fragmentos de 64 unidades; se conservan los antecedentes seleccionados. |
| Control de memoria | Terminación solicitada al superar 6 GiB de RSS muestreada del inferidor. No es una cuota agregada de procesos. |
| Conservación | JSONL con secuencia y SHA-256 encadenadas; sincronización de escritura; límite de 512 MiB. |
| Observación | Proceso Linux separado, periodo de 5 s y límites explícitos de recorrido. |
| Datos profesionales | Sesiones individuales, atribución profesional y políticas de ausencia pendientes. Usar información sintética de ensayo. |

## Evidencia y limitaciones

Las comprobaciones conservadas de 0.1.3 comprenden once pruebas nativas de observación e integridad y una inferencia sintética correlacionada. El informe original distingue la huella de aquella inferencia de la revisión posterior instalada, sobre la que se repitieron las once pruebas. El ensayo DOC-01 identifica por separado sus condiciones y el ejecutable real utilizado; no se atribuyen resultados de un binario a otro.

Los informes de versiones anteriores mantienen su fecha, configuración y alcance. No constituyen una ejecución nueva ni una validación general de esta entrega. Los resultados adversos o incompletos se conservan.

Se han observado respuestas incorrectas y demoras relevantes en el uso experimental. La Beta no acredita competencia clínica ni suficiencia para el consejo profesional. El cumplimiento de un contrato literal de prueba no valida una explicación libre. La bibliografía citada en un documento no se considera consultada por el modelo si no se le ha suministrado su contenido.

## Integridad y recuperación

El manifiesto conserva huellas, tamaños y procedencia. Su finalidad es permitir el cotejo de una copia, de sus recursos y de la evidencia asociada. No constituye una firma independiente ni una garantía de ausencia de defectos.

Los expedientes se almacenan fuera del programa. La aplicación verifica su cadena al abrirlos; una alteración o un archivo incompleto exige intervención y no se repara silenciosamente. La función `--check` puede añadir un cierre por reinicio a una generación pendiente; debe utilizarse sobre una copia cuando se requiere conservar intacto el original.

## Ruta B y continuidad

La interfaz, el proceso de inferencia y la observación realizan una parte de la ruta B. Permanecen pendientes la separación material entre custodia y control, la guarda exterior, el cierre contractual completo y la integración de fuentes, versiones, permisos y veto en la conversación interactiva. DOC-01 aporta un ensayo documental delimitado, sin habilitar esas capacidades pendientes ni modificar semántica o IR.
