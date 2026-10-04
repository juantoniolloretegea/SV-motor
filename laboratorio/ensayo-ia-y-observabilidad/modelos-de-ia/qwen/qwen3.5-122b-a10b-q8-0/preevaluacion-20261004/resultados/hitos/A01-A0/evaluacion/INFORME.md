# Primera medición documental: A01 de A0

Encargo QWEN35-PRE-20261004/r1. Corte: 4 de octubre de 2026, 22:51:52 UTC. Candidato Qwen3.5-122B-A10B Q8_0. Resultado del caso: **0, correcto y completo**. Es una adjudicación externa de ejecución; la recepción independiente permanece pendiente. Un caso no constituye una capa completa ni acredita acceso al examen o aptitud clínica.

## Respuesta y fundamento

La respuesta original clasifica como RESPALDADA la apertura del archivo Liria el miércoles a las 08:30. El pasaje DA01/S1/0 fija apertura de martes a sábado, de 08:30 a 13:30. La continuación DA01/S1/1 mantiene ese horario para atención presencial y no añade aperturas extraordinarias. La respuesta conserva día y hora, aporta cita literal válida, aplica D1 y D5, declara ambas páginas recibidas y contiene los seis campos requeridos. La revisión es null, conforme a la condición inicial.

Se han leído la respuesta final y el razonamiento emitido íntegros. La adjudicación sustantiva se conserva en ADJUDICACION-EXTERNA.json y se integra mediante Rust. El razonamiento emitido se conserva por separado y no se convertirá en antecedente de otra solicitud. Sin nueve adjudicaciones válidas no se declara κ ni puntuación completa. Los ocho casos restantes aún no ejecutados quedan fuera de la terna; no son U.

## Observación temporal y consumo

| Magnitud | Medición |
|---|---:|
| Entrada efectiva | 2121 tokens |
| Procesamiento de entrada, medición nativa | 391,886 s |
| Primera emisión observable | 391,951540063 s |
| Primer contenido final | 7185,565369052 s |
| Generación, medición nativa | 7839,775 s |
| Duración nativa total | 8231,661 s |
| Solicitud y recepción completa | 8232,722806332 s |
| Consumo contabilizado, incluido control | 8235,166101177 s |
| Salida nativa total | 1691 tokens |
| Velocidad nativa de generación | 0,21569496 tokens/s |
| Razón emitida / final | 5019 / 618 bytes |
| Mayor intervalo observado sin progreso nativo | 391,076121606 s |
| Memoria máxima muestreada del modelo | 225135276032 bytes |
| Incremento de eventos del límite suave | 79; de 1503 a 1582 |
| Eventos de límite duro, OOM e intercambio | 0 |

La API nativa declara el total de tokens de salida, pero no entrega sus identificadores ni el reparto nativo exacto por canal. La recodificación externa del texto con el tokenizador recibido produce 1464 tokens de razonamiento y 225 de final: 1689 en conjunto. Es una medida reproducible del texto emitido, no los identificadores originales de generación. La diferencia de dos respecto del total nativo no se atribuye a un canal sin evidencia. Los textos y todos los fragmentos emitidos están conservados íntegramente. La observación del contador de entrada tiene una resolución nominal de cinco segundos; no sustituye a la temporización interna nativa.

La solicitud termina con HTTP 200, stop y señal final completa. El servicio del modelo permanece activo, con cero reinicios, cero secuencias en ejecución y cero en espera en el corte. La unidad de control ha terminado con código cero. El diario del servicio en journald está vacío porque ambas salidas se redirigen al archivo nativo del modelo: se conserva una copia íntegra de ese archivo hasta A01 y el registro nativo de finalización. No se presenta el diario vacío como evidencia de ausencia de actividad.

## Adecuación y escenarios prospectivos

A01 ha completado el contrato, sin truncamiento ni incidencia instrumental observada, dentro de 18000 segundos por solicitud y 1800 sin progreso nativo. Su duración es una limitación operativa importante. El presupuesto consumido es 8235,166101177 de 86400 segundos; quedan 78164,833898823 segundos. La instalación ya estaba cargada al comenzar y no se ha efectuado otra carga.

MEDICION-RUST.json calcula escenarios externos a partir de esta única muestra. Dieciocho solicitudes de tamaño y duración equivalentes requieren aproximadamente **41,164 horas**. Las 72 generaciones máximas teóricas, con historiales representativos crecientes, requieren unas **166,751 horas** si se mantiene el tamaño de salida, o **284,347 horas** si éste se duplica en las revisiones. Todos exceden las 24 horas autorizadas. No son pronósticos validados: se desconoce la duración y longitud de las respuestas futuras, y el máximo de 72 no es un objetivo de consumo.

Los escenarios de entrada inicial y con uno, dos y tres antecedentes representativos contienen 2121, 2503, 2885 y 3267 tokens. Sumando salida máxima y reserva ocupan 8265, 8647, 9029 y 9411, respectivamente, dentro de 32768. Los antecedentes de esas simulaciones son repeticiones externas del final de A01 únicamente para medir tamaños; jamás se suministran al candidato ni se presentan como respuestas reales posteriores.

La preparación, revisión y publicación se registran fuera del presupuesto de carga/inferencia. La continuidad documenta recepción del encargo a las 19:50:39,7598094 UTC; A01 se admitió a las 20:18:14,4344637 UTC. Este intervalo documentado de 1654,6746543 segundos no pretende medir preparación anterior a ese hito. Desde dicho hito hasta el corte de las 22:51:52 UTC transcurren 10872,2401906 segundos de campaña observada; no se confunden con tiempo de inferencia ni se atribuye todo el resto a cálculo activo de preparación.

Se admite la continuación ordenada por A02: hay evidencia íntegra, adjudicación del primer caso y margen superior a la cota completa de otra solicitud. Se conservan los límites prospectivos; la estimación temporal no los amplía ni introduce retrospectivamente un nuevo criterio de parada. Antes de cada admisión se volverá a exigir margen para 18000 segundos. Si el límite conjunto impide completar las adjudicaciones necesarias, el cierre será «Admisión no acreditada por impedimento» con la causa temporal delimitada, sin extrapolar incapacidad general del modelo.

## Integridad y límites de recepción

Los 29 archivos originales, 58265110 bytes, y nueve complementos, 377445 bytes, recuperados del servidor coinciden con los manifiestos calculados allí mediante Rust. Trece comprobaciones del auditor externo han pasado después del cierre de A01; la auditoría real confirma entrada, correspondencia de los canales y emisión original, citas, recepción documental y diarios encadenados. Los registros contienen nueve intercambios MCP, 1692 entradas de salida y 1643 muestras de telemetría.

El conductor de inferencia no se ha sustituido. Las utilidades de medición y manifiestos se compilaron y utilizaron después del cierre, con el modelo sin solicitudes activas. Núcleo, semántica V0.2, IR 0.3 y pesos permanecen intactos. Cero reparaciones instrumentales consumidas y ninguna repetición de A01.

Huellas SHA-256 principales: final `42dab6fa22c105393981d52833cc29eb887364350ccac4caeee39f503d333762`; razonamiento `da5d68b886c7553dab70d96dd5218b0f8e6291fc11d252ffa085737a9bb99971`; emisión original `f5c98956662864f367c7673a1aebaf4ff6ec4f6f0d523893ac62cc84a5ff1f62`. Los manifiestos y cotejos detallan los demás archivos. La custodia en GitHub de este hito se acreditará mediante su referencia inmutable y recuperación cotejada; este informe por sí solo no la acredita.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
