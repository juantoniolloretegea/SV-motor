#!/usr/bin/env bash
set -euo pipefail
jq -e '.adquisicion_habilitada and .ensayo_habilitado' pruebas/estado-acceso.json >/dev/null
test -f Cargo.lock
test -f pruebas/COMPONENTES_VERIFICADOS.sha256
# Descarga, instalación repetida y redirecciones: pendientes de fijación, no improvisar.
test -f descargas/Qwen3-0.6B-Q4_K_M.gguf
test -f descargas/tokenizer.json
sha256sum -c pruebas/COMPONENTES_VERIFICADOS.sha256
bash pruebas/vigilar.sh compilacion 1200 rustup run 1.98.0-x86_64-unknown-linux-gnu cargo build --release --locked --features inferencia -j 2
bash pruebas/vigilar.sh banco 120 "$RUNNER_TEMP/eio/target/release/banco"
i=0
# Calentamiento on y tres pares: on/off, off/on, on/off. Siete inferencias.
for modo in on on off off on on off; do
 i=$((i+1))
 bash pruebas/vigilar.sh "inferencia-$i-$modo" 120 "$RUNNER_TEMP/eio/target/release/inferencia" descargas/Qwen3-0.6B-Q4_K_M.gguf descargas/tokenizer.json pruebas/peticion.txt "$modo"
done
bash pruebas/vigilar.sh costes 30 "$RUNNER_TEMP/eio/target/release/costes" "$RUNNER_TEMP/eio/evidencia"
bash pruebas/vigilar.sh compilacion-wasm 600 rustup run 1.98.0-x86_64-unknown-linux-gnu cargo check --locked --target wasm32-unknown-unknown --features navegador --lib -j 2
