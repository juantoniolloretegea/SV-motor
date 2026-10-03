# Corrección instrumental del extractor

La preparación utiliza una variante local identificada como `pdf-extract 0.12.1-sv.1`, derivada de 0.12.1. No se atribuye esta corrección al proyecto original ni se presenta como una edición publicada por sus mantenedores.

En el PDF fijado, la versión original pierde ocho caracteres de ligaduras: cinco en la página de índice 8 y tres en la de índice 9. La caché de fuentes utiliza como clave el nombre del recurso; distintos diccionarios de recursos reutilizan ese nombre con codificaciones diferentes. Se delimita la clave mediante la identidad del diccionario de fuentes y el nombre. Las ligaduras se recuperan sin modificar el original.

Se añade un método de salida con identidad de fuente, manteniendo el método anterior para sus implementaciones existentes. Permite convertir únicamente los glifos declarados de Wingdings2, sin sustituir letras ordinarias. Se fijan 69 sustituciones tipográficas en este documento: 63 viñetas y seis círculos. Se acota la descompresión a 64 MiB y se fija lopdf 0.45.0. El archivo de diferencias conserva cada modificación de la biblioteca.

El adaptador propio enumera las páginas conocidas y exige la extracción de cada una. No utiliza la función original por páginas que interpreta un error como fin de recorrido. Rechaza caracteres nulos, sustitutos, controles inesperados, descodificación vacía y páginas sin texto. Un fallo no produce un catálogo conforme. No se afirma haber corregido todos los posibles defectos de la biblioteca.

Las evidencias EXTRACCIONES y COBERTURA iniciales se conservan como diagnóstico, incluidas sus discrepancias. La clave histórica `pdf_extract_0121` también aparece en la salida diagnóstica corregida; su identidad efectiva es la variante local indicada en el nombre y esta nota. El cotejo inicial que trata globalmente las viñetas como letras es diagnóstico y no constituye la aceptación final. La comprobación final aplica equivalencias según la fuente real y compara cada página con la referencia conservada.

Los archivos de terceros mantienen su atribución y licencia MIT declaradas. El pie siguiente corresponde a esta nota y al trabajo documental propio, no relicencia la biblioteca ni el PDF.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
