# Diseño mínimo y correspondencia

## Componentes y capacidades propuestas

| Proceso | Fuente candidata | Capacidades previstas |
|---|---|---|
| servidor | nativa/servidor.rs; web/ | Axum/Tokio, escucha 127.0.0.1:3000; tres recursos identificados; POST /api; socket Unix; sin acceso intencionado a pesos ni escritura de evidencia |
| inferidor | nativa/inferidor.rs; inferencia/adaptador.rs | Lee dos archivos fijos en directorio de entradas; CPU/Candle; stdout con frames y stderr; sin API de red, sin comandos/herramientas del modelo |
| supervisor y custodio | nativa/supervisor.rs | Padre del servidor e inferidor; un intento; /proc para familia de procesos; señales a grupos propios; escritura exclusiva lógica de evidencia; manifiesto/cotejo |
| testigo | nativa/testigo.rs | Sustituye sólo al inferidor en escenarios deterministas; nunca acredita inferencia |
| cotejador receptor | nativa/cotejo.rs | Lee copia recuperada y compara cuatro tamaños/SHA-256, sin reparar ni reescribir |

Supervisor/custodio es un proceso con hilos, contabilizado conjuntamente; no se presenta como dos medidas separables inexistentes. Cliente web y navegador receptor se identifican aparte: no pertenecen al proceso remoto ni al umbral remoto, y su coste no se ha medido. Host, editor de Codespaces y agentes de plataforma tampoco se hacen desaparecer: consumen recursos reales fuera de la familia experimental, cuyo total no equivale al uso total de la máquina.

Permisos 0700 en evidencia y 0600 en socket reducen acceso de otras cuentas. env_clear evita heredar credenciales/variables de cuantización; no constituye aislamiento del sistema operativo. Misma cuenta y mismo kernel no aíslan de procesos hostiles/administrador. Se requiere revisar privilegios, lectura efectiva de entradas, permisos y política de red del entorno futuro; esta fuente no impone seccomp, namespaces, cgroup ni bloqueo de salida de red. No afirma privacidad.

## Transporte y exclusión

Axum analiza HTTP. IPC local es JSON con longitud big-endian u32 y límites, no un analizador HTTP casero. 4 conexiones de API en curso como máximo mediante semáforo, sin cola de inferencias. El supervisor procesa una orden por iteración, con 100 ms de timeout por lectura/escritura IPC. El servidor espera como máximo 2 s por respuesta. Un fallo de transporte significa resultado desconocido, no reintento.

Un ID de campaña ASCII de hasta 48 caracteres se configura antes; ID de tarea = campaña-01. Una vez consumido un intento, la sesión no acepta otro, aun tras fallo. Recuperar evidencia o consultar estado no inicia cálculo. Las rutas de ejecutables son hermanas del supervisor; la entrada es una elección entre dos literales. Nunca hay comandos o nombres de archivo interpretados desde la salida del modelo.

Origin y Host exactos más application/json restringen la frontera del navegador. No equivalen a autenticación. El puerto privado autenticado de Codespaces debe verificarse por una prueba negativa ajena a la sesión; no confiar en X-Forwarded-* como identidad. Si la plataforma transforma Host, se estudiará ese comportamiento antes de habilitar; no se relajará automáticamente la comprobación.

## Correspondencia con el corte público dde1a59fcd1bb23d9f146dff4e99750725ffa76f

| Antecedente | Tratamiento |
|---|---|
| pruebas/lib.rs | Verificador, condiciones, referencias, parada y frontera de fuentes preservados; añadido módulo nativa al final |
| pruebas/banco.rs, casos.json | Copias literales; esperados originales, sin ejecutarlos |
| pruebas/peticion.txt; petición estructurada | Copias literales; la interfaz incorpora los mismos strings, el receptor compara igualdad exacta |
| pruebas/ENTRADAS_ENSAYO.json | Copia literal histórica con sus reservas de procedencia; no se reescribe el estado histórico |
| inferencia/adaptador.rs | Misma plantilla, CPU, ArgMax/semilla, EOS, 128/2048/120s; añadidas marcas antes/después de carga y primer forward. Ninguna optimización |
| inferencia/main.rs | Sustituido sólo en esta candidata por inferidor.rs con archivos fijos, cotejo de identidad y frames de resultado separados |
| observabilidad/telemetria.rs | Conserva Vec acotado y banco de fallos; añadido envío incremental por stdout en cada exportación. Sin sink instalado, banco no genera evidencia exterior implícita |
| resultados/revision-06/json-01/banco.rs | Copia literal ubicada en pruebas/json_complementario.rs, que conserva sus include_str relativos |
| CASOS.json e INFERENCIAS.jsonl históricas | Copias literales necesarias para los 35 controles; jamás resultados de esta candidata |
| Cargo.lock | Copia literal en antecedentes; no lock de 0.2.0 |

Los archivos copiados y derivados se identifican en el manifiesto. No se alteran originales ni contratos canónicos del Lenguaje. Las referencias A/B son dependencias conjuntas sintéticas; no son vector, célula ni SV(9,3). No se usa HashMap para inventar orden.

## Parada y memoria

