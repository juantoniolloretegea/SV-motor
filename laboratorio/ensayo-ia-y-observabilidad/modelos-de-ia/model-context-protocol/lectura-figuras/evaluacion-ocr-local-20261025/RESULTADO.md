# OCR local como herramienta auxiliar: viabilidad y fidelidad

25 de octubre de 2026. Evaluación delimitada de ocrs 0.13.1 con RTen 0.26.0, sin ONNX Runtime. La configuración examinada no queda admitida para lectura científica ni integrada en el Gestor.

## Función y alcance

Una herramienta OCR auxiliar recupera caracteres de una imagen. Aunque su reconocimiento utilice modelos de IA, su función permanece separada de los nodos 1, 2 y 3 y no le concede capacidad de consejo experto, modificación del SV, admisión de conocimiento ni acceso al núcleo. La traducción auxiliar y el reconocimiento óptico son tratamientos distintos, con modelos, licencias, entradas, resultados y límites propios.

Las fuentes estructuradas y la extracción nativa conservan prioridad cuando bastan. Las seis figuras SVG recibidas disponen de texto nativo, por lo que no necesitan este reconocimiento. El estudio examina una alternativa para documentos rasterizados sin fuente textual suficiente; no demuestra una necesidad de aplicarla al conjunto de la biblioteca.

## Configuración identificada

Se utilizó la versión publicada de ocrs 0.13.1, correspondiente a la revisión `e5f1c5205326804637368422fe420aa1b9769676`. Esta publicación depende de RTen 0.26.0. La revisión posterior de desarrollo que conserva el mismo número de versión declara RTen 0.27.0; no son configuraciones intercambiables. El [ensayo numérico del motor](../evaluacion-rten-20261025/RESULTADO.md) corresponde a RTen 0.27.0 y se conserva por separado.

