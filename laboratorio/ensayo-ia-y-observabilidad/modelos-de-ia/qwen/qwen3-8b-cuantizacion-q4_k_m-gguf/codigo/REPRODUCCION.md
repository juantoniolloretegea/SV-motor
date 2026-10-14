# Fuentes y reproducción del ensayo Qwen3-8B

Las fuentes del Selector, la conexión, el supervisor, las mediciones y los verificadores se encuentran en `selector/`. Su disposición conserva las relaciones de los manifiestos Cargo utilizados. `historicos/r1/main.rs` y `historicos/r2/main.rs` identifican las fuentes de las dos ejecuciones anteriores; no son la fuente de la conexión posterior.

El archivo `aprender-d0a59c1882ddf43656d1067cc6d5c1943aba22d3-original.tar.gz` contiene la copia íntegra del repositorio comunitario en la revisión examinada. SHA-256: `1b3804d6693198c08bfd68d4878080a2db6616a2be011abc7707313f1a707f10`. Descomprímase bajo `selector/dependencias/`, conservando el directorio `aprender-d0a59c1882ddf43656d1067cc6d5c1943aba22d3`. Después, incorpórense en ese directorio los diez archivos de `instrumentacion-aprender/`, respetando sus rutas relativas. `INSTRUMENTACION.json` identifica sus huellas finales. La copia original permanece separada de las modificaciones del ensayo.

`OPERACIONES-GPU.json` relaciona las doce funciones observadas con sus definiciones Rust, copiadas para lectura directa en `operaciones-gpu/`. Estas copias coinciden con las fuentes del archivo comunitario; no requieren aplicarse como modificaciones adicionales. Las modificaciones instrumentales registran funciones, PTX, lanzamientos y transferencias, conservan vectores de paridad, exponen el flujo de ejecución para su medición e impiden la inicialización de cuBLAS/cuBLASLt y WGPU. No cambian las ecuaciones del modelo ni sus pesos.

## Compilación

Entorno utilizado: Linux x86-64, Rust 1.98.1, edición 2021. Las dependencias de registro y sus sumas de comprobación están fijadas en `selector/ensayo-qwen/Cargo.lock`; `DEPENDENCIAS.json` facilita su consulta. La copia íntegra de Aprender incluye también documentación y herramientas que no intervienen en la inferencia examinada. Las dependencias de registro se recuperan mediante Cargo; no se distribuyen el compilador, el sistema operativo, los controladores ni los pesos del modelo.

Desde `codigo/`:

```text
cargo test --locked --manifest-path selector/ensayo-qwen/Cargo.toml
cargo build --release --locked --manifest-path selector/ensayo-qwen/Cargo.toml
```

El programa principal se compila con el nombre `sv-contraste-qwen-cpu-gpu`. El archivo ejecutado en Kaggle se denominó `sv-aprender-selector-conexion`; el cambio de nombre no altera sus bytes. Para reconstruir r1 o r2, utilícese su `main.rs` histórico en lugar del actual; el módulo `observacion.rs` se conserva con cada fuente. La comprobación histórica r1 debe mantener la omisión documentada y no utilizarse como configuración recomendada. Los verificadores actuales tienen alcance posterior al ensayo.

## Ejecución delimitada

El programa proporciona `descargar`, `ejecutar` y `conectar`. `descargar` obtiene exclusivamente los pesos oficiales fijados y comprueba su SHA-256. La inferencia debe efectuarse con Internet desactivado, tras verificar el ejecutable y los pesos. Ejemplo de argumentos, con rutas del entorno de ejecución:

```text
sv-aprender-selector-conexion descargar /kaggle/working/Qwen3-8B-Q4_K_M.gguf
sv-aprender-selector-conexion conectar /kaggle/working/Qwen3-8B-Q4_K_M.gguf /kaggle/working/resultado-conexion
```

`conectar` supervisa un proceso específico, fija su configuración y limita su duración a 600 segundos. Conserva los archivos de resultado y crea un archivo tar.gz; no introduce reintentos de inferencia. Sus argumentos no convierten el proceso en un servicio productivo del Árbitro-Director.

Para revisar un resultado ya conservado, sin inferencia, los programas `cotejar`, `auditar_gpu` y `verificar_conexion` reciben la ruta del archivo, su SHA-256 esperado y la ruta JSON de salida. El primero comprueba integridad y dimensiones; el segundo relaciona funciones, PTX y lanzamientos; el tercero reconstruye las decisiones y recalcula las comparaciones completas. Los tres se incluyen en `selector/ensayo-qwen/src/bin/`.

## Licencias y alcance de Rust

Aprender mantiene su licencia MIT y la atribución a Pragmatic AI Labs, conservadas en `LICENCIA-APRENDER.txt` y en el archivo original. Las dependencias conservan sus propias licencias. La atribución documental del ensayo no sustituye esas condiciones.

El cálculo GPU observado está definido en Rust, emite PTX y utiliza CUDA Driver API. El controlador NVIDIA y su compilador PTX siguen siendo necesarios. La criptografía, la compresión y otras dependencias auxiliares pueden contener código nativo de otros lenguajes; su existencia no equivale a delegar el cálculo matricial observado en C++. No se declara que todo el entorno esté escrito en Rust.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
