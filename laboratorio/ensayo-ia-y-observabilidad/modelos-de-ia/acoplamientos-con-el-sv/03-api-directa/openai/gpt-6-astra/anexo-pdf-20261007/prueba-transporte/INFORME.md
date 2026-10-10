# Acoplamiento PDF y transporte de Astra: prueba PDF06

El 07/10/2026 se ha ejecutado una solicitud real con GPT-6 Astra. El suministro textual completo recibido por MCP y admitido por el auxiliar Rust del Árbitro-Director llegó al transporte Responses sin cambios. Se recibieron una respuesta JSON, sus fundamentos y tres citas verificables. La reproducción del protocolo, la entrega y la instrumentación es conforme.

La prueba comprueba el acoplamiento del caso PDF06, con las páginas físicas **5 y 8** de la ficha LLS de 2018. No constituye ejecución de los nueve casos, adjudicación del anexo, examen, estabilidad entre repeticiones ni admisión clínica. El resultado A0 anterior permanece intacto. S39 y TT-0021 conservan el mismo expediente.

## Medición efectiva

| Magnitud | Resultado |
|---|---:|
| Solicitudes de inferencia / reintentos | 1 / 0 |
| Tiempo hasta entrega y cotejo inmediato | 23.582 ms |
| Primer texto recibido | 3.294 ms |
| Ventana de observación del proceso | 26.675 ms |
| Eventos SSE conservados | 848 |
| Bytes SSE conservados | 240.775 |
| Muestras del proceso | 101 |
| Fallos de medición / intervalo máximo | 0 / 283 ms |
| Memoria residente máxima | 35.483.648 bytes |
| CPU acumulada en la ventana | 1.766 ms |
| E/S leída / escrita del proceso | 13.964.922 / 967.384 bytes |
| Conexiones TCP simultáneas observadas, máximo | 6 |
| Observaciones UDP | 0 |
| Tokens de entrada / salida | 5.228 / 844 |
| Tokens totales / entrada en caché / razonamiento comunicado | 6.072 / 0 / 0 |

La terminación fue response.completed, HTTP 200, modelo solicitado y declarado gpt-6-astra. El tiempo de 23.582 ms incluye recepción, escritura durable y cotejo inmediato; no es duración interna del modelo. Las mediciones de CPU, memoria y E/S corresponden al cliente, receptor local e instrumentación. Las seis conexiones incluyen acceso y retorno local; no se imputan todas a una petición. Se observaron Listen, Established y CloseWait. El proceso terminó después de servir el resultado, con salida 0.

El proveedor desglosa la entrada en 326 tokens de instrucciones, 4.900 del contenido de entrada y 2 atribuidos al elemento de respuesta: total 5.228. La salida contiene 844 tokens. La suma y la atribución fueron cotejadas en Rust. No se recibió un resumen adicional de razonamiento. Los fundamentos entregados constituyen explicación auditable, no acceso a procesos internos privados del modelo.

## Gobierno, fuente y trazabilidad

Antes de la conexión se repitió la extracción del original fijado, se recorrieron las diez páginas y treinta fragmentos por MCP aislado y se reprodujo su diario. El caso transmitido emplea únicamente sus seis fragmentos de las páginas 5 y 8. El auxiliar de admisión documental exige fuente, orden, totalidad, aislamiento local y externo del MCP, diario e instrumentación; el transporte recompone y coteja el contenido inmediatamente antes del envío. La clave de evaluación, la telemetría y las decisiones del evaluador no forman parte de la solicitud.

La solicitud quedó fijada en 19.640 bytes, SHA-256 e62db6b03782ae955671adfbc5f35c3916d4884115d6391b3529a45ad83bfc86. Original PDF: 21322ed388c999bd0713ba5ec5c7ab34402d4bb4cd6100903aa5ea6d8bf35d9c. Los inventarios y recibos identifican fuentes y ejecutables utilizados; no basta con que un componente figure declarado.

Se conservaron solicitud, SSE original, entrega, texto final, cabeceras permitidas, catálogo autorizado, diario, muestras y recibos. La lectura de los mensajes SSE se coteja con los elementos concluidos y con los totales del proveedor. La cadena SHA-256 permite detectar cambios respecto de los recibos conservados; no constituye firma ni sello temporal externo. La auditoría posterior no envió ninguna solicitud al modelo.

