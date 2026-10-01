# Modelos de IA

**Edición documental 9 · 30 de septiembre de 2026.**

**Corte experimental:** 30/09/2026, 08:42 UTC; apertura documental posterior de Thinking identificada separadamente.

El [índice general del ensayo](../README.md) reúne el estado, la cronología de publicaciones y las versiones de software. Este catálogo mantiene una sola entrada por modelo y enlaza su ficha.

| Secuencia del estudio | Modelo y ficha | Estado de la vía B nativa |
|---|---|---|
| 1 | [Qwen3-0.6B · Q4_K_M](qwen/qwen3-0.6b/README.md) | Campaña cerrada como realización parcial; distribución 0.1.3-beta.1 conservada. |
| 2 | [GPT-OSS-20B · MXFP4](openai/gpt-oss-20b/README.md) | Campaña cerrada; configuración excluida de la función médica prevista. |
| 3 | [Qwen3.8-27B](qwen/qwen3.8-27b/README.md) | No pasa la selección actual; seguimiento y archivo de cierre conservados. |
| 4 | [GPT-OSS-120B](openai/gpt-oss-120b/README.md) | Estudio preliminar entregado: configuración considerada desfavorable. Sin inferencia ni máximo de memoria medido; [TT-0015](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0015.md). |
| 5 | [Qwen3-Next-80B-A3B-Instruct · UQFF Q4K](qwen/qwen3-next-80b-a3B-instruct/REGISTRO-INSTALACION-20260929.md) | Dos consultas documentales completas recibidas. Examen posterior parcial comunicado: ocho respuestas, interrupción en P09 y recepción pendiente; [TT-0016](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0016.md). |
| 6 | [Qwen3-Next-80B-A3B-Thinking · UQFF Q4K](qwen/qwen3-next-80b-a3b-thinking) | Prueba independiente preparada y transmisión humana comunicada. Carpeta propia abierta; sin funcionamiento recibido; [TT-0017](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0017.md). |

Las vías de ejecución pertenecen a configuraciones concretas. La vía A requiere inferencia en el navegador mediante WebAssembly; una interfaz web conectada a un proceso nativo corresponde a B. La continuación principal corresponde a Instruct; Thinking mantiene su realización independiente. A queda diferida hasta Apto experimental nativo y autorización específica.

La recepción instrumental confirma un funcionamiento acotado, no la corrección de todo un examen. En la [ronda de 25 posiciones](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/6f1118a323bcefac9400d01274e4f8d1db3636ff/dominios/inmunologia/tes-examenes-conocimientos/ronda-01-pdq-tricoleucemia-20260929/PROTOCOLO.md), sólo las adjudicaciones válidas sustentan el vector completo, su frame y el dictamen Apto, No apto o U. Las incidencias técnicas y las preguntas pendientes se registran fuera de la terna; no se completan artificialmente con U.

## Componentes y estudios separados

- [Servicio documental MCP](model-context-protocol/README.md): versiones 0.1.0, 0.1.1 y 0.1.2; componente de acceso documental, no modelo.
- [Derivado comunitario de 4,8B](openai/comunidad/gpt-oss-4.8b-5-expertos/README.md): estudio documental del empaquetado; sin instalación ni inferencia acreditadas. No es una publicación oficial de OpenAI.
- [Registro de versiones](../VERSIONES.json): etiquetas, commits, versiones declaradas e identidades de los artefactos publicados.

La capacidad de generar texto, la fidelidad documental, la conformidad contractual y la aptitud para cada dominio se evalúan por separado. Los resultados no se transfieren entre modelos ni se convierten en una tasa global.

[Índice Qwen](qwen/README.md) · [Índice OpenAI](openai/README.md) · [Contrato experimental](../contrato/README.md).

---

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es). Las licencias de terceros se identifican en las fichas y distribuciones correspondientes.
