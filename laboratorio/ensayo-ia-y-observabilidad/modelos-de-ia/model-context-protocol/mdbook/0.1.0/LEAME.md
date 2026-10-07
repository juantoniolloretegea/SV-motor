# Libros Markdown mediante MCP · prototipo 0.1.0

Preparación instrumental del 07/10/2026. Componente Rust `sv-mcp-documental 0.1.5-mdbook.1`, generador humano mdBook 0.5.4. **36 comprobaciones Rust favorables.** No se ha consultado a un modelo ni se ha admitido esta ampliación en los tres transportes del SV.

## Qué se ha realizado

El preparador admite un índice `SUMMARY.md` y capítulos Markdown cuya identidad fija un plan autorizado. Produce catálogo, lista de fuentes, mapa de localizadores y constancia final. El servicio conserva `buscar_documentos` y `leer_documento`, el protocolo MCP stdio, la validación estricta JSON, la paginación y el diario reconstruible del componente anterior.

La nueva capacidad conserva exactamente el texto, sus saltos de línea y su orden; divide por encabezados CommonMark, excluyendo los aparentes encabezados dentro de ejemplos delimitados. Cada respuesta identifica documento, sección, huellas, líneas de la sección y posiciones Unicode del fragmento. Las líneas permiten cotejo con el archivo; no son páginas de un PDF. La búsqueda sigue siendo literal y no demuestra comprensión ni suficiencia por sí sola.

El Árbitro-Director conserva su autoridad. El preparador recibe del operador un plan y su SHA-256; el candidato no puede autorizar fuentes, elegir rutas, emitir órdenes ni modificar el catálogo. La comprobación de esta edición utiliza un conductor instrumental Rust, **no ejecuta el Árbitro completo ni el transporte Astra**. Su incorporación a esos recorridos deberá recibirse por separado; no se atribuye aquí una integración que no ha sido realizada.

La [skill](skill/sv-consultar-libros/SKILL.md) contiene las instrucciones de consulta, cita y declaración fundada de U. Se conserva como fuente del laboratorio; no se ha instalado automáticamente en aplicaciones o cuentas. Su formato se ha revisado; el validador auxiliar de empaquetado no pudo ejecutarse porque falta PyYAML. Ese auxiliar no forma parte del SV ni de las 36 comprobaciones Rust.

## Resultado verificable

El [recorrido final](evidencias/recorrido/COMPROBACION.json) contiene cuatro documentos, 31 secciones, 31 entregas de texto y dos consultas inválidas rechazadas. Se reconstruyen exactamente todas las secciones; las solicitudes y respuestas recibidas coinciden con los bytes registrados. El diario contiene 110 sucesos para 36 tramas y registra 36 mediciones del proceso. [Pruebas completas](evidencias/PRUEBAS-03.txt).

Se miden duración de operación, bytes, PID, memoria residente, hilos, descriptores, sockets propios y tiempos de CPU expresados en ticks. La medición es por operación, no muestreo continuo; no mide recursos de un proveedor ni energía consumida. Los datos no disponibles se conservan como nulos. Linux x86_64 aplica el filtro de red heredado; las pruebas observan EPERM al crear/conectar sockets externos y locales. No equivale a una auditoría exhaustiva del sistema operativo ni del generador web.

## Alcance admitido y límites

- Máximo: índice más 63 capítulos, 256 KiB por archivo, 1 MiB de texto original, 64 secciones por documento y 2 MiB de catálogo. Un límite excedido produce rechazo explícito. El catálogo general admite ahora hasta 64 documentos; las demás cotas de respuesta y sesión se conservan.
- El servicio admite 128 llamadas y hasta 256 tramas por sesión; respuestas hasta 8.000 caracteres JSON. El conductor debe comprobar el presupuesto necesario. El término de una sección no acredita la lectura del libro completo.
- Esta primera edición rechaza macros de inclusión, HTML incrustado e imágenes. No los ejecuta ni declara haber leído su contenido. Su admisión futura requiere resolver y cotejar todas las fuentes incorporadas.
- Se rechazan rutas absolutas, ascensos, enlaces simbólicos, archivos no regulares, identidades repetidas, índices discordantes, archivos alterados y contenido sin UTF-8 válido. El plan proviene del operador; sus huellas acreditan identidad, no aprobación científica.
- Un directorio incompleto no es una preparación recibida. `PREPARACION.json` se escribe al final; deben cotejarse las huellas antes del suministro.
- El preparador y el servicio ensayados requieren Linux x86_64. La edición HTML se ha generado en Windows. No se declara comprobada la ejecución del servicio MCP en Windows ni se modifica una instalación histórica.

