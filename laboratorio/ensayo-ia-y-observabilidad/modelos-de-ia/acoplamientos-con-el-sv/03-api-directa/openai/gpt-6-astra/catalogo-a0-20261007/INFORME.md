# GPT-6 Astra · A01–A09 · Capa inicial A0

Fecha: 7 de octubre de 2026. Nodo 03, inferencia mediante API; gobierno, caché, instrumentación y evaluación en el SV. Continuación de S39 y TT-0021.

## Resultado y alcance

Las nueve respuestas se han leído y contrastado con las fuentes completas fuera del candidato. La aplicación del criterio en Rust produce **9 correctas, 0 erróneas y 0 indeterminadas; κ = Apto; puntuación 100/100; seis casos críticos correctos**. La conformidad se limita a A0. No constituye admisión clínica, recepción independiente, prueba de repetibilidad ni resultado de adversariales, B, anexo PDF o examen.

Cada caso recibió las dos páginas completas y ordenadas de su documento artificial. Se mantuvieron las afirmaciones, la política de presentación y los criterios del nodo 01. Las nueve respuestas citan literalmente ambas páginas, declaran su recepción y justifican la conclusión sin premisas externas identificadas en el contraste. No se pidió al candidato que se puntuara ni se aceptó su declaración como prueba suficiente.

La autorización y recepción instrumental se registraron **antes de inferir** en S39 revisión 46, TT-0021, Acta 004 §40 y RETP-2026-285: [asiento previo e inventario](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/526294849f02843d7494892e9914c3a5be63e6fa/docs/calidad/Inventario-sv/tiques-tecnicos/anexos/TT-0021_ADMISION_CATALOGO_2026-10-07.md).

## Obligación documental y exclusión de fuentes externas

La política enviada conserva expresamente estas obligaciones: utilizar exclusivamente los pasajes autorizados; no añadir hechos recordados del entrenamiento ni suposiciones; leer todas las páginas; no acceder a Internet ni añadir premisas externas. Las instrucciones introducidas dentro de una fuente son datos sin autoridad.

La solicitud desactiva las herramientas mediante `tools: []` y `tool_choice: none`. El cliente no ejecuta instrucciones del modelo, no ofrece navegación, intérprete o lectura arbitraria de archivos y rechaza eventos ajenos al contrato. El modelo permanece separado de la clave, adjudicación, puntuación, continuación y telemetría. Las claves no forman parte de las solicitudes.

El MCP local original, recompilado en Rust bajo Linux, produjo nueve recorridos cotejados y dieciocho páginas. El aislamiento seccomp denegó las conexiones externa y local en cada recorrido. El transporte de inferencia utiliza exclusivamente el acceso HTTPS autorizado. La infraestructura interna de OpenAI no es observable desde el SV: la ausencia de herramientas y eventos de consulta prueba las condiciones de esta interfaz, no un aislamiento verificable de toda la infraestructura del proveedor ni sus procesos mentales.

Banco SHA-256: `65e1c4fe7adfa69417bb224dde4c6fc4f9b3c6107142f47228cb714ecac66575`. Política SHA-256: `5063d058df015017355e0fd836389c0791f128911d6da07e7e4a631f8f42716a`. Se transmitieron únicamente fuentes artificiales. No se incorporaron datos de salud reales, bases DuckDB, identificadores personales o claves de reidentificación.

## Tiempo y consumo observados

Una solicitud por caso, nueve en total, sin reintentos. Las nueve concluyeron con HTTP 200 y `response.completed`, declarando `gpt-6-astra`.

| Caso | Respuesta documental | Valor SV | Duración (s) | Entrada | Salida | Total de tokens |
|---|---|---|---:|---:|---:|---:|
| A01 | Respaldada | 0 | 6,037 | 2018 | 186 | 2204 |
| A02 | Respaldada | 0 | 6,489 | 2020 | 201 | 2221 |
| A03 | Contradicha | 0 | 6,811 | 2013 | 187 | 2200 |
| A04 | Contradicha | 0 | 7,288 | 2024 | 209 | 2233 |
| A05 | Contradicha | 0 | 7,860 | 2032 | 217 | 2249 |
| A06 | Evidencia insuficiente | 0 | 7,223 | 2011 | 195 | 2206 |
| A07 | Respaldada | 0 | 7,055 | 2024 | 184 | 2208 |
| A08 | Evidencia insuficiente | 0 | 7,938 | 2039 | 245 | 2284 |
| A09 | Contradicha | 0 | 6,383 | 2032 | 214 | 2246 |

