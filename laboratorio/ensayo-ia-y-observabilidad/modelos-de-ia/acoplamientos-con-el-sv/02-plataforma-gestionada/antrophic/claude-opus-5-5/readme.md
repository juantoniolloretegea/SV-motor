# Claude Opus 5.5 · Plataforma gestionada · Nodo 02

Edición documental 1.6 · 10/10/2026. **Estado vigente:** manual MD09 cerrado, resultado asistido 9/9; CYB09 terminado, R2 88,89/100 y No apto asistido por C02 crítico. Custodia cotejada y sesión apagada; recepción científica independiente pendiente. Véanse la [matriz de recepción](ensayo-cyb09-20261010/entrega/MATRIZ-RECEPCION-NODO02.md) y los límites de cada campaña.

## Antecedente fechado del manual · edición 1.5

Edición documental 1.5 · 09/10/2026. **Nueve preguntas del manual completadas en R0/R1/R2; resultado asistido final 9/9, seis críticos correctos. Recepción independiente pendiente. Ejecución detenida tras el manual por instrucción humana.**

## Modelo y despliegue

Claude Opus 5.5 es un modelo de Anthropic presentado el 22/09/2026. Sus capacidades anunciadas son declaraciones del fabricante: [anuncio oficial](https://www.anthropic.com/claude-opus-5-5), [fichas del fabricante](https://www.anthropic.com/system-cards). El identificador recibido en las respuestas fue `anthropic/claude-opus-5-5@default`; default no fija una revisión inmutable de pesos. Parámetros, precisión, región y hardware interno no se han acreditado.

Se sirve por Kaggle Benchmarks/Model Proxy: los pesos permanecen remotos. El cuaderno Linux ejecuta el transporte, composición, controles y observación Rust del SV, sin acelerador asignado a ese soporte. Las credenciales temporales de Kaggle se utilizan sólo en el entorno autorizado, sin publicación. Véase la [biblioteca oficial de Kaggle](https://github.com/Kaggle/kaggle-benchmarks/blob/01183088103937114b723b82b8d5ea0c7a87ccce/quick_start.md).

## Condiciones de realización

Fuentes exclusivamente autorizadas, suministradas mediante MCP mdBook Rust y preincorporadas íntegramente a la petición. Herramientas y navegación del candidato deshabilitadas; la conexión de Internet del cuaderno sirve para transporte, no como fuente documental. Gemini excluido. Núcleo, semántica e IR preservados. Banco, clave y criticidad fijados antes de inferir; los criterios reservados no entran en el contexto del candidato. R0 provisional, R1 autocrítica y R2 verificación final neutral. No se elige la mejor etapa.

Control Rust, dependencias fijadas, `max_tokens=16384`, `reasoning_effort=high`, respuesta completa sin flujo, 300 segundos por solicitud y cotas de banco de 90 minutos. La pausa instrumental cuenta dentro del plazo original del manual. El primer HTTP 400 y la recuperación de MD01-R0 por refusal vacío se conservan sin atribuirlos a error científico del candidato. No hubo repetición de R0 ni reintentos automáticos.

## Puesta en servicio y reproducción documental

[Código, binario y comprobaciones r2](instrumentacion-rust/0.1.0-r2/). Rust 1.98.1, glibc 2.34 o posterior, binario x86_64 identificado por SHA-256. Su admisión real Linux y telemetría previa se comprobaron sin inferencia. El auxiliar del cuaderno sólo descarga, arranca y conserva; los controles del recorrido pertenecen a Rust. Una biblioteca criptográfica C/ensamblador permanece como excepción documentada; no se declara una dependencia íntegramente Rust.

El procedimiento histórico comprobó primero 75 composiciones con `--comprobar` y después continuó desde el original fijado mediante `--recuperar-md01-r0`. Este ensayo está cerrado en el corte conservado: la reproducción autorizada ahora es documental, reconstruyendo el archivo y ejecutando sus cotejos locales. Iniciar otra generación requiere una nueva decisión competente y no forma parte de este cierre.

## Resultado y límites

[Informe del manual, terna, puntuación, originales y evidencias](ensayo-md09-cyb16-20261009/corte-01/INFORME-MANUAL.md). 27 respuestas del manual; las tres etapas dan 9/9 en los extremos solicitados, con advertencias documentadas. Coste comunicado del manual: 7,425529 USD. C01 y R0 de C02 se conservan como corte adicional anterior a la precisión de parada, sin dictamen global de ciberseguridad. Total conservado: 31 respuestas y 8,698073 USD; HTTP 400 inicial de coste desconocido, con reserva retenida de 0,606024 USD. El controlador terminó antes de C02-R1; no hay nuevas inferencias.

Se conservan respuestas, explicaciones públicas, informe operativo, canales recibidos, contadores y telemetría íntegros. No se recibió texto del razonamiento interno; se comunicaron sus tokens incluidos en salida. No se inventan caché, primer token ni recursos internos del proveedor. Resultado asistido favorable del manual, antecedentes históricos y recepción independiente pendiente permanecen separados. No acredita aptitud clínica ni uso autónomo del MCP.

GitHub es la sede documental central. La custodia se declara tras recuperación de la revisión publicada, cotejo SHA-256 en Rust y reconstrucción del archivo. Las fuentes de terceros mantienen sus derechos; los originales históricos no se reescriben.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

## Complemento gráfico · 09/10/2026

Se completa la omisión del polígono en la entrega inicial: [entrega Rust/egui](ensayo-md09-cyb16-20261009/corte-01/poligono-egui/ENTREGA-POLIGONO.md) y [HTML autónomo](ensayo-md09-cyb16-20261009/corte-01/poligono-egui/POLIGONO-EGUI.html), con las nueve posiciones R2, terna 9/0/0, seis críticos correctos, citas, advertencias y comparación de etapas. Dictamen asistido y recepción independiente pendiente permanecen separados. Ninguna nueva generación ni cambio de resultado; la campaña sigue detenida. Antecedente íntegro bajo la revisión c57f2a191a02571cc13b8319ccec60cf630732fe.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).


## Custodia y referencias competentes · 10/10/2026

[Control de conservación](ensayo-cyb09-20261010/entrega/CONTROL-CUSTODIA.json), [puntuación cotejada en Rust](ensayo-cyb09-20261010/entrega/PUNTUACION-RUST.json), [tique TT-0026](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/a201e592a304abd982e070399038d861981aa036/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0026.md) y [Acta 005](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/a201e592a304abd982e070399038d861981aa036/docs/calidad/tuberias-ia/continuacion-15-09-2026/ACTA_005_CLAUDE_OPUS_5_5_NODO02_2026_10_09.md). El informe administrativo específico es SV-GASTO-20261010-010 en su sede privada; una reserva o un contador de uso no acreditan liquidación. El manual y sus originales permanecen íntegros en su corte histórico.

## Contraste CYB09 terminado · 10/10/2026

El nodo 02 conserva su primer modelo, anthropic/claude-opus-5-5@default, mediante Kaggle Benchmarks Model Proxy. El [contraste abreviado](ensayo-cyb09-20261010/entrega/INFORME.md) contiene nueve preguntas críticas y 27 entregas: cuatro originales recuperados y 23 generaciones nuevas, sin repetir el manual. Dictamen asistido R2: 8/9, no admisible según la clave por el requisito de conservación de C02; R1 9/9 se conserva como etapa histórica y no sustituye R2. Recepción científica independiente pendiente.

El despliegue es gestionado por el proveedor a través de Kaggle; no se instalan pesos de Claude en el nodo. Composición, adaptación, observación, continuidad y cotejo se realizan en Rust. El auxiliar del cuaderno sólo descarga, inicia y conserva el ejecutable. En la frontera efectivamente usada: suministro íntegro del corpus mdBook fijado, JSON completo sin flujo, sin herramientas autónomas del candidato, reasoning_effort high, salida máxima de 16.384 tokens y límite de transporte de 300 segundos. No se acredita revisión inmutable de pesos, hardware interno del proveedor ni primer token.

Se conserva la [petición operativa del nodo 02](ensayo-cyb09-20261010/entrega/OBSERVABILIDAD-NODO02.md), con declaraciones del candidato, contadores del proveedor y observación exterior separados. El servicio contabiliza razón pero no devuelve su texto en un canal separado; se conservan respuesta, fundamentos públicos, informe operativo y originales, sin inventar pensamiento interno. El coste nuevo comunicado fue 8,21776 USD, distinto de liquidación. La sesión quedó apagada y no se amplía el examen.

[Polígono interactivo Rust/egui](ensayo-cyb09-20261010/entrega/poligono-egui/POLIGONO-EGUI.html) · [Originales y manifiesto](ensayo-cyb09-20261010/entrega/MANIFIESTO-ORIGINALES.json). El HTML autónomo se descarga y abre en un navegador compatible con WebAssembly y WebGL; GitHub muestra su fuente. Los resultados históricos y sus límites permanecen íntegros.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).