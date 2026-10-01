# Ficha técnica · GPT-OSS-Safeguard-120B

**Fecha:** 01/10/2026. **Estatuto:** instalación acreditada; contraste corregido en ejecución, recepción independiente pendiente.

## Instalación y contraste autorizado · corte de 01/10/2026

Safeguard instalado y carga completa de 36 capas acreditada en el recurso existente de 128 GB y 32 CPU. Se mantienen 25 pruebas Rust MCP, nueve MXFP4 y una prueba específica de terminaciones Harmony favorables. Se han preservado los intentos instrumentales y corregido el cálculo CPU de cuatro expertos y la terminación prematura de los canales. El contraste sintético de ocho casos continúa con revisión identificada; todavía no hay respuestas válidas adjudicadas ni recepción independiente. Instancia y accesos conservados; sin intervención en Qwen.

| Elemento efectivo | Evidencia y límite |
| --- | --- |
| Identidad | openai/gpt-oss-safeguard-120b, revisión 3c7391182603991a904031244e7822488c67796d; 27 artefactos cotejados. |
| Cálculo | MXFP4 original en expertos, restantes F32; CPU/Rust, mistral.rs v0.9.4 con revisión e instrumentación conservadas. |
| Ejecución corregida | SHA-256 b1a229a29b872fc6ba432f2dcbc16a378b3ae8767b5344f484006a6a684e9615. Cuatro expertos por token y terminaciones Harmony comprobados. |
| Recursos | Recurso existente de 128 GB y 32 CPU; intercambio cero. Máximo conjunto de intentos cerrados hasta contraste-04: 93.550.489.600 bytes. La medición final del contraste corregido permanece pendiente. |
| Aislamiento | Lectura de caché autorizada, bloqueo de red y de rutas externas, custodia separada; pruebas bajo las identidades efectivas. |
| Seguimiento | TT-0018 y S39 revisión 30; TT-0014 mantiene recepción propia pendiente. |

[Avance y originales, acceso restringido](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/926542570f74760296898f8e3fa8b432a68b8218/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-01/AVANCE-INSTALACION-20261001.md) · [Correcciones y pruebas, acceso restringido](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/926542570f74760296898f8e3fa8b432a68b8218/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-01/CORRECCIONES-INSTRUMENTALES.md). La tabla de preparación siguiente conserva su fecha como antecedente; sus campos pendientes quedan actualizados por este corte material. No se atribuye aptitud clínica ni recepción favorable.

## Antecedente de preparación documental

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
