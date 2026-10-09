# Consulta documental de tres formatos

La instrucción vigente delimita la actuación a la caché HTML, el PDF y el documento Markdown ya conservados. La documentación de los universos de inmunología y ciberseguridad se recibió como información para una incorporación futura; no forma parte de esta prueba. No se modifica el README principal.

Se reutiliza el acceso de Astra por el nodo 3 y el cliente común Rust. El modelo solicita identificadores de documentos y secciones; el controlador decide su admisión y el MCP devuelve el texto íntegro de cada sección. Los enlaces son referencias documentales. La solicitud al proveedor lleva `tools: []`, `tool_choice: none` y `store: false`. El proceso documental Linux impide conexiones de red. La observación local no inspecciona los procesos internos del proveedor.

## Fuentes fijadas

| Documento | Original conservado | Alcance |
|---|---|---|
| HTML01 | Caché NCI-PDQ profesional, SHA-256 `00018ac31108eecc4709f0ce80439d4ca2d56b02262c5c334a184b9c0944e04d` | Es la caché histórica realmente utilizada; no se sustituye por una descarga actual de la versión para pacientes. |
| PDF01 | LLS, *Leucemia de células peludas*, FS16S8/18, diez páginas, SHA-256 `21322ed388c999bd0713ba5ec5c7ab34402d4bb4cd6100903aa5ea6d8bf35d9c` | Texto de una edición histórica; no acredita recomendaciones médicas actuales ni interpretación de figuras. |
| MD01 | `ejecucion/zai-cyb25-preparacion-20261008/fuentes/BANCO-CYB25.md` | Enunciados de veinticinco casos sintéticos; no contiene la clave de corrección. Se consulta como documento, sin realizar el examen. |

## Incidencias previas al envío

La conversión Rust Xberg 1.3.6 concluyó en 1361,582 ms y conservó su instrumentación. La revisión encontró un desplazamiento de «tricoleucemia» desde el primer punto clave hacia la introducción. Se rechaza esa conversión PDF para el suministro. Se reutiliza la extracción Rust histórica `EXTRACCION-FINAL.json`, ligada a la misma huella PDF, cotejando cada página y conservando sus textos literalmente bajo encabezados de página física.

La primera preparación del libro se detuvo porque el Markdown convertido del HTML incluía referencias de imagen. El control existente las rechaza; no se eliminó esa protección. La revisión siguiente transforma expresamente la notación de imagen en una referencia textual con el aviso «Recurso visual no interpretado», conservando texto alternativo y destino como datos. No se descarga ni se interpreta la imagen.

Las conversiones reales requieren ahora un recibo exterior cuya huella queda incluida en la política autorizada. Se vinculan original, Markdown y evidencia del cotejo; una declaración del candidato no puede sustituirlos. La recepción queda limitada al ensayo, con producción excluida.

## Comprobación y límites

La regresión Rust pasó veinte comprobaciones de biblioteca, incluidas las de recibos de conversión, y cincuenta y nueve del cliente y de su acceso. La recepción íntegra del proceso MCP, la navegación real de Astra y su respuesta final requieren evidencias posteriores; este documento no las da por ejecutadas.

Consulta prevista: localizar el linaje celular en HTML, los nombres de la enfermedad en el PDF y el caso sobre versión persistida y ejecución activa en Markdown. Máximo de doce turnos documentales y 2048 tokens de salida por turno. La cota controla este ensayo y no representa un examen de capacidad. Se conservan las entregas originales y el uso comunicado por el proveedor; un dato ausente o un coste pendiente no se convierte en cero. No se realizan compras ni recargas.

La corrección formal de las citas se distingue del cotejo semántico exterior. Este ensayo no declara recepción integral de producción ni recalifica los exámenes anteriores.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
