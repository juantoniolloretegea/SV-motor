# Código y reproducción

El cliente común, su adaptación Kimi y los comprobadores Rust se publican en sus sedes compartidas, enlazadas en INVENTARIO-RUST.json. IDENTIDAD-CODIGO.json distingue los bytes ejecutados de la sustitución de raíz local para publicación. Las dependencias se conservan en Cargo.lock; la instrumentación y MCP mantienen sus sedes previas. No se distribuyen claves ni ejecutables con acceso configurado.

El visor conserva Cargo.toml, Cargo.lock y fuentes bajo visor-egui/. Su representación procede de CAPA-R2 y se comprueba contra su huella. Rust se compila a WebAssembly y egui dibuja el polígono. El HTML sólo incorpora el arranque mínimo del navegador, sin autoridad sobre la adjudicación. La recepción usa sv-adjudicar-cyb16-api, que admite también este contrato MD09, con clave exterior jamás suministrada al candidato.

Se conservan la excepción criptográfica C/ensamblador y la recepción independiente pendiente; no se afirma que todas las dependencias sean Rust puro.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
