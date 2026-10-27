# Plan de tiempos y entregas

27 de octubre de 2026 · Versión 0.3 · Propuesta, sin aplicación retroactiva.

## Periodos separados

1. Preparación: recepción del banco, fuentes, configuración, identidad, crédito autorizado y observación. No es tiempo del candidato.
2. Entrega de solución: desde el registro monotónico inmediatamente anterior al envío de la solicitud íntegra hasta la recepción completa y validada de su cierre. Máximo propuesto: 900 segundos por entrega. Se registran envío, primer fragmento recibido y cierre; ninguno se presenta como tiempo interno de generación.
3. Admisión y compilación: comprobación del formato y construcción del artefacto dentro del recinto. Se registra por separado; no se suma ficticiamente al tiempo de generación.
4. Ejecución y corrección: límites propios por conjunto y operación, fijados antes del intento. Los recintos ordinarios observados tienen un límite exterior de 45 segundos por conjunto; no significa 45 segundos para cualquier operación ni para toda la convocatoria.
5. Recuperación instrumental: hasta 300 segundos por incidencia acreditada, con observación activa y sin crédito temporal ilimitado. Si no se recibe recuperación, se cierra sin calificación el periodo afectado. No se reanuda como íntegra una generación rota.

El reloj efectivo excluye sólo pausas acreditadas. Si no existe mecanismo fiable para suspender y reanudar una generación remota, se termina el intento afectado y se conserva como incidencia; no se inventa una pausa del proveedor. Una repetición no es automática y no borra el consumo del intento anterior.

## Base de calibración

Doce entregas conservadas están vinculadas a comprobaciones técnicas satisfactorias. La mayor duración observada fue 484.856 ms (8 min 4,856 s). El límite propuesto de 900 s ofrece margen y coincide con la cota técnica de transmisión progresiva utilizada. Es una elección contractual motivada, no un percentil, una garantía estadística ni una prueba de viabilidad de los casos aún pendientes.

Los tiempos por bloque no se dividen entre sus parámetros para fabricar duraciones individuales. Una respuesta válida recibida antes del límite puede ser comprobada después sin convertir su entrega en tardía; deben respetarse los límites independientes de ejecución. El tiempo máximo del transporte no produce U por sí solo.

## Secuencia propuesta

| Bloque | Escenarios | Entregas independientes | Máximo agregado de entrega |
|---|---|---|---|
| B01 | C01–C03 | 1 | 900 s |
| B02 | C04: cuerpo, metadatos, nombre y resultado de herramienta | 4 | 3.600 s |
| B03 | C05 | 1 | 900 s |
| B04 | C06 | 1 | 900 s |
| B05 | C07–C09 | 1 | 900 s |
| B06 | C10 y C14 | 1 | 900 s |
| B07 | C11 | 1 | 900 s |
| B08 | C12 | 1 | 900 s |
| B09 | C13: simulación y comprobación física diferenciadas | 1 | 900 s |
| B10 | C15 | 1 | 900 s |
| B11 | C16 | 1 | 900 s |
| B12 | C17 | 1 | 900 s |

Total propuesto: quince entregas, 13.500 segundos como suma máxima de sus periodos (3 h 45 min). No es una estimación de duración real ni incluye preparación, conexión, compilación, ejecución, corrección o incidencias. C13 deberá demostrar que una misma entrega puede cubrir ambos recorridos; si no, el cambio de oportunidades se fija antes del siguiente intento formal. No se añaden llamadas durante el examen para completar lo que falte.

## Límites que deben recibirse

El plan estructurado mantiene nulos los tiempos particulares aún no recibidos, especialmente C12, C13 y el conjunto completo de C15. No se sustituyen por cero ni por 45 segundos. Los límites de compilación, x86, TLS, solicitudes, concurrencia y cancelación se obtienen de la revisión congelada de cada programa y de su calibración; deben relacionarse con obligación, inicio, cierre, criterio temporal y reacción al vencimiento.

No se abre el intento formal con esos campos pendientes. La recepción exige referencia positiva, variante defectuosa y observación suficiente para distinguir un incumplimiento de una incidencia. Un resultado anterior a favor de un candidato acredita viabilidad de lo comprobado, pero no impide revisar una objeción técnica fundada de otro candidato.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
