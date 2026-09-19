#!/usr/bin/env bash
# Prueba del transporte Bash, con curl sustituido por una función sin red.
# No ejecuta Rust, el banco del SV, pesos ni inferencia.
set -euo pipefail
cd "$(dirname "$0")"
source ./adquirir.sh
RUNNER_TEMP=$(mktemp -d)
export RUNNER_TEMP
trap 'rm -rf -- "$RUNNER_TEMP"' EXIT
mkdir -p "$RUNNER_TEMP/eio"
sha=ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
curl() {
 local cabecera='' parte='' url='' n=0 primero=$1
 [[ "$primero" == --disable ]] || return 99
 while (( $# )); do
  case "$1" in
   -D) cabecera=$2; shift 2 ;;
   -o) parte=$2; shift 2 ;;
   --location|-L|--location-trusted) return 99 ;;
   *) url=$1; shift ;;
  esac
 done
 [[ -f "$RUNNER_TEMP/contador" ]] && read -r n <"$RUNNER_TEMP/contador"
 n=$((n+1)); printf '%s\n' "$n" >"$RUNNER_TEMP/contador"
 printf '%s\n' "$url" >>"$RUNNER_TEMP/solicitudes"
 : >"$cabecera"; : >"$parte"
 if [[ "$caso" == transporte ]]; then return 7; fi
 if [[ "$caso" == http403 ]]; then printf '403'; return 22; fi
 if [[ "$caso" == bucle || "$n" == 1 && -n "$location" ]]; then
  printf 'HTTP/1.1 302 Found\r\nLocation: %s\r\n\r\n' "$location" >"$cabecera"
  if [[ "$caso" == multiple ]]; then printf 'Location: https://otro.invalid/\r\n' >>"$cabecera"; fi
  printf '302'; return 0
 fi
 if [[ "$caso" == sin_location ]]; then printf '302'; return 0; fi
 printf 'abc' >"$parte"
 printf '200'
}
fallos=0; total=0
probar() {
 local caso=$1 url=$2 politica=$3 location=$4 esperado=$5 llamadas=$6 patron=$7
 local bytes=3 huella=$sha rc n destino="$RUNNER_TEMP/eio/entrada"
 rm -f -- "$destino" "$destino.headers" "$destino.part" "$RUNNER_TEMP/contador" "$RUNNER_TEMP/solicitudes"
 [[ "$caso" == tamano ]] && bytes=4
 [[ "$caso" == huella ]] && huella=0000000000000000000000000000000000000000000000000000000000000000
 if adquirir "$url" "$destino" "$bytes" "$huella" "$politica" >"$RUNNER_TEMP/salida" 2>&1; then rc=0; else rc=$?; fi
 n=0; [[ -f "$RUNNER_TEMP/contador" ]] && read -r n <"$RUNNER_TEMP/contador"
 total=$((total+1))
 local correcto=true
 [[ "$rc" == "$esperado" && "$n" == "$llamadas" ]] || correcto=false
 if [[ "$esperado" == 0 ]]; then [[ -f "$destino" && "$(cat "$destino")" == abc ]] || correcto=false
 else [[ ! -e "$destino" ]] || correcto=false; fi
 if [[ -n "$patron" ]] && ! grep -Fq -- "$patron" "$RUNNER_TEMP/salida"; then correcto=false; fi
 if grep -Eq 'SECRETO|Signature=' "$RUNNER_TEMP/salida"; then correcto=false; fi
 if [[ "$correcto" == true ]]; then printf 'CONFORME\t%s\tretorno=%s\tsolicitudes_simuladas=%s\n' "$caso" "$rc" "$n"
 else printf 'NO_CONFORME\t%s\tretorno=%s esperado=%s\tsolicitudes=%s esperadas=%s\n' "$caso" "$rc" "$esperado" "$n" "$llamadas"; fallos=$((fallos+1)); fi
}
probar rust https://static.rust-lang.org/a rust '' 0 1 IDENTIDAD
probar hf https://huggingface.co/a hf '' 0 1 IDENTIDAD
probar cdn1 https://cdn-lfs.huggingface.co/a hf '' 0 1 IDENTIDAD
probar cdn2 https://cdn-lfs.hf.co/a hf '' 0 1 IDENTIDAD
probar xet https://cas-bridge.xethub.hf.co/a hf '' 0 1 IDENTIDAD
probar redireccion https://huggingface.co/a hf 'https://cas-bridge.xethub.hf.co/a?Signature=SECRETO' 0 2 IDENTIDAD
probar rechazado https://huggingface.co/a hf 'https://destino.invalid/a?Signature=SECRETO' 65 1 'host=destino.invalid'
probar relativo https://huggingface.co/a hf '/b?Signature=SECRETO' 0 2 IDENTIDAD
probar relativo_autoridad https://huggingface.co/a hf '//cdn-lfs.hf.co/b?Signature=SECRETO' 0 2 IDENTIDAD
probar relativo_rechazado https://huggingface.co/a hf '//destino.invalid/b' 65 1 'host=destino.invalid'
probar http https://huggingface.co/a hf 'http://huggingface.co/b' 65 1 URL_NO_CANONICA
probar credenciales https://huggingface.co/a hf 'https://SECRETO@huggingface.co/b' 65 1 AUTORIDAD_NO_CANONICA
probar puerto https://huggingface.co/a hf 'https://huggingface.co:443/b' 65 1 AUTORIDAD_NO_CANONICA
probar control https://huggingface.co/a hf $'https://huggingface.co/b\tSECRETO' 65 1 URL_NO_CANONICA
probar control_inicial $'https://huggingface.co/b\nSECRETO' hf '' 65 0 URL_NO_CANONICA
probar multiple https://huggingface.co/a hf 'https://cdn-lfs.hf.co/b' 67 1 LOCATION_AUSENTE_O_MULTIPLE
probar sin_location https://huggingface.co/a hf '' 67 1 LOCATION_AUSENTE_O_MULTIPLE
probar bucle https://huggingface.co/a hf 'https://huggingface.co/a' 68 6 LIMITE_REDIRECCIONES
probar politica https://huggingface.co/a desconocida '' 64 0 POLITICA_NO_ADMITIDA
probar rust_fuera https://huggingface.co/a rust '' 65 0 HOST_FUERA_DE_LISTA
probar sufijo https://huggingface.co.destino.invalid/a hf '' 65 0 HOST_FUERA_DE_LISTA
probar tamano https://huggingface.co/a hf '' 66 1 ''
probar huella https://huggingface.co/a hf '' 66 1 ''
probar transporte https://huggingface.co/a hf '' 7 1 ERROR_TRANSPORTE
probar http403 https://huggingface.co/a hf '' 22 1 ERROR_TRANSPORTE
probar_diagnostico() (
 local nombre=$1 location=$2 autorizado=$3 esperado=$4 llamadas=$5 rc n
 export EIO_PRUEBA_LOCATION="$location" EIO_DIAGNOSTICO_AUTORIZADO="$autorizado" GITHUB_ACTIONS=true
 rm -f "$RUNNER_TEMP/contador"
 curl() {
  local cabecera='' parte=''
  while (( $# )); do
   case "$1" in
    -D) cabecera=$2; shift 2 ;;
    -o) parte=$2; shift 2 ;;
    --location|-L|--location-trusted) return 99 ;;
    *) shift ;;
   esac
  done
  printf '1\n' >>"$RUNNER_TEMP/contador"
  printf 'HTTP/1.1 302 Found\r\nLocation: %s\r\n\r\n' "$EIO_PRUEBA_LOCATION" >"$cabecera"
  : >"$parte"; printf '302'
 }
 export -f curl
 if env -u BASH_ENV bash --noprofile --norc ./diagnosticar-transporte.sh >"$RUNNER_TEMP/diag" 2>&1; then rc=0; else rc=$?; fi
 n=0; [[ -f "$RUNNER_TEMP/contador" ]] && n=$(wc -l <"$RUNNER_TEMP/contador")
 [[ "$rc" == "$esperado" && "$n" == "$llamadas" ]] || { printf 'NO_CONFORME\t%s\n' "$nombre"; exit 1; }
 ! grep -Eq 'SECRETO|Signature=' "$RUNNER_TEMP/diag" || exit 1
 printf 'CONFORME\t%s\tretorno=%s\tsolicitudes_simuladas=%s\n' "$nombre" "$rc" "$n"
)
for prueba in admitido rechazado cerrado; do
 total=$((total+1))
 case "$prueba" in
  admitido) location='https://cas-bridge.xethub.hf.co/a?Signature=SECRETO'; autorizacion=EIO-TR-01; rc=0; n=1 ;;
  rechazado) location='https://destino.invalid/a?Signature=SECRETO'; autorizacion=EIO-TR-01; rc=65; n=1 ;;
  cerrado) location='https://destino.invalid/a'; autorizacion=''; rc=64; n=0 ;;
 esac
 if ! probar_diagnostico "diagnostico_$prueba" "$location" "$autorizacion" "$rc" "$n"; then fallos=$((fallos+1)); fi
done
printf 'TOTAL=%s FALLOS=%s RED_REAL=0\n' "$total" "$fallos"
(( fallos == 0 ))
