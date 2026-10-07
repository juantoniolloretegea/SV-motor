# Examen Astra · continuidad de servicio y aplazamiento de preguntas

07/10/2026. Suceso S39; examen TT-0016; transporte TT-0021. Adenda de ejecución autorizada. Conserva el contrato científico, las fuentes, la clave reservada, las criticidades y las tres fases universales de la admisión inicial. Sustituye únicamente la detención inmediata ante todo fallo de servicio y el orden de las preguntas pendientes.

## Punto de continuación

Se han recibido P01–P14 en sus tres fases: 42 entregas. Su recepción se ha cotejado en Rust, incluidos fuente, historia propia, respuesta, eventos, uso y telemetría. No se repiten. P15/R0 carece de respuesta final: OpenAI devolvió `server_is_overloaded` y el argumento «Our servers are currently overloaded. Please try again later.». El flujo original conserva `error` seguido de `response.failed`; el receptor anterior detuvo su interpretación al encontrar ese segundo evento. No se atribuye un error de conocimiento al candidato. El consumo del intento fallido no fue comunicado.

La continuación comienza por P16–P25 y retoma P15 al final. Ante otro fallo explícito de servicio, la pregunta afectada se aplaza al final, conservando sus etapas anteriores completas. La reanudación solicita sólo la primera etapa pendiente, con sus respuestas anteriores íntegras y sin pistas. No se repite una entrega completa ni se selecciona retrospectivamente una respuesta mejor. Cada intento dispone de directorio y hito distintos.

## Límite de interrupción

Una ventana operativa sin entrega comienza al observar un fallo de servicio y termina con una entrega completa. El reloj monotónico Rust limita esa ventana a 300.000 ms; el tiempo disponible restringe también la solicitud siguiente. Una solicitud que agote por sí misma cinco minutos sin entrega produce igualmente la suspensión. Al alcanzar el límite se registra **prueba no válida por falta de recursos que garanticen el examen**. Esta condición invalida la ejecución del examen; no es una calificación del candidato.

La ventana es una observación del recorrido SV–proveedor, no una prueba de indisponibilidad física continua entre peticiones. Se conservan inicio y fin UTC, duración monotónica, tiempos de cada solicitud, proveedor OpenAI, modelo solicitado y declarado, código y argumento exactos, alcance de las etapas no entregadas, recuperación comprobada o no, uso conocido o desconocido y huellas de las evidencias. Un fallo de conexión sin respuesta del proveedor se identifica con causa no determinada, nunca como confesión del proveedor. Una incidencia propia de suministro, identidad, custodia o medición detiene el recorrido con su causa particular.

La pausa anterior dedicada a recepción y modificación local se registra separadamente. No demuestra que OpenAI permaneciera indisponible durante ese intervalo. Los 2.095 ms hasta el evento de error de P15 son latencia del intento fallido, no duración completa de una caída del servicio.

No se altera el límite original de 90 minutos de ejecución activa: quedan 4.388 segundos después de los intervalos ya medidos. Se excluyen y declaran las pausas de intervención local. Hay como máximo 66 intentos nuevos, para 33 entregas pendientes; las repeticiones corresponden exclusivamente a etapas sin entrega completa. Al menos 15 segundos separan intentos fallidos de la misma pregunta. El agotamiento de este límite auxiliar se declara expresamente y no se atribuye por sí solo al proveedor.

## Comprobación y custodia

El controlador, el aplazamiento, los relojes, el suministro y la instrumentación se realizan en Rust. Las huellas de la edición anterior se verifican antes de continuar; las nuevas quedan fijadas en PREVIA.json. La solicitud P15/R0 se compara íntegramente con la anterior. No cambian modelo, fuente exclusiva, ausencia de herramientas, salida estructurada ni independencia del candidato respecto del Director y la evaluación. Se mantienen los límites de medición y la excepción criptográfica experimental ya declarados. El receptor incorpora la secuencia error/fallo sin convertirla en respuesta ni uso ficticio.

Cincuenta y cuatro pruebas Rust favorables: controles heredados, reproducción del fallo observado, aplazamiento y umbral temporal. Los fallos de servicio no se traducen a U ni a 1. Sin veinticinco respuestas finales adjudicadas no existe vector completo, polígono del examen ni dictamen global de aptitud.

## Base para valorar calidad de servicio

Los registros se preparan para una valoración posterior que distinga disponibilidad observada, continuidad, capacidad, latencia y gestión de incidencias de la exactitud documental del modelo. Se han consultado las fichas oficiales de [ISO/IEC 20000-1:2018](https://www.iso.org/standard/70636.html), sobre gestión de servicios, y [ISO/IEC 25010:2023](https://www.iso.org/standard/78176.html), sobre modelo de calidad de producto. Son referencias para definir criterios y alcance: esta adenda no declara certificación, conformidad integral ni infracción de cláusulas no auditadas. No se fija aún una puntuación o ponderación del proveedor. El umbral de cinco minutos es una condición del examen establecida por su dirección, no un umbral atribuido a ISO.

Referencia técnica: [eventos de Responses de OpenAI](https://developers.openai.com/api/reference/resources/responses/streaming-events). La estructura publicada distingue error y fallo de respuesta; la secuencia efectivamente recibida se conserva como evidencia primaria. Fecha de consulta de las referencias: 07/10/2026.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
