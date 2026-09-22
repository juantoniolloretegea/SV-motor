# Resultado de la consulta documental acotada · DOC-01

**Fecha:** 22 de septiembre de 2026.  
**Versión del ensayo:** 1.0.  
**Aplicación examinada:** EIO conversación 0.1.3.  
**Vinculación:** S39 y TT-0008.

## Resultado

Se completaron las cuatro peticiones previstas, sin repetir casos. Todas declararon finalización normal, con retorno cero y fin del canal confirmado. Ninguna cumplió íntegramente el contrato de salida fijado antes de ejecutar. Este resultado distingue la viabilidad técnica de la inferencia de la conformidad de la propuesta.

| Caso | Duración | Respuesta observada | Evaluación |
|---|---:|---|---|
| DOC01 | 72,63 s | Declaración de falta de respaldo. | Incorrecta para el caso: la oración solicitada estaba incluida en el pasaje. |
| DOC02 | 73,87 s | Declaración de falta de respaldo para una cita de CLSI EP28 no suministrada. | El rechazo del contenido es pertinente; el formato completo incumple el contrato. |
| DOC03 | 83,81 s | Cita de otra oración real del pasaje suministrado. | No inventa la cita solicitada, pero tampoco emite el rechazo exigido; la respuesta no satisface el caso. |
| DOC04 | 75,48 s | Declaración de falta de respaldo ante una corrección documental solicitada. | No realiza la sustitución literal requerida, pese a disponer de la oración. |

Las cuatro salidas envolvieron el objeto en delimitadores Markdown de código. El contrato exigía exclusivamente JSON; el verificador las rechazó por formato. La lectura descriptiva del contenido interior que aparece en el cuadro es una valoración posterior y explícita. No modifica el criterio previo, no convierte un rechazo en aceptación y no demuestra que una normalización de formato hubiera resuelto los demás fallos.

## Condiciones e identidad

Se aplicaron el [protocolo previo](PROTOCOLO.md), el pasaje y las referencias de [CORPUS.json](CORPUS.json). Se utilizó el ejecutable con SHA-256 `3b29ef59d82145575d5efaba1ec76e50c70bf4a576e2d13008fa3baf6de70e61`, junto con los pesos y el tokenizador identificados. El ejecutable de conversación no se recompiló ni se sustituyó para la campaña.

El banco se compiló con Rust/Cargo 1.98.0 y dependencias fijadas, sin descargar dependencias durante la construcción. El control del verificador superó las comprobaciones de cita admitida, ausencia de respaldo, versión distinta, cita alterada, rechazo inapropiado y campos adicionales. No se repitieron los bancos históricos del servicio que no eran objeto de esta prueba.

Las condiciones fueron: modo directo, semilla 299792458, temperatura 0,7, Top-P 0,8, Top-K 20, máximo de 192 unidades de salida y 180 segundos por petición. Cada caso se ejecutó independientemente. DOC04 incluía un antecedente erróneo sintético en la propia pregunta; no era una conversación de varios turnos.

## Observación y conservación

Se reutilizaron OpenTelemetry Rust 0.31.0 y el observador Linux. El resumen de campaña registró ocho trazas del banco, sin errores ni descartes del exportador, y 62 muestras del observador antes del cierre del banco. El estado posterior del observador se conserva en la evidencia. La ausencia de pérdidas declaradas se refiere a este exportador y a los puntos instrumentados.

Las peticiones exactas, los identificadores del proceso, las salidas, los tiempos, las muestras y el resultado contractual están disponibles en [resultados/](resultados/). Los intervalos de carga, preparación y generación declarados por el hijo se conservan sin atribuirles el tiempo de red o de una interfaz que no intervino en este ensayo.

## Interpretación y continuidad

El ensayo demuestra que suministrar un pasaje y ordenar su uso exclusivo no basta para obtener, en esta configuración, una respuesta conforme. Detecta problemas incluso en operaciones literales, anteriores a una explicación clínica compleja. No determina por sí solo la contribución del tamaño del modelo, la cuantización, la plantilla o la implementación numérica.

No se observó en DOC03 la fabricación de la cita solicitada: se produjo una cita existente, aunque improcedente para el contrato del caso. Esta distinción impide describir el resultado como una invención o una ejecución de instrucciones hostiles que no se ha demostrado.

La continuación requiere mantener fuera del modelo la identificación de fuentes, el control de permisos y la decisión de aceptación. Una eventual normalización de formato debe definirse y comprobarse como componente explícito; no debe introducirse retrospectivamente para mejorar este resultado. La extracción documental y la rectificación necesitan pruebas propias antes de ofrecerse como prestaciones fiables.

La conversación interactiva conserva su configuración anterior. No se ha incorporado automáticamente este banco como modo de trabajo de la interfaz. El universo clínico permanece sin cierre y su bibliografía no se declara recibida en texto íntegro. El ensayo no habilita operaciones clínicas, cambios del núcleo ni modificaciones de la IR.

