#!/usr/bin/env bash
# Transporte HTTPS acotado. Archivo de funciones; no se adquiere nada al leerlo.
set -euo pipefail
# Sólo se registran hosts canónicos; nunca autoridades con usuario, puerto o
# caracteres de control. EIO-TR-01 justifica el nodo concreto us.aws.cdn.hf.co.
validar_destino_eio() {
 local url=$1 politica=$2 salto=$3 autoridad
 EIO_HOST=no_disponible
 case "$politica" in rust|hf) ;; *) echo 'POLITICA_NO_ADMITIDA'; return 64 ;; esac
 if [[ "$url" != https://* || "$url" =~ [[:space:][:cntrl:]] || "$url" == *'#'* ]]; then
  printf 'DESTINO_NO_ADMITIDO salto=%s motivo=URL_NO_CANONICA host=no_disponible\n' "$salto"
  return 65
 fi
 autoridad="${url#https://}"; autoridad="${autoridad%%[/?#]*}"
 if [[ ! "$autoridad" =~ ^[a-z0-9]([a-z0-9.-]*[a-z0-9])?$ ]]; then
  printf 'DESTINO_NO_ADMITIDO salto=%s motivo=AUTORIDAD_NO_CANONICA host=no_disponible\n' "$salto"
  return 65
 fi
 EIO_HOST=$autoridad
 case "$politica:$EIO_HOST" in
  rust:static.rust-lang.org|hf:huggingface.co|hf:cdn-lfs.huggingface.co|hf:cdn-lfs.hf.co|hf:cas-bridge.xethub.hf.co|hf:us.aws.cdn.hf.co) return 0 ;;
 esac
 printf 'DESTINO_NO_ADMITIDO salto=%s motivo=HOST_FUERA_DE_LISTA host=%s politica=%s\n' "$salto" "$EIO_HOST" "$politica"
 return 65
}

# Resultado en EIO_REDIRECCION. No se escribe la URL en stdout/stderr.
leer_redireccion_eio() {
 local cabecera=$1 host=$2 n
 n=$(awk 'tolower($1)=="location:"{n++} END{print n+0}' "$cabecera") || return 67
 if [[ "$n" != 1 ]]; then
  printf 'REDIRECCION_NO_VALIDA motivo=LOCATION_AUSENTE_O_MULTIPLE\n'
  return 67
 fi
 EIO_REDIRECCION=$(awk 'tolower($1)=="location:"{sub(/^[^:]*:[ \t]*/,"");sub(/\r$/,"");print}' "$cabecera") || return 67
 [[ -n "$EIO_REDIRECCION" ]] || return 67
 case "$EIO_REDIRECCION" in
  //*) EIO_REDIRECCION="https:$EIO_REDIRECCION" ;;
  /*) EIO_REDIRECCION="https://$host$EIO_REDIRECCION" ;;
 esac
}

adquirir() {
 local url=$1 destino=$2 bytes=$3 sha=$4 politica=$5 estado cabecera parte salto host rc
 [[ "$bytes" =~ ^[0-9]+$ && "$sha" =~ ^[0-9a-f]{64}$ ]] || return 64
 (( bytes > 0 && bytes <= 536870912 )) || return 64
 [[ ! -e "$destino" && "$destino" == "$RUNNER_TEMP/eio/"* ]] || return 64
 cabecera="$destino.headers"; parte="$destino.part"
 for salto in 0 1 2 3 4 5; do
  validar_destino_eio "$url" "$politica" "$salto" || return $?
  host=$EIO_HOST
  # No registrar query firmada de CDN ni suministrar credenciales/cookies.
  printf 'GET host=%s salto=%s destino=%s\n' "$host" "$salto" "$(basename "$destino")"
  if estado=$(curl --disable --globoff --silent --fail --retry 0 --proto '=https' --tlsv1.2 --connect-timeout 20 --max-time 180 --max-filesize "$bytes" -D "$cabecera" -o "$parte" -w '%{http_code}' "$url"); then
   :
  else
   rc=$?
   printf 'ERROR_TRANSPORTE host=%s salto=%s codigo=%s\n' "$host" "$salto" "$rc"
   rm -f -- "$cabecera" "$parte"
   return "$rc"
  fi
  printf 'HTTP=%s\n' "$estado"
  if [[ "$estado" == 200 ]]; then
   [[ "$(stat -c %s "$parte")" == "$bytes" ]] || return 66
   printf '%s  %s\n' "$sha" "$parte" | sha256sum --check --strict || return 66
   rm -f -- "$cabecera"
   mv -- "$parte" "$destino" || return $?
   printf 'IDENTIDAD bytes=%s sha256=%s archivo=%s\n' "$bytes" "$sha" "$(basename "$destino")"
   return 0
  fi
  case "$estado" in 301|302|303|307|308) ;; *) return 67 ;; esac
  if leer_redireccion_eio "$cabecera" "$host"; then
   url=$EIO_REDIRECCION
   rm -f -- "$cabecera" "$parte"
  else
   rc=$?
   rm -f -- "$cabecera" "$parte"
   return "$rc"
  fi
 done
 echo 'LIMITE_REDIRECCIONES'; return 68
}


