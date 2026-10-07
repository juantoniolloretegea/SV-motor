# Examen documental de GPT-6 Astra · P01–P25

Edición de 07/10/2026. Nodo 03: inferencia mediante proveedor externo, suministro y control locales del SV. Suceso S39; examen TT-0016, vinculado a transporte TT-0021.

## Dictamen y alcance

**Apto para el contrato documental de esta edición.** Las 25 respuestas finales R2 reciben 0; los veinte parámetros críticos son correctos. No hay errores ni indeterminaciones en el vector final. R0 y R1 conservan el mismo vector. R2 es siempre la entrega final: no se escoge retrospectivamente la mejor respuesta.

La admisión es documental, respecto de la fuente congelada y el contrato declarado. No acredita aptitud clínica, estabilidad estadística entre ejecuciones ni recepción médica independiente. El contraste sustantivo es exterior al candidato y asistido por IA; la verificación Rust autentica evidencias, correspondencias y reglas, no constituye una demostración automática de verdad médica.

| Propiedad | Resultado |
|---|---|
| Preguntas y entregas completas | 25 preguntas, tres fases universales; 75 entregas |
| Fases | R0: respuesta; R1: autocrítica; R2: verificación final neutral |
| Vector final | (0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0) |
| Umbral | T(25)=floor(7·25/9)=19; κ=Apto |
| Criticidad | Veinte críticos; P05, P10, P15, P20 y P25 no críticos |
| Regla eliminatoria | Un 1 crítico impone No apto; una U crítica impide admisión |
| Puntuación ponderada | No constituida para este examen; no se inventa una equivalencia con A0 |
| Intentos reales | 77: 75 completos y dos interrumpidos, ambos conservados |

La pareja matemática y visual Frame_C=(frmat,frvis) se representa mediante un vector ordenado de Σ^25, Σ={0,1,U}, b=5, n=25, con 3^25=847.288.609.443 estados. No es una matriz ni un espacio vectorial algebraico. El polígono dispone de 25 posiciones y tres radios. Se conserva la convención del logo SV: 0 rojo/r1, 1 verde/r2 y U azul/r3. El verde no significa aprobación. Las advertencias auxiliares no alteran los símbolos adjudicados.

## Fuente, encargo y separación de funciones

Se conserva la fuente histórica NCI-PDQ en español sobre leucemia de células pilosas, actualización declarada 14/11/2024 y recuperación 26/09/2026. No se confunde fecha de recuperación con actualización clínica. Las 25 preguntas, la clave reservada y las criticidades proceden del examen histórico. Cinco secciones completas y 25 fragmentos MCP se prepararon y cotejaron en Rust. Cada pregunta recibe su sección pertinente completa; las revisiones reciben esa misma fuente y exclusivamente las entregas anteriores de su pregunta.

El Director gobierna el suministro, el orden y las etapas, comprueba las identidades y recoge las entregas. La clave, las calificaciones y la telemetría quedan fuera de los mensajes al candidato. No se habilitaron herramientas ni navegación; se exigió fundamentación exclusiva en la fuente entregada y se contrastaron las citas y conclusiones. Esta evidencia acredita los controles del SV, no aislamiento físico interno del proveedor ni eliminación de los conocimientos previos del modelo.

Se permiten paráfrasis y deducciones documentales fundadas; la coincidencia literal se exige a las citas de apoyo, no a la conclusión. La U debe expresar una insuficiencia concreta y justificada. No se convierte un fallo de servicio en U ni en error científico del candidato.

La fuente y el objeto permiten contraste con el examen del nodo 1. Las condiciones de transporte y recursos son distintas, y esta edición tiene tres fases universales y salida estructurada. Por tanto, no debe presentarse como réplica instrumental idéntica ni atribuir sus diferencias exclusivamente al modelo.

## Observaciones de la revisión

- P03, P12, P21 y P24 conservan advertencia de suficiencia documental. Sus ceros reconocen, respectivamente, el límite de estadificación aceptada, la utilidad no establecida de la ERC para modificar decisiones terapéuticas, el alcance histórico y probatorio de recomendaciones y la ausencia de autoridad normativa individual del PDQ. No acreditan el conocimiento o la autorización que falta.
- P01/R2 contiene una explicación de revisión inexacta al atribuir un defecto de etapa a las respuestas anteriores: R0 y R1 ya estaban correctamente identificadas como etapas 0 y 1. La respuesta documental sobre el linaje y comportamiento permanece correcta. El defecto auxiliar se conserva; no se afirma perfección general de la entrega.
- P17 usa en la pregunta histórica una contrapartida «inmediata». La fuente informa mayor necesidad de transfusión plaquetaria, sin fijar ahí su tiempo exacto. R1 y R2 evitan reforzar esa atribución temporal. La observación de diseño queda registrada sin penalizar una prudencia fundada.
- P23/R0 utiliza «necesidad de intervenir», expresión menos precisa si se separa del resto de su respuesta. El conjunto exige valoración, respeta la infección activa y no prescribe inicio automático. R1 y R2 precisan «valorar tratamiento». Se conserva esta mejora de formulación, sin imputar una prescripción inexistente.

