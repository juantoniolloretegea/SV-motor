#!/usr/bin/env bash
set -euo pipefail
source pruebas/adquirir.sh
root="$RUNNER_TEMP/eio"
prefix="$root/toolchain-1.98.0"
m=pruebas/ENTRADAS_ENSAYO.json
test ! -e "$prefix"
mkdir -p "$root"/{dist,cargo,tmp}
export CARGO_HOME="$root/cargo" TMPDIR="$root/tmp" CARGO_NET_RETRY=0
# Se reutilizan únicamente las tres distribuciones nativas ya fijadas.
jq -e '[.rust[] | select(.archivo | endswith("x86_64-unknown-linux-gnu.tar.xz"))] | length == 3' "$m" >/dev/null
jq -r '.rust[] | select(.archivo | endswith("x86_64-unknown-linux-gnu.tar.xz")) | [.archivo,.bytes,.sha256,.url] | @tsv' "$m" |
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
"$prefix/bin/cargo" -Vv
[[ "$("$RUSTC" --version)" == "rustc 1.98.0 "* ]]
[[ "$("$prefix/bin/cargo" --version)" == "cargo 1.98.0 "* ]]
printf '%s  Cargo.lock\n' fcbe8edec58cc8eded54d40b6852b12061962d521fb60308b106506a4a693f63 | sha256sum --check --strict
# Cargo puede recuperar fuentes de dependencias opcionales del lock existente.
# No se descargan pesos, tokenizador ni otra biblioteca; no se habilita inferencia.
"$prefix/bin/cargo" fetch --locked
printf '%s  Cargo.lock\n' fcbe8edec58cc8eded54d40b6852b12061962d521fb60308b106506a4a693f63 | sha256sum --check --strict
