# EIO-NAV-01 · candidato previo a ejecución

Preparación autorizada el 20/09/2026. Una sola ejecución adicional del workflow existente 361319025: número 8, intento 1, fase navegador. No es un resultado de ejecución.

## Cortes y lectura rectora

- Laboratorio recibido: SV-motor@ee2946062f878282513b519e467cb6bfbb8a05d1.
- Encargo recibido: SV-sala-de-maquinas@e14c948d747e9c235fb01fddfec5d06c8fb9625c, encargos-ejecucion/EIO-GITHUB-01/navegador-01/ENCARGO.md, blob 9d8313de46dfdf2f92337c5068a5108c3ccf63a0.
- Lenguaje@1b3cd0d4c31516807e87f4261cb6ffa795a7591d: AGENTS, Pilares de 05/09 completos incluidas adendas, acta de perfiles/contratos/ensamblaje de 06/09 completa y acta de transición desde OP-IMM-001 completa (§§1–30). No se modifica ninguna pieza rectora ni el núcleo.
- EIO-06 aporta Cargo.navegador.toml y Cargo.lock recibido (171 paquetes). EIO-JSON-01 conserva su banco y su reserva histórica de custodia.
- S38 sigue pendiente en su sede canónica; esta prueba no lo abre.

## Candidato y cambios

Originales preservados en originales/. CAMBIOS.patch muestra sustituciones completas de los archivos derivados, sin aplicar cambios sobre las fuentes originales. MANIFIESTO.json identifica cada archivo previo, salvo él mismo.

Se conserva el nombre de paquete, las dependencias y el lock de EIO-06. Del manifiesto derivado se retiran sólo los destinos binarios que esta construcción --lib no utiliza. El verificador comprobar/ejecutar, el adaptador Candle/Qwen, la plantilla literal y los parámetros permanecen; los cambios del adaptador se limitan a importar el reloj condicional. navegador.rs añade sondas deterministas y observación explícita de la cobertura de generación.

Relojes: en WASM, Instant usa performance.now() y devuelve Duration; el tiempo civil medido mediante Date.now() se convierte en SystemTime desde UNIX_EPOCH. Inicio y final de cada span se pasan explícitamente al SDK. La ruta nativa sigue usando std::time y los métodos originales. La exportación añade timestamps originales a los campos ya existentes. Drop de Telemetria cierra el span WASM con tiempo medido también al retornar un error, evitando el reloj implícito. No se inventan marcas ni se equipara el tiempo civil a duración monotónica.

Inspección de API: opentelemetry-rust v0.31.0, trace/tracer.rs usa el tiempo dado o, si falta, time::now; trace/span.rs permite end_with_timestamp; SimpleSpanProcessor exporta síncronamente y force_flush no tiene cola. No se utiliza BatchSpanProcessor ni su Instant/thread. Esta inspección no acredita ejecución.

## Ensayo precomprometido

CASOS.json fija entradas literales, eventos, condiciones, resultados y plazos antes del despacho. NAV-01 y NAV-02 pasan por el Rust/WASM y el verificador recibido. NAV-04 omite únicamente consulta.B en el exportador y exige EVENTO_AUSENTE y efecto no ejercido. No se convierte en U.

NAV-03 invoca un bucle Rust infinito, con black_box, sin consulta de cancelación ni retorno. La página permanece fuera del Worker, ordena terminate a los 2000 ms desde el aviso de entrada, invalida el ID y elimina sus manejadores. Comprueba una respuesta tardía sintética rotulada como tal por el mismo receptor y una tarea finita nueva; ambas tareas del caso tienen una cota conjunta de 10 s. La prueba no produce un mensaje real desde un Worker ya terminado ni demuestra por ausencia de mensajes el cese físico de toda actividad.

Sólo si NAV-01..04 pasan se llama una vez a Candle/Qwen: mismo prompt, pesos y tokenizer, ArgMax, semilla 299792458, 128 tokens nuevos, contexto 2048, EOS 151645. La cota exterior de 120 s comprende carga de recursos y generación; no se consume una segunda inferencia si no llega a completarse. Se preserva cualquier ESTRUCTURA, interrupción o error sin reparar el texto.

