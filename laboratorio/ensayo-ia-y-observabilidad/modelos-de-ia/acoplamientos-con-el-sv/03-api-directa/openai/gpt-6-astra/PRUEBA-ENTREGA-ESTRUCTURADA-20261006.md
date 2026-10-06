# Tercera prueba de GPT-6 Astra: entrega estructurada y trazabilidad

Fecha: 6 de octubre de 2026. Nodo 03, inferencia mediante API directa de proveedor. Identificador solicitado y declarado: `gpt-6-astra`.

## 1. Objeto y dictamen acotado

La prueba instrumental está concluida: una sola consulta artificial, una respuesta JSON íntegra y telemetría conservada. Se acredita que esta conexión permite recibir una explicación breve, afirmaciones relacionadas con datos, código propuesto, límites y condiciones de revisión. No se acredita fidelidad científica, repetibilidad ni aptitud clínica.

El catálogo A01–A09, las tres revisiones adversariales y el examen posterior no se han ejecutado en esta actuación. Su separación se mantiene. No es necesaria otra consulta trivial para acreditar de nuevo esta misma entrega; el retorno es preparar y recibir el instrumento del catálogo, conservando los requisitos pendientes del apartado 7.

## 2. Método y datos transmitidos

La petición INSTR-03 contiene únicamente datos artificiales: D1 = 37 grupos, D2 = 24 elementos por grupo y D3 = 16 elementos adicionales. Se solicita D1 × D2 + D3, acompañado de justificación breve y verificable. No se transmiten documentos del SV, datos personales, sanitarios o casos del catálogo.

El cliente Rust reutiliza el registro y la autorización de ChatGPT. Se solicita `store=false`, `stream=true`, esfuerzo `medium`, resumen opcional `auto` y formato JSON estricto. No se habilitan herramientas. El código propuesto por el modelo no se compila ni ejecuta. El límite local es de 120 segundos y un MiB; no constituye una garantía de cancelación remota ni un techo monetario.

## 3. Entrega efectiva

| Elemento solicitado | Observación |
|---|---|
| Resultado y conclusión | 904 elementos; coincide con 37 × 24 + 16 |
| Justificación breve | Multiplicación 37 × 24 = 888 y suma de 16 |
| Afirmaciones y apoyo | A1 y A2 con referencias a los datos del enunciado |
| Fuentes externas | Lista vacía; no se habilitó navegación |
| Código | Propuesta mínima Rust con enteros; sin ejecución |
| Acciones declaradas | Elaboración de respuesta y propuesta textual de código |
| Límites e incertidumbre | Reconoce falta de ejecución y de comprobación empírica de las entradas |
| Revisión de la conclusión | Cambios de datos, operación o interpretación |
| Resumen explicativo del proveedor | Solicitado, no recibido; cero tokens de razonamiento comunicados |
| Procesos internos del modelo | No observables mediante esta entrega |

La explicación incluida en el JSON sí llegó. El resumen opcional del proveedor es un elemento diferente y no llegó. Ninguno equivaldría a una transcripción íntegra del razonamiento interno ni a una garantía de veracidad. La ausencia observada no demuestra indisponibilidad universal de esa función.

La respuesta literal, sin metadatos de cuenta ni topología, se conserva en [RESPUESTA-ESTRUCTURADA-20261006.json](RESPUESTA-ESTRUCTURADA-20261006.json). Es una declaración del modelo; la evidencia instrumental se registra separadamente.

## 4. Medición y conservación

| Magnitud | Resultado |
|---|---:|
| Estado HTTP / terminal | 200 / response.completed |
| Solicitudes de inferencia de esta prueba | 1 |
| Reintentos | 0 |
| Duración total / primer texto | 17,910 s / 4,661 s |
| Primer evento | 3,255 s |
| Tokens de entrada / salida / total | 584 / 605 / 1189 |
| Tokens de razonamiento comunicados | 0 |
| Eventos SSE / bytes recibidos | 599 / 173752 |
| Muestras de proceso | 79: 9 previas, 66 durante, 4 posteriores |
| Intervalo máximo / fallos de medición | 318 ms / 0 |
| Memoria residente máxima del cliente | 27,9140625 MiB |
| CPU acumulada del cliente en la ventana | 1250 ms |
| Ventana observada | 20,939 s |

Se observan identidad del proceso, memoria, CPU, E/S, escucha local y conexiones TCP/UDP del proceso. La nueva conexión HTTPS pasa por ESTABLISHED y FIN-WAIT-1 y deja de aparecer; su extremo coincide con el comunicado por HTTP. Las conexiones previas no se atribuyen todas a la inferencia. Estos recursos pertenecen al cliente y su instrumentación, no al modelo remoto.

