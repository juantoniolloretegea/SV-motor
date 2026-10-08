# Grok 4.7 · Examen documental de 25 preguntas

Edición de 8 de octubre de 2026. Nodo 03, API directa de xAI. **Dictamen final: Apto para el contrato documental.** Este resultado se refiere exclusivamente a la fidelidad al corpus histórico recibido y al contrato de este examen; no acredita aptitud clínica general. Revisión exterior al candidato asistida por IA, con recepción competente independiente pendiente.

## Resultado y evolución

| Etapa | Correctas | Errores | U | Errores críticos | κ | Admisión |
|---|---:|---:|---:|---:|---|---|
| R0 | 24 | 1 | 0 | 1 | Apto | No apto |
| R1 | 25 | 0 | 0 | 0 | Apto | Apto para el contrato documental |
| R2 | 25 | 0 | 0 | 0 | Apto | Apto para el contrato documental |

Cada posición se evalúa en R0, R1 y R2. La respuesta final es siempre R2, sin escoger retrospectivamente la más favorable. [Tres vectores y transiciones](COMPARACION-RUST.json), [adjudicaciones por entrega](hitos) y [polígono interactivo egui](presentacion/POLIGONO-EGUI.html). La clasificación ternaria usa 0, 1 y U; T(25)=⌊7×25/9⌋=19. Veinte posiciones críticas se fijaron antes del envío: todas salvo P05, P10, P15, P20 y P25. Un único 1 crítico determina No apto, con independencia de κ. No se inventa una puntuación ponderada ausente del contrato.

En P02, R0 situó incorrectamente la leucocitosis «dentro del espectro de esa leucopenia». R1 corrigió espontáneamente la relación y R2 conservó la distinción. El error crítico inicial permanece visible y no desaparece del vector por la corrección posterior.

Algunas revisiones objetan cambios de palabras, saltos de línea o deducciones válidas. En P16, por ejemplo, el candidato retiró «todos respondieron» porque no era literal, pese a ser una deducción admisible del 100 % comunicado para ese grupo. La revisión exterior distingue esos cambios de una corrección real; no exige copiar literalmente las conclusiones ni confunde ampliar evidencia con demostrar falsedad previa. El detalle de cada posición conserva esas observaciones.

P03, P12, P21 y P24 reconocen límites documentales expresos. Llevan advertencia visible y una cita que Rust contrasta con el suministro real. El 0 indica que se ha reconocido correctamente ese límite; no demuestra capacidad para responder lo que el documento no proporciona. La adecuación del diseño a su finalidad permanece sometida a revisión.

## Contrato, fuente y comparación

Se reutilizan sin cambios el banco EVAL-PDQ-HCL-25-20260929/r1, sus 25 enunciados, clave reservada y corpus NCI-PDQ usados en Astra. La fecha editorial del texto es 14/11/2024; no se presenta como información clínica actual. El MCP local entrega completas las secciones pertinentes, con reconstrucción, localizadores, diario y huellas. Son supuestos documentales; no se envían historias de pacientes.

R0 ofrece una respuesta inicial; R1 realiza autocrítica; R2 verifica de forma neutral con libertad para mantener, corregir o declarar U. Se conservan íntegros el encargo y los antecedentes de la misma pregunta en las revisiones. El candidato no recibe la clave, las criticidades, el dictamen externo, la instrumentación ni las preguntas de otros casos. No se le avisa de los errores que la evaluación exterior vaya encontrando.

Las mismas preguntas y reglas permiten comparar los resultados documentales con Astra y sus antecedentes, conservando las diferencias de transporte, modelo, recursos e instrucciones históricas. No se afirma igualdad de entornos físicos ni causalidad exclusiva de las dos revisiones. Los 75 resultados de un único examen tampoco miden estabilidad entre ejecuciones independientes.

## Transporte y observabilidad

