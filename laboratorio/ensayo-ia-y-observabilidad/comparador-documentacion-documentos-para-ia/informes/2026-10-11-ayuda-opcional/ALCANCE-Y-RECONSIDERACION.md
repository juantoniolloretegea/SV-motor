# Buscador-Semántico - Diferencial: ayuda opcional para la revisión documental

**11 de octubre de 2026 · Precisión del alcance y reconsideración de evidencias conservadas.**

## Objetivo vigente

El **Buscador-Semántico - Diferencial** se estudia como una ayuda que el revisor puede utilizar a voluntad para comprobar si un documento propuesto para la IA contiene afirmaciones en conflicto con documentos ya incorporados al conjunto documental correspondiente. Su ámbito comprende Medicina y Ciberseguridad, con originales PDF, HTML y Markdown. Las sugerencias deberán facilitar el examen de los pasajes relacionados y de las condiciones en que podrían resultar incompatibles.

El recorrido es concreto: **documento propuesto y dominio → elección del conjunto de comparación → filtrado en Rust → Buscador-Semántico - Diferencial → posibles conflictos con pasajes y referencias**. Se reutilizan la identificación, las versiones, las representaciones y los permisos de la administración existente. El documento propuesto se mantiene diferenciado del conjunto de referencia hasta que se adopte la decisión documental correspondiente. Cada sugerencia debe relacionar un pasaje del documento propuesto con un pasaje de un documento ya incorporado. Esta actuación no añade una auditoría general de todas las relaciones internas del conjunto.

Al solicitar la ayuda, el responsable elegirá una de tres opciones: **un documento determinado, un grupo de documentos o todos los documentos ya incorporados de su dominio a los que tenga acceso**. No se seleccionará el conjunto completo automáticamente. El objetivo es ajustar el trabajo a la comparación necesaria.

Rust resolverá esa selección mediante la administración existente, comprobará la pertenencia al mismo dominio y los permisos de acceso, identificará las versiones y entregará al Buscador-Semántico - Diferencial el documento propuesto y el conjunto delimitado. La selección del grupo no ampliará el dominio. «Todos» significa todos los documentos accesibles ya incorporados en ese dominio, no todos los documentos de la aplicación.

La separación por dominio se comprobará en Rust antes de recuperar los contenidos para compararlos; no dependerá exclusivamente de las opciones visibles en la interfaz. Una referencia de otro dominio se rechazará, aunque se envíe directamente. Un documento de inmunología se compara con documentos de inmunología, no con los de ciberseguridad. Una selección vacía no provocará una búsqueda más amplia: se informará de que no hay documentos seleccionados disponibles para la operación. La presentación identificará el dominio y el conjunto efectivamente utilizado.

La utilidad no exige detectar todas las contradicciones ni eliminar todo error. Depende de que las sugerencias aporten información pertinente y comprobable, con un esfuerzo de consulta razonable. Una función limitada puede resultar útil si su alcance está identificado. Una diferencia textual o una coincidencia de palabras no se presentará por sí sola como contradicción de significado.

La consulta es voluntaria: no es necesario abrir la ayuda, ejecutar el análisis, examinar todas sus sugerencias ni pronunciarse sobre ellas para continuar el procedimiento documental. Una sugerencia no altera el texto, no determina permisos y no impide por sí misma publicar o suministrar un documento. Las condiciones de autorización ya existentes conservan su alcance; no se añade una condición consistente en resolver los avisos del comparador.

Esta precisión cambia el criterio de utilidad de las actuaciones futuras. Los informes anteriores se conservan con los criterios, entradas y resultados de sus respectivas comprobaciones. El rechazo de una configuración para el objetivo previo no demuestra que todas sus funciones carezcan de utilidad auxiliar. Tampoco la nueva finalidad convierte retrospectivamente un error en un acierto o una configuración examinada en una función ya disponible.

## Qué permiten concluir las evidencias existentes

Se reconsideran los 24 casos sintéticos de [speccheck-core 0.4.1](../../comprobaciones/2026-10-11-speccheck-0.4.1/RESULTADO.md), con sus [referencias previas](../../comprobaciones/2026-10-11-speccheck-0.4.1/REFERENCIAS.json), [salida conservada](../../comprobaciones/2026-10-11-speccheck-0.4.1/EJECUCION-01.json) y [cotejo Rust](../../comprobaciones/2026-10-11-speccheck-0.4.1/COTEJO-RUST.json). No se ejecutó de nuevo el candidato ni se añadieron casos. El cotejo anterior cuenta alertas por caso; no acredita automáticamente la corrección de todos los fundamentos de cada alerta.

