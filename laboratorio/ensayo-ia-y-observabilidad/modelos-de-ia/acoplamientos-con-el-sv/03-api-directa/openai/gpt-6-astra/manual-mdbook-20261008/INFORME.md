# Ensayo documental del manual SVP · GPT-6 Astra · nodo 03

Edición 1.0.0 · 08/10/2026 · MD01–MD09, tres etapas universales.

**Resultado: No apto para el contrato documental completo de este ensayo.** Vector final R2: `(1,0,0,0,0,0,0,0,0)`. Ocho respuestas correctas, un error y ninguna U. La clasificación matemática κ es Apto porque ocho posiciones superan T(9)=7; la admisión es No apto porque MD01 es crítica y un solo error crítico es eliminatorio. Las seis criticidades se fijaron antes de la inferencia: MD01, MD02, MD05, MD06, MD07 y MD09. No se utiliza una puntuación ponderada inexistente en este banco.

Se juzga la entrega completa, incluida su revisión verificable. La respuesta principal de MD01 sobre el constructor y el manual es correcta. El error aparece en la verificación final cuando afirma que las etapas anteriores incumplían la obligación de usar etapa 2. Los originales R0 y R1 llevaban los valores 0 y 1, exigidos respectivamente. La conclusión no es un fallo demostrado de comprensión del manual ni un error médico. Es una afirmación inexacta sobre el historial de la propia prueba.

## Fuente, preguntas y condiciones

Cuatro documentos íntegros, 31 secciones Markdown: presentación, índice, constructor y tramo 10. El contenido se fija en la edición previa; no se interpreta como manual terminado ni como implementación recibida. Las nueve preguntas examinan naturaleza del constructor, subordinación documental, localización de materias, ficha de doce campos, cierre acumulativo, Wishlist/IRQ, preservación semántica, materias futuras y estado de implementación. No se pregunta por sintaxis o funciones todavía no desarrolladas.

El Árbitro suministró todo el corpus mediante el MCP Rust y compuso cada encargo. El candidato no navegó autónomamente por MCP: recibió el libro completo. Banco, clave, controles, criticidades y revisión externa quedaron separados; sólo fuente, pregunta, contrato y antecedentes de esa misma pregunta se suministraron al candidato.

Todas las preguntas tuvieron R0 provisional, R1 autocrítica y R2 verificación final neutral. Cada etapa entrega una respuesta autosuficiente; se permite mantener, corregir o declarar U fundada. Los resultados finales son siempre R2; no se elige retrospectivamente la mejor versión. Las revisiones son del mismo candidato, no tres evaluadores independientes.

## Evolución y atribución del defecto

| Etapa | Vector | Admisión al contrato |
|---|---|---|
| R0 | (0,0,0,0,0,0,0,0,0) | Apto |
| R1 | (0,0,0,0,0,0,0,0,0) | Apto |
| R2 | (1,0,0,0,0,0,0,0,0) | No apto: MD01 crítica |

El texto inexacto consta en FINAL.txt y concuerda con los eventos originales recibidos. Las 27 respuestas terminaron con HTTP 200 y response.completed; sus esquemas, citas y localizadores resultaron conformes. Esto acredita recepción e integridad, no corrección semántica automática.

La instrucción de etapa corresponde a la entrega actual. En R2 el historial incluye R0 y R1 como respuestas anteriores, y el encargo final exige etapa 2. Puede reforzarse la delimitación temporal de esa instrucción en un contrato futuro. Este ensayo no demuestra qué causa interna produjo la interpretación incorrecta ni permite excluir toda contribución de la redacción del encargo. Se conserva como limitación de diseño y no se cambia la instrucción ni la calificación retrospectivamente. MD04/R2 emplea una fórmula imprecisa sobre corregir la etapa, pero no afirma el incumplimiento retrospectivo concreto de MD01; se conserva la observación sin atribuirle el mismo error inequívoco.

La doble revisión no garantiza mejora: en esta ejecución introdujo una incorrección histórica tras dos entregas correctas. Esta observación no es una estimación estadística de estabilidad. Los resultados anteriores de Astra, incluido el examen documental de 25 preguntas, pertenecen a otros contratos y permanecen intactos.

## Instrumentación y medidas

Las 27 solicitudes se completaron en **779.679 ms** de duración total del banco, aproximadamente **12 min 59,7 s**. La suma de duraciones HTTP registradas es 670.096 ms; la suma de operaciones es 681.936 ms. Son intervalos distintos y no se suman entre sí. No hubo reintentos de inferencia ni interrupciones de servicio durante las preguntas.

Uso comunicado por OpenAI y cotejado desde SSE: **516.548 tokens de entrada, 23.988 de salida y 540.536 totales**. Entrada de caché y razonamiento: cero según los contadores recibidos; no significa ausencia de procesos internos. La cifra de entrada incluye las repeticiones del corpus completo y los antecedentes de cada pregunta. No equivale a palabras únicas del manual. Créditos descontados e importe atribuible no están comunicados: quedan pendientes de conciliación, sin aplicar una tarifa API ajena a esta autorización de acceso.

