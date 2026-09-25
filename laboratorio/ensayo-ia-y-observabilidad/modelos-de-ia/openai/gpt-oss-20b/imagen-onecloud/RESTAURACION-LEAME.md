# Repetición del procedimiento de restitución

Esta es la secuencia de la prueba realizada, no una orden para sobrescribir el servidor original. `RESTAURACION-INIT-864367.sh` es el script exacto del entorno en RAM y está fijado a la instancia desechable 864367. No debe ejecutarse en una consola normal: requiere el initramfs preparado, el disco destino desmontado y la identificación de una instancia nueva expresamente autorizada.

## Preparación en la instancia nueva

- Ubuntu 26.04 x86_64, BIOS, kernel 7.0.0-14-generic. En la prueba se utilizaron 64 GB de RAM y el plan DED640G12C.
- Acceso por clave SSH propio, diferente identidad de máquina y red del destino. No reutilizar la dirección del original.
- Descargar el activo cifrado de la release, verificar su SHA-256 y descifrar mediante OpenSSL según README. Cotejar también el SHA-256 del archivo comprimido interior.
- Instalar, sólo para preparar el rescate de la instancia vacía: `busybox-static`, `kexec-tools`, `cpio`, `curl`, `gdisk`; `zstd` y `depmod` estaban disponibles.
- Verificar que `kexec_load_disabled` es 0 y que el modo de bloqueo del kernel permite kexec. No se deshabilitaron protecciones para forzar el ensayo.

## Contenido del initramfs utilizado

Se creó un directorio con `bin`, `dev`, `proc`, `sys`, `newroot`, `payload`, `identity` y `lib/modules`.

1. `bin/busybox`: copia del ejecutable estático. Enlaces a sus applets `sh`, `mount`, `umount`, `mkdir`, `cat`, `cp`, `chmod`, `sleep`, `modprobe`, `gzip`, `dd`, `sync`, `reboot`, `blockdev`, `grep`, `sed` y `rm`.
2. Copia de `/lib/modules/$(uname -r)`; módulos `.ko.zst` descomprimidos y dependencias reconstruidas mediante `depmod -b DIRECTORIO_INITRAMFS VERSION_KERNEL`.
3. `payload/imagen.raw.gz`: imagen comprimida ya verificada; no contiene su clave de descifrado.
4. `identity`: copia de la red, DNS, `/etc/hosts`, `/etc/shadow`, claves de host SSH y `authorized_keys` de la **instancia nueva**, para adaptar su identidad después de escribir el disco. Este directorio es privado y no se publica.
5. `/init`: el script archivado en esta carpeta. Dispositivos de consola y null creados con `mknod`.

Se empaquetó mediante `find . -print0 | cpio --null -o --format=newc | gzip -1`. Se verificó la sintaxis del script con `busybox sh -n` antes de cargarlo.

Se cargó el mismo kernel de la instancia inicial y ese initramfs con `kexec -l`, incluyendo `rdinit=/init` y el identificador explícito `sv_restore_instance=864367`. Se verificó `/sys/kernel/kexec_loaded = 1` antes de iniciar `systemctl kexec`.

Desde RAM, el script escribió el disco completo, leyó de nuevo la tabla de particiones, montó la raíz restaurada, adaptó la identidad del destino y reinició. El disco lógico de la imagen es de 32 GiB; en esta prueba no fue necesario ampliar su raíz al tamaño comercial de 640 GB.

## Recepción posterior

1. Comprobar por consola y SSH el nombre del destino, su IP, raíz con etiqueta `sv-gptoss-root` y `/var/log/sv-restauracion-disco.log`.
2. Cotejar las huellas de aplicativo, motor y tokenizador. Recuperar los pesos de la revisión fija y cotejar tamaño y SHA-256.
3. Arrancar `sv-conversacion.service`; el motor se inicia bajo demanda. Mantener ambos servicios en localhost y el acceso mediante un túnel privado.
4. Ejercitar contexto previo, petición, estado final, cancelación, exportación, parada real del motor y persistencia al reiniciar el aplicativo.
5. Con el aplicativo detenido, su comprobador de registro se ejecutó como `eio-conversacion --check /opt/sv-lab/conversacion-20260924/datos`; se obtuvo integridad conforme.
6. Conservar y verificar las evidencias antes de retirar la instancia desechable. La instancia original no forma parte de esa retirada.

Cambios de proveedor, arquitectura, firmware, kernel o política de acceso requieren nueva comprobación. La disponibilidad futura de recursos no se deduce del éxito de esta restitución.