## Edición humana del manual

[Fuentes del ejemplo](manual/src/SUMMARY.md) y [configuración](manual/book.toml). Se conservan el constructor y el tramo 10 de `docs/manual_svp`, revisión `f97173e07a9dfb8385ed1268d11de422b291507d`. El plan identifica esa base documental; los archivos nuevos de presentación e índice se identifican además por sus propias huellas. La sede original y los once estados pendientes permanecen intactos. Estas copias fijadas son entradas de prueba, no otra sede de mantenimiento.

Generación: `mdbook build manual`. El índice permite navegar por lo existente y muestra los tramos sin contenido. Se desactivan preprocesadores, ejecución en Rust Playground, editor y carga MathJax. El JavaScript del generador se limita a presentación, navegación y búsqueda humana; no decide políticas ni adjudicaciones del SV. La búsqueda de «compatibilidad» y la navegación al tramo 10 se comprobaron en el navegador integrado. No se ha desplegado un sitio público.

El libro no debe publicarse con corpus reservado: la salida HTML y su índice de búsqueda contienen texto. El prototipo sólo utiliza documentación pública del Lenguaje y presentación propia. No se suministran datos clínicos ni credenciales.

## Reconstrucción técnica

En Linux x86_64, con Rust 1.93.1 o compatible:

```text
cargo test --locked --tests
cargo run --locked --bin preparar-libro -- manual/src manual/PLAN.json SHA256_PLAN SALIDA_NUEVA
cargo run --locked --bin sv-mcp-documental -- SALIDA_NUEVA/CATALOGO.json SHA256_CATALOGO DIARIO_NUEVO 256 --fuentes-autorizadas SALIDA_NUEVA/FUENTES.json SHA256_FUENTES
cargo run --locked --bin verificar-diario -- SALIDA_NUEVA/CATALOGO.json SHA256_CATALOGO DIARIO --oficial --fuentes-autorizadas SALIDA_NUEVA/FUENTES.json SHA256_FUENTES
```

Los argumentos SHA-256 deben proceder del cotejo del operador; no son valores a elegir por el candidato. Las pruebas crean evidencias bajo `target/`, nunca en carpetas externas. `Cargo.lock` fija dependencias. La fuente de protocolo, diario y aislamiento procede de MCP 0.1.4-pdf.1; se mantiene su historia. Esta distribución acotada no contiene el extractor ni el preparador PDF y no sustituye su distribución anterior. Preserva el consumo de sus catálogos y localizadores, cuya regresión se ha comprobado.

El código nuevo de lectura e instrumentación prohíbe `unsafe`. El filtro Linux conserva la llamada Rust a `prctl`; no se presenta como ausencia de interfaz con el sistema operativo. No se han añadido Python, C o JavaScript al recorrido de lectura y control. No se declara auditoría binaria integral de todas las dependencias.

Los componentes externos conservan sus licencias. mdBook se distribuye bajo MPL-2.0; su generador y sus recursos no adquieren la licencia documental del SV. Esta entrega no modifica la licencia de las fuentes históricas.

## Dependencia y retorno

Preparación y ensayo instrumental concluidos. Quedan para una actuación posterior la recepción competente del adaptador en el Árbitro y en el transporte elegido, y una prueba del candidato con este suministro. Los resultados anteriores de Astra, PDF y caché no cambian. Seguimiento: S39 y TT-0014; no se declara cierre integral del MCP.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
