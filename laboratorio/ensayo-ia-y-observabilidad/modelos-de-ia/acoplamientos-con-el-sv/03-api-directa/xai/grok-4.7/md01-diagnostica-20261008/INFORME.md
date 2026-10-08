# Grok 4.7 · Diagnóstico MD01 mediante MCP Markdown

08/10/2026 · Nodo 03, API directa de xAI. **Diagnóstico parcial: no se emite conformidad del conjunto.** Se recibieron R0 y R1; la verificación final R2 no se envió porque su reserva previa no cabía en el margen autorizado. No hubo rechazo del proveedor ni reintento. Se conservan los originales y los dictámenes anteriores.

## Pregunta y resultado

«¿Qué función cumple el constructor y qué diferencia establece respecto de un manual final?» La clave prefijada exige distinguir la arquitectura preparatoria y el cierre progresivo de un manual terminado, sin atribuir implementación. MD01 es crítica: un error impediría la conformidad; U tampoco la acredita.

| Etapa | Valor | Recepción y alcance |
|---|---|---|
| R0 · inicial | 0 | Distingue función y estatuto documental; su enumeración adicional mezcla algunas denominaciones resumidas del constructor y del índice. Se registra esta precisión sin convertir una paráfrasis en error del núcleo preguntado. |
| R1 · autocrítica | 0 | Mantiene el núcleo y distingue los títulos de las fuentes. Su revisión se refiere al antecedente efectivo y conserva su etapa histórica. |
| R2 · verificación neutral | No ejecutada | Reserva insuficiente antes del envío. No es U ni error del candidato. |

La adjudicación es exterior al candidato y asistida por IA. Rust coteja estructura, localizadores, huellas, procedencia de la revisión y aplica los valores revisados; no sustituye el juicio semántico por una comprobación de cadenas. La recepción científica independiente queda pendiente. Las etapas de una pregunta no forman tres preguntas ni un polígono de nueve.

⚠ La fuente declara un manual preparatorio y un tramo 10 pendiente; no acredita una implementación. La pregunta sí tiene respuesta explícita en el constructor. Los ceros reconocen esa distinción y no una capacidad inexistente, ni castigan deducciones fundadas.

## Suministro y comparación

Misma pregunta, cuatro documentos y 31 secciones íntegras de la réplica MD01 de Astra. Fuentes históricas del manual: Lenguaje en revisión f97173e07a9dfb8385ed1268d11de422b291507d. El Árbitro reconstruyó de nuevo el suministro mediante MCP Rust, comprobó los fragmentos, las huellas y el diario. Se conservó el contrato temporal corregido y se cotejó cada instrucción histórica con la enviada realmente, incluida la licencia.

El modelo recibió el corpus completo, no herramientas de consulta autónoma. Las solicitudes prohibieron navegación y fuentes externas; tools=[], store=false. El proveedor comunicó cero herramientas y cero fuentes externas, y confirmó retención cero. El aislamiento de sockets se comprobó para el proceso MCP local; no demuestra inspección de los servidores de xAI.

## Medición efectiva

| Magnitud | R0 | R1 |
|---|---:|---:|
| Tokens de entrada | 21.546 | 25.217 |
| Tokens de salida, incluido razonamiento | 7.779 | 12.247 |
| Razonamiento incluido en salida | 5.118 | 8.836 |
| Total | 29.325 | 37.464 |
| Duración de operación, ms | 91.876 | 148.860 |
| Primer texto, ms | 64.239 | 117.640 |
| Muestras locales | 347 | 561 |

Total: 66.789 tokens; banco 243.560 ms; 908 muestras, intervalo máximo 367 ms y cero fallos. MCP previo: 25 muestras, máximo 274 ms, separado del cliente. Dos HTTP 200 / response.completed; los originales SSE, texto final y contadores concuerdan. Caché y razonamiento están incluidos en sus categorías, no se suman dos veces. Cada etapa observó una conexión TCP simultánea Established; memoria máxima observada 22.745.088 bytes. CPU, memoria, E/S y red se conservan en las mediciones Rust por etapa. No son recursos internos del proveedor.

## Corrección necesaria del control económico

En R1 se solicitó max_output_tokens=8192 y la respuesta reprodujo ese parámetro, pero usage.output_tokens fue 12247, incluidos 8836 de razonamiento. La [documentación vigente de xAI](https://docs.x.ai/developers/rest-api-reference/inference/responses), consultada el 08/10/2026, explica que el parámetro limita sólo la salida visible, excluyendo razonamiento y llamadas de función. La salida comunicada menos razonamiento fue 3411, dentro de la cota visible.

Por tanto, era incorrecto considerar esa cota como límite de toda la salida facturable en la reserva común. No se atribuye al proveedor un incumplimiento de su semántica documentada. El gasto efectivo quedó dentro del límite humano y R2 no se envió, pero no se declara demostrada una garantía absoluta del coste de cada solicitud. El control necesita una reserva válida para el razonamiento o un límite económico efectivo del proveedor antes de otra campaña. No se cambiaron las condiciones de la prueba ni sus originales retrospectivamente. Los importes e intentos se conservan en el archivo privado.

## Componentes y custodia

[Inventario y comprobaciones](INVENTARIO-Y-COMPROBACIONES.md) · [Dictamen Rust](DICTAMEN.json) · [Revisión sustantiva](REVISION-SUSTANTIVA.json) · [Cotejo del límite](LIMITES-PROVEEDOR.json).

El suministro, contrato y localizadores se incorporan a la sede del cliente común Rust; no se crea otro transporte por proveedor. El acceso, HTTPS/SSE, credencial e instrumentación reutilizan lo recibido. Criptografía nativa pendiente bajo la excepción experimental. Las fuentes históricas permanecen intactas. Los archivos públicos conservan código, adjudicaciones y mediciones sin reproducir respuestas originales, direcciones, identificadores de cuenta ni credenciales. Las respuestas, la presentación con su texto y los registros operativos íntegros permanecen conservados localmente; no se afirma que estén publicados. Los dictámenes públicos son copias derivadas que excluyen el texto de las respuestas y declaran la huella de su original.

Conservar los originales; recibir la revisión competente y resolver el control económico antes de proponer otra ejecución. No hay continuación automática ni ampliación de gasto.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).