#!/usr/bin/env bash
# Sólo ejecutor remoto autorizado. No se invoca rustup ni un instalador del PATH.
set -euo pipefail
jq -e '.adquisicion_habilitada == true and (.fundamento|type=="string" and length>0)' pruebas/estado-acceso.json >/dev/null
[[ "$RUNNER_ARCH" == X64 && "$FASE" == preparacion ]]
root="$RUNNER_TEMP/eio"
prefix="$root/toolchain-1.98.0"
test ! -e "$prefix"
mkdir -p "$root"/{cargo,tmp,evidencia,dist}
export CARGO_HOME="$root/cargo" TMPDIR="$root/tmp" CARGO_NET_RETRY=0
dist="$root/dist"
registro="$dist/COMPONENTES.tsv"
printf 'archivo\tbytes\tsha256\turl\n' > "$registro"
obtener() {
 local url=$1 salida=$2 max=$3 status
 printf 'GET %s (sin redirecciones ni reintentos)\n' "$url"
 status=$(curl --silent --show-error --fail --proto '=https' --tlsv1.2 --connect-timeout 20 --max-time 300 --max-filesize "$max" -w '%{http_code}' -o "$salida" "$url")
 [[ "$status" == 200 ]] || { echo "HTTP inesperado: $status"; return 70; }
}
for componente in rustc cargo rust-std; do
 nombre="$componente-1.98.0-x86_64-unknown-linux-gnu"
 url="https://static.rust-lang.org/dist/$nombre.tar.xz"
 obtener "$url.sha256" "$dist/$nombre.tar.xz.sha256" 1024
 obtener "$url" "$dist/$nombre.tar.xz" 536870912
 esperado=$(awk 'NR==1{print $1}' "$dist/$nombre.tar.xz.sha256")
 [[ "$esperado" =~ ^[0-9a-f]{64}$ ]]
 printf '%s  %s\n' "$esperado" "$dist/$nombre.tar.xz" | sha256sum --check --strict
 printf '%s\t%s\t%s\t%s\n' "$nombre.tar.xz" "$(stat -c %s "$dist/$nombre.tar.xz")" "$esperado" "$url" >> "$registro"
 # Sólo se extrae después del cotejo oficial; se conserva suma del instalador incluido.
 tar -xJf "$dist/$nombre.tar.xz" -C "$dist" --no-same-owner
 sha256sum "$dist/$nombre/install.sh"
 bash "$dist/$nombre/install.sh" --help
 bash "$dist/$nombre/install.sh" --prefix="$prefix" --disable-ldconfig
done
export RUSTC="$prefix/bin/rustc" RUSTDOC="$prefix/bin/rustdoc"
export PATH="$prefix/bin:$PATH"
"$RUSTC" -Vv
[[ "$("$RUSTC" --version)" == "rustc 1.98.0 "* ]]
"$prefix/bin/cargo" -Vv
"$prefix/bin/cargo" generate-lockfile
"$prefix/bin/cargo" metadata --locked --format-version 1 --features navegador > "$dist/metadata.json"
jq '{packages:[.packages[]|{name,version,source,license}],resolve:.resolve}' "$dist/metadata.json" > "$dist/DEPENDENCIAS.json"
# Paquetes descargados: huellas y tamaños de los archivos del registro, sin ejecución.
find "$CARGO_HOME/registry/cache" -type f -name '*.crate' -print0 |
 while IFS= read -r -d '' paquete; do
 printf '%s\t%s\t%s\tcrates.io\n' "$(basename "$paquete")" "$(stat -c %s "$paquete")" "$(sha256sum "$paquete" | cut -d ' ' -f1)" >> "$registro"
 done
# Evidencia textual íntegra; nunca reconstruir un lock truncado.
for dato in Cargo.lock "$dist/DEPENDENCIAS.json" "$registro"; do
 test "$(wc -c < "$dato")" -le 524288
 echo "EIO_ARCHIVO_BEGIN $(basename "$dato")"
 wc -c < "$dato"
 sha256sum "$dato"
 base64 -w 76 "$dato"
 echo "EIO_ARCHIVO_END $(basename "$dato")"
done
echo 'PREPARACION_COMPLETA: sin compilar, sin banco, sin pesos, sin inferencia'
