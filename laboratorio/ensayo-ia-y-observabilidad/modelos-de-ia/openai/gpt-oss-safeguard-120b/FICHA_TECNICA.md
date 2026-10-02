# Ficha técnica · GPT-OSS-Safeguard-120B

**Corte vigente:** L01-ARBITRO = 0; condición asistida conforme, 100/100 para N=1. Inferencia cerrada; recepción independiente pendiente. Dictámenes históricos conservados.

**Fecha:** 2026-10-02T06:29:40Z. **Estatuto:** condición asistida terminada; alcance conocido y recepción pendiente.

## Antecedente conservado: instalación y contraste inicial

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

## Contraste posterior de recuperación documental íntegra · 2026-10-01T15:55:53Z

Contraste de recuperación documental íntegra Safeguard v2 autorizado expresamente y en ejecución, con banco nuevo SG-LECTURA-INTEGRA-20261001/r1 y exactamente tres casos autónomos. Corpus, afirmaciones, referencias reservadas, criticidad y configuración fijados antes de inferir. Contrato de páginas aclarado en la representación efectiva y observador pasivo incorporado; quince pruebas del conductor y veinticinco del MCP conformes. Tres rutas documentales completas comprobadas sin inferencia, con reserva mínima observada de 3869 tokens. Carga completa acreditada; primera secuencia en curso. El resultado anterior de 87,5/100 y C08 se conservan intactos. La nueva puntuación y el dictamen quedan pendientes del cierre y cotejo íntegros.

[Comprobaciones y admisión](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/b8db5c30c94863bc4fd95f050c9ae0b56f5d25d0/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-02/ADMISION-PREVIA.json). S39, revisión 32, conserva el seguimiento. La recepción independiente y las dependencias de los demás expedientes permanecen pendientes según sus propios registros. La aclaración de MCP y sus pruebas no constituyen una recepción integral de TT-0014 ni aptitud clínica.

### Reserva metodológica y avance del contraste de lectura íntegra · 2026-10-01T17:14:10Z

La secuencia v2 conserva dos finales sin repetición: L01 clasifica correctamente y cita la restricción, pero omite la página 1 exigida; adjudicación provisional 1 crítico. L02 lee las dos páginas, clasifica correctamente y cita regla general y excepción; adjudicación provisional 0. L03 está en ejecución con contexto nuevo. La adjudicación definitiva y la puntuación propia requieren el cotejo íntegro de cierre. Una revisión estática posterior a la fijación detecta un defecto del banco: consultas dirigidas pueden revelar cláusulas decisivas en fragmentos de 220 caracteres. Se rectifica la afirmación excesiva del protocolo, se conserva el banco sin cambios y se documenta la reserva metodológica. Esto impide declarar conformidad integral del diseño; no se corrige repitiendo resultados ni ampliando la campaña. El resultado previo de 87,5/100 y C08 permanecen intactos.

[Reserva y prueba reproducible](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/bb9158c52c1c21de691745d79ad21b9969286f73/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-02/RESERVA-DE-DISENO.md). S39, revisión 33. Se continúa únicamente L03 bajo las guardas vigentes; después corresponden cierre, custodia, adjudicación propia y recepción independiente. Esta actualización no cierra S39 general ni la recepción propia de TT-0014.

## Cierre del contraste de lectura íntegra · 2026-10-01T18:54:25Z

Contraste Safeguard v2 terminado: 66,67/100; No apto para este contraste. L01 conserva clasificación y cita correctas, pero omite la página 1: un 1 crítico. L02 y L03 reciben 0, con lectura íntegra de dos y tres páginas respectivamente; la elipsis explícita de la primera cita de L03 se acepta con cotejo de sus segmentos y conservación del resultado negativo de coincidencia continua. N=3, N₀=2, N₁=1, Nᵤ=0; sin errores no críticos ni impedimentos técnicos. Se conservan 2701 cálculos finalizados, 13 emisiones completas, 26 segmentos de canal, diez llamadas documentales y 4942 muestras de telemetría. Los siete archivos de la edición propia fueron descargados y cotejados en Rust. Se mantiene una reserva de diseño: las búsquedas dirigidas pueden revelar cláusulas decisivas en fragmentos breves; ese atajo no se materializó en el recorrido observado, pero impide declarar conformidad integral del banco. Inferencia terminada, carga deshabilitada y servidor, pesos y accesos conservados. El resultado anterior de 87,5/100 y C08 permanecen íntegros. Recepción científica independiente pendiente; sin aptitud SV general ni clínica acreditada.

