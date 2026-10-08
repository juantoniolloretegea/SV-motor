# Nodo 03 · Inferencia mediante API directa de proveedor

**Versión documental 1.3 · 8 de octubre de 2026.**

**Estado:** modalidad con ejecución experimental acreditada. GPT-6 Astra dispone de cierre Apto para su contrato documental; Qwen3.8-Max-0902 recibe Apto documental con reservas para el alcance revisado de 16 preguntas. La integración general y las dependencias pendientes conservan su alcance propio.

Este nodo desarrolla la tercera modalidad del [marco común de acoplamientos](../readme.md). Se refiere a servicios de inferencia de proveedores externos; una API que exponga un motor administrado por el proyecto se examina en el [nodo 01](../01-inferencia-bajo-control-propio/readme.md). Se conservan las mismas obligaciones del SV y una aceptación independiente para esta modalidad.

Los expedientes de API externa se organizan aquí por proveedor y modelo. La carpeta [OpenAI de este nodo](openai) corresponde a esta modalidad y se distingue del [índice histórico de GPT-OSS](../../openai/README.md), que conserva los antecedentes en su ubicación original. Las [remisiones del nodo 01](../01-inferencia-bajo-control-propio/readme.md#expedientes-en-su-ubicación-de-origen) permiten consultarlos sin trasladarlos ni atribuir sus resultados a una API externa.

<a id="expediente-comprobado--gpt-6-astra"></a>

## Expedientes y recepciones documentales

| Candidato | Alcance y recepción | Evidencia |
|---|---|---|
| [GPT-6 Astra · OpenAI](openai/gpt-6-astra/readme.md) | **Apto** para el contrato P01–P25: 25 finales correctas, 20 críticas en 0; T(25)=19. | [Informe del 07/10/2026](openai/gpt-6-astra/examen25-20261007/resultado/INFORME.md). |
| [Qwen3.8-Max-0902 · Alibaba Cloud](qwen/qwen3.8-max-0902/readme.md) | **Apto documental con reservas**, para 16 preguntas, tras revisión metodológica y réplica P13. | [Recepción del 08/10/2026](qwen/qwen3.8-max-0902/RECEPCION-DOCUMENTAL-20261008.md). |

Cada recepción corresponde a su expediente y alcance. El proveedor ejecuta la inferencia; el Árbitro-Director y los componentes Rust del SV administran suministro, orden, recepción, adjudicación y medición propios. La recepción no se extiende a otros modelos, a uso clínico o a toda la modalidad. La dependencia criptográfica y la evaluación del servicio conservan su seguimiento específico.

## 1. Objeto y alcance del control

El SV prepara una solicitud autorizada y la dirige mediante un cliente Rust a la API del proveedor. Recibe una respuesta y aplica las comprobaciones previstas por la función, el dominio y el protocolo experimental.

Se administra el componente propio: fuentes utilizadas, finalidad, destinatario, datos transmitidos, parámetros solicitados, permisos, límites y tratamiento de resultados. No se administra la implementación interna del modelo remoto. Una respuesta conforme acredita el caso examinado dentro de sus condiciones; no demuestra por sí sola cómo se produjo internamente.

El cliente puede ejecutarse en un equipo propio o en un servidor admitido. Su ubicación se decide por las necesidades de custodia, protección y operación. No requiere albergar los pesos del modelo. Tampoco convierte automáticamente en admisible la transmisión de información disponible en ese equipo.

## 2. Contrato común y adaptación por proveedor

El recorrido previsto es: operación constituida del SV → control y preparación en Rust → adaptador del proveedor en Rust → API remota → recepción y comprobaciones en Rust → evaluación y decisión competentes.

La comunicación directa con una API remota no exige por sí misma una FFI ni Python. La frontera mantiene las obligaciones semánticas, de identidad, privacidad y trazabilidad del [marco común](../readme.md#4-contrato-de-frontera-ffi-y-api). El protocolo de red sólo materializa una parte del intercambio.

Se busca reutilizar el contrato aplicable y las comprobaciones propias, con adaptaciones explícitas a cada proveedor. No se presume una operación universal idéntica. Cada adaptación declara la correspondencia de campos, capacidades y restricciones, preservando los significados requeridos por el SV.

| Aspecto | Declaración y comprobación por proveedor |
|---|---|
| Identidad | Servicio, destino, modelo solicitado, versión disponible y metadatos devueltos. |
| Entrada | Formato conversacional, documentos, modalidades, contexto y límites; transformaciones declaradas. |
| Resultado | Texto, estructura, fin de generación, respuesta parcial, rechazo y errores diferenciados. |
| Herramientas | Capacidades admitidas y control propio previo a cualquier efecto solicitado. |
| Operación | Tiempo máximo, concurrencia, repetición, cancelación e idempotencia cuando estén disponibles. |
| Datos | Destinatarios, ubicación declarada del tratamiento, conservación, uso ulterior y mecanismos de supresión. |
| Consumo | Límites de uso y gasto autorizados, medición observable, estimaciones y condiciones de facturación. |

Una capacidad ausente se declara como incompatibilidad o restricción del alcance. No se suplanta mediante cambio oculto de modelo, reducción silenciosa de contexto o interpretación improvisada. Compartir un formato de API no acredita equivalencia entre proveedores.

## 3. Habilitación de datos y acceso

La primera preparación emplea sólo datos completamente artificiales. Medicina e inmunología y ciberseguridad conservan su contenido profesional y sus exigencias; esta condición de prueba evita introducir pacientes, registros privados o secretos durante la comprobación instrumental.

Antes de transmitir se determina qué información sale, para qué operación, hacia qué destinatarios y bajo qué condiciones de conservación y uso. Se incluyen instrucciones, adjuntos, herramientas, resultados y registros. Subir un archivo mediante una operación auxiliar de la API también comunica datos, aunque no se haya solicitado inferencia.

La autorización se comprueba en el momento de actuar y de exportar. Los cambios de finalidad, destinatario, modelo o categoría de datos requieren el tratamiento previsto por el contrato. La utilización futura de datos personales no queda habilitada por estas pruebas; necesitará evaluación contextual, autorización y evidencia específicas.

Las credenciales se limitan a las operaciones necesarias y permanecen fuera de documentos públicos, instrucciones del modelo y registros de contenido. Las respuestas no pueden utilizarlas ni ampliar facultades. La revocación de un acceso local no demuestra la supresión de datos ya recibidos por el proveedor; los alcances de ambas actuaciones se registran por separado.

## 4. Interrupciones, repetición y consumo

El encargo fija antes de ejecutar las cotas de solicitudes, volumen, duración y coste aplicables. Una autorización de estudio económico no equivale a autorización de consumo. La medición debe distinguir estimación local, uso comunicado por el servicio y liquidación efectiva.

Si se pierde la conexión, puede desconocerse si el proveedor llegó a procesar la solicitud. No se repite automáticamente como si no hubiese ocurrido nada. Se aplica la política de continuación autorizada, utilizando mecanismos de correlación o idempotencia sólo cuando el servicio los ofrezca y se hayan comprobado.

Del mismo modo, una cancelación local no prueba que la inferencia remota haya terminado ni que deje de computar consumo. Se conservan los estados confirmado, rechazado, parcial o indeterminado según la evidencia. Ninguno se transforma automáticamente en U ni en aceptación semántica.

La selección de un modelo alternativo o un segundo proveedor requiere las condiciones de admisión que correspondan. Este nodo no permite alternancias silenciosas para obtener una respuesta a cualquier precio o bajo condiciones diferentes.

## 5. Pruebas y aceptación

La comprobación instrumental comienza con respuestas controladas y datos artificiales: admisión y denegación, representación inválida, cambio de modelo, contexto excesivo, permisos vencidos, agotamiento de cuota, interrupción y respuesta incompleta. Debe distinguirse qué pruebas son locales y cuáles se han ejecutado realmente contra el servicio.

El cliente Rust comprobará integridad de la representación recibida, correlación y diagnósticos disponibles. La validez de formato no demuestra la corrección médica, científica o de ciberseguridad del contenido. La evaluación del candidato utiliza los casos y criterios externos previstos en el protocolo común, sin permitir que el modelo reciba la clave ni adjudique su propia aceptación.

Se identifican por ejecución las fuentes y transformaciones admitidas, revisión del cliente y adaptación, configuración efectiva observable, resultado, consumo y causas de terminación. Los registros se minimizan según su finalidad y protección; la necesidad de auditoría no autoriza conservar indiscriminadamente datos sensibles.

Una comparación con los nodos 01 o 02 debe declarar cambios de versión, precisión, instrucciones, contexto y funciones de servicio. El resultado de una API no garantiza el de pesos publicados bajo una denominación similar. La transferencia de un candidato a inferencia propia exige sus pruebas y recepción específicas.

## 6. Documentación y condición de continuación

Se sigue la [prioridad de candidatos del marco común](../readme.md#2-tres-nodos-experimentales), condicionada por la identidad y disponibilidad efectivas y por la admisibilidad del servicio. La versión concreta se fija en el expediente de cada prueba, evitando presentar preferencias como prestaciones acreditadas.

El expediente de esta modalidad relacionará especificación, adaptación Rust, capacidades y condiciones fechadas del proveedor, pruebas, resultados, incidencias y decisión con límites explícitos. Utiliza las [sedes de seguimiento y retorno existentes](../readme.md#8-documentación-y-retorno-al-sv). Una insuficiencia del adaptador se distingue de un fallo del candidato o de una carencia demostrada del contrato.

**Condición de continuación:** proveedor y operación delimitados, protección de datos y consumo autorizados, y realización propia comprobada antes de la campaña. La publicación documental no activa servicios, introduce credenciales, ejecuta llamadas ni modifica los contratos del SV.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
