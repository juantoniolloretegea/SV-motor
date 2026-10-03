# Lectura documental de PDF mediante MCP · Preparación 0.1.4-pdf.1

Estado: implementación y comprobación instrumental local; no instalada en el servidor ni utilizada por un modelo. La recepción científica y una eventual admisión al ensayo siguen siendo etapas distintas. Esta preparación conserva el MCP 0.1.3 y no modifica el Núcleo, su semántica, su representación intermedia, los pesos ni las fuentes de una campaña en curso.

## Objetivo y relación con las fuentes

Se incorpora una capacidad de extracción de texto PDF al mismo servicio documental de las pruebas. El original y las vistas HTML se conservan en la [colección de tricoleucemia](https://github.com/juantoniolloretegea/SVperitus-dataset/tree/dominio-inmunologia/dominios/inmunologia/literatura-tricoleucemia). El [original PDF fijado](https://raw.githubusercontent.com/juantoniolloretegea/SVperitus-dataset/488d597fa1999a1fc2eb6538fd610a36046f6f01/dominios/inmunologia/literatura-tricoleucemia/lls-hoja-informativa-2018/original/hairy-cell-leukemia.pdf) contiene diez páginas, 157315 bytes y SHA-256 `21322ed388c999bd0713ba5ec5c7ab34402d4bb4cd6100903aa5ea6d8bf35d9c`. Sustituye como vía de acceso al enlace externo indisponible; no es una edición nueva.

La capacidad está implementada en Rust, no consiste únicamente en instrucciones escritas para el modelo. El preparador lee un archivo local de identidad fijada y produce catálogo, extracción, lista de fuentes y constancia. El MCP consulta esa caché mediante las dos herramientas existentes, `buscar_documentos` y `leer_documento`. No se añade una vía de descarga libre, de ejecución de órdenes o de selección de rutas por el modelo. La lista de fuentes es un dato del operador cuya huella se fija al inicio y se coteja en el diario; no es una autorización emitida por el modelo.

## Numeración y alcance de lectura

Cada página física del PDF ocupa una sección `PDF-P0000` a `PDF-P0009`. Su índice comienza en 0 y su ordinal físico en 1. El argumento `pagina` del MCP elige un fragmento **dentro de esa sección**, también desde 0. La respuesta conserva ambos localizadores y las huellas. `siguiente_pagina: null` sólo indica el final de los fragmentos de esa sección; no demuestra lectura de los anteriores ni del resto del PDF. El contrato enviado por `tools/list` declara expresamente esta diferencia.

## Comprobaciones y resultados

El lector supera siete comprobaciones Rust en Windows. El MCP se compila y comprueba en Linux x86_64 con Rust 1.93.1. Se conservan las comprobaciones anteriores de protocolo y auditoría, y se añaden cuatro de integración PDF. El resultado detallado y el número de pruebas aparecen en `evidencias/PRUEBAS-MCP.txt`.

El recorrido instrumental entrega **30 fragmentos de las diez páginas**; reconstruye exactamente cada texto del catálogo; conserva solicitudes, respuestas y diario; reproduce el diario y rechaza una lista de fuentes de identidad distinta. Esta consulta exhaustiva pertenece exclusivamente a la prueba del instrumento: no completa lecturas de ningún candidato y no representa una inferencia. La prueba de aislamiento del proceso MCP comprueba el rechazo de sockets locales y externos.

El cotejo de extracción compara, página por página, la frecuencia de caracteres tras normalización Unicode NFKC y exclusión de espacios, frente a la referencia de la conversión HTML previamente conservada. Coincide para las diez páginas y detecta las ocho pérdidas del extractor original. Este criterio demuestra cobertura de caracteres, **no identidad del orden de lectura ni equivalencia semántica**. En cambio, la reconstrucción del transporte sí exige igualdad exacta de texto, incluido su orden, respecto al catálogo extraído.

La [selección de bibliotecas](ESTUDIO-BIBLIOTECAS.md) y la [corrección delimitada](correccion/NOTAS.md) documentan versiones, mantenimiento, alternativas y defectos observados. Se conservan originales diagnósticos; la corrección no se atribuye a los mantenedores externos.

## Guardas y límites

El preparador exige huella previa, PDF de hasta 20 MiB, hasta 64 páginas físicas, descompresión por contenido de hasta 64 MiB, texto total de hasta 1 MiB y catálogo de hasta 2 MiB. Su proceso Linux limita CPU a 30 segundos y espacio de direcciones a 512 MiB y bloquea la creación de sockets. La preparación sólo admite una carpeta nueva. Rechaza documentos cifrados, fallos de extracción, símbolos no descodificados y páginas sin texto; no presenta un resultado parcial como completo. Una terminación forzada por el sistema podría dejar archivos incompletos: sólo una carpeta con PREPARACION.json y huellas cotejadas puede considerarse preparada.

No incorpora OCR ni interpretación visual. Los documentos escaneados requieren una etapa posterior distinta; tablas, columnas, ecuaciones, gráficos y orden visual complejo necesitan contraste específico. El orden conservado procede de los operadores PDF con separación espacial de palabras y líneas, y no se garantiza universalmente. El PDF original y sus reproducciones siguen disponibles para la revisión. Las cotas no equivalen a aislamiento exhaustivo de todo el sistema ni a una auditoría completa de dependencias.

## Reproducción y estructura

`lector/` contiene adaptador, pruebas, referencias y variante de biblioteca. `mcp-documental-0.1.4-pdf/` conserva el servicio anterior con extensión delimitada, preparador y cuatro nuevas pruebas. `evidencias/recorrido-pdf/` contiene catálogo, lista fijada, texto, solicitudes, respuestas y diario. `correccion/` permite examinar la modificación de terceros. Los archivos Cargo.lock fijan las dependencias.

En Linux x86_64, desde esta carpeta, ejecutar `cargo test --locked --manifest-path lector/Cargo.toml --test extraccion` y `cargo test --locked --manifest-path mcp-documental-0.1.4-pdf/Cargo.toml`. La segunda orden realiza el recorrido integral local sin modelo. El programa de preparación es `preparar-pdf PDF SHA256 IDENTIFICADOR URL_PROCEDENCIA CARPETA_NUEVA`. El servicio recibe `CATALOGO SHA256 DIARIO 256 --fuentes-autorizadas FUENTES SHA256`; el verificador recibe `CATALOGO SHA256 DIARIO --oficial --fuentes-autorizadas FUENTES SHA256`. Cada SHA256 corresponde al archivo que lo precede. Las herramientas diagnósticas del lector no sustituyen el preparador con límites.

Los registros publicables sustituyen únicamente la ruta local absoluta por un marcador; sus constancias conservan la huella anterior y posterior. Los archivos originales permanecen bajo custodia local. No se publica un ejecutable para sustituir automáticamente el servicio activo. Una incorporación efectiva exige fijar esta revisión, comprobarla en su destino y aplicar las condiciones del encargo correspondiente.

Los componentes de terceros mantienen sus licencias; el PDF y su texto conservan la atribución de LLS. La licencia siguiente corresponde a la documentación y trabajo propios, sin extenderse a esos materiales.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
