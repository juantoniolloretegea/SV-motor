# EIO-NAV-02 · diagnóstico observado y entrega

Fecha:20/09/2026. Resultado parcial para revisión receptora; ejecución consumida, sin reintentos.

## Identidad y decisión

Candidata preparada, publicada y releída66/66: [8744e3daf0c03f3675954ff774603454e44bfc45](https://github.com/juantoniolloretegea/SV-motor/tree/8744e3daf0c03f3675954ff774603454e44bfc45/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-navegador-02).
[Run35507386447](https://github.com/juantoniolloretegea/SV-motor/actions/runs/35507386447), workflow361319025, número9, intento1, job106069395638: completed/failure, retorno de campaña92. Inicio de plataforma2026-09-20T11:16:51Z; updated_at final11:21:11Z, sin atribuirle precisión de cierre del proceso.
La dirección autorizó expresamente publicación en main sin forzar historia, ejecución única y cierre de guardas incluso ante fallo. NAV01 y sus resultados se conservan.

**El supervisor interrumpió NAV05 por RSS agregada4667400192bytes (4,346855GiB), superior al umbral conservado4294967296bytes.** Se recibió la marca posterior a ModelWeights y la inmediatamente anterior al primer forward. No se recibió primer token ni resultado contractual. La inferencia completada sigue sin acreditarse.

## Matriz de resultados

| Caso | Observado | Dictamen acotado |
|---|---|---|
| NAV01 | JS/WASM real; control OK, objeto true, cuatro spans | Conforme en Chrome identificado |
| NAV02 | Control Rust existente OK, objeto true, cuatro spans | Conforme para entrada sintética A/B |
| NAV03 | terminate a2000,2ms; ID cancelado rechazado; nueva tarea58,4ms, caso2442,5ms | Conforme; evidencia de parada/revocación persistida antes del modelo |
| NAV04 | EVENTO_AUSENTE; objeto false, un descarte de consulta.B, tres spans | Control negativo conforme, no U |
| NAV05 | Entradas verificadas; entrada en Rust, tokenizador y ModelWeights completados; marca primer-forward-inicio | Interrupción por memoria; no tokens/EOS/salida/juicio del contrato |

[COTEJO_CONTROLES.json](COTEJO_CONTROLES.json) verifica orden, unicidad de span, trace y padres en NAV01/02/03-nueva/04. Los literales Rust originales se conservan como cadenas, sin convertir sus enteros temporales a otra representación.
[preflight-custodia.json](originales-recuperados/evidencia/preflight-custodia.json): ok=true, cuatro controles conformes, muestra RSS10 reciente, supervisor exterior activo, captura y evidencia NAV03 escritas. Complemento_journal_presente sólo acredita archivo, no suficiencia de PSS; su disponibilidad efectiva se contrasta en las lecturas.

## Localización de memoria: lo medido y lo desconocido

[Marcas originales](originales-recuperados/evidencia/marcas.jsonl), con secuencia de emisión, recepción en página/Node y escritura. Capacidad de memoria lineal WASM:

| Frontera recibida de NAV05 | Bytes lineales |
|---|---:|
| Módulo listo / recursos verificados / antes-enlace |1769472|
| Entrada efectiva Rust tras copias |409993216|
| Tokenizador listo / antes-ModelWeights |485687296|
| Después-ModelWeights |2739208192|
| Inmediatamente antes del primer forward |2739208192|

Entre antes y después de ModelWeights se observan2253520896bytes adicionales de capacidad lineal, en592ms según el mismo reloj del Worker. Entre antes-enlace y entrada Rust hay54,4ms. Estos datos localizan crecimiento en fronteras; no miden residencia física, asignaciones vivas ni atribuyen todo el pico a embeddings/rotación.
La marca primer-forward-inicio se emite antes de llamar forward. Su recepción no prueba que forward retornase. Ausencia de una marca posterior no demuestra que nunca se emitiera ni que no comenzase una operación posterior.
Se conservan los hallazgos estáticos del [precompromiso](../preparacion-navegador-02/README.md): copia del enlace, embeddings y preparación rotatoria. No se modificó Candle, cuantización, cargas, parámetros, prompt o dependencias.

[Muestras RSS](originales-recuperados/evidencia/navegador.muestras.jsonl):13 muestras. Último intervalo entre inicios de observación1,123032s; muestras12→13, RSS1942335488→4667400192bytes. En la última hay16procesos. PID11169, inicio33132ticks desde arranque, comm=chrome: RSS3310759936bytes; muestra12 del mismo PID573505536bytes. No se dispone de vinculación inequívoca entre ese PID y el Worker; rol más preciso no acreditado. Node PID11040 permanece incluido:75501568bytes al final. No se excluyó ningún proceso para reducir el agregado.
Los cuatro PIDs desaparecidos durante la campaña se conservan; errores de lectura stat registrados0. La familia se sigue por PID/inicio, sesión/grupo y descendientes observados; no incluye fugados antes de observarlos ni procesos ajenos.

[Complemento smaps_rollup](originales-recuperados/evidencia/navegador.complemento.jsonl):12 recorridos completos y uno final parcial. Último recorrido completo, muestra12: suma PSS1373797376bytes y Private_Clean+Private_Dirty1182846976bytes. Lecturas secuenciales en40,656ms, posteriores a la muestra RSS; no son una fotografía simultánea.
En el complemento13, iniciado después de ordenar la terminación,15/16 procesos no permiten lectura completa por desaparición. Sus campos quedan null con error literal. **PSS y privada agregadas del pico no disponibles**; no se suman parciales como si fueran totales ni se usan para revocar la parada RSS. [Diagnóstico receptor](DIAGNOSTICO_RECEPTOR.json) mantiene esos agregados como null.

## Coste y cobertura del observador

Periodo nominal1s. Intervalos efectivos entre comienzos:1,089886–1,154497s. Se conserva el método RSS anterior y sus cotas; el complemento opera en un hilo exterior con cola de una muestra, sin bloquear deliberadamente la decisión de parada. Ante exceso se envía SIGTERM antes de nuevas escrituras/métricas complementarias.
CPU registrada del hilo principal de muestreo:0,090532261s; hilo complementario:0,205818021s. No son coste instrumental total ni comparación causal con NAV01; son tramos medidos, con el perímetro de cada campo declarado. El coste_ciclo_completo_s está en navegador.medidas.json; el JSONL previo contiene tiempos hasta antes de su propia persistencia. Intervalos incluyen sobrecoste y planificación.
Supervisor PID11038, fuera del agregado previo: RSS observado hasta15138816bytes y final15532032bytes. Hilo complementario terminado, cola sin omisiones ni errores internos. Custodio exterior PID11249: RSS máximo27639808bytes, preparación0,370571038s/CPU0,370554361s, medido antes de emitir el texto, no coste total de transmisión. Ninguno ejecuta trabajo del modelo.
**Reserva de coste:** Node calculó duración de append/fsync en memoria y la envió a una variable de página que no está incluida en la instantánea exportada. No se conserva una medida completa del coste de escritura del controlador. No se corrige tras consumir la campaña; queda para decisión receptora.
Relojes monotónicos de Worker, página y Node tienen orígenes distintos; no se restan entre sí. Sus marcas civiles identificadas sirven de correlación con reserva de reloj/precisión. Date.now procede de milisegundos; representación Rust en ns no acredita precisión nanosegundo.

## Fases, entorno e identidades

[FASES.json](FASES.json): órdenes, inicios/fines, retornos, límites y residuales. stdout/stderr literales en originales-recuperados/evidencia/.

| Fase | Segundos | Retorno |
|---|---:|---|
| Adquisición |21,552644596|0|
| Herramienta |91,107096196|0|
| Construcción WASM |115,981183951|0|
| Enlace |2,219965473|0|
| Navegador |14,472925738|-15(SIGTERM), causa memoria|

Rust1.98.0(88d9e12ae2026-08-18); Cargo1.98.0(797e8a9bc2026-08-05); wasm-bindgen0.2.104. Chrome152.0.7977.82, CDP1.3/revisión@d04cdb24d67b081f6cf80200ffc5233f44b61109, JS15.2.124.21; Nodev22.23.2; imagen20260907.300.1. La huella de /usr/bin/google-chrome identifica el lanzador leído, no necesariamente el ELF cargado. Versión y órdenes originales conservadas.
Cargo.toml y lock del candidato171paquetes intactos respecto a NAV01; lock instrumental wasm-bindgen separado y conservado. No actualización por conveniencia.

| Recurso verificado en Worker | Bytes | SHA256 |
|---|---:|---|
| JS |9691|edd5f83e963d9fdc8d4d9dc3e8dfdf42e7369177574cb1772ad258fc467aa3a0|
| WASM |5606792|6b579fdb7f28ab627da4667a774f547253b540e49b5de307a1845dd10d220c8e|
| Pesos |396705472|ac2d97712095a558e31573f62f466a3f9d93990898b0ec79d7c974c1780d524a|
| Tokenizador |11422654|aeb13307a71acd8fe81861d94ad54ab689df773318809eed3cbe794b4492dae4|
| Petición |395|2ec34e21c96200a2106aed7f8a696e31ddbbe02e92238803a2c1727d40e1101f|

Disco propio+checkout máximo3138215936bytes, libre inicial92410114048bytes y mínimo89280057344bytes. Sólo runner; no medida del PC.
Máximos previos conservados: fases2080s, interno2280s, trabajo2400s; navegador200s e inferencia120s; disco10GiB/libre2GiB, transporte20MiB. El exceso RSS no es cumplimiento del presupuesto de memoria: es actuación de un umbral por muestreo, no límite duro.

## Captura y custodia

[Manifiesto emisor](MANIFIESTO_EMISOR.json) y [cotejo receptor](COTEJO_RECUPERACION.json):49/49 archivos,6044704bytes originales. Tamaños y SHA256 calculados antes de transporte, comparados tras recuperación; también tamaño/hash gzip y CRC32/tamaño al descomprimir. Segmentos completos y marcador final presentes. Texto de transporte por debajo de20MiB.
Los49originales incluyen los registros incrementales y la atestación exterior. Los textos legibles se guardan en originales-recuperados/ y todos los bytes, incluido WASM, en transporte/ (gzip/base64). No se publican pesos, tokenizador ni direcciones firmadas.
Recuperación offline en destino nuevo: `python3 RECUPERAR.py /ruta/nueva`. Sólo reconstruye/coteja; no ejecuta inferencia. En esta recepción se reconstruyó en memoria, no se ejecutó dicho script en el PC.
[MANIFIESTO_ENTREGA.json](MANIFIESTO_ENTREGA.json) identifica los archivos receptores, separadamente del emisor.

Atestación exterior:
- accesos:48líneas (incluida apertura); excepciones:1(apertura); marcas:14(incluida apertura); eventos:31(incluida apertura y decisión local).
- 94secuencias de escritura consecutivas en los cuatro journals Node;29secuencias de página consecutivas.13muestras y13complementos, más sus aperturas.
- Todas las líneas conservadas son JSON completo, sin truncamientos detectados. NAV03 tiene revocación/parada y tarea nueva conservadas antes del preflight.
- Última marca NAV05:secuencia de emisión8, primer-forward-inicio; última escritura94. No excluye mensajes emitidos aún en tránsito.
- No cierre normal del controlador; navegador.json finalizado=false. Los archivos finales accesos.json/excepciones.json/cierre-controlador.json no se generan; esta vez se conservan sus registros incrementales y la atestación exterior. No se fabrica cierre normal.
- Cero excepciones capturadas después de las aperturas no acredita ausencia universal de errores. chrome.stderr conserva DBus/DEPRECATED_ENDPOINT, sin atribuirles causalidad del exceso.

Integridad de archivos existentes no equivale a cobertura absoluta. Las huellas del mismo ejecutor no prueban independencia frente a un anfitrión malicioso.

## Cierre y recepción

Todas las fases registran residuales_observados=[]; navegador/Node/servidor observados terminados, complemento sin hilo pendiente. El puerto no recibió consulta independiente y no se inspeccionaron procesos ajenos del PC. El run está completed; guardas adquisición/ensayo false en el commit de esta entrega. No nueva ejecución, reintento ni cambio posterior del candidato.
Seguridad pasiva: ámbito/permiso ordinario; activa: revocación, rechazo y parada externa; observabilidad: marcas y medidas. Chrome conserva aislamiento ordinario sin --no-sandbox. Recursos de bucle local y CSP no acreditan encapsulación integral.

El diagnóstico localiza crecimiento material en la carga y un proceso Chrome dominante, pero no separa causalmente embeddings, rotación, copias y primer forward. Se propone únicamente para una autorización posterior revisar esas reservas con la evidencia obtenida y decidir una corrección o medición más fina, además de persistir el coste de escritura. No se ejecuta ni se eleva el umbral ahora.
Revisión receptora pendiente; S38 permanece separado. No aceptación científica, suficiencia de todas las células SV, privacidad general, seguridad universal o aptitud productiva.
