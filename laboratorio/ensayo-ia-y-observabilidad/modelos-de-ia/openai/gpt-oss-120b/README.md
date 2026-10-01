# OpenAI · GPT-OSS-120B ordinario

**Actualización documental:** 1 de octubre de 2026.  
**Estado:** estudio preliminar conservado; instalación, inferencia y aptitud pendientes de acreditación.  
**Vía vigente:** B · ejecución nativa en CPU mediante Rust.

Esta carpeta corresponde exclusivamente a **`openai/gpt-oss-120b`**, de propósito general. La variante especializada dispone de su propia [carpeta GPT-OSS-Safeguard-120B](../gpt-oss-safeguard-120b/README.md). Los resultados, configuraciones y recepciones se mantienen separados.

El objeto es determinar si una configuración identificada del modelo satisface los mínimos de fidelidad documental, cumplimiento de instrucciones, trazabilidad y tiempo de respuesta del ensayo. Se mantienen evaluaciones diferenciadas para inmunología y ciberseguridad. Cargar el modelo o recibir una respuesta HTTP no constituye un resultado Apto.

## Estado y antecedentes

El estudio preliminar del 27/09/2026 concluyó que la configuración entonces examinada no ofrecía memoria suficiente; no se instaló ni se ejecutó el modelo. Los 62,79 GiB documentados eran la memoria del anfitrión, no un máximo de consumo observado del candidato. El [TT-0015](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0015.md) conserva el alcance de esa conclusión. La disponibilidad posterior de un recurso nominal de 128 GB no acredita todavía carga, rendimiento ni aptitud.

La [guía del sistema conjunto](https://github.com/juantoniolloretegea/SV-motor/blob/b8ff9198275ee0139dad7565d23733667aadb066/laboratorio/sistema-conjunto-lenguaje-computacion-ia-gobernada/README.md) distingue las dos variantes: el expediente previo prepara el ordinario y Safeguard permanece como hipótesis con función propia. No consta en ese antecedente una prioridad definitiva de ensayo de Safeguard. El orden de una campaña nueva deberá quedar fijado en su encargo; esta separación documental no inicia inferencias.

## Formato de conversación y cálculo

GPT-OSS requiere el formato **Harmony**. En la composición candidata, mistral.rs implementa la inferencia y utiliza **Candle** para operaciones numéricas; la biblioteca oficial **openai-harmony** interviene en el tratamiento de mensajes. Son funciones complementarias. La revisión exacta, las dependencias compiladas y su compatibilidad efectiva con el 120B permanecen sujetas a cualificación.

El cotejo debe incluir errores y truncamientos, correspondencia de tokenizador y plantilla, vocabulario local identificado, cálculo MXFP4 y controles de red. La compatibilidad conceptual no acredita seguridad ni aptitud del ensamblaje. [Funciones y límites de la composición](../../../README.md#harmony-candle-y-funciones-del-conjunto) · [Harmony oficial](https://github.com/openai/harmony).

## Documentación del modelo

- [Ficha técnica y condiciones de selección](FICHA_TECNICA.md).
- [Estado estructurado](ESTADO.json).
- [TT-0015](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0015.md), vinculado a [S39](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md#s39).
- [Acta 004, continuación del ensayo](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/continuacion-15-09-2026/ACTA_004_FINALIDAD_ALCANCE_Y_CONTINUIDAD_EIO_2026_09_22.md).

Se conserva la organización por modelo de [gpt-oss-20b](../gpt-oss-20b/README.md): ficha, fuentes, resultados por campaña y distribución identificada cuando exista. El nuevo candidato no hereda sus pruebas, binarios ni dictámenes. Todavía no hay instalación, imagen o release de GPT-OSS-120B acreditada en este ensayo.

La inferencia futura utilizará fuentes locales admitidas. El MCP documental permanece como componente experimental conservado; no atribuye verdad a una respuesta ni constituye por sí mismo el Árbitro SV. La incorporación al núcleo no forma parte de esta selección.

La vía A —inferencia en navegador mediante WebAssembly— queda diferida hasta que el candidato obtenga **Apto en el alcance experimental definido** y se autorice un ensayo específico de esa vía. Ese resultado tampoco equivaldría a aptitud clínica general o habilitación productiva.

[Índice OpenAI](../README.md) · [Catálogo de modelos](../../README.md) · [Ensayo](../../../README.md).

---

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es). Los componentes de terceros conservan sus licencias; véase la ficha.
