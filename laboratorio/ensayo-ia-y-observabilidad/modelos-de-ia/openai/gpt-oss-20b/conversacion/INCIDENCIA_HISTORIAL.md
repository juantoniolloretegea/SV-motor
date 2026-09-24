# Incidencia de serialización del historial y corrección

Fecha: 24 de septiembre de 2026. Detección durante la inspección del tokenizador, después de completar las doce tareas breves del banco inicial y antes de la conversación acumulativa de ocho intervenciones.

El tokenizador verificado identifica el fin de una respuesta con `<|return|>` (ID 200002). La adaptación inicial había escrito `<|fim_suffix|>` al serializar respuestas anteriores. Este último texto no figura entre los tokens especiales del archivo instalado. Que coincidan los recuentos de entrada de interfaz y motor no basta para acreditar el significado correcto de un delimitador.

Se detiene el controlador del banco y después el servicio, cuya parada cancela cualquier petición aún activa y detiene el motor. Se conservan íntegramente el banco inicial, sus registros y el ejecutable 0.2.0; no se borran ni reclasifican sus respuestas.

Las doce tareas Q01–Q12 no contienen respuestas anteriores y permanecen evaluables con sus entradas originales. Las condiciones M del banco inicial se conservan como resultados de la integración previa; no se utilizan para acreditar conversación conforme. No se atribuye esta incidencia al modelo.

La versión 0.2.1 usa `<|return|>`, valida al iniciar los cinco identificadores especiales utilizados y rechaza texto de usuario que contenga delimitadores reservados. Se amplía la prueba de historial y se incorpora un control negativo de vocabulario. También se sustituye una casilla desactivada de la interfaz por una indicación textual del canal de salida.

La continuación ejecuta de nuevo únicamente las condiciones de historial M y de contexto creciente L, en un directorio independiente y con la misma formulación y criterios. No se repiten Q01–Q12. La conversación acumulativa D se ejecutará después, con el protocolo de ampliación ya publicado. La ventana automatizada común sigue terminando a las 16:38:50 UTC.

## Incidencia posterior de identificadores y suspensión de la continuación

Corte 2026-09-24T15:01:35.167Z. La referencia al ID 200002 del apartado anterior corresponde al identificador declarado en el archivo; no al cargado por la biblioteca de la interfaz. El nuevo control de inicio de 0.2.1 detectó diferencias entre ambos vocabularios y abortó antes de abrir el servicio. La continuación M/L descrita anteriormente era una actuación prevista: no llegó a comenzar. Tampoco se ejecutó D.

La inspección nativa observó start: 201075 frente a 200006; end: 201076 frente a 200007; message: 201077 frente a 200008. La explicación por entradas de relleno del vocabulario base es una hipótesis pendiente de inspección completa. No se modifica el tokenizador ni se desactiva el control. Los identificadores almacenados en las vistas previas anteriores no deben presentarse como IDs canónicos del motor. Las solicitudes enviaban texto al motor, no esos IDs; la coincidencia de cantidades no acredita equivalencia de identificadores.
