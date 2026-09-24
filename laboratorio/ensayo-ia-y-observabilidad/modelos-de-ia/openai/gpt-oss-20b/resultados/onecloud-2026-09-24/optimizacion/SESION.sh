#!/bin/bash
set -u
ensayo=/opt/sv-lab/optimizacion-20260924
caso=${1:?Identificador de sesión requerido}
modo=${2:?Modo requerido}
banco=${3:?Banco requerido}
export EIO_MXFP4_MODO="$modo"
export EIO_MXFP4_PERFIL=1
export EIO_BANCO="$banco"
export EIO_LIMITE_UTC_S=1790254680
export CANDLE_NUM_THREADS=12
export RAYON_NUM_THREADS=12
export EIO_MOTOR_SHA256
EIO_MOTOR_SHA256=$(cut -d ' ' -f 1 "$ensayo/evidencias/MOTOR_SHA256.txt")
grupo=$(awk -F: '$1==0 {print $3}' /proc/self/cgroup)
observar() {
  date -u --iso-8601=seconds
  for archivo in memory.max memory.current memory.peak memory.events memory.swap.max pids.max pids.current cgroup.procs; do
    printf '%s=' "$archivo"
    cat "/sys/fs/cgroup$grupo/$archivo"
  done
}
observar > "$ensayo/evidencias/${caso}_CONTENCION_INICIAL.txt" 2>&1
"$ensayo/bin/eio-controlador-oss-revision" --instalacion "$ensayo/instalacion" --evidencias "$ensayo/evidencias/$caso" --ventana-segundos 1500 --carga-segundos 180 --peticion-segundos 300 --limite-as-mib 32768 --reserva-entorno-mib 512
resultado=$?
observar > "$ensayo/evidencias/${caso}_CONTENCION_FINAL.txt" 2>&1
printf 'Retorno del controlador: %s\n' "$resultado"
exit "$resultado"
