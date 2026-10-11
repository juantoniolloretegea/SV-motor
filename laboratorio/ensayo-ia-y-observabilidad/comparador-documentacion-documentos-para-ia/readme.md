# Buscador-Semántico - Diferencial: herramientas, resultados y retirada

**11 de octubre de 2026. Ensayo concluido; herramienta retirada por utilidad insuficiente.**

## Objetivo y método

Se investigó una ayuda opcional para advertir contradicciones entre un documento propuesto y otros documentos del mismo dominio. La selección podía comprender un documento, un grupo o todos los documentos admisibles. Inmunología y ciberseguridad se trataron por separado.

El objetivo eran las afirmaciones incompatibles sobre el mismo objeto y bajo las mismas condiciones, no las diferencias de redacción, la menor precisión ni la información complementaria. El detector debía funcionar mediante código Rust, sin recurrir a modelos de IA para decidir si existía una contradicción.

Se emplearon controles sintéticos y casos preparados a partir de pasajes sobre tricoleucemia —documentación NCI-PDQ y material de la Leukemia & Lymphoma Society— y de ciberseguridad, incluidos materiales de INCIBE. Se distinguieron los textos de referencia de las alteraciones introducidas para construir los casos. No se evaluaron íntegramente los documentos originales ni se atribuyeron a sus autores las afirmaciones incompatibles creadas para el experimento.

Los casos documentales comprendían seis incompatibilidades, cuatro compatibilidades y dos relaciones con contexto insuficiente. Se utilizaron los originales españoles y traducciones inglesas obtenidas mediante Google Translate. Las traducciones se conservaron sin reformularlas para favorecer al detector; su correspondencia textual no equivalía a una comprobación de equivalencia semántica. Las versiones española e inglesa representan los mismos doce casos, no veinticuatro casos independientes.

## Herramientas de Rust utilizadas

| Herramienta | Función en el experimento | Limitación observada |
|---|---|---|
| **speccheck-core 0.4.1** | Ejecución de reglas deterministas sobre requisitos y afirmaciones. Para comparar documentos se utilizó `CrossFileContradiction`, regla `SC-XF-002`, basada en expresiones inglesas de obligación, negación y coincidencia de palabras. | Cobertura reducida de prosa ordinaria, sujetos, condiciones, unidades, vigencia y relaciones entre pasajes. |
| **Adaptación experimental de speccheck-core 0.4.1** | Ampliación del reconocimiento de modalidades con `should` y comparación de una variante más restrictiva que exigía coincidencia del resto de la afirmación. | Ampliar los patrones aumentó también los falsos avisos; restringirlos redujo esos avisos, pero mantuvo numerosas omisiones. |
| **logicaffeine-language 0.10.1** | Análisis de enunciados ingleses y obtención de una representación lógica, sin servicios de IA ni Z3. Se examinó si conservaba la información necesaria antes de intentar comparar contradicciones. | Rechazos y pérdida de condiciones esenciales durante la representación de las oraciones. No llegó a acreditar la comparación documental pretendida. |
| **unicode-segmentation 1.13.3** | Separación de palabras y oraciones conforme a Unicode 17.0.0; comparación del lector original, los párrafos completos y las oraciones separadas, manteniendo la misma regla de detección. | Entregar más unidades de texto no mejoró la detección documental. Separar oraciones podía desvincular una afirmación de su contexto. |
| **pulldown-cmark 0.13.0 y mdBook 0.5.4** | Estructuración de Markdown y presentación de documentos, pasajes y resultados. | Organizar y mostrar el texto no aporta interpretación de contradicciones. |
| **Axum y redb** | Comprobaciones experimentales de consulta, selección, separación por dominio y conservación de registros mediante componentes documentales existentes. | Las comprobaciones de funcionamiento no acreditaban la utilidad semántica del comparador ni su despliegue operativo. |

