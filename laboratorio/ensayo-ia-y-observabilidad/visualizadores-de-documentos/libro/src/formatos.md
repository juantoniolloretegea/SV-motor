# Formatos, representaciones y límites

El formato del original y la forma en que se presenta deben permanecer identificados. Una apariencia legible no acredita recuperación íntegra del contenido.

## Capacidades comprobadas en la edición de referencia

| Original | Tratamiento disponible | Qué debe tener presente el revisor |
| --- | --- | --- |
| Caché HTML | Conversión autorizada del archivo conservado a Markdown y presentación mediante mdBook | No se vuelve a consultar el sitio para completar contenido ausente. No se ejecutan programas de la página. La conversión requiere cotejo |
| PDF | Extracción autorizada de texto a Markdown; representación gráfica de páginas completas dentro de mdBook | Son procedimientos distintos. La imagen permite cotejar figuras y tablas, pero puede contener omisiones; no proporciona su interpretación ni su estructura de datos |
| Markdown | Presentación organizada mediante mdBook | Se conserva el texto del original; la presentación controlada no ejecuta HTML incorporado ni carga enlaces o imágenes externos |

El original permanece descargable para quienes tengan permiso. La consulta de PDF y HTML muestra las representaciones disponibles vinculadas a ese original; si no existe una representación accesible, lo indica. Abrir el visor no inicia por sí solo una conversión.

Las referencias a imágenes incluidas en Markdown siguen mostrándose como una indicación textual: **la figura no está disponible en esa representación textual**. En un PDF habilitado, «Ver páginas y figuras en mdBook» abre la representación gráfica de una página completa. Las dos formas de consulta se identifican por separado y no se completan entre sí mediante inferencia.

## Consulta gráfica de PDF

La vista ofrece selección de página física, página anterior y siguiente, ampliación normal o doble y desplazamiento por teclado. No depende de un complemento PDF del navegador. La imagen procede del original conservado y mantiene juntos texto, trazos y figuras en el contexto de la página. No se ejecutan programas del PDF ni se recuperan sus recursos externos.

Cada preparación identifica el método y su versión, la huella del original, la página, el total de páginas, la ampliación, las dimensiones y la huella de la imagen. Se registran los avisos que emite el intérprete y el uso de fuentes incluidas en el lector cuando el original no las proporciona. La ausencia de avisos no acredita fidelidad completa. Hayro 0.8.0, el componente empleado, declara limitaciones y un estado experimental.

La imagen es temporal; su informe y la actuación permanecen en el historial del documento. Se registra quién solicitó la preparación y cuándo, sin inferir que haya leído o aprobado la página. Cada página y cada imagen requieren permisos vigentes. Una vista caducada debe abrirse de nuevo desde la ficha.

La consulta gráfica exige consulta personal y uso Markdown acreditados, además del acceso al dominio y de un PDF recibido. El depósito reservado por sí solo no habilita esta representación. Tampoco se concede autorización de suministro a IA.

## Texto y codificación

La conversión HTML actual recibe texto UTF-8. Una codificación diferente debe identificarse y tratarse expresamente; no se debe asumir que una cadena legible está libre de sustituciones incorrectas.

En PDF, el orden de lectura y los caracteres recuperados dependen también de cómo se haya construido el archivo. Deben cotejarse tildes, ligaduras, signos, números, unidades, negaciones y continuidad entre columnas. El informe de extracción conserva advertencias y sustituciones cuando las detecta. La ausencia de advertencias no demuestra ausencia de errores.

## Páginas y libros extensos

La edición comprobada mantiene los siguientes límites de operación:

