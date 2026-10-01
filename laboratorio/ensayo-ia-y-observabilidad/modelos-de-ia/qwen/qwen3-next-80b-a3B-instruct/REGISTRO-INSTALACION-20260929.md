# Registro instrumental de instalación: Qwen3-Next-80B-A3B-Instruct Q4K

## Estado vigente · Diagnóstico P05–P07, 01/10/2026

El diagnóstico contemporáneo completó cuatro celdas y conservó sus primeras respuestas: P07-A U, P07-B 0, P05-A U y P05-B 0, como propuesta pendiente de recepción independiente. A realizó búsquedas vacías sin lectura; B recibió previamente la página fijada mediante una llamada real del conductor. La respuesta documental se transmitió íntegra antes de generar. No hubo quinta celda ni repetición de finales.

El resultado es compatible con una limitación del recorrido de recuperación y no demuestra por sí solo un defecto del buscador. Se trata de dos preguntas bajo un protocolo diagnóstico nuevo, sin calificación clínica general ni sustitución del examen v7. El intento instrumental previo cerró antes de generar; la corrección de admisión obtuvo 72 pruebas y doce controles dinámicos conformes.

Cierre conforme: 255 archivos sincronizados, cero procesos propios, carga deshabilitada y sockets sin escucha. Se conservaron la instancia y sus accesos. Pico del grupo: 59 GiB exactos, sin intercambio ni OOM observados; no se acredita margen estático. La prueba humana de la interfaz permanece pendiente.

[Informe y evidencias](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/598039cae73f8f40f820bc5e3f78551a722ab58e/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-10/INFORME.md), [resultados del diagnóstico](tests-y-pruebas-efectuadas/DIAGNOSTICO-P05-P07-20261001.md). Commit de entrega: `598039cae73f8f40f820bc5e3f78551a722ab58e`; 17 archivos cotejados. S39 / TT-0016 / TT-0014 permanecen sujetos a recepción independiente; no se cierran tiques. Los antecedentes siguientes se conservan.

## Antecedente · Cierre del examen v7, 30/09/2026

**Ejecución terminada: 19 respuestas finales, seis impedimentos técnicos y cero preguntas sin ejecutar.** P25 terminó a las 18:46:05.912 UTC y la guarda cerró conforme a las 18:46:07.897 UTC. Carga deshabilitada, ausencia de procesos propios comprobada, servicio y conductor detenidos, socket enmascarado; instancia y accesos administrativos conservados.

La [entrega consolidada 09](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/1b3e1f999acb6dc6493388b1e80424dc00dd51cb/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-09/INFORME.md) conserva los cinco segmentos. Las correcciones instrumentales limitaron el efecto de rechazos conocidos a la pregunta, manteniendo las validaciones y los originales, sin repetir inferencias concluidas. Los impedimentos corresponden a P09, P12, P14, P15, P20 y P22.

| Corrección documental propuesta | 0: acierto | 1: error penalizado | U: indeterminación | Impedimento técnico |
|---|---:|---:|---:|---:|
| Total | 7 | 2 | 10 | 6 |
| Críticas | 6 | 2 | 8 | 4 |
| No críticas | 1 | 0 | 2 | 2 |

[Preguntas, respuestas y fundamentos](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/1b3e1f999acb6dc6493388b1e80424dc00dd51cb/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-09/RESULTADOS-P01-P25.md). **Propuesta: No apto en el alcance examinado**, por errores críticos P07 y P19; revisión humana pendiente. P19 se señala para valorar la generalización absoluta documentada. Los impedimentos no se convierten en U y no hay un frame completo. Este resultado no acredita aptitud clínica ni del dominio.

La corrección compara clave previa, corpus congelado y trazas MCP efectivas; conserva separados los resultados originales y la valoración. Los [resultados completos en Markdown](tests-y-pruebas-efectuadas/RESULTADOS-PDQ-HCL-25-20260930.md) y su [registro JSON](tests-y-pruebas-efectuadas/RESULTADOS-PDQ-HCL-25-20260930.json) quedan en la carpeta de pruebas del modelo. La presentación web se conserva como antecedente; su servicio se retiró por instrucción posterior. S39 / TT-0016 mantienen la recepción humana pendiente. La ejecución Thinking conserva su propio expediente y no se intervino.

