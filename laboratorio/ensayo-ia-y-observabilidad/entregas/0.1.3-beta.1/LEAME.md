# EIO conversación 0.1.3 · Beta

**Identificador de entrega:** `eio-conversacion-v0.1.3-beta.1`.  
**Fecha:** 22 de septiembre de 2026.  
**Plataforma:** Linux x86-64, CPU.  
**Uso previsto:** evaluación técnica de conversación, conservación de expedientes y observación de inferencia nativa.

Esta entrega fija una versión descargable del programa y su documentación técnica. Incluye el ejecutable 0.1.3, sus fuentes, la composición identificada, los avisos de licencias y las evidencias de comprobación. La versión de entrega identifica este conjunto; no certifica una aplicación profesional terminada.

El modelo utilizado es **Qwen3-0.6B Q4_K_M**. Los pesos y el tokenizador se identifican en `RECURSOS_MODELO.json`; deben obtenerse separadamente y comprobarse antes de iniciar el programa. No se incluyen expedientes de usuario.

## Lectura y contenido

| Elemento | Contenido |
|---|---|
| `FICHA_TECNICA.md` | Composición principal, condiciones de uso y límites. |
| `MANIFIESTO.json` | Identidad de la entrega, procedencia y huellas de sus archivos. |
| `COMPOSICION.json` | Paquetes resueltos por Cargo para la plataforma y licencias declaradas. |
| `RECURSOS_MODELO.json` | Identificación de pesos y tokenizador externos. |
| `bin/eio-conversacion` | Ejecutable Linux de la aplicación 0.1.3. |
| `fuentes/` | Código de la aplicación y resolución de dependencias. |
| `licencias/` | Avisos y textos de licencia conservados de los componentes. |
| `evidencias/` | Comprobaciones técnicas identificadas. |
| `consulta-documental/` | Protocolo, corpus y resultado del ensayo acotado DOC-01. |

## Preparación y arranque

Comprobar primero `SHA256SUMS` con `sha256sum -c SHA256SUMS`, desde la carpeta de la entrega. Comparar también la huella del archivo comprimido con la publicada junto a la descarga. Las huellas comprueban identidad; no sustituyen la verificación de procedencia.

Colocar los dos recursos externos identificados en una carpeta propia. Definir un directorio de datos persistente, distinto del código y con acceso restringido. En una instalación local de ensayo:

```sh
export EIO_MODELS=/ruta/absoluta/modelo
export EIO_DATA=/ruta/absoluta/datos
export EIO_BIND=127.0.0.1:3000
export EIO_ORIGIN=http://localhost:3000
./bin/eio-conversacion
```

El programa comprueba las huellas de los recursos al arrancar. La publicación remota exige un acceso protegido y transporte HTTPS. En Codespaces, el puerto debe permanecer privado. No se incluye un servicio de alojamiento ni una configuración de inicio automático.

La parada ordinaria se solicita con `Ctrl+C` o `SIGTERM`. Detener el anfitrión puede interrumpir el servicio y una petición. Al reiniciar, el registro reconstruye las conversaciones y marca las peticiones sin cierre; no reenvía preguntas automáticamente. Consultar `fuentes/RECUPERACION.md` antes de una intervención de recuperación.

## Uso y límites

La aplicación permite preguntas libres; **DOC-01 es un banco de pruebas separado y no convierte automáticamente esas conversaciones en consultas documentales restringidas**. Una terminación normal sólo acredita la finalización técnica declarada. No garantiza que la respuesta sea verdadera, completa o adecuada para una decisión.

La Beta no está validada para uso clínico. La guarda exterior, la custodia independiente del control y la integración completa de fuentes y permisos de la vía B siguen pendientes. La observación implementada no constituye aislamiento ni auditoría exhaustiva del sistema anfitrión.

## Licencias

Se conservan los avisos del SV en `licencias/AVISO_LICENCIAS.json`. Los componentes de terceros mantienen sus respectivas condiciones, identificadas en la composición. La entrega no amplía permisos de uso, modificación o redistribución. Los textos incorporados deben acompañar a los componentes cuando así lo exijan sus condiciones.
