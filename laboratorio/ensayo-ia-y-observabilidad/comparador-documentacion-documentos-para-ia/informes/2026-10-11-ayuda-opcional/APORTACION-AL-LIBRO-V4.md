# Buscador-Semántico - Diferencial: ayuda para examinar la coherencia documental

**Aportación editorial, edición 4 · 11 de octubre de 2026.**

## Texto para «Información disponible para la revisión»

El **Buscador-Semántico - Diferencial** tiene como objetivo ayudar a comprobar si un documento propuesto para la IA contiene afirmaciones en conflicto con documentos ya incorporados al conjunto documental correspondiente. Su ámbito comprende Medicina y Ciberseguridad. La comparación atiende a sujetos, condiciones de aplicación, excepciones, magnitudes, unidades y vigencia. Dos redacciones distintas pueden ser compatibles; dos formulaciones semejantes pueden afirmar cosas incompatibles.

Se concibe como una ayuda de consulta voluntaria. El revisor podrá abrirla cuando le resulte útil, examinar las sugerencias que considere pertinentes, descartarlas o continuar sin utilizarla. Su uso no constituye un requisito de publicación ni de suministro documental. Las sugerencias no modifican textos ni aplican decisiones; las condiciones de autorización documental conservan sus procedimientos.

Al solicitar la comparación, el responsable elegirá un documento determinado, un grupo de documentos o todos los documentos ya incorporados a los que tenga acceso dentro del mismo dominio. «Todos» se limita a ese dominio. Un documento de inmunología se compara con documentos de inmunología; los de ciberseguridad pertenecen a un conjunto separado. La selección permite acotar el trabajo y no se amplía automáticamente.

Rust aplicará los filtros, comprobará dominio y permisos, identificará las versiones y entregará el conjunto elegido al Buscador-Semántico - Diferencial. Las referencias de otro dominio se rechazarán antes de recuperar su contenido para la comparación. Una selección vacía no se sustituirá por el conjunto completo. Se reutilizan las representaciones y la administración documental existentes.

El funcionamiento previsto consiste en tomar el documento propuesto, localizar pasajes relacionados en ese conjunto y presentar posibles incompatibilidades con su procedencia, contexto y fundamento. Cada sugerencia deberá permitir examinar ambos pasajes y sus versiones. La utilidad depende de la pertinencia de esas indicaciones y del esfuerzo necesario para examinarlas. No se exige una detección perfecta. Los errores y la cobertura deberán hacerse explícitos para que una función parcial pueda utilizarse con conocimiento de sus límites.

La coherencia documental y la fidelidad de sus representaciones son propiedades diferentes. Una conversión fiel puede conservar afirmaciones contradictorias. La pérdida de una negación, una condición o una unidad durante la extracción o la traducción puede producir un conflicto aparente. El examen debe permitir reconocer los originales, las representaciones comparadas y las partes efectivamente disponibles.

Los originales pueden estar en PDF, HTML o Markdown. mdBook proporciona una presentación común, sin determinar por sí solo el formato suministrado a un modelo. Cuando se utilice una traducción instrumental al inglés, se conservará su correspondencia con los originales. La traducción de lectura y la representación utilizada en el análisis deberán identificarse por separado. El **Árbitro - Director** conserva sus funciones documentadas de entrega.

La ayuda permanece en estudio y no está integrada. La ausencia de sugerencias no demuestra ausencia de contradicciones. Su consulta no constituye el examen del modelo que posteriormente reciba los documentos.

## Texto para «Comprobaciones, estado y fuentes»

La primera comprobación determinista de speccheck-core 0.4.1 utilizó 24 casos sintéticos de Medicina y Ciberseguridad. Emitió alertas en 6 de 12 incompatibilidades y en 5 de 8 casos compatibles; otros 4 casos tenían contexto insuficiente y se contabilizaron por separado. No superó los criterios de aquella comprobación. Ese resultado se conserva: no acredita una detección general ni demuestra que todas las funciones carezcan de utilidad como ayuda opcional.

Las salidas permiten estudiar sugerencias sobre oposición entre obligación y prohibición, orden temporal, restricciones numéricas y relaciones entre archivos. También muestran confusiones entre condiciones, actores, vigencias y unidades equivalentes. De los dos casos distribuidos entre archivos, se señaló uno y se omitió otro. Los demás resultados no acreditan por sí mismos la comparación del documento propuesto con el conjunto ya incorporado. La utilidad auxiliar deberá valorarse mediante la pertinencia de los avisos, sus errores, las omisiones conocidas y el esfuerzo de consulta. No se atribuye a este conjunto sintético una precisión general sobre la bibliografía ni un ahorro de tiempo todavía no medido.

Una comprobación distinta de Logicaffeine 0.10.1 admitió ocho de doce enunciados especializados, pero al menos cuatro de los admitidos perdieron condiciones o complementos esenciales. Este resultado no justifica utilizar esa representación para la comparación prevista. Las comprobaciones de admisión y de contradicciones se mantienen diferenciadas.

La [documentación del Buscador-Semántico - Diferencial](https://github.com/juantoniolloretegea/SV-motor/tree/main/laboratorio/ensayo-ia-y-observabilidad/comparador-documentacion-documentos-para-ia) conserva métodos y resultados. El [primer contraste](https://github.com/juantoniolloretegea/SV-motor/blob/31a5aa341f20bdd1c7126c498347960da7d162c4/laboratorio/ensayo-ia-y-observabilidad/comparador-documentacion-documentos-para-ia/comprobaciones/2026-10-11-speccheck-0.4.1/RESULTADO.md) y la [comprobación de admisión](https://github.com/juantoniolloretegea/SV-motor/blob/29b1b0be3dbb99e3c4f7c14da752d896d3bdefb2/laboratorio/ensayo-ia-y-observabilidad/comparador-documentacion-documentos-para-ia/informes/2026-10-11-paso4/RESULTADO-ADMISION.md) identifican las evidencias y los límites de cada actuación. La actualización de la finalidad auxiliar no modifica esos resultados históricos.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
