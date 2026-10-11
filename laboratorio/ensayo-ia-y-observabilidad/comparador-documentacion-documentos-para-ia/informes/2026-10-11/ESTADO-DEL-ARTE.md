# Revisión de soluciones para búsqueda y contraste documental sin IA

**Antecedente conservado; no es la propuesta vigente.** La propuesta inicial fue retirada. La [estructura del 11/10/2026](ESTRUCTURA-LOGICA-RUST-MDBOOK.md) incorpora las precisiones posteriores: Medicina y Ciberseguridad, mdBook como presentación y traducción Google aceptada como instrumento externo para comparar en inglés. Las exclusiones generales de traducción o IA de este antecedente corresponden a su corte anterior; permanece excluida la IA como detector o juez del contraste.


**10 de octubre de 2026. Estudio y propuesta de integración; no es una recepción de la aplicación.**

**Estado vigente tras revisión humana: NO APTO; propuesta de integración retirada.** La propuesta que sigue se conserva como antecedente. Su dependencia de vocabularios y relaciones preparados por personas traslada a éstas una parte esencial del trabajo solicitado. Tantivy y la comparación textual no acreditan descubrimiento autónomo de posibles contradicciones. La precisión y la comprobación adicional figuran en [REVISION-DE-APTITUD.md](REVISION-DE-APTITUD.md); este documento no constituye una recomendación vigente de implementación.

## Necesidad y condiciones

Se necesita una aplicación que ayude a una persona competente a localizar, examinar y gestionar contradicciones dentro de un documento y entre varios documentos HTML, PDF y Markdown. Debe conservar el contexto, la edición, la localización original y la resolución humana antes de habilitar el corpus para un uso documental posterior.

La aplicación y su recorrido de control se realizarían en Rust. Queda excluida toda asistencia de IA, interna o externa: modelos generativos, modelos de clasificación, representaciones vectoriales obtenidas mediante modelos y reconocimiento de imágenes mediante modelos. La expresión «Buscador-Semántico - Diferencial» designa aquí una búsqueda apoyada en conceptos y relaciones explícitos aprobados por personas, junto con comparación documental reproducible. No implica una interpretación automática general del significado.

La sede prevista es la administración existente en **administracion.itvia.online**. Se conservarían el Árbitro-Director, el aislamiento por dominio, las funciones humanas, la persistencia administrativa y la trazabilidad ya establecidos. El estudio no propone modificar el núcleo, su semántica ni su representación intermedia.

## Método y alcance del estudio

Se consultaron en Internet los repositorios y la documentación de sus responsables, las revisiones y los metadatos de actividad disponibles, y los antecedentes locales de extracción y lectura. Se distinguen las capacidades publicadas, el código examinado y las comprobaciones históricas efectivamente ejecutadas. No se instalaron ni ejecutaron las aplicaciones comparadas en esta actuación.

La conclusión es delimitada: **no se ha verificado una aplicación completa que reúna el recorrido solicitado, ejecución íntegra en Rust y ausencia total de asistencia de IA**. Sí existen componentes comunitarios mantenidos que evitan desarrollar de nuevo la extracción, la búsqueda y la comparación textual. La búsqueda no demuestra que no exista alguna otra solución.

## Incorporaciones existentes que deben reutilizarse

**Xberg ya forma parte del trabajo documental.** El manifiesto y el registro de dependencias del conversor documental examinado fijan **Xberg 1.3.6**, sin funciones predeterminadas y con las funciones `tokio-runtime`, `html`, `office` y `pdf`. Los antecedentes registran un perfil sin IA ni OCR. No procede presentarlo como una incorporación nueva.

La recepción sintética corregida registró 22 de 22 comprobaciones de contenido. Ese resultado tiene límites: el PDF real de tricoleucemia produjo una alteración del orden de lectura; su salida Xberg fue rechazada y se reutilizó la extracción Rust histórica cotejada. En otro PDF de nueve páginas se registró omisión de texto visible en la portada. Ninguno de esos antecedentes acredita conversión PDF universal.

El lector Markdown existente ya usa **pulldown-cmark 0.13.0** y conserva posiciones del texto original. El lector PDF existente usa **pdf-extract 0.12.1-sv.2**, con correcciones delimitadas, control de huellas y rechazo de pérdidas de extracción. Sus límites incluyen 64 páginas y 1 MB de texto; un documento extenso que los supere no está admitido por este estudio. Se conservarán los límites hasta una comprobación específica autorizada. Una página sin texto extraído no equivale a una página sin contenido.

La administración Axum/redb y los lectores documentales permanecen como base. El índice de búsqueda no sustituiría a la persistencia de permisos, ediciones o decisiones. Los visores egui históricos de resultados tampoco se presentan como un visor PDF general ya recibido.

