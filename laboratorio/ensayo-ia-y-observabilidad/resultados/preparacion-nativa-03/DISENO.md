# Correcciones R1/R2/R3: diseño y límites

## R1 · Decisión común

nativa/mod.rs define Admision. Son necesarias conjuntamente salida correcta del proceso, ambos EOF, secuencia completa, no revocación, ningún fallo de observación/custodia, oráculo técnico conforme y sellado confirmado. El juicio contractual no interviene: ESTRUCTURA puede acompañar a una ejecución técnicamente admisible cuyo texto original debe mostrarse.

El cierre pasa a sellando sin declarar éxito. El custodio evalúa la misma condición de forma condicionada al éxito de su sellado y sólo devuelve Snapshot después de sync, cotejo de bytes, manifiesto y cierre de todos sus descriptores. El supervisor registra ACK y usa Admision::admisible para estado y exposición de resultado. Sin ACK, fallo o plazo de cinco segundos: custodia invalidada, revocación técnica e interrumpida/no admisible. Un ACK tardío después de invalidación no habilita descargas.

La API devuelve admisible, condiciones y resultado=null cuando no se satisface la decisión. La interfaz sólo muestra original/juicio cuando admisible===true, mediante textContent. No exige juicio OK ni extrae/repara JSON. Evidencia parcial se presenta explícitamente aparte; nunca se promueve a resultado admitido. La cancelación recibida antes de la barrera de cierre revoca; desde sellando se responde NO_CANCELABLE: el cálculo ya ha terminado y se está fijando su custodia, sin admitir aún el resultado.

## R2 · Escritor único y barrera de sellado

El hilo de custodia es el único propietario de los archivos bajo evidencia/sellado/. El hilo de control no escribe archivos de evidencia ni espera a write/sync. Cada trabajo aceptado recibe ordinal; la cola FIFO admite 64. Sellar(corte) se inserta después de los trabajos ya aceptados y cierra la recepción inmediatamente en el controlador. El custodio verifica que recibió exactamente el ordinal de corte. Una cola llena marca fallo: no hay ACK conforme.

Tras recoger el hijo, se drenan pipes hasta ambos EOF o un máximo de dos segundos desde detección de terminación. Al expirar sin EOF se congela un prefijo incompleto con laguna_tardia_posible=true. Un tiempo vencido no acredita exhaustividad. Se procesa el trabajo ya encolado antes de calcular hashes. El objeto Disco se consume y sus descriptores se cierran antes de publicar Snapshot; el hilo escritor retorna. No hay ruta de reapertura/escritura desde la recuperación.

Snapshot conserva los bytes originales, cuatro archivos más manifiesto, y sello=SHA-256 de los bytes del manifiesto. Se sirve desde memoria inmutable, con archivo/offset/sello; un sello diferente se rechaza. Los cinco botones de recuperación quedan ligados al primer sello de esa sesión de interfaz y bloquean cualquier cambio. Incluso si un tercero modifica el archivo en disco, la API no mezcla esa modificación con el snapshot. Esto no es protección de disco frente a una cuenta o administrador comprometidos.

Datos recibidos tras la barrera no se añaden al conjunto sellado. Se mantiene contador saturado de bytes tardíos no custodiados en el estado de sesión; el manifiesto ya advierte la posibilidad de esa laguna y no pretende conocer su volumen futuro. **No se implementa suplemento**: se elige explícitamente pérdida declarada y acotada. Al terminar la sesión ese contador volátil puede perderse; no se afirma que toda llegada tardía quede registrada durablemente. La guarda exterior cierra descendientes/tuberías sin espera indefinida.

Error de write, sync, lectura para hash, identidad o creación/sync del manifiesto: ningún Snapshot válido, ninguna descarga API como conjunto sellado, ninguna nueva tarea. Podrán conservarse archivos parciales en la sede; su recuperación manual posterior requiere tratarlos como evidencia no sellada. No se repara retrospectivamente el manifiesto.

Límites conservados: frame 65536 bytes con LF; colas de lectura 64 y custodia 64; archivo 8 MiB; journals 20 MiB más entrada/manifiesto; margen de filesystem 2 GiB; una tarea por sesión/directorio nuevo. El snapshot añade hasta aproximadamente 20 MiB más overhead al RSS, que se contabiliza. Dos colas pueden retener aproximadamente 8 MiB de payload más metadatos. Son estimaciones, no medidas.

## R3 · Control que no espera a la custodia

El controlador sólo invoca try_send/try_recv y atomics para comunicarse con custodia. No comparte mutex de disco ni hace join del escritor bloqueado. Lectura/escritura de evidencia, hash, fsync, statvfs y snapshot pertenecen al hilo escritor. IPC de API mantiene timeout acotado; el cálculo Candle sigue en otro proceso. El control puede seguir sondeando y decidir parada mientras custodia está bloqueada.

nativa/parada.rs inicia un hilo de escalada dedicado: intento SIGTERM, deadline monotónico a 250 ms y SIGKILL; no hay write/sync ni espera por journal entre señales. Sus dos avisos usan try_send a cola 4; pérdida de aviso no suspende KILL ni se interpreta como evidencia íntegra. El padre evita recoger el PID hasta finalizar la escalada para reducir reutilización de PID antes de KILL. Se conserva la comprobación observable de terminación; una señal enviada no equivale a proceso detenido.

El sondeo no es tiempo real: planificación, /proc, kernel, permisos y llamadas de control pueden demorarse o fallar. Se precomprometen 250 ms de gracia monotónica, no una garantía de latencia máxima. La prueba exterior mide actuación y sobrepaso; no se sustituye la observación por el valor del temporizador. Errores de señal/observación quedan pendientes de medida, nunca se ocultan como éxito.

### Guarda exterior preparada, inactiva

