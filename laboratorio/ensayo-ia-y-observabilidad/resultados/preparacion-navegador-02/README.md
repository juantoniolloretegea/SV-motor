# EIO-NAV-02 · precompromiso de diagnóstico

Encargo autorizado por la dirección en esta conversación: [768d225fa7ea481713b3a8744f1db566049033ea](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/768d225fa7ea481713b3a8744f1db566049033ea/encargos-ejecucion/EIO-GITHUB-01/navegador-02/ENCARGO.md). Base pública4c68db73b48962bb185a523c1de34da85678d375; derivado de preparacion-navegador-01@966b4b23326312371ad0512ef586c8d9ea401c6b. Un ejecutor, sin delegación. Preparación no acredita ejecución.

## Invariantes y presupuesto

Una ejecución adicional número9/intento1 del workflow361319025. Máximo40min,2280s internos; fases300+600+950+30+200=2080s. Una inferencia como máximo120s, sólo si NAV01–04 y custodia básica pasan. Sin reintentos.
4.294.967.296bytes de RSS agregada, misma familia del controlador Node/Chrome (sesión/grupo y descendientes observados por PID/inicio). Periodo nominal1s más coste de observación. No sustituir por PSS ni sustraer coste dentro de la familia. Disco10GiB, libre mínimo2GiB, transporte20MiB. No límite duro ni medición de memoria física única.
Pesos, tokenizador, petición literal, plantilla, parámetros, dependencias, Rust1.98.0, wasm-bindgen0.2.104 y Cargo.lock del candidato se conservan exactamente. No se optimizan copias, carga, cuantización ni algoritmos Candle.

## Instrumentación

[CASOS.json](CASOS.json) incorpora sin cambiar esperados los controles NAV01–04 y el caso condicionado NAV05.
Rust sólo añade llamadas de diagnóstico en fronteras del adaptador: entrada real tras copias del enlace, tokenizador listo, antes/después de ModelWeights, primer forward/token y finalización. JavaScript registra recursos verificados, antes del enlace y retorno. Se mide capacidad lineal WASM actual; no es RSS. Las marcas usan secuencia por Worker y reloj monotónico/civil del emisor; la página y Node registran recepción con sus propios relojes. No se restan relojes monotónicos entre contextos.
Transporte Worker→página→binding CDP→Node. Se persiste cada recepción con append y fsync en archivos JSONL acotados antes de NAV01. Un mensaje emitido pero no recibido sigue siendo desconocido. La secuencia de página es comprobada; NAV03 exige evidencia persistida de terminación/rechazo antes de permitir el modelo.
Node comprueba preflight: muestras RSS recientes, PID propio ligado al supervisor, archivos de captura activos y cuatro controles conformes. PSS/privada se declaran complementarias, nunca sustituyen RSS.
El supervisor exterior observa y termina la familia. Su hilo complementario lee smaps_rollup por proceso, PSS/Private_Clean/Private_Dirty en bytes, con identidad antes/después, rol derivado, instantes y errores explícitos. Cola de una muestra: omisiones contadas, no muestras inventadas. No bloquea deliberadamente la decisión RSS; señal de exceso antes de publicar nuevos registros o solicitar complemento. Costes CPU/hilo, tiempos de lectura, intervalo efectivo y RSS del supervisor se registran separadamente. El hilo comparte el proceso supervisor; no contiene trabajo del modelo.
La atestación exterior se escribe después de terminar la familia. Conserva últimas líneas completas, errores y presencia/ausencia de cierre normal, incluso si Node no ejecuta finally. Las colas, mensajes en tránsito y el intervalo final mantienen lagunas posibles. Cada journal del controlador se limita a2MiB y los del supervisor a4MiB; el límite global de20MiB se conserva.

## Inspección estática y fuentes

El enlace previo passArray8ToWasm0 reserva y copia entradas a memoria WASM; Uint8Array sobre ArrayBuffer no es otra copia por sí solo. Cursor toma referencia al GGUF; no demuestra duplicación entera. [Candle fijado](https://github.com/huggingface/candle/blob/ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a/candle-transformers/src/models/quantized_qwen3.rs) desquantiza embeddings y usa qwen3.context_length para RotaryEmbedding con sin/cos y vectores f32. Son ubicaciones estáticas, no atribución causal del pico. No se modifica Candle ni se afirma desquantización universal.
[Documentación Linux proc](https://docs.kernel.org/filesystems/proc.html): smaps_rollup proporciona métricas acumuladas; RSS de stat puede ser aproximada. PSS distribuye páginas compartidas; privada y RSS describen magnitudes distintas. Permisos o ausencia se registran; no se elevan privilegios.
Lecturas rectoras fijadas en Lenguaje@68772d8bad39730425d1b73d9c82cf687c39ef02: AGENTS, Pilares05/09, perfiles06/09 y transiciónOP-IMM-001 con adendas completas. Blobs idénticos a los leídos íntegramente para NAV01, comprobados en esta preparación. No decisión de dominio, no A/B como célula SV, no fallo→U, no cambios en núcleo/S32/BIS-03/S38.
Fuentes y adquisiciones permanecen en los guiones originales; canales oficiales Rust, crates.io y repositorios fijados, más HuggingFace conforme a la política recibida. No nuevos destinos de adquisición.

## Custodia y límites

originales-nav01/ conserva el paquete previo; CAMBIOS.patch identifica diferencias completas. MANIFIESTO.json coteja bytes de preparación, excluyéndose a sí mismo. El verificador remoto comprobará identidades y sintaxis antes de adquirir; revisión estática previa no acredita compilación.
El custodio exterior reutiliza gzip/base64 y SHA256 del emisor. El receptor recuperará los bytes y comparará ambas identidades. Se distingue integridad de archivos existentes de cobertura de captura. No guardar pesos, tokenizador, credenciales ni URLs firmadas.
Sin instalaciones, compilación o inferencia en PC; ejecución en runner Linux con Chrome ordinario, sin --no-sandbox. Sin delegación, nuevos servicios, GPU, nuevas dependencias o ampliación de permisos. Cerrar ambas guardas al concluir y entregar incluso el fallo, sin modificar esperados ni repetir. Revisión receptora posterior separará controles, coste/cobertura, custodia, inferencia y contrato. S38 permanece pendiente.
