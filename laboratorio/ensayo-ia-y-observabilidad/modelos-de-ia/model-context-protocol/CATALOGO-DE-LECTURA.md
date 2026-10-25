# Lectura de documentos y figuras: selección, fidelidad y aislamiento

La elección de una capacidad de lectura debe conservar la identidad de la fuente, declarar las transformaciones realizadas y justificar sus límites. El menor consumo de recursos sólo es un criterio de selección entre procedimientos que satisfacen la fidelidad exigida. Una extracción parcial no constituye lectura íntegra.

## Capacidades y estado de comprobación

| Capacidad | Aplicación | Estado y referencia |
|---|---|---|
| Fuente estructurada de la figura | Texto, etiquetas y relaciones conservados durante su elaboración | Opción preferente cuando existe una fuente fiel; no exige reconocimiento óptico |
| Texto documental PDF | Recuperación de texto ya contenido en el documento | [Preparación Rust y límites de lectura PDF](LECTURA-PDF.md); conserva su recepción específica |
| Texto nativo SVG | Recuperación de etiquetas y descripciones del archivo vectorial | [Extractor Rust 0.1.0](lectura-figuras/0.1.0/CONTRATO.md), compilado y comprobado localmente; no añade una herramienta al servidor MCP instalado |
| Reconocimiento óptico local | Imagen sin texto nativo suficiente ni fuente estructurada disponible | [Estudio de candidatos](lectura-figuras/ESTUDIO-OCR-LOCAL.md); ningún motor OCR queda instalado o admitido por este estudio |
| Presentación interactiva | Exploración de una figura y sus datos de procedencia | Requiere datos explícitos y relaciones conocidas; un visor no acredita reconocimiento ni interpretación científica |

## Selección regulada por Rust

El procedimiento comprueba primero la disponibilidad de una fuente estructurada fiel, después la extracción nativa aplicable y, cuando éstas resultan insuficientes, la disponibilidad de OCR local recibido para el uso requerido. La realización de esta prioridad se conserva en `politica.rs`. Las condiciones de disponibilidad proceden de comprobaciones competentes del sistema; no puede establecerlas libremente un modelo de IA.

El OCR permanece contemplado. Su uso no debe convertirse en una obligación para toda imagen ni descartarse sólo por consumir más recursos. Si no existe una vía recibida, se declara la lectura no disponible o parcial y se explica qué comprobación falta. No se completa el contenido mediante inferencias silenciosas.

## Aislamiento y custodia

No se admite ningún servicio externo de OCR. Las imágenes, sus derivados y las preguntas asociadas no se envían a terceros para este tratamiento. Los motores y modelos locales, cuando se reciban, tendrán versiones, licencias, huellas de identidad y límites de ejecución explícitos. Las descargas automáticas durante una consulta no forman parte del procedimiento permitido.

El tratamiento requiere separación de procesos, ausencia de acceso al núcleo, denegación de comunicaciones externas y acceso mínimo a archivos. Un documento o el texto reconocido constituyen datos no fiables, nunca instrucciones ejecutables. La validación de entrada debe acotar tamaño, dimensiones, número de páginas, memoria, tiempo y concurrencia; la denegación se comunica sin ocultar su causa.

El original se conserva. Cada derivado identifica fuente, método, versión y alcance. La reutilización por identidad documental evita repetir trabajo, pero no autoriza incluir datos privados en la distribución pública ni en su almacenamiento compartido. El reconocimiento, la traducción y la revisión científica se registran como operaciones distintas.

## Figuras propias y lectura científica

Las figuras elaboradas para el SV deben conservar sus textos, leyendas, relaciones y fuentes junto al original gráfico. Cuando existen etiquetas SVG, puede ofrecerse una lectura textual accesible y traducible sin modificar la imagen. El orden de los elementos en el archivo no demuestra, por sí solo, su orden visual ni el significado de las flechas.

La representación mediante egui u otro visor sólo será interactiva respecto de los datos realmente suministrados. No corresponde atribuir al visor una interpretación automática de fórmulas, diagramas o polígonos. La corrección de símbolos, relaciones y conclusiones exige una recepción específica.

## Referencias relacionadas

- [Ficha técnica del servicio documental](FICHA_TECNICA.md).
- [Código limpio y auditable](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/continuacion-15-09-2026/CODIGO-LIMPIO-Y-AUDITABLE.md).
- [Consecuencias de la omisión: fuentes y delimitación](../estandar-comparativo-o-benchmark/CONSECUENCIAS-Y-FUENTES.md).
- [Reglas, usos y descripciones documentales](https://documentos-sv.itvia.online/documentacion/reglas-usos-descripciones-documentales/).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
