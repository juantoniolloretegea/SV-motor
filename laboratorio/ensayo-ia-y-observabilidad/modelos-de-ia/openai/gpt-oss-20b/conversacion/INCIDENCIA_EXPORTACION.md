# Incidencia de exportación y continuación de la comparación

Fecha: 24/09/2026. Unidad de ejecución experimental. Registro previo a la continuación.

## Hechos conservados

La campaña de contexto finalizó con cuatro recuperaciones de datos conversacionales correctas y cuatro recuperaciones de identificador correctas entre 268 y 3096 tokens de entrada. El diálogo secuencial D completó cuatro de ocho condiciones; las cuatro restantes no se admitieron por el plazo disponible. Las cuatro consultas documentales DOC01–DOC04 terminaron con objeto JSON, fuente y cita conformes. No representan una validación integral del universo de inmunología.

La comparación de preguntas recuperadas comenzó con R02-01 a las 16:34:20 UTC. El motor produjo 256 tokens, con terminación por límite de generación y respuesta parcial. R02-02 fue rechazada antes de admitir otra inferencia, a las 16:37:21. El controlador terminó con error. La ventana comparativa anterior, iniciada a las 16:26:33, venció a las 17:26:33 UTC; no se modifica retroactivamente.

El resultado íntegro R02-01 contiene 35 959 bytes en su valor `phase_timings` serializado. Ese valor incorpora la respuesta del motor y las probabilidades por token. Su duplicación en el atributo de cierre OpenTelemetry sobrepasó la cota individual de 32 768 bytes. El exportador pasó a estado no íntegro y la guardia bloqueó la siguiente petición. La cota total de 10 MiB no se había alcanzado: figuraban 149 991 bytes exportados y una pérdida al cierre de la petición. No se observa en este hecho un error de memoria ni de generación del modelo.

El observador separado seguía operativo durante la recuperación: 1368 muestras, cero pérdidas y cero errores a las 17:37:17 UTC. Esta observación no subsana la pérdida del tramo de cierre del servicio. El corte del asistente y la suspensión del Codespace se registran como hechos instrumentales distintos. El servicio y el motor se detuvieron antes de sustituir el ejecutable. Se conservan sus trazas cerradas y el diagnóstico.

Los resultados originales y expedientes se publicaron en `1b58b75`; las trazas cerradas, en `cbfeb9de8b2bd1837f1f72310ab798e7b2b9277a`. Una primera transferencia del archivo de trazas mezcló texto de estado y datos comprimidos y no pudo extraerse; se corrigió separando ambas salidas. El origen no se alteró y la segunda transferencia quedó conservada y publicada.

## Corrección propuesta, versión 0.2.3

La traza de cierre pasa a contener una lista fija de contadores numéricos, tiempos declarados, tamaño serializado y huella SHA-256 de `phase_timings`. El expediente mantiene íntegramente el valor original, la respuesta y las probabilidades por token. No se elevan las cotas ni se desactiva la guardia de integridad. La huella corresponde a `serde_json::to_vec(Value)` en UTF-8, sin equivalencia presupuesta con los bytes de un transporte HTTP.

La prueba de regresión reproduce el desbordamiento con datos sintéticos extensos, exige la pérdida explícita en el recorrido anterior y comprueba exportación íntegra, tamaño acotado, preservación del original y coincidencia de huella en el recorrido corregido. La compilación y su resultado se recibirán en el registro de verificación; este apartado no los anticipa.

## Continuación autorizada

La instrucción de continuar recibida tras el corte permite reanudar el trabajo pendiente. Se fija una ventana nueva de hasta 7200 segundos desde el inicio efectivo, una única inferencia simultánea y hasta 900 segundos por petición. El incremento respecto de 600 segundos se fundamenta en los 891,512 segundos observados en L3072. Se conserva la reserva de 256 tokens, canal final, temperatura cero, semilla y límite de contexto de 4096. Cambia la condición temporal y se declara expresamente.

El controlador concilia el expediente exportado con la conversación viva mediante igualdad estructural completa de sus turnos. Retoma R02-02 sin repetir R02-01 y conserva la respuesta truncada como antecedente. Después ejecuta R01 con historial propio. Antes de cada admisión registra identidad, contexto y perfil; conserva cada resultado y exporta el expediente. No recorta historias ni reemplaza respuestas por soluciones ideales. Un fallo técnico o de observación suspende el banco y exige diagnóstico. El vencimiento puede dejar condiciones pendientes y no se presenta como fallo semántico.

No se adquieren recursos ni se crean ramas adicionales. Quedan pendientes el resultado de la regresión, despliegue, continuación de preguntas, cotejo científico, recepción registral y comprobación de la URL privada.

## Recepción de verificación y despliegue

Rust 1.98.0: 22 pruebas aprobadas, cero fallos; ejecución 2,12 segundos. Compilación de producción terminada en 13,55 segundos. Fuentes `e668d8e1efa18dd5462e9519eb9c5f73cd4e48a0`; registro y Cargo.lock en `2a112ac`. Se mantienen tres avisos de compilación, sin tratarlos como errores ni ocultarlos.

Ejecutable del servicio: SHA-256 `00d5b666c61c8ca22acf899eaefa15208f997b8355f893305490c74ca230ef99`. Controlador: `f21d2583bb66a38e05cb81ce6bd6e53c9c63e887292325b2edfd60db93aac376`. Ambas huellas coinciden tras la transferencia al servidor. Servicio y motor anteriores comprobados inactivos con MainPID=0. Se conservan los ejecutables anteriores y se inicia el servicio corregido con PID 32742. El resultado de una inferencia posterior todavía debe recibirse.

El primer arranque del guion a las 17:52:28 UTC falló antes de crear el directorio de resultados: la ruta de entrada omitía `src/`. Se conservó el diagnóstico, se corrigió la ruta y se añadieron comprobaciones de lectura. La unidad temporal se restituyó tras retirarse al limpiar su estado fallido. Ninguna de esas operaciones admitió una inferencia. El controlador corregido empezó a las 17:55:35 UTC y conservó el vencimiento inicial de las 19:52:28. R02-02 fue admitida tras conciliar el antecedente; la interfaz privada volvió a mostrar el servicio conectado y ocupado. No se inicia una prueba web adicional durante el banco.
