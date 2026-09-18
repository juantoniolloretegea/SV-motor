# Contrato experimental de inferencia y observabilidad

**Identificador:** EIO-CONTRATO-01 · revisión 1  
**Fecha:** 18 de septiembre de 2026  
**Estatuto:** contrato de ensayo tecnológico; ejecución y recepción pendientes.

## 1. Fuente, rango y continuidad

Se reciben por referencia el [acta de rutas de conocimiento de 14/09/2026](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3d362a01c05644362a8bf77f456b4ecfedd8fd57/docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/ACTA_EVALUACION_Y_RECEPCION_DOCUMENTAL_RUTAS_CONOCIMIENTO_SV_2026_09_14.md) y el [Acta 001 de continuidad de 15/09/2026](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3d362a01c05644362a8bf77f456b4ecfedd8fd57/docs/calidad/tuberias-ia/continuacion-15-09-2026/ACTA_001_CONTINUIDAD_Y_RUMBO_2026_09_15.md), en el corte canónico `3d362a01c05644362a8bf77f456b4ecfedd8fd57`. La [copia de referencia en dominios](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/main/dominios/ACTA_EVALUACION_Y_RECEPCION_DOCUMENTAL_RUTAS_CONOCIMIENTO_SV_2026_09_14.md) facilita la localización y conserva el rango indicado en su fuente; no se crea otra copia del acta.

Rigen asimismo los [Pilares](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3d362a01c05644362a8bf77f456b4ecfedd8fd57/docs/calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md), el [acta de perfiles y contratos](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3d362a01c05644362a8bf77f456b4ecfedd8fd57/docs/calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md) y la [transición secuencial](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/3d362a01c05644362a8bf77f456b4ecfedd8fd57/docs/dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md). Se han consultado sus textos completos en ese corte. La adscripción es (p1+P3)-Bis, con recepción pertinente en S32; no se modifica su estado ni se anticipa el retorno a la adenda del universo 1 de Ciberseguridad.

La autorización de un ensayo no constituye una operación SV, un dominio, un agente ni una garantía de plataforma. Calidad del Lenguaje, Sucesos y RETP conservan la autoridad sobre recepción y continuidad.

## 2. Operación experimental

La operación recibe una petición sintética, un conjunto finito de fuentes sintéticas identificadas y una especificación previa del recorrido exigido. Produce:

1. una propuesta del modelo, conservada como contenido sin autoridad;
2. un registro del recorrido efectivamente expuesto por las interfaces instrumentadas;
3. un informe determinista de cobertura, referencias y fallos;
4. una conclusión experimental limitada a esos controles.

Se distinguen tres objetos: **recorrido exigido**, **recorrido observado** y **justificación presentada**. Su correspondencia no se deduce de la narración del modelo.

La especificación sintética puede exigir dos consultas conjuntas, una consulta condicionada por un dato explícito y un veto. El banco fija sus condiciones y resultados esperados antes de ejecutarse. Son objetos de prueba externos: no células, rutas clínicas ni una realización de la álgebra soberana. No se enumeran todas las rutas posibles de un dominio.

## 3. Correspondencia de obligaciones

