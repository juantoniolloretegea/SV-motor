# Contraste con la revisión fijada

Motor de referencia: mistral.rs 0.9.4, commit `2370966bb91e2e3dafa0b1521b87c50fd5c01244`. La inspección fue documental; no se compiló ni inició el motor.

1. `mistralrs-mcp/src/transport.rs` implementa `ProcessTransport`, mensajes JSON-RPC delimitados por nueva línea y la notificación `notifications/initialized`. Es el transporte de este prototipo.
2. `mistralrs-mcp/src/client.rs` solicita `initialize`, luego lista herramientas mediante `tools/list` y llama mediante `tools/call`. Lee contenido textual de la respuesta. Por ello los datos y su procedencia se serializan dentro de `content[type=text].text`.
3. `mistralrs-mcp/src/types.rs` define `McpToolResult.is_error` sin renombrado serde. El protocolo utiliza `isError`. El prototipo emite ambos campos con idéntico valor; `is_error` es una extensión explícita para esta revisión. No se modifica el motor. Una prueba comprueba ambos indicadores y la deserialización con esa forma de estructura. No se presenta como prueba ejecutada contra el cliente real completo.
4. `mistralrs-mcp/src/lib.rs` confirma el formato `source.type=Process`, `command`, `args`, `work_dir`, `env`, prefijo de herramientas, plazo y límite de concurrencia.
5. El servidor negocia protocolo `2025-06-18`. Si un cliente solicita otra versión, responde con la soportada; el cliente debe decidir si la admite. Debe comprobarse esa negociación en la recepción del motor. No se afirma compatibilidad universal con versiones posteriores.
6. El transporte de esta revisión extrae el resultado de la siguiente línea y no acredita correlación del identificador al recibirlo. Mantener concurrencia uno. Si vence el plazo, cerrar la sesión: una respuesta tardía no debe confundirse con otra petición.

Referencias exactas:

- https://github.com/EricLBuehler/mistral.rs/blob/2370966bb91e2e3dafa0b1521b87c50fd5c01244/mistralrs-mcp/src/transport.rs
- https://github.com/EricLBuehler/mistral.rs/blob/2370966bb91e2e3dafa0b1521b87c50fd5c01244/mistralrs-mcp/src/client.rs
- https://github.com/EricLBuehler/mistral.rs/blob/2370966bb91e2e3dafa0b1521b87c50fd5c01244/mistralrs-mcp/src/types.rs
- https://github.com/EricLBuehler/mistral.rs/blob/2370966bb91e2e3dafa0b1521b87c50fd5c01244/mistralrs-mcp/src/lib.rs
- https://modelcontextprotocol.io/specification/2025-06-18/server/tools

La documentación actual https://docs.mistralrs.dev/guides/agents/connect-mcp-server/ advierte de que el servidor HTTP puede continuar con MCP desactivado tras un fallo de inicialización. No basta HTTP 200. La futura campaña deberá registrar herramientas disponibles, llamada efectiva, resultado e incorporación a la conversación, y detenerse ante un fallo.

La existencia del módulo fuente no demuestra que el ejecutable instalado exponga las opciones necesarias. La recepción debe verificarlo por inspección de ayuda y configuración, sin cargar pesos ni actualizar el motor.
