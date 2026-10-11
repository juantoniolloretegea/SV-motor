# Comprobaciones, estado y fuentes

**Referencia de esta edición: 11 de octubre de 2026.**

## Capacidades y límites

El sistema dispone de lectura Markdown mediante mdBook, representación gráfica de páginas PDF, conversión textual autorizada, índice y cobertura por intervalos, historial documental y consulta de los expedientes de entrega.

Las comprobaciones abarcan conservación de originales, identidad de las representaciones, intervalos parciales, controles de acceso y licencia, retirada de autorizaciones y mantenimiento de los antecedentes. Incluyen resultados favorables e impedimentos. No constituyen una certificación general de seguridad ni de exactitud científica.

Se han comprobado libros sintéticos de 130 y 1.025 páginas mediante intervalos. Esas pruebas verifican enumeración y recuperación del texto previsto; no demuestran que cualquier tratado complejo pueda convertirse íntegramente. La revisión gráfica y textual de originales reales tiene un alcance limitado a los documentos y páginas examinados.

Permanecen sin acreditar la fidelidad universal de los PDF, la reconstrucción íntegra de tablas y diagramas complejos y la entrega directa de PDF o HTML a los modelos en este recorrido. El extractor no realiza reconocimiento óptico de caracteres.

## Función de los componentes

| Componente | Función |
| --- | --- |
| mdBook 0.5.4 | Organizar y presentar la lectura |
| html-to-markdown-rs 3.17.2 | Convertir HTML conservado a Markdown |
| pulldown-cmark 0.13.0 | Interpretar la estructura Markdown para su presentación |
| Extractor documental PDF del SV | Recuperar texto por páginas e informar de su cobertura |
| Hayro 0.8.0 | Representar páginas PDF como imágenes; requiere cotejo cuando la fidelidad sea decisiva |

Estas versiones identifican los componentes de referencia. Que exista una versión posterior no significa que esté incorporada. Ninguno de estos componentes autoriza por sí mismo una entrega documental.

## Fuentes primarias

- [mdBook: documentación oficial](https://rust-lang.github.io/mdBook/).
- [html-to-markdown: proyecto del conversor](https://github.com/xberg-io/html-to-markdown).
- [pulldown-cmark: proyecto del analizador](https://github.com/pulldown-cmark/pulldown-cmark).
- [pdf-extract: biblioteca de extracción](https://github.com/jrmuizel/pdf-extract).
- [Hayro: representación PDF y limitaciones](https://github.com/LaurenzV/hayro).

Los resultados detallados se conservan en el [registro público de comprobaciones de los visualizadores](https://github.com/juantoniolloretegea/SV-motor/tree/main/laboratorio/ensayo-ia-y-observabilidad/visualizadores-de-documentos/comprobaciones). Las ediciones anteriores permanecen en el historial de publicación. Esta guía explica el funcionamiento vigente sin reproducir la secuencia de desarrollo.

## Derechos y alcance de la explicación

Cada documento y componente de terceros conserva su propia licencia. La visualización, traducción o transformación no amplía los derechos de uso. El permiso de suministro a IA debe examinarse separadamente.

Este libro no acredita cumplimiento íntegro de normas sanitarias o de ciberseguridad ni sustituye la revisión profesional del contenido.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
