# Ficha técnica · GPT-OSS-Safeguard-120B

**Fecha:** 01/10/2026. **Estatuto:** preparación documental, sin mediciones propias.

| Elemento | Identidad y condición de comprobación |
|---|---|
| Modelo | `openai/gpt-oss-safeguard-120b`; revisión y artefacto de ejecución pendientes de fijar. |
| Función declarada | Clasificación de texto conforme a una política explícita. El uso como interlocutor especializado constituye una hipótesis adicional. |
| Conversación | Harmony; deben comprobarse plantilla, tokenizador, canales y finalización en la realización elegida. |
| Política de evaluación | Categorías, definiciones, criterios, excepciones y salida fijados antes de la prueba. |
| Realización candidata | CPU y Rust conforme al ensayo; motor, dependencias, representación de pesos y compatibilidad efectiva todavía por comprobar. |
| Memoria y tiempo | Sin máximo ni latencia medidos para este candidato. La disponibilidad nominal de 128 GB no demuestra que una configuración concreta cargue e infiera. |
| Fuentes | Corpus local identificado y MCP dentro de su recepción aplicable; sin acceso a Internet durante la inferencia. |
| Resultado | Pendiente; ninguna adjudicación de aptitud. |

La función de clasificación y el formato conversacional proceden de la [guía oficial de Safeguard](https://developers.openai.com/cookbook/articles/gpt-oss-safeguard-guide). Las restantes condiciones de la tabla pertenecen al diseño experimental del SV y no se atribuyen al fabricante.

## Comprobaciones para una prueba propia

Fijar la revisión del modelo y del motor, los avisos y licencias de sus artefactos, la representación numérica, la política aplicable y los casos con respuestas de referencia. Medir la memoria de carga y ejecución, las conversiones, la caché, los tiempos y el suministro efectivo de documentos. No extrapolar estas magnitudes desde el ordinario.

Evaluar por separado la clasificación según la política y la corrección sustantiva de la respuesta. La interpretación SV de `0`, `1` y `U`, los elementos críticos y la regla de aptitud deben fijarse en el banco; una etiqueta producida por el modelo no equivale automáticamente a esa adjudicación. Conservar aparte errores de ejecución y preguntas no realizadas.

La organización documental no instala pesos, no inicia consultas y no sustituye la recepción independiente.

[Estado](ESTADO.json) · [Presentación de la variante](README.md) · [Ficha del ordinario](../gpt-oss-120b/FICHA_TECNICA.md).

---

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es). Los componentes de terceros conservan sus licencias.
