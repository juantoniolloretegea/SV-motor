#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
sha256sum --check SHA256SUMS
sv_recepcion="$(mktemp -d "${PWD}/recepcion-XXXXXX")"
printf '%s\n' "$sv_recepcion" > ULTIMA_RECEPCION.txt
rustc --version > "$sv_recepcion/rustc.txt"
case "$(cat "$sv_recepcion/rustc.txt")" in
  'rustc 1.98.0 '*) ;;
  *) printf '%s\n' 'Se requiere Rust 1.98.0; recepción detenida.' >&2; exit 1 ;;
esac
export CARGO_TARGET_DIR="$sv_recepcion/target"
timeout --kill-after=10s 600s cargo test --locked --jobs 4 > "$sv_recepcion/pruebas.txt" 2>&1
timeout --kill-after=10s 600s cargo build --release --locked --jobs 4 > "$sv_recepcion/compilacion.txt" 2>&1
timeout --kill-after=5s 60s "$CARGO_TARGET_DIR/release/comprobar-mcp" \
  "$CARGO_TARGET_DIR/release/sv-mcp-documental" \
  fuentes/catalogo-pdq.json "$(cat fuentes/catalogo-pdq.sha256)" \
  "$sv_recepcion/sesion.jsonl" > "$sv_recepcion/resultado.json"
sha256sum "$CARGO_TARGET_DIR/release/sv-mcp-documental" > "$sv_recepcion/ejecutable.sha256"
cat "$sv_recepcion/resultado.json"
printf 'Recepción conservada en: %s\nParada: no se ha cargado ningún modelo.\n' "$sv_recepcion"
