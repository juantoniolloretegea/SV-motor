# Método y procedencia de las comparaciones

Versión 1.1 · 08/10/2026.

## Clases de evidencia

| Clase | Ejemplos | Qué acredita |
|---|---|---|
| Observación del SV | Duración desde el cliente, recepción, interrupción, respuesta conservada y evaluación documentada | Lo observado por el instrumento o evaluador dentro de su alcance |
| Dato comunicado por el proveedor sobre una ejecución SV | Tokens de entrada, salida y caché; importe atribuido a una solicitud | Contador o cargo comunicado; no una medición independiente de sus sistemas |
| Cálculo derivado | Sumatorios, coste estimado, clasificación por un campo | Resultado reproducible sobre entradas identificadas; no una liquidación |
| Tarifa comercial | Precio publicado por millón de tokens | Regla de precio aplicable bajo sus condiciones; no consumo medido |
| Evaluación externa | Índice, coste por tarea o latencia de Artificial Analysis | Resultado de un tercero bajo su procedimiento |

La procedencia se asigna a cada magnitud: una misma fila puede contener medición local, contador del proveedor y estimación. Ausente, comunicado, estimado y conciliado son estados distintos. La ausencia nunca se rellena con cero.

## Evidencia propia y límites de clasificación

Se cotejan primero los ensayos ya conservados. Los [resultados por ensayo](RESULTADOS-SV.md) y las [tablas publicables de consumo](evidencia-sv/PROCEDENCIA.md) separan contenido científico y situación económica. Se publica lo necesario para reproducir las cifras sin cuentas, credenciales, medios de pago ni dependencias de lectura privadas.

Los 363 registros proceden de un índice contrastado; pueden reunir varios intentos y tres carecen de contadores completos. Sólo el subconjunto CYB16 de 48 registros cuenta además con la recuperación y cotejo individual realizada para esta edición. No se extiende ese alcance a expedientes no examinados.

El cotejo temporal utiliza MD01-R0: mismo corpus, pregunta y contrato sustantivo, una ejecución por candidato. Se conserva el tiempo hasta el primer evento de texto registrado. **El origen de los relojes y las solicitudes completas difieren**; por tanto, se muestran las observaciones sin puestos y no se presenta un ranquin propio de latencia. El [cotejo](COTEJO-MD01.md) conserva las diferencias y los enlaces al instrumento. No se descuentan duraciones desconocidas mediante supuestos.

La selección del subconjunto es retrospectiva y no dispone de repeticiones ni intervalos de incertidumbre. R1 y R2 incorporan respuestas previas diferentes: se publican como complemento y no se agregan para fabricar un orden global. R2 de Grok no se envió. CYB16 y P13 no se incluyen en el orden MD01 porque son contratos distintos.

## Condiciones para una clasificación económica propia

Una comparación monetaria requiere la misma unidad de trabajo y calidad admisible, contabilidad de todos los intentos atribuibles y una modalidad económica identificada. El volumen de tokens de diferentes segmentadores no constituye por sí mismo una unidad lingüística común.

El coste por respuesta final conforme necesita un denominador acreditado y el coste de todas las etapas, intentos e interrupciones del mismo contrato. Si no hay respuestas conformes, el cociente no se presenta como cero. Si falta liquidación, una estimación conserva esa condición. Los costes de ensayos distintos no se ordenan como equivalentes.

Cuando el cotejo de evidencia existente no baste, una comparación posterior deberá definir antes de ejecutarse corpus, huellas, preguntas, criticidades, etapas, reloj, límites y ajustes efectivos. Cada candidato recibirá sus propias respuestas anteriores, sin conocer claves ni respuestas de otros. La preparación metodológica no autoriza inferencias ni gasto.

El Árbitro-Director conserva la decisión. Un error crítico no se compensa con ventajas de coste o demora. La conformidad instrumental tampoco sustituye la recepción científica ni acredita aptitud clínica u operativa general.

## Orden externo de Artificial Analysis

La referencia externa mantiene cuatro versiones exactas: GPT-6 Astra, Grok 4.7, Qwen3.8-Max-0902 y GLM-5.3. Se aplica la misma definición de coste medio ponderado por tarea del índice v4.3.2, en el corte conservado del 08/10/2026.

Se utiliza la entrada principal de cada ficha: Astra max, Grok xhigh, GLM max y Qwen con razonamiento sin etiqueta de esfuerzo equivalente. No se escoge retrospectivamente el ajuste más favorable. Igual nombre de ajuste no prueba igual cómputo.

El orden económico es ascendente en USD por tarea; capacidad y velocidad, descendentes; demora, ascendente. Los empates ocupan el mismo puesto y el siguiente refleja cuántos modelos lo preceden. Coste, calidad y tiempo permanecen separados, sin pesos arbitrarios. El índice compuesto no se transforma en probabilidad de acierto.

Una tarea evaluada puede ser incorrecta: coste por tarea no significa coste por respuesta correcta. Los ensayos del índice y las mediciones de servicio de API no son la misma ejecución. Las cifras redondeadas no acreditan significación estadística. No se dispone de los registros internos del evaluador: esta publicación reproduce su orden documental, no audita su experimento.

Fuentes metodológicas: [definiciones](https://artificialanalysis.ai/methodology), [índice de capacidad](https://artificialanalysis.ai/methodology/intelligence-benchmarking) y [rendimiento de API](https://artificialanalysis.ai/methodology/performance-benchmarking).

## Tarifas y cálculo convencional

El informe conserva los precios comerciales y el ejemplo uniforme de 1.000.000 tokens de entrada ordinaria y 100.000 de salida. La mezcla externa 7:2:1 —caché, entrada ordinaria y salida— es otra comparación convencional; no acredita una tasa real de caché del 70 %.

La caché es un subconjunto de la entrada; el razonamiento facturable incluido en la salida no se suma otra vez. La tarifa específica de caché de Qwen continúa sin confirmación oficial para el supuesto utilizado: el precio externo de 0,25 USD/M se distingue del campo oficial ausente. No se equiparan tarifas, promociones o cuotas a importes liquidados.

## Reproducción, integridad y actualización

[DATOS-SV.json](DATOS-SV.json) identifica la evidencia temporal propia. [DATOS.json](DATOS.json) conserva magnitudes externas, configuraciones, fuentes, fecha y reservas. Las tablas económicas depuradas incorporan ausencias y grado de conciliación.

Los [programas Rust](calculo-rust/REPRODUCCION.md) reproducen las clasificaciones, sumas, estimaciones y huellas. La aritmética conforme no certifica los contadores internos, las facturas, las evaluaciones externas ni la aptitud científica de un modelo.

El [manifiesto](MANIFIESTO.json) identifica bytes y SHA-256 de los archivos publicados, excluyéndose a sí mismo. La revisión remota se recupera y coteja antes de declarar la publicación recibida. Cada actualización conserva su fecha y el [historial](HISTORIAL.md); no se reescriben hitos previos como observaciones nuevas.

Los datos y métodos de terceros mantienen sus derechos. La licencia del documento propio no se atribuye a sus páginas, programas o conjuntos de evaluación.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
