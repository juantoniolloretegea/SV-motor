# Selección de bibliotecas Rust para lectura documental de PDF

Corte de consulta: 3 de octubre de 2026. Datos obtenidos de las API de GitHub y crates.io de los proyectos originales; se conservan revisiones y huellas en `evidencias/MANTENIMIENTO.json` y `VERSIONES.json`. Las descargas son acumuladas de la biblioteca, no sólo de la versión. Los recuentos de contribuyentes de 100 son un mínimo porque la consulta se limitó a la primera página.

| Proyecto | Versión publicada y fecha | Actividad más reciente observada | Estrellas / contribuyentes observados | Decisión |
|---|---|---|---|---|
| [lopdf](https://github.com/J-F-Liu/lopdf) | 0.45.0, 08/09/2026 | 02/10/2026 | 2262 / ≥100 | Analizador PDF de base, Rust, MIT; versión exacta fijada |
| [pdf-extract](https://github.com/jrmuizel/pdf-extract) | 0.12.1, 16/09/2026 | 16/09/2026 | 600 / 21 | Extracción textual; variante local delimitada y comprobada |
| [hayro](https://github.com/LaurenzV/hayro) | 0.7.1, 05/06/2026 | 03/10/2026 | 779 / 29 | Proyecto activo, pero su descripción advierte carácter experimental; no seleccionado para esta etapa |
| [pdfium-render](https://github.com/ajrcarey/pdfium-render) | 0.9.4, 06/09/2026 | 16/08/2026 | 713 / 41 | Envoltura Rust de PDFium, cuyo motor es C++; no satisface un recorrido íntegro en Rust |
| [SDK oficial MCP para Rust](https://github.com/modelcontextprotocol/rust-sdk) | rmcp 3.5.0, 28/09/2026 | 02/10/2026 | 3975 / ≥100 | Referencia mantenida para MCP; no se sustituye el servicio existente por este SDK en la preparación |

La actividad y la comunidad son indicios de mantenimiento, no garantías de exactitud o seguridad. El contraste con el PDF real es decisivo: ni la extracción directa inicial de lopdf ni la versión original de pdf-extract satisfacen por sí solas el cotejo. La variante local corrige el defecto observado de recursos de fuente; la cobertura de caracteres y el transporte se comprueban por separado.

Se ha consultado [RUSTSEC-2026-0187](https://rustsec.org/advisories/RUSTSEC-2026-0187): afecta a lopdf hasta 0.41 y declara corrección desde 0.42. La versión fijada 0.45.0 queda fuera de ese intervalo. Esta comprobación concreta no constituye una auditoría exhaustiva de todas las dependencias.

La integración sigue la descripción oficial de [servidores MCP de OpenAI](https://developers.openai.com/plugins/concepts/mcp-server), su [guía de conectores y MCP](https://developers.openai.com/api/docs/guides/tools-connectors-mcp) y la [especificación de herramientas MCP 2025-06-18](https://modelcontextprotocol.io/specification/2025-06-18/server/tools). Se preservan los nombres, esquemas, resultado textual y resultado estructurado del servicio existente. No se requiere contratar API ni realizar llamadas de inferencia para preparar y comprobar esta capacidad.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