[Informe y adjudicación](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/84028c8d2611ed762c11b21dd4d8e6d72e3c9c0b/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-02/INFORME-FINAL.md) · [Puntuación estructurada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/84028c8d2611ed762c11b21dd4d8e6d72e3c9c0b/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-02/PUNTUACION-FINAL.json) · [Custodia cotejada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/84028c8d2611ed762c11b21dd4d8e6d72e3c9c0b/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-02/COTEJO-CUSTODIA.json) · [Cierre material](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/84028c8d2611ed762c11b21dd4d8e6d72e3c9c0b/respuestas-ejecucion/GPTOSS-SAFEGUARD-INSTALACION-20261001/entrega-02/COTEJO-CIERRE.json).

S39, revisión 34; Acta 004 §28. Recepción científica independiente de entrega-02, incluidas adjudicación, elipsis, reserva de diseño y custodia. Mantener la inferencia detenida y preservar servidor, pesos y accesos. Retorno a expediente Safeguard, TT-0018, relación instrumental de TT-0014, S39 y Calidad. Otra campaña requiere su encargo y autorización propios. TT-0014 conserva su recepción propia pendiente. S39 general permanece en ejecución. Los cortes anteriores conservados son antecedentes y no autorizan reiniciar esta secuencia.


## Contraste asistido único L01-ARBITRO · 02/10/2026

L01-ARBITRO terminado: 0, condición asistida conforme, 100/100 con N=1. Una carga y una generación; T1/S1 páginas 0 y 1 incorporadas íntegramente por el Árbitro. Clasificación CONTRADICHA, cita literal y justificación conformes. Cuarenta y siete pruebas Rust previas; núcleo, semántica e IR sin cambios. Custodia, plantilla, tokens y ambas fronteras MCP cotejados: 358 cálculos, una emisión, dos canales y 320 muestras. Cuatro archivos recuperados exactamente desde GitHub. Memoria conjunta máxima 114 GiB, con 24 eventos nuevos de presión, sin intercambio ni OOM. Inferencia cerrada, carga deshabilitada y servidor, pesos y accesos conservados. Se mantienen L01 autónomo, C08, los resultados de 66,67/100 y 87,5/100 y la reserva del banco. No acredita generalización, lectura autónoma ni aptitud clínica; recepción independiente pendiente.

[Informe](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/fc02eee0abfb49b9e653dc21e4540c8467a62e7e/respuestas-ejecucion/ARBITRO-SV-SAFEGUARD-20261002/entrega-01/INFORME-FINAL.md) · [Resultados](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/fc02eee0abfb49b9e653dc21e4540c8467a62e7e/respuestas-ejecucion/ARBITRO-SV-SAFEGUARD-20261002/entrega-01/RESULTADOS.json) · [Custodia recuperada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/fc02eee0abfb49b9e653dc21e4540c8467a62e7e/respuestas-ejecucion/ARBITRO-SV-SAFEGUARD-20261002/entrega-01/COTEJO-CUSTODIA.json) · [Cierre](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/fc02eee0abfb49b9e653dc21e4540c8467a62e7e/respuestas-ejecucion/ARBITRO-SV-SAFEGUARD-20261002/entrega-01/METRICAS-Y-COTEJO-CIERRE.json).

La integración utiliza las operaciones documentales públicas compile_svp y validate_bindings de LIG/0.1, con referencias exactas y bytes obtenidos por MCP. Se aplica la distinción del contrato CYB §§8.1–8.3: contraste documental sin constitución productiva R1. Se rectifica la dependencia indebida de esa constitución; no se modifican el núcleo ni su frontera de autoridad. La corrección semántica se adjudica externamente y no se simula una función canónica de clasificación.

S39, revisión 35; Acta 004 §29. Recepción independiente de L01-ARBITRO, su integración documental, adjudicación y custodia. Mantener la inferencia cerrada; conservar servidor, pesos y accesos. Retorno a expediente Safeguard, TT-0018, relación pertinente de TT-0014, S39 y Calidad. Se conservan las recepciones pendientes anteriores; otra prueba requiere encargo y autorización propios. TT-0014 conserva su recepción propia; S39 general permanece en ejecución.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
