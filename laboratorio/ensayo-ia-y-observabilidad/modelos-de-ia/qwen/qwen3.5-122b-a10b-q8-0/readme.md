# Qwen3.5-122B-A10B · Q8_0

**Expediente de selección técnica · Versión 1.1 · 4 de octubre de 2026**

**Estado:** instalación y primera respuesta instrumental acreditadas; evaluación documental, viabilidad temporal y aptitud pendientes. Véase la [recepción instrumental del 4 de octubre](seguimiento/RECEPCION_INSTRUMENTAL_20261004.md), con sus mediciones y reservas.

Se estudia **Qwen3.5-122B-A10B**, desarrollado por Qwen, en la distribución **GGUF Q8_0 de Unsloth**. El objetivo es comprobar si puede interpretar fuentes locales delimitadas, aplicar una política explícita y producir respuestas completas y verificables bajo el gobierno del Sistema Vectorial SV.

La selección se fundamenta en sus características técnicas, los resultados comparativos publicados y necesidades que no quedaron satisfechas en los contrastes documentados de GPT-OSS-Safeguard-120B. Es una hipótesis de mejora: las puntuaciones del modelo general y los resultados de otra representación numérica no constituyen recepción del candidato Q8_0.

## Recorrido de lectura

| Orden | Documento | Contenido |
|---|---|---|
| 1 | [Selección y necesidad](justificacion/SELECCION_Y_NECESIDAD.md) | Problema, antecedentes, hipótesis y límites de la sustitución. |
| 2 | [Ficha técnica](ficha-tecnica/FICHA_TECNICA.md) | Identidad, arquitectura, contexto, modalidades, cuantización y condiciones materiales. |
| 3 | [Comparativa con GPT-OSS](comparativa/COMPARATIVA_GPT_OSS.md) | Doce pruebas publicadas, diferencias y alcance de la comparación. |
| 4 | [Criterios de recepción](evaluacion/CRITERIOS_DE_RECEPCION.md) | Evidencias necesarias para comprobar utilidad, fidelidad, aislamiento y viabilidad temporal. |
| 5 | [Fuentes y versiones](fuentes/FUENTES.md) | Referencias primarias, revisiones fijadas y límites de verificación. |
| 6 | [Estado estructurado](seguimiento/ESTADO.json) y [revisión documental](seguimiento/REVISION_DOCUMENTAL_20261004.md) | Estado de las afirmaciones y resultado de la revisión crítica previa a publicación. |

**Material complementario:** [identidad y archivos Q8_0](ficha-tecnica/IDENTIDAD_Y_ARCHIVOS.json), [datos comparativos CSV](comparativa/RESULTADOS_PUBLICADOS.csv), [tabla PNG](comparativa/COMPARATIVA_GPT_OSS_QWEN.png) y [huellas del expediente](fuentes/SHA256SUMS.txt).

## Situación que debe conservarse

- La comparación externa corresponde a **GPT-OSS-120B general**; los antecedentes experimentales propios citados corresponden a **GPT-OSS-Safeguard-120B**. Son objetos distintos.
- Q8_0 y FP8 son representaciones diferentes. No se afirma equivalencia de resultados, tiempos ni errores entre ellas.
- El modelo propone una respuesta documental. La validación y la adjudicación SV conservan su autoridad externa al modelo.
- La consulta se proyecta sobre un corpus local identificado y mediante acceso documental controlado. La restricción de fuentes requiere controles efectivos, además de instrucciones.
- El expediente no acredita ausencia universal de errores, aptitud clínica ni integración en el Núcleo.

```mermaid
flowchart LR
    A["Necesidad documental y política explícita"] --> B["Selección del candidato"]
    B --> C["Recepción instrumental"]
    C --> D["Evaluación documental acotada"]
    D --> E["Adjudicación externa y decisión"]
    E --> F["Registro de capacidades, límites y necesidades"]
```

El diagrama conserva la secuencia del ensayo. La recepción instrumental mínima está documentada; la evaluación documental y la adjudicación de aptitud siguen pendientes.

## Organización y continuidad

Cada subcarpeta tiene una función única. La ficha conserva especificaciones; la comparativa conserva datos externos; la justificación relaciona necesidades y antecedentes; la evaluación define condiciones pendientes; el seguimiento identifica las realizaciones efectivamente recibidas. Los resultados futuros deberán añadir fecha, configuración y evidencias propias, sin modificar retrospectivamente los antecedentes.

[Índice Qwen](../README.md) · [Catálogo de modelos](../../README.md) · [Ensayo de IA y observabilidad](../../../README.md).

---

Documentación del Sistema Vectorial SV. Los modelos y componentes de terceros conservan sus licencias de origen.
