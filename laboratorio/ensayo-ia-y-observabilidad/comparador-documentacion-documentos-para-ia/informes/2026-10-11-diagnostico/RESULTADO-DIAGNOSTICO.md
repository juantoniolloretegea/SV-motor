# Buscador-Semántico - Diferencial: localización de pérdidas de significado

**11 de octubre de 2026 · Diagnóstico complementario de Logicaffeine 0.10.1.**

## Conclusión

Las omisiones observadas se producen durante la construcción de la representación de la oración. Ya están presentes antes de la aplicación de axiomas y del tratamiento pragmático. En los casos examinados, esas etapas posteriores mantienen la representación recibida y no recuperan las condiciones perdidas. Por tanto, no se ha identificado un ajuste de configuración que resuelva el problema.

Se han localizado dos mecanismos de pérdida en el código y otra deficiencia en la representación de una obligación. Subsisten, además, rechazos de entrada y un problema de alcance de una variable temporal. Una corrección puntual de uno de estos mecanismos no resolvería el conjunto. **La configuración examinada continúa sin ser apta para la comparación documental prevista.** No se ha modificado el candidato ni se ha iniciado su integración.

Esta conclusión se limita al candidato, versión y configuración examinados. No demuestra que sea imposible desarrollar el Buscador-Semántico - Diferencial ni descarta toda biblioteca de análisis formal.

## Método

