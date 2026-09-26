# Auditoría acotada de GPT-OSS: ejecución, Harmony y calidad

Profesor · Dirección: Juan Antonio · 26 de septiembre de 2026

**Dictamen: hay un defecto de terminación de Harmony demostrado en la ejecución Rust; los errores de contenido persisten en el contraste independiente. No procede dar por resuelto el problema mediante más entrenamiento ni sustituir el motor sin una decisión técnica separada. Llama.cpp se ha usado sólo como comparador.**

## Objeto y límites

Determinar si la ejecución instalada está perjudicando a GPT-OSS-20B y si habilitar razonamiento o contrastar con otro motor corrige dos fallos aportados por el Director. No se ha entrenado, modificado pesos ni integrado un motor nuevo en SV. La muestra consta de cuatro casos, dos de ellos controles nuevos. No estima una tasa general de error, no constituye validación clínica y no permite prometer error cero.

Llama.cpp se utiliza exclusivamente como comparador independiente en C/C++; no forma parte de la aplicación original Rust. Harmony es el formato de conversación empleado en ambas ejecuciones, no un motor alternativo a llama.cpp. No se ha ejecutado Python para esta auditoría.

## Identidades conservadas

| Componente | Ejecución original o auditada | Identificación |
|---|---|---|
| Aplicación original | Rust, eio-conversacion | SHA-256 a54a15c9c709e66ee6c6b14d1102b3df8a73464dc7dc021e07137b5558630c2c |
| Motor original y copias de auditoría | Rust, mistral.rs/Candle | SHA-256 f6ec0bb908e18ced633d66d6a762a915349281bdbb1397991b268fe487bcc418 |
| Fuente cotejada para EOS | mistral.rs upstream | Commit 24dbf5c256f232176ee5949485ba264049407fbe; chat_template.rs instalado idéntico al archivo de ese commit |
| Comparador | llama.cpp, CPU, C/C++ | Release b11193, commit 4e7481175; versión declarada por binario 0.5.0-dev |
| Pesos, compartidos sin copia ni modificación | gpt-oss-20b-MXFP4.gguf, 12 109 566 624 bytes | SHA-256 27cd6c432c7672cb812a92f611cf3ba7bbc35928262bb1e1253ff4ee6ae35901 |

El archivo oficial descargado de llama.cpp es llama-b11193-bin-ubuntu-x64.tar.gz; su SHA-256 def277c3a4f0c5e2ec3b1413971878d7d5e7da7eee64f5120a0cd8f024f5a364 coincide con el digest publicado por GitHub. Esto verifica el artefacto de comparación, no certifica que todas sus operaciones reproduzcan exactamente una implementación de OpenAI.

## Diseño y comparabilidad

Los casos y criterios se fijaron en CASOS-Y-CRITERIOS.json antes del cotejo. La evaluación es documental por el Profesor; no es ciega ni independiente de quien diseñó el ensayo.

- C01: tres preguntas; se exige tres patas, blanco y plátano. Las tres deben ser correctas.
- C02: reo y dos bolas negras; ocultar o destruir sin mostrar la extraída y usar la negra restante para invocar la composición anunciada. No vale inventar una blanca, sustituir bolas o evitar el sorteo.
- C03: extraer y devolver la misma ficha de una bolsa con dos negras; cero blancas.
- C04: todos los A llevan azul y Elena lleva azul; no se sigue necesariamente que Elena sea A.

Banco bruto: mismos pesos, sistema, preguntas, contexto de 4096, temperatura cero y máximo de 1000 tokens; se compara prefijo final con analysis. El presupuesto de analysis incluye el razonamiento. En C01 se verificaron los 185 identificadores de tokens de entrada frente a la exportación original; no se afirma haber cotejado todos los identificadores de todos los casos.

El primer ensayo Rust de analysis agotó el timeout de 600 segundos sin respuesta HTTP completa. Se repitió mediante streaming y límite HTTP de 900 segundos para conservar salida parcial, manteniendo 1000 tokens. El cambio de transporte y tiempo está documentado; no se presenta como repetición idéntica. En Rust, el número de fragmentos SSE no se considera automáticamente el número de tokens.

