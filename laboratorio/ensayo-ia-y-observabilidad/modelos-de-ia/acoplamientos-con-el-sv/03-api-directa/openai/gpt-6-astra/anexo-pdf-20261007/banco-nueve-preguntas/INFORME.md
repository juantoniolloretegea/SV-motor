# GPT-6 Astra · Anexo PDF de nueve preguntas

Fecha: 07/10/2026. Nodo 03. S39 / TT-0021. Fuente única: ficha LLS *Leucemia de células peludas*, FS16-S, edición FS16S 8/18. Evaluación de fidelidad a esa edición histórica, sin afirmaciones de vigencia clínica actual.

**Resultado documental: ocho valores 0 y un valor 1, en PDF08.** Los nueve casos recibieron respuesta por API, HTTP 200 y `response.completed`, sin reintentos. El polígono completo conserva el orden PDF01–PDF09: **[0, 0, 0, 0, 0, 0, 0, 1, 0]**. No se adjudica una puntuación, κ o criticidad heredadas de A0; tampoco aptitud clínica ni recepción especializada independiente.

## Entrega y adjudicación

| Caso | Objeto documental | Valor | Fundamento abreviado |
|---|---|---:|---|
| PDF01 | Origen celular y alteraciones sanguíneas | 0 | Conserva linfocito B, acumulación medular y consecuencias documentadas. |
| PDF02 | Exclusión de la variante | 0 | Distingue enfermedad y tratamientos; limita correctamente la extrapolación. |
| PDF03 | Aspiración seca y biopsia | 0 | Diferencia muestras y no interpreta el fracaso de aspiración como ausencia de enfermedad. |
| PDF04 | Citometría, BRAF e IGHV | 0 | Conserva marcadores, frecuencias, contexto terapéutico y asociación pronóstica. |
| PDF05 | Espera vigilante | 0 | Conserva condiciones, proporción aproximada y seguimiento sin inventar umbrales. |
| PDF06 | Remisión y seguimiento | 0 | Cuatro elementos completos; no confunde remisión con curación. |
| PDF07 | Riesgo infeccioso | 0 | Distingue deficiencia previa, inmunosupresión y comunicación preventiva. |
| PDF08 | Dianas y contexto terapéutico | 1 | JSON inválido y relación de evidencias incompleta; no se imputa un error médico no demostrado. |
| PDF09 | Dosis y duración pendientes | 0 | Reconoce correctamente la insuficiencia documental; no es U. |

En PDF08, el texto explicativo conserva CD20/CD22/CD25, las condiciones acumulativas de moxetumomab y la investigación de LMB-2. Sin embargo, la entrega contiene un punto y coma y una cadena inconclusa en `cita_literal_breve`, línea 63, columna 83. No es JSON válido y sus evidencias terminan en rituximab, sin citas/localizadores de las otras dos dianas. La correspondencia del texto final con SSE y el cierre del proveedor fueron cotejados: es un defecto de la entrega recibida, no un fallo de nuestra conexión. Se conserva íntegro, sin reparación ni consulta adicional.

El candidato no recibió la clave, adjudicaciones, telemetría ni respuestas anteriores. La revisión sustantiva se documentó externamente, frente a la clave prefijada y los pasajes efectivamente suministrados; Rust comprobó identidades, condiciones de admisión y correspondencia antes de formar el vector. Esta revisión no se presenta como auditoría clínica independiente.

## Incidencias del comprobador

El cotejo original 0.2.0 aceptó directamente PDF03, PDF05, PDF07 y PDF09. Rechazó PDF01/PDF04 por añadir al identificador un título que sí figura en la página; PDF02 por presentar varias citas como lista, tipo no prohibido por el banco; PDF06 por usar omisiones explícitas `[...]`. PDF08 fue rechazado por JSON inválido.

La revisión 0.3.0, realizada fuera de línea, mantuvo los originales y distinguió esas representaciones del error real. Verificó el identificador exacto, la pertenencia del título, todos los segmentos literales y su orden; sólo normalizó espacios y reconoció las omisiones explícitas. No aceptó citas por semejanza ni corrigió respuestas. Los cuatro rechazos por representación resultaron verificables; PDF08 mantuvo el incumplimiento. `REVISION-CITAS-RUST.json` conserva ambos estados. Se rectifica el alcance del comprobador, no las preguntas ni la clave semántica.

La prueba de integración PDF06 anterior ya había terminado correctamente y su incidencia de localizadores se había corregido sin inferencia adicional. PDF06 en este banco es una observación nueva, contabilizada por separado. Ambas explicaciones conservan el mismo sentido sobre remisión y seguimiento; dos observaciones no demuestran estabilidad general.

## Tiempo, tokens y observación

