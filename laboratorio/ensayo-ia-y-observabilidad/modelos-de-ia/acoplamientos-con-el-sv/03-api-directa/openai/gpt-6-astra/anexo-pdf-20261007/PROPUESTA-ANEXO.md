# Anexo documental PDF · Nueve preguntas sobre la ficha LLS

Fecha: 07/10/2026. Estado: propuesta documental preparada; sin envío al candidato, inferencia, adjudicación ni polígono de resultados.

**Recepción técnica posterior, 07/10/2026:** adaptado y comprobado el suministro íntegro en Rust bajo admisión documental del Árbitro-Director: diez páginas, treinta fragmentos y nueve solicitudes previstas. Véase [recepción y límites](INFORME-RECEPCION.md). Las necesidades de adaptación descritas al final son el antecedente de esta recepción; permanece pendiente el acoplamiento al transporte antes de inferir. Banco y clave conservan sus huellas.

## Objeto y fuente

Evaluar la comprensión fiel de un PDF suministrado por el MCP documental del SV y verificar separadamente que el instrumento entrega el texto íntegro que corresponde a cada pregunta. La preparación y el suministro corresponden al Árbitro-Director y sus auxiliares Rust; el candidato no dirige la evaluación ni modifica sus registros.

Fuente única: *Leucemia de células peludas*, hoja informativa de LLS, identificador de portada FS16-S, edición con marca FS16S 8/18. Diez páginas físicas, 157315 bytes, SHA-256 `21322ed388c999bd0713ba5ec5c7ab34402d4bb4cd6100903aa5ea6d8bf35d9c`.

