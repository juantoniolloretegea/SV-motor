# Modelos de IA

**Edición documental 12 · 6 de octubre de 2026.**

**Corte experimental conservado de la edición 10:** 03/10/2026, 09:37 UTC. Cada expediente conserva la fecha, configuración y alcance de sus propios resultados.

La edición 11 incorpora los candidatos Kimi K3 y GLM de zai-org, el componente de cálculo y su trazabilidad. La tabla y descripción experimental de la edición 10 se conservan como antecedente fechado, sin reactivar sus instrucciones de continuación. El [índice general, con corte posterior](../README.md), y cada expediente reúnen sus sucesores. [Qwen3.5-122B-A10B Q8_0](qwen/qwen3.5-122b-a10b-q8-0/readme.md) conserva su realización CPU y su fase cerrada, con [archivo de cierre](https://github.com/juantoniolloretegea/SV-motor/releases/tag/qwen3.5-122b-a10b-q8-0-archivo-cierre-20261006-v1) y [custodia cifrada cotejada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/releases/tag/qwen35-122b-q8-imagen-cierre-20261006-v1), sin duplicar pesos. Retirada administrativa pendiente; arranque restaurado no ensayado. No se atribuye a CubeCL.

El [índice general del ensayo](../README.md) reúne el estado, la cronología de publicaciones, las versiones de los componentes y sus diagramas. Este catálogo mantiene una entrada por modelo y separa la demostración pública de las realizaciones nativas.

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

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
