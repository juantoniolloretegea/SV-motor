# Código del traductor y su contraste de dispositivos

El motor y la integración se conservan sin cambios. `motor` contiene el programa Rust autónomo; `integracion/traduccion.rs` pertenece al gestor documental y requiere el resto de esa aplicación. `selector` contiene el Selector del Árbitro - Director CPU/GPU/TPU/NPU y sus pruebas. Esta edición no incorpora una conexión ejecutable entre Marian y TPU.

El motor se construye en Linux x86_64 con Rust 1.93 o posterior:

```text
cargo build --release --locked --manifest-path motor/Cargo.toml
```

Recibe como argumento una carpeta con los seis archivos del modelo y MANIFIESTO.json. Recibe por entrada estándar un objeto JSON por línea, con `id` y `texto`, y devuelve un objeto por traducción. Requiere las funciones de aislamiento Linux utilizadas en sus fuentes; no deben deshabilitarse para eludir una incompatibilidad.

El programa de comprobación verifica el manifiesto y recalcula las huellas de las trece entradas y salidas anteriores, sin ejecutar el modelo:

```text
cargo run --release --locked --manifest-path comprobacion/Cargo.toml -- RUTA_EXPEDIENTE
```

Para cotejar dos recuperaciones independientes, añádase como segundo argumento la otra carpeta del expediente. La comprobación exige igualdad binaria para todos los archivos enumerados. No equivale a ejecutar el motor ni a recibir su calidad lingüística.

Candle, Tokenizers y las demás dependencias se fijan mediante Cargo.lock. La licencia Apache 2.0 se conserva en `motor/LICENSE-APACHE.txt`; no se sustituye por la licencia documental. La copia del Selector conserva su identidad y sus condiciones de distribución de origen.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