Se ejecutaron 53 pruebas Rust del transporte, acceso y suministro: 53 conformes. La revisión específica de localizadores ejecutó seis pruebas, tres de ellas del lector JSON ya comprobado: seis conformes. No deben presentarse ambas ejecuciones como 59 pruebas distintas.

## Incidencia del comprobador y rectificación

El comprobador inicial exigía que fragmentos contuviese índices enteros. El banco pedía ese campo, pero no prescribía esa representación. Astra identificó los fragmentos mediante inicio_caracter y fin_caracter_exclusivo, datos presentes en el suministro. Por tanto, aquel rechazo formal no acredita un incumplimiento del candidato.

Una revisión Rust fuera de línea cotejó los intervalos exactos con los fragmentos reales: página 5, caracteres [2000,4000); página 8, [0,2000). Las tres citas son literales al normalizar espacios. Las pruebas negativas rechazan intervalos parciales o ajenos, citas inventadas y páginas distintas. Se conservan sin sobrescribir el resultado y el rechazo originales, junto con REVISION-LOCALIZADORES-RUST.json. No hubo una segunda inferencia. Antes de extender el ensayo, debe incorporarse esta resolución al comprobador general y fijar expresamente la representación de los localizadores.

La lectura documental de la entrega encuentra los cuatro elementos pedidos, su relación con el seguimiento y límites expresos de la fuente histórica. No se identificó una premisa externa en esa lectura. Esto no sustituye la adjudicación completa y separada del anexo ni una revisión médica de vigencia.

## Límites y consumo

La solicitud prohíbe información externa y no ofrece herramientas: tools vacío y tool_choice none. No se ejecutó ninguna herramienta ni código del candidato. El cliente emplea un destino HTTPS fijo, sin redirecciones, intermediario configurado ni reintentos, y un plazo de 300 segundos. Esta evidencia no acredita aislamiento de la infraestructura interna de OpenAI ni permite borrar conocimientos del entrenamiento.

La telemetría cubre el proceso propio, sus recursos y sus conexiones mediante muestreo. No mide recursos internos del proveedor, descomposición DNS/TLS, RTT, retransmisiones, hilos, handles ni asignaciones individuales de memoria Rust. La CPU porcentual carece de calibración independiente. La criptografía con componentes C/ensamblador conserva su excepción experimental y su condición pendiente.

El saldo agregado comunicado antes y después fue 49.905,8723330000 créditos; el uso semanal comunicado fue 6 % en ambas lecturas. Es una cuenta compartida: no se deduce de ello el cobro individual ni un coste cero. Créditos e importe atribuibles permanecen desconocidos. No se modificaron recargas, permisos económicos ni límites. Registro individual: **SV-GASTO-20261007-012**, en el archivo privado de usos, gastos, créditos y tokens.

## Custodia y continuación

La publicación técnica contiene este informe, métricas y recibos sin datos de cuenta ni direcciones de red, y las fuentes Rust pertinentes. Los originales operativos completos, el texto enviado y recibido, la clave y los diarios permanecen bajo conservación controlada; sus huellas se declaran en los recibos. No se afirma custodia remota íntegra de esos originales ni se redistribuye aquí el texto médico. El informe económico excluye el contenido documental y clínico.

El transporte usa el cliente de acceso existente cuya ruta histórica conserva gpt-6.1-sol; el candidato de esta prueba y su expediente son gpt-6-astra. La compilación de referencia es cargo con --locked --offline; el ejecutable de inferencia y el verificador posterior tienen huellas distintas y funciones distintas. No se alteraron la clave, el banco ni el suministro después del envío.

Retorno: recepción de esta integración y resolución del formato de localizadores antes de ampliar los casos. TT-0021 permanece pendiente; no se genera un polígono con ocho posiciones sin evaluar.

Referencias del transporte: [inferencia con acceso ChatGPT](https://developers.openai.com/siwc/token-sharing-open-source/models-and-inference), [eventos de Responses](https://developers.openai.com/api/docs/guides/streaming-responses). La comprobación de esta ejecución se fundamenta en sus recibos, no sólo en la documentación del proveedor.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).