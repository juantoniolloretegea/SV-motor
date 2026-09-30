# Qwen3-Next-80B-A3B-Thinking · instalación y primera prueba documental

**Fecha:** 30 de septiembre de 2026. **Expediente:** QWEN80-THINKING-Q4K-ONECLOUD-20260930, v1, entrega 01. **Seguimiento:** S39 / TT-0017. Dependencia MCP: TT-0014. Eventual examen: TT-0016.

La primera respuesta final de **THINK-DOC-01** se conservó a las **20:45:22.212 UTC** y la inferencia quedó cerrada a las 20:45:25 UTC. El cuarto intento completó tres generaciones y utilizó efectivamente buscar_documentos y leer_documento. Los tres intentos anteriores, sus errores de custodia y las correcciones R6–R8 permanecen conservados. No se repitió una respuesta completa para mejorarla.

La [entrega consolidada e informe](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/1cf92e92768938bd304ee22a792d16fc7a12bb9e/respuestas-ejecucion/QWEN80-THINKING-Q4K-ONECLOUD-20260930/entrega-01/INFORME.md) reúne 28 archivos cotejados: fuentes efectivas, configuración de cierre, comprobaciones, originales, respuesta y manifiesto. El [registro estructurado de esta carpeta](REGISTRO-INSTALACION-20260930.json) permite seguir el estado; el [manifiesto de entrega](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/1cf92e92768938bd304ee22a792d16fc7a12bb9e/respuestas-ejecucion/QWEN80-THINKING-Q4K-ONECLOUD-20260930/entrega-01/MANIFIESTO.tsv) fija sus identidades.

## Resultado y límites

| Aspecto | Observación |
| --- | --- |
| Carga e identidad | Diez archivos Thinking Q4K cotejados; 45.350.323.142 bytes. |
| Circuito documental | Dos llamadas reales y reenvíos idénticos; una página recibida de tres disponibles. |
| Respuesta | Título, sección y localizador coincidentes. La cita corresponde tras unir sólo LF/CRLF; no hay igualdad exacta de bytes. Contiene dos oraciones frente a una solicitada. |
| Tiempo de consulta | 5.516,496 segundos desde admisión hasta conservación de la respuesta final. |
| Memoria | Pico de 63.350.767.616 bytes, exactamente 59 GiB; presión contra la cota, OOM 0 e intercambio 0. Sin margen estático demostrado. |
| Cierre | 98 archivos sincronizados; procesos propios cero; carga y arranque automático deshabilitados; unidades inactivas y sockets sin escucha. |
| Telemetría | OpenTelemetry Rust 0.31.0: 1045 registros, errores 0 y descartes 0. Observador exterior: 118 muestras conformes. |

La [respuesta original](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/1cf92e92768938bd304ee22a792d16fc7a12bb9e/respuestas-ejecucion/QWEN80-THINKING-Q4K-ONECLOUD-20260930/entrega-01/RESPUESTA-FINAL.txt) y el [cotejo documental](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/1cf92e92768938bd304ee22a792d16fc7a12bb9e/respuestas-ejecucion/QWEN80-THINKING-Q4K-ONECLOUD-20260930/entrega-01/COTEJO-DOCUMENTAL.json) se conservan sin corregir el contenido producido por el modelo. No se declara cumplimiento literal sin reservas ni aptitud clínica. El razonamiento emitido es texto generado y no observación exhaustiva de los procesos internos.

## Componentes y correcciones

Rust/Cargo remoto 1.98.0; mistral.rs 2370966bb91e2e3dafa0b1521b87c50fd5c01244 y Candle 66a8cf184a5a519671454066b1b9efd446ec9f5c, con las correcciones recibidas y dependencias fijadas. Distribución mistralrs-community/Qwen3-Next-80B-A3B-Thinking-UQFF, revisión 3b8734e41d4a42855a03a8eb14eda91d53ebba86, Q4K. Motor SHA-256 1d32385f0a8947a9b06bbcc95a80e7a9c4cd5ae265c89aaf969f21c84b221112.

La dirección aceptó expresamente el MCP 0.1.2 recompilado, SHA-256 115e2fc8ea91d7b0e76824ebc9b2e17f22518e3da3cf4b9315b190cd4a1ba377, conservando catálogo y HTML congelados. Las excepciones nativas aws-lc-sys 0.37.0 y ring 0.17.14 incorporan C y ensamblador en criptografía de comunicaciones; no son motores de inferencia C/C++.

R8 separó la señal volátil de custodia de la sincronización de los siete archivos probatorios y conservó la fecha de su última confirmación completa. Se mantuvieron controles finitos: período 2 s, confirmación 30 s, pulso 45 s, cierre 40 s y vigilancia 5 s. La compilación remota y 70 pruebas de supervisor, cuatro de conductor y doce controles dinámicos resultaron conformes. No se modificaron tensores, dependencias ni muestreo entre R6 y R8. La causa física de las latencias permanece indeterminada.

## Estado posterior

La instancia, SSH y consola permanecen conservados. El cierre del modelo no elimina la facturación. La eventual retirada de la instancia corresponde a la dirección. La consulta local de RESULTADOS.html presenta primero la respuesta, con el razonamiento separado por intento; funciona sin conexión y sin inferencia. La prueba humana de presentación está pendiente.

Esta actualización sustituye el estado preliminar publicado en c3b68173410c879d5bcd5cac18840626cd902359. La entrega está publicada y cotejada; la recepción independiente y la decisión sobre un examen comparable de TT-0016 siguen pendientes. No se ejecutaron P01–P25 ni se intervino en el ensayo Instruct.


