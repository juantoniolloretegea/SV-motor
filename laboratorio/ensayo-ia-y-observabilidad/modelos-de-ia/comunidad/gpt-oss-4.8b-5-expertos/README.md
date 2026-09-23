# Derivado experimental gpt-oss de 4,8B: cotejo del cargador

**23 de septiembre de 2026 · Resultado: incompatible con la carga directa del motor fijado.** No se creó una instancia, no se descargaron los pesos ni se ejecutó inferencia. Este resultado procede del cotejo documental de los archivos publicados y del código del motor; no es un fallo de ejecución observado.

## Identidad y alcance

| Elemento | Referencia comprobada |
|---|---|
| Derivado comunitario | `AmanPriyanshu/gpt-oss-4.8b-specialized-all-pruned-moe-only-5-experts`; no constituye una publicación oficial de OpenAI. |
| Revisión del modelo | `958d3985e02052839124e092ea9a5507db6fa5a9`. |
| Parámetros y almacenamiento | El índice declara 4 782 977 976 parámetros y 9 565 955 952 bytes de tensores, repartidos en cinco archivos. La ficha declara BF16. No equivale a memoria máxima de carga. |
| Arquitectura | 24 capas; cinco expertos por capa; cuatro seleccionados por token; dimensiones oculta e intermedia de 2880. |
| Motor cotejado | mistral.rs 0.9.3, revisión `24dbf5c256f232176ee5949485ba264049407fbe`. |
| Harmony | Se mantiene la referencia 0.0.8. No se ha sustituido ni ensayado otra variante. |
| Seguimiento | S39 y TT-0012; el ensayo oficial de 20B conserva su estado y sus evidencias. |

La finalidad prevista era obtener un testigo de arranque con una carga menor, conservando el motor. La condición de compatibilidad previa no se cumple para los archivos tal como se publican.

## Incompatibilidad concreta

El [índice publicado](https://huggingface.co/AmanPriyanshu/gpt-oss-4.8b-specialized-all-pruned-moe-only-5-experts/blob/958d3985e02052839124e092ea9a5507db6fa5a9/model.safetensors.index.json) utiliza en las 24 capas proyecciones combinadas BF16 con claves como:

- `model.layers.0.mlp.experts.gate_up_proj`;
- `model.layers.0.mlp.experts.gate_up_proj_bias`;
- `model.layers.0.mlp.experts.down_proj`;
- `model.layers.0.mlp.experts.down_proj_bias`.

El [cargador GPT-OSS fijado](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-core/src/models/gpt_oss.rs), en `has_interleaved_expert_projection` y `GptOssMoE::new`, reconoce la disposición combinada mediante `gate_up_proj_blocks` o una fuente cuantizada que contenga `gate_up_proj.weight`. La carga directa de este derivado no proporciona esas entradas. La alternativa de carga solicita proyecciones separadas `gate_proj.weight`, `up_proj.weight` y `down_proj.weight`, también ausentes del índice.

Se cotejaron asimismo [normal_loaders.rs](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-core/src/pipeline/loaders/normal_loaders.rs), [normal.rs](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-core/src/pipeline/normal.rs), [macros.rs](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-core/src/pipeline/macros.rs) y [varbuilder_utils.rs](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-core/src/utils/varbuilder_utils.rs). En el recorrido directo de Safetensors examinado no se aplica una conversión de estas proyecciones BF16 al formato solicitado por el modelo.

**Conclusión técnica:** cambiar únicamente el identificador del modelo no permite efectuar el ensayo propuesto con esta revisión del motor. Una adaptación tendría que tratar disposición de tensores, separación de las proyecciones y sesgos, además de sus nombres; no queda validada por un simple renombrado.

## Consecuencias y límites

La incompatibilidad identificada pertenece a la lectura de pesos. Sustituir Harmony no la corrige, porque Harmony representa y procesa la conversación, no convierte los tensores del modelo. El hallazgo tampoco identifica la causa de las señales o del consumo registrados con el modelo oficial de 20B.

No se modifica el cargador ni se transforma el modelo en este cotejo. Para retomar esta candidata haría falta una representación ya compatible o una adaptación explícita y verificada del cargador Rust. Eso constituiría una intervención distinta de mantener fijo el motor y sustituir únicamente el modelo.

La [ficha del autor](https://huggingface.co/AmanPriyanshu/gpt-oss-4.8b-specialized-all-pruned-moe-only-5-experts) declara reducción de expertos sin reentrenamiento y carácter experimental. La menor cantidad de parámetros no acredita calidad de respuestas ni suficiencia de memoria. No se ha realizado evaluación semántica, clínica o de ciberseguridad.

## Evidencia conservada

Se conservan [config.json](config.json) y [model.safetensors.index.json](model.safetensors.index.json), transcritos del visor público de archivos correspondiente a la revisión identificada. Sus huellas describen estas transcripciones locales; no se presentan como comprobación de los bytes descargados del repositorio ni de los pesos.

| Archivo conservado | SHA-256 |
|---|---|
| config.json | `bfbe5bc3daaaddc7805fa0b73a4f374e5eed12fbf6204ba946024209f1620527` |
| model.safetensors.index.json | `55cf2733cc79c0c4d451e46c2e35b314aed7245c5f3053f61c15e173b7b7aeec` |

Cortes de entrada: SV-motor `ae9ceaef37e18d9cc12e25ea0369e4ebf58d9fb8`; seguimiento en SV-lenguaje-de-computacion `e9e4553d8f354678bc099a4a43bb6bae5cdf4f6a`. Las instalaciones existentes no se activaron ni modificaron.

---

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es). Metadatos del modelo derivados de la publicación de Aman Priyanshu, que declara [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0); se conserva esa atribución y no se les aplica la licencia del SV. Código del motor consultado bajo su [licencia MIT](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/LICENSE).