Se reutilizaron los catorce enunciados y resultados del [contraste de admisión conservado](https://github.com/juantoniolloretegea/SV-motor/blob/29b1b0be3dbb99e3c4f7c14da752d896d3bdefb2/laboratorio/ensayo-ia-y-observabilidad/comparador-documentacion-documentos-para-ia/informes/2026-10-11-paso4/RESULTADO-ADMISION.md). Se trata de casos conocidos; el diagnóstico no constituye una nueva validación independiente, no modifica la puntuación anterior y no mide detección de contradicciones.

Un instrumento Rust separado reproduce el recorrido público de `compile`: reconocimiento léxico, agrupación de expresiones compuestas, análisis de la oración, aplicación de axiomas, tratamiento pragmático y representación final. Conserva las estructuras completas de las tres últimas etapas, sus fórmulas, posiciones del texto y errores. En M01 y M02 añade un análisis nominal independiente para observar los complementos antes de construir la oración completa.

Las identidades se fijaron antes de ejecutar. El [cotejo Rust](COTEJO-DIAGNOSTICO-RUST.json) acredita 14 resultados idénticos a los antecedentes y 14 reproducciones por etapas concordantes con la función pública. Comprueba identidad instrumental; la interpretación del significado requiere examinar las entradas y estructuras. El diagnóstico completo se conserva en [DIAGNOSTICO.json](DIAGNOSTICO.json).

## Resultados y localización

| Casos | Observación ejecutada | Inspección de la fuente y alcance de la conclusión |
| --- | --- | --- |
| M01 y M02 | El análisis nominal conserva, respectivamente, `With(_PP_SELF_, Active_infection)` y `Without(_PP_SELF_, Active_infection)`. Ambos sujetos presentan determinación `None`. Las condiciones ya no están en la representación de la oración. | El tratamiento de modalidad remite el sintagma nominal completo a una función de determinación; su alternativa `None` devuelve el predicado sin incorporar los complementos. Se localiza así un mecanismo concreto compatible con ambas pérdidas. |
| M05 | Los elementos léxicos incluyen gravedad e insuficiencia renal. La primera estructura de la oración contiene sólo la contraindicación del tratamiento y su relación con pacientes. Las etapas siguientes conservan esa estructura incompleta. | En el tratamiento de complementos de una construcción pasiva, la fuente analiza el sintagma completo, pero incorpora al predicado únicamente su sustantivo principal. No incorpora allí sus modificadores. Es un mecanismo distinto del anterior. |
| C03 | La primera estructura es una negación de `Require(Guests)`; no representa activar autenticación multifactor. La acción y su objeto sí están en el reconocimiento léxico. | El tratamiento de la construcción pasiva contiene un recorrido para complementos introducidos por `to`, pero no conserva aquí el complemento de la obligación. El diagnóstico localiza la pérdida antes de los axiomas; no acredita una corrección completa de este recorrido. |
| M03 | La estructura sitúa el evento y `Within(e, 24 hours)` como términos de una conjunción. La representación cuantifica `e` sólo dentro del evento. | El problema de alcance señalado en el informe anterior no es una alteración introducida por las etapas posteriores. Requiere examinar conjuntamente construcción y representación lógica; no se declara resuelto. |
| M04, C04, C05 y C06 | Se reproducen los cuatro rechazos anteriores durante el análisis, con los mismos errores y posiciones. | Corregir las omisiones de enunciados admitidos no resolvería por sí solo estos rechazos. No se ha modificado el texto para acomodarlo a la gramática. |

Los dos controles y M06, C01 y C02 mantienen sus resultados anteriores. Las diez entradas admitidas conservan la misma fórmula a través de las tres etapas observadas. Esta estabilidad no equivale a corrección semántica: también se conservan las representaciones incompletas.

## Fuentes precisas

El paquete publicado identifica como origen la revisión `d7c86c1bc55dc88abc38f08096192f86f9be82d0` de Brahmastra-Labs/logicaffeine. La fijación conserva SHA-256 de los archivos inspeccionados; los pasajes sobre determinación y complementos se cotejaron además con esa revisión en GitHub.

- [Recorrido público de compilación](https://github.com/Brahmastra-Labs/logicaffeine/blob/d7c86c1bc55dc88abc38f08096192f86f9be82d0/crates/logicaffeine_language/src/compile.rs#L121).
- [Construcción nominal y conservación inicial de complementos](https://github.com/Brahmastra-Labs/logicaffeine/blob/d7c86c1bc55dc88abc38f08096192f86f9be82d0/crates/logicaffeine_language/src/parser/noun.rs#L983).
- [Tratamiento del sujeto ante modalidad](https://github.com/Brahmastra-Labs/logicaffeine/blob/d7c86c1bc55dc88abc38f08096192f86f9be82d0/crates/logicaffeine_language/src/parser/mod.rs#L10368).
- [Determinación del sintagma nominal](https://github.com/Brahmastra-Labs/logicaffeine/blob/d7c86c1bc55dc88abc38f08096192f86f9be82d0/crates/logicaffeine_language/src/parser/quantifier.rs#L2053) y [alternativa sin determinación](https://github.com/Brahmastra-Labs/logicaffeine/blob/d7c86c1bc55dc88abc38f08096192f86f9be82d0/crates/logicaffeine_language/src/parser/quantifier.rs#L2593).
- [Complementos introducidos por `to`](https://github.com/Brahmastra-Labs/logicaffeine/blob/d7c86c1bc55dc88abc38f08096192f86f9be82d0/crates/logicaffeine_language/src/parser/mod.rs#L11114) y [construcción de complementos en pasivas](https://github.com/Brahmastra-Labs/logicaffeine/blob/d7c86c1bc55dc88abc38f08096192f86f9be82d0/crates/logicaffeine_language/src/parser/mod.rs#L11256).

## Consecuencia para el desarrollo

No procede presentar la adición de una condición omitida como solución suficiente. Sería necesario establecer cómo se representan sujetos genéricos, restricciones, negación y obligaciones; conservar complementos; resolver alcances temporales; y comprobar que no se producen regresiones. Añadir palabras a la salida, sin demostrar sus relaciones lógicas, tampoco resolvería la necesidad.

El desarrollo propio de estas capacidades no queda justificado por una sola corrección local. Con las evidencias disponibles, la siguiente decisión técnica exige una propuesta de alcance y viabilidad fundada en capacidades demostrables, o una alternativa comunitaria que conserve el significado. El paso de idiomas, formatos e integración sigue condicionado a esa capacidad principal. La revisión documental puede explicar entretanto el objetivo, el método y las limitaciones acreditadas.

No se han utilizado servicios de inferencia ni modificado el Núcleo, su semántica, su representación intermedia, el Árbitro - Director o los servicios operativos. El código del diagnóstico es un instrumento de comprobación independiente. La licencia BUSL-1.1 del candidato conserva sus condiciones; este estudio no afirma habilitación para un despliegue operativo.

## Reproducción y límites

El [protocolo](PROTOCOLO-DIAGNOSTICO.md), la [fijación](FIJACION.json), el [código Rust](codigo/src/main.rs) y su [resolución de dependencias](codigo/Cargo.lock) permiten examinar el procedimiento. La compilación se realizó con Rust 1.98.1 y dependencias ya disponibles, sin recuperación adicional de paquetes. El perfil mantiene desactivadas las características predeterminadas del candidato.

Las duraciones por caso incluyen la llamada pública y el análisis por etapas; no incluyen la compilación ni el análisis nominal adicional. No deben compararse directamente con las duraciones de la prueba anterior. No son una medición del coste total. El diagnóstico no abarca documentos completos, traducción, archivos PDF ni un contraste clínico o de ciberseguridad aplicado.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
