# Conservación transversal de fuentes de los ensayos · V.1

Fecha: 7 de octubre de 2026. Esta incorporación conserva fuentes, manifiestos Cargo y versiones de realización de los ensayos Astra, Safeguard, Qwen y GPT-OSS, así como un cálculo documental PDQ. La sede de cada conjunto corresponde al modelo, ensayo o componente del que depende. Las versiones históricas se conservan con su identidad; no sustituyen automáticamente la realización recibida más reciente.

El [inventario transversal V.1](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/tree/main/docs/calidad/tuberias-ia/continuacion-15-09-2026/mapa/inventario-transversal-de-codigo-mantenido-versiones-y-dependencias) es el punto de consulta de sedes, versiones, dependencias y condiciones de uso. Centraliza la orientación, manteniendo la organización documental y los enlaces anteriores.

## Identidad y alcance

`FUENTES-PRESERVADAS-V1-20261007.json` enumera las rutas incorporadas y las ya idénticas en su destino, con tamaño, SHA-256 del antecedente, SHA-256 de la copia pública y adaptación aplicada. Los recuentos corresponden a archivos por versión, no a componentes originales independientes. Un manifiesto o una fuente no acredita por sí solo una compilación, una ejecución efectiva o una recepción científica favorable.

Las adaptaciones declaradas se limitan a referencias del entorno y, cuando procede, sustitución de direcciones de infraestructura por direcciones documentales. Esas copias no se presentan como registros brutos de ejecución. Las huellas distinguen expresamente original y versión pública. Las reglas de adjudicación no se modifican con esta conservación. Los originales diferentes no se declaran prescindibles por publicar una adaptación.

## Dependencias y reconstrucción

- El núcleo del Lenguaje mantiene su sede en `SV-lenguaje-de-computacion/rust/sv_core`; los destinos nativo y WebAssembly son proyectos distintos dentro de su espacio de trabajo Cargo.
- El cliente de acceso conserva su ruta histórica `gpt-6.1-sol/acceso`. Las pruebas y resultados Astra pertenecen a `gpt-6-astra`. La denominación de una carpeta no cambia el modelo efectivamente solicitado y declarado en cada prueba.
- Las fuentes del cliente se conservan con sus módulos y controladores. Algunas pruebas unitarias dependen de archivos SSE reservados de las primeras conexiones. Esos archivos no se publican con este conjunto: una reconstrucción pública completa de las pruebas requiere sustituirlos por muestras públicas equivalentes, identificadas y verificadas, sin exponer respuestas operativas reservadas.
- Las realizaciones históricas de Safeguard incluyen referencias a `mistral.rs`, al núcleo del Lenguaje, al suministro documental MCP y a entornos de ejecución específicos. Los manifiestos conservan esa dependencia; su incorporación no instala ni garantiza la disponibilidad actual de tales entornos.
- Los visores egui y sus fuentes se mantienen en las ediciones ya publicadas de cada ensayo. Los derivados JavaScript/WebAssembly y HTML autosuficientes deben vincularse a su edición y manifiesto, sin confundirlos con una nueva implementación del núcleo.
- Permanece documentada la excepción criptográfica del cliente: la incorporación de fuentes no convierte sus dependencias C/ensamblador en Rust ni acredita su admisión íntegra en el SV.

No se incluyen credenciales, claves reservadas de examen, historiales personales, saldos, expedientes económicos privados, administración ajena al ensayo ni cachés de compilación. Se mantienen las licencias de terceros. Esta incorporación no ejecuta inferencia, no adjudica de nuevo los resultados y no integra ramas experimentales.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