La dependencia `speccheck-nli 0.4.1` estuvo presente como interfaz compilada, pero no se instanció ni ejecutó un detector de IA. **Tantivy se estudió como buscador textual; no se utilizó como detector de contradicciones ni produjo los resultados siguientes.**

## Resultados

En una exploración inicial de speccheck-core sobre 24 casos sintéticos se señalaron seis de doce incompatibilidades, pero también cinco de ocho compatibilidades y tres de cuatro casos de contexto insuficiente. Esta configuración no distinguía de forma suficientemente útil los conflictos de las diferencias compatibles.

Logicaffeine admitió los dos controles lingüísticos y ocho de doce enunciados de dominio: cinco de seis de medicina y tres de seis de ciberseguridad. Rechazó cuatro enunciados y perdió información esencial en al menos cuatro de los ocho admitidos. Se identificaron pérdidas de condiciones relacionadas con infección, complementos de obligaciones y alcance temporal. Estos resultados evalúan la representación del texto, no una tasa de detección de contradicciones.

La comparación posterior de las variantes de speccheck produjo los siguientes resultados. Cada celda cuenta **casos con aviso**, no avisos individuales:

| Procedimiento | Controles sintéticos: incompatibilidades señaladas, de 6 | Controles sintéticos: falsos avisos, de 8 compatibles | Casos documentales ingleses: incompatibilidades señaladas, de 6 |
|---|---:|---:|---:|
| Regla original | 1 | 0 | 0 |
| Ampliación con `should` | 5 | 6 | 2 |
| Ampliación con `should` y coincidencia restrictiva | 4 | 1 | 1 |

Los controles de esta tabla forman un conjunto posterior de dieciséis casos, distinto de los 24 iniciales. Incluyen dos casos de contexto insuficiente, con cero, dos y un aviso, respectivamente. En los doce casos documentales, ninguna variante avisó sobre las cuatro compatibilidades o las dos relaciones indeterminadas. La ejecución documental de la regla original precedió a las adaptaciones; sus resultados no representan una repetición independiente.

Además, uno de los dos avisos documentales de la variante amplia relacionó «debería priorizarse» con «no debería implantarse»: coincidía una forma verbal auxiliar, pero no la acción. Por tanto, señalar un caso clasificado como incompatible no bastaba para demostrar que el fundamento del aviso fuera correcto. La variante restrictiva conservó un aviso relativo a verificar el acceso frente a su negación y omitió las otras cinco incompatibilidades.

La prueba de segmentación mantuvo los mismos resultados en los doce casos documentales: **cero de seis incompatibilidades detectadas en español y una de seis en inglés**, tanto con el lector original como con párrafos completos y con oraciones separadas. No aparecieron avisos en los casos compatibles o indeterminados. Admitir más texto no resolvió la limitación del comparador.

En doce controles artificiales adicionales, con cinco incompatibilidades y siete compatibilidades, la separación por oraciones pasó de dos a tres detecciones y de uno a tres falsos avisos. Los dos errores nuevos separaron una afirmación de una condición temporal o de la aclaración de que un ejemplo estaba obsoleto. La segmentación conservaba los textos, pero la regla no interpretaba correctamente el contexto que determinaba su significado.

## Conclusión

**La combinación ensayada no ofreció la utilidad necesaria para ayudar a localizar contradicciones documentales y se retiró.** Sus limitaciones decisivas fueron las omisiones, los falsos avisos y la falta de correspondencia segura entre un aviso y una incompatibilidad de significado. Las mejoras de presentación, selección o segmentación no corrigieron esa capacidad central.

Las muestras fueron pequeñas, deliberadas y conocidas durante el desarrollo; las clasificaciones documentales eran preliminares. Los resultados no estiman una fiabilidad general ni demuestran que cualquier solución futura sin IA sea imposible. La ausencia de aviso tampoco acredita compatibilidad documental.

Este documento reúne el objetivo, las herramientas, el método, los resultados y la conclusión del ensayo. La herramienta no está disponible para uso ni incorporada al libro de la Biblioteca del SV.
