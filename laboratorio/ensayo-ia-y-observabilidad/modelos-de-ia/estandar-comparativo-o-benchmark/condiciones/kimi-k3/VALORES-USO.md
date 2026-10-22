# Kimi K3: condiciones de uso para EET-CYB16

Fecha de comprobación: **22 de octubre de 2026**. Modelo solicitado: `kimi-k3`. Categoría adaptada a Markdown, 33 %. Estos valores describen las condiciones del candidato en esta convocatoria; no son propiedades permanentes del modelo ni resultados del examen.

| Condición de la cuenta | Tier0, antecedente | Tier1, vigente y comprobado |
|---|---:|---:|
| Tokens por minuto | 500.000 | 2.000.000 |
| Tokens por día | 1.500.000 | Sin límite diario declarado |
| Peticiones por minuto | 3 | 100 |
| Solicitudes simultáneas | 1 | 15 |

La tabla de niveles del proveedor vincula Tier1 a 10 USD de recarga acumulada. La recarga acumulada, el saldo y el coste de una prueba son datos distintos. Los datos administrativos pueden demorarse diez minutos. No se ha medido una cuota diaria restante y no se sustituye su ausencia por cero. Otros niveles o cambios futuros requieren comprobación de la [tabla oficial](https://platform.kimi.ai/docs/pricing/limits); no se presuponen efectivos en esta cuenta.

El [cómputo de admisión](https://platform.kimi.ai/docs/introduction#rate-limits) considera entrada más salida máxima solicitada. El recuento conservado es de 72.759 tokens; el contexto es 1.048.576. Reserva máxima y elegida: **1.048.576 − 72.759 = 975.817 tokens**. La solicitud representa 1.048.576 tokens para ese control: supera Tier0 y cabe en Tier1. El uso facturado es distinto de esta reserva.

A [tarifas consultadas](https://platform.kimi.ai/docs/pricing/chat) de 3 USD por millón de entrada, 15 por millón de salida y una hipótesis conservadora de escritura íntegra de caché de cinco minutos a 3 por millón:

- Máximo teórico: (72.759 × 3 + 975.817 × 15 + 72.759 × 3) / 1.000.000 = **15,073809 USD**.
- Escenario económico expresamente elegido: (72.759 × 3 + 100.000 × 15 + 72.759 × 3) / 1.000.000 = **1,936554 USD**.

No incluyen impuestos. El segundo es un escenario para comprobar cobertura; **no predice el consumo ni reduce la reserva de salida**, y no garantiza que cualquier respuesta quede cubierta por el saldo. Se ha autorizado continuar con ese criterio, sin cota administrativa; el consumo real y su conciliación se conservarán por separado. El Evaluador debe comprobar crédito y cuotas justo antes de lanzar. Una respuesta truncada o una incidencia no se disimula mediante una cifra de consumo supuesta.

La recepción del contador y su revisión local conservan el original. No se ha repetido el recuento. El intento único ha finalizado con dieciséis aciertos y dictamen 0. [Resultado y evidencias](../../resultados/kimi-k3/cyb16-56831d2845b3/resultado.json). La previsión de 1,936554 USD y la reserva de 975.817 se aceptaron antes del lanzamiento; son datos previos, distintos de los 72.797 tokens de entrada y 9.562 de salida comunicados al terminar. La API declaró además 2.150 tokens de razonamiento y 72.704 de escritura de caché; no se suman de nuevo al total comunicado de 82.359. El cargo individual y el coste comparable quedan pendientes de conciliación. Tiempo efectivo: 230.957 ms; transporte: 230.245 ms; sin pausas por incidencia. No hubo truncamiento comunicado. Este resultado no autoriza producción.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).