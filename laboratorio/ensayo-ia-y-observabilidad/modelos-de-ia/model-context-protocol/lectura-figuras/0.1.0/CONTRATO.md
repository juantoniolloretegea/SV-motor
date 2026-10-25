# Extracción de texto nativo SVG · 0.1.0

Esta utilidad Rust recupera texto ya contenido en una figura SVG. No realiza OCR, traducción, reconocimiento de relaciones espaciales ni interpretación científica. Su política de selección mantiene el OCR local como alternativa posterior a una fuente estructurada fiel y a la extracción nativa suficiente.

## Entrada y salida

La entrada se recibe como bytes UTF-8 por la entrada estándar. Se limita a 2 MiB, 128 niveles, 128 KiB de texto y 4096 etiquetas. Se rechazan documentos incompletos, raíces ajenas, DTD, entidades no admitidas y contenido exterior a la raíz. No se abren referencias, rutas, fuentes, enlaces o recursos contenidos en la figura.

La salida JSON identifica el formato, la huella SHA-256 del original, el método, las etiquetas y la ausencia de texto extraíble. `orden_documental_no_relaciones_espaciales` declara que el orden corresponde al archivo. El resultado no acredita flechas, posiciones, alineación, índices matemáticos ni orden visual. La normalización de espacios facilita lectura; el original gráfico conserva su identidad y autoridad.

El procedimiento recupera `title`, `desc` y `text`, incluidos sus segmentos textuales. El contenido reconocido como texto se trata exclusivamente como datos. La presentación HTML debe codificarlo como texto, sin interpretarlo como marcado o instrucciones.

## Comprobación y límites

Se conservan código, dependencias fijadas, pruebas y comprobación Clippy. La extracción se ha ejercitado sobre seis SVG distintos de la biblioteca, con comunicaciones externas denegadas. La evidencia relaciona sus huellas y número de etiquetas. No constituye una auditoría integral del servicio MCP ni una admisión general de imágenes suministradas por terceros.

La utilidad no modifica las herramientas del servidor MCP instalado. La política Rust selecciona un procedimiento según disponibilidades comprobadas; declarar un motor OCR disponible requiere una recepción propia. La condición de ausencia de texto nativo no autoriza automáticamente OCR: puede existir una fuente estructurada preferente.

## Referencias

- [Catálogo de lectura y aislamiento](../../CATALOGO-DE-LECTURA.md).
- [Estudio de reconocimiento óptico local](../ESTUDIO-OCR-LOCAL.md).
- [Código de extracción](src/lib.rs) y [política de selección](src/politica.rs).
- [Pruebas](evidencias/PRUEBAS.txt) y [figuras examinadas](evidencias/FIGURAS.json).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
