# Revisión de aptitud de la propuesta documental

**Antecedente conservado; no es la propuesta vigente.** La propuesta inicial fue retirada. La [estructura del 11/10/2026](ESTRUCTURA-LOGICA-RUST-MDBOOK.md) incorpora las precisiones posteriores: Medicina y Ciberseguridad, mdBook como presentación y traducción Google aceptada como instrumento externo para comparar en inglés. Las exclusiones generales de traducción o IA de este antecedente corresponden a su corte anterior; permanece excluida la IA como detector o juez del contraste.


**10 de octubre de 2026. Estado: NO APTO; propuesta retirada.**

## Defecto identificado

La propuesta de búsqueda con Tantivy y vocabularios aprobados por personas no satisface la necesidad formulada. Obliga a preparar conceptos y relaciones para encontrar aquello que la aplicación debe ayudar a descubrir. La extracción multiformato y la comparación textual tampoco resuelven por sí solas esa carencia.

Tantivy puede realizar búsqueda textual sin que se prepare un vocabulario específico; el defecto no es una obligación técnica de esa biblioteca, sino el recorrido propuesto para esta aplicación. No debe confundirse una biblioteca de búsqueda útil con una solución apta para descubrir contradicciones.

La función humana requerida es examinar las evidencias localizadas y decidir cómo resolver el conflicto. El hallazgo previo de los pasajes y sus relaciones no puede depender de que la persona haya reconocido y codificado previamente el problema.

## Condición de aptitud corregida

Una alternativa deberá encontrar automáticamente posibles contradicciones dentro de documentos y entre documentos, localizar los pasajes exactos y conservar su contexto. No exigirá preparar vocabularios, equivalencias o anotaciones particulares del corpus para iniciar ese hallazgo. Mantendrá la resolución y la habilitación documental bajo autoridad humana, sin asistencia de IA y sin modificar el núcleo del SV.

La capacidad de descubrimiento deberá demostrarse en documentos representativos, extensos, en español y de distintos formatos. Diferencias de unidades, fechas, ámbitos, negaciones y excepciones deben aparecer con evidencia suficiente para revisarlas. Una búsqueda exitosa o una lista de cambios textuales no acreditan esa capacidad.

## Comprobación adicional de fuentes

