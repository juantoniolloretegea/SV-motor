#!/usr/bin/env bash
set -euo pipefail
jq -e '.adquisicion_habilitada == true and (.fundamento|type=="string" and length>0)' pruebas/estado-acceso.json >/dev/null
test "$RUNNER_ARCH" = X64
mkdir -p "$RUNNER_TEMP/eio"/{cargo,rustup,tmp,evidencia,dist}
export CARGO_HOME="$RUNNER_TEMP/eio/cargo" RUSTUP_HOME="$RUNNER_TEMP/eio/rustup" TMPDIR="$RUNNER_TEMP/eio/tmp"
dist="$RUNNER_TEMP/eio/dist"
# Recurso bloqueado: estas órdenes NO se ejecutan en esta preparación documental.
curl --fail --proto '=https' --tlsv1.2 --max-time 30 https://static.rust-lang.org/dist/channel-rust-1.98.0.toml -o "$dist/channel-rust-1.98.0.toml"
curl --fail --proto '=https' --tlsv1.2 --max-time 30 https://static.rust-lang.org/dist/channel-rust-1.98.0.toml.sha256 -o "$dist/channel-rust-1.98.0.toml.sha256"
(cd "$dist" && sha256sum -c channel-rust-1.98.0.toml.sha256)
# El manifiesto debe identificar el ejecutable rustup real antes de invocarlo.
test -f pruebas/INSTALADOR_VERIFICADO.sha256
sha256sum -c pruebas/INSTALADOR_VERIFICADO.sha256
rustup toolchain install --help
rustup --version
rustup toolchain install 1.98.0-x86_64-unknown-linux-gnu --profile minimal --no-self-update
rustup run 1.98.0-x86_64-unknown-linux-gnu rustc -Vv
rustup run 1.98.0-x86_64-unknown-linux-gnu cargo -Vv
rustup run 1.98.0-x86_64-unknown-linux-gnu cargo generate-lockfile
test "$(wc -c < Cargo.lock)" -le 524288
sha256sum Cargo.lock
printf 'CARGO_LOCK_BASE64_BEGIN\n'
base64 -w 76 Cargo.lock
printf 'CARGO_LOCK_BASE64_END\n'
rustup run 1.98.0-x86_64-unknown-linux-gnu cargo metadata --locked --format-version 1 --features inferencia > "$dist/metadata.json"
jq '{packages:[.packages[]|{name,version,source,license}],resolve:.resolve}' "$dist/metadata.json" > "$dist/dependencias.json"
test "$(wc -c < "$dist/dependencias.json")" -le 262144
sha256sum "$dist/dependencias.json"
cat "$dist/dependencias.json"
