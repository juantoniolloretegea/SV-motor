# Buscador-Semántico - Diferencial

**Edición documental 3 · 11 de octubre de 2026.**

Esta sede reúne los estudios, revisiones y evidencias del **Buscador-Semántico - Diferencial** del Sistema Vectorial SV. Su finalidad es permitir que los revisores conozcan cómo se examinan los documentos destinados a una IA, qué contradicciones se han localizado, qué cobertura tiene el examen y qué decisiones han autorizado su suministro.

El ámbito comprende **Medicina y Ciberseguridad**, con documentos PDF, HTML y Markdown. Se buscan **incompatibilidades de significado**, tanto dentro de un documento como entre documentos, atendiendo a sujetos, condiciones, versiones, cantidades, unidades, fechas y excepciones. Una diferencia de redacción no basta para declarar contradicción.

## Estado comprobado

**Todavía no se ha acreditado una aplicación que detecte las contradicciones complejas requeridas.** Se han examinado componentes, fuentes y límites. La primera comprobación Rust de speccheck-core 0.4.1 no supera el criterio previo: alertas en 6 de 12 incompatibilidades y en 5 de 8 casos compatibles; cuatro casos de contexto insuficiente se contabilizan aparte. El conjunto es sintético y diagnóstico. La comprobación posterior de admisión de Logicaffeine 0.10.1 tampoco permite avanzar a la integración: admite ocho de doce enunciados especializados, pero al menos cuatro de los admitidos omiten condiciones o complementos esenciales. El Buscador-Semántico - Diferencial permanece en estudio.

| Función | Situación al corte |
| --- | --- |
| Administración y presentación | Axum, redb y mdBook ya utilizados. mdBook es el lector común; su presentación no determina por sí sola el permiso ni el formato de suministro a modelos. |
| Preparación documental | Xberg y los lectores Rust existentes conservan sus usos y límites. html-to-markdown-rs y pulldown-cmark también están incorporados. La extracción PDF no acredita recuperación universal de tablas, figuras o lectura íntegra. |
| Búsqueda de pasajes | Tantivy es candidato. Su código es Rust; queda por comprobar el perfil de dependencias que excluya la compresión Zstandard implementada en C. |
| Idioma de comparación | Traducción Google al inglés aceptada como instrumento externo, conservando original y traducción. La recepción de un texto inglés estable para el análisis sigue pendiente de adaptación y comprobación. |
| Contradicciones de significado | Capacidad pendiente de acreditación. La búsqueda textual, las reglas parciales y la comparación de ediciones no la sustituyen. |
| Decisión sobre documentos | La decisión y sus efectos sobre el suministro se conservan en el expediente; las alertas no constituyen una autorización. |

La propuesta inicial basada en vocabularios preparados por personas está retirada: trasladaba al revisor parte esencial del hallazgo solicitado. No haber encontrado una solución completa tampoco demuestra imposibilidad técnica. El desarrollo propio sólo se considerará cuando se justifique su necesidad frente a los recursos existentes.

## Informes y fuentes

| Documento | Función y estado |
| --- | --- |
| [Revisión de alternativas y avance por pasos](informes/2026-10-11-paso4/REVISION-ALTERNATIVAS.md) | Revisión vigente del paso 4; recursos examinados, procedencia y límites. |
| [Admisión y conservación del significado: Logicaffeine 0.10.1](informes/2026-10-11-paso4/RESULTADO-ADMISION.md) | Comprobación nueva, separada del ensayo anterior. Resultado insuficiente para integración. |
| [Comprobación preliminar de speccheck-core 0.4.1](comprobaciones/2026-10-11-speccheck-0.4.1/RESULTADO.md) | Resultado vigente: candidato no apto para incorporación, con protocolo, casos, salidas, contraste Rust y límites. No demuestra inviabilidad general. |
| [Estructura lógica y componentes Rust](informes/2026-10-11/ESTRUCTURA-LOGICA-RUST-MDBOOK.md) | Estudio vigente: funciones conjuntas, comunidades, mantenimiento, mdBook, traducción y revisión de lenguajes y dependencias. Distingue lo existente de lo propuesto. |
| [Revisión de aptitud y contraste adversarial](informes/2026-10-11/REVISION-DE-APTITUD.md) | Conserva el rechazo de la propuesta inicial, la corrección de una desestimación excesiva y la reconsideración bilingüe anterior a la precisión sobre traducción. |
| [Estado del arte inicial](informes/2026-10-11/ESTADO-DEL-ARTE.md) | Antecedente del 10/10/2026, con propuesta retirada expresamente identificada. |
| [Fuentes y componentes del 11/10/2026](informes/2026-10-11/COMPONENTES-RUST-MDBOOK-FUENTES.json) | Referencias, versiones, actividad observada, objetos Git y límites de las comprobaciones. |
| [Fuentes del estudio inicial](informes/2026-10-11/FUENTES.json) | Registro histórico del 10/10/2026, con precisión sobre las instrucciones posteriores. |
| [Manifiesto de esta edición](informes/2026-10-11/MANIFIESTO.json) | Identidad, tamaño y SHA-256 de los documentos publicados; correspondencia con los informes originales. |

