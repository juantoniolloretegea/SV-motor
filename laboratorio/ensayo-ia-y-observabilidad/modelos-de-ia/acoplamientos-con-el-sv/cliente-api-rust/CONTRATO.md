# Cliente común del SV para inferencia mediante API

Versión experimental 0.1.0, 8 de octubre de 2026. Componente transversal del nodo 03. La identidad del proveedor y del modelo se declara mediante un perfil; no se crea un cliente autónomo por candidato.

El transporte HTTPS, la conservación de originales, la recepción progresiva, el cotejo de texto, la instrumentación Rust, la prohibición de herramientas y el control de límite autorizado son comunes. El expediente de cada modelo conserva su perfil y sus resultados. El controlador documental reutiliza el suministro MCP, los localizadores y el contrato de tres etapas del ensayo de Astra; incorpora la conservación de instrucciones históricas recibida en la réplica MD01. Los originales de Astra no se modifican.

El primer perfil recibido es xAI/Grok 4.7 mediante Responses. Se configura tools=[], store=false y esfuerzo medium. Se omite tool_choice porque xAI rechazó expresamente su presencia sin herramientas en una comprobación sintética. Esa omisión no habilita una herramienta. La recepción exige un mensaje textual y rechaza resultados de herramientas. La retención cero se exige mediante cabecera del proveedor y la licencia propia se incluye en cada solicitud.

La estructura admite perfiles compatibles con Responses. La mera inclusión de otro destino en el código no acredita compatibilidad, autorización ni recepción de ese proveedor. Autenticación OAuth, otras variantes HTTP o API distintas requieren su adaptación acotada y su comprobación. No se declara universalidad de protocolos ni se sustituye la conexión histórica de Astra por una versión no ensayada.

El Árbitro conserva fuentes fijadas, contratos, identidad de cada pregunta, antecedentes verificados, agenda y reglas de suspensión. El candidato recibe solamente el encargo, las secciones completas y sus antecedentes. La clave de corrección, la instrumentación y los controles quedan fuera de su contexto. La evaluación sustantiva posterior se distingue de las comprobaciones automáticas de estructura y citas.

Los tres pasos se aplican siempre: respuesta provisional, autocrítica y verificación final neutral. Se permite mantener, corregir o declarar U justificada. Las respuestas anteriores son evidencia a contrastar, no autoridad. No se solicita razonamiento interno: se solicitan fundamentos y referencias comprobables.

La dependencia sv-instrumentacion se mantiene en su sede común. Se miden proceso propio, CPU, memoria, E/S, TCP/UDP, estados, tiempos, cabeceras y sucesos de recepción. No se presentan como mediciones de la infraestructura del proveedor. La criptografía nativa conserva la excepción experimental ya declarada; forbid(unsafe_code) se refiere al código propio.

Las fuentes conservan provisionalmente la localización del entorno experimental y el ejecutable MCP previamente recibido. Su parametrización portátil es una dependencia de generalización; no debe confundirse con la separación ya realizada entre cliente y proveedor. No se incorporan credenciales al repositorio. Los límites económicos se fijan exclusivamente por autorización humana y se archivan de forma privada.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
