#!/usr/bin/env bash
set -euo pipefail
export EIO_BASE="$PWD"
cd resultados/preparacion-navegador-02
root="$RUNNER_TEMP/eio"
mkdir -p "$root/evidencia"
export CARGO_HOME="$root/cargo" CARGO_TARGET_DIR="$root/target" CARGO_NET_RETRY=0 TMPDIR="$root/tmp"
export PATH="$root/toolchain-1.98.0/bin:$PATH"
export RUSTC="$root/toolchain-1.98.0/bin/rustc" RUSTDOC="$root/toolchain-1.98.0/bin/rustdoc"
trap 'rc=$?; printf "{\"retorno_campana\":%s,\"fin_unix\":%s}\n" "$rc" "$(date +%s)" > "$root/evidencia/campana.json"; python3 custodiar.py' EXIT
python3 verificar-preparacion.py
python3 supervisar.py adquisicion 300 bash aprovisionar.sh
python3 supervisar.py herramienta 600 bash herramienta.sh
python3 supervisar.py construccion 950 bash construir.sh
python3 supervisar.py enlace 30 bash enlazar.sh
python3 supervisar.py navegador 200 node controlador.mjs