Las copias públicas conservan las conclusiones, adendas y fuentes. Se han retirado las rutas de trabajo y añadido indicaciones de estado para evitar que una propuesta histórica se lea como vigente. El manifiesto distingue la huella original de la edición pública; no afirma igualdad entre contenidos editados.

## Recorrido previsto para la revisión

1. Identificar los documentos, ediciones, procedencias y permisos; declarar qué contenido se ha recuperado y qué contenido permanece fuera de cobertura.
2. Preparar las representaciones y, cuando corresponda, la traducción inglesa, manteniendo correspondencia con los pasajes originales.
3. Localizar posibles contradicciones y presentar evidencia enfrentada con contexto y condiciones. Esta capacidad principal está pendiente de demostración.
4. Revisar cada hallazgo: confirmar el conflicto, explicar una diferencia compatible o mantener el asunto pendiente.
5. Aplicar la decisión autorizada sobre el corpus y conservar qué versión se entrega efectivamente a la IA mediante el **Árbitro - Director**.

El recorrido reutiliza la administración y la trazabilidad existentes. El revisor no debe preparar previamente los conceptos o conflictos que la herramienta tiene que descubrir. El comparador propuesto no incorpora una IA como analizador o juez; Google se circunscribe a la traducción aceptada.

## Relación con la evaluación del modelo y los pilares del SV

La revisión del corpus prepara y documenta las condiciones del examen; **no constituye el examen ni su dictamen**. Los resultados del modelo se expresan, en el alcance constituido para cada prueba, como **Apto (0), No apto (1) o Indeterminado (U)**, conforme al contrato, los parámetros y criticidades establecidos por la sede competente y las reglas canónicas aplicables.

La aplicación de los [pilares y restricciones de diseño del Lenguaje SV](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/f10aac725676997adc3840a1bd1a41e1579589ea/docs/calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md) exige preservar identidad, orden, procedencia y autoridad. Esta documentación no introduce una nueva semántica, no elige células ni parámetros, y no modifica el Núcleo ni su representación intermedia. En particular:

- **U conserva su significado de indeterminación honesta.** Una extracción fallida, un documento ausente o una prueba no ejecutada se registran como incidencias o pendientes; no se convierten automáticamente en U.
- **La ausencia de alertas no demuestra ausencia de contradicciones.** Deben declararse cobertura, omisiones y límites.
- **Ninguna corrección será silenciosa.** Una nueva edición, traducción o resolución conserva el antecedente y obliga a identificar qué conclusiones quedan afectadas.
- **Las funciones se mantienen diferenciadas.** La comparación aporta evidencias; la constitución del dominio, la evaluación del modelo y la autorización documental conservan sus procedimientos.

El [ensayo de inteligencia artificial y observabilidad](https://github.com/juantoniolloretegea/SV-motor/blob/94a3cda79dc3cadbf820c23e62946276fc32f45d/laboratorio/ensayo-ia-y-observabilidad/README.md#evaluación-documental-y-resultado) distingue adjudicaciones por posición, puntuación, criticidad, dictamen y recepción independiente. Cada examen mantiene su expediente en el nodo correspondiente; esta carpeta conserva los estudios del comparador y sus comprobaciones propias, sin duplicar ni sustituir aquellos resultados.

## Continuidad de los informes

Los informes posteriores se incorporarán con fecha, objeto, fuentes, método, resultados, límites y estado de revisión. Una corrección se explicará y conservará el antecedente. Esta entrada señalará siempre cuál es la conclusión vigente; los registros científicos y de Calidad conservarán sus sedes competentes.

La comprobación algorítmica posterior no consultó modelos de IA ni modificó la instalación del SV. Las revisiones históricas se conservan.

## Relación con el libro de la Biblioteca del SV

El libro publicado [«Reglas, usos y descripciones documentales de documentos para el conocimiento de la IA»](https://documentos-sv.itvia.online/documentacion/reglas-usos-descripciones-documentales/index.html) conserva su estructura y sus cinco capítulos. La [aportación editorial vigente, edición 2](informes/2026-10-11-paso4/APORTACION-AL-LIBRO-V2.md) describe objetivos, alcance, funcionamiento y límites, para los capítulos de información de revisión y comprobaciones. Sustituye editorialmente la propuesta anterior, que se conserva como antecedente. La aportación está preparada para incorporación coordinada; no se declara ya incorporada ni constituye otro libro. Las evidencias técnicas permanecen en esta carpeta.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
