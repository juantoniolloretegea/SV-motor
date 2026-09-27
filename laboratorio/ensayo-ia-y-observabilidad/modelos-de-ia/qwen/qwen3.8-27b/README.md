# Qwen3.8-27B

**Cierre documental: 27 de septiembre de 2026. Estado: NO PASA la selección actual.**

La configuración ensayada se retira de la selección médica de esta campaña. La comprobación final recibió una respuesta completa, pero encontró una omisión material y el incumplimiento del contrato de citas y localización. El resultado no se atribuye a un fallo de acceso documental ni a un agotamiento del tiempo. No acredita incapacidad general del modelo, una tasa de error clínico ni aptitud clínica.

## Identidad y condiciones

| Elemento | Identificación |
|---|---|
| Modelo | Qwen/Qwen3.8-27B; distinto de Qwen3-0.6B, Qwen3-8B y Qwen3.8-Max |
| Revisión de pesos y configuración | `1d4bf0f2ff6012fd82039f2fa52739d0dd7c60c0` |
| Motor | mistral.rs; revisión `2370966bb91e2e3dafa0b1521b87c50fd5c01244` |
| Ejecución | Rust nativo en CPU, Linux x86_64; vía B |
| Entorno observado | 12 vCPU, 64 GB de RAM; cota del motor de 54 GiB |
| Consulta final | Recuperación documental local mediante MCP, sin acceso a Internet durante la consulta |
| Instrumentación reutilizable | [MCP documental 0.1.2](../../model-context-protocol/0.1.2) |
| Pesos en las imágenes | No incluidos; recuperación externa sujeta a disponibilidad y cotejo de huellas |

La cota de 54 GiB incluye caché de archivos; no determina la memoria mínima intrínseca del modelo. No se ha acreditado la misma ejecución en 32 GB. El motor nativo de este ensayo no implica que la composición histórica de Qwen3-0.6B sea equivalente.

## Resultados y conservación

- [Secuencia de ensayos, dictámenes y fuentes](SEGUIMIENTO.md).
- [Dos imágenes de conservación, manifiesto y límites de recuperación](imagen-onecloud/README.md).
- [Entrega de archivo experimental](https://github.com/juantoniolloretegea/SV-motor/releases/tag/qwen38-27b-archivo-cierre-20260927-v1).
- [Expediente privado de la comprobación final](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/tree/7a116cddac3f0b98bdc52fb84e1ed1e0f2b23791/respuestas-ejecucion/QWEN38-SELECCION-MINIMA-LOCAL-20260927/entrega-01), con acceso restringido.

La imagen del 26/09 conserva el primer diagnóstico; la del 27/09 añade los ensayos posteriores y el cierre. La imagen inicial no sustituye el archivo final. Ninguna de estas dos imágenes Qwen se ha arrancado en una instancia restaurada. La comprobación previa de otro modelo no demuestra su restauración.

La conservación técnica no reabre la selección ni declara recibido el MCP. S39 y TT-0014 mantienen sus registros propios. Las preguntas reservadas, las rúbricas y los intercambios completos permanecen en custodia restringida o cifrada.

[Índice Qwen](../README.md) · [Catálogo de modelos](../../README.md).
