# gpt-oss-20b · Continuación nativa y diagnóstico de carga

**23 de septiembre de 2026 · Estado: ejecución de inferencia pendiente.**

**Rectificación posterior:** la [revisión del controlador y de la cuantización](RECTIFICACION_CONTROLADOR.md) incorpora los intentos 09–12. Las menciones Q2K/Q3K identifican opciones solicitadas; no acreditan una representación residente con esos tipos. Para los expertos de este modelo, el código del motor selecciona Q4_0 por incompatibilidad dimensional con los bloques K. El corte inicial 01–08 se conserva a continuación y no sustituye esa rectificación.

Se ejecutó el motor instalado con supervisión Rust y medidas conservadas durante la carga. **Ninguno de los ocho intentos alcanzó el servicio HTTP ni envió la petición de inferencia.** Se confirmó la terminación de cada hijo. La instancia de Codespaces quedó detenida, comprobado a las 11:22 UTC.

El diagnóstico distingue una señal externa observada en dos intentos y las paradas instrumentales por tiempo o RSS. No identifica el servicio exterior emisor, no demuestra que todas las interrupciones tengan la misma causa y no acredita inviabilidad general del modelo.

## Configuración y evidencia de partida

| Elemento | Identificación |
|---|---|
| Fuentes de supervisión de partida | SV-motor `8cddcc83359bf6733a360d5bba2cd72426f8b631`; documentación de entrada `bf488a86dcb1a311ddf4355345e267901adad7c9`. |
| Plataforma | Linux en Codespaces; 4 núcleos, 16 GB nominales, sin swap observado. |
| Compilador | Rust y Cargo 1.98.0, invocados explícitamente. |
| Motor | mistral.rs 0.9.3, binario CPU; revisión `24dbf5c256f232176ee5949485ba264049407fbe`. SHA-256 `0bd54bd0d17eb79be22b21cccaac0a3f32add5918f93ea162a888feccd7a2158`. |
| Pesos | Distribución oficial gpt-oss-20b MXFP4, revisión `6cee5e81ee83917806bbde320786a8fb61efebee`; archivos de la instalación anterior conservados, sin nueva descarga ni conversión en disco. |
| Conversación | Harmony 0.0.8 en el motor; vocabulario local. No se llegó a comprobar una respuesta real. |
| Servicio previsto | `127.0.0.1:8089`, una secuencia, caché de prefijos deshabilitada, atención paginada deshabilitada, sin interfaz integrada ni herramientas habilitadas. |

La precisión de activaciones solicitada fue BF16. `--max-seq-len 1024` corresponde a la estimación del reparto de memoria; no limita por sí solo el contexto de ejecución. El intento 05 reintrodujo por error `--max-model-len 1024`, cuya incompatibilidad ya figuraba en el primer intento histórico. El cargador la rechazó antes de cargar pesos y se retiró. Es un error de configuración de esta continuación, no un hallazgo nuevo del modelo.

La petición prevista permanece en las fuentes: caso sintético de tres elementos más dos, temperatura 0, máximo 96 tokens, esfuerzo bajo. **No se envió.** No hay resultado de calidad, evaluación clínica ni interacción con el núcleo del SV.

## Supervisión y variantes

La admisión conservó el inventario previo de 13 123 MiB como estimación mínima, no como máximo de consumo. Se declaró una reserva de 512 MiB mediante argumento explícito; el valor predeterminado del controlador sigue siendo 1 GiB. La RSS del hijo se comparó con la disponibilidad inicial menos esa reserva. Se mantuvieron la observación de disponibilidad, el límite virtual `RLIMIT_AS` de 32 GiB, los plazos, la custodia y el cierre TERM/KILL. Una cota de direcciones virtuales no es una reserva de RAM ni una cuota agregada.

| Controlador | Cambio aplicado | Intentos |
|---|---|---|
| 0.1.2 | Reserva explícita entre 512 y 4 096 MiB; valor predeterminado sin cambio. | 01–02 |
| 0.1.3 | Identificación de la lectura fallida de `/proc`, estado del proceso y reconsulta de 250 ms; sin admitir HTTP mientras falte atribución del puerto. | 03–04 |
| 0.1.4 | Variante con topología Q8_0 para vocabulario y salida y opción de contexto rechazada. | 05 |
| 0.1.5 | Topología selectiva, sin la opción de contexto rechazada; expertos MXFP4 conservados. | 06 |
| 0.1.6 | Recuantización Q3K durante la carga, con topología solicitada Q8_0 para vocabulario y salida. | 07–08 |

En 08 se añadió `MISTRALRS_ISQ_SINGLETHREAD=1`, opción identificada en el código del motor. Los archivos originales no se modificaron. La recuantización supone una configuración numérica distinta y puede reducir la precisión. Al no terminar la carga, tampoco se certifica la composición final de una instancia residente. Estas variantes quedan como evidencia experimental; no se adoptan como configuración operativa. El controlador de referencia publicado conserva la carga original de 0.1.3.

## Resultados medidos

Duración desde el inicio del controlador hasta su resultado final. RSS máxima **muestreada del hijo**, en bytes; no máximo exacto ni memoria total del anfitrión. Los intentos 04–08 se ejecutaron con `strace`; sus tiempos no constituyen una comparación de rendimiento con los restantes.

