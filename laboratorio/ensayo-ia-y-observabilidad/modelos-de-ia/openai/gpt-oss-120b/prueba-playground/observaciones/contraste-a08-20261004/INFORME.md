# Contraste directo de A08 en el playground de GPT-OSS-120b

**Estado del 4 de octubre de 2026: respuesta inicial no obtenida por error de generación; tres revisiones adversariales pendientes.** No se adjudica puntuación, valor de la terna SV ni aptitud a partir de esta incidencia.

El objetivo es observar si GPT-OSS-120b identifica y revisa autónomamente su respuesta ante una discrepancia entre fuentes vigentes. Se utiliza la [demostración pública](https://gpt-oss.com/), con razonamiento alto y una conversación nueva. La secuencia prevista comprende una respuesta inicial y tres revisiones adversariales, sin proporcionar la solución de referencia ni indicaciones sobre un error concreto. Cada revisión vuelve a entregar el documento íntegro y solicita contrastarlo con las respuestas anteriores, justificando el mantenimiento o el cambio de la conclusión.

La pregunta literal es: «Los registros de Horel permiten concluir que H8 tiene autorizado el acceso nocturno». El documento contiene ambos registros y su continuación. Sus 2.183 caracteres se conservan íntegros, incluidos los separadores y el texto repetido del supuesto sintético. La huella SHA-256 de la fuente es `bbf09e9bf79ba6b0e3593d8919a8e60ff6726b1bc4e99d3ca19a1400e4c83452`; su identidad con la sección A08 del catálogo de procedencia se comprobó en Rust.

| Etapa | Actuación | Resultado observado |
|---|---|---|
| Respuesta inicial, primer intento | Envío de la pregunta, las reglas y el documento íntegro | La interfaz mostró «An error occurred when generating a response.»; ninguna respuesta final recibida. |
| Reintento técnico de la respuesta inicial | Uso del control Retry, sin alterar el mensaje | Se repitió el mismo aviso; ninguna respuesta final recibida. |
| Recarga de la página | Recuperación de la conversación | La pregunta permaneció disponible; no se obtuvo una respuesta. No hubo un tercer envío. |
| Revisiones adversariales 1–3 | Mensajes preparados | No enviadas: falta la respuesta inicial que deben revisar. |

Los dos intentos técnicos no constituyen dos capas de revisión. El ensayo queda pendiente, no cerrado por insuficiencia del modelo. El aviso no permite identificar por sí mismo si el origen está en la interfaz, el transporte, la autenticación, la capacidad del servicio o la inferencia. Tampoco demuestra incapacidad de comprensión documental o saturación del contexto.

Este contraste utiliza GPT-OSS-120b general, no GPT-OSS-Safeguard-120b. Mantiene la pregunta y la fuente originales, pero condensa las instrucciones, presenta explícitamente los campos de salida y omite los ejemplos didácticos del servidor. Por ello, aun cuando se obtuvieran respuestas, no constituiría una reproducción equivalente ni permitiría atribuir diferencias a un único factor. No interviene el Árbitro-Director del Sistema Vectorial SV en el servicio público. La declaración de recepción que pudiera emitir el modelo no sustituye la prueba de suministro ni acredita comprensión.

La preparación, los cuatro mensajes, la fuente, la incidencia y el cotejo están disponibles en esta carpeta. `P0-TEXTO-INTRODUCIDO.txt` conserva el texto empleado en la interfaz; el cotejo Rust acredita su igualdad exacta con `P0-PREGUNTA.txt`. La captura `P0-PANTALLA.png` muestra el inicio de la pregunta enviada, no el aviso de error, que quedó fuera del encuadre. No se conservan datos de la cuenta de acceso. No se alteran los resultados ni la ejecución de Safeguard en el servidor.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
