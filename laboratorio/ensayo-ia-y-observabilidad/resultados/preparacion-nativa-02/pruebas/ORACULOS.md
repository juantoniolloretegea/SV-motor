# Oráculos NAT02 · preparados, no ejecutados

Se conservan los casos N01–N18 del [antecedente literal](../antecedentes/ORACULOS-NAT01.md). Las órdenes antiguas de lanzamiento directo se sustituyen por la guarda exterior; versiones antiguas de frontera no se usan para probar NAT02. No se heredan resultados 24/24 o 35/35 como evidencia de esta copia.

## Bancos y fases futuras

Sin ejecutar ahora. Tras recepción, autorización remota, lock nuevo, contención y gasto cero:

~~~sh
cargo +1.98.0-x86_64-unknown-linux-gnu test --locked --test frontera
cargo +1.98.0-x86_64-unknown-linux-gnu test --locked --test json_complementario -- --nocapture
cargo +1.98.0-x86_64-unknown-linux-gnu run --locked --bin banco
cargo +1.98.0-x86_64-unknown-linux-gnu build --locked --bins
~~~

Esperados: 7 tests de frontera; 35 controles JSON; 24 controles del banco. Ningún comando acredita interfaz, aislamiento, señales o persistencia. registro-futuro.sh conserva orden/stdout/stderr/retorno en directorio nuevo; se mantiene sin cambios. No construir feature inferencia para estos testigos.

## Contención previa obligatoria

Antes de un bloqueo, comprobar hoja cgroup v2 domain vacía y exclusiva, cgroup.kill/cgroup.events y permisos ya concedidos. No concederlos ni cambiar plataforma desde estos scripts. Sin esto, NO lanzar testigos bloqueados. Confirmar también control superior si la guarda muere o kernel no responde.

Preparar copia de INACTIVA.json en sede nueva, nunca editar la publicada. Contrato EIO-NAT/2, habilitada=true sólo bajo autorización futura, modo=testigo, caso elegido, campaña única, origen=https://eio.test para cliente local del banco. Esa URL nominal no requiere DNS: curl conecta al loopback y envía Origin/Host de prueba. No acredita autenticación del puerto remoto; ésta conserva su prueba independiente.

Lanzar una guarda por escenario, con rutas absolutas verificadas y directorio evidencia nuevo:

~~~sh
/path/bin/guarda /path/config-CASO.json /path/evidencia-CASO /path/entradas-CASO /sys/fs/cgroup/HOJA_EXCLUSIVA
~~~

Guardar stdout/stderr/retorno de la guarda desde un proceso receptor exterior. La guarda arma membresía antes del supervisor, y éste crea servidor y FIFO de inyección sólo si corresponde. No iniciar supervisor directamente; no sustituir guarda por cerrar una pestaña. Finalizar la sesión tras recuperar evidencia mediante señal al supervisor y verificar salida de guarda/hoja vacía; conservar fallos.

## API real y recepción concurrente

Desde el mismo entorno remoto, con curl ya disponible y servicio de testigo contenido:

~~~sh
/path/bin/api-real CASO
~~~

El binario usa HTTP real POST /api hacia 127.0.0.1:3000, atraviesa Axum, IPC, supervisor y custodia; no llama directamente a Admision. Cabeceras de prueba fijas, cuerpo literal, máximo 2 s por operación. Un intento por lanzamiento; no reintenta inicio. Las consultas son sondeos acotados, no nuevos ensayos. Un hilo consulta/recupera durante el sellado; sin sello exige rechazo. Después recupera cinco originales con el mismo sello, vuelve a recuperarlos cinco segundos más tarde y exige bytes idénticos y hashes concordantes. Retorno 0 sólo si los esperados fijos coinciden.

