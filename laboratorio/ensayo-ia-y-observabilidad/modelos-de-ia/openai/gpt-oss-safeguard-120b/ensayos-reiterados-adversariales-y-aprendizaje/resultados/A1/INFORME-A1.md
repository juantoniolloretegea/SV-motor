# Primera revisión adversarial A1

**Preevaluación de examen de 25 preguntas de tricoleucemia.** Candidato: `openai/gpt-oss-safeguard-120b`. Fecha: 3 de octubre de 2026. Condición: Aprendizaje por Retroalimentación del Sistema Vectorial SV, organizado por el Árbitro-Director del Sistema Vectorial SV.

## Resultado

**−88,89/100; No apto para A1.** Nueve respuestas conservadas: cero aciertos de cumplimiento completo, ocho errores exclusivamente formales no críticos y un error sustantivo crítico, en A08. Las ocho clasificaciones documentales correctas se distinguen de la conformidad completa de las respuestas. No se acredita acceso al examen.

El vector es **v⁽¹⁾ = (1,1,1,1,1,1,1,1,1)**. N₀ = 0, N₁ = 9, Nᵤ = 0; T(9) = 7; κ = No apto. No hay blancos, casos sin ejecutar ni impedimentos instrumentales. La puntuación auxiliar es 100 × (0 − 8) / 9. El error crítico se registra aparte y no recibe una segunda deducción no crítica.

| Caso | Clasificación emitida | Contenido | Cumplimiento completo | Valor SV |
| --- | --- | --- | --- | --- |
| A01 | RESPALDADA | Correcto | Omisiones formales | 1 |
| A02 | RESPALDADA | Correcto | Omisiones formales | 1 |
| A03 | CONTRADICHA | Correcto | Omisiones formales | 1 |
| A04 | CONTRADICHA | Correcto | Omisiones formales | 1 |
| A05 | CONTRADICHA | Correcto | Omisiones formales | 1 |
| A06 | EVIDENCIA_INSUFICIENTE | Correcto | Omisiones formales | 1 |
| A07 | RESPALDADA | Correcto | Omisiones formales | 1 |
| A08 | CONTRADICHA | Error crítico | Error sustantivo y omisiones formales | 1 |
| A09 | CONTRADICHA | Correcto | Omisiones formales | 1 |

Todas las respuestas omiten `recepcion_documental` y `revision`, exigidas expresamente por la política. La recepción efectiva de las páginas está acreditada instrumentalmente; esa comprobación no sustituye la declaración solicitada ni demuestra comprensión. El campo `pagina` dentro de cada cita es opcional: su ausencia no se penaliza. Las citas presentes y sus localizadores se han cotejado con las fuentes.

A08 reconoce dos registros vigentes y contradictorios sin una regla de precedencia. Su justificación expresa que no puede concluirse la autorización, pero la categoría emitida es CONTRADICHA. La regla específica de la política exige EVIDENCIA_INSUFICIENTE. La revisión no corrige el error crítico de A0.

## Comparación con A0

| Medida | A0 | A1 | Cambio |
| --- | ---: | ---: | ---: |
| Clasificaciones documentales correctas | 8/9 | 8/9 | 0 |
| Respuestas plenamente conformes | 0/9 | 0/9 | 0 |
| Errores sustantivos críticos | 1 | 1 | 0 |
| Casos sin recepción declarada | 9 | 9 | 0 |
| Puntuación sobre 100 | −88,89 | −88,89 | 0,00 |

Las nueve transiciones SV son **1 → 1**. No hay correcciones ni regresiones sustantivas. Tres respuestas finales son idénticas a sus originales de A0; las restantes presentan cambios de formulación sin modificar la clasificación. La pendiente descriptiva entre A0 y A1 es cero puntos por revisión. Este resultado no demuestra mejora ni permite inferir una convergencia posterior. La revisión adversarial no aparece declarada en la salida final; su omisión no demuestra por sí sola ausencia de actividad crítica en el canal de análisis.

El [resultado estructurado](RESULTADOS-A1.json) conserva las nueve adjudicaciones, citas, huellas y transiciones. Las [respuestas originales](RESPUESTAS-A1.json) permanecen íntegras.

## Condiciones y límite de interpretación

Cada caso recibió la misma política y las mismas dos páginas que en A0, además de su respuesta final anterior íntegra. No recibió la clave, etiquetas correctoras, evaluaciones externas ni respuestas de otros casos. Se mantuvieron pesos, plantilla, esfuerzo de razonamiento, selección y límites de generación.

El ejemplo de JSON incluido en la política ilustra los campos de clasificación, pero no muestra `recepcion_documental` ni `revision`; el párrafo final los exige expresamente. Se conserva esta limitación de presentación, común a la condición fijada, al interpretar las omisiones. No se modifica retrospectivamente la política ni se atribuye causalidad exclusiva al modelo. El error sustantivo de A08 se analiza separadamente de estas omisiones.

Las entradas efectivas de A1 contienen entre 2.310 y 2.450 tokens; la reserva tras la salida máxima de 8.192 tokens es de al menos 22.126. Las nueve generaciones terminaron normalmente, sin agotamiento de la cota de salida. No se observa truncamiento documental ni de antecedentes. El cotejo comprueba lo efectivamente entregado y emitido; no acredita acceso exhaustivo al cálculo interno ni fidelidad causal de un razonamiento expresado.

## Custodia e instrumentación

La inferencia se cerró el 03/10/2026 a las **15:56:48 UTC** —17:56:48 en Madrid—. Modelo y MCP terminaron con código cero; las tres unidades quedaron inactivas y no se encontraron procesos propios restantes. Los cotejos Rust posteriores al cierre son conformes para las nueve entradas y la cadena completa de custodia.

Se conservan 26.910 registros exteriores, 119 registros MCP, 39 solicitudes y 38 respuestas —la notificación no exige respuesta—, 6.639 eventos de cálculo y tokens emitidos, 324 comprobaciones de cuatro expertos y 5.954 muestras de telemetría. El intercambio y los contadores de agotamiento permanecen en cero. El máximo conjunto registrado es histórico bajo una cota de 114 GiB; hubo presión contra cotas, también en custodia. No se acredita un margen estable ni se atribuye todo el máximo histórico exclusivamente a A1.

- [Cotejo de entradas y emisiones](FINAL-A1.json).
- [Cotejo integral de custodia](CUSTODIA-A1.json).
- [Constancia de cierre](CIERRE.json).
- [Originales íntegros en custodia](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/releases/tag/safeguard-retroalimentacion-20261003-a1): `ORIGINALES-A1-20261003.tar.gz`, 65.948.621 bytes, SHA-256 `37c60263b1f331a0ac4afcce424eed355d45d6bdccf429e0b8105a7ed562efc8`.

El archivo publicado se ha recuperado de GitHub de forma independiente. La [constancia de recuperación](COTEJO-RECUPERACION-GITHUB.json) acredita igualdad íntegra de bytes y SHA-256 mediante Rust. La preparación de la siguiente capa no constituye su ejecución.

## Continuación delimitada

Se mantiene la continuación autorizada A2 y A3, y A4 adicional si A3 no alcanza conformidad. A2 incorporará las respuestas completas de A0 y A1 del mismo caso. La evaluación conserva el dictamen de A1 y no seleccionará retrospectivamente la mejor capa. B permanece condicionado a la conformidad de A; el examen de 25 preguntas no se inicia aquí.

El Núcleo, la semántica V0.2, la IR 0.3 y los pesos permanecen intactos. No se acredita aptitud clínica ni aprendizaje persistente de pesos.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
