# GPT-6 Astra · Nodo 03 · OpenAI

**Corrección de la entrega gráfica · 07/10/2026:** el HTML anterior era una captura estática y no contenía un polígono interactivo. Se sustituye por el visor Rust/egui 0.2.0, compilado a WebAssembly e incorporado en un HTML autónomo. Resultados científicos inalterados. [Defecto, corrección y pruebas](catalogo-a0-20261007/CORRECCION-VISOR.md).

Fecha de apertura: 6 de octubre de 2026. Identificador solicitado: `gpt-6-astra`.

## Catálogo A0 ejecutado · 07/10/2026

Nueve casos completos y adjudicados: 9/9 correctos, incluidos los seis críticos; κ Apto y 100/100, limitados a A0. Dos páginas íntegras por caso; sin herramientas del candidato ni premisas externas identificadas en el contraste. Tiempo conjunto con observación: 90,992 s; 20.051 tokens. 340 muestras Rust y 1.874 eventos, sin fallos de captura. Polígono completo presentado en egui. [Informe y evidencias](catalogo-a0-20261007/INFORME.md).

Adversariales, B, anexo PDF y examen permanecen separados. Importes atribuibles no comunicados; recepción independiente y criptografía nativa pendientes. Los apartados siguientes conservan el estado de cada antecedente en su fecha.

## Tercera prueba: entrega estructurada

Una consulta artificial concluida, JSON válido y cotejo Rust de 22 archivos. Resultado correcto: 904; se reciben justificación breve, afirmaciones con datos de apoyo, código Rust propuesto, limitaciones y condiciones de revisión. No se habilitan herramientas ni se ejecuta el código propuesto. El resumen opcional del proveedor no se recibe.

Duración: 17,910 s; primer texto: 4,661 s; uso: 584/605/1189 tokens; 599 eventos y 79 muestras instrumentales, sin fallos registrados. Una alerta por referencias textuales se identifica como falso positivo del comprobador local; el informe original y la revisión posterior se conservan sin otra llamada. El uso temporal de créditos queda desactivado.

Véase [tercera prueba y condiciones de continuación](PRUEBA-ENTREGA-ESTRUCTURADA-20261006.md). Esta fase acredita entrega instrumental; catálogo, adversariales y examen permanecen separados y sin ejecutar. La recepción integral del instrumento científico sigue pendiente. En las tres actuaciones se conservan un rechazo inicial y tres generaciones, sin transferir resultados de unos contratos a otros.

## Antecedente: segunda prueba instrumentada

Una autorización posterior permite una segunda inferencia, ya con observación local Rust desde antes del envío hasta después del cierre. Resultado: 4296 ms, primer texto a 3984 ms, 49/12/61 tokens, 16 eventos; 28 muestras de proceso y conexiones, intervalo máximo 284 ms, sin fallos registrados. Se miden CPU, memoria y E/S del cliente, extremos y estados TCP y puertos. Memoria residente máxima: 26,6211 MiB; CPU acumulada en la ventana: 500 ms. Estos recursos pertenecen al cliente e incluyen la instrumentación.

La corrección del ensamblaje de texto queda incorporada y probada para el contrato estrecho de esta prueba. Los 17 archivos del nuevo expediente se cotejan en Rust; no hay tercera generación. En el conjunto de ambas actuaciones hay un rechazo y dos generaciones. Se restablece a desactivado el uso adicional de créditos. La recepción integral del instrumento, su generalización, privacidad y criptografía siguen pendientes. Véase [informe de instrumentación y límites](PRUEBA-INSTRUMENTADA-20261006.md).

## Antecedente: primera prueba de conexión

**Conectividad e inferencia acreditadas en una prueba artificial:** OpenAI declaró `gpt-6-astra`, emitió `response.completed` y devolvió «CONEXION ASTRA CONFIRMADA». Tiempo total medido en Rust: 3223 ms; primer texto: 2913 ms. Uso comunicado: 49 tokens de entrada y 12 de salida, total 61; razonamiento y caché: 0.

Se conservan dos intentos diferenciados: el primero fue rechazado por límite de uso compartido; el segundo concluyó después de habilitar temporalmente el uso de créditos existentes. Esa opción se restableció al finalizar. No hubo recarga, compra ni tercera solicitud. No se dispone de liquidación monetaria ni descuento de créditos atribuible comunicado por la respuesta.

