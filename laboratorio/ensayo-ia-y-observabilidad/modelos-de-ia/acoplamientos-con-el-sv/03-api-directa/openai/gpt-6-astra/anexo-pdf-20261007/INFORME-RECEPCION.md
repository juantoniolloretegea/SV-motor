# Recepción técnica del suministro PDF en Rust

Fecha: 07/10/2026. Expediente: S39 / TT-0021. Alcance: adaptación y comprobación local del suministro documental; no ejecución del candidato.

## Resultado y autoridad

Se ha extraído de nuevo el original y recorrido el MCP real: **diez páginas físicas, treinta fragmentos distintos y nueve solicitudes previstas**, recompuestas y cotejadas en Rust. Las páginas de cada pregunta llegan completas, con su procedencia y localizadores. El recorrido no depende de suponer dos o tres fragmentos: sigue el cursor devuelto hasta el final de cada sección y rechaza omisiones, repeticiones y finales anticipados.

El auxiliar `DireccionPdf` aplica la admisión documental del Árbitro-Director: identidad del banco y del PDF, integridad de las páginas, aislamiento, reproducción del diario e integridad de la instrumentación. La composición de solicitudes permanece impedida hasta satisfacer esas condiciones. Esta adaptación no modifica ni vuelve a examinar la doctrina del Árbitro y no ejerce adjudicación científica.

El candidato no interviene en ese recorrido. Su proyección contiene exclusivamente instrucciones, pregunta, identidad de la fuente y fragmentos completos de las páginas asignadas; carece de herramientas. La clave, los controles, la telemetría y las respuestas anteriores quedan fuera de esa proyección. El proceso de control sí conserva acceso local a la clave para cotejar su huella: no se confunde esa separación de contexto con un aislamiento adicional del sistema de archivos.

La admisión documental **no autoriza el envío, la inferencia ni la continuación automática**. Esta realización no contiene autenticación ni transporte al proveedor. Antes de ejecutar el anexo deberá conectarse al transporte existente y cotejarse la solicitud efectiva inmediatamente antes del envío. No basta con reutilizar una constancia histórica. Se mantienen independientes las revisiones, la adjudicación y el examen.

## Fuente y lectura completa

Fuente única: ficha LLS FS16-S, edición FS16S 8/18, *Leucemia de células peludas*, diez páginas, 157315 bytes. SHA-256: `21322ed388c999bd0713ba5ec5c7ab34402d4bb4cd6100903aa5ea6d8bf35d9c`. Es una fuente histórica para evaluar fidelidad documental, no una actualización de práctica clínica.

Cada página se reconstruye conservando Unicode, orden, intervalos de caracteres y final explícito. Se coteja tanto con la extracción nueva como con el catálogo y su huella. Las dos representaciones de cada respuesta MCP deben coincidir. El diario encadenado se reproduce, y sus tramas se comparan con los bytes realmente transmitidos y recibidos.

Los treinta fragmentos se leen una vez en la recepción integral. Las nueve solicitudes contienen treinta y tres apariciones de fragmentos porque algunas preguntas comparten páginas. PDF06 recibe las páginas 5 y 8; PDF08, las páginas 6 y 7. Las restantes reciben una página completa. Ninguna solicitud contiene las diez páginas indiscriminadamente.

Se conserva el límite del preparador de **64 páginas físicas**, distinto del contexto disponible del modelo. No hay OCR ni evaluación de comprensión visual. La incidencia textual de la página 2 y su exclusión de las preguntas puntuables se mantienen conforme a la [propuesta](PROPUESTA-ANEXO.md).

## Componentes efectivamente utilizados

| Componente | Función verificada | Estado |
| --- | --- | --- |
| Preparador PDF en Rust, variante 0.1.4 | Identidad del original, extracción y catálogo completos | Compilado y ejecutado en Linux/WSL |
| MCP documental 0.1.4 | Protocolo real y suministro por fragmentos; aislamiento de conexiones | Compilado y ejecutado en Linux/WSL |
| Verificador del diario | Reproducción de 101 eventos y 33 tramas | Compilado y ejecutado en Linux/WSL |
| `sv-suministro-pdf-astra` 0.1.0 | Recepción, admisión y composición limitada por el Árbitro-Director | Compilado y ejecutado en Windows |
| Instrumentación SV en Rust | Proceso propio, CPU, memoria, E/S y TCP; diario encadenado | Incorporada al ejecutable y utilizada |
| Transporte del proveedor y candidato | Inferencia y entrega del modelo | No utilizados en esta comprobación |

