# Segunda prueba de GPT-6 Astra: instrumentación local y recepción

Fecha: 6 de octubre de 2026. Nodo 03, inferencia mediante API directa de proveedor. Candidato: `gpt-6-astra`.

## 1. Motivo, autorización y resultado

La primera prueba acreditó una respuesta del proveedor, pero no midió recursos del proceso, conexiones TCP ni puertos. La objeción instrumental es fundada: tokens y duración no bastan para recibir el acoplamiento conforme a las exigencias del SV. Esa carencia histórica permanece identificada; no se rellena con mediciones posteriores.

Se autoriza expresamente una segunda inferencia breve para medir el recorrido. Se ejecuta una sola solicitud nueva, sin reintentos, con el mismo texto artificial y sin documentos del SV ni datos de salud. Se reutiliza el cliente registrado y la autorización oficial; el uso de créditos existentes se habilita temporalmente y se restablece a desactivado al terminar. No se gestionan pagos, compras, recargas ni medios de pago.

**Resultado de la segunda prueba: respuesta y terminación conformes; instrumentación local ejecutada y cotejada dentro del ámbito descrito.** No constituye admisión científica ni recepción integral del instrumento. En el conjunto de ambas actuaciones hay tres solicitudes: un rechazo inicial y dos generaciones concluidas.

## 2. Realización y observación

El cliente incorpora una biblioteca compartida Rust, `sv-instrumentacion 0.1.0`, independiente del proveedor. La lógica propia prohíbe código `unsafe`. Las bibliotecas `sysinfo 0.39.6` y `netstat2 0.11.2` consultan las API nativas del sistema operativo; esto no significa que Windows esté realizado en Rust.

Se observa exclusivamente el proceso inscrito, comprobando PID e instante de inicio para detectar discordancias de identidad. La tabla de conexiones se filtra por ese PID antes de conservarla. No se guardan argumentos, variables de entorno ni usuarios del sistema. La observación empieza antes de la petición y continúa después de liberar su transporte.

El muestreo tiene un intervalo objetivo de 250 ms. Cada muestra conserva memoria residente y virtual, CPU acumulada y porcentaje equivalente a un núcleo, E/S del proceso, conexiones TCP/UDP, extremos, puertos, estados y duración de captura. La primera lectura de porcentaje de CPU se deja sin valor porque necesita un intervalo de cálculo. No se confunde ausencia de medida con cero.

Una traza Rust registra preparación, identidad del ejecutable por SHA-256, serialización, inicio de envío, recepción de cabeceras, cada lectura del cuerpo, tipos y secuencias de eventos SSE, liberación del transporte y cierre de la observación. Comparte secuencia y reloj monotónico con las muestras; añade tiempo Unix en milisegundos. Sus metadatos no contienen credenciales ni texto documental.

Los registros se conservan en una cadena SHA-256 y se vuelven a leer para comprobar secuencia, tiempos e integridad. La cadena es un mecanismo de cotejo local; no una firma digital ni un sellado temporal externo.

## 3. Mediciones efectivas

| Magnitud | Resultado | Procedencia y alcance |
|---|---:|---|
| Modelo solicitado y declarado | gpt-6-astra | Petición y respuesta del proveedor |
| Resultado textual | CONEXION ASTRA CONFIRMADA | Fragmentos y cierres correlacionados |
| Terminación | response.completed | Proveedor; estado completed |
| HTTP | 200, HTTP/1.1 | Transporte del cliente |
| Duración desde envío hasta recepción terminal | 4296 ms | Reloj monotónico Rust |
| Cabeceras recibidas | 2453 ms | Desde el inicio del envío |
| Primer texto recibido | 3984 ms | Recepción local, no primer token interno del modelo |
| Entrada / salida / total | 49 / 12 / 61 tokens | Uso comunicado por el proveedor |
| Eventos SSE | 16 | Flujo conservado y cotejado |
| Cuerpo recibido | 8483 bytes | Datos de aplicación; no bytes de red con TLS |
| Muestras del proceso | 28 | 8 antes, 17 durante y 3 después de la petición |
| Primera a última muestra | 7218 ms | Ventana local |
| Inicio a cierre de observación | 7443 ms | Incluye intervalo final |
| Intervalo máximo entre muestras | 284 ms | Objetivo 250 ms; sin fallos registrados |
| Memoria residente máxima | 26,6211 MiB | Proceso cliente, incluida la instrumentación |
| CPU acumulada en la ventana | 500 ms | Proceso cliente, no modelo remoto |
| CPU máxima equivalente a un núcleo | 50,35 % | Muestreo local; no normalizado a toda la máquina |
| E/S leída / escrita en la ventana | 9 587 200 / 77 096 bytes | Contadores de E/S de Windows |
| Combinaciones de conexión y estado observadas | 7 | Mismo PID; no siete inferencias ni siete conexiones distintas |

La E/S de Windows incluye operaciones del proceso y no equivale exclusivamente a disco ni a tráfico TCP. La lectura del propio ejecutable para calcular su huella explica el volumen de lectura de esta comprobación. CPU y memoria incluyen esa operación y la instrumentación. No se atribuye a instrumentación, modelo o red la diferencia de duración frente a la primera prueba: no hay experimento controlado que permita esa inferencia causal.