Se conservaron 23.853 eventos SSE, 6.942.880 bytes SSE y 2.826 muestras del proceso cliente. Intervalo máximo entre muestras: 392 ms, frente a 250 ms de objetivo y 750 ms de límite contractual. Sin fallos de medición declarados. Los informes por intento recogen tiempo hasta primer texto, CPU, máximo de memoria residente, lectura/escritura del proceso, estados TCP, puertos observados y UDP, además de huellas de solicitud, respuesta, eventos y telemetría. Los contadores de E/S no equivalen exclusivamente a tráfico de red. La ausencia de sockets UDP en las muestras no demuestra su inexistencia entre muestras.

Inventario efectivamente utilizado:

| Componente | Función y evidencia |
|---|---|
| Preparador y MCP Rust | Catálogo y cuatro fuentes fijados; 31 secciones, 34 tramas y 104 sucesos de suministro reconstruidos; rechazo de sockets en el proceso aislado. |
| Árbitro/controlador Rust | Banco y clave separados; contexto íntegro; identidad de fuentes y ejecutable antes de cada envío; historia por pregunta; límites, cola de incidencias y custodia de entregas. |
| Cliente Rust | Acceso existente, HTTPS al destino autorizado, esquema de respuesta estricto, herramientas vacías, sin reintento automático. Modelo solicitado y declarado: gpt-6-astra en las 27 entregas. |
| Instrumentación Rust | Muestreo del cliente Windows antes, durante y después; 20 muestras adicionales en la preparación MCP. No se confunden con las 2.826 del candidato. |
| Recepción Rust | Cotejo de originales, historias, SSE, texto, uso, citas, identidad y medidas; adjudicación exterior al candidato incorporada con huellas. |
| Rust/egui y WebAssembly | Representación del vector, criticidades, pareja matemática/visual, evolución, respuestas y advertencias. JavaScript mínimo para inicialización y presentación, sin autoridad SV. |

No se miden por separado DNS, TLS, RTT, retransmisiones, asignaciones del montículo, hilos ni recursos internos de OpenAI. El muestreo se refiere al cliente, no a un inventario de todos los procesos del equipo. La instrumentación no tiene recepción metrológica independiente. El proveedor no entregó un resumen de razonamiento en estas respuestas; los fundamentos documentales verificables se conservan, sin presentarlos como pensamiento interno.

## Red, interpretación y representación

Las solicitudes prohíben información externa y deshabilitan herramientas; no se ejecutó ninguna herramienta del candidato. El MCP rechazó la creación/conexión de sockets. Son controles acreditados del SV; no prueban aislamiento interno de los servidores de OpenAI ni eliminan conocimiento previo de los pesos. No se identificó una premisa externa necesaria en las respuestas examinadas.

Las advertencias de MD01, MD08 y MD09 remiten a límites expresos del documento, cotejados en Rust. No convierten una deducción válida en falta de evidencia, no alteran el vector y no penalizan el reconocimiento correcto de un estado preparatorio. Las preguntas tienen soporte suficiente.

El polígono (9,3) usa la convención del logo SV: **0 rojo, radio 1; 1 verde, radio 2; U azul, radio 3**. Verde no significa admisión favorable. La pareja matemática/visual y sus fuentes se incluyen en DICTAMEN.json y en el visor; un error crítico prevalece sobre κ. Se muestra sólo después de adjudicar las nueve posiciones.

## Comprobaciones, conservación y límites

Comprobaciones Rust finales: 53 del controlador y contratos, 13 de recepción, 4 de instrumentación y 10 del visor, incluyendo recorrido de los 19.683 estados ternarios y veto crítico. Los conjuntos comparten módulos y no se suman como cobertura independiente. Se conservaron los intentos auxiliares fallidos: caché Cargo incorrecta, corrección de compilación del receptor, expectativas antiguas de una comprobación visual y selección inicial de una herramienta sin destino WebAssembly. Se utilizó Rust 1.98.0 ya instalado para el visor; no se instaló software para resolverlo. Ninguna de estas incidencias ocasionó nuevas llamadas al candidato.

Los resultados públicos se alojan aquí, en el expediente de Astra del nodo 03, junto con código, banco, medidas, originales de respuesta, hitos y polígono. La documentación de admisión mantiene su fecha y significado histórico. Las mediciones económicas y fuentes operativas completas se archivan en el repositorio privado de usos, gastos, créditos y tokens; no se publican credenciales, saldos ni clave reservada. Las adaptaciones de rutas en fuentes públicas se declaran en el inventario de publicación y no se confunden con los bytes compilados.

La revisión sustantiva es exterior al candidato y asistida por IA; la recepción científica independiente queda pendiente. No acredita aptitud clínica, implementación del Lenguaje, universalidad del acoplamiento ni estabilidad entre ejecuciones. La excepción experimental de criptografía nativa con C/ensamblador permanece pendiente. Este cierre no autoriza por sí mismo otra inferencia, contratación o repetición.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
