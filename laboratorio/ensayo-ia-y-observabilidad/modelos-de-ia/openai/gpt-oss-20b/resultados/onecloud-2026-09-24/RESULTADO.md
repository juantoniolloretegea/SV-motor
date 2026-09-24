# OC-01 y OC-02 · Primera respuesta correcta y contraste en OneCloud

**Registro:** 2026-09-24T11:44:10Z. **Unidad:** Agente Watson / W-S39-02. **Ámbito:** investigación lateral EIO, S39 / TT-0012. Dos inferencias autorizadas y secuenciales; originales completos conservados. Antecedente: [recuperación y regresión CPU](../recuperacion-onecloud-2026-09-24/INFORME.md).

## Resultado

La candidata corregida responde **«Hay cinco elementos en total.»**. El ejecutable anterior, en la misma máquina y con la misma petición, produce texto inconexo. OC-01 satisface el criterio aritmético previo; OC-02 no lo satisface. Ambos completan HTTP y se detienen bajo el controlador.

| Medida | OC-01 · candidata | OC-02 · anterior |
|---|---:|---:|
| Inicio UTC | 2026-09-24T11:23:23.602Z | 2026-09-24T11:34:11.519Z |
| Fin del controlador UTC | 2026-09-24T11:26:37.325Z | 2026-09-24T11:35:47.090Z |
| HTTP | 200 | 200 |
| Terminación de generación | stop | length |
| Tokens de entrada / salida | 60 / 7 | 60 / 16 |
| Tiempo del motor: entrada | 96,918 s | 40,097 s |
| Tiempo del motor: salida | 15,616 s | 21,055 s |
| Tiempo total informado por el motor | 112,534 s | 61,152 s |
| Salida, tokens/s informados por el motor | 0,4482582 | 0,7599145 |
| Duración total del controlador | 193,723 s | 95,571 s |
| Pico RSS muestreado del hijo | 25 163 407 360 B | 25 166 041 088 B |
| Lecturas RSS / ausentes | 3482 / 0 | 1401 / 0 |
| MemoryPeak conservado por systemd | 13 151 395 840 B | 13 174 923 264 B |
| Registros JSONL conservados | 192 | 95 |

La duración del controlador incluye comprobación de identidades, carga, petición y cierre; no debe confundirse con la medida del motor. Los siete tokens de OC-01 incluyen la señal especial de terminación. Los registros periódicos JSONL no son las 3482/1401 lecturas internas: el controlador resume su muestreo en registros menos frecuentes. RSS y memoria cargada al cgroup son ámbitos contables distintos; no se suman ni se identifican sus picos. El RSS incluye páginas de archivos mapeados que pueden estar cargadas a otro grupo, sin acreditarse aquí un reparto completo.

**Texto completo de OC-02, en notación JSON que conserva el espacio final:**

```json
"... <|constrain|>1. 1}... 1. 1... "
```

La valoración de contenido es del agente, aplicada al criterio publicado; el controlador conserva literalmente `content_validation: no_realizada`. No se modifica esa declaración instrumental.

## Método e identidades

Los protocolos se publicaron antes de sus respectivas ejecuciones: [OC-01](PROTOCOLO_OC01.md), commit `9a90884f9e537cffc7929c6b3518ece971728343`, y [OC-02](PROTOCOLO_OC02.md), commit `c2e7695fabc40685ae712790e4a31d2e9ab9f631`. Se conserva el [comando de cada servicio](EJECUCION.sh).

Petición literal Harmony, canal final abierto, /v1/completions, temperatura 0, máximo 16 tokens, logprobs 1, sin streaming. CPU, precisión solicitada BF16, contexto 1024 y una secuencia; caché de prefijos 0 y paged attention desactivada. La precisión solicitada no certifica la precisión de todas las operaciones internas. No se habilitaron herramientas ni ejecución de texto del modelo.

GGUF ggml-org/gpt-oss-20b-GGUF, revisión b97cbb20d1995efd41dce8c4dd1ddf86e8db375b, 12 109 566 624 B, SHA-256 `27cd6c432c7672cb812a92f611cf3ba7bbc35928262bb1e1253ff4ee6ae35901`. Se descargó otra vez y se cotejó la huella antes de utilizarlo; cada controlador volvió a comprobarla. Tokenizador y Harmony comparten archivos y [manifiesto auxiliar](evidencias/IDENTIDADES_AUXILIARES.sha256), cotejado en destino.

| Artefacto | SHA-256 |
|---|---|
| Motor candidato | 2d6856918349d85a073fea59190c59886e780a62bcd94c6d7789060d04e99fb1 |
| Motor anterior | 0bd54bd0d17eb79be22b21cccaac0a3f32add5918f93ea162a888feccd7a2158 |
| Controlador OC-01 | dfc8afe8ce7c76019fc35687c481927b429ea731e69058444b86a6c151c6554f |
| Controlador OC-02 | 4b4a8923da88e0c492552595cd0368a9ff5fedc471a24523788b9ed734939091 |

Fuentes instrumentales y Cargo.lock completos en [controlador OC-01](evidencias/controlador-admision-revisada) y [controlador OC-02](evidencias/controlador-referencia-oc02); [diff](evidencias/DIFERENCIA_OC02.patch) limitado al hash esperado y ruta del motor. Compilación offline y bloqueada con [Rust 1.98.0](evidencias/RUSTC.txt). El [banco instrumental](evidencias/GUARDAS.log) ejecutado en OneCloud declara nueve pruebas aprobadas, incluida una auxiliar; la invocación hija de esa auxiliar no se cuenta como décima prueba independiente. Comprueba admisión, límite virtual impuesto por Linux, muerte del hijo al desaparecer su padre y selector de modelo. No es la suite completa de mistral.rs.

