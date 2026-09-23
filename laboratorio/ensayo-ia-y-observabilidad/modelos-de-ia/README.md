# Modelos de IA

**Edición:** 5 · 23 de septiembre de 2026.  
**Ámbito:** catálogo experimental del ensayo de inteligencia artificial y observabilidad.

El modelo, el motor numérico y la vía de ejecución son elementos distintos. Este catálogo identifica su relación y remite a las fuentes y evidencias existentes. Las instalaciones y los resultados conservan su ubicación; no se duplican pesos, dependencias ni expedientes.

| Ficha | Modelo | Vía B · nativa | Vía A · navegador/WebAssembly |
|---|---|---|---|
| [Qwen3-0.6B](qwen/qwen3-0.6b/README.md) | Qwen3-0.6B, Q4_K_M | Campaña cerrada como realización parcial con limitaciones; Beta 1 conservada. | Antecedentes NAV-02; inferencia real completa no acreditada. |
| [gpt-oss-20b](openai/gpt-oss-20b/README.md) | gpt-oss-20b | Instalado; continuación nativa sin respuesta. Señales externas y límites de carga documentados; seguimiento abierto. | Candidato sujeto a viabilidad; no se ha ejecutado. |
| [Derivado experimental de 4,8B](comunidad/gpt-oss-4.8b-5-expertos/README.md) | Reducción comunitaria de gpt-oss-20b; BF16 | Cotejo documental: empaquetado incompatible con la carga directa del motor fijado. Sin instalación ni inferencia. | No ensayado. |

Las fichas identifican versiones concretas, estado, requisitos y límites. Un nombre de proveedor no acredita equivalencia entre modelos. La [composición distribuida](../README.md#versión-distribuida), el [contrato experimental](../contrato/README.md) y la evidencia de cada ejecución mantienen su función propia.

Las fichas distinguen la instalación, la disponibilidad del servicio y la evaluación de inferencias. La presencia de los componentes instalados no acredita un servicio operativo.

## Organización por marca y modelo

Cada modelo tiene su propia carpeta: `qwen/qwen3-0.6b/` y `openai/gpt-oss-20b/`. Los índices [Qwen](qwen/README.md) y [OpenAI](openai/README.md) conservan las rutas anteriores y permiten incorporar otros modelos sin ocupar toda la carpeta de la marca.

Los derivados comunitarios se identifican en `comunidad/` con su autoría y modelo de origen; no se presentan como publicaciones oficiales de la marca.

La organización por modelo conserva las ubicaciones de las instalaciones y de sus evidencias. Cada actualización identifica el estado correspondiente a su fecha.

---

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es) · [Aviso del SV y licencias de terceros](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/controlador-nativo/AVISO_LICENCIAS.json).
