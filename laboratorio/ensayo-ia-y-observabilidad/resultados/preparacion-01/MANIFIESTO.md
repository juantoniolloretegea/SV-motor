# Manifiesto previo de componentes

Estado: candidato no compilado ni ejecutado. No hay descargas de componentes ni huellas medidas de sus binarios. Las huellas de ARCHIVOS.tsv sólo corresponden al texto de esta entrega.

| Componente | Identidad fijada / procedencia | Tamaño, integridad y licencia |
|---|---|---|
| Rust | 1.98.0; x86_64-unknown-linux-gnu; wasm32-unknown-unknown previsto; static.rust-lang.org | Tamaños y SHA-256 pendientes de manifiesto oficial y contraste antes de instalación. MIT/Apache-2.0, avisos por recuperar. |
| Candle | huggingface/candle, ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a | Commit fijado, archivos/package completos pendientes. MIT OR Apache-2.0; aviso MIT conservado en inferencia/AVISOS.md. |
| Pesos | unsloth/Qwen3-0.6B-GGUF; Qwen3-0.6B-Q4_K_M.gguf; f2d6f9ca53a254cc379437c49e4b2eb447f779df | SHA-256 publicado ac2d97712095a558e31573f62f466a3f9d93990898b0ec79d7c974c1780d524a; NO contrastado con bytes. 397 MB redondeados; exacto pendiente, máximo 500 MB. Apache-2.0 declarado; aviso pendiente. |
| Tokenizador/configuración/plantilla | Qwen/Qwen3-0.6B, c1899de289a04d12100db370d81485cdf75e47ca | Hash/tamaño de bytes pendientes; Apache-2.0 declarado. Plantilla especializada un turno, sin tools, thinking=false; equivalencia por comprobar. |
| OpenTelemetry API/SDK | opentelemetry y opentelemetry_sdk =0.31.0; crates.io | default-features=false, trace; tamaños/checksums del registro y lock pendientes. Apache-2.0 declarado, textos pendientes. |
| Serde / JSON | serde =1.0.228 +derive; serde_json =1.0.145; crates.io | MIT OR Apache-2.0 declaradas; integridad/transitivas pendientes. |
| tokenizers | =0.23.1, default-features=false, fancy-regex; crates.io | Apache-2.0 declarada; integridad/transitivas y compatibilidad pendientes. |
| wasm-bindgen | =0.2.104, opcional destino WASM; CLI igual versión prevista | MIT OR Apache-2.0 declaradas; adquisición y enlace no realizados. |
| checkout | actions/checkout, 11bd71901bbe5b1630ceea73d27597364c9af683, referencia v4.2.2 | Commit fijado; MIT declarado; no ejecutado. |

Las licencias declaradas se distinguen de avisos efectivamente conservados. No se cambia la licencia de el proyecto ni se redistribuyen pesos. Las dependencias transitivas, características efectivas, sumas del registro y tamaños se desconocen hasta resolución autorizada. No se inventa Cargo.lock.

Fuentes previstas exclusivamente: distribución oficial Rust, GitHub oficial Candle/OpenTelemetry/actions, crates.io, distribuciones Qwen/Unsloth identificadas. Sus redirecciones y bytes pendientes requieren fundamento de acceso y verificación; no se consultan ahora ni se proponen espejos. No se publican credenciales.

## APIs e integración

Firmas tomadas de las lecturas documentales disponibles: modelo quantized_qwen3, GGUF, LogitsProcessor ArgMax, Tokenizer; exportador OpenTelemetry v0.31.0. No acreditadas por compilación: combinación de paquetes, constructores/límites del SDK, SpanData y exportación síncrona, carga de pesos/tokenizador, plantilla y objetivo WASM. CPU sin CUDA/Metal; semilla 299792458, muestreo ArgMax; contexto 2048, salida128. Sin herramientas propuestas por el modelo, entrenamiento o conversión.

Preparación remota prevista: resolución sin compilación, recuperación íntegra del lock/manifiesto y nuevo corte antes del ensayo. En este corte faltan prerrequisitos intencionadamente; ninguna guarda autoriza su propia apertura.
