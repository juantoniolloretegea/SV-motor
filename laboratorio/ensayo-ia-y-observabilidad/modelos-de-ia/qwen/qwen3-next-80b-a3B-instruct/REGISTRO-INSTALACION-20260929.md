# Registro instrumental de instalación: Qwen3-Next-80B-A3B-Instruct Q4K

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

