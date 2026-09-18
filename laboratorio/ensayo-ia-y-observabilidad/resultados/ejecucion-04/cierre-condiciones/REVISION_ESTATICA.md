# Cierre de condiciones · avance técnico, no listo para lanzamiento

Autorización humana del cierre técnico localizado en el commit privado bd2b45c315ca01b4ab452866906453baded3521f. Base pública e78a55dabb5f8eeab525c49898c0474f35a995fc. Se conservan todos los antecedentes y el contador 2/3; no se activa Actions. Esta aportación añade evidencia bajo ejecucion-04/cierre-condiciones/ sin sobrescribir los informes anteriores.

## Entradas y plantilla

ENTRADAS_ENSAYO.json distingue paquetes Rust, entradas consumidas y referencia documental. Se incorporan los metadatos receptores: GGUF 396705472 bytes / ac2d97712095a558e31573f62f466a3f9d93990898b0ec79d7c974c1780d524a; tokenizer.json 11422654 bytes / aeb13307a71acd8fe81861d94ad54ab689df773318809eed3cbe794b4492dae4. El descargador mantiene los commits completos, sin reemplazarlos por main. GGUF: evidencia editorial consultada en main y asociación por último cambio; no se afirma recuperación directa a su commit ni cotejo de bytes. Tokenizador: puntero editorial en revisión fijada. Ambos se verificarían por tamaño/SHA antes de consumo, nunca por inferencia previa.

Se intentó una lectura ordinaria de los bytes originales de tokenizer_config.json en c1899de289a04d12100db370d81485cdf75e47ca con System.Net.Http, HTTPS, sin redirecciones y máximo 30 s. Falló antes de recibir HTTP: conexión rechazada en 127.0.0.1:9. No se recibió archivo, código HTTP o hash; no se siguió otro canal ni se alteró proxy/TLS/permisos. El proceso terminó con retorno1. No se atribuye ese resultado al servidor HF ni se declara una prohibición global.

Falta cotejar chat_template del archivo original con la especialización de un turno sin tools/thinking. El adaptador consume GGUF y tokenizer.json; NO lee tokenizer_config.json. Sus parámetros arquitectónicos se toman de metadatos GGUF mediante Candle; plantilla especializada en el código. La referencia pendiente no se presenta como configuración aplicada.

Código y petición fijados, sin modificación de sus bytes en esta aportación: adaptador.rs de e78a55d, SHA-256 b7119805972497f84058528677bf1381a6722a3e48420cd08cb7362b38df62a5; peticion.txt del mismo corte, SHA-256 2ec34e21c96200a2106aed7f8a696e31ddbbe02e92238803a2c1727d40e1101f. Tamaños/blobs de fuente contrastados por lectura; huellas recalculadas en copia textual UTF-8 local. No se calcula SHA de la configuración sobre contenido renderizado.

## Aprovisionamiento candidato

Se reemplaza ensayar.sh incompleto y se añaden adquirir.sh/aprovisionar-ensayo.sh. Instalarían distribución directa en prefijo nuevo RUNNER_TEMP/eio/toolchain-1.98.0, usando cargo/rustc/rustdoc explícitos; nunca Rustup ni continuidad de runner anterior. Los tres paquetes nativos reutilizan tamaños/SHA efectivamente verificados en la preparación satisfactoria. Se exige además rust-std-1.98.0-wasm32-unknown-unknown.tar.xz: su URL/revisión/función están fijadas, pero tamaño y suma oficial siguen pendientes, identificados como null; no se ha consultado su distribución en este cierre.

Cargo.lock se conserva íntegro, SHA-256 fcbe8edec58cc8eded54d40b6852b12061962d521fb60308b106506a4a693f63. Adquisición con cargo fetch --locked; construcción con --locked --offline, mismas características/versiones. No generar ni actualizar lock. Toda la adquisición está dentro del supervisor de aprovisionamiento.

HTTPS sin credenciales/cookies, seis saltos como máximo, cada destino cotejado ANTES de la siguiente solicitud. Rust sólo static.rust-lang.org; entradas HF sólo huggingface.co, cdn-lfs.huggingface.co, cdn-lfs.hf.co y cas-bridge.xethub.hf.co. Destinos distintos, cambio a HTTP, fallo TLS/HTTP, tamaño o SHA distintos detienen la operación; sin espejos ni reintentos. Esa lista es un límite del candidato, no evidencia de que esos destinos se hayan usado en este cierre ni de que cubra toda distribución posible. Se registran host/salto/archivo sin queries firmadas de CDN. Los bytes se cotejan antes de extraer, instalar o cargar el modelo.

