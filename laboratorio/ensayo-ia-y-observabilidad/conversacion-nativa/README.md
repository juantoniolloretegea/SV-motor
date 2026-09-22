# Conversación nativa con Qwen y conservación por expediente

Aplicación experimental del ensayo de inteligencia artificial y observabilidad, en SV-motor. Permite entradas libres, conversaciones de varios turnos, conservación por expediente y consulta de los sucesos asociados. Su existencia no constituye un dominio clínico ni modifica la semántica o la IR del Lenguaje SV.

## Realización

Rust 1.98.0, Candle en la revisión `ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a`, Qwen3-0.6B Q4_K_M y el tokenizador identificados en la instalación nativa del 22/09/2026. Servidor Axum/Tokio y página HTML/CSS con JavaScript de interacción. El contenido del modelo se presenta como texto; no se ejecuta HTML procedente de preguntas o respuestas.

Cada petición se ejecuta en un proceso hijo propio. Se admite una generación simultánea y varias conversaciones conservadas. El proceso hijo recibe el contexto completo seleccionado por la conversación; no tiene herramientas de navegación ni ejecución de órdenes. La cancelación termina ese proceso y conserva la salida observada. Esto no equivale a demostrar aislamiento frente a un sistema anfitrión comprometido.

## Contexto y generación

La ventana operativa inicial es de 16.384 unidades de texto, entre entrada y reserva de generación. Se declara como configuración de este despliegue, no como medición de rendimiento ni como máximo intrínseco del modelo. Qwen anuncia 32.768; no se ofrece ese máximo sin contrastar su consumo en la máquina de 8 GB. La lectura inicial se procesa por fragmentos de 64 unidades, sin omitir antecedentes.

El modo con razonamiento utiliza temperatura 0,6, Top-P 0,95 y Top-K 20; el modo de respuesta directa, 0,7/0,8/20. Semilla 299792458. La plantilla separa papeles de sistema, usuario y asistente. El historial reenviado conserva las respuestas finales y marca expresamente las interrupciones; el razonamiento anterior permanece en el expediente pero no se reinserta como antecedente conversacional, conforme a la recomendación de Qwen.

Reserva predeterminada de generación: 2.048 unidades, ajustable entre 32 y 4.096. Tiempo máximo predeterminado: 600 segundos, ajustable entre 30 y 1.800. El supervisor observa la memoria residente del proceso de inferencia y solicita su terminación si supera 6 GiB; es una observación muestreada, no una cuota de grupo de control ni una garantía de máximo agregado. No se modifican permisos administrativos.

Antes de enviar se cuentan las unidades del tokenizador en el contexto exacto y se comprueba la reserva de salida. Si no cabe, la petición queda rechazada y registrada. No hay resúmenes automáticos ni eliminación silenciosa del historial. La capacidad práctica con conversaciones largas queda pendiente de medición específica.

## Conservación y auditoría

Los expedientes se conservan fuera del repositorio, en `/workspaces/eio-conversaciones/datos`, mediante registros JSONL con sucesos identificados, fecha UTC en milisegundos y huellas encadenadas. Las escrituras se sincronizan antes de actualizar el estado visible. Al iniciar se comprueba la cadena y se reconstruyen los expedientes. Un archivo incompleto o alterado impide su apertura normal y se conserva sin reparación silenciosa. Las generaciones sin cierre registrado pasan a interrupción por reinicio, conservando la última salida registrada.

Las huellas locales detectan alteraciones cuando permanece una referencia íntegra; no sustituyen una firma o un anclaje externo. El límite total de conservación es de 512 MiB. No se borran expedientes automáticamente. Cada exportación JSON incluye conversaciones, contextos exactos, configuración, identidad del modelo y sucesos. La copia exportada debe conservarse para recuperar el trabajo si se elimina el Codespace; detenerlo y eliminarlo son operaciones distintas.

Los tiempos son intervalos monotónicos observados desde la admisión. La primera salida incluye carga y preparación. La memoria registrada corresponde al proceso de inferencia y a las muestras obtenidas. No se atribuyen consultas de fuentes ni operaciones externas a una narración del modelo.

## Ejecución

Compilar el paquete con Rust/Cargo 1.98.0 y las dependencias fijadas. Variables admitidas: `EIO_MODELS` para pesos y tokenizador; `EIO_DATA` para conservación; `EIO_ORIGIN` para la URL HTTPS. En Codespaces se obtiene el origen de `CODESPACE_NAME`. El servicio escucha en el puerto 3000, que debe permanecer privado y protegido por la autenticación de GitHub.

El programa reconoce `--check DIRECTORIO` para comprobar y reconstruir registros; esta apertura añade el cierre por reinicio si encuentra una generación pendiente. `--worker` es el modo interno del proceso de inferencia.

Referencia del fabricante: [Qwen3-0.6B](https://huggingface.co/Qwen/Qwen3-0.6B/blob/c1899de289a04d12100db370d81485cdf75e47ca/README.md). Los resultados de la comprobación funcional realizada en Codespaces se recogen en [COMPROBACION_2026_09_22.json](COMPROBACION_2026_09_22.json): conversación libre y continuidad, respuesta con razonamiento, cancelación, rechazo por exceso de contexto y recuperación de cuatro conversaciones tras reiniciar el servicio. Las exportaciones anterior y posterior al reinicio resultaron idénticas. Estas observaciones no acreditan la capacidad práctica con historias largas ni la suficiencia del modelo para un dominio profesional.
