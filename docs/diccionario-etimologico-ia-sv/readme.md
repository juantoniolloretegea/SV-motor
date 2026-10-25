# Diccionario de la Inteligencia Artificial (IA)

**Enciclopedia temática del Sistema Vectorial SV · Volumen II · Primera edición, 25 de octubre de 2026.**

Este volumen explica los conceptos empleados para evaluar, autorizar y gobernar el uso de modelos de IA en el SV. Delimita el significado de cada término y su relación con los libros competentes. No constituye una autorización de uso ni convierte una capacidad descrita en una capacidad acreditada.

Las definiciones propias del SV se identifican como tales. Las siglas y equivalencias inglesas facilitan la consulta; su presencia no sustituye la explicación en español. Las denominaciones de una convocatoria se conservan con su alcance original y no se generalizan a personas ni a capacidades no examinadas.

## Consulta temática

| Materia | Términos relacionados |
| --- | --- |
| Conocimiento y organización | [Agente Especializado](#agente), [biblioteca de dominio](#biblioteca), [dominio](#dominio), [Universo](#universo). |
| Modelo y uso | [Modelo de IA](#modelo), [modo predominante](#modo), [grados de libertad](#grados), [predominancia](#predominancia), [nodo](#nodo). |
| Examen y decisión | [Evaluación](#evaluacion), [clasificación](#clasificacion), [dictamen](#dictamen), [autorización](#autorizacion), [recomendación](#recomendacion). |
| Funcionamiento y protección | [Aislamiento](#aislamiento), [IA gobernada](#gobernada), [inferencia](#inferencia), [producción](#produccion), [seudo-producción](#seudoproduccion). |
| Uso y evidencia | [Conexión](#conexion), [consulta](#consulta), [evidencia](#evidencia), [sesión](#sesion), [token](#token). |

## Términos

Las entradas están ordenadas alfabéticamente. Las remisiones enlazan explicaciones complementarias, sin sustituir los contratos de los libros.

### Agente Especializado del SV {#agente}

Construcción definida íntegramente mediante la DSL del Lenguaje de Computación del SV, con alcance, función, versiones, fuentes y condiciones de ejecución identificados. Puede comprender dominios completos, subdominios o conjuntos de Universos.

**Distinción:** no es sinónimo del modelo de IA empleado. La selección del modelo no modifica la semántica ni concede facultades sobre el núcleo. Una consulta no debe recorrer todo el dominio si su objeto no lo requiere, pero tampoco puede omitir dependencias obligatorias. Véase [Alcance de un Agente Especializado](https://documentos-sv.itvia.online/dominios/universos/conceptos.html#alcance-de-un-agente-especializado).

### Aislamiento de la IA {#aislamiento}

Separación material de las facultades del modelo respecto de los recursos y operaciones que no le están autorizados. En el SV impide que la IA alcance directamente el núcleo, modifique el sistema, conceda permisos o acceda a credenciales.

**Límite:** una instrucción dirigida al modelo no basta para demostrar ese aislamiento. Deben existir controles externos y evidencia de su resistencia a entradas maliciosas y accesos alternativos. La exigencia permanece en seudo-producción y en los tres nodos. Véase [Consulta sin autoridad sobre el sistema](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/autoridad.html#consulta-sin-autoridad-sobre-el-sistema).

### Autorización de uso {#autorizacion}

Decisión de una función competente que permite un uso delimitado bajo condiciones expresas. Identifica, según corresponda, institución, persona, dominio, nodo, modelo, configuración, finalidad, vigencia y límites.

**Distinción:** autenticación, evaluación favorable, recomendación y autorización son actos diferentes. Una cuenta admitida no adquiere automáticamente todos los usos. Véase [Facultades, autorización y producción](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/autoridad.html).

### Biblioteca documental de dominio {#biblioteca}

Conjunto de fuentes identificadas y organizado para los fines y condiciones del dominio. Su uso exige comprobar las versiones, la pertinencia, los derechos y la disponibilidad efectiva del contenido requerido.

**Distinción:** un catálogo, una referencia bibliográfica o un DOI no acreditan por sí solos que el modelo haya recibido el documento completo. Véase [Documentos y condiciones de entrega](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/documentos.html).

### Clasificación comparativa de modelos {#clasificacion}

Ordenación de resultados conforme a un criterio fijado para una edición y unas condiciones comparables. Conserva categorías, empates, mediciones ausentes y límites de interpretación.

**En el SV:** elegir un modelo como referencia visual no lo convierte en ganador. La predominancia de uso tampoco añade puntos ni modifica el orden recibido. Los criterios particulares de una convocatoria no se presentan como una regla universal. Véase [Comparación y lectura de resultados](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/comparacion.html).

### Conexión con un proveedor o modelo {#conexion}

Recorrido técnico identificado para acceder a las funciones permitidas de un proveedor o a una instancia de modelo. Debe distinguir el mecanismo de autenticación, el catálogo disponible y las operaciones realmente habilitadas.

**Distinción:** comprobar autenticación o consultar un catálogo no acredita una inferencia ni autoriza enviar contenido. Una suscripción y una API son mecanismos diferentes; ninguna elimina por sí misma la transmisión de la consulta a un modelo remoto. Véase [Preparación del acceso y límites individuales](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/autoridad.html#preparación-del-acceso-y-límites-individuales).

### Consulta al modelo {#consulta}

Solicitud delimitada que se presenta a un modelo para obtener una respuesta bajo condiciones identificadas. Su finalidad, fuentes, autorización, consumo y resultado deben conservar la relación que permita examinarlos.

**Distinción:** abrir una página o cargar un visor no equivale a iniciar una consulta. Una respuesta tampoco constituye por sí sola una decisión determinista del SV ni una autorización profesional. Véase [Estado de seudo-producción](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/autoridad.html#estado-de-seudo-producción).

### Dictamen de evaluación {#dictamen}

Resultado interpretado conforme a la cuestión, al contrato y a la regla de evaluación aplicables. En los exámenes descritos del SV se distingue 0 — Apto, 1 — No apto y U — Indeterminado.

**Límite:** las incidencias técnicas conservan causa propia y no se convierten automáticamente en dictamen. La aptitud acreditada para un examen no demuestra competencia universal ni autoriza producción. Véase [Corrección y criterio de aptitud](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/correccion.html).

### Dominio {#dominio}

Ámbito de conocimiento organizado para una finalidad y unas operaciones delimitadas. En el SV su constitución exige identificar los Universos, las fuentes, las relaciones y los límites pertinentes.

**Distinción:** la denominación de una especialidad no acredita que exista un agente activo ni que todo su conocimiento esté formalizado. Un dominio clínico y otro de ciberseguridad conservan sus propias reglas, aunque se utilicen en la misma institución. Véase [Definiciones y relaciones](https://documentos-sv.itvia.online/dominios/universos/conceptos.html).

### Evaluación de un modelo de IA {#evaluacion}

Examen de capacidades o resultados mediante una edición, un protocolo, fuentes, criterios y condiciones identificados. Permite emitir conclusiones dentro del alcance de la evidencia obtenida.

**Distinción:** una prueba de comprensión documental no acredita por sí sola competencia clínica, control de un servidor o funcionamiento íntegro del SV. Los antecedentes conservan las condiciones con las que se evaluaron. Véase [Finalidad y alcance de la evaluación](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/finalidad.html).

### Evidencia de evaluación {#evidencia}

Contenido o registro identificable que permite examinar una afirmación dentro de un alcance declarado. Incluye, cuando corresponda, fuentes, entradas, respuestas, criterios de corrección, mediciones y condiciones de obtención.

**Límite:** una afirmación sin comprobación no se convierte en evidencia por registrarla. La integridad de un archivo y la validez científica de su contenido son cuestiones distintas. Véase [Comparación y lectura de resultados](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/comparacion.html).

### Grados de libertad del modelo {#grados}

Denominación organizativa del SV que relaciona el modo predominante de un uso con la asignación del modelo a dominios y funciones. Permite describir predominancias y relaciones secundarias sin reducir un modelo a una única especialidad.

**Distinción:** aquí no designa grados de libertad estadísticos ni físicos. Un modelo puede ser predominante en varios dominios; varios modelos pueden coexistir como predominantes si sus funciones están delimitadas. La declaración no concede autonomía adicional. Véase [Modos y grados de libertad](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/autoridad.html).

### IA gobernada {#gobernada}

Expresión que identifica un uso de IA sometido a reglas externas, responsabilidades, permisos, límites y comprobaciones. En el SV, el modelo no determina por sí mismo las condiciones de su actuación ni la validez de sus resultados.

**Distinción:** gobernar el uso de un modelo no vuelve determinista su generación de texto. Las propiedades del recorrido del SV y las de la inferencia deben acreditarse separadamente. Véase [Agentes Especializados y frontera](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/autoridad.html#agentes-especializados-y-frontera).

### Inferencia del modelo {#inferencia}

Operación mediante la cual un modelo obtiene una salida a partir de una entrada y de su configuración. En este diccionario se distingue de la evaluación científica de esa salida y de la decisión institucional sobre su uso.

**Límite:** una respuesta generada no acredita por sí misma corrección, exhaustividad o determinismo. Debe conservarse la identidad del modelo y la configuración relevante. Véase [Finalidad y alcance](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/finalidad.html).

### Modelo de inteligencia artificial {#modelo}

Sistema computacional identificado por su versión y configuración, utilizado para producir salidas a partir de entradas. Sus capacidades se examinan mediante evidencia pertinente al uso previsto.

**Distinción:** el nombre comercial no sustituye la versión ni la conexión utilizada. Modelo, proveedor, Agente Especializado y autorización institucional son identidades diferentes. Véase [Validación, autorización y puesta en Producción](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/index.html).

### Modo predominante del modelo {#modo}

Declaración del objetivo principal de un uso identificado del modelo, sin excluir otras capacidades. El SV distingue **Sin modo como objetivo**, **No actuación o neutral** y **Actuación o activo**.

El modo neutral orienta al experto mediante consejo, evaluación o revisión, sujeto a su aprobación. El activo contempla actuaciones dentro de capacidades expresamente configuradas y autorizadas. «Sin modo como objetivo» conserva los usos o exámenes que no fijaron ese objetivo.

**Distinción:** no es una propiedad inmutable del modelo ni una licencia para actuar. El examen conserva la declaración con la que se preparó. Véase [Facultades, autorización y producción](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/autoridad.html).

### Nodo de uso del modelo {#nodo}

Categoría que identifica el recorrido de acceso y ejecución considerado por el SV: **nodo 1**, modelo bajo control propio; **nodo 2**, acceso mediante plataforma intermedia; **nodo 3**, acceso a un proveedor externo. La configuración concreta debe identificar recursos, conexión y responsabilidades.

**Límite:** describir un nodo no autoriza su utilización. Ninguno permite acceso directo de la IA al núcleo. El control propio puede evitar un coste de proveedor, pero no elimina el consumo de capacidad ni la necesidad de cuotas. Véase [Presupuesto y capacidad por nodo](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/autoridad.html).

### Predominancia en un dominio {#predominancia}

Relación de asignación por la que un modelo desempeña una función principal en un dominio y uso identificados. Puede coexistir con relaciones secundarias o con otras predominancias expresamente delimitadas.

**Distinción:** no equivale a superioridad universal ni a un puesto de clasificación. Su fundamento debe relacionar función, capacidades evaluadas, límites y evidencia. La mera proximidad temática no justifica otra asignación. Véase [Comparación por modos y predominancias](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/comparacion.html#comparación-por-modos-y-predominancias).

### Producción {#produccion}

Estado de uso que requiere recibir las condiciones técnicas, científicas, organizativas y de autorización aplicables a la instalación y a la operación. No se obtiene cambiando una etiqueta de presentación.

**En el SV:** la protección del núcleo, la biblioteca y las relaciones constituidas, la trazabilidad y el recorrido determinista exigible conservan sus contratos. Una prueba parcial o una figura visible no acreditan el conjunto. Véase [Facultades, autorización y producción](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/autoridad.html).

### Recomendación de un modelo {#recomendacion}

Conclusión fundada en resultados comparables y en un criterio declarado, limitada a su finalidad y condiciones. Debe explicar las reservas y, cuando corresponda, la imposibilidad de identificar una recomendación inequívoca.

**Distinción:** no sustituye la selección responsable de una institución ni la autorización de una operación. Las mediciones ausentes no se reemplazan por cero. Véase [Selección, clasificación e independencia](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/comparacion.html).

### Seudo-producción {#seudoproduccion}

Estado de ensayo u observabilidad en laboratorio de una configuración de uso del SV. Puede presentar externamente la interacción con agentes sin acreditar las condiciones de producción.

El protocolo puede permitir Internet, prescindir de una biblioteca cerrada constituida y ofrecer respuestas sin un recorrido determinista acreditado. Estas excepciones se declaran antes de la consulta y acompañan los resultados. **El aislamiento de la IA y la protección del núcleo permanecen obligatorios.**

**Distinción:** es un estado de funcionamiento, no uno de los tres modos del modelo ni una autorización abreviada. Véase [Estado de seudo-producción](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/autoridad.html).

### Sesión de uso {#sesion}

Contexto identificado de acceso y actividad bajo una autorización. Debe relacionar persona, modalidad, dominio, nodo, modelo, periodo y límites pertinentes.

**Distinción:** la sesión de identificación no prueba una consulta al modelo. Enviar una invitación tampoco demuestra uso efectivo. Los accesos ordinario e invitado mantienen autorizaciones separadas, sin cambio automático al agotar una cuota. Véase [Acceso ordinario y acceso invitado](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/autoridad.html).

### Token · Unidad de procesamiento del modelo {#token}

Unidad empleada por un modelo o servicio para representar o contabilizar parte de su entrada o salida. Su delimitación depende del sistema utilizado; no equivale de forma fija a una palabra ni a un carácter.

**Límite:** los tokens de proveedores diferentes no son unidades lingüísticas idénticas. Tampoco se convierten automáticamente en tiempo, consultas, créditos o moneda. La contabilidad exige fuente, tarifa, categorías y periodo identificados. Véase [Comparación y lectura de resultados](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/comparacion.html).

### Universo del SV {#universo}

Unidad constituida para una operación delimitada dentro de un dominio, con finalidad, fuentes, relaciones, exclusiones y resultado previsto según su marco específico. En el marco de Inmunología es unidad de evaluación y autorización de la operación clínica definida.

**Distinción:** no es una carpeta temática ni se define trasladando automáticamente condiciones de otro dominio. La entrada remite a la definición documental recibida y no introduce una formulación matemática nueva. Véase [Dominio y Universo](https://documentos-sv.itvia.online/dominios/universos/conceptos.html#dominio-y-universo).

## Publicaciones

Los enlaces se abren en una pestaña nueva. Las referencias externas aportan contexto; no certifican el SV ni sustituyen las reglas de sus exámenes. Los DOI sólo se incorporan cuando están identificados en la publicación correspondiente.

| Ref. | Publicación y enlace | Identificación y relación |
| --- | --- | --- |
| IA-01 | [Validación, autorización y puesta en Producción de modelos de Inteligencia Artificial](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/index.html) | Edición consultada el 25/10/2026. Sin DOI identificado. Evaluación, modos, predominancias y condiciones de uso. |
| IA-02 | [Dominios y universos del Sistema Vectorial SV - Definiciones y parámetros · V.03](https://documentos-sv.itvia.online/dominios/universos/index.html) | Sin DOI identificado. Constitución de dominios, Universos y alcance de agentes. |
| IA-03 | [Reglas, usos y descripciones documentales de documentos para el conocimiento de la IA · V.08](https://documentos-sv.itvia.online/documentacion/reglas-usos-descripciones-documentales/index.html) | Sin DOI identificado. Fuentes, representaciones, permisos y entrega documental. |
| IA-04 | [Artificial Intelligence Risk Management Framework (AI RMF 1.0)](https://doi.org/10.6028/NIST.AI.100-1) | NIST AI 100-1, 2023. DOI: 10.6028/NIST.AI.100-1. Marco voluntario de gestión de riesgos; referencia contextual para responsabilidades y evaluación, sin equivalencia con una certificación. |

## Casos de uso o ejemplos específicos

### Dos funciones principales en una misma institución

En un hospital, una vinculación puede reservar el consejo inmunológico a un modelo y la protección técnica a otro. Ambos pueden desempeñar funciones principales delimitadas. Compartir la institución no convierte la aptitud clínica en aptitud de ciberseguridad, ni permite intercambiar documentos o permisos entre consultas. Intervienen [dominio](#dominio), [predominancia](#predominancia) y [autorización](#autorizacion).

### Comparar sin modificar la clasificación

Una persona fija un modelo como referencia y compara otros con él. La interfaz puede disponerlos en columnas o en filas sucesivas, pero conserva los puestos, categorías, edición y criterio de la evaluación. Si no existe evaluación de un uso determinado, la declaración de predominancia no proporciona el resultado que falta. Intervienen [clasificación](#clasificacion), [evaluación](#evaluacion) y [evidencia](#evidencia).

### Distinguir demostración, sesión y consulta

Una cuenta invitada accede a una página de preparación. La identificación y la carga del visor quedan comprobadas, pero todavía no se ha enviado una consulta. Si se habilita un ensayo de seudo-producción, sus excepciones documentales y sus límites deben explicarse antes del uso. La IA continúa sin autoridad sobre el núcleo. Este ejemplo no activa un modelo ni acredita un consejo profesional.

### Interpretar un consumo incompleto

Una respuesta tiene un recuento conocido de tokens, pero su cargo final aún no está confirmado. El registro conserva ambas circunstancias; no atribuye coste cero ni convierte tokens en euros sin tarifa y alcance documentados. La autorización de una consulta y su conciliación económica son comprobaciones diferentes.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
