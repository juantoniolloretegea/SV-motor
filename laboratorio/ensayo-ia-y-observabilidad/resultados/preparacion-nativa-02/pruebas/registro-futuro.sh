#!/usr/bin/env bash
# No ejecutado. Usar sólo con autorización posterior; no activa ni instala nada.
# LOG debe ser directorio NUEVO y absoluto dentro de la sede futura de pruebas.
set -u
if [[ $# -lt 3 || "$1" != /* || -e "$1" ]]; then
  printf '%s\n' 'uso: registro-futuro.sh LOG_NUEVO etiqueta orden [argumentos...]' >&2
  exit 64
fi
log=$1
etiqueta=$2
shift 2
umask 077
mkdir -- "$log" || exit 73
{
  printf 'inicio_UTC='
  date -u +'%Y-%m-%dT%H:%M:%SZ'
  printf 'orden='
  printf '%q ' "$@"
  printf '\n'
} >"$log/ORDEN.txt"
"$@" >"$log/stdout.txt" 2>"$log/stderr.txt"
rc=$?
printf '%s\n' "$rc" >"$log/retorno.txt"
date -u +'%Y-%m-%dT%H:%M:%SZ' >"$log/FIN_UTC.txt"
exit "$rc"
