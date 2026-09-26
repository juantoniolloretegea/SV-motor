# Imagen de recuperación de GPT-OSS en OneCloud

## Cierre del 26/09/2026

La entrega actual de archivo y su estado de retirada se documentan en [cierre-20260926](cierre-20260926/README.md). Incluye auditoría del día 26 y pesos cifrados separados. La nueva imagen no se ha restaurado por instrucción de la Dirección. El texto que sigue conserva como antecedente la entrega del día 25.

---

# Imagen de recuperación de GPT-OSS en OneCloud

Fecha de captura: 25 de septiembre de 2026. Preparación: Profesor, por autorización expresa del Director.

## Estado de recepción

Copia de recuperación cifrada y reducida del sistema existente. No es una reproducción integral del disco original de 640 GB. El sistema de archivos supera `e2fsck -fn`; las huellas de los dos ejecutables coinciden con las del servidor inspeccionado. La verificación de descifrado reproduce exactamente la huella del archivo comprimido.

**Arranque y restitución en una instancia nueva de OneCloud: comprobados. Inferencia, cancelación, integridad y persistencia: comprobadas en el ensayo acotado. Véase [ACTA-RESTITUCION.md](ACTA-RESTITUCION.md). La instancia original se conserva.**

## Entrega

El archivo binario se conserva como activo de una release, enlazado desde esta carpeta, para evitar incorporarlo al historial Git:

[Descargar imagen cifrada](https://github.com/juantoniolloretegea/SV-motor/releases/download/gpt-oss-imagen-onecloud-20260925-v1/gpt-oss-sistema.raw.gz.enc)

[Release de la imagen](https://github.com/juantoniolloretegea/SV-motor/releases/tag/gpt-oss-imagen-onecloud-20260925-v1)

[Entrega experimental previa del aplicativo](https://github.com/juantoniolloretegea/SV-motor/releases/tag/gpt-oss-conversacion-v0.2.4-beta.1)

| Magnitud | Valor |
|---|---:|
| Disco original | 640 GB nominales |
| Disco reconstruido, tamaño lógico | 34 359 738 368 bytes (32 GiB) |
| Árbol copiado, según `du -sh` | 2,3 GiB |
| Imagen comprimida | 1 206 832 479 bytes |
| Imagen comprimida y cifrada | 1 206 832 496 bytes |

32 GiB describe el **disco**, no la RAM necesaria para inferencia.

## Contenido y exclusiones

Se conservan Ubuntu 26.04, kernel 7.0.0-14-generic, bibliotecas del sistema, unidades de servicio, configuraciones, tokenizador, elementos de Harmony, los ejecutables instalados y datos/evidencias de la aplicación bajo `/opt/sv-lab`. Se mantienen sus rutas absolutas y enlaces simbólicos.

Se excluyen los pesos GGUF, las cachés y herramientas de compilación de Cargo, el árbol `target` salvo el ejecutable de inferencia reinstalado expresamente en su ruta, las listas de APT, el estado de cloud-init, `/root`, `/home`, los registros generales del sistema y los sistemas de archivos virtuales. Los datos históricos de la aplicación sí permanecen en el contenedor cifrado.

La copia contiene configuraciones privadas y la base local de cuentas; por eso **no debe publicarse ni compartirse descifrada**. Se han retirado de la copia las claves de identidad SSH del servidor y el identificador de máquina. El cifrado usa OpenSSL AES-256-CBC, PBKDF2, 600 000 iteraciones y sal aleatoria. La clave aleatoria se custodia aparte, fuera de GitHub. CBC no aporta autenticación integrada: hay que cotejar el SHA-256 publicado antes del descifrado y proteger la integridad del manifiesto y de la clave.

## Cambios efectuados exclusivamente en la copia

- Tabla GPT nueva, partición BIOS de 1 MiB y raíz ext4 con etiqueta `sv-gptoss-root`.
- GRUB para BIOS x86_64 y configuración de raíz por etiqueta; no se ha preparado arranque UEFI.
- `fstab` y nombre de máquina de recuperación nuevos.
- Inicio automático de `sv-conversacion` y su motor deshabilitado. Las unidades se conservan para revisión y arranque manual posterior.
- Identidad de máquina vaciada y claves de host SSH retiradas.

No se ha sustituido el disco original ni modificado su aplicación. Al inspeccionarlo, el servicio de conversación estaba en estado `failed` y el motor `inactive`; no se realizó inferencia durante esta operación.

## Recuperación de los pesos

Modelo: `ggml-org/gpt-oss-20b-GGUF`, archivo `gpt-oss-20b-MXFP4.gguf`, revisión fija `b97cbb20d1995efd41dce8c4dd1ddf86e8db375b`.

[Descarga de la revisión fijada](https://huggingface.co/ggml-org/gpt-oss-20b-GGUF/resolve/b97cbb20d1995efd41dce8c4dd1ddf86e8db375b/gpt-oss-20b-MXFP4.gguf)

Tamaño esperado: 12 109 566 624 bytes.

SHA-256: `27cd6c432c7672cb812a92f611cf3ba7bbc35928262bb1e1253ff4ee6ae35901`.

Destino dentro del sistema recuperado: `/opt/sv-lab/onecloud-20260924/gguf/gpt-oss-20b-MXFP4.gguf`.

La descarga externa y su disponibilidad futura son dependencias. Para independencia del proveedor de pesos será necesaria una copia separada de ese archivo; no está incluida en esta entrega compacta.

## Procedimiento de recuperación y límites

1. Descargar el activo cifrado y verificar el SHA-256 de `MANIFIESTO.json`.
2. En un entorno privado con espacio suficiente, descifrar con la clave custodiada por el Director y descomprimir. La imagen expandida ocupa lógicamente 32 GiB; una extracción dispersa puede reducir la ocupación física.
3. Ensayar primero en una máquina virtual x86_64 con BIOS y red deshabilitada. No sobrescribir ningún disco existente para esta comprobación.
4. Antes de conectarla a una red, renovar credenciales locales, regenerar identidad SSH (`ssh-keygen -A`), preparar una clave pública de acceso propia, revisar red, cortafuegos y servicios del sistema. La imagen no proporciona acceso SSH listo para usar.
5. Recuperar y verificar los pesos en la ruta indicada. Revisar origen HTTP y mecanismo de acceso privado: la URL antigua de Codespaces no constituye un acceso portable.
6. Iniciar los servicios de forma controlada y contrastar una conversación, cancelación, persistencia y consumo con el expediente original.

Comandos de descifrado para un equipo de recuperación Linux con OpenSSL y gzip, en un directorio privado:

```sh
openssl enc -d -aes-256-cbc -pbkdf2 -iter 600000 \
  -in gpt-oss-sistema.raw.gz.enc -out gpt-oss-sistema.raw.gz \
  -pass file:/ruta/privada/CLAVE-IMAGEN-20260925.txt
sha256sum gpt-oss-sistema.raw.gz
gzip -dc gpt-oss-sistema.raw.gz | dd of=gpt-oss-sistema.raw bs=4M conv=sparse status=progress
```

La importación requiere que el proveedor admita un disco RAW/convertido o un mecanismo de restauración de disco. **Se ha restituido el disco en una instancia nueva de OneCloud mediante un entorno de restauración en RAM y kexec. No se ha acreditado importación directa con el panel de instantáneas.** El procedimiento y sus adaptaciones de identidad se describen en el acta.

No se certifica aptitud clínica, integración del núcleo SV, contención general ni reproducción bit a bit del disco de origen. La portabilidad operativa se ha comprobado para la configuración y el procedimiento del acta; no se generaliza a otros entornos.