Los dos archivos de modelos se recibieron en formato RTen desde la revisión `df0edd170279ab971b53e094c627255a87e1a503` del repositorio de modelos de ocrs. Ocupan conjuntamente 12.240.008 bytes, aproximadamente 11,67 MiB. Se cotejaron sus huellas SHA-256 con las declaradas por el repositorio. No se utilizó ONNX Runtime, Python, un servicio externo de OCR ni una descarga durante el reconocimiento. [Archivos y licencia declarada de los modelos](https://huggingface.co/robertknight/ocrs/tree/df0edd170279ab971b53e094c627255a87e1a503).

El código de ocrs y RTen declara licencia MIT o Apache-2.0; los pesos examinados declaran CC BY-SA 4.0. Las licencias se conservan diferenciadas. Los pesos de terceros no se publican bajo la licencia de la documentación del SV ni se redistribuyen con este informe.

## Método y recursos observados

La prueba se ejecutó localmente en CPU, en un equipo con procesador Intel Core i7-1165G7, cuatro núcleos y 16 GB nominales de memoria. Utilizó Linux mediante WSL y Rust 1.98.1. No se utilizó GPU ni equipo adicional.

La muestra controlada contiene seis líneas de texto nítido: palabras sin diacríticos, español con tildes y eñe, letras griegas, subíndices y una desigualdad. Se conserva la [imagen](evidencias/muestra.jpg) y su [transcripción de referencia](evidencias/referencia.txt). Se realizaron dos reconocimientos consecutivos con una única carga de modelos. La prueba no es un corpus representativo de documentos científicos ni un estudio de rendimiento general.

| Medida de la ejecución instrumentada | Resultado observado |
|---|---:|
| Carga conjunta de los modelos | 60 ms |
| Primer reconocimiento | 3.268 ms |
| Segundo reconocimiento, sin recargar modelos | 3.311 ms |
| Máximo de memoria residente del proceso OCR | 170.540 KiB, aproximadamente 166,54 MiB |
| Memoria residente al terminar | 65.992 KiB |
| Tareas de reconocimiento simultáneas | 1 |
| Rutas de salida de red visibles | 0 |

Se fijó un trabajador de cálculo en cada grupo de ejecución; el proceso informó de dos hilos totales. Los tiempos corresponden a esta muestra y configuración. La reutilización evitó una segunda carga de modelos, pero no redujo significativamente el tiempo de reconocimiento en esta observación. No debe atribuirse una aceleración inexistente.

El proceso se ejecutó con prioridad reducida, aislado de la red y del sistema de archivos de la aplicación. Sólo recibió los archivos locales de prueba, de sólo lectura, y un directorio temporal separado. La comprobación interna confirmó ausencia de rutas de red y de los directorios del equipo anfitrión. Los límites fueron 4 MiB de entrada, dos millones de píxeles, 768 MiB de memoria virtual, 20 segundos de CPU y 30 segundos de duración total. La aplicación de prueba y sus límites están escritos en Rust; la preparación del entorno de ensayo no constituye un componente incorporado al SV.

La medición externa del proceso supervisor no representa la memoria del OCR; el dato comunicado procede del propio proceso de reconocimiento. Se conservan [resultados y comprobaciones](evidencias/RESULTADO.json). Hubo un rechazo preliminar de una captura JPEG identificada inicialmente como PNG, antes de cargar los modelos; se corrigió la identificación mediante la firma del archivo. La versión final compiló y superó Clippy sin advertencias.

## Fidelidad: configuración no recibida

El reconocimiento presentó pérdidas, sustituciones y detecciones espurias. Entre los ejemplos observados aparecen «evaluaci?n» y «se?al», y no se conservaron correctamente las fórmulas ni varias líneas ordinarias. La salida completa permite examinar estos defectos sin sustituirlos por una valoración favorable del consumo.

El alfabeto predeterminado del reconocedor no contiene los diacríticos españoles ni las letras griegas requeridas. Añadir caracteres a una lista no acredita que sus pesos hayan aprendido a reconocerlos. La configuración no cumple el requisito de fidelidad para estos documentos. Esta conclusión se refiere a los archivos y versiones examinados; no declara imposible cualquier OCR futuro realizado en Rust. [Alfabeto de la versión publicada](https://github.com/robertknight/ocrs/blob/e5f1c5205326804637368422fe420aa1b9769676/ocrs/src/lib.rs).

## Condiciones de una posible integración posterior

La ejecución observada resulta compatible con un estudio local de recursos modestos. No acredita todavía ausencia de ralentización del Gestor: se requiere un ensayo de coexistencia con carga representativa y supervisión del tiempo de respuesta antes de habilitarla.

Una integración deberá conservar un proceso separado, prioridad reducida, una tarea activa, una cola pequeña y acotada, cancelación y caducidad. Su selección corresponderá a una política Rust que justifique la insuficiencia de la lectura nativa. Las decisiones de capacidad, permisos y conservación no se delegarán al modelo ni al navegador. La interfaz informará de espera, reconocimiento, rechazo, cancelación y revisión pendiente, con sus causas comprensibles.

El resultado será un derivado identificable junto al original, nunca una sustitución silenciosa. Los datos privados no se distribuirán por CDN ni se enviarán a servicios externos. El texto reconocido no podrá convertirse en una orden ni alterar el núcleo, las autorizaciones o las rutas críticas. La precisión, especialmente de expresiones científicas, deberá recibirse antes de admitir la herramienta para ese contenido.

| Omisión examinada | Consecuencia posible | Control y evidencia | Estado |
|---|---|---|---|
| Alfabeto y precisión adecuados | Alteración de términos o fórmulas | Muestra y salida conservadas; contraste explícito | No recibido |
| Límites de memoria, tiempo y concurrencia | Competencia por recursos del Gestor | Cotas del proceso y una tarea activa | Ensayo local, sin integración |
| Aislamiento y prohibición de instrucciones | Acceso ajeno al tratamiento documental | Red y archivos separados; salida tratada como datos | Comprobación delimitada |
| Evaluación de coexistencia | Ralentización no detectada | Ensayo representativo exigido antes de habilitar | Pendiente; no se presume conforme |

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
