#!/usr/bin/env bash
set -euo pipefail
export CARGO_TARGET_DIR="$RUNNER_TEMP/eio/target-herramienta"
cargo install wasm-bindgen-cli --locked --version 0.2.104 --root "$RUNNER_TEMP/eio/bindgen"
"$RUNNER_TEMP/eio/bindgen/bin/wasm-bindgen" --version
