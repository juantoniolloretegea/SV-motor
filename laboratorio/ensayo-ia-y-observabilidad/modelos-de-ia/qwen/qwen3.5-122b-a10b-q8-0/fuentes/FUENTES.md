# Fuentes, versiones y trazabilidad

**Consulta: 4 de octubre de 2026.** Las revisiones fijadas identifican los documentos consultados; no acreditan descarga ni ejecución de pesos. Los enlaces de documentación externa sin revisión inmutable conservan su fecha de consulta y pueden cambiar.

## 1. Fuentes primarias del candidato

| Código | Fuente | Aportación |
|---|---|---|
| Q01 | [Ficha oficial](https://huggingface.co/Qwen/Qwen3.5-122B-A10B/blob/dc4d348443bc740c68e2d77492492c11606384d5/README.md) | Identidad, arquitectura declarada, contexto, modalidades y evaluaciones publicadas. |
| Q02 | [Configuración oficial](https://huggingface.co/Qwen/Qwen3.5-122B-A10B/blob/dc4d348443bc740c68e2d77492492c11606384d5/config.json) | Arquitectura, capas, expertos y dimensiones efectivamente declarados en la configuración. |
| Q03 | [Archivos Q8_0 de Unsloth](https://huggingface.co/unsloth/Qwen3.5-122B-A10B-GGUF/tree/51eab4d59d53f573fb9206cb3ce613f1d0aa392b/Q8_0) | Distribución elegida y cuatro fragmentos. |
| Q04 | [Metadatos públicos de la distribución](https://huggingface.co/api/models/unsloth/Qwen3.5-122B-A10B-GGUF?blobs=true) | Tamaños exactos y SHA-256 declarados. Valores fijados en el inventario de este expediente. |
| Q05 | [Metadatos del modelo de origen](https://huggingface.co/api/models/Qwen/Qwen3.5-122B-A10B?blobs=true) | Revisión y licencia declarada del origen. |
| Q06 | [Licencia del origen](https://huggingface.co/Qwen/Qwen3.5-122B-A10B/blob/dc4d348443bc740c68e2d77492492c11606384d5/LICENSE) | Apache-2.0; alcance propio de los componentes de terceros. |
| M01 | [Compatibilidad GGUF de mistral.rs](https://docs.mistralrs.dev/reference/gguf-support/) | Arquitectura `qwen35moe`, representación Q8_0 y carga textual sin proyector. |
| O01 | [Guía oficial de GPT-OSS-Safeguard](https://developers.openai.com/cookbook/articles/gpt-oss-safeguard-guide) | Especialización en políticas y necesidad de instrucciones de salida coherentes. |

El inventario fija la revisión del origen `dc4d348443bc740c68e2d77492492c11606384d5` y la de distribución `51eab4d59d53f573fb9206cb3ce613f1d0aa392b`. Son repositorios distintos. No se deduce la revisión exacta utilizada durante la conversión a partir de la revisión actual de la ficha original.

## 2. Antecedentes del SV

Se consultó el estado de SV-motor identificado por `7e2bcb2ac9d5b7803c8dba76df91757fa749c458`, que ya contiene la apertura de esta carpeta.

| Código | Fuente fijada | Alcance utilizado |
|---|---|---|
| S01 | [Informe A0](https://github.com/juantoniolloretegea/SV-motor/blob/7e2bcb2ac9d5b7803c8dba76df91757fa749c458/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-safeguard-120b/ensayos-reiterados-adversariales-y-aprendizaje/resultados/A0/INFORME-A0.md) | Nueve respuestas, ocho clasificaciones correctas, conflicto A08 y omisión de recepción. |
| S02 | [Informe A2](https://github.com/juantoniolloretegea/SV-motor/blob/7e2bcb2ac9d5b7803c8dba76df91757fa749c458/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-safeguard-120b/ensayos-reiterados-adversariales-y-aprendizaje/resultados/A2/INFORME-A2.md) | Persistencia del error y de omisiones; comparación A0–A2; reservas del diseño. |
| S03 | [Política del contraste](https://github.com/juantoniolloretegea/SV-motor/blob/7e2bcb2ac9d5b7803c8dba76df91757fa749c458/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-safeguard-120b/ensayos-reiterados-adversariales-y-aprendizaje/protocolo/POLITICA.txt) | Regla para fuentes discrepantes, obligaciones y prohibición de autocalificación SV. |
| S04 | [Guía del sistema conjunto, revisión 2](https://github.com/juantoniolloretegea/SV-motor/blob/7e2bcb2ac9d5b7803c8dba76df91757fa749c458/laboratorio/sistema-conjunto-lenguaje-computacion-ia-gobernada/README.md) | Finalidad arquitectónica, separación de autoridad y representación celular. |

S04 es una síntesis fechada el 27/09/2026. Se utiliza para el marco conceptual, no como inventario actual de instalaciones o ensayos. Los resultados posteriores se contrastan con sus informes específicos. Los encabezados antiguos de otras fichas no prevalecen sobre resultados fechados posteriores.

## 3. Clases de afirmación

| Clase | Ejemplo | Límite |
|---|---|---|
| Dato publicado por el desarrollador | Puntuación en IFEval o contexto declarado | No es una medición independiente de Q8_0. |
| Metadato publicado del archivo | Tamaño o SHA-256 de un fragmento | Requiere cotejo local después de la descarga. |
| Resultado experimental recibido | Clasificación y omisiones de A2 | Limitado a configuración, fuentes y criterio del antecedente. |
| Inferencia de selección | Priorizar un candidato por señales favorables | Hipótesis; no certificación de su conducta futura. |
| Condición prospectiva | Medir latencia y comprobar aislamiento | No se presenta como ya ejecutada. |

## 4. Conservación del expediente

[SHA256SUMS.txt](SHA256SUMS.txt) enumera las huellas SHA-256 de los archivos de esta entrega, con rutas relativas a la raíz del expediente. El propio manifiesto no se incluye en sí mismo. La tabla Markdown, el CSV y el PNG expresan las mismas doce filas y diferencias.

La consulta de metadatos no descargó los pesos. Tampoco ejecutó inferencias, creó una realización del candidato o produjo un dictamen. No se adjuntan expedientes reservados ni se sustituyen los documentos primarios.

[Volver al expediente](../readme.md).
