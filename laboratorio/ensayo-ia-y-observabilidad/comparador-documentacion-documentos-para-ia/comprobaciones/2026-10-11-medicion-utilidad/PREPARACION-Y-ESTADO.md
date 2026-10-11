# Preparación de la medición de utilidad

**11 de octubre de 2026 · Sesiones de revisión: 0 / 12.**

Se han fijado doce casos documentales y su distribución en dos modalidades de seis casos. Rust comprobó la correspondencia de originales españoles, traducciones conservadas y resultados previos, así como la separación de dominios y el equilibrio global de clases. Se reutiliza un aviso documental del perfil restrictivo. No se ejecutó de nuevo el detector, no se tradujo contenido ni se realizaron sesiones de revisión.

El programa de medición superó dieciocho pruebas distintas: aritmética de reducción y rendimiento, falta de datos, duplicados, dominios y modalidades discordantes, identificadores desconocidos, relojes invertidos y solapados, espera excesiva, duración negativa, denominadores nulos, tokens sin fuente, sesiones fallidas, fundamento pendiente, omisiones, falsas incompatibilidades y citas ausentes. Una ejecución anterior había superado catorce de esos mismos controles; no se suman como controles nuevos. Estos son controles instrumentales, no resultados de utilidad.

Se comprobó el registro de inicio y cierre mediante una calibración separada; el intento deliberado de sobrescribir el cierre fue rechazado. Sus tiempos no se incorporan a las sesiones. El cálculo sobre el registro vacío conserva el estado `sin_sesiones`, sin porcentajes ni consumo ficticios.

Las doce páginas de consulta se construyeron con mdBook 0.5.4. Se entregan las fuentes reproducibles del visor y los materiales JSON, derivados de los mismos originales. La construcción acredita generación; no se presenta como una nueva comprobación visual exhaustiva. Este visor de prueba no modifica el libro editorial ni los servicios administrativos.

## Incidencias de preparación

La primera compilación sin red no encontró una dependencia en el almacén de paquetes predeterminado y no ejecutó pruebas. Se reutilizó el almacén ya disponible en el espacio de trabajo; no se instalaron programas ni se descargaron dependencias. La primera normalización de formato encontró una restricción de acceso; la operación posterior sobre el archivo propio concluyó correctamente. No cambió la semántica del instrumento.

La preparación inicial se conserva localmente como antecedente; la edición final de materiales incorpora el pie de autoría en las páginas. La asignación, los textos originales y los avisos no cambiaron. La primera consulta remota utilizó `README.md`; la entrada existente es `readme.md`. Se recuperó su nombre exacto antes de preparar la actualización.

## Entrega y límites

El [protocolo](PROTOCOLO.md) fija objetivo, condiciones, indicadores e interpretación. El [encargo](ENCARGO-REVISION.md) delimita la revisión. El [instrumento Rust](codigo/src/main.rs) prepara los materiales, registra los intervalos exteriores y calcula los indicadores; sus [instrucciones](USO-DEL-INSTRUMENTO.md) describen la entrada y salida. Las referencias de evaluación se conservan aparte y no se entregan como material de respuesta.

La asignación deliberada garantiza que se examine el único aviso existente; no permite estimar su frecuencia en documentos nuevos. Los grupos tienen distinta dificultad no calibrada. La comparación será descriptiva, con resultados desglosados y sus errores visibles. No se declara todavía ahorro, mejora ni aptitud del comparador. El empleo de IA se limita al revisor experimental externo y no forma parte del funcionamiento del SV.

El siguiente acto es realizar la revisión con el encargo autorizado. El paso 8 de integración mantiene sus pendientes previamente documentados; esta medición no los sustituye. El libro permanece sin cambios.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
