# Revisión metodológica de MD01: contexto de las etapas y alcance del dictamen

08/10/2026 · Versión 1.0 · GPT-6 Astra, nodo 03 · Ensayo del manual Markdown.

## Dictamen de esta revisión

**Se confirma una deficiencia de nuestro diseño del contexto: se conservaron exactamente las respuestas anteriores, pero no los encargos completos bajo los que se produjeron.** La orden vigente de numeración de etapa no delimitaba expresamente su aplicación a la entrega actual. El error observado en la revisión de Astra es real; no está demostrada su atribución exclusiva al modelo ni al proveedor. La calidad general de una arquitectura no acredita la corrección de una respuesta concreta.

**El «No apto» histórico queda bajo reserva metodológica.** No debe utilizarse como demostración concluyente de que el candidato falla la competencia crítica de MD01, que era distinguir un constructor documental de un manual terminado. Esa distinción está correctamente respondida en las tres etapas. Tampoco se sustituye el resultado por «Apto»: esta revisión no es una nueva ejecución ni una recepción independiente.

Se conserva íntegra la evidencia del ensayo, incluido el vector R2 `(1,0,0,0,0,0,0,0,0)`, su polígono y el dictamen original. La reserva se añade por adenda; no se corrigen respuestas del candidato, no se convierte un defecto del instrumento en U y no se elige retrospectivamente una etapa más favorable.

## Fuentes y comprobación realizada

