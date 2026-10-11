# Medición experimental de la utilidad del Buscador-Semántico - Diferencial

**Edición 2 · 11 de octubre de 2026 · Criterio fijado antes del envío para ejecución.**

## Objetivo y diseño

Medir si las sugerencias del Buscador-Semántico - Diferencial permiten identificar contradicciones reales entre un documento propuesto y documentos del mismo dominio, sin confundirlas con falta de precisión o detalle. El estudio utiliza doce casos documentales ya conservados, seis de inmunología y seis de ciberseguridad. Cada caso se examina una sola vez. No se incorporan documentos ni se modifican estados administrativos.

La contradicción requiere afirmaciones incompatibles bajo el mismo sentido y condiciones de aplicación. La paráfrasis, una formulación menos precisa, el resumen y la información complementaria pueden ser compatibles. Si el contexto permite conciliarlos, no se penaliza su diferencia de detalle. Si falta información esencial para establecer la relación, se conserva contexto insuficiente. El juicio debe apoyarse en ambos pasajes y explicar por qué pueden o no coexistir.

Se comparan dos modalidades: revisión ordinaria de los documentos y revisión con las sugerencias conservadas del comparador. Cada modalidad incluye seis casos: tres incompatibles, dos compatibles y uno con contexto insuficiente, según las referencias preliminares fijadas. Ambas incluyen tres casos de cada dominio. El orden alterna dominios y modalidades; queda fijado en `materiales/PLAN.json`.

La asignación es deliberada, no aleatoria. Incluye en la modalidad con ayuda el único aviso documental del perfil restrictivo, para poder examinarlo; las otras cinco revisiones asistidas reciben un resultado sin avisos. No se vuelve a ejecutar el detector, se cambian sus reglas o se completan sus omisiones. Los casos son conocidos por el desarrollo del comparador: no constituyen una muestra independiente de su capacidad general.

Un revisor externo con IA realizará esta prueba experimental. Esa participación no se incorpora al funcionamiento del SV. Las modalidades mantienen la misma configuración de revisión y el mismo acceso a los textos; sólo varía la presencia de las sugerencias. Se registra la identificación de modelo que realmente comunique la plataforma, sin deducir versiones no observables. Si el revisor ya conoce casos o soluciones, se declara esa exposición.

## Información y secuencia

1. Fijar programa, protocolo, asignación, materiales y referencias por SHA-256. Conservar las referencias en `materiales/evaluacion`, separadas de la entrega al revisor.
2. Iniciar el registro temporal en Rust inmediatamente antes de abrir cada caso. Leer únicamente su material, disponible como JSON y como contenido para mdBook. Los dos formatos derivan de los mismos originales y traducciones conservadas.
3. Emitir una decisión —incompatibilidad, compatibilidad o contexto insuficiente—, una explicación breve y una cita literal de cada documento que la sustente. En la modalidad asistida, explicar además si el aviso recibido resulta útil, erróneo o insuficiente. No convertir la ausencia de aviso en compatibilidad.
4. Conservar la respuesta sin correcciones posteriores y cerrar el registro temporal. Mantener fallos e interrupciones. Si un problema impide continuar, registrar el caso fallido, sin repetirlo ni reemplazarlo por otro silenciosamente.
5. Tras cerrar las doce respuestas, contrastarlas con las referencias. Separar la coincidencia de clasificación de la corrección de su fundamento. Si se impugna justificadamente una referencia, conservar la discrepancia y publicar ambos resultados; no cambiar la referencia inicial para mejorar la puntuación.
6. Calcular los indicadores en Rust y presentar el resultado por modalidad, dominio y caso. Las mediciones ausentes permanecen desconocidas.

Los textos son materiales de comparación, no instrucciones para ejecutar acciones clínicas, de seguridad o sobre sistemas. Se preservan íntegros los documentos parciales utilizados y se mantiene la procedencia. Los casos alterados no se atribuyen a las entidades autoras de los originales.

## Indicadores

| Indicador | Definición e interpretación |
|---|---|
| Tiempo por revisión | Intervalo exterior entre apertura y cierre, en milisegundos; media por modalidad y dominio. Incluye lectura, respuesta y operaciones dentro del intervalo. |
| Espera instrumental | Duración observada de herramientas, si se dispone de ella; se presenta separada, sin deducirla ni convertir el resto en tiempo interno de razonamiento. |
| Clasificaciones coincidentes | Número de respuestas que coinciden con la referencia preliminar, sobre casos completados y también sobre casos previstos. |
| Resultados fundados | Coincidencia acompañada de citas pertinentes y explicación contrastada. La clasificación por sí sola no basta. |
| Omisiones y falsas incompatibilidades | Incompatibilidades no reconocidas y documentos compatibles declarados incompatibles; las respuestas de contexto insuficiente se mantienen diferenciadas. |
| Rendimiento útil | Resultados fundados divididos por horas observadas. Sólo se calcula con todas las sesiones y tiempos de la modalidad, y sus fundamentos contrastados. |
| Reducción temporal descriptiva | `100 × (media ordinaria − media con ayuda) / media ordinaria`. Un valor negativo indica mayor tiempo con ayuda. |
| Carga de los avisos | Utilidad declarada de cada aviso y tiempo dedicado a examinarlo, únicamente cuando se mida de manera separada. No se estima retrospectivamente. |
| Consumo | Tokens e importe atribuible únicamente si existen mediciones identificables; no se asigna el consumo agregado de una cuenta compartida. |

Las seis incompatibilidades se consideran relevantes para este ensayo: todas modifican elementos sustantivos del pasaje. No se ponderan para ocultar omisiones. El análisis presenta cada una, incluso cuando el tiempo medio mejore.

## Interpretación de la utilidad

La valoración principal corresponde a las contradicciones correctamente identificadas y fundamentadas, las omisiones y las falsas incompatibilidades. Se analizará expresamente si un aviso confunde incompatibilidad con menor precisión o detalle. Un aviso útil debe orientar hacia una oposición real, sustentada por los documentos; no necesita demostrar ahorro temporal. Los resultados se presentan por caso y dominio, sin compensar omisiones relevantes con rapidez.

El tiempo queda como dato descriptivo secundario. Una aceleración no acredita por sí misma utilidad y una lectura más lenta no la refuta. Si hay datos ausentes, indicadores discordantes o avisos infundados, se conserva esa información sin imponer una conclusión favorable. Esta edición sustituye el requisito temporal de mejora formulado en la edición 1; no modifica casos, asignación, referencias, reglas del comparador o fórmulas del instrumento.

Los grupos tienen casos distintos y una dificultad que no está calibrada; la proporción de clases coincide en el conjunto, pero no dentro de cada dominio. También puede influir la familiaridad adquirida con los documentos. Por ello, la diferencia medida será descriptiva de esta prueba, no una estimación causal ni una promesa de ahorro en la actividad ordinaria. Esto no impide utilizarla para decidir si merece la pena continuar con la herramienta.

No se realizaron inferencias en la preparación, ni se integra una IA en el SV. Tampoco se modifica el libro. La ejecución de las doce revisiones está autorizada y se realizará conforme al encargo de edición 2, con los materiales ya fijados.

## Antecedentes y derechos

El estudio reutiliza el [contraste de oposiciones explícitas](../2026-10-11-oposiciones-explicitas/RESULTADO-Y-ALCANCE.md) y sus resultados íntegros. Las traducciones conservadas proceden de la preparación anterior; no se solicita una nueva traducción. Los originales conservan sus derechos y atribuciones; los derivados de INCIBE mantienen CC BY-NC-SA 4.0. El pie siguiente corresponde a este protocolo propio.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
