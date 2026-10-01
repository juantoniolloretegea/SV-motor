# Criterio común de puntuación final de los modelos

**SV · revisión 1 · 1 de octubre de 2026.**

Cada modelo y configuración examinados deben presentar una **puntuación final sobre 100 y un dictamen SV**, además del detalle de respuestas. Este criterio se aplica inicialmente a Qwen3-Next-80B-A3B-Instruct y se conservará para los modelos siguientes.

## Cálculo

Sean **A** los aciertos, **Eₙ** los errores no críticos, **E꜀** los errores críticos y **N** el número de preguntas fijadas en el banco antes de la prueba:

**Puntos netos = A − Eₙ**

**Puntuación final = 100 × (A − Eₙ) / N**

- Un **0** de la terna es un acierto y aporta **+1 punto**.
- Un **1 no crítico** resta **1 punto**.
- Una respuesta **U** o en blanco aporta **0 puntos** y no resta.
- Un solo **1 crítico** determina **No apto**, cualquiera que sea la puntuación. Su efecto es eliminatorio y se registra separado de las deducciones por errores no críticos.
- Los impedimentos técnicos se registran fuera de la terna, sin convertirlos en errores, blancos ni U. No aportan puntos obtenidos. Se informa siempre cuántas respuestas finales existen respecto del banco completo.
- Se conserva N como máximo del banco; no se reduce al número de respuestas. Así se evita aumentar artificialmente la puntuación excluyendo abstenciones o preguntas sin resultado técnico. La cifra normalizada expresa puntos netos respecto de ese máximo, no una tasa de exactitud entre respuestas ni una probabilidad clínica.
- No se ocultan saldos negativos ni se redondea antes del cálculo final.

La ficha debe mostrar inseparablemente **puntuación + dictamen + número de errores críticos + cobertura**. Una puntuación alta no compensa un error crítico.

## Relación con la terna y el dictamen SV

El criterio aritmético añadido el 01/10 no reescribe los valores originales 0, 1 y U, ni convierte una campaña incompleta en un vector completo. Tampoco sustituye la clasificación algebraica κ ni aplica a los puntos netos sus umbrales, definidos sobre recuentos de la terna. Los antecedentes conservan la regla con la que se redactaron; esta revisión introduce expresamente la deducción solicitada para los errores no críticos.

La ausencia de errores críticos no basta, por sí sola, para declarar Apto: se mantienen las demás condiciones de admisión del protocolo correspondiente. En el banco EVAL-PDQ-HCL-25-20260929/r1 se exige que todas las preguntas críticas estén en 0; una U crítica mantiene la indeterminación prevista por ese protocolo.

## Comparación entre modelos

Se emplearán la misma fórmula, criticidad previa y reglas de corrección. La comparación directa requiere el mismo banco, fuente y condiciones declaradas; normalizar a 100 no equipara exámenes de distinta dificultad. Un diagnóstico posterior se informa aparte y no sustituye las primeras respuestas ni mejora retrospectivamente el examen.

Primera aplicación: [Qwen3-Next-80B-A3B-Instruct: 28/100 — No apto](qwen/qwen3-next-80b-a3B-instruct/tests-y-pruebas-efectuadas/PUNTUACION-FINAL-20261001.md).