El modelo trabaja síncronamente en otro proceso; no se pretende detenerlo con abort de spawn_blocking. Tokio blocking se limita al IPC del servidor. El supervisor solicita SIGTERM al grupo del hijo, espera 250 ms y solicita SIGKILL. El estado no acredita parada hasta waitpid/try_wait y cierre de ambos pipes. Tras 5 s sin confirmar: parada_no_confirmada; el control de plataforma queda obligatorio. Drop intenta SIGKILL y comprobación acotada durante 1 s, sin espera indefinida. SIGKILL/OOM del propio supervisor y procesos D-state no quedan resueltos por esta fuente: requiere guarda de plataforma exterior antes del modelo.

Cancelación recibida antes del compromiso final exterior revoca la tarea, aun si el cálculo ya terminó. Después del compromiso se rechaza con NO_CANCELABLE. Se preservan frames posteriores pero no se presentan como admisibles. La terminación abrupta sin secuencia completa produce desconocida, sin transformar el fallo en U. Retorno 0 tampoco basta para cobertura o conformidad del modelo.

Umbral precomprometido: **4294967296 bytes**, suma RSS de supervisor/custodio, servidor, inferidor y descendientes observados por /proc. RSS = campo 24 de stat por PAGE_SIZE; start_ticks = campo 22; suma de páginas compartidas puede contar repetidamente; RSS aproximada no es PSS. Primera muestra antes de atender trabajo y luego intervalo objetivo 1 s; bucle 50 ms. Se emiten tiempos de observación y escritura, señales y reap. No existe garantía dura de latencia: IPC puede añadir 100 ms por fase, SIGKILL 250 ms después de SIGTERM, E/S o planificación pueden demorar el control. Se medirá intervalo real, demora y sobrepaso; no se inventa un máximo garantizado.

PSS queda null explícito. Identidades de procesos se registran como PID/start_ticks; los descendientes fugaces entre muestras pueden faltar. No hay protección acreditada frente a un proceso hostil que cambie de sesión/grupo o suplante PID. La candidata no crea tales procesos por diseño, pero la contención general necesita mecanismos de SO. El umbral agregado no es requisito de capacidad garantizada de Codespaces.

Candle fijado construye en CPU caché KV de 4096 posiciones por capa. Carga útil estimada por fórmula:
capas × 4096 × cabezas_KV × 2 × dimensión_cabeza × 4 bytes.
No se sustituyen dimensiones no medidas; excluye pesos, activaciones y tablas. El límite 2048 de la aplicación no reduce ese constructor. El cambio de entorno no prueba que desaparezca el pico. Fuentes:
- https://github.com/huggingface/candle/blob/ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a/candle-transformers/src/models/quantized_qwen3.rs
- https://github.com/huggingface/candle/blob/ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a/candle-nn/src/kv_cache.rs

## Custodia y costes

Frames JSONL UTF-8 con ID, contrato, secuencia, tiempo monotónico propio y civil Unix ms del emisor; se guardan los bytes con LF recibidos fuera del proceso. No se reserializan para sustituir el original. Stderr se conserva por separado. Colas 64 elementos × hasta 65537 bytes aproximadamente (más overhead), límite de frame nominal 65536; servidor cola 8 × 4096. No se prometen cuotas exactas de heap a partir de esa estimación.

Un registro demasiado largo, cola llena, pipe roto, secuencia/ID incorrectos o fallo de disco impiden completitud; pueden perderse bytes no persistidos. Lo emitido, recibido, encolado y sincronizado no se equipara. La lectura conserva fragmento final hasta límite; no garantiza recuperar bytes tras cierre de pipe o desbordamiento. Detener el hijo evita crecimiento ilimitado, no recupera lo perdido.

Por archivo 8 MiB, total de tres journals 20 MiB, margen disponible de filesystem 2 GiB; entrada y manifiesto pequeños adicionales. Una campaña por directorio nuevo, ninguna eliminación/rotación automática. Conservar y retirar conforme a custodia autorizada, no mediante esta candidata.

OTel exporta consulta.A, consulta.B, generacion.inicio, generacion.fin y raíz peticion; no instrumenta todo Candle ni demuestra razonamiento, permisos o exhaustividad. Los marcadores de carga/forward son complementarios. Oráculo exterior exige seis marcas y cinco spans fijados, además del resultado y secuencia; no deduce esperados contando lo recibido.

supervision.jsonl incluye contador exterior, tiempos propios, muestras por rol/total, ns de observación, bytes y coste acumulado write+sync_data de escrituras previas. El manifiesto añade coste hasta cierre del journal, excepto su propia escritura y hashing final, expresamente excluidos. RAM de custodio incluida en RSS. No hay medición separada exacta de CPU por función, PSS, tiempo de cola o coste completo del navegador; quedan desconocidos. No restar Instant entre procesos. Reloj civil depende del host/NTP y no acredita sello independiente.

El manifiesto enumera tamaño y SHA-256 de cuatro originales cerrados; no se auto-hashea. Transferir también su hash calculado exteriormente. La recuperación independiente coteja copia completa; identidad no prueba exhaustividad. Si hay error antes del manifiesto, entregar fragmentos y stderr como incompletos, sin generar un cierre conforme retrospectivo.
