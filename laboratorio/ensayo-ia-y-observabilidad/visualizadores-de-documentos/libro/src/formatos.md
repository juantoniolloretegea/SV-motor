# Formatos, representaciones y límites

El formato del original y la forma en que se presenta deben permanecer identificados. Una apariencia legible no acredita recuperación íntegra del contenido.

## Capacidades comprobadas en la edición de referencia

| Original | Tratamiento disponible | Qué debe tener presente el revisor |
| --- | --- | --- |
| Caché HTML | Conversión autorizada del archivo conservado a Markdown y presentación mediante mdBook | No se vuelve a consultar el sitio para completar contenido ausente. No se ejecutan programas de la página. La conversión requiere cotejo |
| PDF | Extracción autorizada de texto por intervalos de páginas físicas; conservación del resultado en Markdown y presentación mediante mdBook | No es una reproducción visual del PDF. No reconstruye automáticamente sus figuras, tablas complejas, fórmulas ni composición editorial |
| Markdown | Presentación organizada mediante mdBook | Se conserva el texto del original; la presentación controlada no ejecuta HTML incorporado ni carga enlaces o imágenes externos |

El original permanece descargable para quienes tengan permiso. La consulta de PDF y HTML muestra las representaciones disponibles vinculadas a ese original; si no existe una representación accesible, lo indica. Abrir el visor no inicia por sí solo una conversión.

Las referencias a imágenes se muestran actualmente como una indicación textual. **La figura no está disponible en esa representación.** Esta limitación pertenece a la integración actual del SV: mdBook admite imágenes y tablas, pero esa capacidad general no acredita su incorporación efectiva al visor.

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

Cada intervalo debe identificar primera página, última página y total de páginas. La numeración física puede diferir de la impresa en el libro. Extraer todas las páginas no demuestra haber recuperado todas sus figuras, notas o relaciones estructurales.

Un libro extenso requiere controlar intervalos, omisiones, solapamientos, índice, capítulos y referencias. La prueba instrumental de un PDF de 65 páginas permite comprobar intervalos; no acredita por sí sola el tratamiento integral de un tratado médico.

## Tablas, figuras y fórmulas

Las tablas sencillas de HTML pueden conservar su estructura. Una tabla PDF puede perder celdas, encabezamientos, notas o relaciones entre valores. No deben completarse datos mediante inferencia.

Un gráfico puede combinar imágenes, texto y trazos vectoriales. Por ello, extraer imágenes incrustadas no basta para recuperar todas las figuras; un contador de imágenes igual a cero tampoco acredita que no existan. La presentación gráfica fiel, la ampliación de figuras y el reconocimiento óptico de páginas escaneadas **siguen pendientes de incorporación y comprobación**.

Cuando se incorporen, cada representación gráfica deberá conservar referencia al original, página, huella, alcance y licencia. Si no es posible aislar una figura con fidelidad suficiente, deberá identificarse esa limitación y conservar una representación de la página completa. La visualización no equivale a interpretación científica.

## Revisión del resultado

Una representación pendiente de cotejo puede consultarse para revisarla; ello no la habilita para suministro. El cotejo debe dejar constancia de lo revisado y de sus límites. Un resultado desfavorable o incompleto no se presentará como reproducción íntegra del documento.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
