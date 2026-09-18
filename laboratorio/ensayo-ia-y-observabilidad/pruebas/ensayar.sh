#!/usr/bin/env bash
set -euo pipefail
jq -e '.adquisicion_habilitada and .ensayo_habilitado' pruebas/estado-acceso.json >/dev/null
jq -e '.referencia.plantilla_cotejada == true and
 (.referencia.sha256|type=="string") and
 ([.rust[],.entradas[]]|all(.bytes>0 and (.sha256|test("^[0-9a-f]{64}$"))))' pruebas/ENTRADAS_ENSAYO.json >/dev/null
root="$RUNNER_TEMP/eio"; prefix="$root/toolchain-1.98.0"
bash pruebas/vigilar.sh aprovisionamiento 480 bash pruebas/aprovisionar-ensayo.sh
export RUSTC="$prefix/bin/rustc" RUSTDOC="$prefix/bin/rustdoc"
export PATH="$prefix/bin:$PATH" CARGO_NET_OFFLINE=true CARGO_NET_RETRY=0
bash pruebas/vigilar.sh compilacion 600 "$prefix/bin/cargo" build --release --locked --offline --features inferencia -j 2
bash pruebas/vigilar.sh banco 30 "$root/target/release/banco"
i=0
for modo in on on off off on on off; do
 i=$((i+1))
 bash pruebas/vigilar.sh "inferencia-$i-$modo" 120 "$root/target/release/inferencia" "$root/entradas/Qwen3-0.6B-Q4_K_M.gguf" "$root/entradas/tokenizer.json" pruebas/peticion.txt "$modo"
done
bash pruebas/vigilar.sh costes 15 "$root/target/release/costes" "$root/evidencia"
bash pruebas/vigilar.sh compilacion-wasm 240 "$prefix/bin/cargo" check --locked --offline --target wasm32-unknown-unknown --features navegador --lib -j 2
