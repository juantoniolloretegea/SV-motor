# Preparación documental de Safeguard y MCP

**Fecha:** 01/10/2026. **Modelo:** openai/gpt-oss-safeguard-120b. **Estado:** preparación; instalación e inferencia no acreditadas.

La función propuesta es clasificar la correspondencia de afirmaciones con pasajes conservados mediante una política explícita. Se mantiene acceso exclusivo a la caché autorizada durante la consulta. Los conocimientos aprendidos del modelo no sustituyen la evidencia documental; su suficiencia deberá comprobarse con casos independientes.

Se ha implementado [MCP documental 0.1.3](../../model-context-protocol/0.1.3/LEAME.md): consulta de hasta 200 caracteres sin restricción adicional de ocho palabras, continuación de resultados, errores recuperables y diario verificable por reconstrucción. [Pruebas Rust](../../model-context-protocol/0.1.3/PRUEBAS.txt) y [evidencia sintética](../../model-context-protocol/0.1.3/evidencias/COTEJO.json). No se ha conectado esta revisión al candidato.

La [adenda de preparación](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/main/encargos-ejecucion/GPTOSS-SAFEGUARD-PREPARACION-20261001/v1/ADENDA.md), en custodia restringida, fija política de ejemplo, integración Harmony, telemetría completa, cotejo y continuidad. La publicación no inicia la instalación; la orden humana de ejecución identificará su revisión.

**Criterio:** todo tramo relevante debe ser observable, trazable y reconstruible. Su ausencia determina No apto para el uso exigido por el SV. La reproducción del contenido admite paráfrasis, pero no cambios de hechos, cantidades, condiciones, negaciones, relaciones o incertidumbre, ni omisiones de elementos exigidos. La salida del modelo no es su propia adjudicación.

La guía oficial de [Safeguard](https://developers.openai.com/cookbook/articles/gpt-oss-safeguard-guide) fundamenta la entrega de políticas escritas. El proyecto [Snilld AI](https://github.com/snilld-ai/openai-assistant-mcp) utiliza servicios remotos de OpenAI y no satisface como sustituto directo el perímetro ni la reconstrucción exigidos. No se instala.

**Continuidad:** TT-0014 recibe las mejoras del componente. TT-0015 mantiene el alcance del GPT-OSS ordinario; no se atribuye a Safeguard. La ejecución futura identificará su tique y cotejará el conjunto modelo–conductor–MCP–observador antes de evaluarlo. Las campañas Qwen conservan expedientes independientes.
