# Conversación nativa con Qwen y conservación por expediente

**Revisión 0.1.2 compilada y comprobada en Codespaces:** seis pruebas unitarias, prueba del proceso HTTP y doce generaciones de comparación. Servicio local iniciado; acceso externo de navegador pendiente de comprobación. [Informe y resultados](verificacion-0.1.2/INFORME.md) · [Acceso y recuperación](RECUPERACION.md). La [revisión candidata](REVISION_0_1_2.md) y [CANDIDATO.json](CANDIDATO.json) conservan el estado histórico anterior a estas pruebas.

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


## Disponibilidad e interrupciones de conexión

El servicio depende de que el Codespace esté iniciado. La plataforma puede detenerlo por inactividad; la conservación de los expedientes en disco no equivale a la disponibilidad permanente del proceso HTTP. Después de reanudar el entorno, se inicia el ejecutable conservado mediante la orden nativa `/workspaces/eio-instalacion-nativa-20260922/target/release/eio-conversacion`. El puerto 3000 debe conservar su visibilidad privada.

La interfaz comprueba el estado HTTP, el tipo de contenido y la presencia de un cuerpo antes de analizarlo como JSON. Una interrupción se presenta como indisponibilidad del servicio. No se reenvían automáticamente peticiones de generación sin confirmación. El texto pendiente se conserva en el almacenamiento de sesión de la pestaña, cuando el navegador lo permite; este borrador no sustituye al registro del expediente. Tras reiniciar el proceso, debe recargarse la página para renovar su identificación de sesión.

Las acciones efectivas del usuario producen una indicación de actividad en la terminal, sin incluir preguntas, respuestas ni nombres de expedientes. Las consultas periódicas de estado permanecen silenciosas. No se modifica el periodo de inactividad de GitHub ni se introduce actividad ficticia para impedir la parada. Véase [la explicación de GitHub sobre actividad e inactividad](https://docs.github.com/en/codespaces/setting-your-user-preferences/setting-your-timeout-period-for-github-codespaces).

La incidencia y el alcance de la comprobación de su corrección se conservan en [INCIDENCIA_CONEXION_2026_09_22.json](INCIDENCIA_CONEXION_2026_09_22.json).


## Titularidad y licencias

Sistema Vectorial SV — © Juan Antonio Lloret Egea, 2026. ITVIA — IA eñ™, ISSN 2695-6411. Se reproduce la licencia **CC BY-NC-ND 4.0** declarada en el [aviso canónico del repositorio](https://github.com/juantoniolloretegea/SV-motor/blob/30683c5ec11d33ecabd6e3defb8110f530290df9/README.md). El aviso visible y las exportaciones incorporan [AVISO_LICENCIAS.json](AVISO_LICENCIAS.json). Qwen3-0.6B conserva Apache 2.0 y Candle conserva MIT o Apache 2.0; este aviso no sustituye sus licencias ni constituye un inventario exhaustivo de dependencias. Tampoco asigna al SV la titularidad de las intervenciones del usuario ni determina los derechos de cada salida generada.

## Interpretación de la sesión recibida el 22 de septiembre

El cotejo de las dos exportaciones recibidas identifica un mismo expediente y una conversación con nueve turnos finalizados. Los primeros cincuenta sucesos se conservan en la exportación posterior de 460 sucesos. La numeración «Conversación 1» corresponde al expediente recién creado; estas exportaciones no aportan evidencia de traslado a otro expediente. Los contextos registrados incorporan progresivamente las intervenciones anteriores.

La muestra revela sustitución del significado de la pregunta, afirmaciones no justificadas, alteración de cantidades aportadas y aceptación de premisas sin comprobación. La configuración observada no acredita aptitud para consejo profesional. El diagnóstico causal permanece abierto: la muestra no separa por sí sola los efectos de la capacidad del modelo, la cuantización y la implementación de inferencia. No se ha comparado esta sesión con un segundo motor ni se ha cambiado el modelo para obtener un resultado favorable.

Las nueve generaciones declaran fin normal y retorno cero. Las entradas aumentan de 126 a 1.772 tokens, con reserva constante de 2.048; ninguna respuesta alcanza esa reserva ni los 600 segundos configurados. La primera salida del proceso pasa de 19,86 a 403,12 segundos. Estos tiempos incluyen carga y procesamiento del contexto, y pueden corresponder al texto de razonamiento antes de la respuesta final. La memoria residente muestreada se sitúa entre 1,21 y 1,96 GiB. La muestra evidencia una limitación operativa; no determina un máximo de contexto utilizable.

La versión 0.1.0 escribía `external_operations: 0` como constante. **Ese campo no acreditaba medición de red ni ausencia de conexiones externas.** La versión 0.1.1 utiliza `null` y declara que no existe instrumentación de red. El número de llamadas a herramientas queda circunscrito a que el servicio no ofrece herramientas al modelo. La interfaz explicita que el contenido no se ha validado. Los sucesos anteriores permanecen intactos y la exportación añade una nota sobre su interpretación.

El [registro de revisión](REVISION_SESION_2026_09_22.json) contiene las identidades de los archivos recibidos, los resultados y sus límites; no reproduce las conversaciones aportadas. Las capturas del titular documentan la recuperación del acceso tras autenticación en una ventana privada. Esta observación apoya una incidencia dependiente de la sesión del navegador, sin demostrar qué cookie o mecanismo originó el HTTP 404.