Navegador del runner, sin instalación adicional: versión observada y CDP Browser.getVersion. Sandbox ordinario preservado, sin --no-sandbox ni privilegios nuevos. Entorno del proceso Chrome reducido a PATH/HOME temporal/TMPDIR/LANG, sin credenciales heredadas. Perfil efímero nuevo y servidor/CDP en 127.0.0.1. El servidor sólo ofrece GET de las rutas inventariadas en recursos.json. CSP limita scripts, Worker y conexiones al mismo origen. Esto no es encapsulación integral ni cortafuegos del host. JavaScript transporta y coordina; las condiciones y el juicio sintético permanecen en Rust.

## Recursos, adquisición e instrumentación

Entradas originales fijadas en pruebas/ENTRADAS_ENSAYO.json, copiadas documentalmente en originales/. Se usa la política hf recibida sin ampliarla; está preservado adquirir.sh. Rust 1.98.0 se instala sólo en RUNNER_TEMP/eio/toolchain-1.98.0 del runner; no se usa rustup ni se toca el PC.

Herramienta adicional autorizada: cargo install wasm-bindgen-cli --locked --version 0.2.104 --root temporal. Fuentes instrumentales oficiales: índice https://index.crates.io/ y archivos https://static.crates.io/, con checksums de Cargo; cualquier fuente Git declarada por su lock quedará identificada. Su lock y paquetes se inventarían aparte, sin sumar sus paquetes a los 171 del candidato. No se atribuye una suma oficial no consultada al ejecutable construido.

Máximos: adquisición 300 s, herramienta 600 s, construcción 950 s, enlace/inventario 30 s y navegador 200 s = 2080 s. Plazo interior global 2280 s y job 2400 s. Cada fase queda limitada al tiempo restante. El supervisor conserva orden, inicio, fin, retorno, causa, medidas y cierre. Muestreo nominal cada segundo más coste del observador: RSS agregado de familia observada, umbral 4 GiB en navegador; no límite duro. Disco propio más checkout máximo 10 GiB, reserva 2 GiB y evidencia 20 MiB. No se pretende observar procesos fugados antes de registrarlos ni otros procesos del host.

## Custodia y cierre

Antes de transportar, custodiar.py calcula bytes/SHA-256 originales y gzip, y emite base64 numerado con manifiesto y marcador final. Se recuperan los bytes y se cotejan contra las huellas del emisor. Se incluyen los archivos de evidencia y recursos ligeros servidos, JS, snippets y WASM enlazado. Pesos/tokenizer no se publican: se preservan sus identidades en recursos.json; el artefacto WASM previo al enlace y el ejecutable wasm-bindgen se identifican, sin atribuirles carga en navegador. Cualquier ausencia o exceso de cota se declara custodia incompleta.

Cota de representación emitida 20 MiB. Las huellas del emisor y el receptor demuestran identidad bajo su perímetro, no independencia ante un anfitrión malicioso. El servidor registra las identidades de archivos previamente inventariados, sin proceso propio que los modifique; el Worker coteja hash de WASM y entradas antes del uso. La identidad del enlace JS descargado se coteja contra el mismo recurso servido; no prueba atestación independiente del motor.

El controlador termina Workers, cierra navegador y servidor; el supervisor termina y vuelve a comprobar los procesos propios observados mediante PID y tiempo de inicio. Las guardas del repositorio se cerrarán al recibir el resultado, incluso si falla. No hay reintentos, cachés remotas, upload-artifact, servicios persistentes ni escrituras del runner al repositorio. La publicación receptora y el juicio científico conservan sedes diferentes.

## Revisión estática

Revisión manual de YAML, shell, Python y API Rust; parseo sintáctico V8 del controlador y scripts JS (sin ejecutarlos). El propio runner verificará identidades, AST Python, bash -n y node --check antes de adquirir. Ninguna comprobación preparatoria se presenta como ejecución del navegador.

Referencias primarias consultadas:
- https://github.com/open-telemetry/opentelemetry-rust/tree/v0.31.0/opentelemetry-sdk/src/trace
- https://html.spec.whatwg.org/multipage/workers.html#dom-worker-terminate
- https://wasm-bindgen.github.io/wasm-bindgen/reference/cli.html
- https://docs.github.com/en/actions/reference/runners/github-hosted-runners