| Magnitud | Límite actual | Interpretación |
| --- | --- | --- |
| Archivo original | 20 MiB | Tamaño admitido por la operación actual; no es un límite inherente a PDF o mdBook |
| Páginas físicas del PDF | 4.096 | Cota del extractor vigente |
| Intervalo de extracción | Hasta 64 páginas | Se aplica a una operación, no al tamaño máximo conceptual de un libro |
| Texto resultante | 1 MiB por operación | Cota del resultado procesado |
| Derivados reunidos en el índice | Hasta 256 por original | La vista rechaza un conjunto superior; no lo presenta truncado como completo |
| Representación gráfica | Una página por operación; escala normal o doble | No se representa todo el libro de una vez |
| Dimensiones gráficas | Hasta 4.096 píxeles por lado y ocho millones de píxeles | La ampliación puede superar la cota aunque la vista normal sea admisible |
| Imagen resultante | Hasta 8 MiB en PNG | Una imagen rechazada no se presenta como resultado válido |

Cada intervalo debe identificar primera página, última página y total de páginas. La numeración física puede diferir de la impresa en el libro. Extraer todas las páginas no demuestra haber recuperado todas sus figuras, notas o relaciones estructurales.

«Índice y cobertura del libro» reúne las extracciones recibidas del mismo original. Distingue páginas extraídas, páginas con cotejo favorable registrado, páginas pendientes, solapamientos y páginas cuyas huellas textuales difieren. Una página presente en varios intervalos cuenta una sola vez. Una discordancia impide atribuirle cotejo favorable en la vista de conjunto. Los informes incompletos o ligados a otro original no acreditan cobertura.

Recibir el último intervalo no acredita la recepción de los anteriores. La opción «Preparar el siguiente intervalo pendiente» propone hasta 64 páginas y requiere confirmar la conversión. Puede reducirse ese intervalo. La preparación no convierte automáticamente todo el libro ni habilita su suministro.

El índice utiliza los marcadores incorporados al PDF, cuando existen. Recupera títulos y destinos internos explícitos, con un máximo de 128 entradas y 32 niveles. Los destinos nominales o externos no se resuelven; se declaran pendientes. No se deducen capítulos a partir del texto ni se siguen enlaces externos. La ausencia de marcadores no impide consultar las páginas y preparar intervalos. La numeración utilizada es física, desde 1; no se presupone correspondencia con la numeración impresa.

Antes de describir o extraer se comprueba la estructura de páginas. Las cuentas discordantes, páginas repetidas y ciclos impiden aceptar un total aparente. Si una página carece de texto recuperable o presenta un error de extracción, el intervalo no se recibe parcialmente: debe reducirse y conservarse identificada la laguna. Esta política puede rechazar PDF defectuosos que otros lectores reparan de forma implícita.

El índice y su informe requieren permisos vigentes; sus enlaces no eluden la comprobación de acceso. El informe distingue cobertura física y cotejo, sin atribuir fidelidad universal a ninguno. Las cotas indicadas son límites operativos de esta edición: no autorizan a afirmar que todo tratado extenso pueda procesarse íntegramente.

## Tablas, figuras y fórmulas

Las tablas sencillas de HTML pueden conservar su estructura. Una tabla PDF puede perder celdas, encabezamientos, notas o relaciones entre valores. No deben completarse datos mediante inferencia.

Un gráfico puede combinar imágenes, texto y trazos vectoriales. Por ello, extraer imágenes incrustadas no basta para recuperar todas las figuras; un contador de imágenes igual a cero tampoco acredita que no existan. La representación de página completa conserva el contexto visual que el intérprete haya podido producir. **No se acredita fidelidad universal, reconocimiento óptico ni recuperación semántica de figuras o tablas.**

No se separan automáticamente figuras individuales. Las páginas escaneadas pueden mostrarse como imágenes si su codificación es admitida; ello no recupera su texto. La visualización no equivale a interpretación científica, extracción accesible ni comprobación de la validez de una tabla.

## Revisión del resultado

Una representación pendiente de cotejo puede consultarse para revisarla; ello no la habilita para suministro. El cotejo debe dejar constancia de lo revisado y de sus límites. Un resultado desfavorable o incompleto no se presentará como reproducción íntegra del documento.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