nativa/guarda.rs es padre del supervisor y queda fuera de una hoja cgroup v2 vacía asignada expresamente. Verifica tipo domain, ausencia de subgrupos, controles y apertura permitida de cgroup.procs/cgroup.kill. Inicia supervisor esperando ARMED; lo incorpora a la hoja y confirma membresía antes de liberar el arranque. El supervisor exige padre e instrucción ARMED. Esto es protocolo entre componentes confiables, no autenticación frente al mismo usuario hostil.

La guarda recibe heartbeat por pipe, con hilo auxiliar separado de su control. Ausencia durante 2 s, muerte del supervisor, 40 min o RSS superior a 4 GiB provocan limpieza. El núcleo ofrece cgroup.kill para terminar el árbol de la hoja; cgroup.events permite comprobar populated. Se exige confirmación vacía durante hasta 5 s y recogida del supervisor; lectura fallida nunca prueba vacío. Drop vuelve a intentar limpieza en salidas por error. Referencia primaria: https://docs.kernel.org/admin-guide/cgroup-v2.html#core-interface-files (consultada 20/09/2026).

No se crea ni delega cgroup aquí; no sudo, instalación, cambio de permisos/controladores ni memoria.max. Que exista Codespaces no demuestra delegación disponible. Si falta capacidad, el mecanismo NO es viable en ese entorno y no se habilitan procesos de prueba bloqueados. Mantener vacía y exclusiva esa hoja hasta finalizar la guarda; verificar permisos que impidan migración y creación de subgrupos por hijos. Mismo usuario/administrador comprometido, proceso D-state, muerte de la guarda y bloqueo del kernel requieren un control superior de plataforma; no están resueltos por estas fuentes.

### RSS y costes

Se mantienen **4294967296 bytes** y la métrica RSS del campo stat.rss × PAGE_SIZE, no PSS ni memory.current. Supervisor/custodio (incluidos hilos/snapshot), servidor, inferidor, descendientes/observador dentro de la hoja y guarda exterior se suman. La enumeración por membresía cgroup evita perder huérfanos porque cambie PPID. Subgrupos inesperados producen error/parada, no un subtotal presentado como total. Muestras objetivo cada 1 s; control cada 50 ms. El cliente de prueba/CLI y navegador se identifican como instrumentos exteriores y su coste no se presenta como uso total del anfitrión.

El custodio conserva coste de escritura; sus tiempos mono_ns_custodio tienen origen propio. El controlador produce duraciones de observación/señales; la guarda y el observador tienen relojes monotónicos propios. No restarlos entre procesos; reloj civil Unix ms no es sello independiente. PSS y coste del propio manifiesto siguen desconocidos. La última muestra no equivale a medida instantánea.

Candle CPU mantiene caché de 4096 posiciones por capa; 2048 de contexto de aplicación no cambia ese constructor. Fórmula de carga útil: capas × 4096 × cabezas_KV × 2 × dimensión_cabeza × 4 bytes, excluyendo pesos/tablas/activaciones. Sin optimización, cambio de cuantización o conclusión causal nueva. Mantener DISENO de la preparación 01 como antecedente, no como descripción del cierre corregido.

## Alcance

Sin parser HTTP casero, librerías nuevas, Node/npm/Python del servicio, agentes auxiliares, ejecución de contenido del modelo, otras interfaces o celdas SV. OTel registra tramos instrumentados; no impone permisos ni exhaustividad. Los archivos históricos copiados no son resultados de la candidata. Todo comportamiento descrito es implementación propuesta en fuentes, pendiente de compilación, pruebas y recepción.

## Ajustes acotados NAT03

NAT02-A: el banco deja de identificar conformidad con cualquier rechazo. CASOS-NAT03 fija trece oráculos; se valida identidad de estímulo, terminal, condiciones de admisión, sello, prefijo, laguna y error exacto. El custodio incorpora recibos secuenciales por trabajo en el journal existente; no introduce otro archivo/servicio/dependencia. Estos recibos consumen los mismos límites de bytes y se incluyen en el coste de escritura.

La recuperación global se limita a 20 s, 1600 fragmentos y 42 MiB, incluidas dos pasadas y pausa. Cada solicitud usa el tiempo restante. El banco comprueba progreso, total constante, fin coherente y errores permitidos por fase. Los esperados de frames se codifican independientemente del testigo.

NAT02-B: fallar al encolar waitpid no propaga ?. Se conserva el resultado real de try_wait, se marca fallo de custodia, se revoca y se prosigue hacia cierre no admitido. Un error de try_wait no afirma reap: queda diagnosticado e inicia escalada una vez. Se revisan también observación del servidor, timeout de socket, medida RSS y evento inicial; el bloque de preparación de hijo conserva su captura explícita de errores.

Sin recogida confirmada después de cinco segundos de parada: API parada_no_confirmada durante una ventana adicional de dos segundos, después salida y guarda exterior. Si muere servidor/API, no se promete mantener transporte: el banco devuelve API_INDISPONIBLE y exige evidencia exterior; no es negativo conforme. La fuente de guarda permanece literal y sus límites/errores exigen revisión dinámica futura.

Los errores/avisos que no caben en custodia se muestran en diagnóstico **volátil** acotado: hasta 16 errores y 8 señales; no son datos durables ni resultado admitido. Se conserva el error del escritor cuando llega por canal. La revocación técnica por custodia se aplica incluso después del reap; parada_solicitada distingue si se ordenó señal. Ningún reap se inventa.

El inyector escritor_desconectado mantiene el proceso vivo hasta TERM/KILL; el escritor falla después del primer frame. Esto prepara el camino exacto de fallo del registro waitpid. Su ejecución, latencias, carreras y capacidades del kernel siguen pendientes.