| Intento | Duración (s) | RSS máxima muestreada (B) | Resultado |
|---|---:|---:|---|
| 01 | 506,154 | 13 423 575 040 | Plazo de carga de 500 s alcanzado; cierre solicitado por el controlador. |
| 02 | 211,689 | 14 032 244 736 | Error de observación `Permission denied`; cierre conservado. Sin traza del emisor inicial. |
| 03 | 48,724 | 14 069 985 280 | Error al enumerar descriptores, con SIGTERM pendiente antes del cierre propio. El intento posterior de adjuntar `strace` llegó cuando el proceso ya había terminado. |
| 04 | 48,980 | 14 043 533 312 | SIGTERM externo registrado antes de la orden de cierre propia. |
| 05 | 0,439 | 8 675 328 | Opción de contexto no admitida; salida 1 antes de cargar pesos. |
| 06 | 76,904 | 13 990 146 048 | SIGTERM externo registrado durante la carga con topología selectiva. |
| 07 | 25,424 | 14 148 222 976 | Límite de RSS muestreada alcanzado durante Q3K. |
| 08 | 50,549 | 14 141 001 728 | Límite de RSS muestreada alcanzado con cuantización secuencial. |

En 02–08 se configuraron 1 000 s para carga, 450 s para petición y una ventana de 1 500 s. Por tanto, sus interrupciones no se explican por el plazo de 500 s del intento 01. Los registros contienen 904 muestras persistidas de recursos entre los ocho intentos, además del muestreo interno más frecuente. `RESUMEN_VERIFICADO.json` conserva el número por intento, las capacidades, los límites efectivos y las señales.

## Atribución de la señal

En el intento 04, `SENALES_04.17332` registra:

```text
1790161533.560348 --- SIGTERM {si_signo=SIGTERM, si_code=SI_USER, si_pid=0, si_uid=61876} ---
```

La llamada propia aparece después, en `SENALES_04.17324`:

```text
1790161533.909567 kill(-17332, SIGTERM) = 0
```

La separación observada es de 349,219 ms. La señal recibida inicialmente no corresponde a esa llamada del controlador. En 06 se repite `SI_USER`, PID 0 y el mismo UID exterior. La interpretación de un emisor no visible desde el espacio de PID se apoya en la [documentación Linux de espacios de nombres](https://www.man7.org/linux/man-pages/man7/pid_namespaces.7.html). **No se identifica un servicio concreto ni se atribuye su decisión a falta de memoria.** Los contadores OOM del grupo visible permanecieron a cero; esa lectura no cubre todos los ancestros.

En 07–08 la traza identifica el PID del controlador como emisor de SIGTERM y el registro previo declara `limite_rss_muestreada`. Son paradas instrumentales distintas. Los umbrales fueron 14 125 498 368 B y 14 104 965 120 B, respectivamente. Las señales posteriores de escalada también se conservan aunque el estado final del hijo corresponda a SIGTERM.

## Continuación abierta

El objetivo de obtener una respuesta permanece pendiente. La siguiente intervención debe centrarse en la memoria transitoria y la retención de tensores del cargador CPU, o en una representación compatible que reduzca ese consumo. La evidencia actual no justifica repetir las mismas cargas, retirar guardas, ampliar recursos sin límite ni presentar gpt-oss como operativo. Una alternativa deberá conservar Harmony, la identidad del modelo y las diferencias de precisión de la configuración efectivamente ejecutada.

Qwen/B conserva su [cierre parcial delimitado](../../../../../resultados/cierre-qwen-b-20260923/INFORME.md). Esta continuación no modifica su instalación ni reabre sus campañas. El seguimiento de gpt-oss queda en [TT-0012](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0012.md), vinculado a S39.

## Paquete verificable

- [Evidencias originales](EVIDENCIAS.tar.gz): 152 127 bytes; SHA-256 `8487aae312b700db23a31f563359cc616bbc0a1be612fcbb4d2e33d388efd070`.
- El manifiesto interior coteja **150 archivos**: registros, trazas, compilaciones y fuentes de las variantes. El archivo del intento de adjunción fallido se conserva como tal; no se presenta como una traza útil.
- [Resumen derivado y cotejado](RESUMEN_VERIFICADO.json) y [lector Rust](analizar-evidencias.rs). Los registros originales prevalecen sobre el resumen.
- [Confirmación de parada](PARADA.jpg). La captura muestra el estado detenido de la instancia; no acredita por sí sola la parada de cada proceso. Esta última se documenta en los resultados y en `PROCESOS_FINALES.txt`, sin procesos enumerados, a las 11:17:57 UTC; el puerto previsto tampoco aparece en la lectura final.
- [Controlador de referencia 0.1.3](../../controlador-nativo/README.md) y su [verificación local](../../controlador-nativo/verificacion-0.1.3/INFORME.md).

Fuentes técnicas consultadas en la revisión exacta del motor: [topología selectiva](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/docs/src/content/docs/guides/perf/topology.mdx), [cuantización durante la carga](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/docs/src/content/docs/guides/quantization/quantize-a-model.mdx), [ejecutor ISQ y opción de un trabajador](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-quant/src/isq_executor.rs) y [operaciones CPU de expertos MXFP4](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-quant/src/mxfp4/mod.rs).

---

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es) · [Aviso del SV y licencias de terceros](../../controlador-nativo/AVISO_LICENCIAS.json). El paquete no contiene pesos ni ejecutables del motor.