Se observan la escucha local, las conexiones del catálogo y la autorización, la comunicación del navegador con el receptor y una conexión HTTPS nueva. Esta última aparece en estado SYN-SENT y luego ESTABLISHED; desaparece del conjunto atribuido al proceso después de liberar el transporte de inferencia. El extremo remoto comunicado por HTTP coincide con uno observado en TCP. La coincidencia de un extremo por sí sola no atribuye todas las conexiones del proceso a una petición: había también una conexión previa al mismo extremo.

## 4. Corrección del lector y comprobaciones

El lector anterior esperaba el texto en la salida del evento terminal. Se incorpora al cliente de esta prueba el ensamblaje y cotejo de los eventos del flujo: secuencia, identidad de respuesta, índice e identidad del mensaje, fragmentos, cierre textual, cierre de contenido y mensaje concluido. Se admite un evento terminal con salida vacía únicamente cuando los otros cierres correlacionados acreditan el texto. Se coteja también la suma del uso comunicado.

El contrato comprobado es deliberadamente estrecho: una respuesta textual y un solo mensaje, sin herramientas ni contenidos adicionales. No es todavía un lector universal de todas las variantes de Responses.

Comprobaciones ejecutadas:

- 17 pruebas del cliente: autenticación, control de retorno, recepción, flujo real conservado, secuencia incompleta, texto alterado, identidad terminal discordante y uso ausente, entre otras.
- 4 pruebas de la biblioteca: proceso propio y escucha/conexión/cierre TCP reales, discordancia de identidad, integridad con alteración/truncado y escape de HTML.
- 1 prueba del cotejador anterior.
- Cotejo posterior de la segunda ejecución en Rust: 17 archivos, integridad de telemetría, cobertura previa/durante/posterior, 16 eventos instrumentales correspondientes al flujo, texto y uso, escucha local y coincidencia del extremo HTTP con TCP. El cotejo no envía otra solicitud.

El criterio de cobertura aplicado exige muestras previas, durante y posteriores, ausencia de fallos registrados y separación máxima de 750 ms. El resultado cumple ese criterio acotado; no se presenta como umbral general aprobado del SV.

## 5. Custodia y presentación

Expediente local: `prueba-instrumentada-20261006/`, dentro de la carpeta de este candidato. Incluye petición, marcador de envío único, SSE, evento terminal, resultado de transporte, resultado completo, catálogo, fuentes y dependencias de la compilación utilizada, telemetría, cotejos y cuadro HTML. `RECEPCION-RUST.json` identifica los 17 archivos por tamaño y SHA-256.

Huella del flujo recibido: `486467a83cfa700c842c7f412605b4d29de418f23d8f70b6c02561a8b60a09d9`.

Huella del registro de telemetría: `b0214b6f6dcf6007882dcc0c860a1170f65c540c5785a8246614eb01577e388e`.

La evidencia completa se conserva localmente: contiene identificadores seudónimos y topología de red del anfitrión. Este informe público omite esos datos y no acredita custodia remota del expediente íntegro. El cuadro en el navegador representa la ejecución concluida, con series temporales, conexiones y estados, fases, eventos y límites; no simula actividad en directo. El receptor de autorización de la prueba se cierra y se verifica su ausencia; sólo queda un visor local temporal sin credenciales y sin inferencia.

## 6. Límites y retorno

No están medidos los recursos internos de OpenAI; ni DNS/TLS por separado, RTT y retransmisiones TCP, hilos y descriptores del sistema, ni asignaciones individuales de memoria Rust. Los recursos internos del proveedor no son observables mediante esta API. Los demás puntos son límites de esta realización, no imposibilidades universales. El muestreo puede omitir estados y conexiones efímeros; los contadores de procesos proceden del sistema operativo y su biblioteca de acceso no informa todos los posibles fallos por campo.

El proveedor no comunica en la respuesta créditos descontados ni liquidación monetaria. El saldo general de la cuenta incluye otros usos y no permite atribuir su variación a esta petición.

La dependencia criptográfica C/ensamblador continúa pendiente bajo la excepción humana autorizada. Las credenciales de sesión no se incorporan a los registros. No se ejecutan casos A01–A09, examen, valoración científica ni polígono incompleto.

Retorno: recibir formalmente el mínimo instrumental y su correspondencia con los contratos, privacidad y control de consumo del SV antes de abrir la campaña. La sede por modelo conserva este ensayo; los registros canónicos de sucesos, tiques y calidad mantienen su autoridad y la conciliación de estas pruebas debe completarse allí, sin numeración paralela.

## Referencias técnicas

- [Eventos de Responses, documentación oficial de OpenAI](https://developers.openai.com/api/reference/resources/responses/streaming-events).
- [Proceso y contadores, sysinfo](https://docs.rs/sysinfo/0.39.6/sysinfo/struct.Process.html).
- [Tablas de conexiones, netstat2](https://docs.rs/netstat2/0.11.2/netstat2/).
- [Primera prueba: evidencia y limitaciones](PRUEBA-CONEXION-20261006.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