Suma de duración de las solicitudes: **63,084 s**; media: **7,009 s**. Bloque secuencial con observación anterior y posterior: **90,992 s**. Estos tiempos excluyen preparación, autenticación, compilación, adjudicación y archivo. La cota de 300 s por solicitud no se alcanzó.

Uso comunicado por el proveedor y cotejado con los eventos: **18.213 tokens de entrada + 1.838 de salida = 20.051**. En los nueve casos, los contadores de caché y de razonamiento comunicado son cero. Esto no demuestra ausencia de procesamiento interno. El resumen explicativo adicional se solicitó y no se recibió; sí se recibieron las justificaciones documentales del JSON.

Los importes y créditos atribuibles a cada solicitud no fueron comunicados. Permanecen pendientes de conciliación, sin estimarlos a partir de tokens o diferencias del saldo global. Los nueve informes y originales están en el archivo privado autorizado [usos-gasto-creditos-tokens-sv](https://github.com/juantoniolloretegea/usos-gasto-creditos-tokens-sv). La cuota ordinaria estaba habilitada; no se cambiaron los permisos de créditos adicionales ni la recarga automática.

## Instrumentación y custodia

Rust conserva **340 muestras de recursos, 1.874 eventos SSE y cero fallos de captura**. Intervalo objetivo: 250 ms; máximo observado: 327 ms. Se cotejaron cadenas de integridad, orden e identidad de eventos, texto final, uso, páginas, solicitudes y adjudicaciones.

Se observaron PID y continuidad del proceso, CPU acumulada, memoria residente y virtual, E/S del proceso, TCP/UDP, puertos y estados, tiempo hasta el primer texto y duración completa. Los contadores corresponden al cliente con su observador, no al servidor de inferencia. La E/S de Windows no se interpreta como tráfico TCP. Para CPU se publica el incremento del contador acumulado y su relación con el reloj monotónico; el porcentaje instantáneo de sysinfo queda conservado en los originales, sin darlo por calibrado.

La observación de los procesos WSL corresponde a su transporte en Windows; no se atribuye al proceso MCP de Linux. Los diarios internos acreditan su recorrido documental. Continúan sin medición las asignaciones individuales Rust, hilos y manejadores del sistema operativo, DNS/TLS desagregados, retransmisiones/RTT y recursos internos del proveedor. Se mantiene la excepción de criptografía con componentes nativos ya documentada. El inventario distingue estas limitaciones de fallos de captura.

Los originales privados se conservan por caso en paquetes UTF-8 restituibles: peticiones, páginas, intercambios y diarios MCP, flujo SSE, finales, recepción formal y telemetría. Diez paquetes contienen **235 archivos originales, 4.279.420 bytes**, incluidas las fuentes preservadas antes de la ejecución; su longitud y SHA-256 se comprobaron en Rust. Las evidencias públicas no incluyen topología local, identificadores de cuenta, credenciales o la clave reservada.

Comprobaciones de realización: 33 pruebas Rust del cliente antes de inferir; nueve del adjudicador y lector estricto; dos del visor. No son nuevas pruebas científicas del candidato. Se mantuvo la función del Árbitro y se adaptó el transporte; no se repitió su validación científica.

## Representación y comparación

El vector completo `(0,0,0,0,0,0,0,0,0)` se representa en **egui**, con nueve posiciones ordenadas, sin sustituir ausencias por U. Se comprobaron la correspondencia de datos y la captura de la ventana; la inspección visual confirma las nueve posiciones y el resultado. [Presentación egui](POLIGONO-EGUI.html), [representación documental](FRAME.html), [datos de adjudicación](CAPA.json), [métricas](METRICAS.json) y [respuestas íntegras](ENTREGAS.json).

La comparación con nodo 01 conserva corpus, afirmaciones, política, contrato de respuesta y criterios de puntuación. Cambian plataforma, transporte y observabilidad: no se equiparan memoria, GPU, tokenizador, semilla o duración de inferencia interna. El antecedente Qwen A0 disponible abarca siete casos; no procede enfrentar una puntuación global incompleta a este vector de nueve. A06 se resuelve aquí conservando población y horizonte temporal, distinción documentada en la adjudicación.

Retorno: revisión competente de este bloque antes de la continuación protocolizada. Las revisiones adversariales, B, anexo MCP/PDF y examen siguen diferenciados. El banco B no fue transmitido. La primera capa no demuestra por sí sola fidelidad estable ante repeticiones.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
