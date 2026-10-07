# OpenAI · interrupción y recuperación del servicio

Registro SV-SERVICIO-OPENAI-20261007-001. Fecha del hecho: 07/10/2026. Suceso S39; TT-0016 y TT-0021. Servicio: inferencia de GPT-6 Astra mediante el cliente autorizado. Evaluación del servicio pendiente; entrega recuperada.

## Hecho acreditado y alcance

En P15/R0 del examen documental de 25 preguntas, OpenAI devolvió `server_is_overloaded` y el argumento **«Our servers are currently overloaded. Please try again later.»** La respuesta HTTP inicial fue 200, pero el flujo terminó en `response.failed`. No se recibió respuesta final ni uso de tokens de ese intento. El código HTTP inicial no acredita por sí solo prestación completa.

Quedó sin entregar la respuesta inicial P15/R0; sus dos revisiones dependientes permanecieron pendientes. Se conservaron las 42 entregas completas de P01–P14. La falta de servicio no se transformó en error de conocimiento ni en U del candidato.

| Observación | Tiempo UTC o duración |
|---|---|
| Inicio del envío fallido | 2026-10-07 14:02:13,385 UTC |
| Aparición del error del proveedor | 2.095 ms desde el inicio instrumental de la operación |
| Cierre observado del intento | 2026-10-07 14:02:15,527 UTC |
| Duración monotónica registrada | 2.141 ms |
| Primera entrega completa tras continuar: P16/R0 | 2026-10-07 14:25:43,033 UTC |
| Recuperación de P15/R0, aplazada al final | 2026-10-07 14:39:02,654 UTC |
| P15/R1 completa | 2026-10-07 14:39:22,257 UTC |
| P15/R2 completa | 2026-10-07 14:39:40,916 UTC |

**Duración total de indisponibilidad del proveedor: desconocida.** No se realizaron peticiones continuas entre el error y la reanudación. La pausa de recepción y corrección local no demuestra una caída durante todo ese intervalo. Los relojes UTC y monotónico tienen funciones distintas; no se modifica la duración monotónica para hacerla coincidir con una resta de marcas UTC.

El receptor anterior añadió el rechazo local «Evento posterior al cierre» al recibir `error` seguido de `response.failed`. Esta causa local secundaria queda separada del mensaje original de sobrecarga, conservado en el flujo. Otro intento anterior P01/R0 fue interrumpido por el rechazo local de `keepalive`: se registra en el examen, pero no se imputa aquí como sobrecarga de OpenAI.

## Continuación y condición de suspensión

La continuación r3 aplicó el aplazamiento de preguntas interrumpidas: P16–P25 y, después, P15. No se repitieron respuestas completas ni se suministraron calificaciones al candidato. Las 33 entregas pendientes se completaron sin nuevo fallo de servicio.

El controlador Rust suspende la prueba si una ventana operativa sin entrega alcanza 300.000 ms, o si una solicitud agota por sí sola ese plazo. El estado de suspensión es **«prueba no válida por falta de recursos que garanticen el examen»**. No se alcanzó en esta continuación. Los cinco minutos son una condición de la dirección del examen, no una cifra atribuida a una norma ISO.

El [registro estructurado](REGISTRO-SERVICIO-20261007-001.json), derivado y cotejado en Rust, conserva proveedor, modelo, identidad de la incidencia, argumento literal, tiempos UTC y monotónicos, alcance, recuperación comprobada, consumo desconocido y huellas. La evidencia técnica se enlaza con el [examen y sus resultados](../../modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/examen25-20261007/resultado/INFORME.md). No se incluyen credenciales, saldos ni datos de pacientes.

## Marco de evaluación

Las referencias principales son **[ISO/IEC 42001:2023](https://www.iso.org/standard/42001)**, para el sistema de gestión de inteligencia artificial, e **[ISO 9001:2026](https://www.iso.org/standard/88464.html)**, para el sistema de gestión de calidad. Se cotejaron sus fichas oficiales; la edición de ISO 9001 indicada aquí es la publicada el 16/09/2026.

Este expediente proporciona evidencia para la futura evaluación del proveedor y del proceso propio: servicio esperado y recibido, interrupción, consecuencias, control de la continuidad, recuperación, seguimiento y decisión posterior. Antes de puntuar deberán definirse el alcance, los requisitos verificables, el período de observación, las ponderaciones y la revisión competente. La evaluación deberá consultar el texto normativo aplicable; no se atribuyen cláusulas no cotejadas.

ISO/IEC 20000-1:2018 e ISO/IEC 25010:2023 permanecen como referencias técnicas complementarias para aspectos de servicio y calidad de producto. No sustituyen las dos referencias principales anteriores.

**Puntuación del proveedor: pendiente.** La incidencia aislada no demuestra por sí sola conformidad o incumplimiento integral, certificación, disponibilidad contractual ni nivel de servicio garantizado. El dictamen documental del modelo permanece separado de esta evaluación.

## Seguimiento

Entrega recuperada; conservar la incidencia y su vinculación con S39, TT-0016, TT-0021, Acta 004 §51 y RETP-2026-296. La siguiente actuación es establecer criterios de evaluación del servicio y recibir competentemente las evidencias. No se inicia por este registro otra campaña de inferencia, contratación, reclamación o gasto.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
