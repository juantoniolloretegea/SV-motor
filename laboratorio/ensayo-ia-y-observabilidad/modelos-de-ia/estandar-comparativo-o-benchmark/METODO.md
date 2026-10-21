# Método de comparación

Edición general `eet-cyb16-multiformato-20261021-v2` y edición adaptada `eet-cyb16-markdown-adaptada-20261021-v2`. Dominio: Ciberseguridad Inteligente. Servicios externos de proveedores, Nodo 3.

Las reglas actuales permiten recursos internos declarados del proveedor sin Internet ni acceso o ejecución en el SV. Las explicaciones de cada U identifican carencias o limitaciones; las declaraciones y las mediciones se presentan por separado. Véanse las [reglas comunes](readme.md#reglas-comunes-de-las-ediciones-vigentes).

## 1. Condiciones comunes

Los ocho candidatos reciben las mismas dieciséis preguntas, instrucciones y regla de corrección. Dentro de cada categoría se conservan las mismas fuentes completas y cuadernillos: multiformato en la general y adaptación Markdown revisada en la adaptada. En la general, los formatos de fuente y cuadernillo se cruzan conforme a [EDICION.json](EDICION.json). El orden de entrega común se registra antes del examen; el corrector recupera el orden de identificadores para comparar y representar los resultados.

Se conservan las adaptaciones técnicas necesarias para cada API, con reconstrucción y cotejo de los bytes originales. Una comprobación local de esa reconstrucción no acredita recepción efectiva del servicio. Contexto, compatibilidad documental, identidad y configuración requieren comprobación antes del lanzamiento.

La entrega al candidato excluye las soluciones. Las reglas prohíben Internet, seguimiento de enlaces y memoria de otros episodios; admiten recursos internos del proveedor compatibles con esas restricciones y exigen declarar los que puedan identificarse. No se permite ejecutar herramientas ni acceder a recursos del SV. El SV puede controlar las herramientas que habilita y los datos que transporta; en Nodo 3 no observa de extremo a extremo el funcionamiento interno del proveedor. La prohibición formulada al modelo no acredita por sí sola ausencia de acceso interno.

## 2. Respuesta y corrección

La respuesta 0 indica conformidad con la fuente, 1 indica disconformidad y U declara indeterminación. El corrector compara cada respuesta con la clave fijada: acierto → calificación 0; error → 1; U → U.

Se solicitan explicaciones por pregunta y una revisión adversarial identificada por pregunta como valor inicial, ampliable antes del intento. Se entregan en la misma respuesta conjunta y se conservan sin puntuación. No se solicitan nuevas llamadas automáticas para completarlas. Lo declarado por el modelo no equivale a una medición independiente de sus procesos.

La regla **SV-FUNDAMENTOS-T7N9-CRITICIDAD-1** utiliza T(n)=⌊7n/9⌋. Para n=16, T=12: doce errores determinan κ=1; doce aciertos, κ=0; el resto, κ=U. El contrato añade un veto por error crítico y una reserva por U crítica. Se conservan clasificación matemática κ y dictamen contractual por separado. Las [soluciones](ediciones/ciberseguridad/eet-cyb16-multiformato-20261021-v2/SOLUCIONES.md) identifican las ocho preguntas críticas.

Una respuesta íntegra, correctamente asociada y formalmente inadmisible recibe 1 por incumplimiento formal, sin inventar dieciséis errores ni un polígono. Un fallo del transporte, la preparación o la observación obligatoria conserva lo recuperado y deja el intento sin dictamen ni clasificación. Una causa desconocida no se atribuye al candidato.

## 3. Tiempos y mediciones

Valores recibidos para esta edición:

| Variable | Valor y significado |
| --- | --- |
| Tiempo efectivo total | 1.920 segundos: 32 minutos. |
| Referencia por pregunta | 120 segundos; no establece un corte individual. |
| Incidencia | Hasta 300 segundos para resolver cada incidencia acreditada. |
| Pausa | El tiempo efectivo total se pausa durante la incidencia. |
| Observación | Continúa durante las pausas y debe conservar inicio y cierre. |

El tiempo efectivo, las pausas, la duración del transporte y el primer fragmento HTTP se publican por separado. El primer fragmento no acredita el primer token generado. No se inventan tiempos por pregunta cuando el protocolo no informa de su avance.

Tokens de entrada, salida, caché y proceso mantienen sus nombres nativos y procedencia. Los subconjuntos no se suman de nuevo a sus totales. Un contador no comunicado permanece ausente. Los tokenizadores de distintos servicios no constituyen unidades lingüísticas idénticas.

La observación OpenTelemetry del supervisor conserva inicio, muestras y cierre locales. CPU, memoria y E/S describen el proceso compartido del Gestor; no se atribuyen a la infraestructura del candidato. El fallo de esta observación obligatoria detiene el intento. La ausencia de un contador opcional del proveedor se registra como no disponible.

## 4. Ordenación y costes

El criterio **SV-CLASIFICACION-EET-CYB16-3** mantiene clasificaciones independientes por categoría, admite a recomendación sólo Aptos (0) y compara, sucesivamente:

1. Más aciertos.
2. Menos errores.
3. Menor tiempo efectivo.
4. Menor coste general comparable.

Una ventaja posterior no compensa una desventaja anterior. Tokens y área del polígono no puntúan. Una duración necesaria ausente impide saltar al coste. Si faltan costes comparables en un grupo empatado, el grupo conserva el empate; no se completa el orden mediante nombre, proveedor o posición de examen.

El coste general abarca el examen completo, antes de impuestos, con tarifa general, moneda, fecha común, fuente y cálculo identificados. Descuentos, créditos particulares y recursos aportados no mejoran posiciones. Importe comunicado, coste general calculado y gasto facturado y conciliado se distinguen. Un saldo agregado no se imputa automáticamente al examen; la falta de dato no equivale a cero.

La comparación permanece provisional mientras existan candidatos pendientes, incidencias o varios intentos sin resolución trazable. No se elige automáticamente el mejor intento. Sólo se recomienda un primer Apto inequívoco por categoría cuando su comparación está completa. Los porcentajes 33 % y 66 % comparten la clasificación adaptada; no puntúan ni desempatan.

## 5. Representación y límites

El vector y el polígono representan la misma instancia, contrato y posiciones: **Frame_C=(frmat, frvis)**. Se conserva la convención 0 rojo/radio 1; 1 verde/radio 2; U azul/radio 3, primer vértice arriba, recorrido horario y cierre 16→1. Los radios codifican símbolos, no porcentajes de calidad. Los gráficos se derivan de resultados efectivos y remiten a sus datos.

En la categoría general, el cruce 6/5/5 cubre nueve combinaciones, pero no separa causalmente formato, materia e idioma. Una única ejecución no acredita superioridad universal ni significación estadística. Si un problema se presenta en todos los candidatos, se revisan fuentes, preparación, transporte y posibles límites compartidos antes de atribuirlo al proveedor.

Preguntas y soluciones públicas facilitan la revisión, pero no demuestran ausencia de exposición previa del modelo. Los resultados históricos conservan su edición y no se incorporan como resultados de ésta.

## 6. Consulta y participación

El [libro de evaluación](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/) desarrolla el procedimiento general. Esta sede conserva la edición concreta y sus evidencias publicables.

Los proveedores pueden [solicitar una evaluación](https://administracion.itvia.online/benchmark/proveedores). Presentar una solicitud o aportar recursos no compra posiciones ni garantiza aceptación o un resultado favorable. La recepción de la solicitud no contrata servicios ni inicia pruebas.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
