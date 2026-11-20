# OpenAI · rechazo por sobrecarga del servicio

Registro **SV-SERVICIO-OPENAI-20261120-001**. Fecha del hecho: **20 de noviembre de 2026**. Modelo configurado: **Astra** (gpt-6-astra). Recuperación pendiente de comprobación.

## Hecho acreditado

Durante el encuentro entre perfiles de la convocatoria **eet-cyb16-practica-20261027**, OpenAI devolvió **HTTP 503**, con tipo **service_unavailable_error** y código **server_is_overloaded**. La sobrecarga es la causa expresamente comunicada por el proveedor. No se recibió una respuesta evaluable del modelo.

Se conservaron los 195 bytes del cuerpo, con SHA-256 e7935ecb804baff20fc1efc19e874f0bfce72adc4efbf3839527928040a43d04. El cuerpo no identifica un modelo devuelto; su vinculación a Astra corresponde a la configuración de la solicitud. Este hecho aislado no acredita una indisponibilidad general, su duración total ni un incumplimiento contractual.

## Mediciones y consecuencias

| Magnitud | Valor observado |
|---|---|
| Marca UTC del registro de transporte | 2026-11-20 14:08:36 UTC |
| Duración del transporte | 22.292 ms |
| Tiempo hasta el primer fragmento | 20.722 ms |
| Duración de la operación, incluido el recuento y las comprobaciones | 59.914 ms |
| Recuento previo de entrada | 15.447 tokens; HTTP 200 |
| Consumo y coste de la generación rechazada | No comunicados |
| Duración total de la indisponibilidad | Desconocida |

Las duraciones corresponden a medidas distintas y no se suman entre sí. Un recuento previo no es consumo de generación ni una factura. Los importes no comunicados permanecen pendientes de conciliación; no se sustituyen por cero.

El encuentro quedó detenido conservando respuestas anteriores, intentos, recuentos y tiempo restante. La pausa interrumpió también la continuación del otro perfil. La incidencia del servicio no constituye un fallo de conocimiento de ninguno de los modelos ni determina su terna. La reanudación no puede borrar el rechazo ni repetir automáticamente las operaciones conservadas.

## Vinculación y límites

El hecho se incorpora a los [comentarios de servicio de la convocatoria](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/estandar-comparativo-o-benchmark/ediciones/ciberseguridad/eet-cyb16-practica-20261027/COMENTARIOS-DE-SERVICIO.md) y al [registro estructurado](REGISTRO-SERVICIO-20261120-001.json). Corresponde al encuentro entre perfiles, separado de la evaluación individual.

No se publican preguntas, respuestas, soluciones, credenciales, saldos ni calificaciones provisionales. La recuperación sólo se acreditará con una recepción posterior comprobada. La evaluación global de la calidad del servicio permanece pendiente de los criterios y de la revisión competente.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
