#!/usr/bin/env bash
set -euo pipefail
root="$RUNNER_TEMP/eio"
mkdir -p "$root/web/pkg"
"$root/bindgen/bin/wasm-bindgen" "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/eio_candidato.wasm" --target web --out-dir "$root/web/pkg" --no-typescript
cp web/* "$root/web/"
cp pruebas/peticion.txt "$root/web/peticion.txt"
# Enlaces a entradas inmutables comprobadas: no duplicar pesos en disco.
ln -s "$root/entradas/Qwen3-0.6B-Q4_K_M.gguf" "$root/web/Qwen3-0.6B-Q4_K_M.gguf"
ln -s "$root/entradas/tokenizer.json" "$root/web/tokenizer.json"
python3 inventariar.py
