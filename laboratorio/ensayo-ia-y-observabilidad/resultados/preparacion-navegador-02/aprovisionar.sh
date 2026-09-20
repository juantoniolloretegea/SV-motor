#!/usr/bin/env bash
set -euo pipefail
source "$EIO_BASE/pruebas/adquirir.sh"
m="$EIO_BASE/pruebas/ENTRADAS_ENSAYO.json"
# Precondiciones comprobables antes de red. Los null deliberados impiden el lanzamiento.
jq -e '.referencia.plantilla_cotejada == true and
 (.referencia.sha256|type=="string") and
 ([.rust[],.entradas[]]|all(.bytes>0 and (.sha256|test("^[0-9a-f]{64}$"))))' "$m" >/dev/null
root="$RUNNER_TEMP/eio"
prefix="$root/toolchain-1.98.0"
test ! -e "$prefix"
mkdir -p "$root"/{dist,entradas,cargo,tmp}
export CARGO_HOME="$root/cargo" TMPDIR="$root/tmp" CARGO_NET_RETRY=0
jq -r '.rust[]|[.archivo,.bytes,.sha256,.url]|@tsv' "$m" |
while IFS=$'\t' read -r archivo bytes sha url; do
 adquirir "$url" "$root/dist/$archivo" "$bytes" "$sha" rust
 tar -xJf "$root/dist/$archivo" -C "$root/dist" --no-same-owner
 carpeta="${archivo%.tar.xz}"
 sha256sum "$root/dist/$carpeta/install.sh"
 bash "$root/dist/$carpeta/install.sh" --prefix="$prefix" --disable-ldconfig
done
export RUSTC="$prefix/bin/rustc" RUSTDOC="$prefix/bin/rustdoc"
export PATH="$prefix/bin:$PATH"
"$RUSTC" -Vv
[[ "$("$RUSTC" --version)" == "rustc 1.98.0 "* ]]
"$prefix/bin/cargo" -Vv
test -d "$prefix/lib/rustlib/wasm32-unknown-unknown/lib"
# Lock recuperado: no resolución nueva ni actualización.
printf '%s  Cargo.lock\n' 1a3015a1ea23fbed270d2771e445e2e19e2b1e1d79efe2059a78c42a367e06e9 | sha256sum --check --strict
"$prefix/bin/cargo" fetch --locked
jq -r '.entradas[]|[.archivo,.bytes,.sha256,.url]|@tsv' "$m" |
while IFS=$'\t' read -r archivo bytes sha url; do
 adquirir "$url" "$root/entradas/$archivo" "$bytes" "$sha" hf
done
