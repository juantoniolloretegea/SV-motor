# Contrato transversal de suficiencia documental, abstención y alertas

7 de octubre de 2026 · Versión 1 · Aplicable a los tres nodos de acoplamiento y a todos sus modelos.

Esta obligación depende de la información efectivamente suministrada y de la finalidad de la pregunta, **no del formato**. Rige para PDF, HTML de páginas en caché, texto, libros, tablas, documentos compuestos y cualquier otro material entregado al candidato. El lector o el transporte concretos no modifican la responsabilidad del Árbitro-Director ni los pilares del SV.

## Antes de ejecutar

El encargo debe fijar la pregunta, la capacidad que pretende medir, el conjunto de fuentes, sus versiones y huellas, la cobertura necesaria, la información efectivamente disponible, los parámetros críticos y los criterios de evaluación. Debe distinguir entre obtener un dato o conclusión sustantivos y reconocer justificadamente que una fuente no permite obtenerlos. No se convierte una carencia del diseño en una exigencia imposible al candidato.

El suministro debe acreditar integridad, orden y contexto suficiente. Las referencias dependen del soporte —página y fragmento, sección HTML, rango de texto, celda, etc.—, pero todas deben permitir comprobar las premisas sobre los mismos bytes entregados. Una extracción incompleta no se oculta como insuficiencia de conocimiento del modelo.

## Instrucción obligatoria al candidato

> Trabaje exclusivamente con el material suministrado y dentro del alcance del encargo. No invente hechos, citas, localizadores, acciones realizadas ni conclusiones para completar una respuesta. Si no puede determinar fundadamente lo solicitado, declare expresamente `U`: identifique el extremo indeterminado, la razón, la evidencia que pudo contrastar y la información concreta que falta. Separe lo que sí está respaldado de lo que no puede establecer.
>
> La declaración U exige fundamento verificable; no sustituye la lectura ni es una salida automática. Si el material permite responder, utilícelo. Si la fuente deja un punto abierto, documente ese límite sin completarlo con memoria externa o fuentes no autorizadas. No se le pide pensamiento privado, sino una explicación comprobable de sus afirmaciones y limitaciones.

El adaptador de cada nodo debe incluir realmente esta instrucción en el encargo y conservar la solicitud exacta y su huella. Un documento de preparación no acredita transmisión al candidato. La siguiente ejecución deberá cotejar esa inclusión antes del envío; no se ejecuta inferencia por publicar este contrato.

## Control exterior al candidato

El candidato declara su estado de respuesta, pero no se adjudica un resultado. El Árbitro-Director y sus componentes Rust, con la revisión competente que corresponda, mantienen independientes el suministro, la respuesta original, las comprobaciones, la adjudicación, las alertas y la telemetría.

Ante una alegación de insuficiencia deben contrastarse:

1. Qué se pidió y qué capacidad se pretendía medir.
2. Qué material íntegro recibió realmente el modelo.
3. Qué extremo afirma no poder determinar.
4. Qué pasajes respaldan o contradicen esa afirmación, con localizadores comprobables.
5. Si existe un límite explícito, una inferencia documental insuficiente, una carencia del encargo o una respuesta evasiva sin justificación.
6. Qué revisión competente sostiene la adjudicación y qué permanece pendiente.

La cita literal comprobada aporta trazabilidad; por sí sola no prueba toda la interpretación. El comprobador mecánico no sustituye la revisión semántica ni determina intención de engañar. Una contradicción, una afirmación inventada o una abstención injustificada se documenta por sus evidencias, sin aceptar la versión del modelo como autoridad.

## Resultados y alertas inseparables

| Situación | Tratamiento requerido |
|---|---|
| La pregunta evalúa reconocer un límite y éste queda acreditado | Puede adjudicarse 0 conforme a la clave, **siempre acompañado de símbolo de peligro y explicación de la capacidad no demostrada** |
| La pregunta necesita información que el suministro no contiene y no evaluaba reconocer esa carencia | Alarma e incidencia de diseño para revisión competente; no encubrirla con un 0 ni obligar al modelo a inventar |
| Indeterminación sustantiva justificada y admitida por el contrato | U, conservando causa, evidencia, alcance y consecuencias de criticidad |
| Alegación de falta de datos contradicha por información suficiente suministrada | Revisar el incumplimiento; no aceptar U automáticamente |
| Defecto de suministro, transporte o instrumentación | Incidencia técnica fuera de 0/1/U, sin culpar al candidato ni completar a posteriori su contexto |
| Error en un parámetro crítico | **No apto**, aunque κ sea Apto o la puntuación sea alta |

La advertencia del 0 por límite debe estar visible en el resumen, junto al resultado particular y vinculada a su posición en el polígono. No bastará ocultarla en un desplegable. Conservará motivo, evidencia, control efectuado, revisión de diseño y capacidad que no se acredita. La alerta es una anotación auxiliar: no crea un cuarto símbolo, no cambia la terna ni altera sus radios y colores.

La κ calculada con T(n)=⌊7n/9⌋ no sustituye la condición eliminatoria. Una U crítica tampoco habilita la admisión: se aplica el tratamiento de indeterminación del contrato. Si las criticidades no estaban constituidas, su ausencia no equivale a `false` ni a U y no permite declarar admitido al candidato.

## Evidencia y recepción

Cada ejecución debe conservar la relación entre contrato, pregunta, fuente entregada, solicitud, respuesta, alegación de insuficiencia, comprobaciones, adjudicación y alerta. El registro de sucesos y el tique identificarán incidencias de diseño, condiciones de continuación y revisión pendiente. La instrumentación y los controles propios del SV se realizan en Rust; el candidato no puede alterarlos.

La primera materialización visual de esta regla añade avisos a PDF02 y PDF09 del anexo documental de Astra, conservando sus resultados originales. Este antecedente **no limita el contrato al PDF** ni acredita que todos los adaptadores históricos ya lo ejecuten. Antes de nuevas pruebas, la recepción de cada realización deberá demostrar tanto el envío de la instrucción como el control y la presentación de alertas. Las fuentes y los ensayos históricos permanecen intactos.


© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
