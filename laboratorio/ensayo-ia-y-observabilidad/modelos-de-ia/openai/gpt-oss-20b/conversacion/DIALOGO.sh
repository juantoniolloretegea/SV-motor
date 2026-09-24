#!/usr/bin/env bash
set -euo pipefail
limite=$(date -u -d '2026-09-24 16:38:50' +%s)
while systemctl is-active --quiet sv-conversacion-contexto.service; do
  if [ "$(date -u +%s)" -ge "$limite" ]; then exit 2; fi
  sleep 5
done
if [ "$(systemctl show sv-conversacion-contexto.service -p ExecMainStatus --value)" != 0 ]; then
  printf '%s\n' 'El banco de contexto no terminó correctamente; se requiere diagnóstico antes de continuar.' >&2
  exit 3
fi
exec /opt/sv-lab/conversacion-20260924/bin/ensayo-dialogo /opt/sv-lab/conversacion-20260924/evidencias/dialogo-01 "$limite"
