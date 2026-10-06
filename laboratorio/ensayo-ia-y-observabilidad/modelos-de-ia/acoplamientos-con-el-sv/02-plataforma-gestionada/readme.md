# Nodo 02 · Inferencia mediante plataforma gestionada

**Versión documental 1.1 · 6 de octubre de 2026.**

**Estado:** especificación preparatoria. Kaggle es el primer caso considerado; esta documentación no acredita integración, recursos concedidos ni ejecución de modelos.

Este nodo desarrolla la segunda modalidad del [marco común de acoplamientos](../readme.md). Mantiene las fuentes rectoras, obligaciones de privacidad, correspondencia N-01–N-10 y seguimiento allí establecidos. Su aceptación se determina por separado de la inferencia bajo control propio y de la API directa.

Los expedientes específicos de esta modalidad se organizan aquí por plataforma y modelo. Los estudios anteriores mantienen sus rutas y se consultan mediante las [remisiones del nodo 01](../01-inferencia-bajo-control-propio/readme.md#expedientes-en-su-ubicación-de-origen); su ubicación no los convierte en resultados de plataforma gestionada.

## 1. Objeto y distribución de responsabilidades

El SV utiliza una plataforma que organiza tareas de evaluación y ofrece acceso gestionado a modelos. El gobierno, la preparación semántica, las decisiones y las comprobaciones propios permanecen en Rust. La plataforma atiende las solicitudes mediante su servicio intermediario y la infraestructura de inferencia correspondiente.

Para el caso Kaggle, la comunicación técnica del programa del 06/10/2026 confirma que las tareas pueden utilizar su SDK de Python y llamar a binarios o enlaces Rust. También precisa que la inferencia de modelos se realiza mediante un intermediario gestionado cuya implementación no controla el usuario. Esa posibilidad de llamada no acredita todavía que todos los requisitos del SV puedan imponerse mediante una realización concreta.

Esta modalidad permite estudiar respuestas de modelos sin administrar sus pesos. No proporciona por ello un servidor equivalente al del nodo 01. El acceso a cuadernos, el acceso a modelos y la concesión de una ayuda son condiciones diferentes y se documentan por separado.

## 2. Recorrido y fronteras técnicas

El recorrido previsto es: operación constituida del SV → admisión y preparación en Rust → adaptación mínima exigida por la plataforma → intermediario gestionado → modelo → recepción y comprobaciones en Rust.

Se distinguen la relación Rust–adaptación instrumental y la relación plataforma–servicio de inferencia. La primera puede realizarse mediante FFI o mediante intercambio entre procesos, según las capacidades admitidas y las garantías demostrables. La segunda responde al contrato del servicio remoto. Ninguna elimina la frontera entre el Núcleo y su entorno operacional.

Cuando intervenga Python, su función se limita a lo instrumental exigido por el SDK: declarar la tarea, transportar entradas y salidas y comunicar estados. No asume la semántica del SV, la adjudicación de respuestas, la decisión de permisos ni la aceptación. Si el SDK impone una función incompatible con esta separación, se documentará como impedimento antes de presentar la realización como conforme.

La FFI exige comprobar tipos, tamaños, propiedad y duración de la memoria, fallos y concurrencia. La alternativa entre procesos exige formato versionado, correlación de solicitudes, límites, errores y permisos efectivos. Ni una FFI ni un proceso separado acreditan aislamiento por su mera presencia.

La ubicación del componente Rust se decidirá y verificará en el encargo: entorno propio, entorno de evaluación de la plataforma o distribución expresamente delimitada. **Las comprobaciones previas a la comunicación de datos deben ejecutarse antes de subirlos al entorno remoto.** Ejecutarlas allí después de la carga no revierte esa comunicación.

## 3. Prestaciones que deben quedar acreditadas

| Condición | Evidencia necesaria antes de la campaña |
|---|---|
| Acceso efectivo | Cuenta habilitada para la operación concreta y requisitos de identificación aplicables a ella. |
| Modelo | Identificador ejecutable, versión ofrecida y metadatos disponibles; ausencia de sustituciones silenciosas. |
| Recursos | Cuota aplicable, modo de cómputo, renovación, concurrencia, límites por solicitud y causas de suspensión. |
| Tiempo | Vigencia del acceso o ayuda y condiciones de terminación; ninguna duración se presume indefinida. |
| Entorno | Posibilidad efectiva de ejecutar el componente Rust y la adaptación mínima, con sus restricciones. |
| Evaluación | Acceso a entradas, resultados y diagnósticos suficientes; admisibilidad del protocolo adversarial previsto. |
| Datos y publicación | Destinatarios, conservación, visibilidad de tareas y datos, reserva de la clave y condiciones de apertura. |

Una respuesta favorable a una consulta, el envío de un formulario o su acuse no equivalen a concesión. Los importes, cuotas y catálogos se fijarán mediante evidencia fechada en cada encargo; este README no los presenta como prestaciones garantizadas.

Se sigue la [prioridad de candidatos del marco común](../readme.md#2-tres-nodos-experimentales), sujeta al acceso efectivo y a las condiciones de esta modalidad. La identidad de cada modelo utilizado se conservará en su expediente; una preferencia experimental no acredita disponibilidad ni aceptación.

## 4. Protección de datos, reserva y licencias

El alcance inicial utiliza información completamente artificial, revisada antes de cualquier carga. Las instrucciones, ejemplos, adjuntos y registros reciben la misma revisión de destino y finalidad que los datos principales. Se excluyen historias clínicas, información seudonimizada de pacientes y registros privados de ciberseguridad.

La clave de evaluación se mantiene fuera de lo entregado al candidato. Se distinguirán materiales abiertos, pruebas reservadas y componentes necesarios para evaluar; el diseño concreto debe impedir que el modelo acceda a la clave por archivos, contexto o herramientas. Una marca de conjunto privado no demuestra por sí sola esa separación.

El eventual paquete abierto destinado al programa se delimitará expresamente: archivos nuevos, procedencia, licencias, dependencias y permisos de distribución. La apertura aceptada no transfiere la titularidad ni modifica automáticamente las licencias del Núcleo, de la DSL o de la documentación previa. Si una condición de publicación resulta incompatible con esos derechos, se resuelve antes de publicar el material afectado.

Los controles de salida, las restricciones de persistencia y la supresión se concretan según las capacidades demostrables. No se atribuye al SV control sobre registros o componentes internos del proveedor que no pueda observar o administrar.

## 5. Pruebas, aceptación y límites

Primero se comprueba el recorrido instrumental con casos artificiales controlados: una solicitud autorizada, una denegada sin transmisión, entrada inválida, contexto excesivo, modelo no disponible, agotamiento de cuota, interrupción y resultado incompleto. Las pruebas verifican conservación de identidad, permisos, representación y diagnóstico. Las limitaciones que impidan una prueba se registran; no se convierten en resultado favorable.

Después se aplica al modelo el protocolo completo de contenido y criticidad. La política adversarial debe ser uniforme, preservar antecedentes y registrar tanto correcciones como regresiones. La ejecución, el juicio externo y la decisión de aceptación conservan funciones diferentes. Un fallo de transporte no representa el valor U del SV.

Se conserva la solicitud efectiva y la respuesta dentro del régimen de datos admitido, junto con versión del componente propio, versión del SDK, identificadores de ejecución, parámetros, límites y resultados. Los datos internos no observables del servicio se declaran desconocidos. No se presume equivalencia con un modelo de pesos abiertos por compartir nombre comercial.

**Condición de continuación:** disponer de prestaciones suficientes y delimitadas, y recibir favorablemente la realización Rust y su adaptación instrumental antes de la campaña. Los hallazgos se relacionan con las [necesidades y registros comunes](../readme.md#8-documentación-y-retorno-al-sv). Un impedimento del proveedor no redefine los contratos del SV ni habilita otra modalidad automáticamente.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