Veintidós archivos del expediente se cotejan en Rust: petición, contrato, respuesta, eventos, proyecciones, catálogo, telemetría y fuentes efectivamente ejecutadas. El cotejador vuelve a leer los originales sin contactar con OpenAI. Comparte el módulo de lectura con el cliente: es otro proceso de cotejo, no una auditoría externa ni una segunda implementación independiente.

Huella SHA-256 del flujo recibido: `220f92dd1ebcc9ebd1771fe5823d9c1af103492ca5a6516156ecd2008c267847`. Huella del diario instrumental: `2e706dd371f589b3662d45ee42eefbcf1cbecf27190e9653f2838d69d0d2dbea`. El cotejo acredita identidad local; no es firma del proveedor ni sellado temporal externo.

Los originales operativos contienen identificadores seudónimos y topología, por lo que permanecen en custodia local restringida. Esta publicación no acredita custodia remota de ese expediente íntegro.

## 5. Incidencia del comprobador y revisión conservada

El esquema y la petición admitían referencias textuales a los datos. Sin embargo, una comprobación local exigía exclusivamente las cadenas D1, D2 o D3. Por ello marcó «Referencia a dato no aportado» ante descripciones fieles como D1 = 37 grupos.

La revisión posterior coteja las descripciones y la operación con el enunciado, confirma el falso positivo y conserva el estado inicial `entrega_con_incidencias`. No se corrige la respuesta del modelo ni se borra el informe original. Se añaden `REVISION-DATOS-RUST.json` y una presentación revisada. La distinción entre error del instrumento y error sustantivo queda explícita. El cotejador también se ajusta para conservar y comprobar entregas con incidencias, en lugar de aceptar sólo resultados sin alertas.

Antes del envío pasaron 25 pruebas locales del cliente. Después pasaron nueve del cotejador, ocho compartidas con el módulo anterior y una sobre descripciones correctas frente a un valor alterado; no se suman como 34 verificaciones independientes. No se realizó otra inferencia para resolver la incidencia.

## 6. Cierre operativo y económico

El permiso temporal de uso de créditos se restableció a desactivado y se comprobó en la interfaz. La recarga automática permaneció desactivada. El receptor de autenticación se cerró tras comprobar su identidad. El visor posterior es local, de lectura y sin credenciales; no mantiene inferencia.

La respuesta comunica tokens, pero no descuento de créditos ni precio liquidado de esta solicitud. La variación del saldo general no permite imputar coste, porque existen otros consumidores. No hubo compra ni recarga.

## 7. Retorno a la siguiente fase

Queda resuelta la pregunta instrumental sobre lo que se puede exigir y recibir como entrega estructurada. Antes de iniciar A01 deben recibirse la adaptación al contrato científico vigente, las referencias exactas a las fuentes y el control de admisión de datos, conservación y consumo. La generalización de referencias debe distinguir identificador, valor y texto justificativo; no puede decidir equivalencia científica por coincidencia literal.

Se mantienen como pendientes la recepción integral del acoplamiento y la dependencia criptográfica `ring`, que incorpora C y ensamblador y fue admitida provisionalmente para estas pruebas. No se han medido recursos internos del proveedor, DNS/TLS por separado, RTT, retransmisiones, hilos, identificadores de recursos del sistema ni asignaciones internas de memoria Rust.

Secuencia posterior: catálogo conforme a su protocolo → revisiones adversariales cuando correspondan → examen condicionado. El polígono sólo aparece con el vector completo y adjudicado. La estabilidad de respuestas no demuestra por sí sola corrección; las fuentes, condiciones, negaciones y cambios legítimos por nueva evidencia requieren evaluación competente, especialmente en inmunología y ciberseguridad.

## 8. Referencias

- [Prueba anterior de instrumentación](PRUEBA-INSTRUMENTADA-20261006.md).
- [Salida estructurada: documentación oficial](https://developers.openai.com/api/docs/guides/structured-outputs?api-mode=responses).
- [Resúmenes explicativos: documentación oficial](https://developers.openai.com/api/docs/guides/reasoning).
- [Limitaciones de Sign in with ChatGPT](https://developers.openai.com/siwc/token-sharing-open-source/preview-limitations).
- [Del frame a la colaboración auditable](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/76722442d2899300c69d89ab932890a4d7d74c2f/docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/LEAME_PRIMERO.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
