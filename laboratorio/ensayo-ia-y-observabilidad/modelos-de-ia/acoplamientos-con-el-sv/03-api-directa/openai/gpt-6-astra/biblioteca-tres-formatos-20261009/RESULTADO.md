# Consulta de caché, PDF y Markdown mediante el MCP

**9 de octubre de 2026 · Nodo 3 · OpenAI, gpt-6-astra.** Resultado favorable para esta comprobación delimitada: el candidato encontró la información solicitada en los tres documentos y la respaldó con cuatro citas literales de secciones efectivamente suministradas. No constituye un nuevo examen ni modifica las calificaciones anteriores.

## Resultado documental

| Fuente | Información recuperada | Cotejo exterior de significado |
|---|---|---|
| Caché HTML NCI-PDQ profesional | Linaje B; neoplasia de grado bajo y crecimiento lento. | Conforme con el pasaje suministrado. Se conserva la versión profesional histórica, sin sustituirla por la página actual para pacientes. |
| PDF LLS, FS16S8/18 | Los puntos clave denominan también la enfermedad «leucemia de células pilosas» y «tricoleucemia». | Conforme con la primera página física. No es una recomendación clínica actual. |
| Banco Markdown CYB25 | C02 distingue versión 5 instalada en disco de versión 4 todavía ejecutada; el enunciado sitúa la corrección en la versión 5 activa. | Conforme con el enunciado. Recuperación de información, sin resolución ni repetición del examen. |

El cotejo literal se ejecutó en Rust y se repitió después reconstruyendo los eventos originales del transporte. La revisión de significado es una revisión asistida exterior al candidato; no se presenta como recepción científica humana independiente. El aviso inicial de la pantalla «revisión semántica exterior pendiente» reflejaba el estado anterior a esta revisión y se conserva en el original.

## Recorrido y controles

El SV entregó el índice inicial. Astra solicitó después otro índice y seis secciones de contenido: cuatro de HTML, una de PDF y una de Markdown. El controlador comprobó cada identificador contra el catálogo fijado, y el MCP devolvió las secciones completas. Fueron ocho lecturas MCP y ocho solicitudes al proveedor, incluida la respuesta final; ninguna repetición de inferencia.

Las ocho solicitudes desactivaron herramientas del proveedor, navegación y almacenamiento de la respuesta mediante los parámetros del contrato. El texto prohibió seguir enlaces o completar con fuentes externas. El proceso documental local se ejecutó con el aislamiento de red ya recibido. Se verificaron las cadenas de entrega: cada contexto nuevo contiene exactamente la respuesta anterior y la lectura admitida por el controlador.

Esto acredita el recorrido controlado por el SV y la ausencia de herramientas externas en los eventos recibidos. No permite observar los procesos internos del proveedor ni demuestra la eliminación del conocimiento adquirido durante el entrenamiento. La conexión HTTPS del nodo 3 sigue siendo necesaria para enviar y recibir; no confiere permiso documental al modelo.

## Mediciones

| Magnitud | Resultado |
|---|---:|
| Solicitudes completas / HTTP 200 | 8 / 8 |
| Tokens de entrada | 32.792 |
| Tokens de salida | 847 |
| Total comunicado | 33.639 |
| Entrada en caché comunicada | 0 |
| Razonamiento comunicado, incluido en la salida | 38 |
| Suma de duraciones de las operaciones | 53,827 s |
| Intervalo exterior entre inicio y cierre de observación | 66,669 s |
| Muestras del proceso cliente | 228 |
| Registros de instrumentación | 1.807 |
| Fallos de medición registrados | 0 |

Período observado: 19:44:35,051–19:45:41,720 UTC; 21:44:35,051–21:45:41,720 en Europe/Madrid. El intervalo exterior incluye los pasos intermedios y no equivale al tiempo de cómputo del modelo. El total de entrada incluye el historial reenviado en cada solicitud; no representa 32.792 tokens documentales únicos. Los contadores de razonamiento son una parte de la salida y no se suman otra vez.

El importe atribuible no está determinado. No se aplica una tarifa de API como si fuera un cargo observado. El archivo administrativo conserva los consumos por solicitud y distingue uso del candidato, cuota del plan, asistencia e infraestructura; estas últimas magnitudes no se han imputado como cero.

## Incidencias y límites conservados

1. **Conversión PDF rechazada antes del envío.** La salida inicial de Xberg desplazaba una palabra entre los puntos clave y la introducción. No se suministró al candidato. La edición recibida reutilizó la extracción Rust histórica del mismo PDF, cotejada página a página y ligada a su huella. No se acredita una conversión PDF general ni la comprensión de figuras.
2. **Referencias de imágenes del HTML.** La primera preparación fue detenida por el lector. La edición siguiente conservó texto alternativo y destino como referencias textuales identificadas como recursos visuales no interpretados. No hubo descarga ni interpretación de imágenes.
3. **Autodescripción del candidato.** Su informe operativo habla de un índice y siete secciones documentales. La traza acredita dos índices y seis secciones de contenido; siete lecturas fueron solicitadas por él y la inicial fue suministrada por el SV. Esta discrepancia impide tratar el autorreporte como un contador fiable. No afecta a las cuatro citas ni a las tres respuestas documentales.

La recepción preparatoria completa comprendió cinco entradas documentales —tres fuentes y dos índices—, 71 secciones, 115 solicitudes MCP y tres rechazos deliberados. La regresión pasó veinte comprobaciones de biblioteca y cincuenta y nueve del cliente/acceso. La producción permanece excluida: esta prueba no acredita un Árbitro integral ni la generalidad de todos los formatos.

## Continuación técnica

La navegación por índice y secciones queda comprobada en esta edición. La siguiente necesidad es corregir y contrastar la conversión PDF antes de generalizarla, manteniendo la alternativa ya recibida. La incorporación de bibliografía especializada y cualquier otra inferencia requieren su propio alcance. Las fuentes, los permisos y el dictamen permanecen bajo control del SV.

Evidencia numérica: [cotejo posterior Rust](COTEJO-EJECUCION-RUST.json). Preparación y huellas de originales: [preparación](PREPARACION.md). La custodia de un manifiesto de huellas no acredita por sí sola la custodia remota de todos los originales descritos.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
