# Protocolo de calidad conversacional y contexto creciente

Fecha: 24 de septiembre de 2026. Autorización: continuación expresa del ensayo y habilitación de una interfaz de conversación. No se adquieren recursos. Publicación en `main`.

## Objeto y separación de resultados

Evaluar GPT-OSS-20B MXFP4 mediante el motor Rust optimizado ya verificado, y adaptar la interfaz de conversación conservada para Qwen. La equivalencia numérica y la aceleración del ensayo anterior no constituyen validación de calidad general. Esta fase estudia tareas sintéticas y conservación de antecedentes; no certifica aptitud clínica ni profesional.

Bases: SV-motor `d4e62b29713a2044be0d1d4a7fb463155d3999c3`; repositorio de calidad `13e5becc550b731327ff33e291f79d884eee4b9b`. Se mantienen los documentos rectores previamente leídos, la gramática 0.2 y la IR 0.3. El modelo permanece en la frontera experimental y no adquiere facultades de ejecución ni decisión del SV.

## Condiciones y límites

- Pesos SHA-256: `27cd6c432c7672cb812a92f611cf3ba7bbc35928262bb1e1253ff4ee6ae35901`.
- Motor SHA-256: `f6ec0bb908e18ced633d66d6a762a915349281bdbb1397991b268fe487bcc418`; modo MXFP4 paralelo, CPU, BF16, doce hilos, una generación simultánea.
- Runtime y control: Rust/Cargo 1.98.0. JavaScript limitado a presentación y transporte de la interfaz; no se utiliza Python.
- Entrada Harmony con canal final explícito; temperatura cero. Se conserva el contexto completo y se verifica el número de tokens contra el motor. Una discrepancia invalida la admisión siguiente hasta corregirla.
- Ventana de ensayo automatizado: máximo dos horas desde su lanzamiento registrado; máximo 900 segundos por petición y 128 tokens de salida en el banco. No se repite una condición fallida sin diagnóstico y suceso separado.
- Contexto operativo candidato: 4096 tokens, incluida la reserva de salida. Se mide antes de admitirse. No se recorta ni resume automáticamente. La disponibilidad configurada no equivale a calidad demostrada.
- Cuota del motor: 32 GiB, sin intercambio; interfaz en unidad separada con cuota de 2 GiB. Cancelación o plazo excedido requieren confirmar la parada del motor, no solo la del cliente HTTP.
- Interfaz privada mediante GitHub, servicio y motor enlazados a la dirección local. Duración inicial del servicio interactivo: ocho horas; la reanudación se documenta. No hay publicación anónima del modelo.

## Banco previo a observación de respuestas

Doce tareas breves: suma, resta, multiplicación, ordenación, extracción literal, respuesta JSON, seguimiento de dos condiciones, resumen de hechos suministrados, contradicción explícita, insuficiencia de datos, conservación de unidades y separación entre texto citado e instrucciones. Los enunciados y criterios exactos se fijan en el código del banco antes de ejecutarlo. Una ejecución por tarea; se informa cada resultado sin extrapolación estadística.

Conversación real de al menos cuatro turnos: introducir un identificador, corregir una cantidad, añadir una condición y recuperar conjuntamente los datos vigentes. Las respuestas del modelo se conservan; no se sustituyen por respuestas ideales.

Contexto creciente: cuatro condiciones con aproximadamente 256, 768, 1536 y 3072 tokens de entrada. Los valores efectivos se cuentan y publican. Se coloca un dato al principio, distractores sintéticos en el centro y una consulta al final. Se evalúan recuperación, latencia y terminación. No se presenta este ensayo sintético como equivalente a meses de conversación.

## Criterios y evidencia

Se separan: aceptación HTTP, terminación técnica, fidelidad del contexto, corrección según criterio fijado y utilidad percibida por el usuario. Estados de interrupción o errores de infraestructura no se atribuyen automáticamente al modelo.

Se conservan preguntas, contextos exactos, huellas, parámetros, identidad binaria, respuestas originales, tiempos, uso de memoria y eventos de cierre. El registro local encadenado no sustituye una firma externa. Los resultados se publican en `main` con manifiesto de huellas, informe, parte de trabajo y actualización de los registros de calidad.

La interfaz se verifica con una conversación real, exportación y comprobación de acceso privado. El funcionamiento depende también del Codespace que mantiene el túnel; se documentan parada, reanudación y límites. Los datos conversacionales del usuario no se incorporan automáticamente al repositorio.
