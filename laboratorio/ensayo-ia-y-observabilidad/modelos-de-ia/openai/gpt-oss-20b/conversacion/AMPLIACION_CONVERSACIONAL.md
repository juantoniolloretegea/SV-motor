# Ampliación previa: conversación acumulativa de ocho intervenciones

Registro previo a la ejecución: 24 de septiembre de 2026. Se amplía el alcance conversacional del protocolo sin modificar sus doce tareas ni sus resultados. El banco inicial permanece inalterado.

Objetivo: medir conservación y actualización de antecedentes en una conversación real más extensa. Ocho entradas sucesivas incorporan material contextual y modificaciones explícitas de un mismo expediente sintético. Todas las respuestas anteriores del modelo se conservan; no se reemplazan por respuestas ideales. Temperatura cero, canal final, reserva máxima de 128 tokens y plazo de 900 segundos por petición. Ventana común de trabajo automatizado: finalización antes de las 16:38:50 UTC. No se inicia una petición si no restan 925 segundos. No habrá otra inferencia simultánea.

El identificador inicial es LZ-842, con 18 piezas, almacén Norte y revisión R1. Después se corrige la cantidad a 11; se incorpora la regla de reponer si la cantidad es menor que 10; se cambia el almacén a Sur; se añade el pedido P-64; se corrige la cantidad a 8; se sustituye la fecha prevista del 30 de septiembre por el 2 de octubre de 2026; y se solicita un JSON final con todos los datos vigentes. El umbral permanece en 10 y el identificador no cambia.

Resultado final esperado: identificador LZ-842, piezas 8, almacén Sur, pedido P-64, fecha 2026-10-02 y decisión reponer. Se evalúa también cada respuesta intermedia según el criterio registrado en el código. El contexto auxiliar tiene igual procedencia sintética y no constituye información clínica ni datos personales.

Se registran longitud efectiva, tiempo del motor, tiempo completo del servicio, respuesta original y criterio de valoración por turno. Las condiciones se ejecutan en orden acumulativo; no aíslan experimentalmente todos los factores de latencia ni prueban conservación arbitraria fuera de la ventana de 4096 tokens.
