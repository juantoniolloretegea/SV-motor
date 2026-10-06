# Prueba instrumental de conexión a GPT-6 Astra

Fecha: 6 de octubre de 2026. Nodo 03, inferencia mediante API directa de proveedor. Alcance: texto artificial, una generación completada y su evidencia instrumental; no examen de capacidad científica ni admisión del modelo en el SV.

## Resultado y secuencia observada

Se utiliza el cliente Rust previamente registrado mediante Sign in with ChatGPT. Identidad, firma, emisor, destinatario, vencimiento, vinculación de la sesión y permisos se comprueban nuevamente. El catálogo autenticado incluye `gpt-6-astra`.

La primera solicitud alcanza HTTP 200 y recibe tres eventos, terminando con `error` y código `subscription_sharing_usage_limit_exceeded`, a los 1601 ms. No contiene texto generado ni uso de tokens. El HTTP satisfactorio no se clasifica como inferencia completada.

La consulta de la configuración oficial permite identificar que el uso de créditos por aplicaciones estaba desactivado. Conforme a la autorización expresa de consumir créditos existentes, se habilita temporalmente esa opción para la única aplicación conectada que muestra la lista. La recarga automática permanece desactivada. Se conserva el primer intento y se abre una continuación separada; no se ejecuta un reintento automático.

La segunda solicitud utiliza el mismo texto y parámetros. OpenAI declara `gpt-6-astra`, concluye con `response.completed` y entrega exactamente:

> CONEXION ASTRA CONFIRMADA

Terminada la generación, se restablece a desactivada la opción de uso de créditos por aplicaciones. Se cierran los receptores temporales de autenticación. La pantalla del resultado sólo sirve la evidencia conservada y no mantiene una inferencia activa.

## Petición y mediciones

Petición de 324 bytes: modelo `gpt-6-astra`, `store=false`, `stream=true`, esfuerzo `low`, sin herramientas; instrucciones de respuesta literal y mensaje «Responda exactamente: CONEXION ASTRA CONFIRMADA».

| Magnitud | Resultado de la solicitud concluida |
|---|---:|
| HTTP | 200 |
| Recepción de cabeceras y primer evento | 1902 ms |
| Primer texto recibido | 2913 ms |
| Terminación observada | 3223 ms |
| Eventos recibidos | 16 |
| Cuerpo recibido y conservado hasta terminación | 8483 bytes |
| Tokens de entrada comunicados | 49 |
| Tokens de salida comunicados | 12 |
| Tokens totales comunicados | 61 |
| Tokens de razonamiento comunicados | 0 |
| Lectura y escritura de caché comunicadas | 0 |

Los tiempos proceden del reloj monotónico del cliente Rust y describen la recepción local; no se atribuyen a tiempos internos del modelo. No se reciben cabeceras adicionales de identificación o procesamiento de las seleccionadas para el registro. Se conserva el identificador de respuesta en el expediente local, sin incluir identificadores personales en esta publicación.

Los tokens son uso comunicado por OpenAI. Créditos efectivamente descontados y coste liquidado permanecen no disponibles por solicitud. Las variaciones del saldo general no son atribuibles de forma exclusiva a esta prueba porque coexiste otro consumo. No se equiparan estos tokens con una factura ni se declara coste cero.

## Incidencia del lector y cotejo posterior

El primer lector buscaba el texto en `response.completed.response.output`. En el flujo recibido ese campo final está vacío, aunque los eventos `response.output_text.done`, `response.content_part.done` y `response.output_item.done` contienen la respuesta y el último mensaje figura como concluido. Por ello la proyección inicial consignó «respuesta recibida discordante» y texto vacío.

La revisión se realiza sobre los bytes conservados, sin otra llamada a OpenAI. El cotejo Rust verifica la huella y longitud originales, los 16 números de secuencia consecutivos, la identidad de respuesta entre apertura y cierre, el modelo declarado, el estado `completed`, un único mensaje concluido del asistente, la correlación por mensaje y posición del contenido, y la igualdad entre fragmentos y tres cierres de texto. Comprueba además 49 + 12 = 61 tokens.

El resultado corregido se conserva en `COTEJO-RUST.json`; no se sobrescriben `resultado.json`, el evento terminal ni el flujo original. Las dos revisiones de código efectivamente utilizadas se conservan por intento. La corrección del lector general y sus pruebas de regresión son una dependencia antes de la campaña; el cotejo acotado no equivale a recepción de un adaptador universal.

## Identidad de la evidencia

| Evidencia | Bytes | SHA-256 |
|---|---:|---|
| Petición artificial, común a ambos intentos | 324 | `466aa875641c0060c0dc10e2858d1995f6706c75fe990ad1a00f97d68097a970` |
| Primer flujo, rechazo | 3109 | `d2f0303d1e9614368acb1e1fc977a916e97c9766c0962ab62f6d2a4b11aa5390` |
| Segundo flujo, generación concluida | 8483 | `f297453b49892c9c6b17694b7f93ff3ccdd4b601d6634a89852b84f065fcdba8` |

Los cuerpos íntegros permanecen en el expediente local protegido: incluyen metadatos seudónimos del proveedor y no se publican sin minimización. Esta publicación conserva resultados, método y huellas, no afirma custodia pública íntegra de esos cuerpos. No se conservan credenciales de sesión en el expediente.

## Condiciones pendientes y retorno

1. `ring` incorpora C y ensamblador; se mantiene expresamente pendiente, con autorización limitada a esta prueba. No se acredita una cadena de dependencias íntegramente Rust.
2. Incorporar y verificar la recepción de texto concluido por eventos al adaptador general, manteniendo estados fallido, incompleto e incierto.
3. Completar recepción transversal, protección de datos, presupuesto, instrumentación y condiciones de evaluación antes de A01–A09. No se ha enviado documentación del SV ni información de salud.
4. Mantener la correspondencia con las sedes existentes de sucesos, tiques y calidad; este informe no las sustituye ni crea una numeración paralela. La conciliación canónica de estos nuevos hechos queda identificada para la siguiente recepción documental.

Dictamen acotado: conexión, identidad y generación de texto de GPT-6 Astra comprobadas. No se adjudica capacidad médica, de inmunología o de ciberseguridad, gobierno determinista ni aceptación en el SV.

## Fuentes oficiales consultadas

- [Modelos e inferencia](https://developers.openai.com/siwc/token-sharing-open-source/models-and-inference): exigencia de estado terminal para acreditar éxito.
- [Errores y recuperación](https://developers.openai.com/siwc/token-sharing-open-source/errors-and-recovery): tratamiento del límite de uso compartido.
- [Uso del plan y créditos por aplicaciones](https://learn.chatgpt.com/docs/sign-in-with-chatgpt): configuración de uso y límites.
- [Limitaciones de la modalidad preliminar](https://developers.openai.com/siwc/token-sharing-open-source/preview-limitations): ausencia de `max_output_tokens` y condiciones de Responses. El límite local de espera no acredita cancelación remota ni una cota económica.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
