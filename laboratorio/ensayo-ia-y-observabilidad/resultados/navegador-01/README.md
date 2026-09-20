# EIO-NAV-01 · resultado observado · 20/09/2026

Estado: ejecución única consumida; entrega para revisión receptora. No aceptación científica.

## Identificación

- [Preparación ejecutada](https://github.com/juantoniolloretegea/SV-motor/tree/966b4b23326312371ad0512ef586c8d9ea401c6b/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-navegador-01/): commit 966b4b23326312371ad0512ef586c8d9ea401c6b. Originales, diferencias, matriz previa, órdenes, fuentes y locks conservados allí.
- [Actions run 35503596075](https://github.com/juantoniolloretegea/SV-motor/actions/runs/35503596075), workflow 361319025, número 8, intento 1, job 106059546860. completed/failure; salida del guion 92.
- Inicio de plataforma: 2026-09-20T09:54:41Z; updated_at final 09:59:50Z (no equivale a marca exacta de finalización). Duración del job mostrada: 5m05s.
- No reintento, corrección posterior del candidato ni nueva inferencia. Presupuesto adicional 1/1 consumido; el histórico anterior se conserva.

## Decisiones separadas

| Caso | Observación y decisión | Límite |
|---|---|---|
| NAV-01 | JS/WASM ejecutados en Chrome real; control OK, objeto true, cuatro spans concordantes. | Sólo este navegador/runner y esta entrada. |
| NAV-02 | Verificador Rust existente: OK, objeto true, cobertura completa. | Control sintético A/B; no suficiencia de todas las células SV. |
| NAV-03 | Sonda no cooperativa; orden terminate a 2000,2 ms; canal cerrado; respuesta tardía sintética rechazada. Tarea nueva OK en 58,8 ms; caso completo 2128,2 ms. | No acredita cese físico universal por ausencia de mensajes; familia observada cerrada por supervisor. |
| NAV-04 | Omisión exactamente de consulta.B: EVENTO_AUSENTE, objeto false, un descarte y tres spans conservados, incluido computo.fin. Control negativo conforme. | Rechazo por cobertura, no U. |
| NAV-05 | Recursos verificados y mensaje inferencia-inicio recibido; interrupción exterior por memoria. | Sin resultado, tokens, EOS ni dictamen del contrato. No ESTRUCTURA, no aceptación y no U. |

El mensaje inferencia-inicio se emite justo antes de invocar WASM síncrono: no determina qué punto interno de inferencia se alcanzó. No se atribuyen salidas nativas previas al navegador.
[Cotejo receptor de eventos](COTEJO_CONTROLES.json): número, orden, spans únicos, trace y relaciones padre comprobados en los cuatro controles finitos. Los literales Rust se preservan como cadenas originales en [navegador.json](originales-recuperados/evidencia/navegador.json).

## Fases y cotas

[FASES.json](FASES.json) conserva inicios/fines Unix, órdenes, retornos y perímetro. Salidas literales stdout/stderr y medidas en originales-recuperados/evidencia/.

| Fase | Segundos | Retorno | Pico RSS bytes |
|---|---:|---:|---:|
| Adquisición | 27,547600289 | 0 | 145645568 |
| Herramienta | 116,215202050 | 0 | 987103232 |
| Construcción WASM | 142,721812923 | 0 | 1524707328 |
| Enlace | 2,150396067 | 0 | 21065728 |
| Navegador | 7,579753727 | -15 (SIGTERM), causa memoria | 4671934464 |

RSS agregado observado 4,351078 GiB supera umbral 4 GiB (4294967296 bytes). El supervisor actuó; no es un límite duro. Muestreo nominal 1 s más coste de consulta; son posibles excesos entre muestras. La suma RSS puede contar páginas compartidas repetidamente: no mide memoria física única ni prueba una fuga, un OOM del sistema o la causa interna del crecimiento. Incluye controlador Node y familia Chrome observada; no se imputa al PC del usuario.
Disco máximo observado (directorio propio y checkout): 3133337600 bytes; libre inicial 92413480960 y mínimo 89283796992 bytes. Son medidas del runner.
Presupuestos previos: fases 2080 s, supervisor 2280 s, job 2400 s, navegador 200 s e inferencia 120 s; disco propio 10 GiB, libre mínimo 2 GiB, evidencia textual 20 MiB.

## Versiones, identidades y relojes

Chrome 152.0.7977.82, revisión CDP @d04cdb24d67b081f6cf80200ffc5233f44b61109; motor JS 15.2.124.21, protocolo 1.3. Node v22.23.2; imagen GitHub 20260907.300.1.
Rust 1.98.0 (88d9e12ae 2026-08-18), Cargo 1.98.0 (797e8a9bc 2026-08-05), wasm-bindgen-cli 0.2.104. Lock del candidato recibido: 171 paquetes conservados; lock instrumental separado: 253 paquetes, conservado en Cargo.herramienta.lock.
La huella de ejecutable registrada para /usr/bin/google-chrome identifica la ruta resuelta inspeccionada, que puede ser un lanzador; no se presenta como hash del ELF realmente cargado.

| Recurso verificado antes del uso | Bytes | SHA-256 |
|---|---:|---|
| JS de enlace | 9478 | 4df4bacdd8a3c2f0a2da941e9a9842fb8cc29573a03134765a85a4a342935a02 |
| WASM | 5606357 | 4b3ce0472b9376958b397918edc25d13562d07fd5db2215c8e3e04903d51c142 |
| Qwen GGUF | 396705472 | ac2d97712095a558e31573f62f466a3f9d93990898b0ec79d7c974c1780d524a |
| Tokenizador | 11422654 | aeb13307a71acd8fe81861d94ad54ab689df773318809eed3cbe794b4492dae4 |
| Petición literal | 395 | 2ec34e21c96200a2106aed7f8a696e31ddbbe02e92238803a2c1727d40e1101f |

Reloj de duración performance.now monotónico; civil Date.now. La representación Rust de Unix ns no acredita precisión nanosegundo: procede de milisegundos convertidos en f64 y conserva redondeos. No se sustituyen las cadenas literales por JSON normalizado en JavaScript.

## Captura, transporte y recuperación

[Manifiesto del emisor](MANIFIESTO_EMISOR.json): 39 archivos existentes, 5882290 bytes originales. El emisor calculó tamaño y SHA-256 antes de exportar; se recuperaron segmentos gzip/base64 del log, verificando también tamaño/hash comprimidos y CRC32/tamaño gzip. [Cotejo receptor](COTEJO_RECUPERACION.json): 39/39 identidades conformes, incluido WASM. No incluye pesos ni tokenizador.
Los textos originales están en originales-recuperados/; transporte/ conserva TODOS los originales en gzip/base64, incluido el binario. Para recuperación offline en un destino nuevo: `python3 RECUPERAR.py /ruta/nueva`. Este script sólo decodifica y coteja; no ejecuta el ensayo. La recuperación aquí se efectuó en memoria con decodificación y SHA-256, no mediante ejecución local de ese script.
MANIFIESTO_ENTREGA.json identifica los archivos de esta publicación, separadamente de las huellas del emisor. Identidad bajo el ámbito del emisor no demuestra independencia frente a un anfitrión malicioso.

**Laguna de captura:** navegador.json es una instantánea parcial, finalizado=false. Al recibir SIGTERM, el controlador no escribió accesos.json, excepciones.json ni cierre-controlador.json previstos para su cierre normal. El manifiesto completa=true significa transporte de archivos existentes, no cobertura completa del ensayo. errores=[] sólo describe la instantánea conservada; no prueba ausencia de toda excepción. Chrome.stderr conserva errores DBus y DEPRECATED_ENDPOINT, sin evidencia de que causaran la interrupción.

## Cierre y siguiente decisión

Todas las fases registran residuales_observados=[] tras limpieza de su familia (grupo/sesión y descendientes, PID+inicio). Incluye controlador/servidor y navegador observados; no constituye acuse de cierre normal del controlador ni consulta independiente del puerto. Excluye procesos fugados antes de ser observados y otros procesos del anfitrión.
Ambas guardas se cierran en el commit de esta entrega; el workflow conserva número8/intento1 y no se dispara de nuevo. No quedan trabajos propios de Actions en curso según el run final. Sin instalación, compilación o inferencia en el PC, ni cambios en Cloudflare/sitios, núcleo o registros canónicos.

Separación de mecanismos: permisos y ámbito de archivos son controles pasivos; rechazo de cobertura, invalidación del ID y parada exterior son activos; spans y medidas son observabilidad. Chrome se arrancó con aislamiento ordinario, sin --no-sandbox. Servidor/CDP en bucle local y recursos del mismo origen no prueban aislamiento integral de red; no se conserva un registro completo de accesos por la interrupción.

Se propone para una autorización nueva, sin ejecutarla ahora, localizar el crecimiento con medidas por proceso y memoria proporcional/privada, revisar copias y tiempos de vida de buffers y hacer persistente el registro de cierre ante señales. No se propone elevar automáticamente 4 GiB ni repetir esta ejecución. Revisión receptora pendiente; [S38 sigue separado](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/e3580fd21715aaf992bbbe19241070a5a2884935/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md#s38). No aceptación científica, privacidad general, seguridad universal ni aptitud productiva.
