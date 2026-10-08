# Inventario y comprobaciones · MD01 de Grok

Edición de diagnóstico 1.0.0, contrato MD01 1.1.0 y cliente común 0.1.0. Componentes efectivamente utilizados: sv-manual-diagnostico (control, reserva, contexto temporal y recepción); sv-mcp-documental y verificar-diario Linux (suministro y diario); localizadores Rust; sv-cliente-api (perfil xAI, HTTPS/SSE, licencia y ZDR); sv-instrumentacion (muestreo 250 ms y diario encadenado); sv-recepcion-api individual (métricas); sv-cerrar-manual-diagnostico (aplicación del juicio exterior y presentación estática). La comprobación técnica anterior se reutiliza por huella, sin otra inferencia.

Biblioteca común: 7 pruebas favorables. Diagnóstico: 14 favorables, incluidos contrato, localizadores, fuente completa, exclusión de clave, contexto temporal y presupuesto. Cierre: 1 prueba de veto por error crítico y U. Compilación sin descarga ni instalación. Originales SSE/texto, fuentes, obligaciones históricas y telemetría recibidos nuevamente en Rust después de la ejecución.

Medido: proceso propio inscrito, CPU, memoria residente y virtual, E/S del proceso, TCP/UDP filtrados por proceso, fases, solicitudes, eventos SSE, tiempo monotónico, integridad y uso del proveedor. MCP: aislamiento de sockets, proceso controlado, diario y reconstrucción documental. No medido: recursos internos de xAI, TLS/DNS por separado, asignaciones del gestor de memoria Rust, retransmisiones/RTT, hilos y descriptores del sistema. Los contadores de E/S no son tráfico TCP ni exclusivamente disco.

Sin errores de medición; máximo intervalo 367 ms, bajo el umbral instrumental de 750 ms. Esta cobertura no equivale a observabilidad universal. La insuficiencia económica detectada no queda ocultada por la recepción técnica favorable.

No se incorpora polígono ni egui para dos etapas de una sola pregunta. La presentación estática se genera en Rust y no contiene JavaScript. Los documentos técnicos y el archivo privado se cotejan por recuperación y SHA-256; los originales locales no se eliminan.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).