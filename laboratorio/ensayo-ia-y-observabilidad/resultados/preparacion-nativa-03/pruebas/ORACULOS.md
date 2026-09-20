# NAT03 · oráculos precomprometidos, no ejecutados

La tabla [CASOS-NAT03.json](CASOS-NAT03.json) y las funciones de [oraculos_nat03.rs](oraculos_nat03.rs) fijan los esperados del banco HTTP. La configuración publicada continúa inactiva. Se rechaza un caso desconocido antes de acceder a la API y se exige identidad de modo, caso y contrato antes de iniciar y al consultar. No se deducen esperados de la respuesta del supervisor.

Cada caso requiere sesión/directorio nuevos, modo testigo, contrato EIO-NAT/2, origen nominal https://eio.test, petición referencia literal, un intento. Se conserva la lista antigua del inyector, pero api-real sólo admite los trece casos de esta tabla. Los demás no reciben un negativo genérico conforme.

| Caso | Terminal | Admisible / revocada | Sello | Contenido exigido | Laguna | Error de custodia / salida |
|---|---|---|---|---|---|---|
| normal | terminada | sí / no | exigido | 12 frames completos | no | ninguno; exit 0 |
| hostil | terminada | sí / no | exigido | 12; texto hostil literal y juicio ESTRUCTURA | no | ninguno; exit 0 |
| cola_pendiente | terminada | sí / no | exigido | 12; recibos de trabajos 1..corte, completos y ordenados | no | ninguno; exit 0 |
| omision | desconocida | no / no | exigido | 11: seis marcas, cuatro spans sin consulta.B y resultado; seq renumerada | no | ninguno; exit 0 |
| no_cero | desconocida | no / no | exigido | 12 completos; original recuperable, nunca resultado admitido | no | ninguno; exit 77 |
| eof_ausente | desconocida | no / no | exigido | 12 anteriores al corte; excluye TARDIO_TRAS_PLAZO | sí | ninguno; padre exit 0 |
| tardio | interrumpida | no / sí | exigido | 12, incluido resultado emitido después de TERM | no | ninguno; SIGKILL 9 y reap observados |
| escritura | interrumpida | no / sí | prohibido | no conjunto sellado; parcial exterior no admitido | no determinable por manifiesto | ESCRITURA_INYECTADA; exit 0 o SIGKILL 9 según carrera, este último con parada solicitada |
| fallo_sync | interrumpida | no / sí | prohibido | no conjunto sellado | no determinable | SYNC_INYECTADO; exit 0 |
| bloqueo_sellado | interrumpida | no / sí | prohibido | no ACK en 5 s | no determinable | SELLADO_PLAZO; exit 0 |
| bloqueo_antes | interrumpida | no / sí | prohibido | control exterior mientras escritor espera FIFO | no determinable | SELLADO_PLAZO; SIGKILL 9 y reap |
| bloqueo_entre | interrumpida | no / sí | prohibido | control exterior mientras journal espera FIFO | no determinable | SELLADO_PLAZO; SIGKILL 9 y reap |
| escritor_desconectado | interrumpida | no / sí | prohibido | registro_waitpid_fallido=true, diagnóstico registro_waitpid/COLA_CUSTODIA | no determinable | ESCRITOR_DESCONECTADO_INYECTADO; SIGKILL 9 y reap |

En todos los negativos: resultado=null, segundo inicio rechazado. Los casos con parada prescrita exigen además los avisos SIGTERM y SIGKILL con resultado Ok(()), y waitpid con señal 9: el envío por sí solo no demuestra terminación. La invalidación por custodia revoca técnicamente incluso si el hijo ya terminó; no significa que se le enviaran señales retrospectivamente.

## Contenido y ordinales

El oráculo contiene literales independientes del inyector: seis marcas en orden; cinco spans completos (cuatro en omision); resultado sintético exacto; contrato/ID, seq consecutiva desde 1 y mono_ns no decreciente. Exige entrada literal y stderr vacío. El resultado admitido de API debe coincidir con el frame final recuperado. No se usa sólo el número de eventos.

supervision.jsonl añade recibos de trabajo después de la operación: ordinal, clase, archivo/bytes/SHA para datos y entrada, o tipo/datos del evento inmediatamente anterior. Se exige cadena 1..corte_custodia, consumos exactos de stdout/stderr, entrada única y cierre final con ese corte. El recibo permite detectar un trabajo ausente aunque las dos descargas sean idénticas. Los 12 frames precomprometidos impiden que una omisión renumerada pase como cola completa. Un recibo no acredita por sí solo durabilidad frente a caída; falta probar sync y recuperación.

Cada manifiesto debe inventariar exactamente los cuatro originales sin duplicados; tamaños, SHA-256, estado, completa y laguna deben concordar. El quinto archivo, MANIFIESTO.json, se coteja contra el sello. Se recupera el mismo conjunto dos veces, separado por cinco segundos.

