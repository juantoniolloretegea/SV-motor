# Kimi · rechazo por sobrecarga del servicio

Registro **SV-SERVICIO-KIMI-20261119-001**. Fecha del hecho: **19 de noviembre de 2026**. Proveedor: **Moonshot AI**. Modelo configurado: **Kimi K3.1 Pro** (`kimi-k3.1-pro`). Recuperación pendiente de comprobación.

## Hecho acreditado

Durante la evaluación individual de la convocatoria **eet-cyb16-practica-20261027**, el servicio devolvió **HTTP 429** con el tipo de error `engine_overloaded_error` y el mensaje literal:

> The engine is currently overloaded, please try again later

Se conservaron los 115 bytes del cuerpo recibido. Su identidad SHA-256 es `a2d0d7928ae49b13bab60c4a967b22ffaa7544bcc3ae58a568b8c9efc2e3bf38`. No se recibió una respuesta evaluable ni un identificador de modelo en ese cuerpo. La vinculación al modelo corresponde a la configuración de la solicitud.

La sobrecarga es la causa declarada por el proveedor. Este hecho no acredita por sí solo una indisponibilidad general, su duración total ni un incumplimiento contractual. Tampoco debe confundirse con un diagnóstico de agotamiento de saldo o de cuota: el mensaje conservado identifica expresamente la sobrecarga.

## Mediciones conservadas

| Magnitud | Valor observado |
|---|---|
| Marca UTC del registro de transporte | 2026-11-19 18:04:09 UTC |
| Duración del transporte | 39.101 ms |
| Tiempo hasta el primer fragmento | 38.398 ms |
| Duración de la operación completa, incluido el recuento y las comprobaciones | 84.347 ms |
| Estado HTTP del recuento previo | 200 |
| Estado HTTP de la generación | 429 |
| Consumo y coste comunicados para la generación rechazada | No comunicados |
| Duración total de la indisponibilidad del proveedor | Desconocida |

Las duraciones corresponden a medidas diferentes y no se suman entre sí. La ausencia de consumo comunicado no equivale a gratuidad ni a cero tokens. La conciliación económica permanece separada del resultado científico.

## Consecuencia y continuidad

La evaluación quedó pausada. Se conservan las respuestas completas, el recuento ya recibido, el rechazo original, las evidencias y el tiempo restante. La incidencia de servicio no se transforma en un error de conocimiento ni determina por sí misma un valor de la terna del candidato. La evaluación conserva íntegra la obligación de resolver cada parámetro conforme a sus reglas.

El [precedente de recuperación de servicio](../openai/INCIDENCIA-SERVICIO-20261007-001.md#continuación-y-condición-de-suspensión) establece la conservación de entregas completas y una condición operativa de suspensión de cinco minutos sin entrega. Esa condición procede de la dirección del examen; no es un umbral atribuido a una norma de calidad. El intervalo de pausa administrativa o técnica no demuestra una caída continua del proveedor.

La continuación debe limitarse a la entrega rechazada, reutilizar su recuento válido y conservar por separado ambos intentos. No habilita la repetición de respuestas completas, la modificación del contenido, la comunicación de calificaciones al candidato ni reintentos indefinidos. La recuperación sólo se considerará acreditada cuando exista una nueva recepción comprobada; se documentará mediante una adenda.

## Vinculación y límites de publicación

- [Ficha del modelo y de la prueba](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/estandar-comparativo-o-benchmark/ediciones/ciberseguridad/eet-cyb16-practica-20261027/practicas/candidatos/kimi-k3.1-pro/FICHA.md).
- [Comentarios de servicio de la evaluación](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/estandar-comparativo-o-benchmark/ediciones/ciberseguridad/eet-cyb16-practica-20261027/COMENTARIOS-DE-SERVICIO.md).
- [Registro estructurado de esta incidencia](REGISTRO-20261119-001.json).

Esta comunicación informa de la prestación del servicio. No publica preguntas, respuestas, soluciones, credenciales, saldos ni calificaciones provisionales. El examen individual y el encuentro entre perfiles siguen siendo actividades distintas; esta incidencia corresponde al primero.

**Evaluación de la calidad del proveedor: pendiente de los criterios y de la revisión competente.** Una incidencia aislada no constituye una puntuación global ni una certificación.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
