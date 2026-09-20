#!/usr/bin/env bash
set -euo pipefail
d=resultados/revision-05/continuacion-06
root="$RUNNER_TEMP/eio"
prefix="$root/toolchain-1.98.0"
jq -e '.adquisicion_habilitada == true and .ensayo_habilitado == true' pruebas/estado-acceso.json >/dev/null
# El aprovisionador original exige el lock EIO-05; el manifiesto original sigue activo aquí.
cmp -s Cargo.toml "$d/Cargo.original.toml"
bash pruebas/vigilar.sh aprovisionamiento 480 bash pruebas/aprovisionar-ensayo.sh
export RUSTC="$prefix/bin/rustc" RUSTDOC="$prefix/bin/rustdoc"
export PATH="$prefix/bin:$PATH" CARGO_NET_OFFLINE=true CARGO_NET_RETRY=0
bash pruebas/vigilar.sh dependencias-06 60 bash "$d/resolver-dependencias.sh"
sha256sum --check --strict "$root/evidencia/dependencias-06/FIJACION.sha256"
bash pruebas/vigilar.sh compilacion 600 cargo build --release --locked --offline --features inferencia -j 2
bash pruebas/vigilar.sh banco 30 "$root/target/release/banco"
bash pruebas/vigilar.sh inferencia-base-on 120 "$root/target/release/inferencia" "$root/entradas/Qwen3-0.6B-Q4_K_M.gguf" "$root/entradas/tokenizer.json" pruebas/peticion.txt on
bash pruebas/vigilar.sh inferencia-estructurada-on 120 "$root/target/release/inferencia" "$root/entradas/Qwen3-0.6B-Q4_K_M.gguf" "$root/entradas/tokenizer.json" "$d/peticion-estructurada.txt" on
# Un resultado contractual adverso (2) es evidencia conservada y no impide el brazo de compilación WASM.
rc_contrato=0
bash pruebas/vigilar.sh contrato-06 30 "$root/target/release/recepcion06" "$root/evidencia/inferencia-base-on.stdout" "$root/evidencia/inferencia-estructurada-on.stdout" || rc_contrato=$?
case "$rc_contrato" in 0|2) ;; *) exit "$rc_contrato";; esac
rc_wasm=0
bash pruebas/vigilar.sh compilacion-wasm 240 cargo check --locked --offline --target wasm32-unknown-unknown --features navegador --lib -j 2 || rc_wasm=$?
sha256sum --check --strict "$root/evidencia/dependencias-06/FIJACION.sha256"
printf 'EIO_06_RESUMEN contrato=%s wasm=%s navegador_ejecutado=false\n' "$rc_contrato" "$rc_wasm"
if (( rc_wasm != 0 )); then exit "$rc_wasm"; fi
exit "$rc_contrato"
