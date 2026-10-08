# Inventario de elementos efectivamente utilizados y límites

Edición de 08/10/2026; relación con el expediente Grok 4.7, nodo 03.

| Elemento | Realización y función | Evidencia o límite |
|---|---|---|
| Árbitro/controlador documental | Rust, sv-examen-documental | Fija fuentes, verifica huellas antes de cada solicitud, conserva historia, agenda etapas y aplica límites de tiempo y presupuesto humano. No decide por sí mismo la verdad médica. |
| Suministro MCP | sv-mcp-documental y verificar-diario, Rust, Linux | Secciones completas, reconstrucción y diario cotejados; filtro de sockets del servicio. No equivale a aislar los servidores del proveedor. |
| Contrato y localizadores | Módulos Rust reutilizados de Astra | Esquema, identidad, citas e instrucciones históricas; no se envía la clave. |
| Transporte común | sv-cliente-api, Rust, reqwest/rustls | Destino HTTPS autorizado, sin redirecciones ni reintento oculto. xAI sin tool_choice y sin herramientas. |
| Instrumentación | sv-instrumentacion, Rust | Proceso propio, CPU, RSS, memoria virtual, E/S, TCP/UDP y estados; objetivo 250 ms y máximo admitido 750 ms. No incluye todos los procesos del sistema. |
| Recepción progresiva | Flujo SSE y original final | Eventos y texto concordantes, identidad del modelo, finalización, contador de herramientas/fuentes y ZDR. El razonamiento cifrado no es explicación auditable. |
| Medidas del proveedor | usage conservado | Entrada, salida, caché, razonamiento, total y coste por solicitud. context_details se conserva como magnitud diferente, sin sustituir los contadores principales. |
| Recepción posterior | sv-recepcion-api, Rust | Verifica originales y deriva mediciones sin credencial ni nuevas consultas. |
| Revisión sustantiva | Exterior al candidato, asistida por IA | Cotejo contra fuente y clave reservada. Recepción científica independiente pendiente. |
| Dictamen | sv-adjudicar-api, Rust | Identidad, forma, terna, umbral y veto crítico; tres vectores conservados. No convierte el cotejo formal en verdad clínica. |
| Presentación | Rust/egui → WebAssembly | Sólo tras adjudicar las 25 posiciones; pareja matemática/visual y avisos. JavaScript de enlace sin autoridad semántica. |

No se midieron recursos internos del proveedor, hilos ni asignaciones de memoria, DNS/TLS por separado, RTT o retransmisiones. La biblioteca criptográfica nativa conserva C y ensamblador bajo excepción; forbid(unsafe_code) limita el código propio y no certifica todo el entorno en Rust. DuckDB, JAX y Python no participaron en este recorrido del ensayo.

La universalidad del cliente consiste en compartir contrato, transporte, instrumentación y recepción, con perfiles recibidos individualmente. El primer perfil aquí probado es xAI/Grok 4.7; la lista de destinos del código no acredita la recepción de todos ellos. La parametrización portátil de las localizaciones del laboratorio sigue pendiente, expresamente documentada en el contrato común.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).