#!/usr/bin/env bash
set -euo pipefail
paquete=resultados/revision-06/json-01
export CARGO_NET_RETRY=0
case "${1:-principal}" in
 compilar)
  cargo build --release --locked --offline --no-default-features --bin banco -j 2
  cargo test --release --locked --offline --no-default-features --test eio_json_01 --no-run -j 2
  ;;
 regresion)
  "$CARGO_TARGET_DIR/release/banco"
  ;;
 json)
  cargo test --release --locked --offline --no-default-features --test eio_json_01 -j 2 -- --nocapture --test-threads=1
  ;;
 principal)
  jq -e '.adquisicion_habilitada == true and .ensayo_habilitado == true' pruebas/estado-acceso.json >/dev/null
  sha256sum --check --strict "$paquete/FUENTES.sha256"
  (cd "$paquete"; sha256sum --check --strict PAQUETE.sha256)
  test ! -e tests/eio_json_01.rs
  mkdir -p tests
  cp "$paquete/banco.rs" tests/eio_json_01.rs
  cmp --silent "$paquete/banco.rs" tests/eio_json_01.rs
  bash pruebas/vigilar.sh aprovisionamiento-json 600 bash "$paquete/aprovisionar-json.sh"
  export PATH="$RUNNER_TEMP/eio/toolchain-1.98.0/bin:$PATH"
  export RUSTC="$RUNNER_TEMP/eio/toolchain-1.98.0/bin/rustc"
  export RUSTDOC="$RUNNER_TEMP/eio/toolchain-1.98.0/bin/rustdoc"
  export CARGO_NET_OFFLINE=true
  bash pruebas/vigilar.sh compilacion-json 360 bash "$paquete/ensayar-json.sh" compilar
  bash pruebas/vigilar.sh regresion-json 30 bash "$paquete/ensayar-json.sh" regresion
  bash pruebas/vigilar.sh banco-json 30 bash "$paquete/ensayar-json.sh" json
  sha256sum --check --strict "$paquete/FUENTES.sha256"
  (cd "$paquete"; sha256sum --check --strict PAQUETE.sha256)
  cmp --silent "$paquete/banco.rs" tests/eio_json_01.rs
  echo 'EIO-JSON-01: procedimiento terminado; resultados pendientes de recepción.'
  ;;
 *) exit 64 ;;
esac
