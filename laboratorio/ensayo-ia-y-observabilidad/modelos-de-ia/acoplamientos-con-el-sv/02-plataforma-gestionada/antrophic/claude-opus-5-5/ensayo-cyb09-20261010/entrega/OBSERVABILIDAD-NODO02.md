# Datos operativos y límites del nodo 02

La petición del 09/10/2026, 08:59:26.924 UTC, se incorporó a las instrucciones efectivamente recibidas en las 27 entregas conservadas, incluidas las cuatro históricas. La identidad de cada solicitud y respuesta se vincula mediante SHA-256 en la telemetría. El [registro JSON](OBSERVABILIDAD-NODO02.json) conserva por entrega las tres procedencias siguientes, sin convertir declaraciones en observaciones.

1. **Declaración del candidato:** informe_operativo íntegro, procedimiento público resumido, herramientas declaradas, magnitudes accesibles y límites. Los valores null expresan ausencia declarada; no cero. Los fundamentos verificables de la respuesta y todos los canales emitidos se conservan en los originales.
2. **Metadatos del servicio:** identidad comunicada, tokens de entrada y salida, razón incluida en salida, latencia backend y coste comunicados por usage. Estos contadores exteriores pueden estar disponibles aunque el modelo declare que no los conoce. No prueban una revisión inmutable, la facturación liquidada ni la arquitectura interna.
3. **Observación exterior Rust:** PID, reloj monotónico, duración, recepción de cabeceras y primer byte del cuerpo, CPU y memoria del controlador, hilos, descriptores, espacio de red y conexiones filtradas por su PID; muestras e integridad encadenada. Son recursos del proceso que transporta la solicitud en Kaggle. Los máximos y diferencias se calculan en Rust entre muestras, con sus límites; no son recursos de Claude ni tráfico TCP total.

El suministro fue documental, con cinco fuentes y 28 secciones previamente recuperadas mediante MCP/mdBook. El candidato no realizó llamadas autónomas al MCP ni modificó el SV. Núcleo, semántica e IR permanecieron intactos.

El servicio comunicó tokens de razón incluidos en salida, pero no devolvió el texto de ese canal: reasoning y reasoning_content están ausentes en message y se conservaron como null; no se recibió contenido delimitado think. La respuesta pública aporta fundamentos verificables e informe operativo íntegros. Los contadores no permiten reconstruir ni declarar observada la cadena interna.

El primer token y los recursos internos del proveedor permanecen null con motivo explícito. JSON completo sin flujo permite medir el primer byte del cuerpo recibido, no el instante de producción del primer token. No se solicitan ni se atribuyen procesos internos privados no accesibles. El uso comunicado de razón no se añade de nuevo a los tokens de salida.

La entrega permite gestionar la identidad documental, procedencia, consumo comunicado, recepción y límites del acoplamiento del nodo 02. La recepción documental automática es distinta del juicio asistido de contenido y de la **recepción científica independiente pendiente**. No autoriza ejecución sobre activos, ampliaciones o nuevas inferencias.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).