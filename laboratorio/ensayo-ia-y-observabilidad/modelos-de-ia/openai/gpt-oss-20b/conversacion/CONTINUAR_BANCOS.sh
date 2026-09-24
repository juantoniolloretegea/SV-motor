#!/usr/bin/env bash
set -euo pipefail
base=/opt/sv-lab/conversacion-20260924
limite_anterior=$(date -u -d '2026-09-24 16:38:50' +%s)
while systemctl is-active --quiet sv-conversacion-contexto.service; do
  if [ "$(date -u +%s)" -ge "$limite_anterior" ]; then
    printf '%s\n' 'El contexto sigue activo al vencer su ventana. Se requiere conciliación.'
    exit 2
  fi
  sleep 5
done
if [ "$(systemctl show sv-conversacion-contexto.service -p ExecMainStatus --value)" != 0 ]; then
  printf '%s\n' 'El banco previo informó un fallo. No se inicia otro banco.'
  exit 3
fi
set +e
"$base/bin/ensayo-dialogo" "$base/evidencias/dialogo-02" "$limite_anterior"
dialogo_estado=$?
set -e
if [ "$dialogo_estado" != 0 ] && ! grep -q 'No queda plazo para otra petición' "$base/evidencias/dialogo-02/RESUMEN.json"; then
  printf '%s\n' 'Incidencia conversacional distinta del plazo; continuación suspendida.'
  exit 4
fi
inicio_comparacion=$(date -u +%s)
limite_comparacion=$((inicio_comparacion + 3600))
printf '{"inicio_unix":%s,"limite_unix":%s,"plazo_segundos":3600}\n' "$inicio_comparacion" "$limite_comparacion" > "$base/evidencias/VENTANA_COMPARACION.json"
"$base/bin/ensayo-documental" "$base/src/COMPARACION_DOCUMENTAL_ENTRADAS.json" "$base/evidencias/documental-01" "$limite_comparacion"
"$base/bin/ensayo-comparativo" "$base/src/COMPARACION_QWEN_ENTRADAS.json" "$base/evidencias/comparacion-01" "$limite_comparacion"
