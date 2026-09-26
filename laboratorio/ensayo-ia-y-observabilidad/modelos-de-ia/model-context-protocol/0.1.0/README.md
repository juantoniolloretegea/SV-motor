# MCP documental — entrega recuperada para recepción

Fecha: 26 de septiembre de 2026. **Estado: preparación incompleta.**

Esta entrega publica el paquete original de preparación, conservando su identidad e integridad. No contiene resultados de recepción en OneCloud ni acredita compatibilidad ejecutada con el motor instalado. Publicar estos materiales no autoriza una inferencia, la reapertura de HCL ni el paso 2.

## Archivo que debe recibirse

- [SV-MCP-DOCUMENTAL-0.1.0-20260926.tar.gz](SV-MCP-DOCUMENTAL-0.1.0-20260926.tar.gz), 1.562.144 bytes.
- SHA-256: `6390b1df7a324aa5b5f2590ea3636db7e182269b592540a3ac0f8f7d25df8aa3`.
- [Instrucción original de recepción](sv-mcp-documental-0.1.0/INSTRUCCION-RECEPCION.md), conservada sin cambios dentro del paquete.
- [Precisiones para la recepción y pendientes](PENDIENTES-RECEPCION.md), que deben leerse antes de ejecutar el guion.
- [Código, construcción y límites](sv-mcp-documental-0.1.0/LEAME.md), [compatibilidad documental](sv-mcp-documental-0.1.0/COMPATIBILIDAD.md) y [Cargo.lock](sv-mcp-documental-0.1.0/Cargo.lock).

El archivo comprimido conserva exactamente la huella comunicada al terminar la preparación anterior. La copia desplegada bajo `sv-mcp-documental-0.1.0/` permite inspeccionar el contenido. No se ha corregido, recompilado ni sustituido el programa durante esta recuperación.

## Evidencia conservada y alcance

| Comprobación | Evidencia | Alcance |
|---|---|---|
| Compilador de la preparación | `evidencias/rustc.txt` | Rust 1.98.0, Linux x86_64; registro anterior |
| Pruebas automatizadas | `evidencias/cargo-test.txt` | Diez pruebas correctas en el entorno local anterior |
| Compilación optimizada | `evidencias/cargo-build-release.txt` | Finalización correcta; 11,44 s declarados por Cargo |
| Proceso documental y cliente Rust propio | `evidencias/cliente-final.json` y `evidencias/sesion-mcp-final.jsonl` | 32 peticiones, 25 páginas; reconstrucción idéntica al texto del catálogo |
| Identidad del paquete recuperado | `SV-MCP-DOCUMENTAL-0.1.0-20260926.tar.gz.sha256` | SHA-256 exterior coincidente |
| Integridad interior recuperada | `VERIFICACION-INTEGRIDAD-RECUPERACION.txt` | 25 entradas del manifiesto interior correctas; esta comprobación no ejecuta el servicio |
| Ejecutable instalado de mistral.rs y recepción en 864618 | Sin evidencia en este paquete | Pendientes |
| Utilización de herramientas por Qwen | No ejecutada | Pendiente; prohibida en esta entrega |

Los 35 registros del diario no equivalen a 35 peticiones: incluyen inicio, fin y notificación. Los tiempos de operación registrados por el servicio no incluyen necesariamente toda la espera, el transporte o la sincronización en disco. No se dispone de medición conservada del pico de RAM/CPU de la preparación ni de su duración total; no se inventan estos datos. El coste de una recepción futura aún no ejecutada tampoco es un consumo observado.

## Documento y antecedente

Se conserva el HTML del PDQ profesional en español del NCI, su extracción en cinco secciones y sus metadatos en `fuentes/`. La recuperación documental declarada en el paquete es `2026-09-26T12:20:31Z`; la actualización declarada por el documento es 14-11-2024. No se identifica esta captura actual con la utilizada históricamente en la campaña.

[Antecedente inalterado, fijado en el commit ffed6b40339c4ac19c8da18a1ceaf75c1ce78578](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/ffed6b40339c4ac19c8da18a1ceaf75c1ce78578/respuestas-ejecucion/QWEN38-HCL01-DIAGNOSTICO-20260926/continuacion-directa-20260926/INFORME.md): cuatro controles lógicos conformes; HCL-01 requiere revisión; HCL-02 detenido por error diagnóstico relevante; HCL-03 a HCL-12 no ejecutados. El acceso documental verificable es una hipótesis para evaluación posterior. No se afirma que resuelva el defecto.

El catálogo accesible a las herramientas contiene el documento oficial; no incluye este informe, el antecedente, las rúbricas ni las respuestas del banco. Las huellas acreditan identidad e integridad, no validez clínica ni suficiencia del respaldo de una conclusión.

## Relevo y conservación

Corte de entrada del repositorio privado: `main`, commit `ffed6b40339c4ac19c8da18a1ceaf75c1ce78578`. La ruta MCP no existía en ese corte. Esta publicación sólo añade esta carpeta; mantiene intactos los antecedentes, la gramática, la IR, el núcleo y los registros definitivos de calidad.

Durante esta recuperación no se ha accedido a OneCloud, iniciado una recepción remota, cargado pesos, ejecutado inferencias ni creado servicios, clientes MCP o temporizadores. La comprobación local se limita a lectura e integridad de los archivos. El cierre remoto no se declara nuevamente verificado.

El último estado remoto documentado por el antecedente, a las 12:00:15 UTC, conserva la instancia 864618 contratada, con el motor detenido. **La instancia se conserva y continúa facturándose**; esta publicación no cambia su contrato. La tarifa histórica declarada es 0,108333 EUR/h, no una lectura actual de facturación: 30 minutos equivaldrían aproximadamente a 0,0542 EUR, y 90 minutos a 0,1625 EUR, además del tiempo de permanencia. La recepción deberá contrastar estado y coste reales.

**Siguiente acto:** recepción técnica acotada conforme a las instrucciones y a los pendientes explícitos; entrega de sus registros y revisión humana. No se emite parte TT de ejecución concluida ni aceptación definitiva de calidad. Un resultado técnico favorable no autoriza continuar con el modelo.
