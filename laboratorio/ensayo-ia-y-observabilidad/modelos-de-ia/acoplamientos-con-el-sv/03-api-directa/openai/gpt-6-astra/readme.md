# GPT-6 Astra · OpenAI · Nodo 03

**Versión documental 1.0 · 7 de octubre de 2026.**

**Prueba cerrada: Apto para el contrato documental del examen P01–P25.** Se conservan 25 respuestas finales correctas, incluidas las 20 críticas. Este dictamen se limita a la edición y a las condiciones documentadas; no acredita aptitud clínica general.

## Modalidad y responsabilidad

Astra se consulta mediante la API de OpenAI. **OpenAI ejecuta la inferencia y conserva los pesos del modelo.** El SV administra las fuentes, solicitudes, recepción, adjudicación y mediciones propias con el Árbitro-Director y sus componentes Rust. El candidato permanece fuera de las decisiones de gobierno, la clave de corrección y la telemetría.

En el nodo 01, motor y pesos se ejecutan en infraestructura administrada por el proyecto. La diferencia entre los nodos es la ejecución de la inferencia, no el fabricante ni la ubicación de la pantalla. Los expedientes históricos de GPT-OSS y Qwen conservan sus resultados y sus ubicaciones originales. [Modalidades de acoplamiento](../../../readme.md) · [Nodo 03](../../readme.md).

## Resultado y criterio

| Elemento | Resultado documentado |
|---|---|
| Modelo solicitado y declarado | `gpt-6-astra` |
| Examen y fuente | 25 preguntas; captura NCI-PDQ sobre leucemia de células pilosas, actualización declarada 14/11/2024 y recuperación del 26/09/2026. |
| Respuesta final | R2 en cada pregunta: vector de 25 componentes, todos 0 (correctos). |
| Criticidad | 20 críticas correctas; 5 no críticas correctas. Cualquier 1 crítico habría determinado No apto. |
| Regla SV | T(25)=⌊7×25/9⌋=19; κ=Apto. No hay errores ni U en el vector final. |
| Fases universales | R0: respuesta provisional; R1: autocrítica; R2: verificación final neutral. 75 entregas completas. |
| Incidencias | Dos intentos interrumpidos, conservados y diferenciados de las respuestas adjudicadas. |
| Medición | 842.698 tokens conocidos; 6.545 muestras, sin fallos de medición declarados. Tiempo activo medido: 31 min 16,338 s. El uso no comunicado de los intentos interrumpidos sigue desconocido. |
| Representación | Frame de 25 posiciones y tres radios; visor Rust/egui 0.5.0 con fuentes, criticidades, fundamentos y trazabilidad. |

La puntuación ponderada no está constituida para este examen; no se inventa una equivalencia con la puntuación A0. El [informe](examen25-20261007/resultado/INFORME.md) conserva el par matemático y visual, los avisos de límite documental y las observaciones de las explicaciones de revisión. No se eliminan por el resultado Apto.

Las cinco secciones documentales completas y los 25 fragmentos pertinentes se suministraron bajo control Rust. El encargo restringió la respuesta a esa fuente, permitiendo razonar con sus argumentos. No se habilitaron herramientas de navegación al candidato; esta comprobación no inspecciona el funcionamiento interno de OpenAI. La fuente histórica tampoco equivale a información clínica actualizada.

## Expediente y evidencias

- [Informe del examen: resultado, criterio, mediciones y límites](examen25-20261007/resultado/INFORME.md).
- [Dictamen estructurado](examen25-20261007/resultado/visor/src/DICTAMEN.json), [resumen de métricas Rust](examen25-20261007/resultado/RESUMEN-METRICAS-RUST.json) e [hitos por entrega](examen25-20261007/resultado/hitos).
- [Polígono interactivo Rust/egui](examen25-20261007/resultado/web/POLIGONO-EGUI.html): archivo HTML autónomo para descargar y abrir; la vista de código de GitHub no ejecuta la interfaz.
- [Ficha y antecedentes instrumentales](FICHA.md), [catálogo A0](catalogo-a0-20261007/INFORME.md), [anexo PDF](anexo-pdf-20261007/banco-nueve-preguntas/INFORME.md) y [revisión universal de tres etapas](revision-doble-20261007/INFORME.md). Cada prueba conserva su contrato y resultados propios.
- [Acta 004, apartado 52: cierre documental](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/continuacion-15-09-2026/ACTA_004_FINALIDAD_ALCANCE_Y_CONTINUIDAD_EIO_2026_09_22.md#cierre-documental-astra-nodo03).
- [TT-0016: examen](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0016.md#cierre-documental-astra-nodo03) y [TT-0021: transporte](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0021.md#cierre-documental-astra-nodo03); S39 revisión 58 y RETP-2026-297.

## Alcance del cierre

Se cierra esta prueba documental con el resultado ya adjudicado. No se modifica ninguna respuesta, fuente, clave o medición, ni se ejecuta una inferencia adicional. Las preguntas y el criterio permiten comparación con antecedentes del nodo 01, pero las condiciones instrumentales difieren: API externa, secciones completas y tres fases universales.

La revisión sustantiva documentada fue asistida por IA y exterior al candidato; no constituye recepción médica independiente ni demostración de estabilidad estadística. La dependencia criptográfica que incorpora C y ensamblador permanece pendiente bajo la excepción experimental autorizada. La evaluación del servicio del proveedor y la integración general tienen seguimiento separado. S39 general y TT-0021 no se cierran por extensión.

[Índice de modelos](../../../../README.md) · [Portada del ensayo](../../../../../README.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
