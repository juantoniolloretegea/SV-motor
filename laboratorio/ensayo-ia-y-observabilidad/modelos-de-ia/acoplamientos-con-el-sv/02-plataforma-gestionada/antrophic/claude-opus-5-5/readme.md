# Claude Opus 5.5 · Kaggle Benchmarks · nodo 02

Edición documental 1.3 · 09/10/2026. Estado: acceso restablecido; transporte Rust comprobado en Windows, Linux y el cuaderno real de Kaggle; MD01-R0 recibido; interpretación instrumental de refusal vacío corregida y original recuperado sin nueva inferencia; continuación de R1 pendiente de admisión de la revisión r2. Sin dictamen científico del candidato.

## Identidad y características

Claude Opus 5.5 es un modelo de Anthropic presentado el 22 de septiembre de 2026 para tareas complejas de código, agentes y trabajo de conocimiento. Estas características son declaraciones del fabricante, no resultados del SV. Véanse el [anuncio oficial](https://www.anthropic.com/claude-opus-5-5) y las [fichas de evaluación del fabricante](https://www.anthropic.com/system-cards).

El catálogo de Kaggle consultado ofrece `anthropic/claude-opus-5-5@default`; la respuesta del servicio declaró ese mismo identificador. La denominación `default` no permite fijar una revisión inmutable de los pesos ni asegurar que futuras solicitudes utilicen los mismos parámetros internos. Número de parámetros, formato de pesos, precisión numérica y ubicación física de inferencia no se han acreditado en esta realización.

## Modalidad y lugar de despliegue

La inferencia se solicita mediante Kaggle Benchmarks, plataforma gestionada del nodo 02. Los pesos no se descargan al equipo ni se instalan en un servidor del SV. El cuaderno aloja la interfaz de acceso y, una vez recibido, el transporte experimental Rust; el modelo es servido por la infraestructura gestionada detrás de la plataforma. La plataforma no comunica en esta prueba la región ni el hardware de inferencia.

La apertura de un cuaderno de Benchmarks proporciona el entorno de Kaggle. Su [biblioteca oficial](https://github.com/Kaggle/kaggle-benchmarks/blob/ci/quick_start.md) documenta el catálogo y el acceso gestionado. Las credenciales temporales se usan sólo en ese entorno autorizado; no se incorporan al expediente. La preparación y la recepción técnica del transporte Rust se registran por separado del resultado científico.

## Condiciones experimentales

El control del ensayo, la composición documental, la secuencia de entregas, las comprobaciones y las mediciones propias se realizan en Rust. El núcleo, su semántica y su representación intermedia se preservan. El candidato recibe únicamente fuentes autorizadas y antecedentes de su propia pregunta; banco de referencia, clave reservada, criticidad, puntuaciones y decisión del Árbitro quedan fuera de su contexto.

La campaña autorizada consta de nueve preguntas del manual y, después, dieciséis preguntas de ciberseguridad. Cada pregunta conserva las tres etapas universales: respuesta provisional, autocrítica y verificación final neutral. La última etapa es final; no se selecciona retrospectivamente la mejor respuesta. Corpus, casos y criterios se fijan antes de inferir, tomando el antecedente del nodo 03. Su comparación declarará las diferencias de plataforma, configuración y observabilidad.

Se solicita información operativa accesible y una explicación pública verificable. Se conservan por separado lo declarado por el modelo, los canales de razonamiento efectivamente emitidos, los contadores del servicio y las mediciones del SV. Una explicación no acredita observación de procesos internos; un dato no recibido se declara no disponible. No se habilitan herramientas del candidato ni acceso a Internet como fuente. Gemini está excluido también como evaluador y auxiliar.

Sólo se utiliza la cuota existente, sin adquisiciones ni ampliaciones. El primer envío devolvió HTTP 400 de configuración, sin mensaje del candidato ni contadores de uso. Su coste queda pendiente; se retiene la reserva completa de 0,606024 USD y se reduce el margen restante a 9,383976 USD. La adaptación r1 omite únicamente response_format conforme a la solución oficial para esquemas anidados, conservando idéntico el contrato documental. Véase [incidencia y corrección](ensayo-md09-cyb16-20261009/intento-01/CORRECCION-INSTRUMENTAL.json). Las cotas efectivas de salida, razonamiento, tiempo, reintentos y reserva de cuota se documentarán en la admisión técnica antes del primer caso científico. No se presume que una etiqueta de esfuerzo equivalga al mismo cómputo interno en otros proveedores.

## Preparación y puesta en servicio del transporte

El [adaptador Rust 0.1.0, revisión r2](instrumentacion-rust/0.1.0-r2/) conserva código, dependencias fijadas, binario Linux y comprobaciones. El modelo sigue desplegado por la plataforma gestionada; este ejecutable es transporte y control experimental, no sus pesos ni un motor de inferencia local.

1. Obtener el banco y las secciones mediante el MCP mdBook de Rust. Cotejar corpus, aislamiento, diario y bytes; separar claves y criticidades del contexto del candidato. El [paquete de ensayo](ensayo-md09-cyb16-20261009/admision/PAQUETE-CANDIDATO.json) contiene sólo el suministro admitido para los mensajes.
2. Usar el binario Linux conservado o compilar su código con Rust compatible, Cargo.lock y `cargo build --locked --bin ejecutar`. La versión entregada se construye con Rust 1.98.1 y requiere glibc 2.34 o posterior. No se presupone compatibilidad por la etiqueta Linux.
3. Situar el binario y el paquete en el cuaderno autorizado de Kaggle Benchmarks. La plataforma aporta su acceso temporal; no se copia a archivos, documentos ni al equipo. Un auxiliar de arranque puede descargar y lanzar el ejecutable; la composición, secuencia, controles y mediciones pertenecen a Rust.
4. Ejecutar `./ejecutar PAQUETE-CANDIDATO.json SHA256 --comprobar`. Debe concluir sin solicitudes a modelos, con 75 composiciones y medición remota conformes. El inicio posterior coteja identidad de binario y paquete contra esa admisión.
5. Para la continuación delimitada de este ensayo, ejecutar `./ejecutar PAQUETE-CANDIDATO.json SHA256 --recuperar-md01-r0` únicamente para el ensayo autorizado. Se recupera MD01-R0 íntegro desde sus originales fijados, comprobando petición, contrato, respuesta, huellas y telemetría, y se continúa por MD01-R1. No se repite R0. Se mantiene la secuencia MD01–MD09 y luego CYB16, con R0/R1/R2 por pregunta. La pausa instrumental cuenta dentro de la cota original de 90 minutos del manual; no se reinicia el plazo. Ante recepción incompleta, pérdida de medición, coste desconocido, margen insuficiente o vencimiento de la cota, se detiene la secuencia y conserva lo recibido.

Kaggle ModelProxy desactiva el flujo en su biblioteca oficial. La adaptación usa JSON completo, esquema íntegro en las instrucciones y validación local estricta, esfuerzo high y cota de salida 16384 tokens. Se conserva el cuerpo HTTP original y la separación reversible del razonamiento emitido dentro de etiquetas, además de los demás canales que comunique el servicio. No se mide primer token; sí la recepción externa, CPU, memoria, E/S y conexiones del proceso Rust. Estas medidas no describen la infraestructura interna de Anthropic.

Las [condiciones y límites de admisión](ensayo-md09-cyb16-20261009/admision/FRONTERA-KAGGLE.md) fijan la reserva antes de cada envío, cuota existente y ausencia de reintentos automáticos. La comprobación en el cuaderno real concluyó con 75 composiciones, cero solicitudes a modelos y cinco muestras propias sin fallos, intervalo máximo de 259 ms, Linux x86_64 y glibc 2.41. Se recuperaron los originales y se cotejaron en Rust las huellas, el diario y la identidad del proceso. Véase [recepción técnica](ensayo-md09-cyb16-20261009/admision-remota/RECEPCION-RUST.json). Esta admisión técnica no acredita todavía recepción de una respuesta científica del servicio ni aptitud del candidato. No se atribuye a Claude navegación autónoma del MCP: recibe el corpus íntegro preentregado por el control del SV.
## Primera entrega y recuperación instrumental

MD01-R0 recibió HTTP 200, terminación stop y el identificador autorizado. El servicio comunicó 30330 tokens de entrada, 6713 de salida, 37043 en total y 1695 de razonamiento incluidos en la salida; coste comunicado 0,25558 USD. El campo refusal era una cadena vacía y la guarda anterior la clasificó erróneamente como negativa. La revisión r2 distingue vacío de negativa real y recupera el mismo original sin solicitar otra generación. El formato y 19 citas se cotejaron en Rust; ello no constituye puntuación científica ni recepción independiente.

La [recuperación cotejada](ensayo-md09-cyb16-20261009/recuperacion-md01-r0/RECUPERACION-RUST.json) conserva los bytes originales, la declaración operativa y la telemetría: 203 muestras, ningún fallo y máximo de 260 ms. La solicitud de R0 utilizó el binario r1, identificado por su propia huella; la recuperación y las siguientes solicitudes emplean r2. La revisión instrumental se distingue de la etapa R2 del banco.

El servicio no entregó el texto de los 1695 tokens de razonamiento. Se conserva todo lo emitido y se declara esta limitación; no se reconstruye pensamiento no recibido ni se repite una pregunta para obtenerlo. R0 tampoco medía hilos, descriptores ni espacio de red. Para las siguientes solicitudes, r2 incorpora esos datos del proceso propio cuando /proc los ofrece en Linux; no observa recursos internos del proveedor. Los handles y las magnitudes no accesibles permanecen no disponibles. Véanse [comprobaciones y límites del nodo 02](ensayo-md09-cyb16-20261009/COMPROBACIONES-NODO02.md).
## Comprobación de acceso y límites

Una solicitud breve al identificador de Opus fue aceptada y registró 24 tokens de entrada y 40 de salida. Terminó por longitud y no entregó mensaje textual. Acredita aceptación de la solicitud, no respuesta utilizable, observabilidad íntegra ni competencia documental. No se confunde con las futuras pruebas del manual o de ciberseguridad. El incidente de la plantilla del cuaderno permanece separado en el archivo administrativo.

No se han probado aquí todos los modelos del catálogo ni la asignación efectiva de GPU o TPU. El acceso ordinario tampoco acredita concesión de una ampliación de investigación. La plataforma gestiona infraestructura y parte del transporte: sus procesos internos no son observables por el SV. La recepción científica independiente de las pruebas futuras queda pendiente.

Los originales, solicitudes, respuestas, medidas, cotejos y dictámenes se conservarán bajo esta carpeta por ensayo y revisión. La publicación sólo se declara tras recuperar y cotejar los archivos de GitHub; el material local es temporal. Las fuentes de terceros conservan su régimen original.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