## Recuperación y carreras

Un plazo monotónico global de **20 s** cubre todas las consultas de recuperación, las dos pasadas de cinco archivos y la pausa de cinco segundos; queda también dentro del máximo de 60 s del banco. Máximo **1600 fragmentos y 42 MiB** decodificados agregados; archivo hasta 8 MiB, fragmento hasta 32768 bytes. Cada curl usa como máximo min(2 s, plazo restante); se comprueba vencimiento antes y después. La detección de cierre tiene máximo 12 s. No hay reintentos de inicio.

Estas cotas cubren conservadoramente dos conjuntos de hasta 20 MiB de journals más entrada/manifiesto; los testigos previstos producen mucho menos. No garantizan completar cualquier conjunto al límite bajo cualquier latencia: el exceso se declara fallo. La contención exterior sigue siendo necesaria si el SO no devuelve una llamada.

Se exige identidad constante, offset exacto, total estable, avance positivo y fin equivalente a alcanzar total. Sólo se admite fragmento vacío para archivo vacío: offset=0, total=0, fin=true. Error, cambio de total, ausencia de avance o exceso global abortan el banco.

Las consultas se intercalan mientras el supervisor drena/sella, sin hilo que pueda quedar abandonado. Antes del sello sólo se admite CONJUNTO_NO_SELLADO exacto; SELLO_DISTINTO sólo en la carrera comprobada por una nueva consulta que muestra sello. Después del sello se exige fragmento correcto, identidad y prefijo estables. Ningún error arbitrario ni fallo de transporte es conforme.

## Sensibilidad preparada, no ejecutada

Los cuatro tests añadidos a frontera.rs llevan el total fuente a once. Conservar controles anteriores sin atribuirles ejecución nueva.

| Mutación aislada sobre datos de prueba | Rechazo exigido |
|---|---|
| Quitar sello requerido de un terminal normal válido | SELLO_EXIGIDO_O_PROHIBIDO |
| Caso inexistente | CASO_DESCONOCIDO |
| Caso de la configuración distinto del solicitado | ESTIMULO_DISTINTO |
| Quitar trabajo manteniendo el corte; alterar su ordinal | TRABAJO_AUSENTE / TRABAJO_ORDINAL |
| Fragmento vacío con fin=false | FRAGMENTO_SIN_PROGRESO |
| Cambiar total tras primer fragmento | TOTAL_CAMBIANTE_O_LIMITE |
| Rebasar número global de fragmentos o vencer reloj | RECUPERACION_FRAGMENTOS / RECUPERACION_PLAZO_GLOBAL |
| Error arbitrario durante fase sin sello | ERROR_CONCURRENTE_DISTINTO |

Los originales nunca se mutan: los controles trabajan con valores sintéticos en memoria, cuando se autorice ejecutarlos.

## NAT02-B y prueba exterior

escritor_desconectado emite inicio y permanece vivo hasta TERM/KILL. El escritor consume el primer frame y falla de forma inyectada, desconectando el receptor. Al recoger al hijo, el envío del registro waitpid falla; se exige conservar reap real, invalidar custodia, no emitir sello/resultado y mantener la API terminal. El banco debe fallar si no observa ese camino exacto.

Los campos de diagnóstico son volátiles, acotados (16 errores y 8 avisos); deben ser capturados por el receptor exterior. No reemplazan un journal que no pudo escribirse. Si la API no puede mantenerse: categoría API_INDISPONIBLE, o PARADA_NO_CONFIRMADA si sigue consultable sin reap. Ambas son **incidencias no conformes**, nunca equivalentes a un negativo aprobado.

Conservar entonces orden/stdout/stderr/retorno del banco y guarda, últimas respuestas API, PID y start_ticks antes de la incidencia, muestras /proc, resultado de cgroup.kill, cgroup.events con populated 0 y recogida del supervisor. Si falta cualquiera de las pruebas de cierre, declarar parada no acreditada y usar el control superior previamente comprobado.

Los FIFO y la latencia requieren la observación exterior descrita en [NAT02](https://github.com/juantoniolloretegea/SV-motor/tree/d82bbe2f5f396eb31da5b095ba0849404cb352b0/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-nativa-02/pruebas/ORACULOS.md), con estas precisiones: el banco automático no acredita por sí solo latencia; el receptor deberá armar el observador durante la ventana anterior a cancelación (500 ms) y probar su preparación. Si no alcanza esa ventana, se cierra el intento sin conformidad ni reintento. El datagrama del escritor desconectado es auxiliar y puede perderse; su ausencia no se sustituye por un timestamp inventado. En este caso, señales y reap de API más cierre exterior son obligatorios. La muerte deliberada de supervisor se evalúa aparte, no mediante un caso desconocido de api-real.

No se han iniciado servidores, testigos, observadores, interfaz ni pruebas en esta preparación.
