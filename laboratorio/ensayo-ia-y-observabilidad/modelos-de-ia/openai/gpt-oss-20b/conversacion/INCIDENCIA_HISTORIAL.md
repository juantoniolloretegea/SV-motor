# Incidencia de serialización del historial y corrección

Fecha: 24 de septiembre de 2026. Detección durante la inspección del tokenizador, después de completar las doce tareas breves del banco inicial y antes de la conversación acumulativa de ocho intervenciones.

El tokenizador verificado identifica el fin de una respuesta con `<|return|>` (ID 200002). La adaptación inicial había escrito `<|fim_suffix|>` al serializar respuestas anteriores. Este último texto no figura entre los tokens especiales del archivo instalado. Que coincidan los recuentos de entrada de interfaz y motor no basta para acreditar el significado correcto de un delimitador.

Se detiene el controlador del banco y después el servicio, cuya parada cancela cualquier petición aún activa y detiene el motor. Se conservan íntegramente el banco inicial, sus registros y el ejecutable 0.2.0; no se borran ni reclasifican sus respuestas.

Las doce tareas Q01–Q12 no contienen respuestas anteriores y permanecen evaluables con sus entradas originales. Las condiciones M del banco inicial se conservan como resultados de la integración previa; no se utilizan para acreditar conversación conforme. No se atribuye esta incidencia al modelo.

La versión 0.2.1 usa `<|return|>`, valida al iniciar los cinco identificadores especiales utilizados y rechaza texto de usuario que contenga delimitadores reservados. Se amplía la prueba de historial y se incorpora un control negativo de vocabulario. También se sustituye una casilla desactivada de la interfaz por una indicación textual del canal de salida.

La continuación ejecuta de nuevo únicamente las condiciones de historial M y de contexto creciente L, en un directorio independiente y con la misma formulación y criterios. No se repiten Q01–Q12. La conversación acumulativa D se ejecutará después, con el protocolo de ampliación ya publicado. La ventana automatizada común sigue terminando a las 16:38:50 UTC.