El ensayo original y el controlador están conservados en [Motor, revisión 5f989cb592bbbc7da6568c156b2f8c5da7adf689](https://github.com/juantoniolloretegea/SV-motor/tree/5f989cb592bbbc7da6568c156b2f8c5da7adf689/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008). La propuesta de contenido, la admisión y la clave se fijaron antes de la inferencia. Los originales de transporte y su custodia administrativa se mantienen en sus sedes reservadas.

El comprobador nuevo `sv-revisar-contexto-manual.rs` se ejecutó localmente, sin red ni invocación del candidato. Cotejó las 27 solicitudes y respuestas frente a las huellas de la recepción conservada y comprobó que todos los archivos fijados en PREVIA.json permanecen idénticos. Comprobó también que el corpus y la pregunta permanecen idénticos entre las tres etapas de cada caso, que las respuestas anteriores se reproducen exactamente y que las 27 respuestas consignan correctamente sus respectivas etapas. Su prueba de rechazo detecta alteración, reordenación y falta de respuestas en el historial.

El resultado es [COTEJO-RUST.json](COTEJO-RUST.json), SHA-256 `450dc08be5549c63abffef72cb60d054db3c9205b310aafbf9b4ac35a75f2ca1`. Este cotejo no vuelve a interpretar todos los eventos SSE: contrasta los originales con la recepción formal y de transporte ya conservada. La interpretación sustantiva que sigue es una revisión exterior al candidato asistida por IA, no una demostración semántica producida por el programa ni una auditoría independiente.

## 1. Qué se preguntó y qué respondió el candidato

MD01 preguntó: «¿Qué función cumple el constructor y qué diferencia establece respecto de un manual final?». La propuesta identificó su criticidad como «estatuto documental». El criterio prefijado fue organizar estructura, orden y cierre progresivo, sin presentar el constructor como manual final ni acreditar contenidos implementados.

Las tres respuestas preservan esa distinción. En R2, el candidato explica que el constructor organiza el desarrollo y las condiciones de cierre, pero no acredita que ese desarrollo o cierre se haya producido. Las citas fueron recibidas como conformes. No se encontró falta de suministro del pasaje necesario ni truncamiento de esa respuesta.

El motivo que originó el valor 1 fue esta afirmación adicional en `revision`:

> Las etapas consignadas en aquellas entregas no cumplieron la instrucción de fijar etapa 2; esta entrega utiliza el valor exigido.

La afirmación es inexacta respecto del historial completo del ensayo: R0 debía llevar 0 y lo llevó; R1 debía llevar 1 y lo llevó. R2 llevó correctamente 2. Por tanto, no faltó el parámetro `etapa` en la respuesta final ni falló su validación JSON. Lo defectuoso fue la explicación retrospectiva sobre las entregas anteriores.

La respuesta R2 original tiene SHA-256 `0d32cb6c151e080a6ab345dde80bbe2916bc4c53816324b9f0d260730683ba3c`. La clave fijada tiene SHA-256 `e8f9e1cf7c65856106dab42b07ccc3348de5862ea877979d02dcdd1d7f3e81c7`.

## 2. Deficiencia confirmada en la composición del contexto

La función Rust `contrato::compose` reconstruyó cada solicitud. En R2 incluyó:

1. El corpus completo y la pregunta.
2. La respuesta original R0, sin alteraciones.
3. Una instrucción abreviada de autocrítica.
4. La respuesta original R1, sin alteraciones.
5. La instrucción de verificación final de etapa 2.

La instrucción intermedia abreviada fue «Autocrítica documental universal: revise contra la fuente, sin presuponer errores, y entregue una respuesta completa». **No es el encargo real de R1**, que comenzaba «Etapa 1» y contenía las condiciones completas de esa revisión. Tampoco se repusieron como antecedentes identificados las instrucciones originales que exigían `etapa debe ser 0` y `etapa debe ser 1`.

En cambio, las instrucciones activas de R2 incluían la obligación general `etapa debe ser 2`, sin precisar que no debía aplicarse retrospectivamente. Se pedía además contrastar las entregas anteriores, que no debían tomarse como autoridad. Este conjunto permite una lectura temporal equivocada que nuestro instrumento debía prevenir.

Las nueve solicitudes R2 presentan la misma sustitución; no es un defecto exclusivo del archivo MD01. MD04/R2 incluye la observación «Se corrige el cumplimiento formal de etapa: esta entrega final lleva etapa 2», aunque no formula la misma imputación explícita contra sus antecedentes. Es un indicio adicional, no otra demostración automática de error ni motivo para modificar ahora su valor.

Todas las solicitudes examinadas tienen `store:false` y carecen de `previous_response_id`. No procede presumir que el proveedor disponía de las instrucciones de llamadas anteriores. La documentación oficial admite construir el contexto enviando mensajes e instrucciones; la selección de ese historial es responsabilidad del cliente. La reconstrucción manual no es por sí misma un uso incorrecto de la API, pero aquí fue insuficiente para una comparación científica inequívoca de obligaciones históricas. Véase la [guía oficial de migración a Responses](https://developers.openai.com/api/docs/guides/migrate-to-responses).

**Límite causal:** el contrato también anunciaba tres etapas universales y las respuestas anteriores mostraban 0 y 1. El candidato podía haber interpretado correctamente el alcance temporal. La deficiencia del instrumento no demuestra que el error fuera inevitable, ni que el modelo carezca de responsabilidad en su afirmación inexacta. Sí impide sostener una atribución exclusiva sin una comprobación controlada.

## 3. Alcance de la criticidad y función del Árbitro

La admisión prefijó MD01 como crítica y exigió entrega fiel y completa. El receptor Rust aplica efectivamente el veto cuando recibe un 1 adjudicado a esa posición. **No se ha descubierto un error aritmético del umbral ni del veto, ni se propone suavizar el principio de que un error crítico elimina.**

La insuficiencia está antes: la revisión sustantiva asignó ese 1 por la frase sobre numeración y el programa le aplicó la criticidad de toda MD01. No se había delimitado expresamente cómo relacionar la competencia «estatuto documental» con una incorrección auxiliar acerca del historial de etapas, particularmente cuando ese historial se presentaba incompleto. El Árbitro comprobó el cumplimiento del contrato programado; ello no demuestra que el contrato fuese metodológicamente suficiente.

No basta con declarar ahora inocua cualquier incorrección de trazabilidad: también puede ser grave. Para una futura edición hay que prefijar los requisitos de fidelidad del contenido, de revisión y de estructura, indicar cuáles son críticos y justificar sus consecuencias. La exigencia debe ser conocida por el evaluador antes de recibir respuestas y aplicada uniformemente a todos los candidatos. El candidato no debe recibir la clave ni pistas sobre fallos anteriores.

La conclusión original fue demasiado categórica al convertir este resultado en una incapacidad crítica demostrada. Se mantiene la observación de inexactitud, pero se añade la reserva sobre el diseño y la atribución.

## 4. Comprobación propuesta: sólo MD01, en tres etapas

Se recomienda una réplica diagnóstica separada, con tres llamadas: R0 provisional, R1 autocrítica y R2 verificación final neutral. Repetir sólo R2 sobre las respuestas antiguas no reproduciría el procedimiento completo de revisión. No se debe comunicar al candidato que falló ni en qué consistió la discrepancia.

Mantener el mismo modelo disponible, corpus fijado, pregunta, formato, parámetros de generación y límites de acceso. Corregir exclusivamente la representación del historial y el alcance temporal de las instrucciones. Conservar el encargo real de cada etapa y su respuesta, sin convertir las respuestas del candidato en instrucciones. Añadir como regla general:

> El campo `etapa` identifica únicamente la entrega que se solicita ahora. Cada entrega anterior se produjo bajo su propia instrucción y conserva ese identificador histórico. No aplique retrospectivamente el identificador actual a los antecedentes. Revise el contenido contra la fuente; puede mantenerlo, corregirlo o declarar U justificada. No presuponga errores.

El controlador Rust debe comprobar la correspondencia entre etapa, encargo y respuesta, la integridad del corpus, las restricciones de herramientas, las mediciones y la conservación antes y después del envío. Debe rechazarse localmente un contexto que no cumpla esa correspondencia. Prefijar y comprobar la nueva versión antes de inferir, sin modificar la versión histórica. La telemetría, el archivo por llamada y el tratamiento de interrupciones conservarían las condiciones aplicables.

La comprobación tendrá un expediente propio y resultado diagnóstico. No se sustituirá MD01 dentro del vector histórico como si los nueve casos procedieran de una única edición. Un resultado correcto aportaría evidencia de resolución bajo el contexto corregido; no demostraría por sí solo causalidad exclusiva ni estabilidad estadística. Si reaparece la afirmación inexacta con instrucciones inequívocas, aumentaría la evidencia de un problema del candidato en esa revisión. En ambos casos se conservarían los datos que pudieran refutar la hipótesis de diseño.

**No se ha ejecutado esta réplica.** La presente actuación es la revisión solicitada y la preparación de una propuesta concreta. No se ha reabierto el examen ni modificado el transporte vigente.

## 5. Anexo experimental y posible banco posterior

El «Anexo experimental de modelos de IA» es útil como documentación del manual, separado de esta comprobación causal. Debe declararse experimental, vincularse a los expedientes y no convertir resultados de laboratorio en doctrina o capacidades admitidas del Lenguaje.

Se propone una ficha por modelo y edición efectivamente probados:

- Identidad y versión; nodo; modalidad de inferencia y condiciones del entorno.
- Corpus, contrato, requisitos críticos y límites de las preguntas.
- Pruebas realizadas, resultados de cada etapa y alcance exacto de la admisión.
- Suministro documental, intervención del Árbitro e instrumentación efectivamente utilizada.
- Incidencias del instrumento, del candidato o del servicio, con atribución fundada o incertidumbre expresa.
- Necesidades demostradas para el Lenguaje: fronteras de autoridad, suficiencia, U, trazabilidad, privacidad, formatos y conservación.
- Versiones y enlaces inmutables; diferencias que impiden comparar directamente puntuaciones.

Los modelos pendientes de recursos no se presentarían como probados. Los costes remitirían a la custodia administrativa sin publicar saldos, credenciales o datos personales. Un banco de nueve preguntas sobre ese anexo sólo se fijaría después de recibir su contenido y verificar su suficiencia. Ocho preguntas adicionales también pueden proceder del manual actual, pero constituirían otro banco: no son necesarias para comprobar la deficiencia de MD01 y no deben mezclarse con ella para obtener un polígono aparentemente comparable.

## Estado de los productos

Revisión documental y cotejo Rust realizados. Sin nuevas inferencias del candidato, sin llamadas al proveedor y sin consumo nuevo de tokens del candidato. El consumo de asistencia de esta revisión no se atribuye como cero ni como gasto de Astra. Sin cambios de originales, clave, banco, controlador histórico, README, manual ni polígonos. La adenda acompaña el cierre original y delimita su interpretación; la réplica y el anexo permanecen como propuestas.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
