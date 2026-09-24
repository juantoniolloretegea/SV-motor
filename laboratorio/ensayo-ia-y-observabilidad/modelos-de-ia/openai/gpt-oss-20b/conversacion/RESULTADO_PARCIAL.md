# Evaluación parcial y estado de disponibilidad

Registro: 2026-09-24T15:01:35.167Z. Unidad de ejecución experimental. VERIFICACION_ACOTADA.

## Dictamen

Las doce tareas breves terminaron normalmente y sus resultados sustantivos son correctos. Diez cumplen también el formato estricto; Q07 contiene dos espacios al final de la primera línea y Q11 añade explicación pese a solicitarse solo el resultado. Se conservan ambos incumplimientos y no se repiten las tareas. No hay todavía evidencia válida de conversación prolongada ni interfaz disponible para entrega.

| Caso | Contenido | Formato estricto | Tiempo total, s |
|---|---|---|---:|
| Q01-suma | Correcto | Conforme | 24.017 |
| Q02-resta | Correcto | Conforme | 25.476 |
| Q03-multiplicacion | Correcto | Conforme | 25.113 |
| Q04-orden | Correcto | Conforme | 33.376 |
| Q05-extraccion | Correcto | Conforme | 27.799 |
| Q06-json | Correcto | Conforme | 34.672 |
| Q07-condiciones | Correcto | No conforme | 29.606 |
| Q08-resumen | Correcto | Conforme | 47.409 |
| Q09-contradiccion | Correcto | Conforme | 34.251 |
| Q10-insuficiencia | Correcto | Conforme | 32.686 |
| Q11-unidades | Correcto | No conforme | 85.650 |
| Q12-instruccion-citada | Correcto | Conforme | 30.222 |

Latencias totales: 24.017–85.650 s; mediana descriptiva 31.454 s. Son tareas distintas con longitudes diferentes: esta mediana no mide una aceleración ni una distribución de rendimiento de una tarea uniforme. Los originales contienen tiempos propios del motor y del cliente, conservados por separado.

## Integración, controles y límites

La interfaz derivada de Qwen funcionó en una comprobación web privada: respuesta DISPONIBLE y exportación recuperada. La petición registró 91 tokens de entrada y 5 de salida; tiempo total 87,324 s, carga 25,124 s y petición HTTP 24,590 s. El intervalo restante, aproximadamente 37,6 s, carece de atribución causal y requiere estudio. No se mide tiempo hasta el primer token.

La versión 0.2.0 aprobó 18 pruebas unitarias. El control del servicio comprobó rechazo de sesión inválida, contexto excesivo y delimitadores, idempotencia, cancelación con MainPID=0 del motor y exportación. La cancelación durante esta comprobación no acredita todas las fases posibles de inferencia. La versión 0.2.1 aprobó 19 pruebas unitarias, compilación y Cargo check; su guardia de arranque detectó la discrepancia de IDs y detuvo el servicio. Compilar correctamente no acredita integración correcta.

M01 y M02 pertenecen a la integración anterior; M03 fue interrumpido al detenerla. Se conservan, pero no acreditan conversación conforme porque el cierre del historial utilizaba un delimitador no reconocido. Las condiciones L y el diálogo D de ocho turnos no comenzaron. El intento de validación control-02 falló por conexión rechazada con el servicio detenido.

La versión 0.2.1 corrige el texto del cierre, pero detecta además diferencias de IDs entre el archivo declarado y la biblioteca de la interfaz. [Incidencia y alcance](INCIDENCIA_HISTORIAL.md). La aplicación enviaba texto al motor; los IDs guardados en la vista previa no se enviaban. Las tareas Q conservan su evidencia textual, aunque esos metadatos de IDs no son canónicos.

La revisión automática de seguridad del navegador rechazó el acceso al terminal del Codespace alegando un bloqueo previo del origen. Antes se había rechazado recargar una página chrome-error por protocolo no permitido. No se realizaron intentos alternativos para sortear la restricción. Este impedimento pertenece al entorno de ejecución del asistente y no constituye un fallo del modelo.

## Evidencia y continuidad

Originales: [banco inicial](verificacion/previo-0.2.1/banco-01/RESULTADOS.jsonl). [Evaluación separada](verificacion/EVALUACION_PARCIAL.json); [exportación web](verificacion/EXPEDIENTE_WEB.json); [punto de recuperación](CONTINUIDAD.md). No se han comprado recursos ni creado ramas adicionales. El resultado anterior de optimización permanece acotado a su protocolo; TT-0012 conserva su cierre y S39 permanece abierto.