Cotejo complementario: plantilla chat nativa de llama.cpp, reasoning_effort=medium y 2048 tokens, en C01 y C02. Cambia más de una variable; no permite atribuir causalmente cada diferencia al motor. Un intento de C02 fue interrumpido por decisión del operador y no puntúa.

Los tiempos son observaciones operativas, no un benchmark controlado: llama.cpp reutilizó prefijos en parte de la serie, Rust tenía esa caché deshabilitada y la máquina mantenía el servicio original. La rapidez observada no basta para proponer una sustitución de arquitectura.

## Hallazgos de implementación

### La aplicación solicita directamente el canal final

main-original.rs construye el sufijo <|start|>assistant<|channel|>final<|message|> y rechaza thinking=true. model-original.rs envía el prompt a /v1/completions; el separador de respuesta ignora el argumento _thinking. Por tanto, la interfaz auditada no estaba ejercitando el flujo analysis → final. Esto explica una limitación del ensayo, pero no prueba por sí solo la causa de cada respuesta falsa.

### La configuración de EOS de Rust puede cortar las transiciones de Harmony

En la fuente cotejada, SUPPORTED_ALTERNATE_EOS incluye <|end|>, y HARMONY_ALTERNATE_EOS incluye <|message|>, <|start|> y <|channel|>. calculate_eos_tokens incorpora estos elementos cuando están presentes en vocabulario y plantilla. Son también delimitadores estructurales del formato. C03 y C04 con analysis terminaron con stop sin entregar un canal final.

La fuente instalada coincide byte por byte con el commit upstream indicado: este hallazgo no demuestra una modificación introducida por Watson. Deben distinguirse el comportamiento de la dependencia y la decisión de la aplicación de forzar final.

La comprobación causal adicional conserva C03 y el binario Rust, pero en una petición aislada usa ignore_eos=true y stop explícito para <|return|> y <|call|>. No modifica código ni configuración persistente. El resultado del cierre confirma la recuperación del canal final en C03.

## Fuentes técnicas y custodia

