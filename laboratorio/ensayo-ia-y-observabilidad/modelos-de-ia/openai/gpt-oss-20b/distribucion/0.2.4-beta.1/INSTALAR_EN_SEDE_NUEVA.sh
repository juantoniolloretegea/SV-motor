#!/usr/bin/env bash
# Prepara una sede vacía. No descarga pesos ni inicia servicios.
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
origen=${1:?Indique el origen de acceso, por ejemplo http://127.0.0.1:3000}
pesos=${2:?Indique la ruta absoluta del GGUF ya descargado}
[[ $EUID -eq 0 ]] || { echo 'Se requiere administración de la sede de destino.' >&2; exit 1; }
[[ $origen =~ ^https?://[A-Za-z0-9.-]+(:[0-9]+)?$ ]] || { echo 'Origen no admitido; use esquema, anfitrión y puerto opcional, sin ruta.' >&2; exit 1; }
[[ $pesos = /* && -f $pesos ]] || { echo 'Se requiere un archivo de pesos con ruta absoluta.' >&2; exit 1; }
command -v systemctl >/dev/null
command -v systemd-analyze >/dev/null
command -v timeout >/dev/null
sha256sum -c SHA256SUMS
[[ $(stat -c %s "$pesos") = 12109566624 ]] || { echo 'Tamaño de pesos no conforme.' >&2; exit 1; }
[[ $(sha256sum "$pesos" | cut -d ' ' -f 1) = 27cd6c432c7672cb812a92f611cf3ba7bbc35928262bb1e1253ff4ee6ae35901 ]] || { echo 'Huella de pesos no conforme.' >&2; exit 1; }
for destino in /opt/sv-lab/conversacion-20260924 /opt/sv-lab/onecloud-20260924 /opt/sv-lab/optimizacion-20260924 /etc/systemd/system/sv-conversacion.service /etc/systemd/system/sv-conversacion-motor.service; do
  [[ ! -e $destino && ! -L $destino ]] || { echo "Destino existente; no se sobrescribe: $destino" >&2; exit 1; }
done
app=/opt/sv-lab/conversacion-20260924
rec=/opt/sv-lab/onecloud-20260924
motor=/opt/sv-lab/optimizacion-20260924
install -d -m 700 "$app/bin" "$app/modelos" "$app/datos" "$rec/gguf" "$rec/diagnostico-20260924/tokenizador" "$rec/harmony" "$motor/target/release" "$motor/instalacion"
install -m 755 bin/eio-conversacion "$app/bin/"
install -m 755 bin/mistralrs "$motor/target/release/"
cp -p -- "$pesos" "$rec/gguf/gpt-oss-20b-MXFP4.gguf"
cp -p modelo/tokenizador/* "$rec/diagnostico-20260924/tokenizador/"
cp -p modelo/harmony/* "$rec/harmony/"
ln -s "$rec/gguf/gpt-oss-20b-MXFP4.gguf" "$app/modelos/gpt-oss-20b-MXFP4.gguf"
ln -s "$rec/diagnostico-20260924/tokenizador/tokenizer.json" "$app/modelos/tokenizer.json"
ln -s "$rec/gguf" "$motor/instalacion/gguf"
ln -s "$rec/diagnostico-20260924" "$motor/instalacion/diagnostico-20260924"
ln -s "$rec/harmony" "$motor/instalacion/harmony"
sed "s|^Environment=EIO_ORIGIN=.*|Environment=EIO_ORIGIN=$origen|" servicios/sv-conversacion.service > /etc/systemd/system/sv-conversacion.service
install -m 644 servicios/sv-conversacion-motor.service /etc/systemd/system/sv-conversacion-motor.service
systemd-analyze verify /etc/systemd/system/sv-conversacion.service /etc/systemd/system/sv-conversacion-motor.service
systemctl daemon-reload
echo 'Sede preparada; servicios todavía sin iniciar. Consulte LEAME.md.'
