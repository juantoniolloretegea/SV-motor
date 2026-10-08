# GLM-5.3 · Nueve preguntas del manual por MCP

08/10/2026 · Nodo 03 · API directa de Z.ai · Ensayo documental MD01–MD09.

**Banco completo: 27 entregas recibidas y adjudicadas. Resultado final: No apto para el contrato documental estricto, bajo reserva metodológica.** MD07 conserva el contenido nuclear correcto, pero añade una «s» a «vigente» en una cita requerida como literal. La regla crítica prefijada impide la admisión contractual; esta diferencia no demuestra incapacidad conceptual. La recepción científica independiente está pendiente.

**Revisión posterior de MD07:** [diagnóstico causal y reserva metodológica](revision-md07/REVISION-METODOLOGICA.md). Confirma un defecto de literalidad en apoyo adicional, fuente y transporte conformes y respuesta nuclear correcta. El dictamen y el vector históricos se conservan, sin nueva inferencia.

## Lectura del resultado

| Etapa | Vector MD01–MD09 | Correctas contractuales | Errores críticos | Admisión |
|---|---|---:|---:|---|
| R0, provisional | 0 0 0 0 0 0 0 0 0 | 9 | 0 | Apto para el contrato |
| R1, autocrítica | 0 0 0 0 0 1 0 0 0 | 8 | 1 | No apto |
| R2, verificación neutral | 0 0 0 0 0 0 1 0 0 | 8 | 1 | No apto, con reserva |

R2 es la entrega final. No se selecciona R0 por su mejor resultado ni se combinan etapas. Seis críticos: MD01, MD02, MD05, MD06, MD07 y MD09. T(9)=7; κ=Apto en las tres etapas, pero κ no sustituye el veto crítico del contrato. Célula (9,3), Σ={0,1,U}, 19.683 estados; pareja de representaciones matemática y visual vinculada por identidad. Radios 1/2/3 y colores rojo/verde/azul para 0/1/U, respectivamente. Los avisos triangulares son auxiliares y no alteran el vector.

- **MD06/R1:** falta el campo obligatorio `revision`; el contenido sobre incorporación previa mediante Wishlist/IRQ es correcto. R2 recupera el campo, sin reconocer expresamente la omisión anterior. Las tres entregas se conservan.
- **MD07/R2:** el original del constructor dice «a la gramática vigente»; el candidato cita «a la gramática vigentes». El cotejo Rust identifica una discrepancia literal, introducida al ampliar las evidencias en R2. Preservación semántica y coordinación se contestan correctamente. Debe revisarse la adecuación entre criticidad sustantiva y defecto formal antes de interpretar este resultado como incapacidad del candidato; el contrato y el original no se modifican retrospectivamente.
- **MD09:** el núcleo previsto/implementado y su alcance documental son correctos. Se conserva una imprecisión accesoria: llama subjuntivo a «deberá fijar», futuro de indicativo. No altera la conclusión sobre implementación, pero impide describir la explicación como exenta de toda imprecisión.

Las advertencias de MD01, MD03, MD08 y MD09 remiten a límites explícitos comprobados en las fuentes. Las preguntas tienen apoyo suficiente; no se exige una respuesta ausente ni se premia una U evasiva. Reconocer un estado preparatorio no acredita capacidades implementadas. El juicio sustantivo es exterior, asistido por IA y documentado; el comprobador Rust no decide verdad por coincidencia léxica.

## Suministro y método

Índice, presentación, constructor y tramo 10 íntegros: cuatro documentos, 31 secciones y 31 fragmentos. El MCP Rust los entrega al Árbitro, que incorpora el corpus completo al contexto. Se conservan los enunciados y los criterios históricos del manual y la corrección temporal del contrato recibida tras MD01: cada entrega se juzga según las obligaciones de su propia etapa. No se entregan claves de respuesta, evaluaciones previas ni advertencias sobre errores concretos.

El candidato recibe tres etapas siempre: respuesta, autocrítica y verificación neutral, con libertad para mantener, corregir o declarar U justificada. Son revisiones del mismo modelo, no tres auditores independientes. Este ensayo no demuestra estabilidad entre ejecuciones ni aptitud clínica o de ciberseguridad; CYB25 permanece separado y no ejecutado.

No se habilitaron herramientas ni navegación del candidato. El proceso MCP rechazó sockets locales y externos y la composición enviada se cotejó. Esto acredita el control del suministro y del contrato SV; no inspecciona la infraestructura interna de Z.ai ni excluye el conocimiento previo del modelo.

## Incidencia inicial y corrección instrumental

El primer intento MD01/R0 recibió HTTP 200, pero terminó con `finish_reason=length`: límite solicitado de 8.192 tokens, de los cuales 8.190 fueron declarados como razonamiento; no hubo texto final. Se conservó como intento no calificable, sin valor en el vector y sin atribuir una interrupción del proveedor.

La revisión 2 elevó el máximo solicitado a 16.384 y pasó el esfuerzo de razonamiento de max a high. Corpus, preguntas, criterios y tres etapas permanecieron iguales. El banco posterior completó las 27 solicitudes sin reintentos. El razonamiento que comunica el proveedor es un dato de salida; no acredita acceso a sus procesos internos completos.

## Instrumentación y trazabilidad

Banco completo: 740.803 ms (12 min 20,803 s), 621.932 tokens de entrada y 54.296 de salida: 676.228 en total. Intento inicial: 28.369 tokens y 116.367 ms. Conjunto: **704.597 tokens**, sin volver a sumar razonamiento o caché ya incluidos. Los tiempos de solicitudes y del banco no se confunden con la preparación y recepción.

Telemetría Rust del banco: 2.733 muestras, intervalo máximo 467 ms; intento inicial: 439 muestras, máximo 281 ms; suministro MCP: 29 muestras, máximo 296 ms. Sin fallos de medición declarados. Procesos, CPU, memoria, E/S, TCP, UDP, escuchas y marcas temporales se conservan en los originales y sus huellas. La integridad local no equivale a calibración o recepción independiente de toda la plataforma.

La excepción experimental de criptografía C/ensamblador continúa pendiente. No se ha acreditado retención cero: se suministra documentación experimental pública autorizada con aviso de derechos, sin datos sanitarios personales. El aviso de licencia no sustituye condiciones contractuales del proveedor.

## Consulta y conservación

- [Polígono egui autónomo](web/POLIGONO-EGUI.html): descargar y abrir; representación e interacción en Rust/WebAssembly. HTML aloja el visor y JavaScript mínimo inicia WebAssembly; no decide valores del SV.
- [Comparación R0/R1/R2](COMPARACION-RUST.json), [adjudicación final](adjudicacion/CAPA-R2.json), [revisión exterior](REVISION-EXTERIOR.json) y [cotejo de citas](COTEJO-DETALLADO-CITAS.json).
- [Banco de preguntas](BANCO.json), [mediciones](MEDICIONES.json) e [inventario Rust](INVENTARIO-RUST.json). Los 27 hitos tienen adjudicación, respuesta original, fundamento, identidad y mediciones en `adjudicacion/hitos/`.
- [Código y reproducción](CODIGO-Y-REPRODUCCION.md). La contabilidad por intento y la conciliación se conservan en el archivo administrativo privado; no se imputan importes desconocidos como cero.

Continuación: recibir la reserva metodológica y ajustar el presupuesto antes de decidir otro examen. Se mantienen los originales y las sedes de calidad; no se modifican portadas ni resultados de otros modelos.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
