#!/usr/bin/env bash
# Candidato no ejecutado. Monitor de grupo de procesos propio, muestreo de 1 s.
set -euo pipefail
fase=$1; limite=$2; shift 2
root="$RUNNER_TEMP/eio"
mkdir -p "$root/evidencia"
salida="$root/evidencia/$fase"
printf 'orden:'; printf ' %q' "$@"; printf '\n'
setsid "$@" >"$salida.stdout" 2>"$salida.stderr" &
pid=$!
trap 'kill -TERM -- "-$pid" 2>/dev/null || true; sleep 2; kill -KILL -- "-$pid" 2>/dev/null || true; wait "$pid" 2>/dev/null || true' EXIT
sleep 0.1
grupo=$(ps -o pgid= -p "$pid" | tr -d ' ' || true)
if [[ -n "$grupo" && "$grupo" != "$pid" ]]; then echo "grupo no acreditado"; exit 90; fi
inicio=$SECONDS; pico=0; causa=normal
while kill -0 "$pid" 2>/dev/null; do
 estado=$(ps -o stat= -p "$pid" || true)
 [[ "$estado" == Z* ]] && break
 rss=$(ps -eo pgid=,rss= | awk -v p="$pid" '$1==p{s+=$2} END{print s+0}')
 if (( rss > pico )); then pico=$rss; fi
 bytes=$(du -s -B1 "$root" "$GITHUB_WORKSPACE" | awk '{s+=$1}END{print s+0}')
 libres=$(df -B1 --output=avail "$root" | tail -1 | tr -d ' ')
 evidencia=$(du -s -B1 "$root/evidencia" | cut -f1)
 printf '%s\t%s\t%s\t%s\t%s\n' "$SECONDS" "$rss" "$bytes" "$libres" "$evidencia" >>"$salida.medidas.tsv"
 if (( SECONDS-inicio >= limite )); then causa=tiempo; break; fi
 if (( bytes > 10737418240 || libres < 2147483648 || evidencia > 20971520 )); then causa=disco; break; fi
 if [[ "$fase" == inferencia* || "$fase" == navegador* ]] && (( rss > 4194304 )); then causa=memoria; break; fi
 sleep 1
done
if [[ "$causa" != normal ]]; then
 kill -TERM -- "-$pid" 2>/dev/null || true; sleep 2; kill -KILL -- "-$pid" 2>/dev/null || true
fi
set +e; wait "$pid"; rc=$?; set -e
residuales=$(ps -eo pgid= | awk -v p="$pid" '$1==p{n++}END{print n+0}')
if (( residuales > 0 )); then kill -TERM -- "-$pid" 2>/dev/null || true; sleep 2; kill -KILL -- "-$pid" 2>/dev/null || true; fi
residuales_finales=$(ps -eo pgid= | awk -v p="$pid" '$1==p{n++}END{print n+0}')
printf 'residuales_tras_limpieza=%s\n' "$residuales_finales"
printf 'fase=%s retorno=%s causa=%s pico_rss_kib=%s residuales_al_wait=%s\n' "$fase" "$rc" "$causa" "$pico" "$residuales"
for ext in stdout stderr medidas.tsv; do
 test -f "$salida.$ext" || touch "$salida.$ext"
 n=$(wc -c <"$salida.$ext"); if (( n > 1048576 )); then echo "evidencia excede transmisión; parada"; exit 91; fi
 printf '\n--- %s ---\n' "$ext"; cat "$salida.$ext"
done
[[ "$causa" == normal && "$residuales" == 0 && "$residuales_finales" == 0 ]] || exit 92
trap - EXIT
exit "$rc"