Las dos revisiones no cambiaron ninguna adjudicación. Esto no permite atribuirles una mejora causal de exactitud: el contenido principal ya era correcto en R0. La coincidencia de tres fases dependientes no equivale a tres evaluadores independientes.

## Interrupciones y calidad del servicio

La recepción diferencia causas propias de causas comunicadas por el proveedor. Los flujos originales, resultados, tiempos y huellas se conservan sin reescribir los intentos anteriores.

| Intento | Proveedor y argumento | Alcance y tiempos observados |
|---|---|---|
| r1 · P01/R0 | OpenAI respondió HTTP 200; el receptor local rechazó `keepalive`. No se recibió argumento de fallo del proveedor. | Sin respuesta final ni uso comunicado. Operación observada: 32.247 ms. Incidencia local del contrato de eventos, corregida en r2. |
| r2 · P15/R0 | OpenAI: `server_is_overloaded`; «Our servers are currently overloaded. Please try again later.» | HTTP 200 seguido de error y `response.failed`. Mensaje a 2.095 ms; operación observada 2.141 ms. No se entregó la respuesta inicial P15 ni uso facturable. El receptor anterior añadió el rechazo local «Evento posterior al cierre»; el flujo conserva el error del proveedor. |

**La duración completa de una caída de OpenAI es desconocida.** Los 2.095 ms son latencia hasta el error, no tiempo total de indisponibilidad. La pausa dedicada a recepción y modificación local no se atribuye a una interrupción continua del proveedor. Inicio y cierre UTC, causa literal, etapa no entregada y huellas están en [SERVICIO-PROVEEDOR-RUST.json](SERVICIO-PROVEEDOR-RUST.json).

La adenda r3, autorizada durante el examen, conservó P01–P14 completas, continuó P16–P25 y dejó P15 para el final. Sólo se repitió la etapa incompleta; no se repitieron respuestas completas ni se dieron pistas. Las 33 entregas pendientes llegaron sin otro fallo de servicio.

El controlador Rust aplaza cualquier nueva pregunta afectada y conserva sus etapas ya completas. Una ventana operativa sin entrega de 300.000 ms, o una solicitud que agote ese plazo, suspende el examen con **«prueba no válida por falta de recursos que garanticen el examen»**. No se alcanzó ese supuesto en r3. No es una calificación del conocimiento del candidato. Se mantuvo el límite de 90 minutos de ejecución activa, excluyendo y declarando las pausas de intervención local.

