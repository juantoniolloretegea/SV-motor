#!/usr/bin/env bash
# Preparación técnica acotada: sin inferencia y sin resolución en red.
set -euo pipefail
d=resultados/revision-05/continuacion-06
root="$RUNNER_TEMP/eio"
e="$root/evidencia/dependencias-06"
mkdir -p "$e"
cp Cargo.lock "$e/Cargo.anterior.lock"
cp Cargo.toml "$e/Cargo.anterior.toml"
cargo metadata --locked --offline --format-version 1 --features navegador > "$e/anterior.json"
cp "$d/Cargo.navegador.toml" Cargo.toml
# Se conserva el lock anterior como preferencia. No se usa cargo update ni generate-lockfile.
cargo metadata --offline --format-version 1 --features navegador > "$e/nueva.json"
for nombre in anterior nueva; do
 jq -S '[.packages[]|{name,version,source}]|sort_by(.name,.version,.source)' "$e/$nombre.json" > "$e/$nombre-identidades.json"
done
# Debe permanecer exactamente el mismo inventario: ni nuevas versiones ni nuevos paquetes.
if ! cmp -s "$e/anterior-identidades.json" "$e/nueva-identidades.json"; then
 diff -u "$e/anterior-identidades.json" "$e/nueva-identidades.json" || true
 echo 'DEPENDENCIAS_FUERA_DEL_DELTA_ADMITIDO'
 exit 66
fi
cargo metadata --locked --offline --format-version 1 --features navegador > /dev/null
cargo tree --locked --offline --target wasm32-unknown-unknown --features navegador -e features > "$e/arbol-wasm.txt"
cp Cargo.lock "$e/Cargo.lock"
cp Cargo.toml "$e/Cargo.toml"
# Congelación material antes de compilar y medir. Los demás guiones no pueden resolver de nuevo.
sha256sum Cargo.lock Cargo.toml inferencia/main.rs pruebas/lib.rs \
 "$d/peticion-estructurada.txt" "$d/recepcion.rs" > "$e/FIJACION.sha256"
for dato in "$e/Cargo.lock" "$e/Cargo.toml" "$e/nueva-identidades.json" "$e/arbol-wasm.txt" "$e/FIJACION.sha256"; do
 test "$(wc -c < "$dato")" -le 262144
 echo "EIO_ARCHIVO_BEGIN $(basename "$dato")"
 wc -c < "$dato"
 sha256sum "$dato"
 base64 -w 76 "$dato"
 echo "EIO_ARCHIVO_END $(basename "$dato")"
done
echo 'DEPENDENCIAS_06_FIJADAS: inventario conservado; pendientes compilación y ejecución'
