#!/usr/bin/env bash
set -euo pipefail
cd /opt/sv-lab/conversacion-20260924
sv_inicio=$(date +%s)
sv_limite=$((sv_inicio + 7200))
test ! -e evidencias/VENTANA_COMPARACION_02.json
test ! -e evidencias/comparacion-02
printf '{"inicio_unix":%s,"limite_unix":%s,"maximo_segundos":7200,"segundos_por_peticion":900,"motivo":"Continuación autorizada tras interrupción y corrección de exportación; no modifica ventanas anteriores"}\n' "$sv_inicio" "$sv_limite" > evidencias/VENTANA_COMPARACION_02.json
exec bin/ensayo-comparativo COMPARACION_QWEN_ENTRADAS.json evidencias/comparacion-02 "$sv_limite" evidencias/comparacion-01/CONSERVACION-0.json 900
