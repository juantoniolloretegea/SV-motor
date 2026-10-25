# Fuente común de figuras documentales

Versión 0.1.0 · 25 de octubre de 2026.

Esta utilidad Rust conserva una fuente estructurada común para la figura SVG y su lectura en HTML o Markdown. Recupera textos y composición explícitos; no reconoce píxeles, no ejecuta modelos de IA y no interpreta las relaciones científicas de una figura. Se relaciona con el [catálogo de lectura](../../CATALOGO-DE-LECTURA.md).

## Entrada y representaciones

La operación `importar` recibe un SVG estático admitido y produce JSON con la identidad SHA-256 del original, un repertorio de textos identificados y su estructura visual. Las operaciones `svg`, `html` y `markdown` reciben esa fuente JSON. El contenido se intercambia por la entrada y la salida del proceso; no se reciben rutas de archivos, instrucciones de ejecución ni direcciones de servicios.

Una modificación en un texto de la fuente se refleja al regenerar las tres representaciones. El original histórico se conserva por separado. La fuente importada conserva el orden de las etiquetas y la composición gráfica, pero declara expresamente que no se han inferido relaciones. La dirección aparente de una flecha no se convierte en una relación del SV.

Las figuras derivadas mantienen el español original. La lectura HTML puede participar en la traducción auxiliar de la biblioteca; traducirla no modifica automáticamente las etiquetas del SVG ni acredita equivalencia científica. La traducción y una futura representación mediante egui requieren sus comprobaciones propias. No se presenta esta utilidad como visor interactivo ni como traductor.

## Validación y límites

Se admiten únicamente los elementos estáticos necesarios para las seis figuras recibidas. Quedan excluidos los programas, atributos de eventos, estilos arbitrarios, imágenes incrustadas, referencias externas, instrucciones XML y declaraciones de entidades. Las referencias internas deben existir y los identificadores no pueden repetirse. Cada texto debe tener una única referencia; no se admiten textos adicionales sin correspondencia visual.

La entrada y la salida de la aplicación de consola se limitan a 2 MiB. Se comprueban además 128 KiB de texto, 4.096 fragmentos, 8.192 elementos y fragmentos, profundidad acotada y un máximo de 32 atributos por elemento. La deserialización JSON rechaza propiedades desconocidas. Las salidas codifican los caracteres reservados de su formato. Los límites protegen esta transformación concreta; no constituyen una evaluación general de todo archivo SVG ni sustituyen los límites de proceso de su futura integración.

Un documento rechazado no se corrige silenciosamente ni se completa mediante IA. La aplicación final deberá presentar la causa comprensible de la indisponibilidad, conservando el original y la traza de la operación.

## Comprobación recibida

Siete pruebas automatizadas comprueban la correspondencia entre representaciones, la conservación de caracteres y espacios, el rechazo de contenido ejecutable y recursos externos, las referencias internas, las cotas y los documentos incompletos. Las seis figuras de prueba conservan sus textos y composición explícita tras la transformación de ida y vuelta. Se comprueban formato y Clippy sin advertencias en esta utilidad.

La recepción no acredita equivalencia semántica universal, interpretación de fórmulas, precisión OCR ni conformidad de gráficos ajenos al conjunto admitido. Los archivos de prueba se conservan en `tests/figuras/`; sus nombres identifican el SHA-256 de cada original. Los originales mantienen sus atribuciones.

## Aplicación documental y límites de integración

La edición pública utiliza las seis fuentes para dieciocho apariciones de figuras, incluidas las páginas de impresión. Ofrece la lectura desplegable, la fuente común y el original conservado. La distribución pública sólo contiene figuras que ya eran públicas. Los documentos privados quedan sujetos a sus permisos y no se incorporan a este almacenamiento compartido.

La utilidad se incorpora al catálogo como capacidad documental comprobada. No constituye por sí sola una herramienta habilitada en el servidor MCP ni concede a un modelo autoridad sobre la admisión, los permisos o el núcleo.

| Omisión examinada | Consecuencia posible | Control y evidencia | Límite |
|---|---|---|---|
| Mantener textos independientes de la figura | Diferencias entre lo mostrado y lo leído | Fuente común y prueba de modificación simultánea | La corrección científica del texto requiere revisión |
| Interpretar trazos como relaciones | Atribución de un significado no declarado | Campo de relaciones no inferidas y conservación de la estructura explícita | No se genera semántica del SV |
| Admitir recursos ejecutables o externos | Ejecución o comunicación ajena a la lectura | Lista cerrada de elementos y pruebas de rechazo | No equivale a una auditoría de todos los lectores SVG |
| Distribuir una fuente privada como pública | Divulgación no autorizada | Conjunto limitado a originales públicos identificados | Una integración privada requiere autorización y control propios |

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
