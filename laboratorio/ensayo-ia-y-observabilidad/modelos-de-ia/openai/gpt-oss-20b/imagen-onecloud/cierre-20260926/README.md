# Archivo de cierre GPT-OSS — 26/09/2026

Esta entrega conserva la configuración experimental retirada de la función médica prevista, sus datos y la auditoría del 26/09. No acredita aptitud clínica. El motor original es Rust/mistral.rs/Candle; llama.cpp permanece identificado como comparador de auditoría.

[Release de cierre](https://github.com/juantoniolloretegea/SV-motor/releases/tag/gpt-oss-archivo-cierre-20260926-v1) · [Dictamen](DICTAMEN-CIERRE.md) · [Informe de auditoría](auditoria/INFORME-PROFESOR.md) · [Manifiesto](MANIFIESTO.json).

## Qué se conserva

- Imagen de disco RAW de 32 GiB lógicos, comprimida y cifrada: sistema, ejecutables, bibliotecas, unidades, fuentes de SV, configuración, tokenizador, Harmony, conversaciones y evidencias.
- Pesos exactos gpt-oss-20b-MXFP4.gguf, sin entrenamiento ni modificación, en archivo cifrado separado dividido en partes ordenadas. La imagen mantiene las rutas y enlaces previstos para esos pesos.
- Datos de la aplicación con integridad conforme: 16 expedientes, 44 conversaciones y 236 sucesos en el cierre.
- Informe del Profesor y resultados de la auditoría. Los datos privados de la aplicación permanecen dentro de la imagen cifrada, no publicados en claro.

No es una copia bit a bit del disco comercial de 640 GB. Se excluyen directorios virtuales, temporales, cachés, registros generales y herramientas/artefactos de compilación prescindibles; véase EXCLUSIONES.txt. El ejecutable mistralrs se conserva expresamente en su ruta original aunque se excluya el resto de target. Los pesos quedan fuera del disco compacto, pero se conservan en esta misma release.

En la copia se renuevan la identificación del sistema, fstab y configuración de arranque BIOS; se retiran las claves de identidad SSH y el directorio SSH de root. Los servicios de SV no se habilitan para arranque automático. Una restauración requiere preparar credenciales y red del destino antes de exponerlo.

## Verificación y límite

Se cotejan los datos copiados y ejecutables, se comprueba ext4 sin reparación, se verifica el descifrado y se cotejan las huellas de los activos publicados. **Esta imagen del día 26 no se ha arrancado ni restituido en otra instancia por instrucción expresa del Director.** La restitución real del día 25 acredita un antecedente del procedimiento, no una restauración de los nuevos bytes.

La imagen del día 25 y su acta permanecen disponibles como antecedentes. No se sobrescriben ni se mezclan sus resultados con los de esta entrega. Véase el estado de publicación y retirada en RECEPCION.md.

## Recuperación de archivos

La clave CLAVE-CIERRE-GPTOSS-20260926.txt se custodia fuera de GitHub en la carpeta fijada por el Director. Es una clave simétrica, no un par de claves pública/privada. No debe publicarse ni incorporarse a la propia imagen.

Descargar todos los activos y cotejar ARCHIVOS.sha256 antes de descifrar. AES-256-CBC no incorpora autenticación: la integridad exige cotejar las huellas con este manifiesto versionado. OpenSSL usa PBKDF2 con 600 000 iteraciones y sal aleatoria.

En un entorno privado Linux, dentro del directorio de descargas:

```sh
sha256sum -c ARCHIVOS.sha256
openssl enc -d -aes-256-cbc -pbkdf2 -iter 600000 \
  -in gpt-oss-sistema-20260926.raw.gz.enc \
  -out gpt-oss-sistema-20260926.raw.gz \
  -pass file:/ruta/privada/CLAVE-CIERRE-GPTOSS-20260926.txt
gzip -dc gpt-oss-sistema-20260926.raw.gz | dd of=gpt-oss-sistema-20260926.raw bs=4M conv=sparse status=progress
cat pesos-mxfp4.gguf.enc.part-* > pesos-mxfp4.gguf.enc
openssl enc -d -aes-256-cbc -pbkdf2 -iter 600000 \
  -in pesos-mxfp4.gguf.enc -out gpt-oss-20b-MXFP4.gguf \
  -pass file:/ruta/privada/CLAVE-CIERRE-GPTOSS-20260926.txt
```

Cotejar las huellas del RAW, comprimido, cifrado concatenado y GGUF con MANIFIESTO.json. Los sufijos numéricos de las partes fijan su orden. No escribir el RAW sobre ningún disco con datos: el procedimiento anterior de restitución desde RAM exige una instancia de destino vacía, identificada y autorizada.

Ruta final de los pesos dentro del sistema recuperado: /opt/sv-lab/onecloud-20260924/gguf/gpt-oss-20b-MXFP4.gguf. SHA-256 esperado: 27cd6c432c7672cb812a92f611cf3ba7bbc35928262bb1e1253ff4ee6ae35901. Tamaño: 12 109 566 624 bytes.

Se conserva el código y los ejecutables, no una garantía universal de compatibilidad con cualquier proveedor, firmware o hardware. La URL antigua de Codespaces debe revisarse al recuperar el acceso. La reproducción de la infraestructura no acredita exactitud médica del modelo.
El script de restitución del día 25 estaba fijado a la instancia temporal 864367. No se debe ejecutar sin adaptar explícitamente la identidad del nuevo destino, las huellas y las rutas de esta entrega. Esa adaptación no se ha ensayado para el archivo del día 26.

La imagen conserva los archivos del comparador llama.cpp dentro de la carpeta de auditoría. No sustituye al motor original Rust ni se habilita como servicio de la aplicación.
