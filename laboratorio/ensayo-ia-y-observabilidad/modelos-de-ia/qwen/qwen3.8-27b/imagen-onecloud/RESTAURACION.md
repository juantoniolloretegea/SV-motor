# Condiciones de recuperación

Documento de referencia; no autoriza una nueva instancia, inferencia ni operación de escritura sobre discos.

1. Seleccionar una sola imagen del [manifiesto](MANIFIESTO.json), descargarla de la entrega y comprobar tamaño y SHA-256. Los activos publicados son archivos completos; no se concatenan entre sí.
2. Obtener la clave correspondiente mediante la custodia autorizada, fuera de GitHub. Descifrar con OpenSSL AES-256-CBC, PBKDF2 y 600.000 iteraciones, tomando la clave desde un archivo protegido. No introducir su contenido en documentación ni argumentos visibles.
3. Para la imagen final, cotejar el SHA-256 del archivo comprimido y del disco descomprimido con el manifiesto. Las comprobaciones de integridad no sustituyen un ensayo de restauración.
4. Utilizar un entorno de rescate independiente del disco de destino. Verificar identidad, ausencia de datos ajenos y capacidad del disco antes de cualquier escritura. Formato previsto: x86_64, BIOS y disco lógico de 32 GiB. No se acredita compatibilidad UEFI ni importación directa desde el panel del proveedor.
5. Revisar particiones, GRUB y fstab: etiqueta inicial `sv-qwen38-root`; etiqueta final `sv-qwen38-cierre`. Adaptar red, nombre de equipo, identidad de máquina y nuevas claves de acceso y de host. Los accesos administrativos originales no se conservan.
6. Recuperar separadamente los pesos desde `Qwen/Qwen3.8-27B`, revisión `1d4bf0f2ff6012fd82039f2fa52739d0dd7c60c0`. Cotejarlos con el manifiesto original del modelo antes de usarlo. No sustituir la revisión por `main`. Los pesos se esperaban en `/srv/qwen-eval/model`; su volumen requiere ampliar el almacenamiento o un volumen adicional autorizado.
7. Revisar los servicios antes de cualquier arranque. La imagen conserva software experimental y expedientes; su finalidad es custodia, no despliegue automático. Mantener el motor apagado salvo nueva autorización.

La disponibilidad futura de los pesos externos y el arranque en otra infraestructura no están garantizados por este archivo. El entorno ensayado dispuso de 64 GB de RAM; no hay prueba equivalente en 32 GB. La recuperación no convierte la configuración retirada en candidata médica.
