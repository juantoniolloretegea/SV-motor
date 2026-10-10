# Información operativa exigida al modelo y mediciones independientes

**09/10/2026 · Requisito transversal a los nodos 1, 2 y 3.** En cada tarea se solicita al candidato información sobre los procesos que pueda observar y comunicar, herramientas y scripts ejecutados, etapas, fundamento público verificable, magnitudes de uso y límites. La petición no concede autoridad sobre las mediciones ni modifica la adjudicación científica.

## Tres procedencias conservadas

| Procedencia | Información | Tratamiento |
| --- | --- | --- |
| Modelo | Explicación pública de su procedimiento, fundamento, herramientas o scripts que declara haber usado, límites y observaciones internas accesibles. | Declaración atribuida; requiere evidencia exterior. Código propuesto y código ejecutado se distinguen. No se exige una cadena privada de pensamiento ni se convierte una narración en observación causal. |
| Motor o proveedor | Identidad comunicada, terminación, contadores de entrada, salida, caché y razonamiento, límites de servicio y cargo por tarea cuando los entregue. | Original conservado y normalización Rust. Los subconjuntos no se suman otra vez al total. |
| SV | Tarea, etapa e intento; permisos, corpus y solicitud efectivamente entregados; proceso observado, recursos, conexiones, tiempos, integridad y resultado del receptor. | Instrumentación Rust fuera de la autoridad del candidato, con alcance y disponibilidad de cada medida. |

Los datos ausentes quedan en `null` con causa y origen. Cero sólo se usa cuando haya evidencia de cero. La estimación tarifaria se distingue del importe liquidado; los saldos agregados no se atribuyen a una tarea. Ni la falta de un dato operativo ni un fallo técnico generan por sí mismos el valor semántico U.

## Realización común en Rust

El módulo `observabilidad_modelo.rs` del cliente común contiene la instrucción, el esquema y el registro. `proteger` incorpora la petición antes de adaptar el transporte. `enviar_interno` rechaza, antes de la red, cualquier solicitud sin esa petición en las instrucciones del controlador; su presencia en un documento o mensaje del usuario no basta. Las pruebas comprueban los cinco adaptadores existentes: OpenAI, xAI, Alibaba Cloud, Z.ai y Moonshot AI.

Los contratos nuevos incorporan el objeto JSON `informe_operativo`, separado de la respuesta científica:

- `procedimiento_resumido`: explicación pública, concisa y verificable.
- `procesos_y_herramientas`: nombre, ejecución declarada o desconocida, fuente y evidencia.
- `magnitudes`: nombre, valor o `null`, unidad, procedencia y evidencia.
- `limites`: datos y operaciones que el candidato no puede observar o comunicar.

Un campo de evidencia escrito por el propio modelo conserva su condición de declaración. El controlador debe contrastarlo con los originales y con sus observaciones. El esquema no convierte automáticamente esas afirmaciones en hechos del SV.

Los contratos históricos cerrados no se amplían silenciosamente: mantienen sus campos y sólo admiten información adicional por un canal que el servicio ofrezca. Para solicitar el objeto nuevo en una campaña histórica es necesario preparar y recibir explícitamente la revisión de su contrato. No se alteran respuestas anteriores. Una solicitud antigua sin la petición común se rechaza al enviarla con la versión actual; su revisión debe preceder a cualquier nueva consulta y al cálculo de reserva, que incluye los tokens adicionales.

El transporte actualizado conserva `OBSERVABILIDAD.json` después de recibir una entrega completa, además de los originales y la instrumentación. Si se interrumpe antes, se conservan los registros parciales disponibles: la existencia de ese archivo no se presume. La cobertura de capturas y medidas no equivale a una recepción integral del instrumento ni a conocer todos los procesos internos del proveedor.

## Nodo 1 y condición de recepción

En el nodo 1, el controlador del Árbitro debe usar la misma instrucción antes de tokenizar la tarea y suministrarla al motor local. La función Rust `instrucciones` es independiente del transporte y permite hacerlo sin inventar otro contrato. Debe conservarse tanto el texto compuesto como el suministro efectivamente aplicado; los contadores reales del tokenizador y del motor tienen prioridad sobre cualquier cifra declarada por la IA.

El candidato local permanece en un proceso aislado del SV. No recibe el diario del Árbitro, la clave de adjudicación, los permisos ni capacidad para alterar la telemetría. Hilos, dispositivos, memoria y uso de CPU/GPU se miden desde el observador autorizado cuando estén disponibles. Un proceso local no proporciona acceso automático a todos sus estados internos ni a una explicación causal completa.

La instrucción está implementada y probada localmente; la integración con un motor residente todavía requiere la restauración y prueba autorizadas del nodo 1. La autorización humana del 09/10 mantiene esa restauración como trabajo vigente. No se considera cumplida por la prueba del MCP ni por una llamada API. Nodo 2 conserva el mismo requisito, reservado a laboratorio.

## Corte temporal

La consulta instrumental BIB-API-01 terminó antes de incorporar esta ampliación. Ya conservaba uso del proveedor, canal de razonamiento comunicado e instrumentación, pero no recibió retroactivamente el nuevo objeto operativo. No se ha repetido para obtenerlo. La versión posterior supera 34 pruebas de biblioteca y dos del receptor instrumental; la entrada `sv-biblioteca-prueba-api` se ha recompilado. Los demás ejecutables históricos deberán recompilarse y fijarse por identidad antes de una campaña nueva.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
