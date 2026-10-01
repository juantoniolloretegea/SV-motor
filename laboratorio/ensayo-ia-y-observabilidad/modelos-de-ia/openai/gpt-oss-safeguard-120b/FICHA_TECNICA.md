# Ficha técnica · GPT-OSS-Safeguard-120B

**Fecha:** 2026-10-01T14:49:26Z. **Estatuto:** instalación y contraste inicial terminados; recepción independiente pendiente.

Instalación y contraste sintético inicial de Safeguard terminados: 87,5/100; No apto para el contraste delimitado; un incumplimiento crítico de lectura íntegra en C08; cobertura 8/8. Las ocho clasificaciones y citas son correctas respecto de los pasajes recibidos, pero C08 omite la página 0 de una sección de dos páginas. Carga completa, aislamiento probado y auditoría íntegra de C01-C07 (r3) y C08 (r4) conformes; 3673 tokens y cálculos finalizados, diez emisiones conservadas. Máximo conjunto: 111.693.459.456 bytes, sin intercambio ni agotamiento de memoria. Inferencia cerrada; servidor, pesos y accesos conservados. Entrega original y archivos cotejados desde GitHub; recepción independiente pendiente. No acredita aptitud clínica ni modifica otros expedientes.

| Elemento efectivo | Resultado y límite |
| --- | --- |
| Modelo | openai/gpt-oss-safeguard-120b, revisión 3c7391182603991a904031244e7822488c67796d; 27 archivos cotejados. |
| Cálculo | MXFP4 original en expertos, restantes F32; CPU/Rust; mistral.rs 0.9.4, revisión 4400935451da5e2dc7379a3f92fbbada66557f6c con correcciones conservadas. |
| Realizaciones | r3 para C01-C07 y r4 para C08. Mismo parche numérico; huellas y alcance en ESTADO.json y entrega original. |
| Recursos | Recurso existente de 128 GB y 32 CPU. Cota conjunta 114 GiB; máximo 111.693.459.456 bytes; intercambio y OOM cero. |
| Puntuación y dictamen | 87,5/100; N₀=7, N₁=1, Nᵤ=0; un error crítico; No apto para el contraste delimitado; cobertura 8/8. |
| Regla SV | κ y T(n) no aplicables: n=8 no es célula exacta n=b², b≥3. La criticidad determina el dictamen del contraste. |
| Recuperación | C08 omite la página 0 de una sección de dos páginas. Etiqueta y cita correctas, lectura íntegra incumplida. |
| Auditoría y cierre | Entradas, ambas fronteras MCP, 3673 cálculos/tokens, diez emisiones y telemetría cotejados. Inferencia cerrada; servidor y accesos conservados. |
| Seguimiento | S39 revisión 31 y TT-0018; recepción independiente pendiente. TT-0014 conserva recepción propia y antecedentes. |

[Prueba registrada](tests-y-pruebas-efectuadas/INSTALACION-CONTRASTE-20261001.md) · [Informe original](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/9b23fc5f59d602af8f31ed8666096c0d65471b2d/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-01/INFORME-FINAL.md) · [Puntuación íntegra](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/9b23fc5f59d602af8f31ed8666096c0d65471b2d/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-01/PUNTUACION-FINAL.json) · [Custodia cotejada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/9b23fc5f59d602af8f31ed8666096c0d65471b2d/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-01/COTEJO-CUSTODIA.json).

La clasificación con pasajes aportados, la autonomía, la viabilidad y la conformidad instrumental se valoran separadamente. El banco sintético no acredita aptitud clínica ni permite comparación directa con bancos distintos. La preparación siguiente conserva su condición histórica y queda actualizada por el corte material anterior.

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
