#!/bin/sh
export PATH=/bin
mount -t devtmpfs devtmpfs /dev
mount -t proc proc /proc
mount -t sysfs sysfs /sys
exec >/dev/console 2>&1
set -eu
set -o pipefail
trap 'echo RESTAURACION_FALLIDA_CONSOLA; exec sh' EXIT
cat /proc/cmdline | grep -q 'sv_restore_instance=864367'
echo SV_RESTAURACION_864367_INICIO
for m in virtio_pci virtio_scsi virtio_blk sd_mod ahci ata_piix ext4; do modprobe "$m" || true; done
sleep 3
[ -b /dev/sda ]
[ "$(blockdev --getsize64 /dev/sda)" -ge 34359738368 ]
echo ESCRITURA_DISCO_INSTANCIA_DE_PRUEBA
 gzip -dc /payload/imagen.raw.gz | dd of=/dev/sda bs=4M
sync
blockdev --rereadpt /dev/sda
sleep 3
mount -t ext4 /dev/sda2 /newroot
cp -a /identity/network/. /newroot/etc/network/
cp -a /identity/ssh_host_* /newroot/etc/ssh/
cp -a /identity/shadow /newroot/etc/shadow
cp -a /identity/hosts /newroot/etc/hosts
mkdir -p /newroot/root/.ssh
cp -a /identity/authorized_keys /newroot/root/.ssh/authorized_keys
chmod 700 /newroot/root/.ssh
chmod 600 /newroot/root/.ssh/authorized_keys
cat /identity/resolv.conf > /newroot/etc/resolv.conf.sv-restauracion
rm -f /newroot/etc/resolv.conf
# Se conserva la configuracion DNS como archivo regular para el primer arranque.
cp /newroot/etc/resolv.conf.sv-restauracion /newroot/etc/resolv.conf
printf '%s\n' sv-restauracion-gptoss-prueba-20260925 > /newroot/etc/hostname
sed -i '/^[[:space:]]*linux /s/$/ net.ifnames=0 biosdevname=0/' /newroot/boot/grub/grub.cfg
mkdir -p /newroot/var/log
printf '%s\n' 'SV_RESTAURACION_864367_DISCO_COMPLETO_DESDE_GITHUB' 'Identidad SSH, red y credenciales propias de la instancia de prueba conservadas.' > /newroot/var/log/sv-restauracion-disco.log
sync
umount /newroot
trap - EXIT
echo SV_RESTAURACION_864367_FINALIZADA_REINICIO
reboot -f