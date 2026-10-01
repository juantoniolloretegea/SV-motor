# Ficha técnica del servicio documental MCP local

**Actualización:** 01/10/2026. **Nueva versión verificada localmente:** [0.1.3](0.1.3/LEAME.md). **Seguimiento:** TT-0014 / S39.

La nueva revisión retira el límite de ocho palabras no anunciado, permite recorrer todas las coincidencias y añade registro duradero y reconstrucción determinista de solicitudes y respuestas. Las [pruebas](0.1.3/PRUEBAS.txt) y el [cotejo conservado](0.1.3/evidencias/COTEJO.json) documentan el alcance. La integración completa con Safeguard y la cobertura del motor no han sido ensayadas: no se declara aptitud integral. Un tramo relevante sin observación o reconstrucción determina No apto para el uso exigido por el SV.

Las realizaciones anteriores conservan sus resultados y limitaciones. No se sustituye el componente de campañas Qwen en curso. A continuación se conserva el corte documental anterior.

## Antecedente · 27/09/2026

**Revisión documental:** 2 · 27 de septiembre de 2026.  
**Componente de aquel corte:** [MCP documental local 0.1.1](0.1.1/FICHA_TECNICA.md).  
**Trazabilidad:** [TT-0014](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/6bf677dca4808621738012f796eda009edeb6194/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0014.md), S39 revisión 23. Recepción pendiente; no se declara cerrado el tique.

La versión 0.1.1 corrige la validación de claves JSON repetidas, la paginación sobre la respuesta serializada y la custodia supervisada. Conserva Rust 1.98.0, Cargo.lock, interfaz MCP stdio y exactamente dos herramientas: buscar_documentos y leer_documento. El [LEAME](0.1.1/LEAME.md) describe componentes y límites.

**Resultado técnico:** preparación técnica conforme en las comprobaciones dirigidas MCP-L-01 a MCP-L-08, con observabilidad de recursos parcial declarada. Se conservaron las cinco secciones del PDQ profesional español y se reconstruyeron sus 25 páginas sin diferencias. El catálogo es una emulación documental experimental, no el dominio inmunológico ni una realización completa de OP-IMM-001.

**Resultado del candidato:** MCP-L-09 no comprobable en su totalidad por vencimiento de 600 segundos sin respuesta final. Qwen solicitó ambas herramientas y recibió una página original. Los resultados de herramientas coinciden con las peticiones posteriores conservadas. Esto acredita acceso documental; no acredita interpretación clínica ni modifica los resultados médicos anteriores.

La realización no utilizó Internet durante la consulta, búsqueda web incorporada ni modelos auxiliares. La comunicación cliente-motor quedó limitada a loopback; el proceso MCP tuvo denegada la creación de sockets. El aislamiento incluye permisos y montajes del sistema operativo, no solamente validación de argumentos.

La [ficha inicial](https://github.com/juantoniolloretegea/SV-motor/blob/cb395dfe39f7827e7b1440d534ed65989a2deac6/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/model-context-protocol/FICHA_TECNICA.md), MCP 0.1.0 y sus revisiones permanecen como antecedentes. La entrega operativa completa se conserva en el expediente restringido MCP-DOCUMENTAL-PREPARACION-20260926, entrega-05.

El resultado pertenece al Ensayo de inteligencia artificial y observabilidad. No acredita aptitud clínica, cumplimiento regulatorio, admisión de conocimiento, integración con el núcleo ni cierre de S39. No autoriza otra campaña.
