#!/usr/bin/env bash
# Transporte HTTPS acotado. Archivo de funciones; no se adquiere nada al leerlo.
set -euo pipefail
adquirir() {
 local url=$1 destino=$2 bytes=$3 sha=$4 politica=$5 estado cabecera parte salto host
 [[ "$bytes" =~ ^[0-9]+$ && "$sha" =~ ^[0-9a-f]{64}$ ]] || return 64
 (( bytes > 0 && bytes <= 536870912 )) || return 64
 [[ ! -e "$destino" && "$destino" == "$RUNNER_TEMP/eio/"* ]] || return 64
 cabecera="$destino.headers"; parte="$destino.part"
 for salto in 0 1 2 3 4 5; do
  [[ "$url" == https://* && "$url" != *$'\r'* && "$url" != *$'\n'* ]] || return 65
  host="${url#https://}"; host="${host%%/*}"
  if [[ "$politica" == rust ]]; then
   [[ "$host" == static.rust-lang.org ]] || return 65
  else
   case "$host" in
    huggingface.co|cdn-lfs.huggingface.co|cdn-lfs.hf.co|cas-bridge.xethub.hf.co) ;;
    *) echo "DESTINO_NO_ADMITIDO"; return 65 ;;
   esac
  fi
  # No registrar query firmada de CDN ni suministrar credenciales/cookies.
  printf 'GET host=%s salto=%s destino=%s\n' "$host" "$salto" "$(basename "$destino")"
  estado=$(curl --silent --show-error --fail --proto '=https' --tlsv1.2 --connect-timeout 20 --max-time 180 --max-filesize "$bytes" -D "$cabecera" -o "$parte" -w '%{http_code}' "$url")
  printf 'HTTP=%s\n' "$estado"
  if [[ "$estado" == 200 ]]; then
   [[ "$(stat -c %s "$parte")" == "$bytes" ]] || return 66
   printf '%s  %s\n' "$sha" "$parte" | sha256sum --check --strict
   mv -- "$parte" "$destino"
   printf 'IDENTIDAD bytes=%s sha256=%s archivo=%s\n' "$bytes" "$sha" "$(basename "$destino")"
   return 0
  fi
  case "$estado" in 301|302|303|307|308) ;; *) return 67 ;; esac
  url=$(awk 'tolower($1)=="location:"{$1="";sub(/^ /,"");sub(/\r$/,"");print}' "$cabecera")
  [[ -n "$url" ]] || return 67
  if [[ "$url" == /* ]]; then url="https://$host$url"; fi
 done
 echo 'LIMITE_REDIRECCIONES'; return 68
}