La [documentación de speccheck-core](https://docs.rs/crate/speccheck-core/latest) describe análisis determinista de especificaciones y comparación entre archivos. Su [módulo de contradicciones](https://docs.rs/speccheck-core/latest/speccheck_core/rules/contradiction/index.html) incluye reglas para negaciones directas, restricciones numéricas, orden temporal y tensiones entre requisitos. Es una aproximación más cercana al hallazgo automático que la búsqueda de palabras.

Sin embargo, estas fuentes no acreditan una aplicación recibida para documentos científicos generales en español, largos y multiformato. La descripción de algunas reglas se limita a requisitos de una misma sección y patrones como «shall» y «shall not». Además, el proyecto separa una función de contradicción mediante modelos NLI, que queda excluida de esta necesidad. No se ha ejecutado ni recibido esta biblioteca y no se recomienda como sustituto apto. La documentación general y la del módulo son declaraciones de sus responsables, no comprobaciones propias.

Esta revisión adicional no demuestra que sea imposible construir la aplicación solicitada. Sí confirma que las evidencias reunidas no permiten afirmar que la propuesta anterior satisfaga la necesidad ni recomendar su implementación.

## Estado y retorno

Se conservan las fuentes y la propuesta original como antecedentes identificados. Xberg y los lectores existentes conservan sus usos documentales recibidos; esta retirada no los elimina. No se inicia desarrollo, instalación ni integración del Buscador-Semántico - Diferencial sobre la propuesta rechazada. No se presenta ninguna alternativa nueva como apta sin acreditar primero el descubrimiento que debe realizar.

## 11/10/2026 · Contraste adversarial de la conclusión

La propuesta original continúa retirada. Se corrige, sin embargo, la conclusión posterior que desaconsejaba un desarrollo propio como si su inviabilidad estuviera acreditada. Una búsqueda sin una solución integral encontrada no demuestra inexistencia ni imposibilidad técnica. Tampoco acredita por sí sola que un desarrollo sería demasiado costoso o inútil; no se había realizado una comprobación que permitiera esa valoración.

El contraejemplo más pertinente se examinó en el código publicado de speccheck-core 0.4.1, además de su documentación. Las [reglas de contradicción](https://docs.rs/speccheck-core/latest/src/speccheck_core/rules/contradiction.rs.html) contienen extracción automática de frases y restricciones numéricas mediante patrones incorporados. El [análisis entre archivos](https://docs.rs/speccheck-core/latest/src/speccheck_core/rules/cross_file.rs.html) extrae palabras del texto y compara requisitos de documentos distintos. Ese recorrido no exige que el usuario prepare un vocabulario particular ni ejecuta por esas reglas un modelo NLI. Por tanto, la detección automática limitada sin asistencia de IA tiene una base técnica real.

El propio código acota el contraejemplo: reconoce modalidades normativas inglesas, usa coincidencias léxicas y selecciona palabras con un patrón de letras ASCII. El extractor numérico consultado conserva sujeto, valor y clase de restricción, pero no la unidad capturada, lo que impide atribuirle por esa implementación una comparación dimensional correcta. No se ha ejecutado en esta actuación ni contrastado con bibliografía científica, documentos largos en español, condiciones repartidas o los tres formatos. Esta biblioteca no se declara apta ni se ofrece como sustituto recibido.

Se revisaron también soluciones que anuncian conservación y detección de contradicciones con componentes Rust. La [documentación de semantic-memory](https://docs.rs/crate/semantic-memory/0.5.2) declara representaciones vectoriales obtenidas de un modelo en su recorrido ordinario. La [documentación de donto](https://www.donto.org/docs) confía la extracción de afirmaciones a un modelo generativo. Esas vías no satisfacen la exclusión de asistencia de IA y no refutan el límite del encargo.

La necesidad humana es recibir hallazgos útiles para revisarlos y resolverlos. No se ha exigido expresamente un detector infalible de toda contradicción posible. Sería incorrecto usar esa garantía universal como requisito añadido para descartar cualquier ayuda automática. A la vez, una capacidad parcial de patrones simples no acredita por sí sola utilidad suficiente para los documentos complejos solicitados.

**Conclusión corregida:** no se ha encontrado y verificado una aplicación disponible que cubra la necesidad completa. La posibilidad de un desarrollo útil sin IA permanece abierta; su viabilidad para este corpus no está demostrada ni refutada. Se retira la desestimación general del desarrollo propio y se mantiene retirada la propuesta anterior de búsqueda con preparación conceptual humana. La revisión es documental y de código publicado, no una prueba ejecutada ni autorización de implementación. No se añaden expedientes ni controles al SV.

## 11/10/2026 · Reconsideración con español, inglés y comparación entre idiomas

La necesidad incluye localizar posibles contradicciones en español, en inglés y entre documentos de ambos idiomas, sin traducción, anotación o preparación conceptual particular a cargo del usuario. Se mantiene excluida toda asistencia de IA. El resultado buscado es ayuda efectiva para la revisión humana de documentos complejos; no se añade una exigencia de infalibilidad universal.

**Resultado:** no se ha verificado una aplicación disponible que satisfaga ese conjunto. Hay recursos reutilizables que justifican examinar una vía limitada de detección, pero no declarar su aptitud ni iniciar una aplicación completa como si sólo faltara ensamblar componentes.

| Recurso comprobado | Aportación | Límite para esta necesidad |
| --- | --- | --- |
| [DeCS/MeSH](https://decs.bvsalud.org/en/about-decs/) | Terminología institucional multilingüe, actualizada anualmente, con conceptos y denominaciones inglesas y españolas. Podría evitar que el usuario construya esas equivalencias en Medicina. | No identifica por sí sola afirmaciones contradictorias, ni cubre todos los dominios. Reconocer un término tampoco desambigua automáticamente cada uso. |
| [nlprule](https://github.com/bminixhofer/nlprule) | Tratamiento lingüístico por reglas y recursos léxicos en Rust. | Su documentación declara experimental el español. El último envío al repositorio indicado por la API consultada es del 23/05/2023; no se acredita mantenimiento reciente ni detección de contradicciones. |
| [NegEx-MES](https://github.com/PlanTL-GOB-ES/NegEx-MES) | Reglas publicadas de negación e incertidumbre para textos clínicos, con recursos en español e inglés. | Implementación Java; recibe el término ya identificado junto a la frase. El último envío indicado por la API es del 05/03/2019. No es un componente Rust disponible para integrar directamente. |
| speccheck-core, examinado en la adenda anterior | Detección determinista de ciertos conflictos normativos en Rust. | Patrones ingleses y límites ya registrados; no acredita el recorrido bilingüe ni la interpretación de bibliografía científica. |

La [página oficial para desarrolladores de DeCS](https://decs.bvsalud.org/en/for-developers/) distingue consulta pública de descarga XML y acceso mediante API. Estos últimos requieren solicitar una licencia gratuita. No se ha solicitado ni obtenido esa licencia en esta actuación. Tampoco se incorpora DeCS Finder: la [OPS describe desarrollos de IA para su indización](https://www.paho.org/es/eventos/especializacion-inteligencia-artificial-para-indizacion-automatica-materias), incompatibles con el alcance fijado. La posible reutilización se refiere a los datos terminológicos, no a ese servicio de análisis.

La carencia principal sigue siendo relacionar automáticamente sujeto, afirmación, negación, cantidades y unidades, ámbito, tiempo, condiciones y excepciones, conservando su localización y contexto. Una coincidencia terminológica no demuestra esa relación. Por ejemplo, dos intervalos distintos pueden ser compatibles si corresponden a condiciones diferentes. Ese problema no se resuelve añadiendo únicamente una traducción de las palabras.

Como recomendación de estudio, merece consideración una comprobación reducida del hallazgo bilingüe sobre documentos representativos con conflictos y diferencias compatibles conocidos, sin proporcionar al detector sus localizaciones ni equivalencias particulares. Debe medir conflictos encontrados, conflictos omitidos y falsas alarmas, separando español, inglés y comparación entre idiomas. La preparación independiente de la referencia sirve para evaluar el detector, no para alimentarlo. No se ha ejecutado esa comprobación ni se fija una recepción favorable anticipada.

La eventual integración habría de aprovechar la administración y los lectores existentes, presentar los pasajes y sus contextos en una misma consulta y conservar la decisión humana. No se justifica una plataforma documental paralela. Las limitaciones ya declaradas de extracción y presentación PDF siguen vigentes; no se confunde recepción documental con aptitud para detectar contradicciones. El núcleo, su semántica y su IR quedan fuera de esta actuación.

Se conserva el resultado como reconsideración documental. Continúa retirada la propuesta de vocabularios preparados por usuarios; queda abierta, sin acreditación de viabilidad suficiente, una vía por dominio basada en recursos existentes y desarrollo delimitado allí donde éstos no basten. No se han instalado componentes, ejecutado modelos, publicado ni modificado servicios.

**Nota de edición pública:** se conservan las conclusiones y correcciones del estudio; se añaden referencias de lectura y se omiten las rutas de trabajo. Las afirmaciones de preparación sin publicación describen el momento original. La identidad de esta edición y su correspondencia con el original constan en [MANIFIESTO.json](MANIFIESTO.json).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
