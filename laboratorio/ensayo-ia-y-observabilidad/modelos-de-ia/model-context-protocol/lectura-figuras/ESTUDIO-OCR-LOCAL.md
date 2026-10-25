# Reconocimiento óptico local: evaluación documental de candidatos

Revisión: 25 de octubre de 2026. Esta evaluación no constituye una prueba de precisión, velocidad, consumo o aislamiento del motor. Se relaciona con el [catálogo de lectura](../CATALOGO-DE-LECTURA.md).

## oar-ocr 0.10.1

Se ha examinado la versión publicada 0.10.1, identificada por la revisión `9c45037f9e84cc936d8f91c8f41af1d1123e9878`. Su interfaz está realizada en Rust y emplea modelos ONNX para reconocimiento de texto y análisis documental. Contempla detección de regiones, orientación, tablas y fórmulas, con componentes seleccionables. Esta amplitud justifica considerarlo candidato para documentos científicos; no demuestra precisión suficiente en los documentos del SV. [Documentación de la biblioteca](https://docs.rs/oar-ocr/0.10.1/oar_ocr/).

El manifiesto declara Rust 1.95 como mínimo, licencia Apache-2.0 y la dependencia `ort = 2.0.0-rc.13`. La compilación actual del servidor del SV emplea Rust 1.93.1: no debe suponerse compatibilidad directa ni modificar su entorno para incorporar el candidato. La evaluación deberá disponer de un entorno compatible separado. [Manifiesto de la versión](https://github.com/GreatV/oar-ocr/blob/9c45037f9e84cc936d8f91c8f41af1d1123e9878/Cargo.toml).

`ort` es una interfaz Rust de ONNX Runtime. El cálculo de la vía ONNX depende, por tanto, de un motor nativo externo a Rust; no debe describirse el conjunto como ejecución íntegramente realizada en Rust. Esta dependencia exige evaluación diferenciada de su código, distribución, licencia y frontera de aislamiento antes de cualquier incorporación. No se instala ni se autoriza implícitamente mediante este documento. [Interfaz ort](https://docs.rs/ort/2.0.0-rc.13/ort/) y [proyecto ONNX Runtime](https://github.com/microsoft/onnxruntime).

Hay dos mecanismos diferentes de descarga: `download-binaries`, activado por defecto, obtiene ONNX Runtime durante la construcción; `auto-download`, opcional en la biblioteca, obtiene modelos durante el uso. Para el SV se estudiarían recursos previamente verificados, carga por rutas locales y ambas descargas desactivadas. La ausencia de esas opciones no sustituye una denegación material de red ni prueba ausencia de otras comunicaciones. [Configuración de capacidades](https://github.com/GreatV/oar-ocr/blob/9c45037f9e84cc936d8f91c8f41af1d1123e9878/docs/features.md).

El catálogo incluye modelos de escritura latina y griega, así como reconocimiento de fórmulas. Como referencia de almacenamiento, anuncia 4,6 MiB para el detector móvil PP-OCRv5 y 7,7 MiB para el reconocedor latino PP-OCRv5. Los modelos de fórmulas son considerablemente mayores. Estas cifras corresponden a archivos de modelos: no son medidas de memoria durante el cálculo ni garantías de latencia. El español científico, sus diacríticos y las expresiones matemáticas requieren pruebas propias. [Modelos documentados](https://github.com/GreatV/oar-ocr/blob/9c45037f9e84cc936d8f91c8f41af1d1123e9878/docs/models.md).

## Alternativa ocrs

`ocrs` ofrece una biblioteca Rust de OCR con ejecución mediante RTen. Su documentación declara estado de desarrollo temprano y cobertura de escritura latina. Es una alternativa pertinente cuando se prioriza evitar la dependencia de ONNX Runtime, pero no se le atribuyen las mismas capacidades de tablas o fórmulas. La licencia del código y la de los pesos deben comprobarse por separado. [Proyecto ocrs](https://github.com/robertknight/ocrs), [modelos](https://github.com/robertknight/ocrs-models) y [licencia de los pesos publicados](https://huggingface.co/robertknight/ocrs).

## Criterios para una prueba posterior

1. Fijar código, dependencias, modelos, diccionarios y licencias; conservar sus huellas de identidad.
2. Usar un conjunto de documentos propios con transcripción de referencia, sin datos personales innecesarios. Incluir español, símbolos griegos, subíndices, superíndices, tablas, columnas, fórmulas y figuras sin texto.
3. Medir errores de caracteres y palabras, omisiones, orden de lectura y correspondencia entre regiones. Las fórmulas requieren cotejo independiente; una cadena LaTeX válida no acredita igualdad matemática.
4. Medir inicio en frío, reutilización del modelo, memoria máxima, almacenamiento y tiempo por documento. Comparar con extracción nativa cuando ésta existe; no identificar ausencia de tarifa externa con ausencia de coste computacional.
5. Comprobar el proceso sin red, con permisos mínimos y límites de entrada y recursos. Incluir imágenes dañadas, dimensiones excesivas, contenido engañoso y texto que pretenda impartir instrucciones.
6. Declarar por separado cada capacidad recibida. Una lectura OCR no constituye admisión de conocimiento ni habilita acceso al núcleo, consultas a proveedores o rutas críticas.

## Conclusión de esta revisión

oar-ocr es un candidato relevante por su cobertura documental y selección de componentes. Permanece en estudio: no se ha compilado, instalado ni ejecutado en esta evaluación, no se han descargado sus modelos y no se ha transmitido ninguna imagen. Su posible incorporación depende de la recepción de la dependencia nativa, el entorno compatible, las licencias de los modelos y las pruebas anteriores. El tratamiento exclusivamente local es obligatorio en todas las alternativas.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
