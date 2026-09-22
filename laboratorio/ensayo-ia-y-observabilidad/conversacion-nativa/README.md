# EIO conversación 0.1.3 · Beta

Aplicación experimental de inferencia nativa con conservación por expediente. La [entrega versionada](https://github.com/juantoniolloretegea/SV-motor/releases/tag/eio-conversacion-v0.1.3-beta.1) incluye identificación, ficha técnica, composición, licencias y evidencias.

La versión del programa no determina su disponibilidad instantánea. El servicio depende de la actividad del anfitrión y de sus procesos. Su funcionamiento no acredita aptitud clínica ni conformidad completa de la vía B.

## Realización

Rust 1.98.0, Candle en la revisión `ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a`, Qwen3-0.6B Q4_K_M y el tokenizador identificados en la instalación nativa del 22/09/2026. Servidor Axum/Tokio y página HTML/CSS con JavaScript de interacción. El contenido del modelo se presenta como texto; no se ejecuta HTML procedente de preguntas o respuestas.

Cada petición se ejecuta en un proceso hijo propio. Se admite una generación simultánea y varias conversaciones conservadas. El proceso hijo recibe el contexto completo seleccionado por la conversación; no tiene herramientas de navegación ni ejecución de órdenes. La cancelación termina ese proceso y conserva la salida observada. Esto no equivale a demostrar aislamiento frente a un sistema anfitrión comprometido.

## Contexto y generación

La ventana operativa inicial es de 16.384 unidades de texto, entre entrada y reserva de generación. Se declara como configuración de este despliegue, no como medición de rendimiento ni como máximo intrínseco del modelo. Qwen anuncia 32.768; su consumo y rendimiento no se han acreditado en esta instalación. La lectura inicial se procesa por fragmentos de 64 unidades, sin omitir antecedentes.

El modo con razonamiento utiliza temperatura 0,6, Top-P 0,95 y Top-K 20; el modo de respuesta directa, 0,7/0,8/20. Semilla 299792458. La plantilla separa papeles de sistema, usuario y asistente. El historial reenviado conserva las respuestas finales y marca expresamente las interrupciones; el razonamiento anterior permanece en el expediente pero no se reinserta como antecedente conversacional, conforme a la recomendación de Qwen.

Reserva predeterminada de generación: 2.048 unidades, ajustable entre 32 y 4.096. Tiempo máximo predeterminado: 600 segundos, ajustable entre 30 y 1.800. El supervisor observa la memoria residente del proceso de inferencia y solicita su terminación si supera 6 GiB; es una observación muestreada, no una cuota de grupo de control ni una garantía de máximo agregado. No se modifican permisos administrativos.

Antes de enviar se cuentan las unidades del tokenizador en el contexto exacto y se comprueba la reserva de salida. Si no cabe, la petición queda rechazada y registrada. No hay resúmenes automáticos ni eliminación silenciosa del historial. La capacidad práctica con conversaciones largas queda pendiente de medición específica.

## Conservación y auditoría

Los expedientes se conservan fuera del repositorio, en `/workspaces/eio-conversaciones/datos`, mediante registros JSONL con sucesos identificados, fecha UTC en milisegundos y huellas encadenadas. Las escrituras se sincronizan antes de actualizar el estado visible. Al iniciar se comprueba la cadena y se reconstruyen los expedientes. Un archivo incompleto o alterado impide su apertura normal y se conserva sin reparación silenciosa. Las generaciones sin cierre registrado pasan a interrupción por reinicio, conservando la última salida registrada.

Las huellas locales detectan alteraciones cuando permanece una referencia íntegra; no sustituyen una firma o un anclaje externo. El límite total de conservación es de 512 MiB. No se borran expedientes automáticamente. Cada exportación JSON incluye conversaciones, contextos exactos, configuración, identidad del modelo y sucesos. La copia exportada debe conservarse para recuperar el trabajo si se elimina el Codespace; detenerlo y eliminarlo son operaciones distintas.

Los tiempos son intervalos monotónicos observados desde la admisión. La primera salida incluye carga y preparación. La memoria registrada corresponde al proceso de inferencia y a las muestras obtenidas. No se atribuyen consultas de fuentes ni operaciones externas a una narración del modelo.

## Ejecución

Compilar el paquete con Rust/Cargo 1.98.0 y las dependencias fijadas. Variables admitidas: `EIO_MODELS` para pesos y tokenizador; `EIO_DATA` para conservación; `EIO_ORIGIN` para la dirección de acceso. En Codespaces se obtiene el origen de `CODESPACE_NAME`. El servicio escucha en el puerto 3000, que debe permanecer privado y protegido por la autenticación de GitHub.

El programa reconoce `--check DIRECTORIO` para comprobar y reconstruir registros; esta apertura añade el cierre por reinicio si encuentra una generación pendiente. `--worker` es el modo interno del proceso de inferencia.

Referencia del fabricante: [Qwen3-0.6B](https://huggingface.co/Qwen/Qwen3-0.6B/blob/c1899de289a04d12100db370d81485cdf75e47ca/README.md). Los resultados de la comprobación funcional realizada en Codespaces se recogen en [COMPROBACION_2026_09_22.json](COMPROBACION_2026_09_22.json): conversación libre y continuidad, respuesta con razonamiento, cancelación, rechazo por exceso de contexto y recuperación de cuatro conversaciones tras reiniciar el servicio. Las exportaciones anterior y posterior al reinicio resultaron idénticas. Estas observaciones no acreditan la capacidad práctica con historias largas ni la suficiencia del modelo para un dominio profesional.


## Disponibilidad e interrupciones de conexión

El servicio depende de que el Codespace esté iniciado. La plataforma puede detenerlo por inactividad; la conservación de los expedientes en disco no equivale a la disponibilidad permanente del proceso HTTP. Después de reanudar el entorno, se inicia el ejecutable conservado mediante la orden nativa `/workspaces/eio-instalacion-nativa-20260922/target/release/eio-conversacion`. El puerto 3000 debe conservar su visibilidad privada.

La interfaz comprueba el estado HTTP, el tipo de contenido y la presencia de un cuerpo antes de analizarlo como JSON. Una interrupción se presenta como indisponibilidad del servicio. No se reenvían automáticamente peticiones de generación sin confirmación. El texto pendiente se conserva en el almacenamiento de sesión de la pestaña, cuando el navegador lo permite; este borrador no sustituye al registro del expediente. Tras reiniciar el proceso, debe recargarse la página para renovar su identificación de sesión.

Las acciones efectivas del usuario producen una indicación de actividad en la terminal, sin incluir preguntas, respuestas ni nombres de expedientes. Las consultas periódicas de estado permanecen silenciosas. No se modifica el periodo de inactividad de GitHub ni se introduce actividad ficticia para impedir la parada. Véase [la explicación de GitHub sobre actividad e inactividad](https://docs.github.com/en/codespaces/setting-your-user-preferences/setting-your-timeout-period-for-github-codespaces).

La incidencia y el alcance de la comprobación de su corrección se conservan en [INCIDENCIA_CONEXION_2026_09_22.json](INCIDENCIA_CONEXION_2026_09_22.json).


## Titularidad y licencias

Sistema Vectorial SV — © Juan Antonio Lloret Egea, 2026. ITVIA — IA eñ™, ISSN 2695-6411. Se reproduce la licencia **CC BY-NC-ND 4.0** declarada en el [aviso canónico del repositorio](https://github.com/juantoniolloretegea/SV-motor/blob/30683c5ec11d33ecabd6e3defb8110f530290df9/README.md). El aviso visible y las exportaciones incorporan [AVISO_LICENCIAS.json](AVISO_LICENCIAS.json). Qwen3-0.6B conserva Apache 2.0 y Candle conserva MIT o Apache 2.0; este aviso no sustituye sus licencias ni constituye un inventario exhaustivo de dependencias. Tampoco asigna al SV la titularidad de las intervenciones del usuario ni determina los derechos de cada salida generada.


## Consulta documental y desarrollo pendiente

El [ensayo DOC-01](../resultados/consulta-documental-01/PROTOCOLO.md) utiliza un banco independiente y pasajes identificados de OP-IMM-001. No modifica automáticamente las condiciones de las conversaciones libres. Permanecen pendientes la guarda exterior, la separación de custodia y control y la integración contractual completa de fuentes, versiones y permisos.

## Evidencia de versión

El [informe de 0.1.3](verificacion-0.1.3/INFORME.md) distingue la inferencia sintética inicial de la compilación posterior instalada. La [comparación 0.1.2](verificacion-0.1.2/INFORME.md) conserva su alcance histórico. Los resultados técnicos, de contenido y de disponibilidad se evalúan por separado.
