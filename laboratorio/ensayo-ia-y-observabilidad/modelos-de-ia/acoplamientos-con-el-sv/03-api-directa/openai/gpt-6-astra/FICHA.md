# GPT-6 Astra · Nodo 03 · OpenAI

Fecha de apertura: 6 de octubre de 2026. Identificador solicitado: `gpt-6-astra`.

## Resultado comprobado

**Conectividad e inferencia acreditadas en una prueba artificial:** OpenAI declaró `gpt-6-astra`, emitió `response.completed` y devolvió «CONEXION ASTRA CONFIRMADA». Tiempo total medido en Rust: 3223 ms; primer texto: 2913 ms. Uso comunicado: 49 tokens de entrada y 12 de salida, total 61; razonamiento y caché: 0.

Se conservan dos intentos diferenciados: el primero fue rechazado por límite de uso compartido; el segundo concluyó después de habilitar temporalmente el uso de créditos existentes. Esa opción se restableció al finalizar. No hubo recarga, compra ni tercera solicitud. No se dispone de liquidación monetaria ni descuento de créditos atribuible comunicado por la respuesta.

El lector inicial no recogía el texto cuando `response.completed.output` estaba vacío. El cotejo Rust posterior, sin acceso al proveedor, confirmó la igualdad del texto en los fragmentos, el cierre textual, el cierre de contenido y el mensaje concluido, además de la identidad y terminación. Se mantienen tanto la proyección inicial defectuosa como la evidencia íntegra y el cotejo. Incorporar esta forma de recepción al adaptador general y recibir su corrección sigue pendiente antes de la campaña. Véase [informe de la prueba](PRUEBA-CONEXION-20261006.md).

## Objeto y alcance

Comprobación instrumental de una inferencia breve mediante Responses y autorización oficial Sign in with ChatGPT. El candidato fue seleccionado tras su presencia en el catálogo autenticado. El antecedente de GPT-6.1 Sol se conserva en su expediente; esta selección no modifica aquel resultado.

Se autoriza una solicitud artificial con créditos existentes. No se transmiten documentos del SV, información de salud ni casos A01–A09. El resultado de esta comprobación no acredita competencia científica, gobierno por el SV ni recepción del acoplamiento transversal.

## Dependencia criptográfica pendiente

El código propio está escrito en Rust y prohíbe `unsafe`. El transporte HTTPS utiliza reqwest y rustls, con `ring`, que incorpora C y ensamblador. El 06/10/2026 se autoriza expresamente realizar esta prueba manteniendo esa dependencia anotada como pendiente. No se declara que todas las dependencias sean Rust ni se extiende esta aceptación a la admisión definitiva en el SV. La revisión y eventual sustitución de la dependencia nativa siguen pendientes.

## Método y observabilidad

Una solicitud, sin reintentos automáticos, `store=false`, `stream=true` y esfuerzo de razonamiento `low`. Se conserva la petición artificial, la respuesta recibida, su estado terminal, modelo declarado, uso comunicado, duración propia, primer evento, primer texto, tamaños y SHA-256. La ausencia de `response.completed` impide declarar terminación satisfactoria.

La modalidad preliminar no admite `max_output_tokens`. El límite local de 120 segundos y un MiB restringe la espera y la recepción locales; no acredita cancelación remota ni una cota económica. Los créditos descontados y su liquidación no se deducen de los tokens si el proveedor no los comunica. No se modifican recargas ni medios de pago.

El expediente de la prueba se encuentra en `prueba-conexion-20261006/`. La autenticación conserva únicamente el registro necesario para reutilizar el cliente; las credenciales de sesión no se publican ni se guardan en el expediente.

## Referencias oficiales

- [GPT-6 Astra](https://developers.openai.com/api/docs/models/gpt-6-astra).
- [Modelos e inferencia mediante Sign in with ChatGPT](https://developers.openai.com/siwc/token-sharing-open-source/models-and-inference).
- [Limitaciones de la modalidad preliminar](https://developers.openai.com/siwc/token-sharing-open-source/preview-limitations).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