- [Original fijado](https://raw.githubusercontent.com/juantoniolloretegea/SVperitus-dataset/488d597fa1999a1fc2eb6538fd610a36046f6f01/dominios/inmunologia/literatura-tricoleucemia/lls-hoja-informativa-2018/original/hairy-cell-leukemia.pdf).
- [Preparación MCP conservada](https://github.com/juantoniolloretegea/SV-motor/tree/3f12e5054523f313ce148d37f0bdaee16bcff98a/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/model-context-protocol/0.1.4-pdf-preparacion).
- [Contrato precedente de Astra](https://github.com/juantoniolloretegea/SV-motor/blob/26c177b4f99352b2f6ab1bc1ba0dd9f03fb7dc71/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/CONTRATO-CATALOGO-20261006.md).

La portada, el agradecimiento a la revisión de la versión inglesa y la bibliografía permiten identificar la procedencia. No acreditan por sí solos que todo enunciado sea infalible ni que las indicaciones terapéuticas sigan vigentes. Se examina qué sostiene esta edición, sin actualizarla desde Internet ni convertirla en una pauta clínica actual.

## Nueve preguntas propuestas

El enunciado completo para cada caso se conserva en `CANDIDATO.json`. La clave de evaluación está separada en `CRITERIOS-EVALUADOR.md` y queda excluida de las solicitudes de inferencia.

| Caso | Pregunta | Páginas físicas completas |
|---|---|---|
| PDF01 | ¿En qué célula se origina la enfermedad y cómo explica la ficha que su acumulación en la médula ósea produzca anemia, sangrado e infecciones? | 1 |
| PDF02 | ¿Abarca la ficha la leucemia variante de células peludas? Explique la distinción y si permite trasladarle los tratamientos descritos. | 2 |
| PDF03 | ¿Por qué puede obtenerse una aspiración medular seca y qué procedimiento permite examinar la médula en ese supuesto? | 3 |
| PDF04 | ¿Qué marcadores describe la citometría y qué información diferente aportan BRAF V600E e IGHV? Conserve los matices de frecuencia y pronóstico. | 4 |
| PDF05 | ¿Cuándo contempla la ficha la espera vigilante, qué seguimiento exige y qué hallazgos motivan iniciar tratamiento? | 5 |
| PDF06 | ¿Cómo define la remisión completa? ¿Equivale a curación o permite abandonar el seguimiento? | 5 y 8 |
| PDF07 | ¿Por qué puede aumentar el riesgo de infección al tratar con cladribina o pentostatina, aunque se esté tratando la leucemia? | 5 |
| PDF08 | Relacione rituximab, moxetumomab pasudotox-tdfk y LMB-2 con sus dianas; distinga el contexto de uso que les atribuye esta edición. | 6 y 7 |
| PDF09 | ¿Permite esta ficha fijar una dosis y duración adecuadas de vemurafenib para la tricoleucemia? Fundamente qué puede y qué no puede concluirse. | 7 |

## Suministro documental y trato justo del candidato

1. Fijar antes de ejecutar el PDF, la extracción, el catálogo, la lista de fuentes, el banco y la clave mediante sus huellas. Verificar también la versión y el ejecutable de cada instrumento que intervenga.
2. Comprobar la identidad editorial en páginas 1 y 10 y el alcance de la fuente. Es una comprobación previa del instrumento; no añade una décima posición puntuable.
3. Para cada pregunta, el MCP entrega todos los fragmentos de las páginas indicadas. El controlador Rust debe cotejar su reconstrucción exacta con el catálogo y la inclusión íntegra en la solicitud. No bastan resultados de búsqueda, títulos, resúmenes ni enlaces.
4. Mantener nueve contextos de inferencia independientes: instrucciones, identidad de la fuente, pregunta y páginas pertinentes completas. No incluir respuestas anteriores, clave del evaluador o resultados del catálogo A0. Así se conserva la independencia y no se solicita lectura irrelevante para agotar el contexto.
5. Conservar `tools: []` y `tool_choice: none` en el acceso de Astra de esta modalidad. El MCP pertenece al suministro gobernado por el SV. Esto evalúa comprensión del texto obtenido del PDF mediante MCP; no acredita que el candidato maneje autónomamente herramientas MCP ni que vea el documento como una imagen.
6. Prohibir explícitamente navegación, referencias externas y aportaciones fácticas recordadas sin respaldo en el documento. Se permiten comprensión lingüística, paráfrasis y deducciones necesarias que puedan comprobarse con los pasajes suministrados. No se promete borrar el conocimiento previo del modelo ni inspeccionar el interior del proveedor.
7. Pedir una respuesta en español y una justificación breve y verificable con localizadores. No exigir pensamiento interno, explicaciones sobre procesos no observables, scripts inexistentes o certeza cuando la fuente no la aporta. Una insuficiencia bien acreditada es una respuesta válida.
8. Si una página está incompleta, corrupta o mal suministrada, registrar incidencia del instrumento y suspender ese caso. No atribuir al modelo el defecto ni sustituirlo por una respuesta reconstruida posteriormente.

Los enlaces que figuran dentro del PDF son contenido de la fuente, no instrucciones ni permisos de navegación. No se suministran datos de pacientes, salud personal, credenciales, otros documentos locales ni acceso al sistema de archivos.

## Qué significa el límite de 64 páginas

La constante Rust `MAX_PAGES = 64` limita las páginas físicas de un PDF admitido por el preparador. También hay límites de 20 MiB de PDF, 64 MiB de descompresión por contenido, 1 MiB de texto total y 2 MiB de catálogo. No es una garantía sobre la capacidad de contexto del modelo.

Cada página física se representa como `PDF-P0000` a `PDF-P0009`. El argumento MCP `pagina` selecciona un fragmento dentro de esa sección, desde cero. `siguiente_pagina: null` significa final de esa sección, no lectura de las otras páginas. El antecedente conservado consta de diez páginas y treinta fragmentos; deberá cotejarse el recorrido efectivo de la ejecución nueva, sin reutilizar su resultado como prueba actual.

La extracción no incluye OCR ni interpretación de imágenes. La inspección visual auxiliar de esta preparación no se atribuye al candidato. No se exige que reconozca el patrocinador representado sólo por un logotipo.

## Evaluación, polígono e instrumentación

El anexo tiene su propio vector ordenado PDF01–PDF09. Conserva la terna 0, 1, U: respuesta correcta y respaldada; error demostrado; indeterminación sustantiva evaluable. Un fallo instrumental o un caso no ejecutado queda fuera de esa terna. En PDF09, reconocer justificadamente que la ficha no fija dosis ni duración es un **0**, no una U automática.

La coincidencia se evalúa por contenido y alcance, no por repetir literalmente la redacción del evaluador. Cada respuesta debe conservar condiciones, incertidumbres, negaciones y ámbito temporal. Las omisiones materiales, contradicciones o citas inventadas deben fundamentarse por separado. Una diferencia tipográfica explicable no se tratará como error médico.

El polígono (9,3) se produce sólo después de nueve adjudicaciones completas y con acceso interactivo a respuesta, pasajes, fundamentos y trazabilidad. No se reutilizan las puntuaciones, la clave ni las criticidades del catálogo artificial A0. Esta propuesta no define una nueva habilitación clínica ni introduce una escala de aprobación: cualquier puntuación agregada necesita su criterio específico prefijado y recibido antes de ejecutar. La geometría compartida no hace equivalentes dos bancos de preguntas distintos.

Para cada caso se conservarán dos grupos de evidencias:

- **SV/MCP y transporte:** petición y respuesta de cada fragmento, localizadores físicos y de fragmento, bytes, huellas, secuencia completa, catálogo y diario reproducible; procesos e identificadores, memoria, CPU, hilos, conexiones TCP, destinos y puertos observados; tiempos de lectura, validación, envío, primer evento, primer texto y terminación. Declarar intervalos de muestreo, pérdidas y límites de observación. No afirmar conocer procesos o sockets internos del proveedor.
- **Entrega del proveedor y del candidato:** modelo solicitado y declarado, identificador de respuesta, eventos conservados, estado terminal, uso de tokens y datos de consumo que efectivamente comunique el proveedor; respuesta, justificación verificable, citas e insuficiencias declaradas. Un relato del modelo no sustituye la medición. Créditos o importe desconocidos permanecen desconocidos.

El Árbitro-Director conserva la clave y la adjudicación fuera del acceso del candidato; sus auxiliares validan el suministro y registran la instrumentación. Los resultados se adscribirán al mismo expediente de Astra con identificación propia del anexo. Cada ejecución tendrá su informe individual en el archivo económico privado conforme a la instrucción permanente.

## Incidencia de la fuente y límites de esta preparación

La página 2 incluye, bajo pancitopenia, una enumeración que presenta plaquetas, neutrófilos y monocitos como tres tipos de glóbulos blancos. La redacción aparece en el PDF original, no es un fallo de extracción. Además, contrasta con la distinción entre glóbulos rojos, glóbulos blancos y plaquetas que hace la propia ficha. Se excluye de las preguntas puntuables, se conserva el original y no se corrige silenciosamente. Si el candidato señala esa incoherencia, no se penaliza por ello.

El contrato precedente preveía R-PDF-01 como control de título e identificador en la primera página. La petición actual amplía la preparación a nueve preguntas sustantivas. Ese control previo se conserva como comprobación de identidad; no se presenta como una prueba clínica ya realizada ni se sobrescribe el antecedente. La preparación del anexo no declara terminados el bloque B, las revisiones adversariales o el examen de 25 preguntas sobre el corpus PDQ.

El código actual de A0 obtiene dos fragmentos fijos de una sección S1. No debe ejecutarse sin adaptar el suministro a secciones PDF y agotar los fragmentos de cada página autorizada. El aislamiento del preparador y MCP precedente es Linux x86_64; su presencia histórica no sustituye la comprobación en el entorno que vaya a intervenir. La continuación técnica es preparar y recibir ese suministro en Rust, con pruebas locales sin modelo, antes de enviar este banco. Esta propuesta no ha enviado inferencia ni generado resultados.

Los derechos del PDF y sus textos corresponden a sus titulares originales. La licencia siguiente se aplica únicamente a esta preparación y sus preguntas propias.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