- [Formato Harmony, OpenAI](https://developers.openai.com/cookbook/articles/openai-harmony).
- [Verificación de implementaciones de gpt-oss, OpenAI](https://developers.openai.com/cookbook/articles/gpt-oss/verifying-implementations).
- [Fuente de cálculo de EOS, commit cotejado](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-core/src/pipeline/chat_template.rs).
- [Artefactos oficiales de llama.cpp b11193](https://github.com/ggml-org/llama.cpp/releases/tag/b11193).

Las solicitudes, respuestas y trazas se conservan en la carpeta de auditoría. La evidencia incluye salidas internas para análisis técnico; el informe presenta conclusiones y respuestas finales, sin tratar el texto interno como una explicación fiel de los mecanismos del modelo. No se publican pesos, secretos ni evidencias automáticamente en GitHub.
## Resultados del banco bruto

Se diferencia respuesta final incorrecta, ausencia de canal final y agotamiento del límite. Un HTTP 200 no acredita corrección semántica.

| Caso | Rust, prefijo final | Rust, prefijo analysis | llama.cpp, prefijo final | llama.cpp, prefijo analysis |
|---|---|---|---|---|
| C01: tres preguntas | Incorrecta: naranja en la tercera; replica exactamente el expediente original | Incompleta: length, sin canal final, 673,18 s | Incorrecta: naranja | Incorrecta: limón; sí entrega canal final |
| C02: reo | Incorrecta: inventa una bola blanca tras devolver una negra; replica exactamente el expediente original | Incompleta: length, sin canal final, 694,09 s | Incorrecta: evita la extracción exigida por el enunciado | Incompleta: length, 1000 tokens, sin canal final |
| C03: conservación de fichas | Correcta: cero blancas | Stop sin canal final: no hay respuesta final entregable | Correcta | Correcta, con canal final |
| C04: inferencia sobre grupo A | Correcta: no se puede concluir | Stop sin canal final: no hay respuesta final entregable | Correcta | Correcta, con canal final |

La copia Rust con prefijo final reproduce literalmente ambas respuestas originales y ambos prompts; COTEJO-REPRODUCCION.json conserva el cotejo. Los controles correctos impiden concluir que el modelo falla en cualquier razonamiento elemental. Los fallos completos en dos motores y la ausencia de respuesta final en otras condiciones impiden afirmar que activar analysis resuelve los dos problemas.

El banco bruto contiene 16 condiciones evaluadas. La primera petición Rust C01-analysis que agotó 600 segundos es un intento adicional fallido de transporte y no se suma como una segunda condición semántica. En la segunda pasada Rust hay 1000 fragmentos SSE en cada salida length; ese contador se conserva como fragmentos, sin usarlo como medición independiente de tokens.

En el contraste chat nativo, C01 terminó normalmente con «Un huevo» en la tercera línea: también incorrecto. C02 y la prueba EOS se documentan en los apartados siguientes.
## Contraste con plantilla nativa de llama.cpp

C01: stop, 208 tokens de salida contabilizados por el servidor, respuesta final «Un huevo»; incorrecta. C02 reanudado: length, 2048 tokens, 117,08 s y contenido final vacío; incompleto. El intento anterior de C02 interrumpido al aclarar la autorización no se puntúa. La plantilla nativa no resolvió estos dos casos dentro de los límites ensayados.

La diferencia entre respuesta incorrecta e incompleta es material: no se atribuye al modelo una respuesta final que nunca produjo. Tampoco se presenta el aumento de presupuesto a 2048 tokens como una prueba exhaustiva de incapacidad: documenta que esa configuración y ese límite no bastaron.
## Comprobación causal de EOS: resultado

C03-analysis-eos terminó con HTTP 200, stop, marcador final presente y respuesta correcta: cero fichas blancas. Tiempo total: 150,01 segundos. La salida de la variante conserva exactamente los 674 caracteres de la salida basal y continúa con <|end|><|start|>assistant<|channel|>final<|message|>, seguido de la respuesta en español. COTEJO-CAUSAL-EOS.json registra la comparación.

Esta intervención sólo cambia ignore_eos y las paradas explícitas de esa petición. Con el mismo binario, pesos y prompt se recupera la transición que antes se cortaba. Es evidencia causal acotada de un problema en las condiciones de terminación para este caso. No demuestra que los fallos semánticos de C01 y C02 se deban a EOS: en sus variantes analysis el motivo observado fue length, no stop prematuro. Tampoco demuestra una mejora general de calidad por habilitar razonamiento.

Se completaron 19 condiciones: 16 del banco bruto, 2 con plantilla nativa y 1 de intervención sobre EOS. «Completadas» significa recogidas y clasificadas; incluye resultados length sin respuesta final. Se conservan además el intento Rust que agotó el timeout y el intento nativo interrumpido por el operador, sin contarlos como nuevas condiciones resueltas.

## Criterio de decisión para la Dirección

1. Corregir primero la conversación y terminación de Harmony. Una respuesta con stop pero sin canal final no debe etiquetarse como respuesta final satisfactoria. Una salida length o timeout debe constar como incompleta. El control debe basarse en estados y delimitadores, no en si el texto parece una respuesta.
2. Incorporar una verificación de regresión breve: transición analysis → final, fin normal, límite de longitud y cancelación, conservando los límites de recursos. No trasladar directamente el parámetro experimental ignore_eos a producción sin revisar todas las condiciones de finalización.
3. Una vez corregido el protocolo, repetir un conjunto cerrado de tareas representativas de SV, con respuestas esperadas y criterios de abstención fijados previamente. Los cuatro casos de este informe pueden servir como regresión, pero no como demostración de generalización ni como material de entrenamiento y evaluación simultáneamente.
4. No iniciar entrenamiento de pesos para ocultar estos fallos ni aprobar sustitución por llama.cpp con esta evidencia. Los errores de contenido observados también en el comparador impiden prometer que basta cambiar de motor. El contraste permite investigar con mejor fundamento, no certificar un ganador.

No se propone abrir una investigación indefinida. La siguiente decisión tiene un alcance concreto: resolver el defecto de protocolo y aplicar una puerta de aceptación sobre tareas delimitadas. Si el sistema no cumple los criterios acordados, se excluye de esa función. La exactitud debe evaluarse sobre la salida entregada, sin conceder valor de garantía a razonamientos extensos o a una terminación HTTP correcta.

La reproducción de respuestas falsas refuta una garantía general de infalibilidad de la configuración ensayada. No cuantifica por sí sola el rendimiento médico ni permite extrapolar un porcentaje de error a medicina. La ausencia de validación clínica se mantiene.
## Cierre operativo y consumo

Cotejo final del servidor: 26/09/2026, 05:26:49 UTC (07:26:49, Europe/Madrid).

- Servicios originales sv-conversacion y sv-conversacion-motor activos, PID 61157 y 61207, idénticos a la inspección inicial; huellas de ambos binarios sin cambios. No reiniciados por esta auditoría.
- Todos los motores y auxiliares de auditoría detenidos. Los puertos experimentales 8091 y 8092 ya no escuchan; permanecen 3000 y 8089 originales.
- Dos unidades históricas de la primera tentativa conservan estado failed por el timeout y la interrupción preventiva de su secuencia. No hay procesos activos en ellas; se conserva ese estado como evidencia y no se confunde con fallo del servicio original.
- La captura de memoria del motor Rust de auditoría muestra MemoryPeak=12 947 025 920 bytes, aproximadamente 12,06 GiB, y cero eventos oom/oom_kill. Es el pico de ese cgroup durante la serie observada, no un dimensionamiento máximo universal ni la RAM de toda la máquina.
- Codespaces no fue necesario tras recuperar SSH directo. Un intento anterior quedó impedido por HTTP 402 de facturación; no se cambiaron pagos ni se usa este hecho como diagnóstico del motor.
- Cuota de cuenta observada: del 43 % restante inicial al 41 % en la comprobación final, una diferencia de dos puntos. La cuenta es compartida; el Director confirmó trabajo paralelo de Watson. No puede atribuirse toda la variación a esta auditoría. No es una medición del coste facturado por OneCloud.
- No se han modificado pesos, entrenado, aplicado cambios a la aplicación original, publicado en GitHub ni cambiado instancias. Permanecen las evidencias y el comparador descargado en la carpeta aislada del servidor, sin servicios activos asociados.

## Entrega de evidencias

Archivo local auditoria-razonamiento-20260926-evidencias.tar.gz: 222 267 bytes; SHA-256 81510f501f44cbe5d41e0fab6e5a90ce5b8fcbbb8c0262bf1839f8048776af8d, coincidente con el archivo del servidor. Se verificaron localmente las 158 huellas del manifiesto interno sin discrepancias. VERIFICACION-CUSTODIA.json conserva el resultado.

La extracción completa está en evidencia-remota; sus RESULTADOS-CONSOLIDADOS.json y COTEJO-CAUSAL-EOS.json son los resultados de cierre. Las copias del mismo nombre en la raíz remiten a ese cierre. Los scripts, el protocolo, los cotejos de fuentes, la reproducción de expedientes y la equivalencia de tokens C01 se conservan junto al informe. Los archivos de ESTADO-PARCIAL documentan incidencias anteriores y quedan expresamente superados por este cierre.

La evidencia está en C:\laboratorio\respaldo-onecloud-privado\auditoria-razonamiento-20260926 y en /opt/sv-lab/auditoria-razonamiento-20260926. El archivo comprimido excluye los binarios del comparador y no incluye los pesos del modelo ni claves privadas.