| Magnitud | Resultado |
|---|---:|
| Solicitudes / reintentos | 9 / 0 |
| Suma de tiempos de solicitud hasta entrega y cotejo inmediato | 232,625 s |
| Duración total del banco, con comprobaciones y observación | 264,710 s |
| Media por solicitud | 25,847 s |
| Entrada / salida / total, comunicados por OpenAI | 30.617 / 7.508 / 38.125 tokens |
| Eventos SSE / bytes conservados | 7.544 / 2.142.566 |
| Muestras del proceso local | 986 |
| Fallos de captura / mayor intervalo entre muestras | 0 / 345 ms |
| Máximo de memoria residente observado | 40.620.032 bytes |

Los tiempos incluyen transporte y registro local; no son tiempos internos de inferencia del proveedor. La instrumentación Rust observó PID, CPU acumulada, memoria, E/S y conexiones TCP/UDP con estados y puertos. Cada caso conserva fases anterior, durante y posterior, diario encadenado, huellas y medición derivada. Los tokens y su atribución se cotejaron contra la entrega del proveedor; el contenido de una explicación no se confunde con telemetría.

Las observaciones de saldo y cuota permanecen en el archivo económico privado. **Créditos e importe atribuibles a cada caso: desconocidos; conciliación pendiente.** Un saldo agregado sin cambio no acredita coste cero. Los nueve expedientes económicos son SV-GASTO-20261007-013 a SV-GASTO-20261007-021. Los consumos de asistencia no tienen desglose atribuible y no se incluyen como cero.

## Hitos, control y comparación

Cada pregunta generó su `HITO-INSTRUMENTAL.json` inmediatamente tras la entrega y el cierre de observación, antes de continuar. La medición derivada y la adjudicación se añadieron después como documentos distintos. El hito original conserva que entonces la evaluación estaba pendiente; no se reescribe su historia ni se forma un polígono parcial.

Se incorporaron además nueve hitos retrospectivos de Astra A0, cotejando respuestas, auditorías, adjudicaciones y métricas contra sus originales. No hubo nuevas consultas ni cambios en aquel resultado. La estructura corresponde a las funciones documentales de los hitos de Qwen: originales identificados, admisión, medición, evaluación y alcance. La fecha de esta incorporación no se presenta como fecha del ensayo original.

La comparación directa de resultados A0 conserva el banco artificial y su criterio. El anexo PDF tiene otra fuente y otras preguntas: la geometría (9,3) y la estructura de registro comunes no igualan dificultad, puntuaciones ni condiciones físicas. OpenAI no proporciona pesos, tokenizador ni instrumentación interna equiparables al modelo alojado localmente.

La admisión se publicó antes del candidato en Motor `6185ed4076ff2c2a0948fd0516fd67026dd60d43`; calidad previa en Lenguaje `e2ee90c4eaeb4026891c24708389ab90c3f5708c`, S39 r51, TT-0021, Acta 004 §45 y RETP-2026-290. Preparación inicial detenida por permisos de WSL, con cero inferencias; nueva recepción r2 conforme tras comprobar el acceso. Se preservan ambas evidencias.

## Límites y custodia

El suministro Rust volvió a recorrer diez páginas y treinta fragmentos MCP, con fuente, identidad, secuencias, huellas y aislamiento local cotejados. Cada contexto recibió las páginas pertinentes completas. No hubo herramientas de navegación o ejecución disponibles para el candidato. Eso no prueba el aislamiento interno de OpenAI ni borra su conocimiento aprendido. Se evalúa texto extraído del PDF, no OCR ni lectura visual autónoma.

Se mantienen declarados como no medidos los recursos remotos, hilos y manejadores, asignaciones individuales de memoria, DNS/TLS desagregados, RTT y retransmisiones. La criptografía nativa sigue pendiente conforme a la excepción experimental autorizada. Custodia y pruebas del software no equivalen a calibración externa integral.

Pruebas locales de software: 40 antes del envío; seis del revisor de citas y seis del adjudicador después, ambas baterías con tres comprobaciones reutilizadas del analizador. Los cotejos de entregas y mediciones se ejecutaron en Rust sin llamadas al modelo. Las comprobaciones del visor se conservan aparte.

Publicación por proyección: respuestas finales, hitos, medición resumida, auditorías, adjudicación y fuentes de adaptación; originales extensos de solicitudes, SSE, diarios y suministro permanecen conservados localmente y quedan identificados mediante huellas. Su presencia en un manifiesto no significa publicación íntegra. No se publica la clave reservada ni el texto completo del PDF. Las copias de código documentan su ubicación y dependencias en el expediente; no son un instalador autónomo. Los derechos de la fuente LLS y las licencias de componentes externos se mantienen.

La ejecución y la autenticación concluyeron. El siguiente trabajo es recibir este anexo y resolver el defecto de PDF08 dentro de la fase que se autorice, manteniendo separados adversariales y examen. No se repite una respuesta automáticamente ni se declara terminado el examen.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
