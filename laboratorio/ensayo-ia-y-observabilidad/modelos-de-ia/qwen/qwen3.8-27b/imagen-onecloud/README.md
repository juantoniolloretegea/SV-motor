# Archivo de imágenes Qwen3.8-27B

**Corte: 27/09/2026.** [Entrega común en SV-motor](https://github.com/juantoniolloretegea/SV-motor/releases/tag/qwen38-27b-archivo-cierre-20260927-v1).

| Imagen cifrada | Tamaño exacto | Alcance |
|---|---:|---|
| `qwen38-sistema-20260926.raw.gz.enc` | 1.294.785.680 bytes | Sistema compacto y diagnóstico inicial; copia idéntica del activo conservado anteriormente con sufijo `.part-000`. |
| `qwen38-cierre-20260927.raw.gz.enc` | 1.423.503.824 bytes | Sistema compacto anterior, actualizado con los expedientes posteriores y el cierre del 27/09. |

Cada imagen representa un disco lógico de 32 GiB. Los pesos Qwen no están incluidos. La imagen final también excluye dos pequeños archivos de tensores de ejemplos de dependencias que estaban presentes en la copia anterior; no eran pesos del candidato.

[Manifiesto y SHA-256](MANIFIESTO.json) · [Condiciones de recuperación](RESTAURACION.md).

Las imágenes están cifradas. Las claves se custodian fuera de GitHub. Su contenido descifrado incluye material reservado y no debe publicarse. Se ha verificado la integridad de la copia final y la correspondencia del descifrado con el archivo comprimido. **No se ha probado el arranque ni la restauración de ninguna de estas dos imágenes Qwen.**

Los archivos originales y los pesos de la instancia no se eliminan mediante esta entrega. El motor queda detenido. La retirada de infraestructura corresponde a una actuación posterior; la existencia del archivo no acredita por sí sola su baja.
