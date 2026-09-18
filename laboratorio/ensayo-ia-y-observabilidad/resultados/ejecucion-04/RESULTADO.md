# EIO v3 · preparación número2 satisfactoria; ensayo pendiente

[Run 35355495301](https://github.com/juantoniolloretegea/SV-motor/actions/runs/35355495301), intento 1, workflow_dispatch, fase preparacion, commit 7f7e36c2ea452b3ed0fd9ab416f4405312d5f9be. Inicio de run 14:19:20Z, actualización final 14:19:45Z del 18/09/2026. Plataforma: completed/success; job 105633731399 y todos sus pasos terminados con success. **Contador acumulado 2/3.** El run 1 fallido se conserva y cuenta.

## Instalación y resolución observadas

Log literal decodificado por conector: [job-105633731399.txt](job-105633731399.txt), sin reconstrucción de salidas. SHA-256 de la copia UTF-8 local conservada:204e6c50cd88332ff8e708ff4bbd16bec4d68760179ce6252f451f55e9de383a. La capa temporal del log no equivale a medición independiente de cada instrucción capturada.

- rustc 1.98.0 (88d9e12ae 2026-08-18), host x86_64-unknown-linux-gnu; commit/date/release íntegros en log.
- cargo 1.98.0 (797e8a9bc 2026-08-05).
- rustc/cargo/rust-std adquiridos por HTTPS oficial, sumas oficiales concordantes antes de extraer y ejecutar sus instaladores. Prefijo RUNNER_TEMP/eio/toolchain-1.98.0; sin Rustup, sudo o PATH persistente. Los tamaños/sumas exactos figuran en COMPONENTES.tsv. Mismo origen para sumas/paquetes; no validación GPG independiente.
- Cargo generate-lockfile y metadata --locked completados; **sin build/test/run del banco, sin pesos o inferencia**. Cargo.lock recuperado contiene 171 paquetes. Metadata: 171 paquetes; versiones directas, transitivas, características resueltas y licencias declaradas conservadas.
- Supervisor: fase=preparacion retorno=0 causa=normal, residuales_al_wait=0 y residuales_tras_limpieza=0.

## Recuperación de evidencia

Los tres bloques base64 de Cargo.lock, DEPENDENCIAS.json y COMPONENTES.tsv se obtuvieron completos, se decodificaron a bytes y se contrastaron tamaño y SHA-256 con las cabeceras emitidas por el ejecutor. Todos coinciden; véase [manifiesto](MANIFIESTO_RECUPERACION.tsv). Cargo.lock se publica en la raíz del ensayo para construcción posterior con --locked. No fue escrito a mano, recortado ni vuelto a resolver localmente. El log sigue disponible aquí, no sólo en retención temporal de Actions.

## Recursos y límites

Inicial: 92418494464 bytes libres y 15423332KiB de MemAvailable. Supervisor muestreado cada 1 s: pico RSS simultáneo del grupo 139660KiB; máximo de disco observado 1523527680 bytes; mínimo libre 90894905344 bytes. Son medidas de preparación, no inferencia. No excedieron sus cotas observadas. Medidas de evidencia durante muestreo pueden anteceder al volcado de salida; el log recuperado y los productos textuales completos son inferiores a 20 MiB. No se añadieron cachés ni artefactos de Actions. El encabezado del runner menciona Cache mode:write como capacidad de plataforma; el workflow no invoca acciones de caché.

Cobertura: grupo de procesos propio, intervalos de1s, posible sobrepaso entre muestras y memoria compartida contada varias veces. No se afirma cota dura o supervisión exhaustiva. GitHub anunció uso de Node 24 para checkout fijado a una acción que declara Node 20; advertencia conservada, sin activar compatibilidad insegura. Runner Ubuntu 24.04.5, imagen 20260907.300.1, versión de agente 2.337.0. No se extrapola al PC, navegador, Windows o WASI.

## Paso experimental no iniciado

Para completar P03 se consultaron, mediante herramienta web, metadatos oficiales de Qwen/Qwen3-0.6B al commit c1899de289a04d12100db370d81485cdf75e47ca y Unsloth/Qwen3-0.6B-GGUF al commit f2d6f9ca53a254cc379437c49e4b2eb447f779df. Ambas consultas devolvieron Internal Error y «not accessible via this tool»; respuestas conservadas en [incidencia](INCIDENCIA_METADATOS.txt). No se determina si la causa es limitación de herramienta, acceso u otra. **No acredita indisponibilidad de Hugging Face ni una prohibición global.** No se ensayó otro canal ni se trasladaron esas consultas al runner para sortear el resultado.

Faltan identidad completa, tamaño y SHA-256 contrastable de tokenizador/configuración/plantilla, tamaño exacto del GGUF y aprovisionamiento experimental verificable por ejecutor. El SHA-256 publicado previamente del modelo sigue siendo antecedente, no cotejo de bytes. No se fabrica manifiesto ni se abre la guarda ensayo=false. La preparación lograda se conserva; la tercera ejecución no se consume hasta cumplir los prerrequisitos. Tampoco se declara corregida o probada la integración WASM/navegador. No compilación de banco, casos, coste de telemetría o calidad evaluados.

Resultado entregado para revisión receptora, sin aceptación científica ni modificación S32/BIS-03. No procesos propios persistentes locales iniciados; trabajo remoto terminado y supervisor informa cero residuales.