El lector inicial no recogía el texto cuando `response.completed.output` estaba vacío. El cotejo Rust posterior, sin acceso al proveedor, confirmó la igualdad del texto en los fragmentos, el cierre textual, el cierre de contenido y el mensaje concluido, además de la identidad y terminación. Se mantienen tanto la proyección inicial defectuosa como la evidencia íntegra y el cotejo. Incorporar esta forma de recepción al adaptador general y recibir su corrección sigue pendiente antes de la campaña. Véase [informe de la prueba](PRUEBA-CONEXION-20261006.md).

## Objeto y alcance

Comprobación instrumental de una inferencia breve mediante Responses y autorización oficial Sign in with ChatGPT. El candidato fue seleccionado tras su presencia en el catálogo autenticado. El antecedente de GPT-6.1 Sol se conserva en su expediente; esta selección no modifica aquel resultado.

Se autoriza una solicitud artificial con créditos existentes. No se transmiten documentos del SV, información de salud ni casos A01–A09. El resultado de esta comprobación no acredita competencia científica, gobierno por el SV ni recepción del acoplamiento transversal.

## Dependencia criptográfica pendiente

El código propio está escrito en Rust y prohíbe `unsafe`. El transporte HTTPS utiliza reqwest y rustls, con `ring`, que incorpora C y ensamblador. El 06/10/2026 se autoriza expresamente realizar esta prueba manteniendo esa dependencia anotada como pendiente. No se declara que todas las dependencias sean Rust ni se extiende esta aceptación a la admisión definitiva en el SV. La revisión y eventual sustitución de la dependencia nativa siguen pendientes.

## Método y observabilidad

En las dos primeras pruebas: una generación por actuación, sin reintentos automáticos, `store=false`, `stream=true` y esfuerzo de razonamiento `low`. La tercera usa `medium`, resumen `auto` y JSON estricto conforme a su informe. Se conserva la petición artificial, la respuesta recibida, su estado terminal, modelo declarado, uso comunicado, duración propia, primer evento, primer texto, tamaños y SHA-256. La ausencia de `response.completed` impide declarar terminación satisfactoria.

La modalidad preliminar no admite `max_output_tokens`. El límite local de 120 segundos y un MiB restringe la espera y la recepción locales; no acredita cancelación remota ni una cota económica. Los créditos descontados y su liquidación no se deducen de los tokens si el proveedor no los comunica. No se modifican recargas ni medios de pago.

El expediente de la prueba se encuentra en `prueba-conexion-20261006/`. La autenticación conserva únicamente el registro necesario para reutilizar el cliente; las credenciales de sesión no se publican ni se guardan en el expediente.

## Referencias oficiales

- [GPT-6 Astra](https://developers.openai.com/api/docs/models/gpt-6-astra).
- [Modelos e inferencia mediante Sign in with ChatGPT](https://developers.openai.com/siwc/token-sharing-open-source/models-and-inference).
- [Limitaciones de la modalidad preliminar](https://developers.openai.com/siwc/token-sharing-open-source/preview-limitations).

## Preparación del catálogo y decisión de alojamiento · 06/10/2026

Se fija el [contrato del catálogo, anexo y examen](CONTRATO-CATALOGO-20261006.md): Astra por API; caché, control, evaluación y custodia en el SV, en equipo propio. El Árbitro-Director y sus auxiliares Rust mantienen el gobierno, con el modelo excluido de clave, adjudicación y telemetría. Prohibida la navegación. Se conservan banco A/B, dos páginas completas por caso, política, revisiones y criterios; el anexo MCP/PDF precede al examen y no altera su calificación.

[Preparación local comprobada](PREPARACION-CATALOGO-20261006.md): 18 casos, 36 páginas previstas, política idéntica y siete comprobaciones Rust conformes. Ninguna consulta al modelo. Pendientes la recepción del recorrido MCP y del transporte científico en este entorno, y la admisión efectiva de consumo. No se rehace el Árbitro ni se atribuye a esta preparación la recepción integral del acoplamiento.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