## Entorno, contención y cierre

Ubuntu 26.04 LTS, KVM, AMD EPYC 7502, 12 CPU virtuales, 64 GB nominales, sin swap. Memoria disponible inicial: 66 369 990 656 B en OC-01 y 66 359 853 056 B en OC-02. El inventario anterior se conserva en el informe de recuperación. No hubo otra inferencia concurrente de esta campaña.

Ventana del controlador 540 s; carga 180 s; petición 300 s; RLIMIT_AS 32 GiB por proceso; reserva de entorno 512 MiB. Servicio systemd con MemoryMax 32 GiB, MemorySwapMax 0, TasksMax 256, RuntimeMaxSec 570, TimeoutStopSec 15, KillMode control-group, red privada, PrivateTmp, NoNewPrivileges, ProtectHome y ProtectSystem strict. El preflight lee el cgroup real y solo interfaz de bucle local. Se conservan propiedades efectivas de ambos servicios y observaciones exteriores durante su ejecución.

**Límite conocido del controlador:** consulta el cgroup raíz visible, cuyos campos resultan null, y usa disponibilidad global para la admisión. Su umbral no-file resulta mayor que la cuota exterior; la contención agregada de esta ejecución la aporta systemd. No se presenta el controlador como un lector correcto de todos los ancestros ni se cambia su algoritmo entre casos.

Las muestras exteriores durante ejecución mostraron cero eventos max, oom y oom_kill en ambos casos. **No se dispone de los contadores finales:** el directorio del cgroup desapareció al terminar. OC-01 conserva campos finales vacíos y un asiento posterior de esta limitación; OC-02 conserva además los errores de lectura. Se distingue esa ausencia de las métricas finales retenidas por systemd. No se declara una prueba de saturación ni de imposición de la cuota de cgroup.

Cada registro final confirma error null, respuesta recibida, hijo detenido y lista vacía de errores de parada. El controlador solicita SIGTERM al grupo que creó después de recibir la respuesta; la señal 15 de salida es ese cierre previsto, no un fallo del modelo. Comprobaciones exteriores: servicios inactivos y PID del hijo ausente; OC-02 conserva también ausencia del controlador. PID e instante de creación figuran en JSONL. Ambas inferencias están terminadas; la VM sigue disponible.

## Interpretación, reservas y decisión

1. La respuesta correcta demuestra viabilidad de este recorrido nativo para este caso concreto. Resuelve el objetivo material de TT-0012: petición sintética, respuesta atribuida al proceso instalado, configuración, consumo y cierre conservados. Su recepción canónica debe registrar finalización con estas limitaciones; S39 mantiene seguimiento abierto.
2. El fracaso semántico del ejecutable anterior en el mismo anfitrión muestra que la migración y la memoria adicional no bastan por sí solas en este contraste. La candidata permite continuar la evaluación.
3. La regresión unitaria previa identifica un defecto específico en la difusión de la entrada MXFP4 compartida entre expertos. Este par de inferencias compara paquetes ejecutables de construcciones distintas: **no acredita causalidad exclusiva del parche**. Una atribución más fuerte requeriría reconstrucción apareada con idéntico entorno y solo esa diferencia.
4. Hay una observación por ejecutable, sin repetición, aleatorización del orden ni control de cachés. No es un benchmark ni un cálculo de fiabilidad. Los tiempos son descriptivos; no se interpreta la mayor rapidez del ejecutable defectuoso como ventaja útil. El rendimiento de la candidata es insuficiente para prometer conversación fluida.
5. No están verificadas tareas amplias, respuestas largas, chat automático, integración web ni conformidad integral de la vía B. Tampoco es prueba de vía A ni de instalación PC. Qwen/B y S42/TT-0010 conservan sus alcances. Los muestreos históricos de pesos y la regresión específica no se convierten en certificación global.

La continuación razonada es formular un pequeño banco de tareas diversas, con oráculos y presupuestos previos, antes de integrar conversación. Este informe cierra las dos ejecuciones descritas; no registra como realizada esa futura campaña.

## Evidencia y custodia

- [Resultados estructurados](RESULTADOS.json), derivados sin sustituir originales.
- [OC-01 · SUCESOS completo](evidencias/resultados/OC01/SUCESOS.jsonl) y [motor](evidencias/resultados/OC01/motor.log).
- [OC-02 · SUCESOS completo](evidencias/resultados/OC02/SUCESOS.jsonl) y [motor](evidencias/resultados/OC02/motor.log).
- [Manifiesto de todos los archivos](MANIFIESTO.json), [control documental](CONTROL_DOCUMENTAL.json), [preflight](evidencias/PREFLIGHT.log), [observación OC-01](evidencias/resultados/OC01_EXTERIOR.log), [observación OC-02](evidencias/resultados/OC02_EXTERIOR.log).

El paquete original tiene SHA-256 `7cb1a19dbc6fcd57bfab1bdbb4e0bd16a70a72577647cb13b09e108aebf3efd9`; la recepción local coincide con OneCloud. Se publican los archivos extraídos completos. Una primera extracción local no pudo reproducir propietarios de archivo de Codespace; se repitió conservando contenidos sin trasladar propietarios. La huella del paquete y las identidades de las fuentes se verificaron; ese incidente de custodia no afecta a la inferencia. Una lectura auxiliar con rg no disponible en la VM se repitió con grep, sin repetir ninguna inferencia.

**Calidad:** PTA-SVM-003; recepción prevista S39 revisión 17, TT-0012, Acta004 §17, RETP-2026-270 y PTA-2026-012. Sin modificación del núcleo, rectores ni mapa HTML. Licencias de terceros y aviso del SV conservados en cada paquete del controlador. Sistema Vectorial SV · CC BY-NC-ND 4.0.