La valoración posterior del proveedor podrá distinguir disponibilidad observada, continuidad, capacidad, latencia y gestión de incidencias de la exactitud del candidato. Las referencias principales son [ISO/IEC 42001:2023](https://www.iso.org/standard/42001) e [ISO 9001:2026](https://www.iso.org/standard/88464.html), cotejadas en sus fichas oficiales. ISO/IEC 20000-1:2018 e ISO/IEC 25010:2023 quedan como referencias técnicas complementarias. El registro específico del proveedor está en [servicios-proveedores-ia/openai](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/servicios-proveedores-ia/openai/INCIDENCIA-SERVICIO-20261007-001.md). **No se atribuye ahora una puntuación, certificación, conformidad integral ni incumplimiento normativo.** El umbral de cinco minutos procede de la dirección del examen, no de un requisito atribuido a ISO.

## Instrumentación y consumo

El control, suministro, recepción, cotejos y cálculo de magnitudes propios se realizan en Rust. El módulo de representación es Rust/egui, compilado a WebAssembly; JavaScript inicia el módulo en el navegador y no adjudica resultados. Los procesos auxiliares de edición y publicación no sustituyen las decisiones del SV.

| Elemento | Realización efectiva y evidencia |
|---|---|
| Director y contrato | Controlador Rust compilado y ejecutado; huellas previas, etapas y aplazamiento temporal registrados |
| Suministro MCP | Rust; secciones completas, fragmentos y fuente cotejados; sin clave en la solicitud |
| Transporte | Modelo solicitado y declarado gpt-6-astra; HTTP, secuencia SSE, terminación, texto, forma y uso cotejados |
| Observación local | Proceso instrumentado, CPU, memoria residente, contadores de lectura/escritura y sockets TCP/UDP, antes/durante/después |
| Integridad | Diarios y originales con SHA-256, cotejo de eventos y respuesta, hitos individuales antes de avanzar |
| Recepción documental | Revisión sustantiva exterior al candidato; Rust verifica identidad, reglas, criticidad y pasajes de las advertencias |
| Representación | egui 0.5.0; CAPA-R2 completa e inmutable; consulta no modifica adjudicación ni ejecuta inferencia |

Las cifras siguientes corresponden a **las 75 entregas completas** salvo el tiempo activo del conjunto, que incluye también los intentos interrumpidos. Los registros individuales distinguen ambos grupos.

| Magnitud | Medida |
|---|---:|
| Tokens de entrada | 807.043 |
| Tokens de salida | 35.655 |
| Total comunicado | 842.698 |
| Tokens de razonamiento comunicados, incluidos en salida | 0; no significa ausencia de procesamiento interno |
| Entrada de caché comunicada | 0 |
| Suma de duración de las entregas completas | 1.503.709 ms (25 min 3,709 s) |
| Ejecución activa total de los tres tramos | 1.876.338 ms (31 min 16,338 s) |
| Duración mínima / máxima por entrega | 10.320 / 38.784 ms |
| Primer texto mínimo / máximo | 3.298 / 18.615 ms |
| Muestras instrumentales | 6.545 |
| Fallos de medición / intervalo máximo | 0 / 379 ms; objetivo 250 ms |
| Eventos SSE / bytes SSE | 35.280 / 10.814.967 |
| CPU acumulada del proceso instrumentado | 180.340 ms |
| Máximo de memoria residente | 49.606.656 bytes |
| Lectura / escritura acumuladas del proceso | 1.743.911.437 / 42.950.505 bytes |
| Máximo de conexiones TCP observadas simultáneamente | 6 |
| Observaciones UDP | 0 |

Los contadores de lectura/escritura no equivalen a bytes de red. Las conexiones y puertos observados pertenecen al proceso instrumentado; no constituyen un inventario global de todos los procesos del equipo. No se midieron separadamente DNS/TLS, RTT, retransmisiones, asignaciones del montículo, hilos ni recursos internos del proveedor. La CPU no tiene calibración externa. La criptografía C/ensamblador continúa pendiente bajo la excepción experimental autorizada; no se declara una cadena íntegramente Rust de todas las dependencias.

| Etapa | Entrada | Salida | Total | Duración acumulada (ms) |
|---|---:|---:|---:|---:|
| R0 | 256.129 | 10.240 | 266.369 | 461.698 |
| R1 | 268.819 | 12.826 | 281.645 | 529.219 |
| R2 | 282.095 | 12.589 | 294.684 | 512.792 |

Los dos intentos interrumpidos no comunicaron uso. Su consumo permanece desconocido y no se añade ficticiamente como cero. Los créditos, importes, impuestos y consumo de asistencia no tienen desglose atribuible por solicitud; la conciliación queda pendiente en el archivo privado `usos-gasto-creditos-tokens-sv`, con un expediente por cada uno de los 77 intentos. Ningún saldo de cuenta se publica aquí ni se atribuye exclusivamente al candidato. Sin pagos ni recargas.

## Comprobaciones, representación y custodia

La continuidad r3 dispone de 54 pruebas Rust favorables; el adjudicador derivado, de siete; el visor, de doce. El visor verifica interacción con los 25 vértices y selectores, correspondencia con fundamentos, integridad, veto crítico, umbrales, radios, colores y alertas. Incluye una regresión exhaustiva de los 19.683 estados de nueve posiciones; no se afirma enumeración exhaustiva de Σ^25.

En la adaptación visual se corrigió la superposición de áreas seleccionables al pasar de nueve a veinticinco vértices, con selección por proximidad y pruebas de interacción. También se mantiene plegable la comparación de fases para reservar espacio a la figura. Estas correcciones locales no modificaron respuestas, mediciones ni adjudicaciones, y no generaron inferencia adicional.

El [polígono autónomo](web/POLIGONO-EGUI.html) contiene sus recursos, con conexiones externas deshabilitadas. Cada posición permite consultar pregunta, criticidad, respuesta original, fundamento, pasajes, advertencia y huellas; la pareja matemática, las referencias y la licencia son desplegables. Su comprobación visual se documenta separadamente.

Se publican CAPA-R0/R1/R2 con respuestas, revisión sustantiva, comparación, mediciones, 75 hitos de medición y adjudicación, informe de servicio y fuentes de realización. La proyección pública del cotejo omite rutas operativas e identificadores internos del proveedor; los originales completos, solicitudes, SSE, diarios y clave permanecen en custodia local. No se afirma custodia remota íntegra de esos originales. Las huellas permiten distinguirlos de los derivados publicados.

Admisión inicial: Motor `c1b7a041ae024e01fa009665b2ad201fa3070c84`; corrección r2 `dda322614e61724b406e70bbd64c4b5df6071897`; adenda r3 `444a36d6180b2a6559af84df705d9758e6a1dde3`. Registro previo: Lenguaje `38386de6f289961be8abc648a88fbb9a8655be42`, S39 r56, Acta 004 §50 y RETP-2026-295. La recepción final se incorpora como S39 r57, Acta 004 §51 y RETP-2026-296, conservando los cierres históricos.

El examen queda concluido; el punto de retorno es la recepción competente del resultado. No se abre otra campaña ni la ampliación futura del MCP. El proceso de inferencia ha finalizado; el visor local sirve únicamente el HTML de consulta.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
