# Ficha técnica de Qwen3.5-122B-A10B · GGUF Q8_0

**Versión 1.0 · Consulta documental: 04/10/2026 · Estado: preparación pendiente de recepción.**

## 1. Identidad y procedencia

| Elemento | Identificación |
|---|---|
| Modelo de origen | `Qwen/Qwen3.5-122B-A10B` |
| Entidad desarrolladora | Qwen |
| Distribución seleccionada | `unsloth/Qwen3.5-122B-A10B-GGUF` |
| Representación | GGUF, cuantización Q8_0 |
| Revisión documental del origen | `dc4d348443bc740c68e2d77492492c11606384d5` |
| Revisión de la distribución seleccionada | `51eab4d59d53f573fb9206cb3ce613f1d0aa392b` |
| Licencia declarada en los metadatos de ambos repositorios | Apache-2.0 |

La revisión del origen identifica la ficha y configuración consultadas. La revisión de Unsloth identifica los archivos elegidos. **No se ha acreditado que el proceso de conversión utilizara precisamente la revisión del origen aquí consultada**; se conservará esa distinción en la recepción. [Ficha oficial](https://huggingface.co/Qwen/Qwen3.5-122B-A10B/blob/dc4d348443bc740c68e2d77492492c11606384d5/README.md) · [Distribución Q8_0](https://huggingface.co/unsloth/Qwen3.5-122B-A10B-GGUF/tree/51eab4d59d53f573fb9206cb3ce613f1d0aa392b/Q8_0) · [Inventario estructurado](IDENTIDAD_Y_ARCHIVOS.json).

## 2. Arquitectura declarada

El modelo combina una mezcla de expertos dispersa con mecanismos de atención híbridos. Declara **122 mil millones de parámetros totales y aproximadamente 10 mil millones activados por token**. La cifra activa describe una fracción del cálculo; no permite dimensionar los pesos como si el modelo completo tuviera sólo 10 mil millones de parámetros. [Ficha oficial](https://huggingface.co/Qwen/Qwen3.5-122B-A10B/blob/dc4d348443bc740c68e2d77492492c11606384d5/README.md).

| Propiedad | Valor de configuración |
|---|---|
| Arquitectura en Transformers | `Qwen3_5MoeForConditionalGeneration` |
| Capas de lenguaje | 48 |
| Dimensión interna del lenguaje | 3072 |
| Secuencia híbrida | Tres capas de atención lineal por cada capa de atención completa |
| Expertos encaminados disponibles / seleccionados por token | 256 / 8 |
| Experto compartido | Presente |
| Cabezas de atención completa / cabezas de clave y valor | 32 / 2 |
| Tamaño del vocabulario | 248320 |

La atención lineal corresponde a la familia Gated DeltaNet. El experto compartido y los componentes comunes participan además de los expertos seleccionados. Los valores describen el diseño, no una medida de rendimiento en el entorno de ensayo. [Configuración oficial](https://huggingface.co/Qwen/Qwen3.5-122B-A10B/blob/dc4d348443bc740c68e2d77492492c11606384d5/config.json).

## 3. Contexto, modalidades y generación

La ficha declara **262144 tokens de contexto nativo**, con extensión hasta **1010000** mediante la configuración correspondiente. La capacidad nominal no acredita que esa longitud resulte viable con una memoria y un motor concretos. El contexto efectivo deberá fijarse según la tarea, incluyendo política, documentos, antecedentes y reserva de salida. [Ficha oficial](https://huggingface.co/Qwen/Qwen3.5-122B-A10B/blob/dc4d348443bc740c68e2d77492492c11606384d5/README.md).

El modelo admite texto, imágenes y vídeo, y declara cobertura de 201 idiomas y dialectos. La primera función considerada en este expediente es **textual y documental**. El uso de imágenes o vídeo exigiría componentes y recepción propios; no se deduce de la mera descarga de los pesos textuales.

El razonamiento está activado por defecto en la configuración de uso descrita por Qwen y puede desactivarse mediante la plantilla correspondiente. La elección efectiva, los límites de salida y los parámetros de generación deberán quedar registrados. La existencia de predicción de varios tokens —MTP— tampoco demuestra que esté disponible o activada en la realización seleccionada. [Ficha oficial](https://huggingface.co/Qwen/Qwen3.5-122B-A10B/blob/dc4d348443bc740c68e2d77492492c11606384d5/README.md).

## 4. Archivos y representación numérica

Los cuatro fragmentos seleccionados suman **129871935104 bytes: 129,87 GB decimales o 120,95 GiB**. Los tamaños y SHA-256 publicados se conservan en [IDENTIDAD_Y_ARCHIVOS.json](IDENTIDAD_Y_ARCHIVOS.json), junto con enlaces a una revisión fija.

| Fragmento | Tamaño en bytes |
|---|---:|
| `Qwen3.5-122B-A10B-Q8_0-00001-of-00004.gguf` | 10943552 |
| `Qwen3.5-122B-A10B-Q8_0-00002-of-00004.gguf` | 49783204544 |
| `Qwen3.5-122B-A10B-Q8_0-00003-of-00004.gguf` | 49805778720 |
| `Qwen3.5-122B-A10B-Q8_0-00004-of-00004.gguf` | 30272008288 |
| **Total** | **129871935104** |

Deben recibirse los cuatro fragmentos; su tamaño desigual no permite omitir el primero. Estas huellas son **referencias publicadas**, todavía no cotejos de una descarga local. El inventario no incluye un componente visual ni archivos auxiliares que pudieran resultar necesarios para una realización concreta. [Metadatos de distribución](https://huggingface.co/api/models/unsloth/Qwen3.5-122B-A10B-GGUF?blobs=true).

Q8_0 y FP8 emplean representaciones distintas, aunque ambas se describan mediante ocho bits. No se declara una pérdida inferior al 1 %, equivalencia numérica ni identidad con un servicio remoto: tales afirmaciones requerirían pruebas específicas.

## 5. Viabilidad prevista en CPU y Rust

La hipótesis material considera **ejecución en CPU y 256 GB de RAM**. Los pesos ocupan aproximadamente la mitad de esa capacidad nominal, pero el resto debe alojar sistema, estructuras del motor, estados de contexto, trabajo temporal y observabilidad. Puede haber consumos adicionales o máximos durante la carga. No se ha medido el máximo residente, el tiempo hasta la primera salida ni la velocidad sostenida.

La documentación vigente de mistral.rs enumera `qwen35moe` y Q8_0 entre sus formatos admitidos. También describe carga textual sin proyector para esta arquitectura. Esto justifica estudiar una realización en Rust, pero no verifica una versión concreta del ejecutable, la carga de estos cuatro fragmentos ni su corrección numérica. [Compatibilidad oficial de mistral.rs](https://docs.mistralrs.dev/reference/gguf-support/).

La recepción fijará versión y huella del motor, opciones de compilación, plantilla, tokenizador y archivos auxiliares. El rendimiento depende de la implementación, el procesador, la memoria, la concurrencia, la longitud de entrada y la salida solicitada; no se deduce únicamente del número de parámetros activos.

## 6. Evidencias todavía pendientes

Descarga y cotejo de los cuatro fragmentos; configuración ejecutable fijada; carga correcta; control efectivo de red y archivos; correspondencia entre fuente recibida y entrada del modelo; consumo y latencia medidos; evaluación independiente de contenido y forma. El tamaño de almacenamiento y la compatibilidad declarada no sustituyen esas comprobaciones.

[Volver al expediente](../readme.md) · [Criterios de recepción](../evaluacion/CRITERIOS_DE_RECEPCION.md).