| Fuente | Obligación recibida | Evidencia experimental exigida |
|---|---|---|
| Acta de rutas §2 y §4.1 | Registrar consultas no acredita cubrir las dependencias exigidas. | EIO-P-13: control completo y omisión de exactamente una dependencia; la omisión debe detectarse aunque las restantes consultas sean correctas. |
| Acta de rutas §4.1 | Activación, conjunción y veto conservan sus condiciones. | EIO-P-13: condición activa e inactiva con causa registrada; rama conjunta incompleta; veto no compensable por otros resultados favorables. |
| Acta de rutas §3.1, §4.4 y §4.5 | Vincular petición, ejecución y consejo; distinguir evidencia de declaración. | EIO-P-08 y EIO-P-14: referencias válidas, referencia inexistente, versión incorrecta y afirmación de una consulta no realizada. |
| Acta de rutas §2 y §4.5 | El contenido externo o del modelo no adquiere autoridad. | EIO-P-07 y EIO-P-15: instrucciones incrustadas no amplían capacidades; efecto sintético autorizado frente a solicitud sin permiso. |
| Acta de rutas §2 | El agotamiento se declara y no se convierte en U ni en éxito. | EIO-P-04, EIO-P-05 y EIO-P-09: límite, cancelación, pérdida de un evento obligatorio y fallo de exportación. |
| Acta de rutas §3 y §6 | La incorporación de conocimiento conserva su reserva formal. | Exclusión expresa: ningún código de promoción, aprendizaje o modificación del conocimiento activo. |
| Acta de rutas §7 | Una prueba se refiere a una realización identificada. | EIO-P-01 y EIO-P-12: identidades exactas y resultados separados por entorno; ninguna transferencia automática a otros modelos. |
| Acta 001 §7 y §10 | Minimización desde el diseño y recepción con alcance. | Datos sintéticos, destinos declarados, volumen y retención limitados, manifiesto y límites de observación. |

Los identificadores EIO-P-01 a EIO-P-12 califican los códigos locales P-01 a P-12 de la [matriz](../pruebas/README.md); no reasignan códigos históricos de otros expedientes.

## 4. Responsabilidades y límites de observación

El modelo genera contenido. Un adaptador determinista comprueba estructura y ligaduras del contrato experimental. Un ejecutor sintético separado ejerce únicamente operaciones sobre datos de prueba en memoria. El permiso de prueba procede del banco previamente fijado, nunca de texto generado por el modelo. No se atribuye a este mecanismo la condición de implementación de R1.

OpenTelemetry Rust registra los puntos instrumentados. No es el decisor de permisos, un monitor del sistema operativo ni una prueba independiente frente a la alteración conjunta del programa y su registro. El ensayo no sustituye el contraste pendiente de capturas del equipo ni acredita retrospectivamente otra ejecución.

El sistema operativo, el navegador y el ejecutor de integración conservan sus responsabilidades de aislamiento y recursos. Una magnitud no observable se declara como tal. En particular, ninguna traza interna permite concluir por sí sola que no hubo otra conexión o escritura.

## 5. Datos, comunicaciones y licencias

Sólo se admiten fuentes y peticiones sintéticas publicables. Los casos se conservan como datos del banco; no se duplican automáticamente en la telemetría. Se registran identificadores, versiones, decisiones y medidas, con el [presupuesto](../README.md) y sin datos personales, credenciales ni contenido privado.

La adquisición de dependencias y pesos se distingue de la inferencia. Se identifican los destinos y los artefactos obtenidos; durante la inferencia no se ofrece al modelo ninguna herramienta de red ni acceso general al sistema de archivos. Esta restricción de interfaz no acredita aislamiento de red impuesto por el anfitrión.

Se documentan las licencias exactas de bibliotecas, dependencias, pesos, tokenizador y código reutilizado. Se preservan los avisos exigidos. Este expediente no modifica la licencia del SV ni presupone que la licencia de una biblioteca cubra el modelo o todos sus componentes.

## 6. Suficiencia y salida

Cada obligación recibe un resultado: satisfecha en el caso observado, incumplida, no ejecutada o no comprobable. Se separan compatibilidad técnica, corrección de los controles y calidad del consejo.

La comprobación de una referencia no demuestra la verdad de una afirmación ni la fidelidad semántica completa de una respuesta. Esa valoración exige sus propios criterios; no se sustituye por un campo de confianza generado por la IA.

Una campaña satisfactoria sólo permite proponer la continuación de esta combinación exacta en el alcance ensayado. Navegador, WASI, Windows, Linux y macOS conservan comprobaciones específicas. No se promueve una familia de modelos, una integración productiva ni el cierre de S32, BIS-03 o de la reserva de incorporación.