**Lectura de los antecedentes siguientes:** sus fechas y resultados permanecen íntegros. Las expresiones antiguas de carga incompleta, ejecución o ausencia de inferencia corresponden a sus cortes y no sustituyen el estado vigente anterior.

Fecha: 29 de septiembre de 2026. Encargo: QWEN80-Q4K-ONECLOUD-20260929.

**La instalación y las compilaciones se conservan; la carga del modelo quedó incompleta por una incidencia de supervisión. No se ha realizado inferencia ni se ha habilitado la prueba humana del modelo.**

## Configuración identificada

| Componente | Identidad |
|---|---|
| Modelo | Qwen/Qwen3-Next-80B-A3B-Instruct |
| Distribución | mistralrs-community/Qwen3-Next-80B-A3B-Instruct-UQFF |
| Revisión de archivos | 193326907e14842f9f0c06bc9d36243fdcc9582b |
| Cuantización | UQFF Q4K: cinco fragmentos y residual |
| Motor | mistral.rs 0.9.4, revisión 2370966bb91e2e3dafa0b1521b87c50fd5c01244 |
| Candle | 66a8cf184a5a519671454066b1b9efd446ec9f5c |
| Compilador y gestor de dependencias | Rust y Cargo 1.98.0 |
| Plataforma observada | Ubuntu 26.04 LTS x86_64, CPU, doce procesadores lógicos; 67 417 481 216 bytes de RAM, sin swap |
| MCP | sv-mcp-documental 0.1.2, revisión 2e2e6ebc5f10e9c3027a0760a26f79f1d99b53ef |

La modificación de Ubuntu 24.04 a 26.04 fue autorizada expresamente. Los originales del motor, del servicio documental y de la captura se conservaron.

## Resultados observados

Se cotejaron tamaño y SHA-256 de los once archivos recibidos. Los pesos y el residual ocupan 45 338 887 469 bytes, aproximadamente 42,23 GiB; esta magnitud no representa el máximo de memoria de ejecución.

El motor adaptado y la integración documental Rust compilaron. Las pruebas previas del servicio reconstruyeron exactamente las cinco secciones y 25 páginas del catálogo conservado. La última comprobación completó 43 transacciones, incluidos rechazos y fallos deliberados. Se comprobaron operaciones de aislamiento: usuario sin privilegios, catálogo de solo lectura, acceso fuera del recinto denegado, MCP sin sockets, cliente sin salida a Internet y ausencia de proxies.

La integración prepara el recorrido petición, herramientas ofrecidas al modelo, propuesta estructurada, MCP, conservación previa a devolución, contexto siguiente y presentación web. **Ese recorrido no se ha demostrado todavía mediante una llamada generada por Qwen.** Las pruebas directas del MCP no sustituyen esta comprobación.

Se preservaron las incompatibilidades de preparación y sus correcciones en el adaptador propio. Se retiraron opciones del constructor no admitidas para CPU y por este cargador. El límite efectivo preparado continúa siendo 3840 tokens de entrada más 256 de salida, con recuento completo antes de cada generación. El informe UQFF multivariante se conserva fuera de una vista de carga formada por enlaces físicos a los originales Q4K; no se descargaron otras variantes.

En la inicialización final comenzó la carga. El observador temporal terminó el supervisor con código 74 y systemd terminó el proceso del motor. No se conservó el punto exacto que demoró la señal de actividad. El máximo de memoria contabilizado por el grupo fue 7 464 132 608 bytes, aproximadamente 6,95 GiB, con swap 0. **La carga no terminó; este máximo no permite concluir viabilidad o inviabilidad en 64 GB.** No se elevó la cota de 54 GiB ni se repitió una consulta generativa para obtener conformidad.

La aplicación y su punto de acceso quedaron detenidos; la instancia y el escritorio administrativo se conservaron. El escritorio no constituye la interfaz experimental del modelo.

