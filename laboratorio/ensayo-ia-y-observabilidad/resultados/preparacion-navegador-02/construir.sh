#!/usr/bin/env bash
set -euo pipefail
export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
cargo build --locked --release --target wasm32-unknown-unknown --features navegador --lib
