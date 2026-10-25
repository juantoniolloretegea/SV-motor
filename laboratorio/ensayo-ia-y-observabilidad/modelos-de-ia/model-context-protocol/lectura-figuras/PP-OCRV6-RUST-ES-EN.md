# PP-OCRv6: reconocimiento local en Rust para español e inglés

Evaluación documental de 25 de octubre de 2026. Esta revisión amplía el [estudio de OCR local](ESTUDIO-OCR-LOCAL.md). No acredita ejecución de PP-OCRv6 ni lo incorpora al servicio. La lectura textual ya publicada de las figuras procede de sus fuentes estructuradas y no utiliza modelos de reconocimiento.

## Procedencia, mantenimiento y licencia

PP-OCRv6 pertenece al proyecto oficial [PaddlePaddle/PaddleOCR](https://github.com/PaddlePaddle/PaddleOCR). El informe científico identifica al equipo PaddlePaddle de Baidu como responsable de su desarrollo. El repositorio consultado no está archivado y declara licencia Apache-2.0. Las implementaciones independientes en Rust no constituyen distribuciones oficiales de ese equipo.

La [colección oficial de modelos](https://huggingface.co/collections/PaddlePaddle/pp-ocrv6) publica detectores y reconocedores en varios formatos. Un detector localiza regiones con texto; un reconocedor convierte las regiones en caracteres. No son agentes de consejo experto, ni pertenecen por ese motivo a los nodos de inferencia del SV. Su eventual función sería una herramienta auxiliar de lectura, aislada y sujeta a recepción específica.

El código oficial utiliza principalmente Python y C++, por lo que su instalación habitual no satisface la condición del SV. Los pesos de un modelo son datos distintos del programa que los ejecuta. Su utilización mediante Rust exige verificar todos los operadores, la preparación de las imágenes, la reconstrucción del texto y el diccionario correspondiente. El formato ONNX no obliga conceptualmente a utilizar ONNX Runtime; tampoco demuestra que cualquier motor Rust pueda ejecutar un archivo concreto.

## Español, inglés y variantes

El apéndice A del informe oficial incluye español e inglés en Tiny, Small y Medium. Presenta resultados agregados para inglés y escritura latina, sin desglosar la exactitud exclusiva del español en esa tabla:

| Variante | Inglés, tabla 11 | Escritura latina, tabla 11 | Tamaño publicado del reconocedor |
|---|---:|---:|---:|
| Tiny | 77,3 % | 63,8 % | 4,4 MB |
| Small | 86,3 % | 84,0 % | 20,4 MB |
| Medium | 88,4 % | 88,0 % | 73,3 MB |

Son medidas del conjunto de evaluación de sus autores, no resultados del SV. Los tamaños corresponden al reconocedor de la documentación consultada: no incluyen detector, memoria de ejecución ni otros archivos. Las cifras de inglés impreso de otra tabla del mismo artículo utilizan una selección distinta y no deben intercambiarse con éstas. [Informe científico, tablas 6 y 11](https://arxiv.org/html/2606.13108v1), [documentación oficial de reconocimiento](https://github.com/PaddlePaddle/PaddleOCR/blob/dab3fe35379033fdcb2d0e9572fac0b36c9a9ebf/docs/version3.x/module_usage/text_recognition.en.md).

En el [diccionario `ppocrv6_dict.txt` examinado](https://github.com/PaddlePaddle/PaddleOCR/blob/dab3fe35379033fdcb2d0e9572fac0b36c9a9ebf/ppocr/utils/dict/ppocrv6_dict.txt) se identifican 18.708 entradas. Contiene ñ, Ñ, las vocales acentuadas, ü, Ü y «¿»; no se ha encontrado «¡» como entrada independiente. Esta observación pertenece a ese archivo: no demuestra la salida de todos los modelos distribuidos. Antes de un ensayo debe cotejarse con el diccionario efectivamente asociado a los pesos y al descodificador. No es correcto añadir un carácter al diccionario sin comprobar la correspondencia con las clases de salida del modelo.

La selección provisional para un ensayo de español e inglés es **Small**, por su relación documental entre exactitud y tamaño. Tiny puede servir como comparación de consumo; Medium permitiría estudiar si una mejora de fidelidad justifica más recursos. No existe todavía evidencia local que permita declarar una variante superior a los modelos anteriormente ensayados. El reconocimiento de texto tampoco acredita interpretación de fórmulas, relaciones gráficas o estructura científica.

## Alternativas de ejecución en Rust

| Implementación y revisión | Evidencia examinada | Dictamen de esta revisión |
|---|---|---|
| PaddleOCR oficial, `dab3fe35379033fdcb2d0e9572fac0b36c9a9ebf` | Código, documentación y modelos de referencia; instalación habitual Python y C++ | Fuente de modelos y especificaciones. No se instala su entorno de ejecución en el SV |
| RustO 0.3.2, `11a6c2947232cef2fef0c13392c065a0ac0d0f0a` | RTen 0.27; lectura ONNX y RTen; geometría mediante `clipper2-rust`; OpenCV opcional | Candidato preferente para una comprobación aislada, desactivando características no necesarias. No compilado ni recibido en esta revisión |
| `ppocr-server` 0.1.0, `e99a4522186e39dcd3d355c73e2921410e84f938` | Motor tract; servidor HTTP; obtención de pesos durante la preparación; paralelismo Rayon | Referencia alternativa. Su configuración de servicio y descarga automática no es admisible sin adaptación a la política local |
| oar-ocr 0.10.1, configuración previamente examinada | Delega la inferencia mediante `ort` a ONNX Runtime | Conserva su exclusión de incorporación en esa configuración |

La revisión actual de RustO cambia una conclusión preliminar: su manifiesto ya utiliza `clipper2-rust`, y el tratamiento posterior de las regiones importa esa implementación. No debe atribuírsele la dependencia obligatoria `geo-clipper` de una revisión anterior. `geo-clipper` sí enlaza con Clipper C++, pero esa constatación no describe el manifiesto actual. Las opciones predeterminadas de RustO incluyen interfaces para otros lenguajes; la candidatura del SV requiere la biblioteca Rust con las características estrictamente necesarias. Un archivo de dependencias que incluya componentes opcionales no demuestra que todos se incorporen al ejecutable, y tampoco sustituye la comprobación de la selección efectiva. [Manifiesto RustO](https://github.com/byrizki/rusto-rs/blob/11a6c2947232cef2fef0c13392c065a0ac0d0f0a/Cargo.toml), [tratamiento geométrico](https://github.com/byrizki/rusto-rs/blob/11a6c2947232cef2fef0c13392c065a0ac0d0f0a/src/postprocess.rs), [alcance de geo-clipper](https://docs.rs/geo-clipper/0.9.0/geo_clipper/).

La alternativa con tract se presenta como servidor y escucha de forma predeterminada en todas las interfaces de red. Su configuración habilita tres tamaños de modelos y una preparación anticipada de cálculos. Dispone además de una opción independiente de comprensión mediante otro modelo, desactivada por defecto. Ninguna de estas opciones debe confundirse con un componente ya autorizado del SV. Para el objetivo actual resulta preferible evaluar una biblioteca local acotada antes que incorporar ese servidor. [Manifiesto](https://github.com/drmhse/paddle-paddle-v6-ocr-rust/blob/e99a4522186e39dcd3d355c73e2921410e84f938/Cargo.toml), [arranque](https://github.com/drmhse/paddle-paddle-v6-ocr-rust/blob/e99a4522186e39dcd3d355c73e2921410e84f938/src/main.rs).

## Qué se ha ejecutado y qué permanece pendiente

El [ensayo anterior de ocrs](evaluacion-ocr-local-20261025/RESULTADO.md) utilizó un detector y un reconocedor locales publicados por el proyecto ocrs, con RTen 0.26.0. Acreditó una ejecución acotada y sin servicio externo, pero produjo errores que impiden admitir esa configuración para lectura científica. Sus medidas no se transfieren a PP-OCRv6.

En esta ampliación se han examinado fuentes, versiones, dependencias declaradas y un diccionario. **No se han descargado ni ejecutado pesos PP-OCRv6.** No se ha instalado Python, ONNX Runtime, un servidor OCR ni un modelo adicional de comprensión. La comprobación de RTen 0.27 realizada con operaciones numéricas tampoco demuestra compatibilidad con estos modelos completos.

Las mediciones publicadas del fabricante utilizan otros equipos y motores. No permiten anticipar la latencia ni el consumo de memoria de RustO o tract en esta instalación. El tamaño de los pesos no equivale a memoria máxima, y una ejecución aislada satisfactoria no acredita ausencia de interferencia con el Gestor.

## Condiciones de una evaluación local posterior

El ensayo debe identificar pesos y diccionarios de procedencia oficial, sus licencias y huellas; resolver y revisar las dependencias Rust efectivamente seleccionadas; y utilizar un proceso sin comunicaciones externas, con acceso mínimo a archivos. La descarga de recursos no se realizará durante una consulta de usuario.

La muestra de referencia incluirá español e inglés, tildes, ñ, signos de apertura, texto científico, símbolos, fórmulas, columnas y figuras sin texto. Se medirán errores, omisiones, orden y localización. Las relaciones gráficas y las fórmulas exigirán un cotejo distinto del reconocimiento de palabras. El original permanecerá disponible y toda lectura incierta se declarará.

El primer ensayo conservará límites explícitos de memoria, tiempo, dimensiones y concurrencia, con un único trabajo auxiliar. Después deberá comprobarse la coexistencia con las funciones ordinarias del Gestor. Una superación de límites dará lugar a una interrupción informada, sin ampliar recursos automáticamente. Sólo una recepción favorable por capacidad permitirá considerar la integración; nunca se otorgará al OCR acceso al núcleo ni autoridad sobre sus reglas.

| Omisión examinada | Consecuencia posible | Control y límite de recepción |
|---|---|---|
| No verificar el diccionario y los caracteres de español | Pérdida o sustitución de signos con alteración del significado | Identidad del diccionario y cotejo de una transcripción de referencia; cobertura declarada no equivale a fidelidad |
| Confundir rótulos con descripción completa | Exclusión de relaciones necesarias para una persona que no ve la imagen | Explicación equivalente y comprobación con tecnologías de asistencia; la transcripción sola no acredita WCAG |
| Omitir dependencias o descargas implícitas | Incorporación de ejecución no autorizada o comunicación externa | Selección efectiva de dependencias, recursos locales identificados y aislamiento sin red |
| Inferir capacidad local a partir de medidas del fabricante | Saturación o ralentización del servicio | Medición local acotada y posterior comprobación de coexistencia; integración pendiente |

## Publicaciones y referencias

- Equipo PaddlePaddle, Baidu. *PP-OCRv6: From 1.5M to 34.5M Parameters, Surpassing Billion-Scale VLMs on OCR Tasks*. Informe arXiv:2606.13108, versión 1, 11 de junio de 2026. [Texto completo](https://arxiv.org/html/2606.13108v1). Su depósito no se presenta como acreditación de revisión por pares.
- [Colección oficial y formatos de modelos](https://huggingface.co/collections/PaddlePaddle/pp-ocrv6).
- [Catálogo de lectura del SV](../CATALOGO-DE-LECTURA.md): prioridad de fuentes estructuradas, texto nativo y OCR local.
- [Declaración de accesibilidad](https://sv.itvia.online/accesibilidad): alcance de la lectura textual, tecnologías de asistencia y privacidad.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
