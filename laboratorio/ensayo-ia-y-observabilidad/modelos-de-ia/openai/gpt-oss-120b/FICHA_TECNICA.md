# Ficha técnica · GPT-OSS-120B · preparación 1

**Base documental:** 27/09/2026. **Actualización:** 01/10/2026. **Seguimiento:** S39 / TT-0015.  
**Estatuto:** especificación documental del candidato; ninguna medición propia de este modelo.

| Elemento | Referencia y límite |
|---|---|
| Identidad comercial | `openai/gpt-oss-120b`. Revisión exacta y artefacto de ejecución pendientes de fijar. |
| Arquitectura publicada | Mezcla de expertos, 36 capas; 116,83 mil millones de parámetros totales y 5,13 mil millones activos por token. |
| Representación publicada | MXFP4 para los pesos de expertos; archivo de pesos de referencia de 60,8 GiB. No es memoria máxima del proceso. |
| Conversación | Harmony; plantilla, tokenizador, canales, fin de turno y configuración de razonamiento deben identificarse conjuntamente. |
| Motor candidato | mistral.rs nativo en Rust. Soporte de familia declarado; ejecutable y ruta CPU efectivos pendientes de comprobación. |
| Recursos | CPU. El antecedente de 62,79 GiB y cota de 54 GiB pertenece al estudio preliminar. Se dispone posteriormente de un recurso nominal de 128 GB autorizado; su capacidad efectiva y el máximo de consumo del modelo requieren medición propia. |
| Documentación | Catálogo local de fuentes originales conservadas y versionadas, con citas y localizadores verificables. |
| Licencias | Publicación oficial Apache-2.0 y política de uso gpt-oss; comprobar avisos de pesos, tokenizador, motor, conversión y dependencias de la distribución elegida. |

Fuente de parámetros y tamaño: [ficha oficial, tabla 1 y §2](https://arxiv.org/html/2508.10925v1). Fuente de conversación y distribución: [ficha oficial del modelo](https://huggingface.co/openai/gpt-oss-120b). Las mediciones oficiales en GPU no se presentan como resultados CPU del SV.

## Cuestiones técnicas previas

La memoria se desglosará en pesos residentes, posibles conversiones, buffers de carga, caché de atención, memoria temporal, motor, MCP, cliente, observador y reserva del sistema. Se distinguirán almacenamiento, RSS y consumo agregado del cgroup. No se obtendrá una cota total multiplicando parámetros activos ni extrapolando el archivo de pesos.

El [soporte declarado en mistral.rs](
https://github.com/EricLBuehler/mistral.rs/blob/2370966bb91e2e3dafa0b1521b87c50fd5c01244/docs/src/content/docs/reference/supported-models.md) debe contrastarse con el binario y sus características reales. La [optimización CPU anterior del SV](
https://github.com/juantoniolloretegea/SV-motor/blob/d4e62b29713a2044be0d1d4a7fb463155d3999c3/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/onecloud-2026-09-24/optimizacion/RESULTADO.md) conserva una corrección de entrada compartida, retirada de copias y paralelismo. Su compatibilidad con el nuevo modelo debe acreditarse sin modificar silenciosamente el motor ni los pesos. El factor de aceleración observado no se extrapola.

La política de razonamiento y el límite de generación se fijarán antes del banco. Se contabilizarán entrada, plantilla, herramientas, razonamiento y respuesta final según la semántica real del motor; una salida interna sin respuesta final no satisface el caso. No se abrirá artificialmente el canal final para atribuir al modelo una terminación que no produjo.

## Condiciones de selección

1. Configuración, fuentes y criterios fijados antes de ejecutar; una sola inferencia activa.
2. Casos separados de inmunología y ciberseguridad, sin datos personales ni intervención sobre sistemas reales.
3. Fuente efectivamente suministrada, respuesta íntegra, citas, localizadores y errores conservados.
4. Presupuestos de tiempo y memoria explícitos; cobertura y lagunas de observación declaradas.
5. Decisión diferenciada: Apto en el alcance ensayado, No apto o No evaluable por impedimento instrumental. No se suman bancos heterogéneos como tasa global.

No se autoriza por esta ficha entrenar, retirar expertos, cambiar cuantización, habilitar Internet durante la inferencia, incorporar modelos auxiliares ni integrar el candidato en el núcleo. La referencia MCP 0.1.2 sigue su recepción propia en TT-0014.

La identidad y los antecedentes de esta ficha pertenecen al modelo ordinario. Véase por separado la [ficha de GPT-OSS-Safeguard-120B](../gpt-oss-safeguard-120b/FICHA_TECNICA.md); no se transfieren presupuestos, compatibilidad ni resultados entre variantes.

[Estado](ESTADO.json) · [Ficha de seguimiento](README.md).
