# Reproducción de la generación gobernada de Qwen

Las fuentes de esta carpeta corresponden exclusivamente a la ejecución identificada en [IDENTIDAD.json](IDENTIDAD.json). Las fuentes anteriores permanecen en su ubicación histórica. El núcleo documental se conserva en [sv-core-1c6b842.tar.gz](sv-core-1c6b842.tar.gz); sus 61 archivos coinciden con [NUCLEO.json](NUCLEO.json), sin modificaciones.

En Linux x86-64, dispóngase la carpeta `selector` con su estructura conservada. Créese `selector/dependencias` y extráigase allí el archivo del núcleo, de modo que exista `selector/dependencias/sv-core-1c6b842/Cargo.toml`. Extráigase también el [archivo original de Aprender](../aprender-d0a59c1882ddf43656d1067cc6d5c1943aba22d3-original.tar.gz) en esa carpeta de dependencias, conservando el nombre `aprender-d0a59c1882ddf43656d1067cc6d5c1943aba22d3`. Aplíquense únicamente las diez fuentes de [instrumentacion-aprender](../instrumentacion-aprender), según [INSTRUMENTACION.json](../INSTRUMENTACION.json). La dependencia comunitaria conserva su [licencia MIT](../LICENCIA-APRENDER.txt).

El programa del ensayo se compila con Rust 1.98.1, el `Cargo.lock` conservado y las características indicadas en `Cargo.toml`:

```text
cargo test --locked --manifest-path selector/ensayo-qwen/Cargo.toml --bin sv-contraste-qwen-cpu-gpu
cargo build --release --locked --manifest-path selector/ensayo-qwen/Cargo.toml --bin sv-contraste-qwen-cpu-gpu
```

El ejecutable conservado se denomina `sv-aprender-arbitro-director`. La descarga oficial se realiza mediante `descargar archivo.gguf`; la ejecución mediante `gobernar archivo.gguf directorio_resultados_nuevo`. La descarga verifica la revisión y SHA-256 fijados. El directorio de resultados debe ser nuevo. La ejecución exige CPU, NVIDIA T4, bibliotecas del controlador y NVML compatibles. El procedimiento de ensayo desactivó Internet antes de inferir. No se acepta un dispositivo sólo por su nombre: la identidad y los recursos se comprueban durante la ejecución.

Para comprobar los resultados conservados no se necesitan pesos ni GPU. El lector posterior requiere la opción `float_roundtrip` de `serde_json`: garantiza recuperar exactamente los valores binarios de las mediciones decimales antes de volver a calcular las huellas de los registros. El primer lector no activaba esa opción y produjo una discrepancia de lectura; se corrigió el lector, conservando intactos los registros y el ejecutable ensayado. No hubo nueva inferencia.

```text
cargo build --release --locked --manifest-path selector/ensayo-qwen/Cargo.toml --bin verificar_gobierno --features serde_json/float_roundtrip
verificar_gobierno qwen-arbitro-director-20261014.tar.gz.parte-00 e7ef5213898a87928b3ce5c58d8dfcb39cd41009e7bbf170510e6d69e24289cf verificacion-nueva.json
```

Las dos partes deben estar juntas. El verificador reúne sus bytes en el orden fijado, coteja el archivo completo y verifica manifiesto, secuencia de registros, decisiones y revalidaciones del Selector, clases ejecutadas, lanzamientos, generación efectiva y vectores completos. También admite el archivo original reunido. La verificación no ejecuta el modelo ni sustituye una revisión independiente.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
