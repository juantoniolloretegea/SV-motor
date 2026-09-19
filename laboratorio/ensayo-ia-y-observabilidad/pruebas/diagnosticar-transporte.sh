#!/usr/bin/env bash
# Diagnóstico de un único GET al origen. No sigue ninguna redirección.
set -euo pipefail
[[ "${EIO_DIAGNOSTICO_AUTORIZADO:-}" == EIO-TR-01 ]] || { echo 'DIAGNOSTICO_NO_AUTORIZADO'; exit 64; }
[[ "${GITHUB_ACTIONS:-}" == true && -n "${RUNNER_TEMP:-}" ]] || { echo 'EJECUTOR_NO_PREVISTO'; exit 64; }
cd "$(dirname "$0")"
source ./adquirir.sh
url=$(jq -er '.entradas[] | select(.archivo=="Qwen3-0.6B-Q4_K_M.gguf") | .url' ENTRADAS_ENSAYO.json)
# La petición inicial queda restringida al origen, no a un CDN nuevo.
validar_destino_eio "$url" hf 0
[[ "$EIO_HOST" == huggingface.co ]] || exit 65
host=$EIO_HOST
temporal=$(mktemp -d "$RUNNER_TEMP/eio-transporte.XXXXXX")
trap 'rm -rf -- "$temporal"' EXIT
# Límite por archivo del proceso, además del límite del cuerpo de curl.
ulimit -f 128
printf 'DIAGNOSTICO metodo=GET host=%s solicitudes_maximas=1 seguimiento=0\n' "$host"
rc=0
estado=$(curl --disable --globoff --silent --retry 0 --proto '=https' --tlsv1.2 --connect-timeout 10 --max-time 30 --max-filesize 65536 -D "$temporal/cabecera" -o "$temporal/cuerpo" -w '%{http_code}' "$url") || rc=$?
printf 'HTTP=%s retorno_transporte=%s\n' "$estado" "$rc"
(( rc == 0 )) || exit "$rc"
case "$estado" in 301|302|303|307|308) ;; *) echo 'SIN_REDIRECCION_EVALUABLE'; exit 67 ;; esac
leer_redireccion_eio "$temporal/cabecera" "$host"
if validar_destino_eio "$EIO_REDIRECCION" hf 1; then
 printf 'REDIRECCION_OBSERVADA host=%s admitido_por_lista=true seguimiento=0\n' "$EIO_HOST"
else
 rc=$?
 printf 'DIAGNOSTICO_CON_RECHAZO codigo=%s seguimiento=0\n' "$rc"
 exit "$rc"
fi