Los ejecutables utilizados están identificados por tamaño y SHA-256 en el recibo original; la proyección pública conserva esas identidades sin datos de sesión. El código nuevo incorpora copias exactas del analizador JSON estricto y de la biblioteca de instrumentación ya existentes; [PROCEDENCIA.json](PROCEDENCIA.json) identifica sus originales y huellas. No se altera su sede anterior. La [fuente canónica del MCP](https://github.com/juantoniolloretegea/SV-motor/tree/3f12e5054523f313ce148d37f0bdaee16bcff98a/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/model-context-protocol/0.1.4-pdf-preparacion) conserva el preparador, el protocolo y la variante del extractor.

Las decisiones y comprobaciones del recorrido son Rust. WSL y el sistema operativo son el entorno de ejecución; las operaciones administrativas de archivo no sustituyen controles del SV. No se declara que todas las dependencias transitivas o llamadas al sistema estén implementadas íntegramente en Rust.

## Comprobaciones y observación

**52 pruebas distintas conformes:** 16 del adaptador y analizador estricto, 29 del MCP y 7 del extractor. Se han comprobado, entre otros casos, pérdidas, duplicación, desorden, cursores y offsets incorrectos, huellas o fuente ajenas, discrepancia entre representaciones MCP, alteración de clave o telemetría y composición antes de la admisión. También se rechaza añadir herramientas, historial o clave a la solicitud, o modificar sus instrucciones o modelo.

La primera recepción local se conserva. Tras incorporar las dependencias de fuente necesarias para conservar el paquete, la recepción definitiva r2 volvió a recorrer el documento y a recomponer las nueve solicitudes. Se verificó después mediante una segunda operación Rust independiente de la preparación, sin inferencia. Las repeticiones técnicas no se cuentan como pruebas distintas adicionales.

En r2 se obtuvieron **24 muestras, 68 registros, cero fallos de medición y 275 ms de intervalo máximo**, frente a un objetivo de 250 ms. La extracción observada duró 4806 ms y el proceso MCP de suministro 696 ms. Son duraciones locales de procesos invocados, no tiempo de respuesta del candidato ni coste facturable. La denegación de conexiones externas y locales se comprobó en el MCP con error EPERM.

La instrumentación observa el proceso Windows propio; los identificadores registrados al invocar WSL corresponden al transporte Windows, no acreditan por sí solos CPU o memoria individual de los procesos Linux. No se observan recursos internos del proveedor, hilos y manejadores del sistema operativo, asignaciones individuales de memoria Rust, retransmisiones o RTT de TCP, ni tiempos DNS y TLS separados. La constancia conserva `plataforma_recibida: false`: la custodia de las muestras es conforme, pero no se convierte por ello en calibración integral ni certificación externa del instrumento. El extractor conserva avisos de compilación anteriores; no se afirma compilación sin avisos.

## Conservación y continuación

El [resumen verificable](RECEPCION.json) conserva resultados, identidades e inventario. Los originales locales incluyen extracción, catálogo, fuentes, solicitudes y respuestas MCP, diario, recibos de procesos, telemetría y las nueve solicitudes previstas. Su conservación local se distingue de la publicación de esta proyección: no se afirma custodia remota de originales que no se hayan recuperado y cotejado.

La clave del evaluador y el texto íntegro del PDF no se duplican en esta publicación. La clave autorizada y los componentes MCP fijados siguen siendo necesarios para repetir el recorrido integral; las pruebas del adaptador pueden compilarse con el banco incluido. La ejecución usa rutas del entorno local de pruebas declaradas en `src/main.rs`; esta recepción no equivale a un instalador genérico.

El informe individual SV-GASTO-20261007-011 registra cero llamadas y cero tokens nuevos del candidato. Consumo de asistencia, créditos e importe atribuible sin desglose permanecen desconocidos. No se han modificado pagos, permisos de créditos ni recursos contratados.

Siguiente dependencia técnica: recepción del acoplamiento de este suministro con el transporte y la observación de inferencia, conservando el cotejo previo por caso y la autoridad del Árbitro-Director. El banco PDF01–PDF09 tendrá adjudicación propia; no hereda el 100/100 de A0. El polígono (9,3) se generará únicamente tras nueve adjudicaciones completas.

Los derechos del PDF corresponden a sus titulares originales. La atribución siguiente se aplica a la documentación y realización propias, sin sustituir licencias de terceros.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
