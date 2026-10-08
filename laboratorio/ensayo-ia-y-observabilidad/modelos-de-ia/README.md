# Modelos de IA

**Edición documental 16 · 8 de octubre de 2026.**

**Corte experimental conservado de la edición 10:** 03/10/2026, 09:37 UTC. Cada expediente conserva la fecha, configuración y alcance de sus propios resultados.

La edición 11 incorpora los candidatos Kimi K3 y GLM de zai-org, el componente de cálculo y su trazabilidad. La tabla y descripción experimental de la edición 10 se conservan como antecedente fechado, sin reactivar sus instrucciones de continuación. El [índice general, con corte posterior](../README.md), y cada expediente reúnen sus sucesores. [Qwen3.5-122B-A10B Q8_0](qwen/qwen3.5-122b-a10b-q8-0/readme.md) conserva su realización CPU y su fase cerrada, con [archivo de cierre](https://github.com/juantoniolloretegea/SV-motor/releases/tag/qwen3.5-122b-a10b-q8-0-archivo-cierre-20261006-v1) y [custodia cifrada cotejada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/releases/tag/qwen35-122b-q8-imagen-cierre-20261006-v1), sin duplicar pesos. Retirada administrativa pendiente; arranque restaurado no ensayado. No se atribuye a CubeCL.

El [índice general del ensayo](../README.md) reúne el estado, la cronología de publicaciones, las versiones de los componentes y sus diagramas. Este catálogo relaciona los expedientes por modelo y separa las demostraciones públicas, las realizaciones nativas y los acoplamientos con servicios externos.

## Organización de los expedientes y modalidades de acoplamiento

La organización distingue los expedientes por modelo y las [tres modalidades de acoplamiento](acoplamientos-con-el-sv/readme.md#2-tres-nodos-experimentales). Las carpetas existentes conservan su ubicación y los enlaces ya publicados.

El [nodo 01](acoplamientos-con-el-sv/01-inferencia-bajo-control-propio/readme.md#expedientes-en-su-ubicación-de-origen) reúne la especificación de inferencia bajo control propio y remite a los expedientes de [Qwen](qwen/README.md) y [GPT-OSS/OpenAI](openai/README.md), así como a los estudios de [Kimi](kimi/kimi-k3/ESTUDIO-VIABILIDAD-20261005.md) y [Z.ai](zai-org/glm-5.3/ESTUDIO-VIABILIDAD-20261005.md). Estos documentos permanecen en sus carpetas de origen. Kimi y Z.ai se mantienen como candidatos pendientes de disponibilidad de recursos. La pertenencia a una familia no determina la modalidad: cada realización, estudio o demostración conserva su alcance y estado.

Los expedientes específicos de plataforma gestionada se organizan en el [nodo 02](acoplamientos-con-el-sv/02-plataforma-gestionada/readme.md) y los de API de proveedor en el [nodo 03](acoplamientos-con-el-sv/03-api-directa/readme.md), por modelo. Los antecedentes se enlazan desde allí sin trasladarlos ni duplicar sus resultados.

<a id="gpt-6-astra--nodo-03--cierre-del-07102026"></a>

## Recepciones documentales · Nodo 03

| Candidato y expediente | Resultado y alcance | Evidencia |
|---|---|---|
| [GPT-6 Astra · OpenAI](acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/readme.md) | **Apto para el contrato documental** P01–P25: 25 respuestas finales correctas, incluidas las 20 críticas; T(25)=19. Cierre del 07/10/2026. | [Informe y evidencias](acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/examen25-20261007/resultado/INFORME.md). |
| [Grok 4.7 · xAI](acoplamientos-con-el-sv/03-api-directa/xai/grok-4.7/readme.md) | **Apto para el contrato documental** P01–P25: 25 respuestas finales R2 correctas, incluidas las 20 críticas; T(25)=19. Cierre del 08/10/2026. | [Informe y evidencias](acoplamientos-con-el-sv/03-api-directa/xai/grok-4.7/examen25-20261008/INFORME.md) · [TT-0022](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0022.md). |
| [Qwen3.8-Max-0902 · Alibaba Cloud](acoplamientos-con-el-sv/03-api-directa/qwen/qwen3.8-max-0902/readme.md) | **Apto documental con reservas**, en el alcance de **16 preguntas**, tras revisión metodológica y réplica separada de P13. Alcance limitado por disponibilidad de recursos. | [Recepción del 08/10/2026 y límites](acoplamientos-con-el-sv/03-api-directa/qwen/qwen3.8-max-0902/RECEPCION-DOCUMENTAL-20261008.md); adjudicación contractual original conservada. |

En el **nodo 03**, el proveedor ejecuta la inferencia mediante su API y el SV administra las fuentes, el Árbitro-Director y los controles propios en Rust. En el **nodo 01**, motor y pesos se ejecutan en infraestructura administrada por el proyecto. Las carpetas históricas de Qwen y GPT-OSS conservan sus ubicaciones; cada realización mediante API tiene su expediente propio.

Los tres expedientes utilizan respuesta provisional, autocrítica y verificación final neutral (R0/R1/R2). Astra y Grok conservan la adjudicación final R2 de sus respectivos 25 casos; la recepción posterior de Qwen reúne revisión del instrumento y réplica diagnóstica, sin sustituir respuestas en el vector histórico ni crear una puntuación nueva. Las comparaciones se limitan a preguntas y condiciones comunes. La recepción documental no constituye aptitud clínica ni una prueba idéntica en instrumentación a las del nodo 01.

El [diagnóstico MD01 del manual con Grok](acoplamientos-con-el-sv/03-api-directa/xai/grok-4.7/md01-diagnostica-20261008/INFORME.md) recibió R0 y R1; R2 no se ejecutó. Es un resultado parcial independiente del examen de 25 preguntas. El [TT-0023 de Qwen](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0023.md) conserva su recepción posterior y sus reservas.

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

## Candidatos adicionales en estudio de viabilidad · 05/10/2026

| Familia o candidato | Estado documental | Estado experimental |
|---|---|---|
| [Kimi K3](kimi/kimi-k3/ESTUDIO-VIABILIDAD-20261005.md) | Se mantiene como candidato; impedimentos de memoria y realización Rust/AMD documentados | No instalado ni ensayado; sin dictamen de aptitud |
| [GLM-5.3 y GLM-5.3-Flash de zai-org](zai-org/glm-5.3/ESTUDIO-VIABILIDAD-20261005.md) | Se mantienen en estudio, con variantes separadas y reservas propias | No instalados ni ensayados; sin dictamen de aptitud |

El cierre de una búsqueda delimitada sin vía admisible inmediata no descarta definitivamente estos candidatos. Su evaluación documental se separa de las pruebas de respuestas. Qwen permanece como línea independiente en UpCloud. CubeCL y rust-gpu son medios de cálculo posibles; no reemplazan a estas familias.

## Puntuación, terna y acceso al examen

El [criterio común](CRITERIO-PUNTUACION-MODELOS-20261001.md) separa la **puntuación 100 × (aciertos − errores no críticos) / N** del dictamen SV. U y blanco no suman ni restan; un error crítico determina No apto en el alcance fijado. No se reduce N por incidencias ni se recortan puntuaciones negativas.

La regla SV utiliza **T(n)=⌊7n/9⌋** sobre un vector completo: N₁ ≥ T(n) determina No apto; en otro caso, N₀ ≥ T(n) determina Apto; los restantes casos son Indeterminados. Los requisitos de criticidad se aplican además. En la [ronda de 25 posiciones](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/6f1118a323bcefac9400d01274e4f8d1db3636ff/dominios/inmunologia/tes-examenes-conocimientos/ronda-01-pdq-tricoleucemia-20260929/PROTOCOLO.md), T(25)=19 y las veinte preguntas críticas deben estar en 0. Las incidencias técnicas y las preguntas pendientes quedan fuera de la terna; no se sustituyen por U ni permiten construir artificialmente el vector y su frame. Un error crítico acreditado conserva su efecto eliminatorio aunque otras posiciones estén impedidas.

La **Preevaluación de examen de 25 preguntas de tricoleucemia** tiene nueve casos de desarrollo y nueve casos nuevos de admisión, con respuesta inicial y un máximo de tres revisiones adversariales por caso. El **Árbitro-Director del Sistema Vectorial SV** asegura el suministro documental y organiza el **Aprendizaje por Retroalimentación del Sistema Vectorial SV** con antecedentes propios completos. No se ajustan pesos. Se decide sobre la capa 3; el bloque nuevo sólo se abre si el anterior cumple. Cada bloque requiere T(9)=7, todas las críticas en 0, nueve respuestas evaluables y controles conformes.

Al corte sólo A0 está en ejecución. El [protocolo, estado y conservación](openai/gpt-oss-safeguard-120b/ensayos-reiterados-adversariales-y-aprendizaje/readme.md) distinguen preparación, ejecución y evaluación. Los dictámenes No apto anteriores permanecen intactos; no se ha acreditado acceso al examen ni aptitud clínica.

## Componentes y estudios separados

- [CubeCL y rust-gpu: cálculo Rust para AMD](../inferencia/cubecl-evaluacion-20261005/ESTUDIO.md): nueva incorporación documental AMD-CALCULO-RUST-20261005. CubeCL recibe prioridad provisional de estudio; rust-gpu es alternativa. No son modelos ni motores completos recibidos. Limitación MFMA/LLVM en MI300X y necesidades N-C01–N-C05 pendientes; [TT-0020](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0020.md).


- [Servicio documental MCP](model-context-protocol/README.md): versiones 0.1.0, 0.1.1, 0.1.2 y [0.1.3](model-context-protocol/0.1.3/LEAME.md); acceso documental y comprobaciones propias, sin transferencia automática de conformidad al modelo.
- [Árbitro-Director del Sistema Vectorial SV](model-context-protocol/controlador-pictograma-algoritmo/DISENO.md): control del recorrido y conservación de evidencias; realización y comprobaciones en Rust. La corrección semántica requiere cotejo externo.
- [Sistema conjunto](../../sistema-conjunto-lenguaje-computacion-ia-gobernada/README.md): funciones y retorno al Lenguaje. Núcleo, semántica V0.2 e IR 0.3 permanecen intactos en esta preevaluación; las necesidades de adaptación se documentan para recepción competente.
- [Derivado comunitario de 4,8B](openai/comunidad/gpt-oss-4.8b-5-expertos/README.md): estudio documental del empaquetado; sin instalación ni inferencia acreditadas. No es una publicación oficial de OpenAI.
- [Registro de versiones](../VERSIONES.json): etiquetas, commits, versiones declaradas e identidades de las distribuciones históricas; las campañas posteriores conservan además sus manifiestos propios.

La capacidad de generar texto, la fidelidad documental, la conformidad contractual y la aptitud para cada dominio se evalúan por separado. Los resultados no se transfieren entre modelos ni se convierten en una tasa global.

[Índice Qwen](qwen/README.md) · [Índice OpenAI](openai/README.md) · [Contrato experimental](../contrato/README.md).

---

[Texto de la licencia CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es). Las licencias de terceros se identifican en las fichas y distribuciones correspondientes.

## Constancia posterior de retirada · 06/10/2026

La instancia y su disco exclusivo están retirados, con desaparición cotejada. Conservación estructural conforme; arranque restaurado no ensayado. Véase el [acta posterior](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3.5-122b-a10b-q8-0/seguimiento/cierre-20261006/ACTA-RETIRADA.md). Los cortes anteriores conservan su fecha y alcance.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