## Componentes criptográficos y cobertura de auditoría

Se identificaron `aws-lc-sys 0.37.0`, interfaz Rust de AWS-LC, y `ring 0.17.14`, con componentes C y ensamblador utilizados por la capa criptográfica de comunicaciones. Su continuación fue autorizada expresamente después de la identificación. No se incorporó un motor neuronal C/C++.

Se conservaron los archivos estáticos de ambas dependencias. En la primera construcción completa se identificaron símbolos AWS-LC; la tabla global inspeccionada no acreditó símbolos ring_core. Se distingue la dependencia construida del código enlazado y de su invocación efectiva. No se afirma que ambas bibliotecas ejecutaran operaciones durante una conexión.

La explicación técnica, las huellas y las fuentes primarias se conservan en el [informe criptográfico fijado](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/9b6d09ff48194d0ebb62c582ea5fc6d505d44215/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-02/DEPENDENCIAS-CRIPTOGRAFICAS.md). Como fuentes generales de implementación pueden consultarse la [documentación oficial de AWS-LC para Rust](https://aws.github.io/aws-lc-rs/) y la [distribución ring 0.17.14](https://docs.rs/crate/ring/0.17.14/source/README.md).

OpenTelemetry Rust conserva los sucesos de los puntos instrumentados. La observación de procesos es acotada y por muestreo; no registra cada instrucción, cada llamada al sistema ni todo el razonamiento interno del modelo. La presencia de Rust o de telemetría no extiende automáticamente garantías de memoria al código nativo ni acredita ausencia de operaciones no observadas.

## Custodia y continuidad

La entrega privada y su cotejo se fijan en el commit `9b6d09ff48194d0ebb62c582ea5fc6d505d44215`:

- [Entrada de consulta](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/9b6d09ff48194d0ebb62c582ea5fc6d505d44215/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-02/LEAME.md).
- [Registro estructurado](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/9b6d09ff48194d0ebb62c582ea5fc6d505d44215/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-02/REGISTRO.json).
- [Cobertura y limitaciones de auditoría](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/9b6d09ff48194d0ebb62c582ea5fc6d505d44215/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-02/COBERTURA-AUDITORIA.md).
- [Diferencias efectivas](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/9b6d09ff48194d0ebb62c582ea5fc6d505d44215/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-02/DIFERENCIAS.md).
- [Manifiesto de integridad](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/9b6d09ff48194d0ebb62c582ea5fc6d505d44215/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-02/MANIFIESTO.sha256).
- [Evidencias originales comprimidas](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/9b6d09ff48194d0ebb62c582ea5fc6d505d44215/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-02/EVIDENCIAS-ORIGINALES.tar.gz).

Se cotejaron los 107 archivos de esa entrega con sus identidades Git locales y se comprobó que los contenidos anteriores permanecían inalterados. El archivo de evidencias tiene SHA-256 `a0a5624b13ab3ea954fedd066377ef64609d718f15fc32d8ff514eb61cce46d0`.

Permanecen pendientes el diagnóstico del supervisor, la carga completa, una generación sintética, la consulta documental única, el cotejo literal de respuesta y localizador, y la prueba humana por la misma vía web. La recuperación independiente y el aseguramiento de imagen tampoco se dan por acreditados. El estado científico del proyecto no se modifica mediante este registro instrumental.

## Corrección de supervisión y comprobación instrumental posterior

Actualización del 29 de septiembre de 2026, posterior a los resultados anteriores.

La supervisión corregida se compiló con Rust 1.98.0, sin modificar el motor, el MCP ni las dependencias fijadas. Se separaron las operaciones bloqueantes de conservación y observación del control temporal y de cancelación, con confirmaciones explícitas y un observador en otro proceso. Systemd conserva un segundo control temporal sobre ese observador.

La revisión final superó trece variantes instrumentales correspondientes a las ocho comprobaciones exigidas, incluidas una conservación demorada más de tres segundos, fallos reales de escritura y persistencia, bloqueo de operación, suspensión del supervisor y del observador exterior, cancelación, salida anómala, exclusión y reutilización de custodia. Las cinco pruebas unitarias de telemetría y análisis estricto también resultaron conformes. La primera ejecución adversa, sus errores y las correcciones se conservan diferenciadamente.

Se comprobó una transición de recepción restringida al acceso administrativo y conservada antes de habilitar consultas instrumentales. Se cotejaron el cuerpo HTTP, su huella, la entrada efectiva a Rust y el eco literal. **No se cargó el modelo ni se realizó inferencia; una respuesta instrumental no acredita una respuesta de Qwen.**

Ejecutable propio revisado: SHA-256 `d30f22089e3a3b0b790c67cde46aa107a98ac4c631ef827a9ee390337ff11291`. Las fuentes compiladas y su cotejo se incluyen en la [entrega-03](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/5bedfca43f1430ab709d3981f604a03322b361c8/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-03/LEAME.md), commit `5bedfca43f1430ab709d3981f604a03322b361c8`, con 68 archivos cotejados. Se conservan los antecedentes y las limitaciones del diagnóstico inicial: no se conoce la operación que demoró la señal original ni se puede concluir la viabilidad del modelo a partir de aquella carga incompleta.

El servicio quedó inactive/dead y su socket enmascarado; no quedaron procesos propios del ensayo. La instancia, el escritorio y sus accesos administrativos permanecen conservados. La recepción real y la carga están deshabilitadas. Los resultados observados no acreditan inspección visual del navegador, agotamiento real de memoria ni recuperación independiente.

Permanecen pendientes la recepción independiente de la corrección, la carga íntegra y memoria máxima, la comprobación Qwen/MCP real y la prueba humana. La prueba adicional anunciada no se ha definido ni implementado. Las dependencias criptográficas y sus excepciones mantienen las identidades y límites documentados anteriormente. No se modifica el estado científico del proyecto.


## Conciliación instrumental del presupuesto de memoria

Actualización del 29 de septiembre de 2026, posterior a la recepción favorable de la supervisión instrumental. Corresponde al encargo v2, commit `6cfd5c1c0eca124c2591421f45c3e3251ceaabf2`, y al seguimiento S39.

Se corrigieron conjuntamente la igualdad fija de 54 GiB de la inicialización y el umbral fijo de 62 GiB previo a la carga. Tres lecturas actuales y un cálculo conservador en bytes determinaron el presupuesto **QWEN80-MEMORIA-04-H52-R8-M1**: objetivo de 50 GiB, límite duro de 52 GiB, reserva de 8 GiB y margen adicional de 1 GiB. La comprobación previa exige 61 GiB disponibles, sin descontar el consumo ya incluido en el grupo. Esta lectura no garantiza una reserva física frente a variaciones posteriores.

La configuración, la inicialización, la observación y la creación futura del motor utilizan el mismo presupuesto. Se cotejan los límites propios y superiores; se rechazan datos ausentes o inválidos, cotas discordantes, restricciones superiores insuficientes e intercambio. Antes de crear el motor se conserva otra decisión de memoria; una disminución incompatible o una lectura caducada impiden su creación. El contador distingue preparación y carga confirmada.

La revisión final compiló con Rust 1.98.0 y dependencias sin cambios. Resultaron conformes **15 pruebas unitarias y 16 variantes instrumentales**. Estas últimas comprenden las trece pruebas anteriores y tres nuevas con cota propia discordante, límite superior insuficiente y disminución sintética de memoria. Se leyeron límites reales del grupo de control y se verificó la terminación de los procesos instrumentales. No se asignó memoria masiva ni se provocó agotamiento real.

Ejecutable de supervisión: SHA-256 `3ae37ea81c3152c5ec5604e4831772388cb1a0d8fe011b5c6eff2b2ef6436a26`. Se conservaron motor, MCP, pesos, corpus, dependencias y antecedentes. El servicio generativo permanece detenido, su activación implícita enmascarada y la recepción real deshabilitada. Instancia y escritorio continúan conservados.

La [entrega-04 con informe, fuentes, configuración, evidencias y manifiesto](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/5b80a79bc747579fe8dbfc42fcce227706c835dc/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-04/LEAME.md) queda fijada en `5b80a79bc747579fe8dbfc42fcce227706c835dc`, con **46 archivos cotejados**. La copia de seguimiento está en `/opt/sv-qwen80/evidencias/memoria-04` y la preparación en `/opt/sv-qwen80/memoria-04`.

**Esta conciliación no ha cargado Qwen ni efectuado inferencias; no acredita que la carga completa quepa en 52 GiB.** Se detiene para recepción independiente. La carga medida, la comprobación Qwen/MCP y el examen posterior requieren sus propias condiciones y autorizaciones. TT-0016 no se ejecuta ni se cierra mediante esta corrección. El estado científico del proyecto permanece en sus registros canónicos.

## Carga única y medición posterior a la recepción de memoria

Actualización del 29 de septiembre de 2026, conforme al encargo v3, commit `7a35a4165433d7685d83a700452df40b4e0143ce`, después de la recepción favorable `158c4014f27547101c4ba4559a99deebf0145670`. Se conserva el historial anterior y el seguimiento S39.

**Resultado: carga no completada.** Se inició una sola carga de Qwen80 UQFF Q4K. El supervisor terminó con código 76 por `PLAZO_OPERACION_AGOTADO` durante `observacion_modelo`, 396,286 segundos después del registro de creación del motor, según su reloj monotónico. No fue el vencimiento de los 3.600 segundos de carga. La causa interna de la demora no está determinada; el resultado no demuestra agotamiento de memoria.

Las decisiones de inicialización y precarga fueron conformes con H52-R8-M1. El máximo recuperado de `memory.peak` fue **53.689.405.440 bytes, 50,002155 GiB**, coincidente con el valor conservado por systemd. El máximo muestreado de `memory.current` fue 53.687.037.952 bytes. Estas magnitudes corresponden a la contabilidad del grupo, no a RSS. Se observó regulación: el último contador `high` era 9232, con `max`, `oom` y `oom_kill` en cero en esa lectura.

El observador Rust conservó 840 muestras con objetivo de 500 ms y una separación máxima de 30.874 ms. El grupo desapareció durante esa laguna, por lo que **no se acredita un máximo final exacto**: el máximo recuperado se conserva como cota inferior del recorrido completo. Los contadores anteriores no sustituyen una lectura final después de la terminación.

No se obtuvo la respuesta de salud exigida ni se realizó la observación favorable de 60 segundos. No hubo inferencia, tokenización de prueba, consultas MCP, examen ni habilitación de recepción humana. No se cambiaron el motor, el supervisor recibido, MCP, pesos, dependencias ni límites, y no se repitió el intento.

El diario registra una demora en la terminación con SIGKILL. Una comprobación posterior acreditó la ausencia de los procesos propios y del grupo real. Se restituyó la unidad original sin permiso de carga. El servicio conserva el estado de fallo `failed/failed`, con MainPID=0 y socket `masked/inactive/dead`; no se borró ese diagnóstico. Se conservan instancia y escritorio.

La [entrega-05 con informe, registro y evidencias](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/0c14961bccd90fb5119be3b35cbf1e7df2be86bd/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-05/LEAME.md) está fijada en `0c14961bccd90fb5119be3b35cbf1e7df2be86bd`, con **74 archivos cotejados**. SHA-256 del archivo de evidencias: `622b11cd81a7566900c43b066147f0c91a35ae81b230acfc4408b9a24f1552a3`. Copia de seguimiento: `/opt/sv-qwen80/evidencias/carga-05`; este registro se conserva también en `/opt/sv-qwen80/evidencias/registro-modelo-carga-05`.

La actuación queda detenida para recepción independiente. No acredita suficiencia de memoria para generación, funcionamiento Qwen/MCP, privacidad ni aptitud científica. TT-0016 y las etapas posteriores permanecen fuera del alcance.

## Continuación funcional y ajustes de memoria — 29/09/2026

La actuación v4 acredita **tres cargas completas**, con respuestas de salud HTTP 200 vinculadas a los procesos nuevos. Los tiempos hasta primera salud fueron 250,551, 240,510 y 240,943 segundos. Una autorización posterior permitió ensayo y error con ajustes justificados: se probaron cotas de **52, 56 y 59 GiB**, preservando los plazos absolutos y los intentos anteriores.

Las tres consultas utilizaron el mismo texto, descubrieron las herramientas MCP y contaron 543 tokens de entrada. Al comenzar la primera generación, el núcleo terminó el grupo por agotamiento de memoria: `CONSTRAINT_MEMCG` y `Result=oom-kill`. No se conservaron emisiones de respuesta; no se ejecutaron `buscar_documentos` ni `leer_documento`. Por tanto, **la comprobación documental no se completó** y no existe reutilización conforme que medir.

En la última terminación se registraron 63.206.232.064 bytes de memoria anónima del grupo. El fallo no se atribuye a un plazo de supervisión ni únicamente a caché de archivos. La cota final de 59 GiB dejaba unos 3,79 GiB físicos fuera del grupo; no se amplió más para conservar margen del sistema. No se determina la memoria mínima suficiente ni se descarta otra configuración futura.

La revisión acotada separa captura, conservación y confirmación; limita la observación periódica a sesenta segundos y mantiene la vigilancia independiente de tres segundos. Las pruebas afectadas fueron conformes; la última compilación superó 22 pruebas unitarias y una comprobación real del perfil sin pesos. Motor, MCP, pesos, catálogo, consulta y dependencias permanecieron intactos. Los máximos del grupo, RSS y memoria anónima se distinguen; las lagunas de observación impiden afirmar continuidad perfecta o un máximo final exacto.

A las **12:29:27 UTC** se verificaron procesos propios ausentes y configuración original restituida, sin permiso de carga ni recepción humana. El servicio conserva el diagnóstico de fallo; el escritorio y la instancia permanecen disponibles. No se eliminó ni reinició la instancia.

[Entrega-06, informe, registro y evidencias](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/0dad86d58198e6babf1c88f41c48077751a894ad/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-06/INFORME.md), commit `0dad86d58198e6babf1c88f41c48077751a894ad`, **16 archivos cotejados**. El archivo único de evidencias tiene SHA-256 `71d60a2c9d925b676952bf3344177cc4296585548fb6102493df754de17c9e49` e índice de 1.396 originales y fuentes. Copia de seguimiento: `/opt/sv-qwen80/evidencias/funcional-06`.

La recepción independiente sigue pendiente. Estos resultados no acreditan funcionamiento documental Qwen/MCP, privacidad integral ni aptitud científica. S39 conserva el seguimiento; TT-0016 no se ha ejecutado ni cerrado.


## Selección de expertos acotada y conectividad — continuación v5, 29/09/2026

La corrección Rust conserva el contexto, F32 y las diez rutas por token, pero materializa las matrices seleccionadas por bloques acotados. La biblioteca final superó 96 comparaciones numéricas con diferencia máxima cero; el supervisor superó 22 pruebas. La construcción final retornó cero y su utilización se cotejó mediante la huella del proceso real: `2e82adbf71c8e73d449b7367fe81fd553788387fb48c1fe84d4eb04bd6fa326b`. Pesos, catálogo, MCP y dependencias se conservaron.

Una carga completa alcanzó primera salud en 295,625 segundos. La consulta mantuvo 543 tokens y se ejecutó con límite de 52 GiB, sin intercambio. Se observaron **1.253 selecciones materializadas**, con máximo de **1,25 GiB**, frente a los 21,2109375 GiB calculados para la selección completa anterior. La proyección descomprimida continuó siendo de 2 GiB. La traza alcanzó la capa de índice 21 de la consulta. Los últimos contadores de agotamiento de memoria fueron cero; el máximo recuperado del grupo fue 52 GiB y la RSS agregada máxima muestreada, aproximadamente 52,867 GiB, con magnitudes y limitaciones diferenciadas.

**La respuesta documental no se completó.** Sólo se ejecutó tools/list; no hubo buscar_documentos, leer_documento ni cita que cotejar. El proceso devolvió `request or response body error` y el supervisor cerró con código 71, aproximadamente 896,559 segundos después de la admisión. La coincidencia con el plazo HTTP de 895 segundos es compatible con su vencimiento, pero el mensaje genérico no determina la causa interna. No se repitió la consulta ni se amplió exclusivamente el plazo sin una nueva corrección comprobada; los tiempos por proyección y la extrapolación orientativa se conservan en la entrega.

Se conservaron **108 comprobaciones exteriores TCP/SSH correctas**, que no acreditan continuidad entre muestras. Una conexión anterior de compilación quedó abierta sin sus procesos remotos; su retorno original sigue sin recuperarse. No se acredita cambio de IP ni reinicio. La consola noVNC permanece pendiente, aunque gdm está activo.

A las **17:27:31 UTC** se verificaron procesos propios ausentes, unidad original restituida, carga deshabilitada y recepción humana cerrada. Los observadores terminaron. El escritorio y la instancia se conservaron. [Entrega-07, informe y evidencia consolidada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/c6b22cfed2b9cd8c7494115240de47933f474c69/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-07/INFORME.md), commit `c6b22cfed2b9cd8c7494115240de47933f474c69`, **24 archivos cotejados**. Archivo de evidencias: 613.633 bytes, SHA-256 `12cacdb506f8ade526ed18c1d972784a90fe612ffbc01e0f26d6b3087e8bbb50`. Copia de seguimiento: `/opt/sv-qwen80/evidencias/funcional-07`.

La mejora de memoria observada no acredita todavía el recorrido documental completo, privacidad integral ni aptitud científica. La recepción independiente sigue pendiente. TT-0016 no se ha ejecutado ni cerrado.


## Continuación v6: respuesta documental y estabilidad — 29/09/2026

Se conserva íntegro el registro anterior, referido a sus respectivas actuaciones. La continuación obtuvo una respuesta mediante búsqueda y lectura auténticas del MCP aprobado y una única repetición con conversación nueva y motor reutilizado. Duraciones: consulta 2: 1532.618 segundos; consulta 3: 1535.382 segundos. Se conserva la entrada inicial de 543 tokens, el modelo, la plantilla y el corpus.

La primera oración conserva letras y signos tras unir un salto de línea de la fuente. No existe identidad byte a byte de esa cita. El cotejo y las respuestas originales permiten examinar la diferencia; no se atribuye una literalidad informática que no se obtuvo.

Se corrigieron las copias y descompresiones redundantes de expertos y la interpretación estricta del formato de llamada emitido por la plantilla Qwen. Las 144 comparaciones numéricas obtuvieron diferencia máxima cero; las treinta pruebas finales del supervisor fueron conformes. El límite efectivo del grupo fue 59 GiB, sin intercambio en disco. No se acredita una garantía general de memoria o rendimiento.

Los procesos propios de ensayo quedaron detenidos y la carga y recepción general deshabilitadas. Se conserva la instancia, el escritorio y SSH. Su facturación puede continuar. La recepción independiente, el acceso humano y el examen TT-0016 permanecen pendientes; no se emite dictamen científico. Persisten las excepciones criptográficas `aws-lc-sys` y `ring`.

El cierre presentó una incidencia: el supervisor comunicó `FIN_CONFORME` y terminó con código 0 a las 20:43:42 UTC; a las 20:43:44 UTC, systemd registró `State 'stop-sigterm' timed out. Killing.`. Posteriormente se comprobaron `MainPID=0`, la ausencia del grupo y la desaparición de los procesos propios. Se conserva `Result=timeout`; no se identifica qué proceso o fase de liberación agotó los dos segundos de parada. La terminación quedó comprobada, pero no corresponde describirla como un cierre sin incidencias.

[Informe y evidencias de la entrega 08](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/cc65d2638e938014863db200ee7ae10540d3e7fc/respuestas-ejecucion/QWEN80-Q4K-ONECLOUD-20260929/entrega-08/INFORME.md). Commit de custodia: `cc65d2638e938014863db200ee7ae10540d3e7fc`. El manifiesto de la entrega identifica los archivos y sus huellas.