Las salidas contienen 25 diagnósticos correspondientes a 14 casos: seis casos de incompatibilidad, cinco compatibles y tres de contexto insuficiente. Otras seis incompatibilidades no produjeron alertas. Los cuatro casos con contexto insuficiente se mantienen separados; no se les asigna el valor U del SV.

| Función examinada | Casos con aviso en las salidas conservadas | Lectura para el uso auxiliar |
| --- | --- | --- |
| Obligación y prohibición; SC-CON-001 y SC-CON-004 | Incompatibles: M01 y C01. Compatibles: M03, M08 y C06. Contexto insuficiente: M07 y C07. Cada regla emite un aviso en cada uno de esos siete casos. | Existen indicios acertados, pero también confusiones entre condiciones, vigencias y actores. Ambas reglas avisan sobre los mismos casos; no son catorce conflictos distintos ni dos confirmaciones independientes. |
| Restricciones numéricas; SC-CON-002 y SC-CON-006 | Incompatible: C09. Compatibles: M04 y C04. Contexto insuficiente: M11. Cada regla emite un aviso en cada uno de esos cuatro casos. | El acierto sobre valores distintos en la misma unidad convive con errores sobre unidades equivalentes. Una comprobación lógica sobre números incompletamente representados no corrige esa pérdida. |
| Orden temporal; SC-CON-003 | Incompatibles: M09 y C05. | Dos avisos pertinentes justifican seguir examinando esta función. La muestra no acredita su precisión sobre otros textos ni contiene suficientes controles temporales para establecerla. |
| Comparación entre archivos; SC-XF-002 | Incompatible: M10. | Localiza una oposición distribuida entre archivos. No acredita comparación general: C11, también entre archivos, quedó sin aviso. |

Las omisiones corresponden a M02, M05, M06, C03, C10 y C11. La tabla incluye todas las reglas ejecutadas, incluidos sus errores; no selecciona únicamente los resultados favorables. Su agrupación es una interpretación posterior de evidencias conocidas, no una validación independiente de un perfil nuevo. No se transforma en un porcentaje de confianza aplicable a los documentos reales.

Para el recorrido concreto de incorporación, la evidencia directamente pertinente es más limitada que esos totales: **M10 y C11 son los dos casos distribuidos entre archivos; hubo aviso en M10 y no lo hubo en C11**. Los resultados dentro de un único archivo son antecedentes diagnósticos, no una comprobación de comparación entre el documento propuesto y el conjunto ya incorporado. Tampoco se han comprobado todavía documentos extensos en ese recorrido. Se prioriza examinar la función de comparación entre archivos que ya ofrece el candidato, sin construir otro analizador ni introducir bibliotecas por anticipado.

Ejemplos del tipo de ayuda que merece comprobarse:

- **M01, oposición pertinente:** se exige informar de la presencia del mismo marcador y se prohíbe hacerlo para el mismo paciente y visita. Mostrar conjuntamente las dos afirmaciones permite examinar la incompatibilidad.
- **M03, aviso improcedente:** la obligación corresponde a una prueba positiva y la prohibición a una negativa. Mostrar las condiciones completas permite advertir que el aviso ha confundido supuestos diferentes. La etiqueta «sugerencia» no elimina el error ni el tiempo necesario para descartarlo.
- **C05, oposición pertinente:** para la misma sesión y los mismos acontecimientos únicos, la concesión de acceso debe ocurrir antes y después de la verificación de identidad. La inversión del orden ofrece un fundamento examinable.
- **M04 y C04, equivalencias omitidas:** veinticuatro horas y un día; sesenta segundos y un minuto. Presentar estas equivalencias como contradicciones perjudica la utilidad y exige valorar el tratamiento de unidades antes de ofrecer la función numérica de manera ordinaria.

Estos ejemplos son sintéticos y no constituyen recomendaciones médicas, normas de ciberseguridad ni resultados sobre los originales de referencia. La selección de ejemplos explicativos no modifica el conjunto completo conservado.

Logicaffeine conserva un resultado distinto: su representación omite condiciones esenciales en varios enunciados admitidos. Cambiar a un uso opcional no resuelve esa pérdida ni justifica prolongar su desarrollo. No se reabre ese diagnóstico. El objeto inmediato es comprobar la utilidad de las sugerencias ya observadas y sus límites, antes de incorporar otras bibliotecas.

## Presentación prevista de las sugerencias

La ayuda se ofrecerá mediante una consulta que pueda abrirse y cerrarse, sin imponer una secuencia de respuestas. Mostrará el dominio, permitirá elegir un documento, un grupo o todos los documentos accesibles del mismo dominio, y presentará el alcance antes de ejecutar. Cada sugerencia presentará los documentos y ediciones, los pasajes completos pertinentes, sus localizaciones y el motivo concreto de comparación. El contexto deberá permitir distinguir sujetos, condiciones, unidades, excepciones y vigencias.

