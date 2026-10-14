# Código del contraste CPU/TPU de OPUS-MT

Esta copia contiene el Selector del Árbitro - Director CPU/GPU/TPU/NPU, el adaptador experimental del modelo Helsinki-NLP/opus-mt-en-es, la observación CPU y las dependencias PJRT necesarias para reproducir la compilación. Las dependencias externas resueltas por Cargo se identifican en los archivos de bloqueo; no se incluyen pesos ni credenciales.

## Organización

- `src/lib.rs`: Selector original, sin modificaciones durante el contraste.
- `adaptador-helsinki/src/main.rs`: identidad del modelo, comprobación numérica CPU y conservación.
- `adaptador-helsinki/src/grafo.rs`: operaciones Marian, evaluación CPU de referencia y representación StableHLO.
- `adaptador-helsinki/src/serie.rs`: solicitudes, mediciones, invocación del Selector y ejecución del destino elegido.
- `adaptador-helsinki/src/tpu.rs`: enlace PJRT, memoria del dispositivo, ejecución síncrona y atribución de las trazas.
- `telemetria`: observación del proceso CPU en Rust.
- `dependencias/pjrt-rs`: interfaz comunitaria y cabeceras del fabricante; se conservan sus licencias.
- `dependencias/xla-protos`: definiciones de los registros originales de observación.

## Compilación y ejecución

Entorno comprobado: Ubuntu 22.04 x86_64, glibc 2.35 y Rust 1.98.1. Se emplearon libclang 14 y protoc 3.12. La generación de interfaces requiere las cabeceras estándar de C++ y, con esta versión de protoc, la opción `--experimental_allow_proto3_optional`. Son herramientas de compilación; no sustituyen las decisiones del Selector ni las comprobaciones Rust.

```text
cargo build --release --locked --manifest-path adaptador-helsinki/Cargo.toml
sv-helsinki-selector verificar-cpu MODELO ENTRADAS.jsonl DESTINO_NUEVO
sv-helsinki-selector serie-tpu MODELO ENTRADAS.jsonl LIBTPU_SO DESTINO_NUEVO
```

La ejecución comprobada utiliza cuatro trabajadores de Rayon y `TOKENIZERS_PARALLELISM=false`. Cada destino de salida debe ser nuevo. El modelo y los seis archivos de su manifiesto deben coincidir con la revisión fijada. La biblioteca oficial libtpu 0.0.50 se carga por su ruta y se identifica mediante SHA-256.

El ejecutable de la serie final tiene SHA-256 `3355a85d4d1272a51c9a75a5baa1d07b49c34e9aa95d45470d0a987dba6a27d4`. El Selector conserva SHA-256 `2b6cd91da365f15ab314ea69ef6eeec1b12826ddf0de01fcca58191ef5afff05`. El archivo de fuentes y los ejecutables conservados permiten distinguir la copia documental de lo efectivamente utilizado.

## Alcance de los lenguajes y de la integración

La construcción de las operaciones del modelo, la selección, la generación, la observación propia y el cotejo se realizan en Rust. La ejecución TPU depende de PJRT y de la biblioteca nativa de Google. Las cabeceras C/C++, las definiciones Protobuf y los archivos auxiliares de compilación de terceros no permiten describir toda la infraestructura como íntegramente Rust.

La representación StableHLO pertenece al adaptador experimental del modelo. No es la representación intermedia del SV y no modifica su semántica. El servicio documental activo conserva su implementación y aislamiento anteriores. Este contraste no incorpora todavía el adaptador al servicio de producción.

Las instrucciones y límites científicos se encuentran en el protocolo y en su adenda de medición completa. Las licencias de terceros se conservan en sus archivos originales.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
