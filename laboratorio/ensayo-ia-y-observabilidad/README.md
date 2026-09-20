# Ensayo de inteligencia artificial y observabilidad

**Versión documental:** 2.2.1  
**Fecha:** 20 de septiembre de 2026  
**Estado:** experimento en curso; resultados parciales en navegador y candidata de servicio nativo pendiente de comprobación ejecutable. Ninguna de las dos vías acredita todavía la solución completa.  
**Corte documental y de código revisado:** `d82bbe2f5f396eb31da5b095ba0849404cb352b0`.

**Procedencia documental:** [0.1 — primera publicación](https://github.com/juantoniolloretegea/SV-motor/blob/875a3df0f2e07fb3c71f98d7fd6968ccae54af24/laboratorio/ensayo-ia-y-observabilidad/README.md) → [0.2](https://github.com/juantoniolloretegea/SV-motor/blob/e1caf6df5bef2696f20d2b9fc5a5b5e9cffe509e/laboratorio/ensayo-ia-y-observabilidad/README.md) → [2.0](https://github.com/juantoniolloretegea/SV-motor/blob/29a0dbe74018807fddf5759f30de12ac29c162bb/laboratorio/ensayo-ia-y-observabilidad/README.md) → [2.1](https://github.com/juantoniolloretegea/SV-motor/blob/88ea2e6b1e28b5c1ff22d29da7003c854bb8461d/laboratorio/ensayo-ia-y-observabilidad/README.md) → [2.2](https://github.com/juantoniolloretegea/SV-motor/blob/1b9190d1e146ef61599783191019c060d0ff899c/laboratorio/ensayo-ia-y-observabilidad/README.md) → [2.2.1](https://github.com/juantoniolloretegea/SV-motor/blob/078aa80d7e530eb4e38c582ea5a305202f12160a/laboratorio/ensayo-ia-y-observabilidad/README.md). La versión primigenia es la 0.1, de 18 de septiembre de 2026. Los enlaces identifican la primera publicación de cada versión; las modificaciones intermedias y posteriores se conservan en el [historial completo del README](https://github.com/juantoniolloretegea/SV-motor/commits/main/laboratorio/ensayo-ia-y-observabilidad/README.md).

**Revisión 2.2.1:** corrección de la presentación de las figuras. Los dos diagramas se incorporan como imágenes SVG incrustadas, ampliables y acompañadas de sus fuentes textuales. Su visualización no requiere interpretar Mermaid en el README. Se conserva el contenido funcional y el alcance probatorio de la revisión 2.2.

**Revisión 2.2:** incorporación de diagramas separados de las vías A y B, con ubicación de componentes, flujo funcional, control, custodia y límites de lo demostrado. La [versión 2.1](https://github.com/juantoniolloretegea/SV-motor/blob/88ea2e6b1e28b5c1ff22d29da7003c854bb8461d/laboratorio/ensayo-ia-y-observabilidad/README.md) permanece en el historial.

**Revisión 2.1:** precisión documental de las funciones de JavaScript y Rust en ambas vías. No modifica código, criterios de aceptación ni autorizaciones. La [versión 2.0](https://github.com/juantoniolloretegea/SV-motor/blob/29a0dbe74018807fddf5759f30de12ac29c162bb/laboratorio/ensayo-ia-y-observabilidad/README.md) permanece identificada en el historial.

## 1. Objeto, adscripción y autoridad

Este laboratorio evalúa si una implementación Rust puede ejecutar un modelo auxiliar acotado, observar sus operaciones instrumentadas y verificar sus resultados técnicos con un coste medible. Se adscribe a (p1+P3)-Bis del Lenguaje SV. Conserva la separación entre propuesta probabilística, autorización humana y efecto.

El [contrato experimental](contrato/README.md) identifica las fuentes rectoras, las obligaciones recibidas y las reservas. Vincula el ensayo con el acta de rutas de conocimiento de 14 de septiembre y con el Acta 001 de continuidad de 15 de septiembre. Esta revisión explica la arquitectura y el criterio de avance; no sustituye esas fuentes ni constituye un estado canónico paralelo.

Las pruebas emplean objetos sintéticos. No constituyen células SV, operaciones clínicas ni una integración productiva. La inferencia completada, la conformidad contractual y la suficiencia para una operación del SV son juicios distintos. El contenido generado carece de autoridad para conceder permisos, modificar el conocimiento activo o decidir efectos.

## 2. Regla de avance entre las dos vías

Se desarrollará primero la vía que permita alcanzar antes un funcionamiento verificable y materialmente factible, conservando todos los ámbitos de seguridad y las exigencias del SV aplicables a la operación y al perímetro declarado. Después se abordará la otra vía, con sus comprobaciones propias. El trabajo será secuencial.

La rapidez y la factibilidad se evaluarán entre alternativas que satisfagan esas exigencias; no compensan un incumplimiento de seguridad, integridad, autoridad, custodia o límites de recursos. Una obligación pendiente o no comprobable no se contabiliza como satisfecha. Las exigencias que excedan el alcance experimental conservan su sede y su reserva: no pueden declararse cumplidas ni suprimirse para favorecer una vía.

El criterio comprende el tiempo necesario para obtener una realización comprobada —preparación, construcción, integración, ejecución y recuperación de evidencias— y su comportamiento medido. No se reduce a la velocidad de generación de tokens. La factibilidad comprende recursos disponibles, mecanismos de control realmente utilizables, mantenimiento y coste dentro del presupuesto autorizado.

La selección del primer itinerario debe apoyarse en resultados identificados y en pendientes explícitos. No se presume que WASM sea globalmente superior por su aislamiento, ni que la ejecución nativa resuelva el consumo o la custodia por disponer de procesos separados. Si ninguna vía satisface las condiciones, se conserva el diagnóstico sin rebajar los criterios de aceptación. Una interrupción justifica revisión; no demuestra por sí sola la inviabilidad definitiva de una arquitectura.

Este README no habilita ejecuciones, reintentos, servicios, modificaciones de guardas ni gastos. Los encargos y las autorizaciones de cada campaña conservan su alcance.

## 3. Componentes comunes y función de cada uno

| Componente | Función en el ensayo | Alcance de la afirmación |
|---|---|---|
| Rust 1.98.0 | Implementación, adaptación y verificación determinista. | La identidad de la herramienta y la construcción deben comprobarse en cada entorno. |
| Candle | Ejecutar las operaciones numéricas de inferencia. | Biblioteca integrada en el programa; no proporciona por sí sola alojamiento, interfaz, autorización o custodia. |
| Qwen3-0.6B, Q4_K_M | Modelo y pesos seleccionados para la referencia experimental de pequeña escala. | La misma identidad de pesos no acredita igual resultado, rendimiento o suficiencia en entornos diferentes. |
| OpenTelemetry Rust | Instrumentar y exportar las señales previstas. | No impone permisos, no observa toda la actividad del sistema y no acredita por sí solo exhaustividad o independencia. |
| Código de integración y verificación | Vincular petición, ejecución, resultado y evidencia; aplicar los controles del contrato. | Debe comprobarse específicamente en cada vía y frente a casos negativos. |

Los manifiestos de ambas candidatas fijan Candle en `ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a` y OpenTelemetry Rust en 0.31.0. El inventario efectivo de dependencias y los artefactos construidos conservan su identificación propia por campaña. La adopción definitiva de los componentes permanece sin resolver.

## 4. Dos vías de ejecución para un objetivo común

| Aspecto | Vía A: inferencia en navegador mediante WASM | Vía B: inferencia nativa con interfaz web |
|---|---|---|
| Finalidad | Evaluar la ejecución del modelo dentro del entorno aislado del navegador y los controles que puedan mantenerse en él. | Evaluar la ejecución del modelo en un proceso nativo, con control y custodia separados, accesible mediante una interfaz web. |
| Papel de Candle | Se compila a WebAssembly junto con el código Rust correspondiente y ejecuta el modelo dentro de un Web Worker. | Se compila para la máquina anfitriona y ejecuta el modelo en el proceso de inferencia. |
| Ubicación del cálculo | Máquina donde se ejecuta el navegador. | Máquina donde se ejecuta el servicio nativo. |
| Papel de la página | Coordinar el Worker, enviar la petición y recibir resultados y señales. | Enviar solicitudes al servicio y consultar su estado, parada, resultados y evidencias. |
| Comunicación propia | Enlace JavaScript/WASM y mensajes entre página y Worker. | JavaScript de la página envía solicitudes HTTP al servicio Rust; el servicio se comunica internamente con el supervisor. |
| Infraestructura específica | Navegador, motor WASM, enlace JavaScript y Worker; el ensayo actual añade control y custodia exteriores. | Interfaz HTML/JavaScript en el navegador; Axum, Hyper y Tokio para la capa HTTP nativa; procesos y mecanismos de supervisión y custodia preparados en Rust. |
| Recursos aportados por el usuario de una futura URL | Si la página ejecuta WASM localmente, su dispositivo aporta el cálculo y la memoria de inferencia. | Si el servicio está alojado remotamente, su dispositivo ejecuta la interfaz; el anfitrión remoto aporta la inferencia. |
| Comprobación requerida | Inferencia completa, integridad contractual, observación, parada y custodia dentro del entorno definido. | Las mismas obligaciones funcionales y de seguridad, realizadas y comprobadas con los mecanismos de la vía nativa. |

WASM es un destino de compilación y ejecución; no contiene automáticamente todos los componentes del servicio nativo. Un Web Worker del navegador no es un Cloudflare Worker.

La referencia nativa inicial en GitHub Actions fue una ejecución de pruebas. La candidata nativa posterior incorpora un servicio para interacción desde una página. Ambas pertenecen a la vía nativa, pero sus resultados y obligaciones no son intercambiables. GitHub Actions es el entorno de las campañas realizadas, no un alojamiento permanente de inferencia.

El Chrome utilizado en NAV-01 y NAV-02 se ejecutó en el ejecutor remoto de GitHub. Esas campañas no ejecutaron el modelo en el PC del usuario ni acreditan su comportamiento en dicho equipo.

### 4.1. Distribución de responsabilidades entre Rust y JavaScript

La vía B **no elimina JavaScript del conjunto**. En la candidata revisada lo conserva en la interfaz del navegador, mientras que la inferencia, el servicio HTTP, la supervisión y la custodia se implementan en Rust nativo. HTTP es el protocolo de comunicación; su utilización no determina el lenguaje de la página.

| Función | Vía A: navegador/WASM | Vía B: servicio nativo |
|---|---|---|
| Presentación y solicitudes del usuario | HTML y JavaScript en la página. | HTML y JavaScript en la página. |
| Carga y coordinación de la inferencia | JavaScript carga recursos, inicializa el módulo WASM y coordina el Worker; Candle ejecuta el cálculo en WASM. | El proceso Rust carga los recursos y ejecuta Candle; JavaScript solicita la operación mediante la API. |
| Solicitud de parada | La página coordina la terminación del Worker; el ensayo conserva además un supervisor exterior. | La página solicita la cancelación; el control nativo debe validarla y actuar, independientemente de la presentación. |
| Recuperación y presentación de evidencias | En la campaña actual intervienen la página y el controlador exterior. | JavaScript solicita los fragmentos al servicio y permite descargarlos; la custodia y la admisión técnica corresponden al código Rust. |
| Papel de Node.js | Forma parte del controlador exterior de la campaña NAV-02; no es el motor de inferencia. | No forma parte del servicio nativo preparado. Esta afirmación no excluye herramientas exteriores de prueba o de plataforma, que deben inventariarse por separado. |

La [interfaz JavaScript de la candidata nativa](https://github.com/juantoniolloretegea/SV-motor/blob/d82bbe2f5f396eb31da5b095ba0849404cb352b0/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-nativa-02/web/app.js) envía JSON a `/api`, consulta el estado, solicita cancelación, presenta resultados y recupera evidencias. El [servidor Rust](https://github.com/juantoniolloretegea/SV-motor/blob/d82bbe2f5f396eb31da5b095ba0849404cb352b0/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-nativa-02/nativa/servidor.rs) publica esa página y atiende la API.

Las comprobaciones de la interfaz no sustituyen a las del servicio. Permisos, límites, identidad de la petición y admisión del resultado deben imponerse en los componentes responsables aunque la página envíe solicitudes alteradas. La presentación también requiere verificación: un resultado correcto en el servicio puede mostrarse incorrectamente al usuario.

Reducir las responsabilidades de JavaScript no demuestra por sí solo una reducción global de fallos. Eliminarlo también de la interfaz supondría otra realización, no incluida en esta candidata. La prioridad entre las dos vías sigue regida por el apartado 2, sin preferencia automática por lenguaje o número de componentes.

### 4.2. Diagrama de la vía A: navegador y WASM

El diagrama representa el flujo completo previsto de la campaña NAV-02, incluida su rama de interrupción. **La rama de inferencia finalizada no se completó en esa campaña.** NAV01–NAV04 fueron controles sintéticos; no sustituyen la validación de una inferencia completa.

Los cinco participantes se sitúan en el entorno de la campaña, salvo el operador humano. El navegador se ejecutó en GitHub. El bloque «Control y custodia» agrupa el controlador Node y las tareas exteriores de recuperación; no atribuye toda la custodia a Node ni al módulo WASM.

![Diagrama completo de la vía A: flujo funcional, control y custodia](diagramas/via-a.svg)

[Ampliar la figura de la vía A](diagramas/via-a.svg) · [Consultar su fuente textual](diagramas/via-a.mmd)

**Lectura de las fronteras.** El cálculo de Qwen ocurre en el Worker, mediante Candle compilado a WASM. La página coordina y transmite mensajes. La supervisión de la familia de procesos y la recuperación exterior pertenecen al entorno de ensayo. Estas funciones no se trasladan automáticamente a un navegador de usuario al publicar una URL.

La verificación de recursos se describe con la reserva del apartado 5: no constituye autenticación independiente previa de todo el JavaScript ejecutado. Los canales instrumentados tampoco acreditan la ausencia universal de otras actividades.

Fuentes del diagrama: [página de control](resultados/preparacion-navegador-02/web/control.js), [Worker](resultados/preparacion-navegador-02/web/worker.js), [enlace y verificación Rust](resultados/preparacion-navegador-02/pruebas/navegador.rs), [controlador exterior](resultados/preparacion-navegador-02/controlador.mjs) y [resultado NAV-02](resultados/navegador-02/README.md), en el corte de código indicado al inicio.

### 4.3. Diagrama de la vía B: servicio nativo e interfaz web

El diagrama representa la candidata EIO-NAT-PREP-02. **Describe una implementación propuesta; no acredita que el flujo haya sido ejecutado ni que sus controles hayan superado las pruebas.** Los fallos de validación en la API se rechazan antes de llegar al supervisor. En el diagrama se desarrolla la petición que alcanza dicha frontera.

La interfaz corresponde al navegador del usuario. API, supervisor, custodio y proceso de inferencia se sitúan en la máquina anfitriona del servicio. La guarda reside en esa máquina, fuera de la hoja cgroup que supervisa. «Supervisor y custodio» agrupa el hilo de control y el hilo de custodia del mismo proceso; no implica que la escritura bloquee deliberadamente el control de parada.

![Diagrama completo de la vía B: flujo funcional, control y custodia](diagramas/via-b.svg)

[Ampliar la figura de la vía B](diagramas/via-b.svg) · [Consultar su fuente textual](diagramas/via-b.mmd)

**Lectura de las fronteras.** JavaScript presenta y solicita; la API Rust valida y comunica; el supervisor dirige el proceso; Candle calcula; el verificador determina el juicio contractual; el custodio fija la evidencia. La guarda exterior observa y puede terminar el conjunto sujeto a sus permisos y límites. La viabilidad efectiva de esa guarda sigue pendiente.

Un juicio contractual adverso puede acompañar a una ejecución técnicamente completa. La admisión técnica no convierte la respuesta del modelo en una decisión autorizada. La evidencia parcial sólo puede recuperarse por la API cuando exista un conjunto sellado disponible y la API continúe accesible; no se garantiza recuperación tras toda terminación o fallo.

Fuentes del diagrama: [interfaz JavaScript](resultados/preparacion-nativa-02/web/app.js), [API Rust](resultados/preparacion-nativa-02/nativa/servidor.rs), [supervisor](resultados/preparacion-nativa-02/nativa/supervisor.rs), [proceso de inferencia](resultados/preparacion-nativa-02/nativa/inferidor.rs), [custodia](resultados/preparacion-nativa-02/nativa/custodia.rs), [guarda exterior](resultados/preparacion-nativa-02/nativa/guarda.rs) y [diseño y reservas](resultados/preparacion-nativa-02/DISENO.md), en el corte de código indicado al inicio.

Los diagramas separan petición, cálculo, verificación, observación, control y custodia. Su orden representa relaciones funcionales; no constituye una medida temporal ni una prueba de independencia entre componentes. La regla de prioridad del apartado 2 se aplica al cumplimiento del conjunto.

## 5. Seguridad, integridad y observabilidad

| Ámbito | Exigencia común | Distinción que debe conservarse |
|---|---|---|
| Seguridad pasiva | Delimitar capacidades, memoria, interfaces, autoridad y exposición al entorno. | La sandbox WASM aporta aislamiento del módulo; la separación de procesos y la guarda nativas no acreditan por sí solas una protección equivalente. |
| Seguridad activa | Detectar incumplimientos, rechazar operaciones no admitidas, revocar tareas y comprobar la parada. | El control de una sonda sintética no acredita automáticamente la parada y custodia durante una inferencia real. |
| Integridad contractual | Preservar identidades, valores, asociaciones y orden cuando formen parte del contrato; detectar omisiones, alteraciones y permutaciones relevantes. | Ni el aislamiento ni la serialización JSON demuestran por sí solos la fidelidad de una representación del SV. |
| Observabilidad | Declarar puntos instrumentados, cobertura, relojes, pérdidas y costes. | OpenTelemetry registra señales previstas; la ausencia de eventos no demuestra ausencia de actividad exterior. |
| Custodia | Conservar originales, identificar artefactos y declarar evidencia incompleta o no recuperable. | La concordancia de huellas acredita las identidades cotejadas; no prueba independencia frente a la alteración conjunta del productor y sus registros. |
| Recursos y coste | Respetar las cotas autorizadas y medir dentro de un perímetro explícito. | La memoria lineal WASM, la RSS agregada y las métricas de otros procesos no son magnitudes intercambiables. No se habilita gasto adicional. |

La protección del anfitrión frente al módulo y la protección de los datos del SV frente a un anfitrión comprometido son problemas distintos. La declaración de una sandbox no cierra ambos.

En la implementación del ensayo de navegador, la página depende de la captura exterior y del permiso instrumental emitidos por el controlador. La supervisión de procesos también es exterior. Publicar únicamente los archivos web no traslada esos controles a un navegador de usuario.

El Worker importa el enlace JavaScript antes de cotejar mediante otra descarga su huella; el manifiesto se obtiene del mismo origen. Ese cotejo no se presenta como autenticación independiente previa de todo el código ejecutado. En la candidata nativa, la guarda requiere una delegación efectiva de cgroup v2 cuya disponibilidad no está acreditada por la documentación.

## 6. Estado de la evidencia en el corte revisado

| Objeto | Evidencia disponible | Pendiente o límite |
|---|---|---|
| Referencia nativa anterior | Campañas EIO-05 y EIO-06, con construcción y controles directos; resultados contractuales adversos conservados. | No acreditan el servicio nativo posterior ni suficiencia general del modelo. |
| Complemento JSON | EIO-JSON-01: 24 controles de regresión y 35 complementarios, según su recepción. | Alcance del banco y reserva de identidad de archivos efímeros; no nueva inferencia. |
| Navegador, NAV-02 | NAV01–NAV04 conformes en el Chrome identificado. NAV05 recibió las marcas posteriores a la carga del modelo y anteriores al primer `forward`. | Interrupción por RSS agregada: 4,35 GiB observados frente a 4 GiB autorizados; sin primer token ni salida contractual. No demuestra un límite intrínseco de WASM ni atribución exclusiva a Candle. |
| Custodia de NAV-02 | Informe de recuperación y cotejo de 49 archivos emitidos. | PSS incompleta del pico, coste de escritura de Node no exportado y ausencia de cierre normal documentados. |
| Candidata nativa EIO-NAT-PREP-02 | Fuentes, contrato, pruebas y diseño de supervisión y custodia publicados. | Sin compilación ni ejecución de esta candidata; lock efectivo, controles y viabilidad de la guarda pendientes. No hereda conformidad de campañas anteriores. |
| Aplicación accesible por URL para uso interactivo | Arquitecturas identificadas y componentes preparados o ensayados parcialmente. | No se acredita todavía una aplicación completa con inferencia, controles y custodia aceptados de extremo a extremo. |

Fuentes del estado, fijadas por el corte revisado:

- [Recepción del complemento JSON](https://github.com/juantoniolloretegea/SV-motor/blob/d82bbe2f5f396eb31da5b095ba0849404cb352b0/laboratorio/ensayo-ia-y-observabilidad/resultados/json-01/RECEPCION.md).
- [Resultado, método y reservas de NAV-02](https://github.com/juantoniolloretegea/SV-motor/blob/d82bbe2f5f396eb31da5b095ba0849404cb352b0/laboratorio/ensayo-ia-y-observabilidad/resultados/navegador-02/README.md).
- [Código de control de la página](https://github.com/juantoniolloretegea/SV-motor/blob/d82bbe2f5f396eb31da5b095ba0849404cb352b0/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-navegador-02/web/control.js) y [controlador exterior](https://github.com/juantoniolloretegea/SV-motor/blob/d82bbe2f5f396eb31da5b095ba0849404cb352b0/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-navegador-02/controlador.mjs).
- [Worker e identificación de recursos](https://github.com/juantoniolloretegea/SV-motor/blob/d82bbe2f5f396eb31da5b095ba0849404cb352b0/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-navegador-02/web/worker.js).
- [Candidata nativa EIO-NAT-PREP-02](https://github.com/juantoniolloretegea/SV-motor/blob/d82bbe2f5f396eb31da5b095ba0849404cb352b0/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-nativa-02/README.md) y [diseño y límites](https://github.com/juantoniolloretegea/SV-motor/blob/d82bbe2f5f396eb31da5b095ba0849404cb352b0/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-nativa-02/DISENO.md).
- [Dependencias de la vía WASM](https://github.com/juantoniolloretegea/SV-motor/blob/d82bbe2f5f396eb31da5b095ba0849404cb352b0/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-navegador-02/Cargo.toml) y [dependencias de la candidata nativa](https://github.com/juantoniolloretegea/SV-motor/blob/d82bbe2f5f396eb31da5b095ba0849404cb352b0/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-nativa-02/Cargo.toml).

Esta síntesis describe el corte indicado; no acredita modificaciones o ejecuciones posteriores. Los resultados medidos proceden de sus expedientes, no de una estimación arquitectónica.

## 7. Secuencia y organización del trabajo

1. Identificar, por vía, requisitos, controles, resultados y pendientes, conservando el contrato común.
2. Determinar con evidencia cuál permite alcanzar antes una realización factible y conforme. La existencia de código o de una compilación no basta para declararlo.
3. Completar y recibir el itinerario prioritario dentro de las autorizaciones vigentes; mantener sus fallos y limitaciones en el expediente.
4. Abordar después el segundo itinerario y comprobar sus obligaciones de forma independiente.
5. Comparar únicamente magnitudes con perímetros y métodos compatibles, sin transferir aceptación entre entornos.

Cada expediente conservará las identidades de código, entradas y entorno, los resultados esperados y observados, los recursos, la actuación de los controles y la custodia. Las guardas de una vía no habilitan la otra.

| Directorio | Contenido |
|---|---|
| [contrato/](contrato/README.md) | Perímetro, fuentes rectoras y obligaciones. |
| [inferencia/](inferencia/README.md) | Modelo, adaptación y límites de generación. |
| [observabilidad/](observabilidad/README.md) | Instrumentación, exportación y cobertura. |
| [pruebas/](pruebas/README.md) | Casos y criterios de aceptación. |
| [resultados/](resultados/README.md) | Candidatas, campañas, evidencias y reservas, diferenciadas por expediente. |

Los presupuestos iniciales y sus posteriores autorizaciones se conservan como historia. El límite aplicable a una campaña se toma de su encargo vigente; esta revisión no renueva contadores ni aumenta recursos. No se modifica el estado de S32/BIS-03 ni de los estudios pendientes S37/S38.

## 8. Antecedente documental conservado

La versión 0.2 y sus notas de continuidad se mantienen a continuación sin alterar su texto. Sus estados y expresiones temporales corresponden a los momentos documentados; la síntesis vigente de este README es la versión 2.2.1 anterior. Las revisiones del contrato y de los expedientes mantienen sus identidades propias.

<details>
<summary>Consultar íntegramente la versión 0.2 y sus notas de continuidad</summary>

# Ensayo de inteligencia artificial y observabilidad

**Versión documental:** 0.2  
**Fecha:** 18 de septiembre de 2026  
**Estado:** preparación experimental; implementación y mediciones pendientes.

## Objeto y adscripción

Este espacio reúne el diseño de un ensayo reproducible de inferencia y observabilidad dentro del dominio de trabajo del Sistema Vectorial SV. Se adscribe al bloque (p1+P3)-Bis del Lenguaje SV y aporta evidencia técnica para sus decisiones de diseño. Su apertura no constituye el motor de inteligencia artificial definitivo ni acredita los resultados de pilotos anteriores.

La pregunta experimental es si una implementación Rust puede ejecutar un modelo auxiliar acotado y registrar sus operaciones instrumentadas con un coste medible, preservando la separación entre propuesta probabilística, autorización y efecto. La calidad de la inferencia y la corrección de la frontera se evaluarán por separado.

El [contrato experimental](contrato/README.md) vincula este ensayo con el [acta de rutas de conocimiento del 14 de septiembre](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3d362a01c05644362a8bf77f456b4ecfedd8fd57/docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/ACTA_EVALUACION_Y_RECEPCION_DOCUMENTAL_RUTAS_CONOCIMIENTO_SV_2026_09_14.md) y con el [Acta 001 de continuidad](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3d362a01c05644362a8bf77f456b4ecfedd8fd57/docs/calidad/tuberias-ia/continuacion-15-09-2026/ACTA_001_CONTINUIDAD_Y_RUMBO_2026_09_15.md). La primera aporta obligaciones y reservas; la segunda conserva la secuencia y las sedes de autoridad. Este laboratorio produce evidencia para su recepción posterior: no establece un estado canónico paralelo.

## Selección experimental

| Función | Selección | Estatuto |
|---|---|---|
| Inferencia | Candle | Motor seleccionado para este ensayo; adopción definitiva no resuelta. |
| Modelo inicial | Qwen3-0.6B, cuantización Q4_K_M | Referencia de pequeña escala para comprobar el acoplamiento. |
| Observabilidad | OpenTelemetry Rust | Único sistema de instrumentación y exportación de telemetría seleccionado para el ensayo. |
| Implementación y verificación | Rust 1.98.0 | Versión que deberá comprobarse y registrarse antes de compilar. |
| Entorno inicial | Ejecución nativa de referencia en GitHub Actions y WebAssembly en navegador | Compatibilidad, reproducibilidad y costes pendientes de comprobación. |

La selección de OpenTelemetry Rust es una decisión experimental efectiva. No constituye una afirmación de suficiencia universal ni incorpora un servicio externo de recolección. La elección de una biblioteca no implica ausencia de dependencias transitivas; estas deberán identificarse en el manifiesto de construcción.

## Organización

| Directorio | Contenido |
|---|---|
| [contrato/](contrato/README.md) | Perímetro, fuentes rectoras y correspondencia entre obligaciones y pruebas. |
| [inferencia/](inferencia/README.md) | Identidad del modelo, adaptación del motor y límites de generación. |
| [observabilidad/](observabilidad/README.md) | Integración de OpenTelemetry Rust, señales, cobertura y límites. |
| [pruebas/](pruebas/README.md) | Protocolo experimental, controles negativos y criterios de aceptación. |
| [resultados/](resultados/README.md) | Evidencias de ejecución, mediciones y límites de interpretación. |

## Perímetro

El ensayo se limita a las entradas, operaciones y efectos expuestos por su propia interfaz. No introduce semántica probabilística en el núcleo del SV ni otorga autoridad a la salida del modelo. Tampoco constituye un antivirus, un sistema de detección de intrusiones o un monitor general del sistema operativo.

La instrumentación del programa no acredita por sí sola todas las escrituras, conexiones o actividades de procesos ajenos. La cobertura observada, los puntos instrumentados y las dependencias del entorno deben declararse. La variante WASI y otros sistemas operativos requieren comprobación específica; una ejecución satisfactoria en navegador no los acredita.

## Presupuesto inicial

Las cantidades siguientes son restricciones del protocolo y objetivos de aceptación; no representan consumos medidos ni controles ya implementados.

| Magnitud | Presupuesto |
|---|---|
| Pesos | Un único fichero, hasta 500 MB; sin descarga de colecciones de variantes. |
| Conversaciones simultáneas | Una. |
| Contexto retenido | Hasta 2.048 tokens, incluida la salida generada. |
| Generación | Hasta 128 tokens por caso. |
| Memoria del conjunto de procesos del ensayo | Objetivo de aceptación: hasta 4 GiB; método y cobertura de medición por declarar. |
| Paquete local conservado | Hasta 2 GiB. |
| Ocupación local adicional máxima | 5 GiB, incluidos temporales y copias atribuibles al ensayo. |
| Reserva del volumen del sistema | Al menos 30 GiB libres; comprobación previa y durante las operaciones que escriban en él. |
| Evidencia exportada | Hasta 20 MiB por ejecución, con registro explícito de truncamientos y pérdidas. |
| Ejecuciones iniciales en Actions | Hasta tres, secuenciales, con límite de 40 minutos por ejecución y sin reintentos automáticos. |

Se distinguen MB decimales, de 10⁶ bytes, y MiB/GiB binarios, de 2²⁰/2³⁰ bytes. La ocupación temporal del ejecutor de Actions y la del equipo local se contabilizan por separado. La disponibilidad efectiva del ejecutor debe medirse antes de construir. No se habilitan recursos de pago, servicios persistentes ni almacenamiento masivo.

### Estimación de memoria

El proveedor de la cuantización publica aproximadamente 397 MB para el fichero Q4_K_M. Esta cifra no equivale al consumo de memoria de la aplicación.

Para una caché convencional de claves y valores:

M_KV = 2 × L × H_KV × d × T × B × s

Con L = 28 capas, H_KV = 8 cabezas, d = 128 componentes, T = 2.048 tokens, B = 1 conversación y s = 2 bytes, el resultado es 234.881.024 bytes, equivalentes a 224 MiB. Con s = 4 bytes asciende a 448 MiB. Estas estimaciones excluyen pesos, reservas, copias y tensores temporales. El tipo numérico real de la caché debe comprobarse; cuantizar los pesos no acredita la cuantización de la caché.

## Condiciones de ejecución

Antes de ejecutar deben estar identificados el código, los pesos, el tokenizador, las dependencias, las opciones de compilación y los mecanismos de limitación y parada. Deben corregirse o descartarse de forma fundada las incompatibilidades del ejemplo de referencia relativas a finalización, formato conversacional y registro del contenido de las consultas.

La primera fase utilizará datos de prueba sin información personal ni credenciales. La inferencia no requiere acceso general a archivos, servicios ni dispositivos del equipo. Las dependencias y los pesos se obtendrán de fuentes identificadas; cualquier comunicación durante el ensayo tendrá una finalidad declarada.

Una interrupción, una pérdida de evidencia o un exceso de recursos no se presentarán como resultado satisfactorio. La imposibilidad de medir una magnitud se declarará expresamente.

## Estado de la evidencia

Esta versión contiene exclusivamente documentación preparatoria. No incluye pesos, ejecutables, dependencias instaladas ni resultados de inferencia. No acredita rendimiento, paridad entre plataformas, integridad del equipo anfitrión o suficiencia del modelo para una operación del SV.

## Referencias

- [Continuidad del bloque de diseño en el Lenguaje SV](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/tree/main/docs/calidad/tuberias-ia/continuacion-15-09-2026).
- [Candle: revisión de referencia](https://github.com/huggingface/candle/tree/ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a).
- [Configuración de Qwen3-0.6B](https://huggingface.co/Qwen/Qwen3-0.6B/blob/main/config.json).
- [Distribución de la cuantización Q4_K_M](https://huggingface.co/unsloth/Qwen3-0.6B-GGUF/blob/main/Qwen3-0.6B-Q4_K_M.gguf).
- [OpenTelemetry Rust](https://github.com/open-telemetry/opentelemetry-rust).
- [Recursos de los ejecutores de GitHub Actions](https://docs.github.com/en/actions/reference/runners/github-hosted-runners).

Las referencias móviles sirven para identificar las fuentes. Antes del ensayo deberán sustituirse en el manifiesto de ejecución por revisiones concretas y huellas verificadas de los artefactos.


## Actualización de continuidad · 20 de septiembre de 2026

La versión preparatoria anterior se conserva como antecedente. Existen ya resultados de EIO-05, EIO-06 y EIO-JSON-01. La [revisión receptora de JSON-01](resultados/json-01/RECEPCION.md) contrasta 24 controles de regresión y 35 complementarios, con una reserva explícita sobre la identidad de los archivos efímeros de evidencia. El rechazo estructural de las salidas del modelo permanece como resultado adverso. La comparación con/sin telemetría ya tiene tres pares en EIO-05; la comprobación WASM de EIO-06 no acredita ejecución en navegador. El siguiente trabajo es preparar esa ejecución, dentro del perímetro original, sin nuevas inferencias o ejecuciones autorizadas por esta nota. No se constituye una solución definitiva ni se cierra la aceptación científica.


## EIO-NAV-01 · preparación autorizada · 20/09/2026

[Paquete derivado y precompromiso](resultados/preparacion-navegador-01/README.md): una ejecución remota adicional número 8, intento 1, sin reintentos. Incluye matriz NAV-01…05, custodia original y cierre; aún no acredita ejecución en navegador. Los antecedentes y sus reservas se conservan. S38 sigue pendiente en su sede propia.


## EIO-NAV-01 · ejecución consumida · 20/09/2026

[Resultado y custodia](resultados/navegador-01/README.md): run35503596075, número8/intento1, failure92. Construcción y controles NAV01–04 conformes; NAV05 interrumpido por umbral RSS, sin salida contractual. Recuperación de39/39 archivos emitidos conforme, con laguna explícita de captura al cierre. Guardas cerradas y sin reintento. Pendiente revisión receptora.


## EIO-NAV-02 · preparación · 20/09/2026

[Diagnóstico acotado y precompromiso](resultados/preparacion-navegador-02/README.md): campaña9/intento1, umbral RSS conservado, métricas complementarias y captura incremental. No optimización del modelo ni reintento de NAV01. Preparación pendiente de observación.


## EIO-NAV-02 · diagnóstico entregado · 20/09/2026

[Resultado observado y custodia](resultados/navegador-02/README.md): run35507386447, número9/intento1, failure92 por umbral RSS conservado. NAV01–04 conformes; ModelWeights completado y marca previa al primer forward recibida, sin primer token ni salida contractual. Captura incremental conservada,49/49 identidades cotejadas; reservas de PSS final y coste de escritura explícitas. Guardas cerradas, sin reintento. Pendiente revisión receptora.

</details>
