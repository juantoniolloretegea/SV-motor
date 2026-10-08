# Réplica diagnóstica de MD01 · GPT-6 Astra · nodo 03

08/10/2026 · Contrato 1.1.0 · Una pregunta y tres etapas universales.

**Dictamen: conforme para la réplica diagnóstica de MD01.** R0, R1 y R2 reciben valor **0: correcto y completo**. Se mantienen la criticidad de MD01 y la exigencia conjunta de contenido, estructura y revisión correctos. La revisión sustantiva es exterior al candidato y asistida por IA; no constituye recepción científica independiente ni aptitud clínica.

## Pregunta y evidencia

«¿Qué función cumple el constructor y qué diferencia establece respecto de un manual final?»

La fuente recibida describe la arquitectura de elaboración, la organización por tramos, las condiciones de cierre y su subordinación documental. Expresa que el constructor todavía no es el manual final. La edición fija cuatro documentos y 31 secciones; el Árbitro los recibe íntegros por MCP y los preentrega al candidato. No se le exige completar capítulos vacíos ni acreditar implementaciones ausentes.

⚠ **Límite documental comprobado:** el constructor no acredita un manual terminado, aprobación ni implementación. Aquí la pregunta exige distinguir esos estados y dispone de información suficiente. La advertencia delimita el alcance de la respuesta; no identifica por sí sola un defecto de diseño ni penaliza razonamiento fundado. Ninguna etapa necesitó declarar U.

## Resultado y revisión

| Etapa | Contenido | Estructura | Revisión de antecedentes | Valor |
|---|---|---|---|---|
| R0 · provisional | Correcto | Conforme | Correcta: sin antecedentes | 0 |
| R1 · autocrítica | Correcto | Conforme | Amplía fundamentos sin invalidar el identificador 0 | 0 |
| R2 · verificación neutral | Correcto | Conforme | Conserva la validez de los identificadores 0 y 1 | 0 |

R1 incorpora coordinación interlenguajes y control de nuevas ideas, ambos documentados. R2 conserva esas precisiones por su evidencia y diferencia «cerrable» de «cerrado», sin atribuir a R0 un cierre efectivo que no afirmó. Su revisión dice: «Los identificadores históricos 0 y 1 permanecen válidos para sus respectivas entregas».

[Revisión sustantiva](REVISION-SUSTANTIVA.json), [dictamen derivado en Rust](DICTAMEN-REPLICA.json) y [presentación autónoma descargable](web/RESULTADO-MD01.html). Los tres valores son etapas temporales de una sola pregunta: no constituyen tres componentes de un vector ni un polígono de nueve posiciones.

## Modificación experimental y antecedente

La [adenda metodológica anterior](../revision-metodologica-20261008/REVISION-METODOLOGICA.md) confirma una deficiencia propia: el contexto de R2 reemplazaba el encargo real R1 por un resumen y no conservaba todas sus instrucciones históricas. La obligación vigente de numerar la entrega no explicitaba su alcance no retroactivo. El corpus y las respuestas anteriores sí se transmitieron íntegros. El cotejo retrospectivo no encontró pérdida documental, truncamiento ni incumplimiento de los identificadores originales.

La réplica conserva pregunta, modelo, corpus, formato, parámetros y secuencia. Cambia la composición temporal: conserva los encargos e instrucciones anteriores íntegros como datos históricos, las respuestas sin alteración y el alcance de cada identificador. La clave y el error observado no se suministran al candidato. El ejecutable y sus fuentes quedaron fijados antes de inferencia; la recepción los cotejó después.

El resultado acredita resolución bajo este contexto corregido. **Una repetición no demuestra causalidad exclusiva ni estabilidad estadística**, y no separa el efecto de cada precisión temporal introducida. La frase inexacta del ensayo anterior permanece conservada. Su «No apto» está bajo reserva metodológica: no se sustituye por una admisión general ni se mezcla este resultado con sus otras ocho preguntas. El examen de 25 preguntas y demás contratos anteriores no se modifican.

## Control e instrumentación efectivamente utilizados

