# Correspondencia y consulta de la evidencia documental

**Buscador-Semántico - Diferencial · Paso 6 / 8 · 11 de octubre de 2026.**

Se ha comprobado la correspondencia y la presentación de los 28 pares conservados del perfil experimental restrictivo: dieciséis controles originales en inglés y doce casos documentales con traducción instrumental. No se ha ejecutado nuevamente el detector. Las cifras de aciertos, errores y omisiones del [contraste anterior](../2026-10-11-oposiciones-explicitas/RESULTADO-Y-ALCANCE.md) permanecen intactas.

## Resultado comprobado

| Objeto | Resultado | Alcance |
| --- | --- | --- |
| Pares documentales | 28 cotejados | Dos documentos parciales por par; separación de inmunología y ciberseguridad. |
| Traducciones conservadas | 20 correspondencias español–inglés | Identificadores, huellas y reconstrucción exacta del documento inglés utilizado. No acredita equivalencia semántica. |
| Avisos anteriores | 7 conservados | A01, A02, A03, A04, A08, A16 y C03. Cada uno declara directamente una sola localización. |
| Localizaciones declaradas | 7 cotejadas | Archivo, intervalo de bytes, límites UTF-8 y correspondencia única con su pasaje de procedencia. |
| Casos sin aviso | 21 conservados | La presentación indica expresamente que el silencio no demuestra coherencia. |
| Controles de integridad | 12 conformes | Un control de referencia, diez alteraciones rechazadas y un control de texto inerte. |
| Presentación HTML | 28 páginas de casos cotejadas | Permanencia íntegra de documentos, originales, traducciones y fragmentos directamente localizados. |
| Inspección visual | C03, A08 e I03 | Examen acotado de las zonas consultadas; no revisión visual exhaustiva de las 28 páginas. |

Las evidencias se encuentran en [CORRESPONDENCIAS.json](CORRESPONDENCIAS.json), [COMPROBACIONES-RUST.json](COMPROBACIONES-RUST.json), [COMPROBACION-HTML-FINAL.json](COMPROBACION-HTML-FINAL.json) e [INSPECCION-VISUAL.json](INSPECCION-VISUAL.json). La [muestra documental](consulta-mdbook/src/indice.md) permite consultar los casos por dominio. Las fuentes mdBook y su HTML comprobado se conservan juntos; esta muestra no crea otro libro de la Biblioteca ni constituye una aplicación integrada.

## Presentación y procedencia

Cada caso muestra completos los dos documentos parciales que intervinieron en la comparación, incluidos los encabezamientos. Cuando existe original español, se presenta junto a la traducción inglesa previamente conservada de Google. Los controles redactados originalmente en inglés se identifican expresamente. Se indican la procedencia, el formato, la edición y el localizador disponibles. En PDF se distingue la página física, cuyo ordinal comienza en 1, del índice que comienza en 0.

Los documentos propuestos contienen alteraciones experimentales y no se atribuyen como afirmaciones de la publicación de referencia. En particular, los controles sanitarios no constituyen recomendaciones clínicas. Se conservan las condiciones de los originales y de sus derivados; los derivados de INCIBE mantienen CC BY-NC-SA 4.0. La licencia indicada al final corresponde a esta exposición propia.

La localización del diagnóstico se enlaza al pasaje de origen sólo cuando la correspondencia es única. El segundo pasaje no está identificado expresamente en los resultados históricos: se muestra el otro documento completo dentro de la cobertura ensayada. No se obtiene una localización mediante el mensaje abreviado, la semejanza textual o la respuesta prevista.

## Método y correcciones instrumentales

El auxiliar Rust reutiliza serde_json para leer las evidencias y sha2 para comprobar su identidad. html-escape se limita a normalizar entidades HTML durante el cotejo. mdBook 0.5.4 realiza la representación. No se ha añadido un analizador semántico ni se han modificado las reglas del comparador, el Núcleo, su representación intermedia o el Árbitro - Director.

Los doce controles se ejecutaron inicialmente y se repitieron sobre la versión final del auxiliar. Las diez alteraciones introducen ausencia o duplicación de traducciones, cambios en el original o la traducción, dominio discordante, modificación del documento inglés, pérdida de un resultado, intervalo fuera de rango, corte de un carácter UTF-8 y duplicación de un caso. El control restante comprueba que una cadena con instrucciones HTML e inclusión documental se representa como texto.

Los dos primeros cotejos HTML no fueron conformes por diferencias de representación que no alteraban el contenido: normalización de las comillas del atributo de presentación y de la entidad correspondiente al apóstrofo. Se corrigió el auxiliar para reconocer estas equivalencias, conservando ambos resultados no conformes. El tercer cotejo y el cotejo final resultaron conformes. El control de contenido activo es acotado; no equivale a una auditoría de seguridad integral del lector.

Después de las correcciones se regeneró únicamente la presentación y se repitieron sus controles. El cotejo independiente en Rust confirmó igualdad íntegra de los dos archivos de resultados y de los 32 archivos HTML generados: 34 archivos, 406.644 bytes. Por ello, la representación final coincide con la inspeccionada. Véanse [IDENTIDAD-FINAL-RUST.json](IDENTIDAD-FINAL-RUST.json) y las [instrucciones de reproducción](REPRODUCCION.md).

## Examen de los límites

- **A08:** los encabezamientos permiten advertir que una disposición es histórica y ya no se aplica. Conservarlos hace examinable el falso aviso anterior, pero no acredita que el comparador comprenda su vigencia.
- **C03:** permanecen visibles la traducción, el original y la diferencia modal entre obligación y recomendación negativa. No se elimina esa diferencia ni se transforma automáticamente el aviso en una contradicción resuelta.
- **I03:** se muestra un caso sin aviso con sus textos y la advertencia de cobertura. La omisión histórica no desaparece por mejorar su presentación.
- Las huellas acreditan identidad documental; no verifican la calidad de la traducción.
- Los pasajes conservados no equivalen a la lectura completa de los PDF, HTML o libros de origen.
- La localización directa de ambos pasajes, la integración opcional con la administración y la utilidad en su funcionamiento conjunto siguen pendientes.

## Continuación del recorrido

El paso 6 queda comprobado en cuanto a correspondencia, conservación y consulta de esta evidencia. La localización bilateral no está acreditada. Antes de presentar nuevos avisos con ambos pasajes señalados, la instrumentación deberá conservar sus dos localizaciones reales y el contexto correspondiente. Este requisito de evidencia no impone al revisor utilizar la herramienta ni resolver los avisos.

El paso 7 mantiene pendiente la integración delimitada con la selección por documento, grupo o conjunto accesible del mismo dominio. El paso 8 conserva la comprobación conjunta y la incorporación editorial coordinada. La [nota para el libro](NOTA-PARA-EL-LIBRO.md) está preparada para los capítulos existentes; no se declara incorporada.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

