# Comprobaciones, estado y fuentes

**Edición documental V.01 · Corte de información: 11 de octubre de 2026.**

## Estado que explica esta edición

La referencia es el lector documental con presentación Markdown mediante mdBook, consulta de representaciones vinculadas a originales HTML y PDF, descarga de originales y separación entre revisión y suministro. La inspección del recorrido de entrega confirma que prepara fragmentos textuales, sin acreditar envío directo de los originales PDF o HTML.

Se conservan resultados anteriores de 65 comprobaciones Rust: 33 de módulos y 32 del recorrido documental de los tres nodos; además, 23 controles HTTPS. Verifican comportamientos delimitados, no tres inferencias nuevas ni una certificación de seguridad. La consulta de la interfaz comprobó índice, procedencia, contenido, tabla sencilla, tildes, retorno de navegación y bloqueo de una conversión no autorizada. La edición de este libro no convierte esas pruebas anteriores en una ejecución nueva.

La preparación y publicación del libro tienen verificaciones propias de construcción, correspondencia, enlaces y consulta. Sus resultados se conservan en el registro de esta edición, separado de las comprobaciones funcionales del lector.

## Componentes y función efectiva

| Componente de referencia | Función en el recorrido descrito | Límite de la afirmación |
| --- | --- | --- |
| mdBook 0.5.4 | Organización y generación del libro | No extrae PDF ni autoriza contenido |
| html-to-markdown-rs 3.17.2 | Conversión de HTML conservado a Markdown | No interpreta científicamente figuras ni garantiza fidelidad de todo HTML |
| pulldown-cmark 0.13.0 | Lectura de la estructura Markdown para su presentación controlada | No resuelve omisiones del original ni reconstruye figuras |
| Extractor documental PDF del SV | Recuperación de texto por páginas e informe de extracción | No acredita OCR, representación gráfica íntegra ni recuperación de tablas complejas |

Las versiones identifican la edición examinada. La existencia de una versión posterior de una biblioteca no implica su incorporación al SV.

## Pendientes identificados

Permanecen pendientes la entrega directa de PDF y HTML en el recorrido descrito, la presentación reunida del expediente de entrega junto al visor, la representación gráfica de PDF, los recursos visuales autenticados y el tratamiento integral de libros extensos. Deben comprobarse antes de presentarlos como funciones disponibles.

Para cada ampliación se deberán revisar tanto resultados favorables como rechazos: autorización ausente o retirada, formato no compatible, extracción parcial, codificación defectuosa, figura no recuperada e incertidumbre de envío. Los fallos se registrarán sin sustituirlos por resultados supuestos.

No se acredita aquí compatibilidad universal entre navegadores, corrección de todo PDF, comprensión del modelo ni cumplimiento íntegro de normas sanitarias o de ciberseguridad.

## Fuentes primarias

- [mdBook: finalidad y documentación oficial](https://rust-lang.github.io/mdBook/).
- [mdBook: organización del índice](https://rust-lang.github.io/mdBook/format/summary.html).
- [mdBook: Markdown, tablas e imágenes](https://rust-lang.github.io/mdBook/format/markdown.html).
- [html-to-markdown: proyecto del conversor](https://github.com/xberg-io/html-to-markdown).
- [pulldown-cmark: proyecto del analizador Markdown](https://github.com/pulldown-cmark/pulldown-cmark).
- [pdf-extract: biblioteca de extracción de contenido PDF](https://github.com/jrmuizel/pdf-extract).

Las fuentes de terceros describen sus propios componentes. Sus capacidades no se atribuyen al SV hasta verificar su integración. La referencia pública de este libro se conserva en [Visualizadores de documentos, SV-motor](https://github.com/juantoniolloretegea/SV-motor/tree/main/laboratorio/ensayo-ia-y-observabilidad/visualizadores-de-documentos); la Biblioteca del SV ofrece la lectura y las fuentes de la edición.

## Ediciones y derechos

Las correcciones posteriores deberán identificar versión, fecha y alcance, conservando las referencias a la edición anterior. Los textos propios mantienen el aviso de autoría y licencia. Los documentos consultados y los componentes de terceros conservan sus derechos y condiciones particulares.

La presentación y traducción de lectura no amplían la licencia de los documentos ni autorizan su suministro. Este libro es una explicación pública del funcionamiento y sus límites, sin contenidos reservados ni configuraciones de acceso.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
