# Nodo 01 · Inferencia bajo control propio

**Versión documental 1.0 · 6 de octubre de 2026.**

**Estado:** especificación preparatoria. Este documento no acredita un motor integrado, un modelo aceptado ni una ejecución en curso.

Este nodo desarrolla la primera modalidad del [marco común de acoplamientos](../readme.md). Sus contratos, fuentes rectoras, correspondencia N-01–N-10 y registros de seguimiento son los allí identificados. La independencia de esta modalidad permite examinarla y aceptarla por separado, conservando las obligaciones del SV y de cada dominio.

## 1. Objeto y alcance

Se estudia el uso de pesos de uno o varios modelos mediante un motor de inferencia cuya instalación, configuración y ejecución administra el proyecto. Puede residir en un equipo propio o en un servidor contratado. La utilización de infraestructura ajena exige examinar sus condiciones de acceso y tratamiento de datos, aunque la inferencia se administre directamente.

El SV establece la función admitida, la información necesaria y las condiciones de actuación; el Lenguaje aporta los contratos aplicables; el Motor concreta su realización experimental. El modelo presta una función auxiliar. Su incorporación al entorno operacional no lo convierte en fundamento matemático, fuente normativa del dominio ni instancia de decisión sobre su propia validez.

El componente de inferencia propio debe satisfacer la exigencia de realización en Rust. La admisibilidad se comprueba sobre dependencias y ejecución efectiva. Que una biblioteca exponga llamadas Rust no demuestra que su motor, sus bibliotecas numéricas o todas sus dependencias estén realizados en ese lenguaje. El uso de controladores, compiladores u otros componentes nativos se declara y contrasta con el alcance autorizado; no se oculta bajo la denominación del programa principal.

## 2. Organización técnica prevista

El recorrido es: operación constituida del SV → admisión y preparación en Rust → motor identificado → modelo → recepción y comprobaciones en Rust → evaluación y decisión competentes.

El motor puede incorporarse como biblioteca o ejecutarse en un proceso separado. La elección requiere justificar aislamiento, transferencia de datos, tratamiento de fallos y capacidad de supervisión. Una API de red administrada por el proyecto sigue perteneciendo a este nodo. No se confunde con el servicio de proveedor externo del [nodo 03](../03-api-directa/readme.md).

La separación de procesos no constituye por sí sola una restricción de permisos. Una integración en el mismo proceso tampoco acredita seguridad de memoria de todas sus dependencias. Ambas realizaciones requieren límites efectivos y pruebas de las operaciones permitidas y denegadas.

## 3. Identificación y condiciones previas

Antes de una ejecución se identifican, al menos:

- Revisión del componente Rust, motor y dependencias efectivamente utilizados, con licencias y procedencia.
- Arquitectura del modelo, versión de los pesos, integridad de los archivos, precisión o cuantización y transformaciones aplicadas.
- Tokenizador, formato conversacional, instrucciones, contexto disponible y parámetros de generación.
- Sistema operativo, procesadores, aceleradores, memoria, almacenamiento y bibliotecas que realmente intervienen.
- Función y dominio, corpus de entrada, criterios de aceptación, casos críticos, límites de ejecución y causas de parada.

Se distingue lo declarado, lo compilado, lo instalado y lo utilizado. La compatibilidad nominal con una arquitectura no acredita una inferencia completa. Que los pesos quepan en memoria no demuestra velocidad suficiente ni calidad del candidato. El mantenimiento y la comunidad del motor forman parte de su valoración técnica, sin sustituir las pruebas de esta configuración.

No se seleccionan aquí proveedor, motor, tamaño de servidor o presupuesto. Esas decisiones pertenecen al encargo de ejecución y necesitan evidencia y autorización aplicables.

## 4. Protección de datos y control de efectos

La preparación inicial emplea datos completamente artificiales. En inmunología se preservan las relaciones relevantes del supuesto sin introducir historias clínicas; en ciberseguridad se utilizan situaciones y registros artificiales sin credenciales, infraestructura privada o información de personas reales.

Los controles del [marco común](../readme.md#6-protección-de-datos-y-límites-tecnológicos) se aplican antes del acceso y durante la operación. Incluyen fuentes, archivos temporales, memoria persistida, registros, cachés, copias y recuperación. Un servidor bajo administración propia no elimina los posibles accesos del proveedor de infraestructura ni las comunicaciones de componentes instalados.

Las herramientas o acciones propuestas por el modelo requieren comprobación independiente de identidad, finalidad y permiso vigente. El texto generado no concede acceso a archivos, red o mecanismos de ejecución. La salida sólo puede incorporarse a una operación del SV después de las comprobaciones que correspondan.

## 5. Pruebas y observaciones necesarias

| Aspecto | Prueba o evidencia requerida |
|---|---|
| Identidad | Reconocer la revisión y los pesos empleados; detectar discrepancias respecto de la configuración autorizada. |
| Fidelidad de entrada | Comprobar fuentes, orden y contenido entregados; detectar truncamiento o desbordamiento del contexto. |
| Permisos | Denegar operaciones fuera de alcance y comprobar que no se produjeron sus efectos. |
| Fallos | Distinguir falta de memoria, interrupción, respuesta parcial, vencimiento del tiempo y rechazo del motor. |
| Recuperación | Conservar el último estado confirmado y evitar duplicar efectos; acreditar por prueba cualquier restauración funcional. |
| Representación | Mantener posiciones, relaciones y diagnósticos previstos por los contratos, sin sustituir un fallo técnico por U. |
| Candidato | Ejecutar el protocolo completo y uniforme, con clave externa reservada y criterios críticos previos. |

Las pruebas instrumentales pueden emplear respuestas controladas para comprobar el acoplamiento sin atribuirlas a un modelo. Las pruebas de un modelo real identifican la variabilidad observada; no prometen igualdad de resultados entre dispositivos, precisiones o motores distintos. Una operación matricial correcta sólo acredita la operación ensayada.

Se medirán, cuando proceda, tiempo de preparación, tiempo hasta la primera respuesta útil, duración total, contexto efectivo, memoria y consumo. Las comparaciones conservarán las variables relevantes o declararán sus diferencias. Las medidas de rendimiento no reemplazan la evaluación semántica ni la suficiencia del conocimiento entregado.

## 6. Aceptación, documentación y retorno

La aceptación exige identificar configuración y evidencia, superar las pruebas instrumentales, comprobar la conformidad semántica y evaluar al candidato para la función delimitada. Los resultados se presentan por separado. Una campaña incompleta conserva sus casos pendientes y no recibe una puntuación global que los oculte.

El expediente material deberá relacionar especificación, realización Rust, manifiesto de archivos y configuración, pruebas, resultados, incidencias y decisión con sus límites. Los sucesos, tiques y actas permanecen en las [sedes de seguimiento existentes](../readme.md#8-documentación-y-retorno-al-sv). Su conservación documental y su recuperación no equivalen a recepción funcional.

Las carencias se clasifican antes de actuar: candidato, motor, recursos, realización del acoplamiento, representación o contrato. Sólo una insuficiencia demostrada del SV o del Lenguaje motiva el procedimiento de evolución competente. El retorno se produce a las necesidades existentes y a su revisión, sin alterar el Núcleo desde este laboratorio.

**Condición de continuación:** seleccionar una realización admisible y un alcance de prueba autorizado, con protección de datos y criterios verificables. La publicación de este README no inicia descargas de pesos, instalación, inferencia o contratación.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
