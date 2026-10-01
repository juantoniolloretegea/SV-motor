# MCP documental 0.1.3: acceso verificable y reconstrucción

**Fecha:** 01/10/2026. **Adscripción:** S39 / TT-0014. **Protocolo:** MCP 2025-06-18.

Esta revisión corrige obstáculos del acceso documental y añade un cotejo reproducible del recorrido MCP. Se conservan intactas las versiones anteriores y las configuraciones utilizadas por Qwen. No acredita instalación ni inferencia de GPT-OSS-Safeguard-120B.

## Cambios

- Búsqueda de hasta 200 caracteres Unicode sin la restricción adicional, antes no anunciada, de ocho palabras. Sigue exigiendo todos los términos literales: no es búsqueda semántica.
- Continuación mediante desplazamiento; todos los resultados pueden recorrerse aunque la cota de respuesta reduzca el número devuelto en una página.
- Errores de argumentos con causa y posibilidad de corrección, sin ejecutar la solicitud inválida ni impedir la llamada válida siguiente.
- Contenido estructurado y texto JSON equivalentes; ambos se incluyen al calcular el tamaño máximo. El contrato emitido se conserva en [HERRAMIENTAS.json](HERRAMIENTAS.json).
- Diario con solicitud previa, resultado previo y constancia posterior de escritura al transporte. Identidad, secuencia, UTC, tiempo monótono, duración, bytes y SHA-256 encadenado; sincronización antes de emitir resultados.
- Reconstrucción determinista con el catálogo fijado: el verificador vuelve a ejecutar las solicitudes y contrasta respuestas, bytes, orden, continuidad y cierre. Rechaza cambios, supresiones, reordenaciones y recorridos incompletos.

## Comprobación y reproducción

El [registro de pruebas](PRUEBAS.txt) conserva la ejecución Rust. [Las pruebas de protocolo](tests/protocolo.rs) comprueban límites, paginación, Unicode, integridad, recuperación y equivalencia de las representaciones. [Las de auditoría](tests/auditoria.rs) ejecutan el proceso real, cotejan entrada y salida, intentan falsificar registros y comprueban que no existe entrega cuando falla la apertura de la custodia.

Se utiliza Linux x86_64, Rust/Cargo 1.93.1 y Cargo.lock. La versión mínima declarada se ajusta al compilador con el que se ejecutan estas comprobaciones; no se afirma una ejecución con otro compilador. Las dependencias se preparan antes de la fase aislada.

~~~sh
cargo test --locked
cargo run --locked --bin verificar-diario -- evidencias/catalogo.json HUELLA_DEL_CATALOGO evidencias/diario.jsonl --sintetico
~~~

La huella del catálogo se consulta en [MANIFIESTO.json](MANIFIESTO.json). [COTEJO.json](evidencias/COTEJO.json) conserva el resultado del verificador. Los archivos del ejemplo son sintéticos y no contienen respuestas de un modelo. El programa [preparar-custodia](examples/preparar-custodia.rs) reúne un recorrido conforme producido por las pruebas y calcula el manifiesto.

## Alcance de la auditoría

El verificador acredita reconstrucción del servicio documental. La huella del binario ejecutado figura en el inicio del diario. Las huellas deben cotejarse contra un manifiesto conservado de forma independiente; una cadena recalculable no es una firma ni protege por sí sola de la sustitución conjunta de todos los archivos.

La constancia de escritura en la tubería no acredita la recepción del conductor ni la incorporación al motor. Es obligatorio cotejar esos dos saltos con los registros del conductor, la plantilla aplicada, los tokens y la generación real. Un corte entre preparación y entrega impide un cierre conforme. El modelo, el conductor, su observador exterior y la custodia remota tienen que acreditar su propia cobertura.

**Un tramo relevante no observable o no reconstruible determina No apto para el uso exigido por el SV.** Las pruebas de este componente no permiten declarar apto el conjunto aún no ensayado. El nombre de una biblioteca de telemetría no acredita su uso efectivo.

La reproducción documental es exacta por bytes. Para el candidato generativo se exige invariancia del contenido, no de la redacción: deben conservarse hechos, cifras, unidades, negaciones, relaciones, condiciones, excepciones, población y grado de incertidumbre, además de todos los elementos solicitados. La coincidencia de vocabulario no sustituye esa evaluación independiente.

## Perímetro y cotas efectivas

El servicio sólo busca y lee el catálogo cargado y verificado; las URL son procedencia. El filtro seccomp del proceso impide crear sockets o conectar a la red. La prueba ejecutada comprueba ese proceso; los permisos de sólo lectura y el aislamiento del futuro motor deben probarse bajo la identidad efectiva de su despliegue.

Se mantienen: catálogo hasta 2 MiB, hasta ocho documentos y 64 secciones por documento; trama hasta 16 KiB; respuesta hasta 8000 caracteres JSON; página objetivo hasta 2000 caracteres, reducida según metadatos; operación hasta 30 segundos; hasta 128 llamadas por sesión y hasta 256 tramas según configuración. El diario se amplía a 64 MiB para conservar solicitudes y resultados íntegros con el nuevo formato. Las cotas se anuncian y el agotamiento produce un impedimento explícito.

La instalación debe dimensionar y comprobar el recorrido legítimo más exigente antes del examen. Si supera una cota, deberá ajustar y verificar una nueva revisión o repartir sesiones documentales identificadas sin perder sus enlaces; no truncar en silencio ni atribuir el impedimento al conocimiento del modelo.

El catálogo real mantiene la fuente PDQ previamente autorizada; los ejemplos requieren autorización explícita de modo sintético. Ampliar las fuentes exige un contrato de corpus propio. No se habilitan órdenes, rutas arbitrarias, descarga de documentos ni modificación del catálogo.

Los programas cliente-minimo y observar-minimo se conservan como antecedentes de alcance limitado. El primero no integra herramientas con Safeguard. Esta revisión no les atribuye capacidades nuevas.