| ID | Configuración/estímulo | Esperado independiente |
|---|---|---|
| R1-01 | omision: falta consulta.B, seq renumerada, resultado final | admisible=false, resultado=null; original en prefijo sellado, evidencia_parcial=true |
| R1-02 | no_cero: secuencia y resultado completos, exit 77 | no admisible, no exposición de resultado admitido |
| R1-03 / R2-02 | eof_ausente: hijo hereda pipes, espera 4 s, emite TARDIO_TRAS_PLAZO | tras plazo de 2 s: prefijo incompleto sellado; salida tardía no cambia tamaños/SHA/sello; contador/laguna explícitos |
| R1-04 | tardio: cancelar, testigo recibe TERM y emite resultado durante gracia | revocada, no admisible aunque aparezca resultado; original preservado si se custodió |
| R1-05 | escritura: falla después del primer frame | no sellado correcto ni resultado admitido; no segunda tarea |
| R1-P | hostil: ejecución íntegra, juicio ESTRUCTURA | admisible=true técnicamente; texto original hostil y rechazo visibles; no HTML ejecutado |
| R2-01 | cola_pendiente: escritor retarda 50 ms cada frame, barrera tras eventos ya encolados | API no sirve antes de ACK; después todos los trabajos previos al corte presentes; mismas identidades |
| R2-03 | fallo_sync: error inyectado al cerrar | sin snapshot/resultado admisible; recuperación concurrente rechazada |
| R2-04 | bloqueo_sellado: open(FIFO) real en cierre, sin lector | no ACK, timeout 5 s, no resultado; petición concurrente no obtiene mezcla; liberar sólo bajo control exterior |
| R3-01 | bloqueo_antes: primer frame queda en open(FIFO) antes de pedir parada | controlador acepta cancelación y escalada progresa con E/S aún bloqueada |
| R3-02 | bloqueo_entre: escritura del evento TERM queda en open(FIFO) | KILL no espera al FIFO/journal; conservar observación exterior |
| R3-03 | SIGKILL deliberado al supervisor, no al testigo, en sesión contenida | guarda detecta salida/latido perdido, cgroup.kill, populated 0 y supervisor recogido; cualquier fallo no conforme |
| REG | normal y bancos previos | conformidad instrumental esperada, sin extrapolar al modelo |

En omision y eof_ausente la ausencia de salto de seq no basta para admitir. En no_cero un resultado previo tampoco basta. Para fallo_sync el error inyectado prueba propagación lógica, no un fallo real del dispositivo. R3 usa **open/write de FIFO efectivamente bloqueante**, no ese error inmediato.

## Observación independiente R3

api-real permite preparar cancelación y comprobar la API, pero por sí solo no acredita latencia de señales. Para R3-01/R3-02 se añade observador.rs y se arma ANTES de cancelar; usar órdenes HTTP de forma controlada en lugar de la cancelación automática de api-real:

1. Iniciar la petición literal por interfaz/cliente y consultar pid_inferidor. Verificar identidad del testigo y su pertenencia a la hoja, sin elegir un PID ajeno.
2. Lanzar observador ENTRADAS PID_TESTIGO dentro de la misma hoja experimental para contabilizar su RSS, pero como proceso independiente del supervisor/custodio. El lanzador autorizado debe incorporarse a cgroup.procs y ejecutar el observador; no conceder permisos nuevos. Comprobar que creó term-observador.sock y está activo.
3. En bloqueo_antes, comprobar en /proc/PID_SUPERVISOR/task/*/wchan que el escritor está esperando en la operación FIFO; el detalle exacto depende del kernel. En bloqueo_entre comprobar después de TERM. Si no puede observarse bloqueo real, no acreditar la prueba por tener el nombre de caso.
4. Solicitar cancelar por API. El testigo recibe TERM, avisa por datagrama no bloqueante al observador y permanece vivo. El observador registra recepción con su Instant y sondea /proc cada 10 ms hasta zombie/ausencia. Registrar también los avisos TERM/KILL del controlador cuando disponibles; su falta en el journal bloqueado no autoriza inventarlos.
5. Esperado: TERM observado, terminación posterior compatible con gracia 250 ms, sin depender de liberar FIFO. Evaluar latencia observada y demoras de transporte/planificación; no exigir falsamente 250,000 ms exactos ni comparar monotónicos de procesos distintos. Si falta evento/terminación, no conforme.
6. Conservar stdout del observador exterior al journal bloqueado. Mantener cgroup contenido; recuperar lo posible. Finalizar supervisor; guarda confirma hoja vacía. Un proceso D-state o limpieza fallida mantiene bloqueo para recepción.

El observador no administra recursos de plataforma ni crea privilegios. Su incorporación a la hoja y las comprobaciones de kernel requieren permisos efectivos, pendientes. No iniciar estas pruebas si sólo se dispone de una guarda documental.

## Interfaz real

Tras cada negativo, pulsar Consultar estado y contrastar DOM mediante INTERFAZ.js desde consola revisada de la página privada. No inicia trabajo; consulta la API y verifica original/juicio conforme al indicador. R1-P exige texto hostil visible como texto y ESTRUCTURA visible. Registrar captura/red sólo con autorización posterior; ninguna captura tomada ahora.

## Recuperación y cierre

Ejecutar cotejo sobre otra copia del conjunto sellado. Comparar sello con SHA-256 del MANIFIESTO exacto en emisor/receptor. No modificar prefijos, añadir tardíos o recalcular hashes para esconder fallo. La identidad puede ser conforme con evidencia declarada incompleta. Repeticiones para investigar carreras necesitan presupuesto explícito; esta preparación no autoriza ninguna ejecución.