## Componentes comunitarios pertinentes

| Componente | Capacidad pertinente | Decisión y límite |
|---|---|---|
| [Xberg](https://github.com/xberg-io/xberg) | Extracción multiformato. La [documentación Rust actual](https://docs.xberg.io/guides/rust-core-api/#diff) describe comparación estructurada de texto, tablas, metadatos y documentos incorporados. | Reutilizar el perfil existente. Comprobar primero si esa comparación está disponible y es adecuada en la versión 1.3.6 fijada. No se ha habilitado ni probado aquí; la documentación actual no acredita por sí sola esa versión. |
| [Tantivy](https://github.com/quickwit-oss/tantivy) | Biblioteca Rust de búsqueda textual, frases, consultas booleanas, campos y clasificación BM25. | Candidato principal para el índice. Permite localizar pasajes, pero su puntuación no mide contradicción, verdad ni cobertura completa. Debe comprobarse su alojamiento efectivo antes de integrarlo con el recorrido nativo y Workers. |
| [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) | Lectura estructurada de Markdown y posiciones originales. | Reutilizar la versión ya incorporada. No sustituye la revisión científica ni la protección de contenido HTML presentado. |
| [scraper](https://github.com/rust-scraper/scraper) | Lectura de HTML mediante estructura y selectores. | Reserva si la extracción existente no conserva una estructura necesaria. Evitar un segundo lector HTML sin necesidad acreditada. |
| [pdf-extract](https://github.com/jrmuizel/pdf-extract) | Extracción textual PDF en Rust, incluida extracción por páginas. | Conservar el lector corregido y sus controles. No es un visor gráfico ni garantiza el orden de lectura de todos los PDF. |
| [Similar](https://github.com/mitsuhiko/similar) | Comparación Rust por líneas, palabras y caracteres. | Alternativa si la comparación de Xberg no basta. No añadir dos motores de comparación para la misma función. |
| [imara-diff](https://github.com/pascalkuthe/imara-diff) | Comparación textual Rust con algoritmos apropiados para distintos tamaños de entrada. | Alternativa técnica a Similar, no un componente adicional obligatorio. Requiere comprobar tiempo, memoria y casos difíciles en el corpus previsto. |
| [Hayro](https://github.com/laurenzv/hayro) | Interpretación y representación gráfica PDF en Rust. | Reserva para una necesidad de visualización que no satisfaga el componente existente. No sustituye la extracción recibida ni se incorpora sin comprobación. |

La actividad pública observada el 10/10/2026 respalda especialmente a Tantivy y Xberg: aproximadamente 16.202 y 9.400 estrellas, respectivamente, y revisiones recientes. Son señales de adopción y mantenimiento, **no acreditaciones de corrección ni garantías científicas**. Las revisiones consultadas figuran en [FUENTES.json](FUENTES.json). Las licencias MIT, ISC y Apache-2.0 de los componentes conservan su propia autoridad; el pie de este estudio no las sustituye.

En la comparación actual de Xberg, `max_content_chars` puede recortar el texto antes del contraste. Esa opción deberá quedar sin recorte o declarar explícitamente la parte excluida. Un contraste parcial no se registrará como lectura o comparación íntegra. El manifiesto consultado de la revisión comunitaria 1.3.7 declara Similar entre sus dependencias; es otra razón para comprobar la función existente antes de incorporar una biblioteca paralela.

## Aplicaciones próximas al problema, pero inadecuadas para este alcance

| Solución | Aportación al estado del arte | Razón para no adoptarla |
|---|---|---|
| [Groundline](https://github.com/guilhermecariru/GroundLineOSS) | Revisión humana de afirmaciones y diferencias, evidencias localizadas y comparación documental. Es la referencia funcional más cercana entre las examinadas. | El análisis usa Python/FastAPI y la aplicación TypeScript. La envoltura Tauri no convierte ese análisis en Rust. La revisión consultada no aporta evidencia de una comunidad consolidada ni recepción independiente suficiente. |
| [RAGLint](https://github.com/Prashanth1998-18/raglint) | Busca conflictos entre fragmentos de un corpus. | Usa Python, representaciones vectoriales de un modelo y un modelo generativo para clasificar contradicciones. Queda excluido. |
| [FactLens](https://github.com/pari-28/FactLens) | Propone comparar valores, unidades, períodos y ámbitos para ordenar alertas. | La extracción de afirmaciones depende de Gemini y la ejecución usa TypeScript. Se declara prototipo, no aplicación recibida para este recorrido. |
| [falsify](https://github.com/crodorg/falsify) | Biblioteca/programa Rust que verifica estructura de evidencias, citas y huellas. | Su documentación distingue verificar de descubrir: la localización y el razonamiento dependen de agentes de IA. No satisface la exclusión de asistencia ni el visor multiformato solicitado. |
| [Draftable](https://api.draftable.com/introduction) | Comparación visual de pares de documentos con presentación de diferencias. | Servicio propietario; no acredita ejecución de nuestro control en Rust ni búsqueda de contradicciones en un corpus. No se contrató ni se remitieron documentos. |

También se revisaron bibliotecas y plataformas de búsqueda, representación PDF y comparación sintáctica. Una plataforma de recuperación con modelos no resuelve este encargo. La diferencia entre árboles sintácticos de código tampoco acredita una contradicción entre afirmaciones científicas.

## Propuesta de una sola aplicación

La opción más unificada es **añadir una función de revisión documental a la administración existente**, con el Buscador-Semántico - Diferencial como servicio Rust. Xberg y los lectores admitidos prepararían el contenido; Tantivy localizaría pasajes; la comparación de Xberg se examinaría antes de recurrir a Similar. La persona vería una sola aplicación, con separación efectiva de permisos, documentos e índices por dominio.

Su funcionamiento propuesto es:

1. Fijar el corpus, las ediciones, sus huellas y la cobertura de extracción. Conservar títulos, secciones, páginas, tablas, notas y condiciones. Toda omisión relevante impide declarar una revisión completa.
2. Localizar pasajes mediante términos, frases y vocabularios controlados aprobados por personas. Agruparlos por concepto, objeto, ámbito y período. Registrar los filtros y la parte del corpus comparada.
3. Presentar juntos los pasajes candidatos y sus originales, con suficiente contexto. Las reglas explícitas podrán señalar diferencias de cifras, unidades, fechas, obligaciones, prohibiciones y condiciones. Producirán **alertas de posible conflicto**, nunca dictámenes científicos automáticos.
4. Permitir que la persona registre conflicto confirmado, diferencia explicada, falsa alerta o revisión pendiente, indicando fundamento, pasajes y edición. La revisión debe cubrir contradicciones internas, entre documentos y condicionadas por un tercer documento, no sólo pares de frases.
5. Resolver mediante una decisión documentada: nueva edición, aclaración, sustitución o delimitación del corpus. Conservar originales y antecedentes. Una edición o retirada posterior invalida la revisión afectada y obliga a comprobar de nuevo su alcance.
6. Mantener separadas la revisión del contenido y la autoridad para habilitar su suministro. La función de revisor no recibe automáticamente facultades de Proveedor de documentos para la IA. El Árbitro-Director conserva la aplicación de los permisos y del estado documental recibido.

Por ejemplo, «administrar 5 mg» y «administrar 10 mg» requieren comparar población, vía, frecuencia, fecha y condición; podrían ser compatibles. Una alerta numérica aislada no basta. «Se permite» y «se prohíbe» también pueden referirse a condiciones distintas. Esas condiciones deben permanecer visibles y ser revisadas por la persona.

Los procesos largos deberán ser acotados, mostrar progreso y conservar un punto de continuación, sin bloquear la interfaz. La extracción incompleta, el fallo de un proceso o la ausencia de resultados no equivalen a ausencia de contradicciones.

## Desarrollo propio estrictamente necesario y comprobación posterior

Lo específico que no se ha encontrado resuelto íntegramente bajo las condiciones exigidas es la pantalla unificada de revisión, el expediente de cada alerta, la vinculación con permisos y versiones y las reglas documentales aprobadas. Sólo esa integración justificaría código propio. Se evitaría desarrollar otro extractor PDF, otro motor de búsqueda o un algoritmo de comparación textual ya disponible.

Antes de implementar, procede cerrar la comprobación de la comparación en Xberg 1.3.6, identificar las estructuras que realmente conserva cada lector y definir con autoridad humana las reglas y los estados documentales. La recepción posterior debería usar un corpus fijado con conflictos y diferencias compatibles conocidos, incluidos cambios de unidades, excepciones, negaciones, notas al pie, tablas, versiones históricas y condiciones repartidas entre tres documentos. Debe medir omisiones, falsas alertas, trazabilidad y coste de revisión humana.

La ausencia de alertas sólo acreditará el resultado de las reglas y del corpus efectivamente comprobados. **No se prometerá ausencia universal de contradicciones en lenguaje natural.** Ese límite no impide construir una aplicación útil para localizar evidencia y ordenar la revisión humana.

El estudio queda preparado localmente. No constituye una publicación remota, una incorporación técnica ni una autorización de uso documental. Se preservan los componentes y trabajos concurrentes.

**Nota de edición pública:** se conservan las conclusiones y correcciones del estudio; se añaden referencias de lectura y se omiten las rutas de trabajo. Las afirmaciones de preparación sin publicación describen el momento original. La identidad de esta edición y su correspondencia con el original constan en [MANIFIESTO.json](MANIFIESTO.json).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
