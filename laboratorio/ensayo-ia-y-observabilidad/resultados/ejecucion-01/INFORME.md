# EIO-GITHUB-01 · preparación interrumpida
Fecha: 18/09/2026. Unidad: UE-LOCAL-CODEX-WINDOWS (Codex, coordinación documental en Windows). Base pública: e1caf6df5bef2696f20d2b9fc5a5b5e9cffe509e.

## Resultado
Bloqueo de comprobación de procedencia, anterior al precompromiso ejecutable. Cero ejecuciones de Actions, cero compilaciones y cero inferencias. No hay resultados de compatibilidad, controles ni calidad del modelo.

La lectura de https://static.rust-lang.org/dist/channel-rust-1.98.0.toml.sha256 mediante navegador devolvió literalmente `net::ERR_BLOCKED_BY_CLIENT`. La herramienta web devolvió error interno para ese recurso y el manifiesto. La consulta local HTTPS devolvió conexión rechazada en 127.0.0.1:9. Son resultados de canales distintos; no acreditan indisponibilidad del servidor oficial ni identifican qué componente impone el bloqueo.

Rust 1.98.0 sí figura en [las notas oficiales](https://github.com/rust-lang/rust/blob/1.98.0/RELEASES.md), con fecha de publicación 20/08/2026. No se sustituye versión ni se presenta una instalación anterior como comprobación de la distribución actual.

Se detiene la preparación por aplicación conservadora de la prohibición de eludir rechazos: no se traslada la consulta denegada a un ejecutor remoto para sortearla. Esto es una decisión de perímetro de la unidad ejecutora, no una prueba de que Actions no pueda acceder al servidor. La revisión receptora debe determinar si la preparación remota ordinaria prevista puede continuar bajo esa restricción o precisar el canal admisible.

## Preparación efectuada
Lectura del contrato general, inferencia, observabilidad, matriz y fuentes rectoras. Localizados Candle exacto y su ejemplo quantized-qwen3, API SpanExporter de OpenTelemetry v0.31.0 y metadatos Qwen/Unsloth. El manifiesto parcial separa identidades publicadas de verificaciones de bytes no realizadas.

SV-motor es público. La [documentación de facturación](https://docs.github.com/en/billing/concepts/product-billing/github-actions) permite cómputo estándar público gratuito y distingue la cuota de artefactos. Se había elegido evidencia textual en logs/resumen, sin upload-artifact ni cachés nuevas, al no acreditar cuota libre de artefactos. [Configuración prevista](https://docs.github.com/en/actions/reference/runners/github-hosted-runners): ubuntu-24.04 x64 estándar. No se ha provisionado ni medido un ejecutor.

## Carencias expresas
No se ha preparado código ejecutable, Cargo.lock, fijación transitiva, banco implementado, precompromiso ni workflow. Las especificaciones y esperados originales permanecen en los documentos del corte base; no se afirma haberlos implementado o ensayado. Véase MATRIZ.tsv.

Sin medidas de carga, memoria, tiempos, disco remoto o coste de telemetría. No cabe recomendar una instalación local basándose en esta preparación. Construcción WASM y ejecución en navegador pendientes; WASI y demás plataformas fuera del resultado.

## Continuidad
Antes de cualquier lanzamiento se requieren resolución del canal de procedencia, identidad e integridad completas, implementación del banco y controles de recursos, Cargo.lock, precompromiso remoto leído de nuevo y comprobación del presupuesto. Se conservan los límites originales: tres ejecuciones totales, 40 minutos cada una, CPU, 2048 tokens retenidos, 128 generados, 120 segundos por caso, 10 GiB de disco adicional y 2 GiB libres, objetivo de 4 GiB conjunto y evidencia hasta 20 MiB por ejecución. Ningún intento consumido.

Esta entrega no constituye recepción ni aceptación, no acredita privacidad, no integra el modelo en el núcleo y no modifica S32/BIS-03. Punto de parada: revisión receptora del bloqueo. Sin continuación automática.
