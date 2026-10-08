# Código y reproducción · Astra CYB16

El [cliente API común](../../../../../cliente-api-rust) mantiene el transporte HTTP, la recepción y la instrumentación compartidos. El ejecutable `sv-cyb16-chatgpt` reutiliza la sesión OAuth ya comprobada de Astra y el suministro MCP común; `src/responses.rs` recibe Responses y `src/bin/sv-recuperar-responses.rs` permite la recuperación local conservada, sin red ni nueva inferencia.

La [recepción común](../../../../../recepcion-api-rust) admite el formato de solicitud Responses y Chat Completion sin cambiar la clave del contrato. `sv-adjudicar-cyb16-api` recibe la revisión exterior identificada y aplica las reglas de criticidad y fidelidad; `sv-cotejar-citas-libro` contrasta cada cita; `sv-comparar-cyb16` recibe las dos campañas completas y calcula magnitudes descriptivas homogéneas. Ninguno de estos procesos de recepción llama al candidato.

Las pruebas del cliente y del controlador corregido suman 47 comprobaciones favorables, incluidas las modalidades válidas e inválidas de cierre SSE. El adjudicador y el cotejo de citas tienen siete comprobaciones favorables. Cargo.lock fija las dependencias; la excepción experimental de componentes criptográficos en C/ensamblador permanece pendiente.

El código del [visor](visor-egui) deriva el dictamen en Rust desde CAPA/BANCO y lo coteja antes de dibujar. Se compila con egui/eframe a WebAssembly. wasm-bindgen 0.2.129 aporta la adaptación mínima para el navegador; el empaquetador Rust incluye código y datos en HTML autónomo. CSP impide conexiones de datos. JavaScript no adjudica, modifica valores ni gobierna el SV. La recepción funcional y sus pruebas se documentan en VERIFICACION-VISOR.json.

La proyección pública sustituye la raíz privada de trabajo por C:/SV; IDENTIDAD-CODIGO.json conserva ambas huellas. Requiere recompilación y configuración propia: no es una afirmación de identidad binaria ni de portabilidad automática. El código impide reiniciar una campaña ya iniciada. Una ejecución posterior necesita su propia autorización, aplicación y perfil; no se distribuyen credenciales ni clave reservada del examen.

Las solicitudes y originales locales están identificados por SHA-256. La publicación de informes y código no acredita por sí sola custodia de todos los originales brutos, ni sustituye una recepción independiente del instrumento.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