Setenta y cinco entregas científicas completas. Duración medida del banco: 3814378 ms. Tokens comunicados del examen: 1030606 de entrada y 243081 de salida, total 1273687; de ellos, 88704 corresponden a entrada en caché y 200370 a razonamiento dentro de la salida. Esos subconjuntos no se suman nuevamente. Eventos SSE: 54740; muestras locales: 14007.

Se conservan por entrega tiempo total y primer texto, proceso, CPU, memoria residente y virtual, E/S, TCP/UDP, estados, cabeceras, sucesos SSE, uso original del proveedor y huellas. La suma de operaciones medida es 3712666 ms; su mediana inferior, 39267 ms; el máximo, 212952 ms. El intervalo instrumental máximo es 446 ms y los fallos de medición suman 0. La duración del banco incluye los controles locales entre solicitudes y no se atribuye íntegramente al proveedor. La recepción Rust contrasta la cadena instrumental, el texto reconstruido del flujo con la entrega completa y el original final, las citas y el esquema. Los datos operativos privados no se incluyen en la edición pública. [Inventario y límites](INVENTARIO-Y-LIMITES.md).

La primera comprobación sintética obtuvo HTTP 400 porque xAI no admite tool_choice cuando no hay herramientas. No recibió respuesta del modelo y no comenzó el examen. Se conservó íntegra; el perfil se corrigió omitiendo ese parámetro y manteniendo tools=[]. La segunda comprobación devolvió exactamente el objeto técnico esperado. No se hizo una repetición selectiva de una pregunta científica por ese ajuste.

Dos precisiones documentales: el párrafo general de la admisión conservada menciona tool_choice=none, pero su adenda inicial y las solicitudes efectivas acreditan la omisión específica de xAI. Una descripción heredada del suministro menciona OpenAI; en esta ejecución el destino real es xAI. Se conservan ambos originales y estas aclaraciones, sin reescribir las evidencias fijadas.

Los importes por solicitud, ambos intentos técnicos y los costes desconocidos se registran en el archivo administrativo privado. Una reserva preventiva no es un cargo, y el consumo no se suma otra vez a la compra de crédito. No hubo compra ni recarga realizada por el SV.

## Alcance de control y derechos

tools=[] y store=false en cada solicitud; fuente externa prohibida y clave reservada excluida. Los contadores comunicados de herramientas y fuentes externas son cero, cotejados por el receptor. Esto acredita la configuración y la declaración recibida, no inspección de la infraestructura de xAI ni supresión del conocimiento adquirido durante el entrenamiento.

Retención cero activada por autorización expresa y confirmada en las cabeceras. La licencia propia y la reserva de derechos se incluyen en cada envío, conservando derechos de terceros. No son un mecanismo técnico capaz de impedir por sí solo usos indebidos del proveedor.

El cliente, Árbitro, MCP, instrumentación, recepción y derivación del dictamen propios se realizan en Rust. Se reutilizan los componentes recibidos; el perfil no duplica el cliente por modelo. La criptografía nativa mantiene la excepción experimental pendiente. egui dibuja desde Rust/WebAssembly; JavaScript se limita al enlace generado y al arranque de presentación, sin decisión ni modificación de adjudicaciones.

## Custodia y cierre

Los hitos incluyen las respuestas originales dentro de su adjudicación, evidencia, fundamento, criticidad, huellas y medición pública. La publicación conserva también código, revisión exterior, vectores, banco y presentación. Los originales operativos completos permanecen en custodia local reservada y no se declaran publicados íntegramente. El [manifiesto público](MANIFIESTO-PUBLICO.json) permite comprobar la edición documental. La clave de corrección y credenciales quedan excluidas.

Los registros de calidad conservan S39 r64, TT-0014/TT-0022, Acta 004 §58 y RETP-2026-303. La incorporación documental posterior no se presenta como un tique creado antes de la ejecución. Próximo punto: recepción competente y conciliación de importes pendientes; ningún nuevo ensayo ni ampliación económica se deduce del resultado.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).