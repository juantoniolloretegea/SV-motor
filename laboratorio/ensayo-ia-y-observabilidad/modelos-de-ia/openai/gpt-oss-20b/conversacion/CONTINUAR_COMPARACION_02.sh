#!/usr/bin/env bash
set -euo pipefail
cd /opt/sv-lab/conversacion-20260924
test -r src/COMPARACION_QWEN_ENTRADAS.json
test -r evidencias/comparacion-01/CONSERVACION-0.json
test ! -e evidencias/comparacion-02
if test -e evidencias/VENTANA_COMPARACION_02.json; then
  sv_limite=$(sed -n 's/.*"limite_unix":\([0-9]*\).*/\1/p' evidencias/VENTANA_COMPARACION_02.json)
  [[ "$sv_limite" =~ ^[0-9]+$ ]]
  test "$sv_limite" -gt "$(date +%s)"
else
sv_inicio=$(date +%s)
sv_limite=$((sv_inicio + 7200))
printf '{"inicio_unix":%s,"limite_unix":%s,"maximo_segundos":7200,"segundos_por_peticion":900,"motivo":"Continuación autorizada tras interrupción y corrección de exportación; no modifica ventanas anteriores"}\n' "$sv_inicio" "$sv_limite" > evidencias/VENTANA_COMPARACION_02.json
fi
exec bin/ensayo-comparativo src/COMPARACION_QWEN_ENTRADAS.json evidencias/comparacion-02 "$sv_limite" evidencias/comparacion-01/CONSERVACION-0.json 900
