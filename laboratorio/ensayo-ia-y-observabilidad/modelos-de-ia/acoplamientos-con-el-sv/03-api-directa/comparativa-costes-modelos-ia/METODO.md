# Método común del ranquin

Versión 1.0 · 08/10/2026.

## Objeto y población

Se comparan cuatro modelos determinados del nodo 03: GPT-6 Astra, Grok 4.7, Qwen3.8-Max-0902 y GLM-5.3. No se cambia la versión para mejorar una posición. El ranquin describe el **coste medio por tarea de una referencia externa**, en el corte fechado de sus fichas principales. No declara un ganador universal ni traslada al SV la calificación de un tercero.

## Criterio aplicado a todos

1. Misma fuente de evaluación: Artificial Analysis, índice v4.3.2. Misma definición de coste ponderado por tarea para los cuatro.
2. Misma selección documental: entrada principal de la ficha del modelo exacto. Se conserva el ajuste declarado, incluida la ausencia de etiqueta de esfuerzo para Qwen. No se optimiza cada configuración después de ver sus resultados.
3. Misma unidad monetaria: USD por tarea, sin convertir saldos, promociones o cuotas de una cuenta en precios comerciales.
4. Orden ascendente por coste; empate si coincide la cifra publicada. No se añade un desempate favorable a un proveedor. Para capacidad y velocidad se ordena en sentido descendente; para demora, ascendente. Los empates ocupan el mismo puesto y el siguiente puesto refleja cuántos modelos lo preceden.
5. Calidad, coste y tiempo permanecen visibles como magnitudes distintas. No se ponderan por una preferencia no declarada ni se transforma un índice compuesto en probabilidad de acierto.
6. Un dato ausente se conserva como ausente. Una configuración o fuente incompatible impide su inclusión en ese orden; no se completa con cero ni con datos de otra versión.

Esta igualdad de método no significa igualdad de recursos de cómputo ni de ajuste interno. El índice externo agrega agentes, programación, razonamiento científico y tareas generales; utiliza evaluaciones con herramientas y fuentes diferentes de un examen documental cerrado del SV. [Metodología de capacidad](https://artificialanalysis.ai/methodology/intelligence-benchmarking).

## Cómo interpretar la relación capacidad/coste

Una comparación multidimensional puede favorecer un modelo en capacidad y otro en coste o demora. El gráfico conserva esa información. La prioridad económica adoptada en esta carpeta es explícita y revisable: «primero, menor coste por tarea». No se denomina a esa prioridad un estándar universal del mercado.

Las métricas de capacidad, precio por token, coste por tarea, velocidad y latencia están documentadas por la fuente escogida. Los datos de capacidad y coste del índice y las pruebas de rendimiento de API no constituyen una misma ejecución. La metodología externa puede estimar componentes de caché a partir de mediciones representativas. Los resultados aquí son una comparación documental, no una auditoría de sus registros internos. [Definiciones](https://artificialanalysis.ai/methodology) · [Rendimiento de API](https://artificialanalysis.ai/methodology/performance-benchmarking).

No se publican intervalos propios de incertidumbre porque no se dispone de las repeticiones necesarias. Un empate del índice redondeado no prueba igualdad real; una diferencia pequeña tampoco acredita significación estadística. Las páginas pueden actualizarse: una nueva versión exige un nuevo corte completo de los cuatro modelos y conservación de la edición anterior.

## Continuidad con el informe de costes anterior

El [informe público](INFORME.md) conserva las tarifas y ejemplos comparables y añade las mediciones externas. El archivo privado continúa custodiando la contabilidad, las fuentes de consumo propias y las conciliaciones. No se trasladan saldos, credenciales, identificadores de cuenta o justificantes a esta carpeta pública. La publicación de una referencia de mercado no incrementa las pruebas ni los tokens del candidato.

## Condiciones de una futura comparación propia del SV

La unidad comparada deberá definirse antes de la ejecución: mismo contrato, contenido y huellas; mismas preguntas, criticidades, etapas, tiempo y presupuesto autorizados. Cada candidato recibe las fuentes completas y sus propias respuestas anteriores, sin conocer la clave, puntuaciones o respuestas de otros candidatos. Las herramientas disponibles y el límite de acceso a Internet se acreditan desde el suministro y transporte del SV.

El Árbitro-Director conserva la decisión. La instrumentación Rust registra intentos, recepción completa o interrupciones, contadores del proveedor, medidas propias y lagunas. Las revisiones R0, R1 y R2 se contabilizan juntas; la respuesta final no oculta costes o errores de etapas previas. Los ajustes específicos de razonamiento se declaran y no se equiparan sólo por nombre.

El coste por respuesta final válida requiere número acreditado de respuestas conformes y costes atribuibles de todos los intentos del mismo contrato. Si falta liquidación se distingue coste estimado de confirmado; si no hay respuestas válidas, no se publica coste por acierto igual a cero. La admisión exige previamente las condiciones críticas y documentales del SV. Una prueba económica no sustituye recepción científica ni acredita aptitud clínica u operativa.

## Reproducibilidad e integridad

[DATOS.json](DATOS.json) conserva sólo las magnitudes utilizadas, identificadores de modelo, ajustes, fuentes, fecha y reservas. No es una copia de las evaluaciones de terceros. El [programa Rust](calculo-rust/src/main.rs) ordena esas magnitudes, conserva empates y ausencias, calcula el precio ponderado y genera el gráfico. Sus pruebas cubren precisamente los casos que podrían falsear la comparación. El [manifiesto](MANIFIESTO.json) identifica los archivos publicados por bytes y SHA-256; la custodia requiere recuperar y cotejar la revisión remota.

Los datos y métodos atribuidos a terceros conservan sus derechos. La licencia del documento propio no se atribuye a sus páginas, software o conjuntos de evaluación.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