| Elemento | Realización y evidencia |
|---|---|
| Árbitro y secuencia | Controlador Rust `astra-md01-replica`; admisión, fuentes fijadas, pregunta única, R0/R1/R2, límites y clave excluida del candidato. |
| Suministro documental | Servicio MCP Rust y verificador de diario: 4 documentos, 31 secciones y fragmentos, 34 tramas y 104 sucesos; reconstrucción conforme. |
| Separación de red | Rechazo de creación/conexión de sockets comprobado para el proceso MCP Linux. No representa aislamiento de OpenAI. |
| Transporte | Cliente Rust del acceso autorizado; modelo solicitado y declarado `gpt-6-astra`. Tres HTTP 200 y `response.completed`; sin reintentos ni interrupciones. |
| Instrumentación cliente | Muestreo Rust de proceso, CPU, memoria residente, E/S, conexiones TCP/UDP, estados y puertos; originales completos en custodia privada. |
| Integridad y entrega | Recepción Rust de solicitudes, contexto, respuestas, secuencia SSE, uso declarado, JSON y localizadores. SHA-256 de originales y fuentes fijadas. |
| Revisión y decisión | Revisión sustantiva exterior identificada; Rust verifica su correspondencia con cada original y aplica los requisitos prefijados. No demuestra semántica por sí solo. |
| Presentación | HTML estático generado por Rust, sin JavaScript, inferencia ni autoridad decisoria. No se genera otro polígono para una pregunta aislada. |

El candidato recibió prohibición expresa de navegar y consultar fuentes externas, con `tools=[]` y `tool_choice=none`. No se habilitó una herramienta de red ni navegación MCP autónoma. Esto no inspecciona ni acredita el aislamiento físico de la infraestructura del proveedor, y no elimina conocimientos previos de entrenamiento. Se exigieron fundamentos apoyados en el corpus suministrado.

La dependencia criptográfica C/ensamblador sigue pendiente bajo la excepción experimental autorizada. La instrumentación propia está realizada en Rust; las API del sistema operativo y la infraestructura remota no se presentan como íntegramente Rust.

## Mediciones y consumo

| Etapa | Entrada | Salida | Total tokens | HTTP (ms) | Primer texto (ms) | Muestras | Intervalo máximo (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|
| R0 | 18.348 | 1.028 | 19.376 | 31.365 | 5.579 | 127 | 289 |
| R1 | 20.165 | 1.645 | 21.810 | 41.503 | 3.519 | 167 | 289 |
| R2 | 22.597 | 1.762 | 24.359 | 39.674 | 3.621 | 158 | 288 |
| Total | **61.110** | **4.435** | **65.545** | **112.542** | — | **452** | **289** |

Duración total del banco: **126.191 ms, 2 min 6,191 s**. El tiempo HTTP agregado no incluye todo el control y no se confunde con la duración del banco. Cero fallos de medición. Preparación MCP separada: 21 muestras, máximo 287 ms, cero fallos. Flujos recibidos: 4.420 sucesos SSE y 1.265.621 bytes. Los máximos de memoria residente por etapa son 35.610.624, 36.745.216 y 37.539.840 bytes; seis conexiones TCP simultáneas como máximo en cada entrega. Son medidas del cliente, no de las GPU o procesos internos del proveedor.

[Recepción íntegra derivada](RECEPCION-RUST.json) y [métricas Rust](METRICAS-RUST.json), con mediciones por hito y huellas. El proveedor declara cero tokens de caché y cero tokens de razonamiento; no se suman otra vez a los totales. No entregó resumen de razonamiento. Estos contadores no demuestran ausencia de razonamiento interno. Las explicaciones documentales de las respuestas permiten contrastar afirmaciones, no observar procesos internos no entregados.

Importe, impuestos, créditos descontados y consumo de asistencia atribuible: **desconocidos, pendientes de conciliación**. No se aplica una tarifa API pública a este acceso ni se convierte automáticamente tokens en créditos. Cada una de las tres solicitudes se archiva por separado en el repositorio privado de gastos; preparación y recepción no añaden inferencias del candidato. No hubo pagos, recargas ni ampliación de recursos.

## Conservación y retorno

Los originales se conservan sin reparación, selección de mejor respuesta ni repetición adicional. Cada hito conserva su estado instrumental inicial y una adjudicación posterior separada. La sede pública contiene admisión, fuentes del controlador, respuestas, dictamen, mediciones y proyección documental; la custodia privada incorpora solicitudes, SSE y observaciones operativas completas con manifiesto. Credenciales, registro de autorización y clave reservada quedan excluidos. [Dependencias y reconstrucción](RECONSTRUCCION.md).

Retorno: recepción competente de este diagnóstico y conciliación económica cuando exista información atribuible. No se inicia otro banco, anexo ni inferencia por este cierre.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
