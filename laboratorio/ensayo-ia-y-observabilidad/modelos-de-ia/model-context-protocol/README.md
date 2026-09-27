# Servicio documental MCP

**Índice documental · revisión 2 · 27 de septiembre de 2026.**

El prototipo Rust proporciona `buscar_documentos` y `leer_documento` sobre un catálogo local de documentos conservados. La adquisición documental es administrativa y separada de la inferencia.

## Secuencia de versiones

| Versión | Aportación documentada | Alcance de recepción |
|---|---|---|
| [0.1.0](0.1.0/README.md) | Primer servicio, catálogo y cliente de comprobación. | Antecedente con preparación incompleta; se conserva íntegro. |
| [0.1.1](0.1.1/LEAME.md) | Validación de claves JSON, paginación completa, conservación y controles temporales dirigidos. | Comprobaciones documentadas; recepción de TT-0014 pendiente. |
| [0.1.2](0.1.2/LEAME.md) | Servicio conservado y cliente mínimo con supervisión y conservación adicionales. | Alcance propio de las comprobaciones descritas; no constituye recepción general del componente. |

Las tres carpetas contienen la versión concordante en Cargo.toml y su Cargo.lock. La versión numérica no acredita por sí sola el conjunto de garantías. La referencia más reciente es 0.1.2; no se designa una versión aceptada para producción.

**Distinción de componentes:** el servicio MCP y los clientes de prueba tienen funciones diferentes. El cliente mínimo añadido en 0.1.2 no proporciona herramientas al modelo. Su existencia no demuestra búsqueda autónoma ni integración de GPT-OSS-120B.

## Identidad y auditoría

Cada carpeta conserva sus fuentes, documentación y manifiestos. Las fuentes y activos anteriores permanecen sin sobrescritura; la identidad reproducible se fija por versión, ruta y commit, según el [registro del ensayo](../../VERSIONES.json). Estas versiones no tienen release específico en el corte consultado.

La [revisión de 0.1.0](revisiones/0.1.0-revision-02/ESTADO_TECNICO.md) conserva los reparos de aquel corte. No describe automáticamente el estado de las versiones posteriores. Los controles de 0.1.1 y 0.1.2 deben leerse con sus límites y realizaciones correspondientes.

La [ficha técnica](FICHA_TECNICA.md) y [TT-0014](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0014.md), vinculado a S39, conservan el contrato y la recepción del componente. Este índice no modifica el tique ni concede aptitud a ningún modelo.

El MCP restringe el acceso documental; no garantiza la verdad de una respuesta ni constituye por sí solo el Árbitro SV. [Volver al ensayo](../../README.md).
