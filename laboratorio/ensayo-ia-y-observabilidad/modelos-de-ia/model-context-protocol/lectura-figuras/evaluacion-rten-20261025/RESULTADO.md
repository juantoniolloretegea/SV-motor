# RTen: valoración del motor de cálculo sin modelos de reconocimiento

25 de octubre de 2026. Versión examinada: RTen 0.27.0. Alcance: cálculo matricial local en CPU y revisión de la configuración utilizada. No se ha ejecutado OCR ni cargado un modelo de IA.

## Distinción y necesidad

RTen es una biblioteca Rust que ejecuta operaciones numéricas. Puede utilizarlas directamente, sin cargar un archivo de modelo. Un modelo entrenado aporta una estructura de operaciones y sus parámetros aprendidos; es un objeto diferente del motor que efectúa los cálculos. El reconocimiento de texto mediante ocrs requiere modelos de detección y reconocimiento. El motor sin esos modelos no convierte una imagen en texto. [Operadores de RTen](https://docs.rs/rten/0.27.0/rten/ops/index.html) y [descripción de ocrs](https://github.com/robertknight/ocrs/blob/f9cf8125e89136f633b2dfd7ea41a37d08add9c9/README.md).

La lectura de las figuras SVG del SV se resuelve con extracción nativa y una fuente estructurada común, sin OCR ni inferencia. Por ello no se incorpora RTen al servicio para esta finalidad. Su posible utilización futura como motor matemático, sin modelos de IA, deberá responder a una necesidad concreta y verificarse en su ámbito. HelsinkiNLP conserva su función de traducción auxiliar, separada de los nodos de inferencia experta; esta valoración no modifica esa función ni incorpora otro modelo.

## Configuración comprobada

Se fijó RTen 0.27.0 con las características predeterminadas desactivadas, junto con rten-tensor 0.27.0. La compilación utilizó Rust 1.98.1 en Linux x86-64; RTen declara un mínimo de Rust 1.94. No se modificó el compilador del servidor del SV. El código, las dependencias exactas y la aplicación de prueba se conservan en este directorio. [Manifiesto de RTen](https://github.com/robertknight/rten/blob/e20e3efbcc4cf941f211cc8d617a149da0c51533/Cargo.toml).

El conjunto de dependencias utilizado no incluye ort, ONNX Runtime, ocrs ni un lector de modelos ONNX. La prueba emplea directamente los operadores matriciales de RTen. La inspección del enlace dinámico muestra las bibliotecas ordinarias del sistema Linux, sin un motor externo de inferencia; no se afirma que todo el sistema operativo o el ejecutable carezcan de componentes ajenos a Rust. La licencia de RTen es MIT o Apache-2.0. Las licencias resueltas se conservan por separado; no se atribuye licencia ni recepción a pesos que no se han descargado.

## Método y resultados

Se construyeron tres pares de matrices con fracciones binarias exactamente representables. Se comparó el producto calculado por RTen en precisión simple con una referencia escalar calculada en precisión doble. El criterio previo de discrepancia absoluta máxima fue 0,0001.

| Dimensiones del producto | Discrepancia absoluta máxima observada |
|---|---:|
| 2 × 3 por 3 × 2 | 0 |
| 17 × 19 por 19 × 13 | 0 |
| 64 × 64 por 64 × 64 | 0 |

El motor rechazó un producto con dimensiones incompatibles. La configuración compiló y superó Clippy sin advertencias en la aplicación de prueba. El resultado completo se conserva en [RESULTADO.json](evidencias/RESULTADO.json).

La ejecución se realizó en un espacio de red aislado, sin rutas de salida. El intento de conexión de comprobación fue rechazado por ausencia de red. El programa Rust impuso límites de 512 MiB de memoria virtual, 10 segundos de CPU, 1 MiB por archivo de salida y ausencia de volcado de memoria. La supervisión externa limitó la duración total a 30 segundos. La ejecución finalizó correctamente; el máximo de memoria residente observado fue 7.620 KiB. Los tiempos particulares constan en la evidencia, pero corresponden a una ejecución de desarrollo y no constituyen un estudio de rendimiento ni una estimación del coste del OCR.

RTen no limita por sí mismo la memoria ni la duración de un modelo. La restricción debe imponerse al proceso. La prueba acredita el aislamiento de red y las cotas aplicadas a este cálculo; no acredita un aislamiento completo del sistema de archivos ni una revisión exhaustiva del código de terceros. [Límites declarados por RTen](https://github.com/robertknight/rten/blob/e20e3efbcc4cf941f211cc8d617a149da0c51533/docs/security.md).

## Conclusión y límites de recepción

La valoración confirma una vía material de cálculo matricial en Rust sin ONNX Runtime y sin cargar un modelo de IA. RTen queda como alternativa de cálculo documentada, sin integración en producción. No sustituye al lector nativo que ya resuelve el objetivo documental actual.

En este ensayo numérico no se han evaluado la carga de modelos, la precisión de reconocimiento, el español científico, las fórmulas, los pesos, la ejecución en Windows o WebAssembly, la GPU ni la NPU. Tampoco se han acreditado todos los operadores ni un rendimiento representativo. Estas exclusiones no son defectos de la lectura nativa ni impiden su utilización recibida. La [evaluación posterior de OCR auxiliar](../evaluacion-ocr-local-20261025/RESULTADO.md), realizada con otra configuración identificada, conserva por separado sus mediciones y su resultado no favorable de fidelidad. Esta distinción evita atribuir a las operaciones matriciales una recepción del reconocimiento.

| Omisión examinada | Consecuencia posible | Control aplicado | Límite conservado |
|---|---|---|---|
| Confundir motor y modelo | Incorporación injustificada de inferencia | Operadores directos, sin archivos de modelos | No acredita reconocimiento |
| Identificar una interfaz Rust con cálculo Rust | Dependencia no autorizada de otro motor | Revisión de dependencias y enlace dinámico | No es auditoría exhaustiva de terceros |
| Omitir límites de recursos | Agotamiento de capacidad | Cotas de proceso y duración | No autoriza modelos arbitrarios |
| Extrapolar tres operaciones a OCR | Atribución de precisión inexistente | Resultados y exclusiones separados | No hay recepción de OCR |

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
