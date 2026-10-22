# Modelos de IA

**Edición documental 25 · 22 de octubre de 2026.**

Este catálogo identifica los modelos estudiados, las configuraciones efectivamente ensayadas y los candidatos de la convocatoria actual. Cada resultado conserva su banco documental, modalidad de inferencia, fecha, condiciones y reservas. Una familia comercial no recibe por extensión el resultado de una versión concreta.

El [ensayo de inteligencia artificial y observabilidad](../README.md) presenta el método, la arquitectura y sus límites. Este catálogo desarrolla las fichas de los modelos; las herramientas auxiliares y las pruebas instrumentales se reúnen en un apartado propio.

**Consulta:** [organización](#organización-de-los-expedientes-y-modalidades-de-acoplamiento) · [ocho candidatos](#ocho-candidatos-de-la-proxima-comparacion) · [configuraciones anteriores](#configuraciones-anteriores) · [criterio](#puntuación-terna-y-acceso-al-examen) · [herramientas y pruebas instrumentales](#herramientas-auxiliares-y-ensayos-instrumentales) · [antecedentes fechados](#antecedentes-fechados).

El [estándar comparativo](estandar-comparativo-o-benchmark/readme.md) fija las reglas vigentes y el turno de examen: primero Claude, Gemini, GPT-6 Astra y Qwen; después DeepSeek, GLM, Grok y Kimi en la categoría adaptada al 33 %. Dentro de cada grupo el orden es alfabético. Las tablas del catálogo son índices de consulta, no clasificaciones por resultados.

## Organización de los expedientes y modalidades de acoplamiento

El [nodo 01](acoplamientos-con-el-sv/01-inferencia-bajo-control-propio/readme.md) exige inferencia dentro del entorno del SV y auditabilidad de extremo a extremo; el [nodo 02](acoplamientos-con-el-sv/02-plataforma-gestionada/readme.md) corresponde al acceso organizado por una plataforma gestionada; el [nodo 03](acoplamientos-con-el-sv/03-api-directa/readme.md), al servicio externo del proveedor. La ubicación física de un servidor o la presencia de una API no bastan para acreditar las condiciones del nodo 01. El [diagrama y las definiciones del ensayo](../README.md#tres-nodos-de-acoplamiento) precisan el control y la observabilidad de cada modalidad.

La modalidad de ejecución se consulta en las [rutas y el diagrama del ensayo](../README.md#vías-de-ejecución-y-diagramas): **A**, inferencia en navegador probada con interrupción antes del primer token; **B**, inferencia nativa en el servidor; **servicio externo**, inferencia gobernada por la plataforma o el proveedor. El visor Rust/egui en WebAssembly presenta resultados y no ejecuta el modelo evaluado. La convocatoria de ocho candidatos corresponde a la evaluación por API del nodo 03; conserva por separado el antecedente de Opus en el nodo 02.

Los expedientes conservan sus carpetas y referencias. Un estudio de viabilidad, una ejecución técnica y un examen de respuestas son objetos distintos. Un resultado previo no habilita una nueva entrega de documentos ni el uso en producción.

<a id="siete-candidatos-de-la-proxima-comparacion"></a>
<a id="ocho-candidatos-de-la-proxima-comparacion"></a>

## Ocho candidatos de la convocatoria actual

La selección se presenta alfabéticamente como índice de consulta. Cuatro candidatos han completado la edición general y tres la adaptada: DeepSeek, GLM y Grok. Kimi permanece pendiente. La clasificación por resultados y las evidencias se consultan en el estándar comparativo. Cada nueva ejecución exige condiciones propias vigentes y autorización de entrega y lanzamiento; un resultado anterior no las sustituye.

| Candidato | Identidad del servicio | Estado de esta convocatoria |
|---|---|---|
| [Claude Opus 5.5](#claude-opus-55) | `claude-opus-5-5` | General: [examen corregido](estandar-comparativo-o-benchmark/resultados/claude-opus-5.5/cyb16-085bea8caae0/resultado.json), 16/16, dictamen 0. |
| [DeepSeek-V4.1-Flash-1017](#deepseek-v41-flash-1017) | `deepseek-flash`; revisión declarada `DeepSeek-V4.1-Flash-1017` | Adaptada al 33 %: [examen corregido](estandar-comparativo-o-benchmark/resultados/deepseek-v4.1-flash-1017/cyb16-1512ff18a4f2/resultado.json), 16/16, dictamen 0. |
| [Gemini 4 Argon](#gemini-4-argon) | `gemini-4-argon` | General: [examen corregido](estandar-comparativo-o-benchmark/resultados/gemini-4-argon/cyb16-67947a76804a/resultado.json), 16/16, dictamen 0. |
| [GLM-5.3](#glm-53) | `glm-5.3` | Adaptada al 33 %: [examen corregido](estandar-comparativo-o-benchmark/resultados/glm-5.3/cyb16-93abf5360f9e/resultado.json), 16/16, dictamen 0. |
| [GPT-6 Astra](#gpt-6-astra) | `gpt-6-astra` | General: [examen corregido](estandar-comparativo-o-benchmark/resultados/gpt-6-astra/cyb16-42214abb1502/resultado.json), 16/16, dictamen 0. |
| [Grok 4.7](#grok-47) | `grok-4.7` | Adaptada al 33 %: [examen corregido](estandar-comparativo-o-benchmark/resultados/grok-4.7/cyb16-8466519c3c7d/resultado.json), 16/16, dictamen 0. |
| [Kimi K3](#kimi-k3) | `kimi-k3` | Adaptada al 33 %: pendiente, sin dictamen. |
| [Qwen3.8-Max-0902](#qwen38-max-0902) | `qwen3.8-max-0902` | General: [examen corregido](estandar-comparativo-o-benchmark/resultados/qwen3.8-max-0902/cyb16-c845bb554c19/resultado.json), 16/16, dictamen 0. |

La capacidad y la configuración deben corresponder a la edición exacta: longitud de entrada, salida y razonamiento, tiempos, adaptación del protocolo y cotas. Cuota gratuita, saldo monetario y límite administrativo son conceptos distintos. Las fichas siguientes distinguen los antecedentes y el estado de la convocatoria actual.

<a id="claude-opus-55"></a>

<details>
<summary><strong>Claude Opus 5.5 · Anthropic</strong></summary>

[Expediente y fuentes](acoplamientos-con-el-sv/02-plataforma-gestionada/antrophic/claude-opus-5-5/readme.md).

El antecedente se ejecutó mediante **Kaggle Benchmarks Model Proxy**, nodo 02, con el identificador `anthropic/claude-opus-5-5@default`. El sufijo del servicio no identifica por sí solo unos pesos inmutables. El manual MD09 obtuvo nueve respuestas finales correctas, incluidas las seis críticas, según adjudicación asistida. El contraste CYB09 terminó con **8/9 en R2, 88,89/100 y No apto asistido por C02 crítico**; R1 9/9 permanece como etapa previa y no reemplaza la final. La recepción científica independiente sigue pendiente.

En la convocatoria EET-CYB16, **Opus por API directa, nodo 03**, ha completado la edición general con dieciséis aciertos y dictamen 0. Su [resultado y evidencias](estandar-comparativo-o-benchmark/resultados/claude-opus-5.5/cyb16-085bea8caae0/) conservan las condiciones propias de esta ejecución. El antecedente de Kaggle y las condiciones de Claude Sonnet permanecen separados.

[Informe CYB09](acoplamientos-con-el-sv/02-plataforma-gestionada/antrophic/claude-opus-5-5/ensayo-cyb09-20261010/entrega/INFORME.md) · [Polígono Rust/egui](acoplamientos-con-el-sv/02-plataforma-gestionada/antrophic/claude-opus-5-5/ensayo-cyb09-20261010/entrega/poligono-egui/POLIGONO-EGUI.html) · [Recepción del nodo 02](acoplamientos-con-el-sv/02-plataforma-gestionada/antrophic/claude-opus-5-5/ensayo-cyb09-20261010/entrega/MATRIZ-RECEPCION-NODO02.md).

</details>

<a id="deepseek-v41-flash-1017"></a>

<details>
<summary><strong>DeepSeek-V4.1-Flash-1017 · DeepSeek</strong></summary>

La conexión utiliza `deepseek-flash`, correspondiente por declaración del proveedor a `DeepSeek-V4.1-Flash-1017`. La correspondencia identifica la preparación; no demuestra ejecución interna ni conserva automáticamente esa identidad ante futuras revisiones del alias.

Participa en la categoría adaptada a Markdown, después de los cuatro candidatos de la categoría general. Su limitación documental se declara para esta prueba y configuración. El examen de esta edición obtuvo dieciséis aciertos y dictamen 0; conserva su identidad, respuesta y mediciones en el estándar comparativo. No acredita lectura de los formatos sustituidos.

La [edición común](estandar-comparativo-o-benchmark/readme.md) conserva las dieciséis preguntas, sus soluciones y los originales. La inclusión no autoriza contenido, exámenes ni decisiones económicas.

</details>

<a id="gemini-4-argon"></a>

<details>
<summary><strong>Gemini 4 Argon · Google</strong></summary>

Gemini 4 Argon ha completado la edición general EET-CYB16 por API del nodo 03, con dieciséis aciertos y dictamen 0. El [resultado y sus evidencias](estandar-comparativo-o-benchmark/resultados/gemini-4-argon/cyb16-67947a76804a/) identifican las condiciones y la entrega conservada.

El dictamen corresponde a esta edición y configuración; no constituye recomendación oficial, habilitación productiva ni acreditación clínica.

</details>

<a id="glm-53"></a>

<details>
<summary><strong>GLM-5.3 · Z.ai</strong></summary>

[Expediente y fuentes](acoplamientos-con-el-sv/03-api-directa/zai/glm-5.3/readme.md).

La realización por API pertenece al nodo 03. El **CYB16 del 08/10/2026** obtuvo 16/16 respuestas finales R2 correctas, incluidas las catorce críticas, y **Apto para el contrato documental**. La recepción científica independiente permanece pendiente; el resultado no acredita un agente operativo.

La convocatoria actual EET-CYB16 tiene un resultado propio de la edición adaptada: dieciséis aciertos y dictamen 0. La [revisión común de correcciones](estandar-comparativo-o-benchmark/REVISION-CORRECCIONES-20261022.md) identifica el defecto de una explicación, la rectificación formal y el antecedente conservado; no hubo otra entrega al proveedor.

El manual MD07 conserva **No apto contractual bajo reserva metodológica**, sin recalificación silenciosa. Los estudios de GLM-5.3 y GLM-5.3-Flash para infraestructura propia tienen otro alcance: no son ejecuciones de esta API ni reciben sus resultados.

[Informe CYB16](acoplamientos-con-el-sv/03-api-directa/zai/glm-5.3/cyb16-20261008/INFORME.md) · [Reserva MD07](acoplamientos-con-el-sv/03-api-directa/zai/glm-5.3/manual-20261008/revision-md07/REVISION-METODOLOGICA.md) · [TT-0024](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0024.md).

</details>

<a id="gpt-6-astra"></a>
<a id="gpt-6-astra--nodo-03--cierre-del-07102026"></a>

<details>
<summary><strong>GPT-6 Astra · OpenAI</strong></summary>

[Expediente y fuentes](acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/readme.md).

El expediente comprende distintos bancos, conservados por separado. **PDQ25, 07/10/2026:** 25 respuestas finales R2 correctas, incluidas las veinte críticas, y Apto para su contrato documental. **CYB16, 08/10/2026:** 16/16 correctas en R0, R1 y R2, catorce críticas correctas y Apto para el contrato documental de ciberseguridad. El CYB16 conserva una interrupción instrumental y su resolución; no se convierte en error del candidato. El manual MD09, su reserva temporal MD01 y la réplica diagnóstica tienen identidades propias; la ficha enlaza también el contraste posterior CYB09.

El antecedente CYB16 usó Responses mediante autorización ChatGPT. La convocatoria actual conserva una recepción propia de la API de Astra: su [edición general EET-CYB16](estandar-comparativo-o-benchmark/resultados/gpt-6-astra/cyb16-42214abb1502/) obtuvo dieciséis aciertos y dictamen 0. Conservar el modelo no hace idénticos el servicio, la instrumentación o las condiciones de sus antecedentes. **GPT-6.1 Sol es un modelo distinto, ajeno a esta selección** y sus condiciones no sustituyen a las de Astra. El proveedor conserva la ejecución interna; los canales y contadores recibidos no acreditan observación de su razonamiento interno.

[Informe PDQ25](acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/examen25-20261007/resultado/INFORME.md) · [Informe CYB16](acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/cyb16-20261008/INFORME.md) · [TT-0021](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0021.md).

</details>

<a id="grok-47"></a>

<details>
<summary><strong>Grok 4.7 · xAI</strong></summary>

La convocatoria EET-CYB16 conserva dieciséis aciertos y dictamen 0 en la edición adaptada. La [segunda revisión común](estandar-comparativo-o-benchmark/REVISION-CIERRE-AUXILIAR-20261022.md) documenta la rectificación del rechazo formal por una llave ausente en una explicación; el original y el antecedente se conservan, sin repetir el examen.

[Expediente y fuentes](acoplamientos-con-el-sv/03-api-directa/xai/grok-4.7/readme.md).

El examen **PDQ25 del 08/10/2026** terminó con 25 respuestas finales R2 correctas, incluidas las veinte críticas, y **Apto para el contrato documental**. Se conservan el error crítico P02 de R0 y su corrección en las etapas posteriores; no se elige retrospectivamente una respuesta de otra etapa.

El diagnóstico MD01 del manual es independiente: recibió R0 y R1, pero **R2 no se ejecutó**. Es un resultado parcial, no una indeterminación del modelo ni un examen completo. La nueva edición requiere comprobar sus límites de salida y razonamiento y su recepción específica.

[Informe PDQ25](acoplamientos-con-el-sv/03-api-directa/xai/grok-4.7/examen25-20261008/INFORME.md) · [Diagnóstico MD01](acoplamientos-con-el-sv/03-api-directa/xai/grok-4.7/md01-diagnostica-20261008/INFORME.md) · [TT-0022](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0022.md).

</details>

<a id="kimi-k3"></a>

<details>
<summary><strong>Kimi K3 · Moonshot AI</strong></summary>

[Expediente y fuentes](acoplamientos-con-el-sv/03-api-directa/kimi/kimi-k3/manual-20261008/INFORME.md).

El catálogo **MD09 del 08/10/2026**, ejecutado por API en el nodo 03, terminó con **8/9 respuestas correctas, seis críticas correctas y Apto para el contrato documental**. MD04 contiene un error no crítico que permanece en R0, R1 y R2. El diagnóstico posterior descartó un defecto de suministro o reconstrucción que explicase ese error; no recalificó el vector ni repitió la inferencia. La revisión del sentido fue exterior y asistida por IA.

**CYB16 sigue sin iniciar.** La configuración histórica del manual, la recepción preparada para la nueva ronda y el estudio de viabilidad de Kimi bajo control propio son objetos distintos. El examen API acredita únicamente el contrato documental ensayado; la instalación bajo control propio conserva sus requisitos de arquitectura y recursos.

[Informe MD09](acoplamientos-con-el-sv/03-api-directa/kimi/kimi-k3/manual-20261008/INFORME.md) · [Diagnóstico de MD04](acoplamientos-con-el-sv/03-api-directa/kimi/kimi-k3/manual-20261008/adendas/revision-md04-20261009/INFORME.md) · [Estudio de instalación propia](kimi/kimi-k3/ESTUDIO-VIABILIDAD-20261005.md).

</details>

<a id="qwen38-max-0902"></a>

<details>
<summary><strong>Qwen3.8-Max-0902 · Alibaba Cloud / QwenCloud</strong></summary>

[Expediente y fuentes](acoplamientos-con-el-sv/03-api-directa/qwen/qwen3.8-max-0902/readme.md).

La recepción del **08/10/2026** establece **Apto documental con reservas en un alcance de dieciséis preguntas** del banco de inmunología. Conserva el **No apto contractual original bajo reserva metodológica**, el vector histórico y la réplica separada de P13. La recepción posterior no sustituye respuestas ni crea una puntuación nueva. Las reservas P02, P11 y P16 siguen documentadas. Este alcance no es el nuevo EET-CYB16 ni equivale al examen completo de veinticinco preguntas.

La conexión abreviada `qwen3.8-max` y la revisión `qwen3.8-max-0902` deben conservar su correspondencia fechada. La diferencia de nombre no prueba por sí sola que sean modelos distintos; tampoco permite aceptar futuras revisiones del alias como si fueran la misma. La admisión debe fijar la revisión efectiva y sus condiciones, incluidas las cuotas aplicables, antes de cualquier entrega.

La convocatoria actual utilizó la identidad efectiva `qwen3.8-max-0902` y conserva un [resultado propio de la edición general EET-CYB16](estandar-comparativo-o-benchmark/resultados/qwen3.8-max-0902/cyb16-c845bb554c19/): dieciséis aciertos y dictamen 0. Este resultado es independiente del antecedente de inmunología y no lo modifica.

[Recepción y reservas](acoplamientos-con-el-sv/03-api-directa/qwen/qwen3.8-max-0902/RECEPCION-DOCUMENTAL-20261008.md) · [TT-0023](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0023.md).

</details>

<a id="primer-expediente-ejecutado--nodo-02"></a>
<a id="recepciones-documentales--nodo-03"></a>

Los antecedentes del nodo 02 y las recepciones del nodo 03 están integrados en sus fichas, conservando su procedencia. Las etapas R0/R1/R2 de respuesta no designan las fases de desarrollo nuclear. La comparación sólo procede entre contratos y condiciones comparables; las revisiones diagnósticas permanecen separadas de las respuestas originales.

## Configuraciones anteriores

<details>
<summary>Consultar resultados, cierres y fuentes de las configuraciones anteriores</summary>

| Modelo o configuración | Resultado y alcance | Situación documental |
|---|---|---|
| [Qwen3-0.6B · Q4_K_M](qwen/qwen3-0.6b/README.md) | Inferencia nativa y controles parciales; cuatro consultas DOC-01 terminadas sin conformidad contractual completa. | Campaña cerrada con limitaciones. |
| [GPT-OSS-20B · MXFP4](openai/gpt-oss-20b/README.md) | Funcionamiento técnico y bancos documentales conservados; la evaluación médica no acreditó la función prevista. | Configuración excluida de esa selección; archivo conservado. |
| [Qwen3.8-27B](qwen/qwen3.8-27b/README.md) | Respuesta completa en la selección mínima, con omisión material y deficiencias de citas y localización. | **No apto** en la selección examinada. |
| [GPT-OSS-120B](openai/gpt-oss-120b/README.md) | El estudio preliminar no llegó a inferencia: los pesos superaban la cota de la configuración considerada. La [demostración pública posterior](openai/gpt-oss-120b/prueba-playground/readme.md) obtuvo −100/100 en tres respuestas relacionadas; no constituye un examen completo ni mide recursos de una instalación propia. | Viabilidad nativa pendiente; [TT-0015](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0015.md). Ensayo 1 público abierto, con impedimentos de servicio conservados. |
| [Qwen3-Next-80B-A3B-Instruct](qwen/qwen3-next-80b-a3B-instruct/REGISTRO-INSTALACION-20260929.md) | Dos consultas MCP recibidas. Examen posterior: **28/100; No apto**. Diecinueve finales: siete aciertos, dos errores críticos y diez U; seis impedimentos técnicos. | Examen y diagnóstico terminados; [puntuación](qwen/qwen3-next-80b-a3B-instruct/tests-y-pruebas-efectuadas/PUNTUACION-FINAL-20261001.md), [archivo y retirada](qwen/qwen3-next-80b-a3B-instruct/tests-y-pruebas-efectuadas/ARCHIVO-Y-RETIRADA-20261001.md). Recepción científica independiente pendiente; [TT-0016](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0016.md). |
| [Qwen3-Next-80B-A3B-Thinking](qwen/qwen3-next-80b-a3b-thinking) | Examen cerrado: **No apto en las condiciones evaluadas por demoras operativas excesivas y falta de finalización fiable**. Nueve finales pendientes de adjudicación de contenido, cuatro impedimentos, P14 incompleta y once no ejecutadas; sin puntuación global. | [Cierre público](https://github.com/juantoniolloretegea/SV-motor/releases/tag/qwen3-next-80b-a3b-thinking-archivo-cierre-20261005-v1) y [conservación privada cotejada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/releases/tag/thinking-imagen-cierre-20261005-v1). [Instancia y almacenamiento retirados](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/2ddf814bb039b0c7876745023ea165baf38ed6f9/respuestas-ejecucion/QWEN80-THINKING-Q4K-ONECLOUD-20260930/entrega-03/retirada-20261005/ACTA-RETIRADA.md); no hay arranque restaurado ensayado. [TT-0017](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0017.md). |
| [GPT-OSS-Safeguard-120B](openai/gpt-oss-safeguard-120b/ensayos-reiterados-adversariales-y-aprendizaje/readme.md) | Contrastes anteriores separados: **87,5/100**, **66,67/100** y **50/100**, con **No apto** en sus respectivos alcances. El diagnóstico D01 reprodujo el error crítico del último contraste. | Preevaluación terminada: A0–A3 sin mejora, −88,89/100 según la rúbrica completa y **No apto para acceder al examen**. Diagnósticos e incidencias separados en el [cierre](openai/gpt-oss-safeguard-120b/ensayos-reiterados-adversariales-y-aprendizaje/resultados/cierre-20261004/INFORME-FINAL.md). Sin aptitud clínica acreditada. [Expediente v3](openai/gpt-oss-safeguard-120b/tests-y-pruebas-efectuadas/ARBITRO-SV-SAFEGUARD-V3-20261002.md) · [TT-0018](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0018.md). |
| [Qwen3.5-122B-A10B · Q8_0](qwen/qwen3.5-122b-a10b-q8-0/readme.md) | Realización CPU en UpCloud: 48 CPU y 256 GB nominales de RAM, sin GPU. Fase A0 concluida: seis 0, un 1 crítico en A06 y dos casos no ejecutados. | [Admisión no acreditada por impedimento temporal](https://github.com/juantoniolloretegea/SV-motor/blob/0b5104c4658bc98a9612fd2a97d7b8a5214b63cc/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3.5-122b-a10b-q8-0/preevaluacion-20261004/resultados/cierre-r1/INFORME-FINAL.md); sin revisiones, B ni examen. Custodia de fase y [imagen de instalación cotejadas](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/releases/tag/qwen35-122b-q8-imagen-cierre-20261006-v1); [archivo público](https://github.com/juantoniolloretegea/SV-motor/releases/tag/qwen3.5-122b-a10b-q8-0-archivo-cierre-20261006-v1). [Instancia y disco retirados el 06/10/2026](qwen/qwen3.5-122b-a10b-q8-0/seguimiento/cierre-20261006/ACTA-RETIRADA.md); arranque restaurado no ensayado. Recepción independiente de A04–A07 y fase pendiente. [TT-0019](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0019.md). |

</details>


<details>
<summary>Estudios de viabilidad bajo control propio</summary>

[Kimi K3](kimi/kimi-k3/ESTUDIO-VIABILIDAD-20261005.md), [GLM-5.3 y GLM-5.3-Flash](zai-org/glm-5.3/ESTUDIO-VIABILIDAD-20261005.md) conservan estudios de arquitectura, memoria y realización Rust/AMD del 05/10/2026. Sus impedimentos de instalación no anulan los ensayos API posteriores ni equivalen a un dictamen sobre respuestas. El [derivado comunitario GPT-OSS de 4,8B](openai/comunidad/gpt-oss-4.8b-5-expertos/README.md) conserva un estudio documental de empaquetado, sin instalación ni inferencia acreditadas; no es una publicación oficial de OpenAI.

</details>


## Puntuación, terna y acceso al examen

El [criterio común](CRITERIO-PUNTUACION-MODELOS-20261001.md) fija la terna vigente **0: correcto; 1: error; U: indeterminación**, la criticidad y el umbral **T(n)=⌊7n/9⌋**. El [ensayo](../README.md#evaluación-documental-y-resultado) explica su aplicación al vector completo y su **único polígono SV**, de dimensión correspondiente al suceso. La puntuación descriptiva sobre 100 y el área de la figura no sustituyen al dictamen. Los fallos técnicos y las posiciones pendientes se registran fuera de la terna; no se completan con 1 o U.

La preevaluación histórica de Safeguard terminó sin admisión al examen; el [cierre](openai/gpt-oss-safeguard-120b/ensayos-reiterados-adversariales-y-aprendizaje/resultados/cierre-20261004/INFORME-FINAL.md) y su [recorrido](../README.md#preevaluación-safeguard-recorrido-histórico-acotado) conservan la preparación y el resultado efectivo. Los estados de preparación anteriores se conservan con su fecha y no reactivan la campaña cerrada.

## Herramientas auxiliares y ensayos instrumentales

Estas contribuciones tienen finalidad propia y **no pertenecen a la selección de ocho candidatos ni a su clasificación de aptitud**.

<details>
<summary><strong>Helsinki-NLP · traducción auxiliar del Gestor Documental</strong></summary>

[OPUS-MT en–es](helsinki-nlp/opus-mt-en-es.md) es una herramienta auxiliar de traducción. **No pertenece a ningún nodo de inferencia experta**: no proporciona el consejo que se somete al experto del dominio. Su configuración, activación y calidad de traducción tienen controles propios. Una traducción no sustituye al original ni acredita fidelidad jurídica, aptitud clínica o conformidad del Núcleo. Las pruebas técnicas documentadas no convierten esta herramienta en un candidato a los exámenes.

</details>

<details>
<summary><strong>Qwen3-8B · contraste instrumental CPU/GPU</strong></summary>

La [prueba de Qwen3-8B Q4_K_M en CPU y GPU](qwen/qwen3-8b-cuantizacion-q4_k_m-gguf/sv-contraste-qwen-cpu-gpu-rust-20261014.md) utiliza el modelo como carga instrumental para comprobar la ejecución y la selección de recursos. Su objeto es el recorrido de cálculo y las mediciones, no la aptitud para aconsejar al experto. El uso de Kaggle como infraestructura de esta prueba no la convierte en el expediente de acceso gestionado de Opus ni la incorpora a un nodo de inferencia experta. Tampoco se identifica con Qwen3.8-Max-0902 ni recibe sus resultados.

</details>


## Componentes y estudios separados

- [Servicio documental MCP](model-context-protocol/README.md) y [versión 0.1.3](model-context-protocol/0.1.3/LEAME.md): suministro y comprobaciones documentales con identidad propia.
- [Árbitro-Director](model-context-protocol/controlador-pictograma-algoritmo/DISENO.md): control del recorrido y conservación de evidencias en Rust; el cotejo semántico conserva su autoridad y procedencia.
- [CubeCL y rust-gpu](../inferencia/cubecl-evaluacion-20261005/ESTUDIO.md): estudios de cálculo para AMD, distintos de los modelos y de la recepción de un motor completo; [TT-0020](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0020.md).
- [Funciones del conjunto](../README.md#componentes-y-funciones-del-conjunto) y [registro de versiones](../VERSIONES.json): función, identidad y cobertura de los componentes, sin atribuir su presencia a todas las configuraciones.
- [Sistema conjunto](../../sistema-conjunto-lenguaje-computacion-ia-gobernada/README.md) y [secuencia de continuidad](../README.md#preparación-examen-y-continuidad): relación con el Lenguaje y condiciones del retorno al desarrollo.

## Antecedentes fechados

Los desplegables conservan los estados publicados en su fecha. Sus instrucciones de continuación y expresiones de trabajo en curso pertenecen a ese momento; el resultado posterior se consulta en las fichas y cierres anteriores.

<details>
<summary>Corte experimental conservado · edición 10, 03/10/2026</summary>

## Antecedente experimental de la edición 10 · 03/10/2026

La tabla y el estado de ejecución que siguen se conservan con su fecha histórica. No describen el estado actual de los candidatos del nodo 03 ni reanudan campañas anteriores.
| Secuencia del estudio | Modelo y ficha | Resultado y situación |
|---|---|---|
| 1 | [Qwen3-0.6B · Q4_K_M](qwen/qwen3-0.6b/README.md) | Campaña cerrada como realización parcial; distribución 0.1.3-beta.1 conservada. |
| 2 | [GPT-OSS-20B · MXFP4](openai/gpt-oss-20b/README.md) | Campaña cerrada; configuración excluida de la función médica prevista. |
| 3 | [Qwen3.8-27B](qwen/qwen3.8-27b/README.md) | **No apto** en la selección examinada; seguimiento y archivo de cierre conservados. |
| 4 | [GPT-OSS-120B](openai/gpt-oss-120b/README.md) | Estudio nativo preliminar desfavorable para la cota considerada, sin inferencia ni máximo de memoria medido; [TT-0015](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0015.md). [Demostración pública](openai/gpt-oss-120b/prueba-playground/readme.md): tres errores en tres respuestas relacionadas, **−100/100** descriptivo, sin vector SV completo. Ensayo 1 público abierto, con impedimentos conservados y sin resultado adversarial final. |
| 5 | [Qwen3-Next-80B-A3B-Instruct · UQFF Q4K](qwen/qwen3-next-80b-a3B-instruct/REGISTRO-INSTALACION-20260929.md) | Dos consultas documentales completas recibidas. Examen posterior: **28/100; No apto**. Siete aciertos, dos errores críticos, diez U y seis impedimentos técnicos. [Puntuación](qwen/qwen3-next-80b-a3B-instruct/tests-y-pruebas-efectuadas/PUNTUACION-FINAL-20261001.md) · [Archivo y retirada](qwen/qwen3-next-80b-a3B-instruct/tests-y-pruebas-efectuadas/ARCHIVO-Y-RETIRADA-20261001.md) · [TT-0016](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0016.md). Recepción científica independiente pendiente. |
| 6 | [Qwen3-Next-80B-A3B-Thinking · UQFF Q4K](qwen/qwen3-next-80b-a3b-thinking) | [Primera consulta documental publicada](qwen/qwen3-next-80b-a3b-thinking/REGISTRO-INSTALACION-20260930.md), tras 5516,496 segundos; dos oraciones donde se exigía una y modificación de saltos de línea. Pico de 59 GiB, sin intercambio ni agotamiento, sin margen estable demostrado. Recepción independiente pendiente; el examen posterior tiene encargo propio, sin entrega final publicada al corte. [TT-0017](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0017.md). |
| 7 | [GPT-OSS-Safeguard-120B](openai/gpt-oss-safeguard-120b/ensayos-reiterados-adversariales-y-aprendizaje/readme.md) | Contrastes anteriores separados: **87,5/100**, **66,67/100** y **50/100**, con **No apto** en sus respectivos alcances; D01 reprodujo el último error crítico. [Expediente v3](openai/gpt-oss-safeguard-120b/tests-y-pruebas-efectuadas/ARBITRO-SV-SAFEGUARD-V3-20261002.md) · [TT-0018](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0018.md). Nueva preevaluación con retroalimentación: A0 en ejecución, sin puntuación ni dictamen de capa. |

Las vías de ejecución pertenecen a configuraciones concretas. La vía A requiere inferencia en el navegador mediante WebAssembly; una interfaz web conectada a un proceso nativo corresponde a B. La fase en curso utiliza Safeguard en vía B. Thinking conserva su realización independiente. A queda diferida hasta Apto experimental nativo y autorización específica. La demostración pública del GPT-OSS-120B ordinario no acredita recursos ni funcionamiento de una instalación propia, y sus resultados no se atribuyen a Safeguard.

</details>

<details>
<summary>Estudios de viabilidad · 05/10/2026</summary>

## Candidatos adicionales en estudio de viabilidad · 05/10/2026

| Familia o candidato | Estado documental | Estado experimental |
|---|---|---|
| [Kimi K3](kimi/kimi-k3/ESTUDIO-VIABILIDAD-20261005.md) | Se mantiene como candidato; impedimentos de memoria y realización Rust/AMD documentados | No instalado ni ensayado; sin dictamen de aptitud |
| [GLM-5.3 y GLM-5.3-Flash de zai-org](zai-org/glm-5.3/ESTUDIO-VIABILIDAD-20261005.md) | Se mantienen en estudio, con variantes separadas y reservas propias | No instalados ni ensayados; sin dictamen de aptitud |

El cierre de una búsqueda delimitada sin vía admisible inmediata no descarta definitivamente estos candidatos. Su evaluación documental se separa de las pruebas de respuestas. Qwen permanece como línea independiente en UpCloud. CubeCL y rust-gpu son medios de cálculo posibles; no reemplazan a estas familias.

</details>


## Constancia posterior de retirada · 06/10/2026

La instancia y el disco exclusivo de Qwen3.5-122B-A10B Q8_0 están retirados. La [constancia posterior](qwen/qwen3.5-122b-a10b-q8-0/seguimiento/cierre-20261006/ACTA-RETIRADA.md) conserva la comprobación y la custodia; no acredita un arranque restaurado. Los cortes anteriores mantienen su fecha y alcance.

<details>
<summary>Edición documental 19 · 19/10/2026</summary>

Sincronización con el ensayo 2.32: ocho candidatos y antecedentes diferenciados, inclusión de CYB16 de Astra y MD09 de Kimi, conservación de Kaggle y de las reservas de Qwen, y separación de Helsinki-NLP y la prueba CPU/GPU. Los resultados originales no se recalifican.

[Edición 18 íntegra](https://github.com/juantoniolloretegea/SV-motor/blob/0d01e72302e9006eb747c4a1fe7da8c7a4822156/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/README.md).

</details>


<details>
<summary>Edición documental 20 · 19/10/2026</summary>

Sincroniza las modalidades de ejecución con el ensayo 2.33: navegador, proceso nativo y servicio externo, con visualización independiente. Los ocho candidatos, sus identidades, resultados y reservas permanecen sin cambios. Helsinki-NLP y Qwen3-8B conservan su condición auxiliar e instrumental, respectivamente.

[Edición 19 íntegra](https://github.com/juantoniolloretegea/SV-motor/blob/9c6390848d6305d76667d07c16e0bf0389772b60/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/README.md).

</details>

[Índice Qwen](qwen/README.md) · [Índice OpenAI](openai/README.md) · [Contrato experimental histórico](../contrato/README.md).

---

[Texto de la licencia CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es). Las licencias de terceros se identifican en las fichas y distribuciones correspondientes.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
