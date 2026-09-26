# Servicio documental MCP local 0.1.1

Versión experimental sucesora de 0.1.0, vinculada a [TT-0014](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/6bf677dca4808621738012f796eda009edeb6194/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0014.md), S39 revisión 23. La versión 0.1.0 permanece conservada. Esta entrega no cierra el tique ni acredita aptitud clínica.

## Cambios

- Validación JSON recursiva que rechaza claves repetidas antes de construir un valor.
- Paginación calculada sobre la respuesta JSON-RPC completa, incluido el escapado del texto. La concatenación de páginas conserva el texto original.
- Cliente externo con supervisor de proceso, plazo común de 30 segundos, terminación del grupo y espera; conservación sincronizada antes de devolver el resultado.
- Telemetría OpenTelemetry Rust 0.31.0 con identificadores de traza y diagnóstico de exportación.
- Servicio MCP sin sockets mediante seccomp en Linux x86_64. La inmovilidad del catálogo y el aislamiento del sistema de archivos requieren además la configuración del sistema operativo documentada en el expediente.

## Componentes y construcción

Rust 1.98.0 y dependencias fijadas por Cargo.lock. Construcción comprobada en perfil de desarrollo mediante `cargo +1.98.0 build --locked --offline -j4`, con límite externo de 8 GiB. El modo offline requiere disponer previamente de las dependencias fijadas.

| Componente | Función |
|---|---|
| sv-mcp-documental | MCP stdio: buscar_documentos y leer_documento. |
| importar-pdq | Extracción administrativa del HTML conservado, sin descarga. |
| ensayo-local | Cliente de custodia, pruebas dirigidas y consulta instrumental acotada. |
| comprobaciones-extra | Casos de JSON duplicado y vencimiento con servicio activo. |
| prueba-cliente | Motor HTTP simulado para verificar conservación y reenvío; no es un modelo. |
| observar | Muestreo de recursos del cgroup experimental. |
| inventariar | Identidad y correspondencia de secciones del catálogo. |
| empaquetar | Manifiesto y preparación de contenido versionado. |
| comprobar-mcp | Cliente técnico heredado; no gobierna la consulta de TT-0014. |

Las utilidades del ensayo identifican rutas y condiciones de la realización concreta. No constituyen un instalador general ni un servicio de producción.

## Fronteras y límites

El catálogo oficial contiene las cinco secciones completas de la captura española del PDQ profesional, con bibliografía e información editorial. No contiene rúbricas, respuestas clínicas ni material sintético. Las fuentes sintéticas se mantienen en el expediente de pruebas.

El servicio rechaza solicitudes mayores de 16 KiB. Cada respuesta MCP completa se limita a 8.000 caracteres Unicode; su tamaño en bytes se registra separadamente. La consulta utiliza solo dos herramientas, como máximo cuatro llamadas y cinco peticiones generativas, cada una con reserva de 512 tokens. La tokenización previa usa el endpoint count_tokens del mismo motor y se contabiliza separadamente.

La comunicación cliente-motor utiliza exclusivamente loopback. El proceso MCP no puede abrir sockets, ni siquiera locales. El sistema operativo monta el catálogo bajo propiedad administrativa y solo lectura, oculta el resto de /srv y mantiene los procesos sin privilegios. Estas medidas son parte de la realización ensayada; ejecutar el binario sin ellas no reproduce el aislamiento.

La conservación previa se verifica en los puntos instrumentados. Un fallo total del anfitrión o del almacenamiento puede impedir guardar incluso el error. Las huellas acreditan identidad, no validez clínica. El documento externo no tiene autoridad para cambiar permisos.

## Resultado y recepción

El resultado de la ejecución, sus límites y el estado de recepción figuran en FICHA_TECNICA.md de esta versión. Los intercambios y las evidencias operativas originales se entregan en el expediente restringido MCP-DOCUMENTAL-PREPARACION-20260926, entrega-05. La revisión y el cierre administrativo permanecen pendientes.
