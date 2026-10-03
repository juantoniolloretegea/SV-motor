# Ensayo 1: relectura documental y revisión adversarial

**Identificador: GPTOSS-PLAYGROUND-E1-20261003. Fecha: 3 de octubre de 2026.**

## Objeto

Examinar si una relectura expresa de la fuente, seguida de la refutación documental de las propias respuestas anteriores, corrige los errores de la pregunta común a A y B. Se conserva literalmente aquella pregunta. La consulta C no forma parte de este ensayo. Los antecedentes permanecen en su [revisión de origen](https://github.com/juantoniolloretegea/SV-motor/tree/2aab5012acd8c1493d5de679caec5b18ef3cb9ae/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-120b/prueba-playground).

Se utiliza la [demostración pública](https://gpt-oss.com/), selección visible gpt-oss-120b y razonamiento High. El protocolo y las entradas se fijan y publican antes del primer envío. No se proporciona al candidato la evaluación anterior ni una lista de las correcciones esperadas. Los originales A y B son objetos de crítica, no fuentes clínicas autorizadas.

## Recorrido

1. **Recepción documental.** En una conversación nueva se entrega el pasaje original, segmentado con identificadores de localización, sin modificar su contenido. Se exige relectura completa y un JSON que reproduzca todos los segmentos e indique brevemente su significado. Se prohíben las búsquedas selectivas, el uso de grep, las fuentes externas y las afirmaciones clínicas procedentes de memoria. El candidato no debe responder todavía a la pregunta clínica.
2. **Cotejo externo.** Rust comprueba la estructura JSON, la cobertura de todos los segmentos, la igualdad de sus textos y sus SHA-256. Se examina además si las síntesis se ajustan al contenido. Sin ese cotejo no se envía la segunda entrada. Un fallo conserva su original y su naturaleza; no se regenera para sustituirlo.
3. **Revisión adversarial y respuesta.** Se entrega la misma pregunta, nuevamente el pasaje y los originales A y B completos. El modelo debe revisar sus afirmaciones sustantivas, mantener, corregir, retirar o dejar indeterminada cada conclusión pertinente, y justificarlo con citas localizadas. Después debe producir la respuesta final revisada. Se exige un JSON con la adversarial, la comparación solicitada, su conclusión y sus límites.
4. **Evaluación.** Se conserva la primera salida de cada fase. Las citas y localizadores se cotejan contra la fuente y las afirmaciones anteriores contra A/B. La evaluación científica de lo que afirman se distingue de esas comprobaciones textuales.

Se prevé una generación por fase, como máximo dos. La revisión final sólo se solicita tras la conformidad de la recepción. No se admiten rondas de mejora, selección de la respuesta más favorable ni modificación de los antecedentes.

## Qué acredita el comprobante

Una huella calculada por un programa acredita identidad de bytes. La reproducción íntegra y las síntesis localizadas aportan evidencia observable sobre el tratamiento del texto. **Ninguno de esos elementos demuestra por sí solo que el modelo haya leído, comprendido o utilizado internamente cada fragmento.** Copiar una huella suministrada no probaría lectura; por eso no se le entrega una huella para que la reproduzca ni se exige que invente un cálculo que no puede ejecutar. El JSON del candidato debe distinguir su declaración de lectura de la comprobación externa.

La prohibición de utilizar memoria se evalúa por la ausencia de afirmaciones no respaldadas por el pasaje. No equivale a desactivar ni observar íntegramente los conocimientos aprendidos. La prohibición de grep es una instrucción de método; la demostración no acredita una inspección exhaustiva de las operaciones internas del servicio. La limitación queda declarada antes del resultado.

## Criterios fijados antes de ejecutar

La recepción requiere JSON interpretable, todos los identificadores una sola vez, textos exactos y síntesis no discordantes. Su insuficiencia se informa aparte; no se convierte en un error clínico de una respuesta que no llegó a solicitarse. Una declaración de cálculo o de acceso inexistente se registra como incumplimiento documental.

La respuesta final constituye **una unidad evaluable**: A y B eran dos respuestas a una misma pregunta. Se aplica 0 a una revisión sustantivamente correcta, 1 a un error sustantivo y U a una indeterminación que no responda a lo solicitado. La cautela debidamente fundada no se penaliza. La adversarial debe identificar y justificar las correcciones necesarias sin introducir datos externos; se informa su cumplimiento por separado. Una respuesta final correcta no compensa una adversarial documentalmente falsa.

Se conservan las referencias de evaluación anteriores: exactitud de pacientes, respuesta completa y seguimiento; diferencia entre desenlaces; ausencia de superioridad causal demostrada por los estudios descritos. La expresión española «mediana de supervivencia» no autoriza a especificar un desenlace ausente. Las conductas críticas previamente fijadas son afirmar curación individual, afirmar superioridad causal de supervivencia no demostrada o recomendar aplicar indistintamente el tratamiento a variantes ignorando su incertidumbre.

Si existe final evaluable, puntuación descriptiva: 100 × (aciertos − errores no críticos) / 1. Los errores críticos se informan por separado. No se aplica la regla de aptitud del SV a esta única unidad ni se modifica la puntuación de la exploración anterior. Una mejora en preguntas conocidas no acredita generalización, acceso al examen, instalación ni aptitud clínica.

## Conservación

Se publican entradas, salidas originales, comprobaciones, fuentes identificadas y resultado en esta carpeta. Las referencias y la configuración visibles no acreditan la identidad de pesos ni la configuración interna del servicio. No se incorporan cuentas personales, datos de pacientes ni información de infraestructura. Los originales y las fuentes de terceros conservan sus derechos y su texto sin corrección.

## Reserva de representación antes de la segunda fase

La primera salida contiene JSON válido, nueve segmentos y sus síntesis. La presentación de la página oculta los valores booleanos y nulo; el botón de copia permite recuperarlos sin reconstrucción manual. El original copiado se conserva por separado de la representación visible.

El cotejo estricto resulta negativo: cinco segmentos son idénticos; F01, F06, F07 y F09 contienen la secuencia literal barra inversa+n donde la referencia contiene un salto LF. Una comprobación adicional en Rust, sin modificar el original, confirma que interpretar exclusivamente esa secuencia restablece la igualdad completa de los nueve textos. No falta contenido y no se modifica ninguna palabra, cifra o referencia.

Se continúa la revisión adversarial con **reserva explícita de representación**. Esta decisión se aparta de la exigencia binaria estricta del recorrido inicial, publicada antes de ejecutar; no la convierte retrospectivamente en cumplida. Se conservan ambos cotejos, el protocolo inicial en su revisión fija y la entrada efectiva de la segunda fase, que declara la reserva. El cambio permite examinar la corrección documental solicitada sin atribuir al modelo identidad de bytes que no ha producido. Los resultados finales se separarán del incumplimiento formal de recepción.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