El JSON conserva plantilla_cotejada=false y referencias/sumas desconocidas null. Los guiones rechazan esos pendientes antes de red; además estado-acceso.json mantiene ensayo_habilitado=false. No se abre la guarda ni se afirma completo el precompromiso experimental.

## Revisión estática y disponibilidad de comprobadores

- [Candle quantized_qwen3](https://github.com/huggingface/candle/blob/ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a/candle-transformers/src/models/quantized_qwen3.rs), blob 3fa004485f2753b71b5e6c8389bd58593ccb86e0: from_gguf(Content, &mut Read+Seek, &Device) y forward(&Tensor, usize) coinciden con llamadas del adaptador. Lee arquitectura de GGUF; no otro archivo de configuración.
- [OpenTelemetry provider v0.31.0](https://github.com/open-telemetry/opentelemetry-rust/blob/v0.31.0/opentelemetry-sdk/src/trace/provider.rs), blob 2b05f89aea73742640e3bb536e78d477684dcf62: métodos with_max_attributes_per_span, with_max_events_per_span y with_max_attributes_per_event presentes.
- [Exportador v0.31.0](https://github.com/open-telemetry/opentelemetry-rust/blob/v0.31.0/opentelemetry-sdk/src/trace/export.rs), blob f09e5d57076f263369a534b0aa8769327eddfa9a: export(&self, Vec<SpanData>) retorna Future Send con OTelSdkResult, concordante con implementación. Esto no valida por compilación toda la combinación/transitivas.
- YAML completo revisado por lectura: sólo workflow_dispatch, preparación=2, ensayo=3, intento=1, máximo3; commit solicitado/GITHUB_SHA/HEAD; permisos contents:read; rutas propagadas por GITHUB_ENV; supervisión y errores set-e/pipefail. No se modifica YAML ni se proclama validación sintáctica automática.
- Se localizó Bash de Git existente y se intentó únicamente --noprofile --norc -n, con BASH_ENV vacío, sin ejecutar scripts. Bash no arrancó: couldn't create signal pipe, Win32 error5; retorno -1073741502. La orden PowerShell finalizó con1. No es un diagnóstico de sintaxis del candidato. No se probaron WSL, otra shell o elevación. Los demás archivos no se intentaron después del fallo.

Esperados y 24 controles deterministas conservados; no se ejecutan ni se adaptan al resultado del modelo. La revisión estática no acredita construcción ni funcionamiento.

## Presupuesto y alcance

Máximos secuenciales previstos: aprovisionamiento480s, compilación600s, banco30s, siete inferencias120s cada una, costes15s y comprobación WASM240s: total2205s. Se mantiene plazo global2280s desde primer paso y trabajo40min. El supervisor comprueba el remanente antes de cada fase y rehúsa iniciarla si no cabe su máximo más5s; deja pendiente esa fase y termina sin relanzar. No se garantiza terminar todas las fases, pues arranque/checkout/limpieza también consumen tiempo. Se mantienen las demás cotas y las limitaciones del muestreo.

Seguridad pasiva: capacidades reducidas y separación efectiva prevista, no garantía por carpetas. Seguridad activa: verificaciones/rechazo/parada cuyo efecto debe observarse. Observabilidad: registro/custodia y pérdidas; no demuestra por sí sola prevención. No se añade vigilancia de servicios/red/equipo ni otra telemetría. cargo check WASM sólo acreditaría construcción; navegador real continúa no automatizado/no acreditado.

## Pendientes exactos y parada

1. Obtener bytes originales de tokenizer_config.json en la revisión fijada, tamaño/SHA y cotejo de chat_template. Operación intentada y fallida documentada; causa de la conexión rechazada no investigada ni reinterpretada.
2. Obtener tamaño y SHA-256 oficial del estándar Rust1.98.0 para wasm32-unknown-unknown y completar esa identidad. Operación no realizada; no se fabrica la suma ni se sustituye componente.
3. Validación de sintaxis automatizada no obtenida; compatibilidad completa requiere futura compilación autorizada. Navegador real no acreditado.

No se consume la tercera ejecución con estos pendientes. Entrega de avance y límites para revisión, sin aceptación experimental, promoción o cambio S32/BIS-03.
