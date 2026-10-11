# Formatos, representaciones y límites

## PDF: leer las páginas

**Ver páginas del PDF** abre el visor del SV. El selector de página, los enlaces **Anterior** y **Siguiente** y la ampliación **Normal** o **Doble** permiten recorrer el documento. La numeración cuenta las páginas del archivo desde uno; puede diferir de la numeración impresa.

El visor representa cada página como una imagen y la presenta mediante mdBook. Así puede mostrar texto, tablas, figuras y diagramas en su disposición original. La representación puede tener diferencias tipográficas u omisiones: no constituye una interpretación científica ni una transcripción accesible. Los avisos y el informe están en **Detalles y límites de esta vista**.

La revisión de un PDF depositado exige admisión y permiso vigente de consulta o depósito para revisión. No requiere autorizar una conversión textual a Markdown. Este acceso no habilita extracción, conversión textual, elaboración de índices ni suministro a la IA.

## PDF: obtener texto y organizar libros

**Preparar Markdown** extrae texto del intervalo elegido y conserva un resultado vinculado al original. Requiere los permisos de edición y de uso correspondientes. El texto queda pendiente de cotejo; terminar la conversión no lo aprueba.

El intervalo máximo es de **64 páginas por operación**, no de 64 páginas por libro. Un libro mayor se procesa por intervalos. El índice y la cobertura permiten consultar los intervalos recibidos, las páginas pendientes y los solapamientos. Los marcadores presentes en el PDF pueden ayudar a navegar; el sistema no inventa capítulos ausentes.

La extracción no aplica reconocimiento óptico de caracteres. Una página escaneada puede resultar legible como imagen y no producir texto. Tampoco se garantiza reconstruir tablas complejas, fórmulas, columnas ni diagramas. La ausencia de texto extraído no demuestra que la página esté vacía.

Actualmente se admiten archivos de hasta **20 MiB** y, para estas operaciones PDF, hasta **4.096 páginas físicas**, con límites adicionales por imagen y extracción. No equivalen a una garantía para cualquier libro de ese tamaño. Si se alcanza una cota, la operación debe informar del impedimento; no debe presentar un resultado truncado como completo.

## Página web conservada

La conversión utiliza el archivo HTML depositado; no vuelve a consultar la página de Internet. Produce Markdown cuando la operación está autorizada. Una referencia a una imagen, un recurso remoto o contenido generado mediante programas no garantiza que ese elemento esté incluido en el archivo ni en el resultado.

La revisión debe comprobar títulos, párrafos, listas, tablas y caracteres. El original conservado permite contrastar lo que estaba disponible en la captura.

## Markdown y mdBook

El lector presenta títulos, párrafos, listas y tablas mediante mdBook. En la representación textual controlada, las referencias a imágenes se muestran como texto; los programas y recursos externos del documento no se ejecutan. Esta lectura no incorpora automáticamente las figuras del PDF original.

**Procedencia y alcance** permite conocer el original, la transformación, la cobertura y los permisos. **Ficha, permisos e historial** reúne la información particular sin anteponerla al documento.

## Elegir la representación adecuada

| Necesidad | Lectura adecuada | Comprobación necesaria |
| --- | --- | --- |
| Examinar la disposición, una tabla o una figura de un PDF | Página del PDF en el visor | Comparar con el original cuando la fidelidad sea decisiva |
| Leer y seleccionar texto | Markdown original o derivado | Verificar caracteres, orden y cobertura |
| Revisar un libro extenso | Índice y cobertura de sus intervalos | Identificar páginas pendientes o repetidas |
| Saber qué recibió una IA | Expediente de la consulta | Examinar el contenido preparado y el resultado del envío |

La visualización no concede derechos nuevos ni determina qué contenido recibe un modelo.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
