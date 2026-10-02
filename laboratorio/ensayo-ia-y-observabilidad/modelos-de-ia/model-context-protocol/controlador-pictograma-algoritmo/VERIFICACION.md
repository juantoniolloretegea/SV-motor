# Comprobaciones del controlador y del sistema compuesto

**Edición 1 · 2 de octubre de 2026.** Plan de comprobación, sin resultados ejecutados. [Diseño](DISENO.md) · [Presentación](README.md).

## 1. Componente local sin inferencia

Implementar las obligaciones del diseño en Rust y comprobarlas con documentos, recibos y respuestas simulados. El programa debe admitir un recorrido conforme y bloquear recorridos inválidos por su causa correcta; bloquear todo no constituye éxito.

| Situación introducida | Resultado exigido |
| --- | --- |
| Falta una página obligatoria. | Bloqueo antes de inferir; identificación de la ausencia. |
| Hay una página duplicada en lugar de la faltante. | Bloqueo por conjunto incompleto, aunque coincida la cantidad. |
| Documento o revisión distintos de los fijados. | Bloqueo de identidad, sin sustitución silenciosa. |
| Contenido modificado respecto de su identidad fijada. | Bloqueo de integridad. |
| Página descargada pero ausente de la entrada efectiva. | Bloqueo de incorporación. |
| La plantilla o un ajuste posterior trunca la entrada. | Bloqueo antes de la generación. |
| El contenido íntegro no cabe con la reserva fijada. | Bloqueo de capacidad, sin resumen automático. |
| Una fuente intenta ordenar cambios de política o lectura exterior. | La fuente conserva condición de dato; no altera reglas ni acceso. |
| Llega un recibo o una respuesta perteneciente a otro caso. | Rechazo de la mezcla por identidad de ejecución y caso. |
| Aparece una emisión antes de completar las precondiciones. | No se presenta como respuesta admitida; incidente conservado. |
| La salida está mal formada o usa categorías no permitidas. | Original conservado y salida no admitida, sin reparación ni nueva generación automática. |
| Cita inexistente, referencia ajena o fragmento modificado. | Resultado negativo de procedencia; no admisión como conforme. |
| Se pierde la conservación, hay interrupción o no se acredita finalización. | Detención con estado alcanzado; no se deduce éxito de un archivo parcial. |
| Se reanuda un proceso con una ejecución ya concluida. | Se impide duplicar la inferencia; la identidad no se reutiliza para obtener otra respuesta. |
| Formato, cita y cobertura conformes, pero excepción ignorada. | Se distingue admisión instrumental de incorrección semántica. |
| Recorrido conforme completo. | Una sola consulta simulada, admisión y entrega a evaluación. |
| Clasificación correcta de evidencia insuficiente en un caso previsto. | Se permite su evaluación conforme al criterio; no se fuerza una cita inventada ni se convierte automáticamente en U. |

Comprobar también que ninguna vía de presentación final eluda la admisión, que las esperas tengan límites técnicos y que cualquier recuperación conserve identidad, incertidumbre y originales. La cobertura de pruebas se describirá con precisión: una lista de casos no acredita exploración exhaustiva de todos los estados posibles.

**Condición de continuación:** todas las obligaciones exigidas tienen comprobación significativa, los casos inválidos se detectan, existe un recorrido conforme y se conservan fuentes, configuración y resultados. Si falla esta etapa, se corrige el instrumento; no se utiliza inferencia del candidato para depurarlo.

## 2. Integración efectiva

Antes de consultar al modelo, comprobar la identidad actual de la instalación y las versiones realmente usadas; contrato MCP; fronteras de observación; entrada después de plantilla y tokenización; canales y finalización; aislamiento; límites de memoria y contexto; conservación, recuperación y cierre.

Debe distinguirse lo declarado, lo compilado, lo instalado y lo efectivamente utilizado. La existencia de una biblioteca, una prueba estática o un cliente documental no demuestra integración con el modelo. La instalación de referencia es un punto de partida, no una confirmación de disponibilidad actual.

El control debe poder demostrar entrega efectiva sin consultar la clave reservada ni añadir ayuda después de observar la respuesta. No se modifica el banco de otro ensayo. La introducción de una gramática de generación exige comprobar por separado que no elimina emisiones ni altera el formato conversacional requerido.

**Condición de continuación:** recorrido legítimo completo y observado, con capacidad suficiente dentro de las cotas existentes y sin tramos instrumentales no auditables. Si exige recursos nuevos, modificaciones del banco o una recuperación no contemplada, debe delimitarse esa nueva decisión antes de actuar.

## 3. Contraste inicial finito

Se proponen exactamente tres casos nuevos y reservados: uno con afirmación respaldada, uno contradicho por una condición o excepción y uno con evidencia insuficiente dentro del corpus fijado. La ficha, política, clave de corrección, criticidad y criterios se conservarán antes de la inferencia. El ejemplo explicativo del diseño queda excluido del banco.

Una consulta prevista por caso, con contexto independiente y configuración identificada. Conservar cada resultado original; no abrir rondas de recordatorios, reparación o repetición para mejorar respuestas. Un incidente instrumental exige conservación y diagnóstico; este diseño no concede por sí solo repetición automática. El encargo ejecutable deberá fijar la identidad y ubicación de su entrega antes de comenzar.

Se informará por separado de cobertura documental, incorporación efectiva, admisión instrumental, clasificación factual, fundamento, cumplimiento de la política, errores críticos y resultado conforme al criterio SV aplicable. Los impedimentos y los casos no ejecutados tendrán sus propios recuentos. Tiempo, memoria, intervenciones y errores se incluirán como magnitudes instrumentales.

La puntuación no sustituye el criterio de aptitud. La regla SV de aptitud que corresponda al banco debe constar expresamente en el encargo; tres casos de funcionamiento no forman por sí solos una célula completa ni permiten declarar aptitud general. No se suman los resultados nuevos a los contrastes históricos ni se compensan incumplimientos críticos con aciertos de otra etapa.

## 4. Decisión sobre utilidad

El primer contraste pregunta si el sistema compuesto funciona y conserva correctamente su separación de responsabilidades. Si la entrega es conforme y la interpretación falla, se registra ese fallo; no se justifica repetir indefinidamente para mantener al candidato en evaluación. Si la instrumentación falla, el resultado no demuestra incapacidad semántica del modelo.

Tres casos no estiman fiabilidad general ni acreditan una mejora causal frente a experimentos con bancos distintos. Una comparación de estrategias o de pictogramas requeriría un diseño específico posterior, con contenido equivalente y criterios previos. No queda abierta por completar este contraste.

## 5. Entrega y recepción

La entrega de ejecución conservará originales, fuentes, versiones, entradas efectivas, intercambios documentales, emisiones, decisiones, errores y telemetría; incluirá copia recuperada y cotejada desde la sede de custodia. Los resultados se presentarán en texto legible y datos estructurados.

Sedes: encargo y evidencias de ejecución en SV-sala-de-maquinas; diseño común en esta carpeta de SV-motor; aplicación y resultados en el expediente del modelo; sucesos, tiques y Calidad en sus registros existentes. Para Safeguard se conserva la relación con TT-0018, TT-0014 si afecta a la interfaz documental, y S39. No se declara cerrada una recepción pendiente por publicar el diseño.

---

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es). Los componentes de terceros conservan sus licencias.