Los mensajes originales de los candidatos se conservarán como evidencia. Su presentación explicativa distinguirá una posible incompatibilidad de una contradicción establecida: no trasladará como conclusión una orden automática como «resuelva el conflicto». Los valores internos de confianza no calibrados no se mostrarán como probabilidades de acierto. Los avisos reiterados sobre una misma comparación podrán reunirse para facilitar la consulta, conservando sus reglas y procedencia.

El diseño de presentación no concede al instrumento una comprensión que no se haya comprobado. Si faltan localizaciones completas, contexto o partes del documento, esa limitación deberá ser visible; no se reconstruirá silenciosamente el contenido omitido. La ausencia de avisos tampoco se convertirá en una declaración de coherencia documental.

Las decisiones sobre documentos utilizarán las funciones existentes de administración e historial. La mera consulta o el descarte de una sugerencia no crean una nueva obligación de redactar una justificación ni una nueva clase de autorización.

## Continuación del recorrido establecido

Se mantiene la relación funcional: preparación documental existente, representaciones vinculadas al original, traducción instrumental cuando proceda, recuperación de pasajes relacionados de los documentos ya incorporados, comparación con el documento propuesto y presentación común mediante mdBook. El **Árbitro - Director** conserva sus funciones de entrega. Tantivy sigue siendo un candidato para recuperar pasajes, no un detector de contradicciones. No se incorpora una IA como analizador ni se exige preparar vocabularios o formalizar previamente los conflictos que la herramienta debe ayudar a localizar.

La secuencia de ocho pasos se conserva con este criterio de utilidad:

| Paso | Situación y continuación |
| --- | --- |
| 1. Objetivo y alcance | Precisados mediante este documento: ayuda opcional, sin exigencia de perfección ni obligación de uso. |
| 2. Introducción en el libro existente | Preparada la aportación editorial V4 para sus capítulos 4 y 5. No se declara incorporada. |
| 3. Fuentes y casos | Se conservan los cinco originales de Medicina y Ciberseguridad y las referencias previas. Los resultados sintéticos no equivalen a comprobar los documentos completos. |
| 4. Selección técnica | Se reconsideran las funciones de speccheck como ayuda experimental. No se selecciona automáticamente un perfil para integración ni se añaden bibliotecas. |
| 5. Comprobación semántica | Continuación inmediata: comparar un documento propuesto con un conjunto de referencia ya constituido, en Medicina y Ciberseguridad por separado. Se fijarán documentos, versiones, referencias y función examinada antes de ejecutar. Se contrastarán sugerencias entre archivos, con incompatibilidades, casos compatibles parecidos y contexto insuficiente. |
| 6. Idiomas y formatos | Se comprobarán cobertura y correspondencia con los originales. La traducción Google aceptada para lectura no acredita todavía una representación inglesa conservada y utilizable por el comparador. |
| 7. Integración | Se apoyará en la aplicación existente. Rust aplicará la selección de un documento, un grupo o todos los documentos accesibles ya incorporados del mismo dominio y entregará ese conjunto al comparador. Se comprobarán selección vacía, referencias de otro dominio, permisos y ausencia de ampliación automática. No se modifican sus servicios en esta actuación documental. |
| 8. Actualización del libro y documentación | Se explicarán las funciones efectivamente disponibles y sus límites, conservando las ediciones y los resultados anteriores. |

El contraste del paso 5 medirá por separado avisos pertinentes, avisos improcedentes, casos que requieren más contexto, omisiones conocidas y avisos repetidos. Distinguirá el tiempo de ejecución del esfuerzo de consulta; este último no se declarará medido sin observar el uso. Las funciones podrán ajustarse o retirarse si no aportan ayuda suficiente. No se impone un porcentaje perfecto ni se inventa un umbral retrospectivo para aprobar las pruebas anteriores.

## Límites de esta actuación

Esta actualización es una reconsideración documental. No contiene una nueva ejecución del candidato, una integración, una inferencia de modelos ni una medición de ahorro de tiempo. Los conteos proceden de las evidencias ya conservadas y cotejadas. Las comprobaciones de conservación de esta edición se distinguen de las comprobaciones científicas del candidato.

La evaluación de la ayuda documental no asigna por sí misma los valores Apto (0), No apto (1) o Indeterminado (U) al modelo que posteriormente reciba el corpus. Se mantienen la semántica y los procedimientos del SV.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
