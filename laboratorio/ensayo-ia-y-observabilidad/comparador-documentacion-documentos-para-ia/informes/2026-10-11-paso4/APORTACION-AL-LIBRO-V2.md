# Buscador-Semántico - Diferencial: coherencia del conjunto documental

**Aportación editorial, edición 2 · 11 de octubre de 2026.**

## Texto para «Información disponible para la revisión»

El **Buscador-Semántico - Diferencial** tiene como objetivo localizar posibles contradicciones de significado dentro de un documento y entre documentos destinados a una IA. Su ámbito comprende Medicina y Ciberseguridad. La comparación debe atender a los sujetos, las condiciones de aplicación, las excepciones, las magnitudes, las unidades y la vigencia de cada afirmación. Dos redacciones distintas pueden ser compatibles; dos formulaciones semejantes pueden afirmar cosas incompatibles.

La coherencia del conjunto documental y la fidelidad de sus representaciones son propiedades diferentes. Una conversión fiel puede conservar afirmaciones contradictorias. A la inversa, perder una negación, una condición o una unidad durante la extracción o la traducción puede producir un conflicto aparente. Por ello, el examen debe identificar los originales, las representaciones comparadas y su cobertura.

El funcionamiento previsto comprende identificar las afirmaciones relacionadas, localizar posibles incompatibilidades y presentar los pasajes completos con su procedencia, contexto y fundamento. La revisión permite confirmar el conflicto, explicar una diferencia compatible o señalar qué información falta para resolverla. La decisión y sus consecuencias sobre el suministro se registran en el expediente. El **Árbitro - Director** mantiene sus funciones documentadas en la aplicación de las condiciones de entrega.

Los originales pueden estar en PDF, HTML o Markdown. mdBook proporciona una presentación común, sin determinar por sí solo el formato que se suministra a un modelo. Cuando se utilice una traducción instrumental al inglés para comparar textos de diferentes idiomas, se conservará la correspondencia con los pasajes originales. La traducción de lectura de la biblioteca y la representación utilizada en el análisis deberán identificarse por separado.

El Buscador-Semántico - Diferencial permanece en estudio. Todavía no se dispone de una realización comprobada para el alcance descrito. La ausencia de alertas no demuestra que el conjunto documental esté libre de contradicciones. El examen de coherencia tampoco equivale a evaluar el modelo que después reciba los documentos.

## Texto para «Comprobaciones, estado y fuentes»

La primera comprobación de un candidato determinista, speccheck-core 0.4.1, utilizó 24 casos sintéticos de Medicina y Ciberseguridad. El candidato emitió alertas en 6 de 12 incompatibilidades y en 5 de 8 casos compatibles; los otros 4 casos tenían contexto insuficiente y se contabilizaron por separado. La configuración examinada no superó los criterios establecidos. Este resultado no corresponde al análisis integral de las fuentes documentales ni permite estimar el rendimiento sobre toda la bibliografía.

Una comprobación posterior de Logicaffeine 0.10.1 examinó la admisión de doce enunciados especializados, acompañados de dos controles elementales. La función analizada admitió ocho de los doce enunciados y rechazó cuatro. Al menos cuatro de los admitidos perdieron condiciones o complementos esenciales. Este resultado impide considerar suficiente la representación obtenida para comparar contradicciones. La comprobación de admisión y la de contradicciones son distintas; sus resultados no se suman.

Las comprobaciones y sus resultados se conservan en la [sede del Buscador-Semántico - Diferencial](https://github.com/juantoniolloretegea/SV-motor/tree/main/laboratorio/ensayo-ia-y-observabilidad/comparador-documentacion-documentos-para-ia). El [informe de la primera comprobación](https://github.com/juantoniolloretegea/SV-motor/blob/31a5aa341f20bdd1c7126c498347960da7d162c4/laboratorio/ensayo-ia-y-observabilidad/comparador-documentacion-documentos-para-ia/comprobaciones/2026-10-11-speccheck-0.4.1/RESULTADO.md) y el [resultado de admisión de Logicaffeine](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/comparador-documentacion-documentos-para-ia/informes/2026-10-11-paso4/RESULTADO-ADMISION.md) contienen el método, las evidencias y las limitaciones.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
