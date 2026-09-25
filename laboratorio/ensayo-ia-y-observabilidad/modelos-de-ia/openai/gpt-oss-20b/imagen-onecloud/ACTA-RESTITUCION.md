# Acta de restitución de GPT-OSS desde la imagen de GitHub

Fecha: 25 de septiembre de 2026. Responsable de la comprobación: Profesor. Autorización: Director, Juan Antonio.

## Dictamen

**Se ha restituido el disco en una instancia nueva de OneCloud y se ha ejecutado GPT-OSS desde el sistema recuperado.** La prueba comprende arranque del sistema, identidad binaria, descarga de pesos con huella fija, conversación HTTP, cancelación efectiva, integridad del registro y persistencia tras reiniciar el aplicativo.

No es una certificación general de calidad del modelo, seguridad, rendimiento ni aptitud clínica. Una conversación correcta prueba el recorrido técnico ejercitado, no todos los comportamientos del aplicativo.

## Custodia y alcance

Origen: instancia `motor-ia-modelos-externos`, identificador 863733. No se reinstaló ni se reinició su aplicativo. Su disco original se conservó.

Destino desechable: `sv-restauracion-gptoss-prueba-20260925`, identificador 864367, Barcelona, plan DED640G12C: 12 núcleos, 64 GB de RAM y 640 GB de disco, 0,108333 EUR/h según el panel. Se creó por autorización expresa del Director para realizar la operación inversa y retirarla después de conservar las evidencias.

El destino descargó **el activo cifrado publicado en GitHub**, no una copia directa del disco original. SHA-256 cotejado: `6f63101afc63b4d394620d35160f7f32e3d4a5ebec39f90fc96d6cb464d85280`.

La clave de cifrado permanece fuera de GitHub. Los archivos grandes no se conservan en el PC; se retiraron las descargas parciales por instrucción expresa del Director.

## Método realmente probado

1. Reconstrucción de un disco RAW de 32 GiB con raíz ext4, GPT y GRUB BIOS, a partir del sistema original; exclusión de pesos, cachés y herramientas de compilación.
2. Comprobación `e2fsck -fn`, cifrado y verificación de descifrado por SHA-256.
3. Arranque de la imagen en QEMU sin red ni inferencia: alcanzó `multi-user.target` y el indicador de inicio de sesión de Ubuntu 26.04. El fallo de configuración de red de ese ensayo no demuestra un defecto de conectividad: la prueba no tenía interfaz de red.
4. Creación de una instancia OneCloud nueva con Ubuntu 26.04 inicial y acceso SSH propio.
5. Descarga de la imagen desde GitHub, descifrado y cotejo de la huella interior.
6. Preparación de un entorno de restauración en RAM con BusyBox y los módulos del kernel. Se cargó mediante `kexec`; desde ese entorno, con el disco del destino desmontado, se escribió el RAW sobre el disco de la instancia desechable.
7. Adaptación exclusivamente de identidad del destino: red, DNS, nombre de equipo, claves SSH y credenciales locales propias; se añadieron `net.ifnames=0 biosdevname=0` a su arranque para conservar el nombre de interfaz esperado por OneCloud. No se reescribió el programa ni se recompiló el motor.
8. Reinicio desde el disco restituido. Comprobados el registro `SV_RESTAURACION_864367_DISCO_COMPLETO_DESDE_GITHUB`, la raíz `/dev/sda2` y `root=LABEL=sv-gptoss-root`.
9. Descarga de los pesos de la revisión fijada, cotejo y prueba funcional.

El panel no ofreció una importación externa directa dentro de «My Images». El método acreditado es **restauración propia del disco desde RAM**, no importación nativa de una instantánea de OneCloud.

## Resultados

| Comprobación | Evidencia y resultado |
|---|---|
| Arranque del disco | Ubuntu 26.04, kernel 7.0.0-14-generic; consola y SSH operativos |
| Motor | SHA-256 `f6ec0bb908e18ced633d66d6a762a915349281bdbb1397991b268fe487bcc418`, coincidente |
| Aplicativo | SHA-256 `a54a15c9c709e66ee6c6b14d1102b3df8a73464dc7dc021e07137b5558630c2c`, coincidente |
| Tokenizador | SHA-256 `7c704477f22686ec2a8d7490ed55c799d875d2033eb833070cdd07c949037fcc` |
| Pesos | 12 109 566 624 bytes; SHA-256 `27cd6c432c7672cb812a92f611cf3ba7bbc35928262bb1e1253ff4ee6ae35901`, coincidente |
| Conversación | Solicitud `restauracion-20260925-01`; respuesta `4` a `2 + 2`; estado `fin_normal` |
| Cancelación | Solicitud `restauracion-20260925-cancelacion-02`; estado `cancelada`; motor `inactive/dead`, `MainPID=0` |
| Integridad | Comprobador del aplicativo: conforme; 14 expedientes, 42 conversaciones, 228 sucesos; 0 peticiones recuperadas |
| Persistencia | Exportaciones idénticas antes y después del reinicio del aplicativo, conservando ambos estados |
| Disco útil | Raíz de 32 GiB, aproximadamente 14 GiB ocupados y 17 GiB disponibles tras recuperar pesos |

SHA-256 del JSON de exportación serializado de forma compacta: `f8d19575c8220275c10d0cd4b3e96706d8c8d765420f4b16bf99bf1df4e6a36f`, antes y después del reinicio. Las huellas de los archivos de evidencia formateados se registran por separado en `HUELLAS-EVIDENCIAS.sha256`.

El reinicio produjo una instancia de servicio nueva (`servicio-1790371188620-0`) respecto de la anterior (`servicio-1790370890839-0`). No se confundió continuidad del proceso con lectura persistente del mismo expediente.

Los archivos de exportación publicados corresponden sólo al expediente sintético creado para esta prueba. Los restantes expedientes conservados en la imagen no se publican descifrados.

## Incidencias y límites observados

- El primer intento del cliente HTTP precedió a la disponibilidad del túnel. Falló antes de admitir una petición; no produjo inferencia. Se repitió tras comprobar el túnel.
- Tras el reinicio hubo comprobaciones SSH sin respuesta. La consola confirmó arranque, dirección del destino y servicio SSH activo; posteriormente se obtuvo acceso. No se corrigió la red después del arranque y no se atribuye la demora a una causa no demostrada.
- El botón NOVNC del panel no abrió una sesión accesible. Se utilizó temporalmente noVNC oficial, revisión `acca57b997f206683d27796829ee1f72da37002a`, mediante un túnel local privado. Se comprobó y corrigió la transmisión de mayúsculas del teclado durante la autenticación.
- La restauración exige adaptar identidad y red. No es una imagen con credenciales listas para arrancar públicamente.
- Los pesos quedan fuera de la imagen compacta y dependen de su descarga separada. La disponibilidad futura de Hugging Face y la disponibilidad comercial de una instancia compatible no quedan garantizadas por esta prueba.
- El ensayo demuestra el recorrido realizado en OneCloud x86_64 con BIOS. No acredita restauración UEFI ni en cualquier proveedor o hardware.
- No se extrapola la memoria observada durante esta prueba a un requisito máximo general de RAM.

## Retirada

La retirada de la instancia temporal se documentará tras verificar la publicación de estas evidencias. La instancia original queda fuera de esa retirada.