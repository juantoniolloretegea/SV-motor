# Autorización y entrega efectiva a la IA

La autorización determina qué documentación puede utilizarse, para qué finalidad y con qué destinatario. La entrega determina qué contenido concreto salió del control documental en una consulta. Ambos aspectos deben conservarse por separado.

## Autoridad y vigencia

Sólo el administrador y el Proveedor de documentos para la IA pueden autorizar o retirar el suministro dentro de su ámbito de competencia. Disponer de una cuenta, consultar un documento, editar una ficha o ejecutar una conversión no confiere esa potestad.

Antes de suministrar contenido se comprueban la autorización vigente, la edición, el dominio, el destinatario, el alcance de la licencia y la situación documental. La retirada impide nuevas entregas, incluso de consultas preparadas previamente. No borra las consultas anteriores ni demuestra que un proveedor externo haya eliminado contenido ya recibido.

## Forma de entrega actualmente comprobada

En el recorrido documental de referencia, los originales se vinculan a una representación Markdown identificada. Se seleccionan secciones autorizadas y se prepara un conjunto de **fragmentos de texto**, con la pregunta, sus referencias y las condiciones documentales aplicables. Ese contenido se remite mediante el mecanismo previsto para el modelo.

El modelo no recibe por ello la página de mdBook que ve el revisor, el archivo PDF completo ni las imágenes que falten en la extracción. Tampoco recibe acceso general a una biblioteca por el hecho de recibir fragmentos de ella.

La entrega directa de un PDF o una caché HTML **es una posibilidad admitida por la regla documental, pero aún no está acreditada en este recorrido**. Su incorporación deberá verificar qué admite realmente el receptor, qué se remite, si existen transformaciones adicionales y cómo se registra el resultado. No se impondrá una conversión no autorizada para suplir una incompatibilidad.

## Lo que acredita cada estado

| Estado o evidencia | Qué acredita | Qué no acredita |
| --- | --- | --- |
| Contenido preparado | Existe una selección identificada y un contenido dispuesto para una consulta | Que haya salido del sistema o llegado al proveedor |
| Intento registrado | Se inició una actuación de envío | Que haya terminado correctamente |
| Entregado al transporte | El mecanismo de envío confirmó la recepción de los datos que se le entregaron | La recepción final por el proveedor, salvo evidencia específica adicional |
| Resultado incierto | No existe confirmación suficiente para resolver el resultado | Que el contenido no haya salido; no debe confundirse con un rechazo seguro |
| Respuesta recibida | Se conserva una respuesta asociada a la consulta | Que el modelo haya utilizado correctamente todos los documentos |
| Citas cotejadas | Las referencias pueden contrastarse con las fuentes conservadas | Corrección semántica, exhaustividad ni validez científica de la respuesta |

El acuse del proveedor, cuando exista, deberá identificarse como evidencia propia. No se sustituirá por el resultado de una función de envío ni por una afirmación del modelo. El procesamiento interno del proveedor puede no ser observable.

## Qué debe conservarse por consulta

Se deben poder relacionar la consulta y el destinatario con la autorización aplicada, la edición de cada documento, la representación utilizada, las páginas o secciones seleccionadas, sus huellas y el contenido remitido. La respuesta, sus citas y la evaluación posterior constituyen evidencias diferenciadas.

Si se conoce el consumo, se registrará con su fuente y unidad: tokens de entrada, salida y coste. Un dato no comunicado permanece desconocido; no se sustituye por cero. Una huella identifica un contenido: no acredita que el modelo lo haya comprendido.

Los nodos 1, 2 y 3 conservan la misma autoridad documental. El nodo 2 se reserva a ensayos. Las diferencias entre ejecución propia, plataforma de ensayo y proveedor externo afectan al transporte y a la evidencia disponible; no amplían los permisos del modelo.

## Ejemplo de interpretación correcta

Si el original es un PDF de 120 páginas, el revisor puede estar viendo una representación textual de las páginas físicas 65 a 80. Si una consulta selecciona sólo dos secciones de esa representación, la entrega corresponde a esas dos secciones. Ninguna de estas circunstancias permite afirmar que el modelo recibió las 120 páginas o sus figuras.

Restringir el suministro es un control verificable sobre los datos proporcionados. No permite observar ni borrar el conocimiento previo del modelo. La adecuación de la respuesta al material autorizado requiere su propia evaluación.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
