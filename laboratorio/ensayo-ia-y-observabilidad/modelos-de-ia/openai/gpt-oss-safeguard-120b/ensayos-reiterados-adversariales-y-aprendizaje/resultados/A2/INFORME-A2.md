# Segunda revisión adversarial A2

**Preevaluación de examen de 25 preguntas de tricoleucemia.** Candidato: `openai/gpt-oss-safeguard-120b`. Fecha: 3 de octubre de 2026. Procedimiento: Aprendizaje por Retroalimentación del Sistema Vectorial SV, organizado por el Árbitro-Director del Sistema Vectorial SV.

## Resultado y evolución

**−88,89/100; No apto para A2.** Nueve respuestas originales conservadas: cero respuestas plenamente conformes, ocho errores exclusivamente formales no críticos y un error sustantivo crítico en A08. Ocho de las nueve clasificaciones documentales son correctas. No se acredita acceso al examen.

El vector es **v⁽²⁾ = (1,1,1,1,1,1,1,1,1)**; N₀ = 0, N₁ = 9, Nᵤ = 0; T(9) = 7; κ = No apto. No hay blancos, respuestas sin ejecutar ni impedimentos instrumentales. La puntuación auxiliar es 100 × (0 − 8) / 9. A08 se registra como crítico y no recibe una segunda deducción formal.

| Medida | A0 | A1 | A2 |
| --- | ---: | ---: | ---: |
| Clasificaciones documentales correctas | 8/9 | 8/9 | 8/9 |
| Respuestas plenamente conformes | 0/9 | 0/9 | 0/9 |
| Errores sustantivos críticos | 1 | 1 | 1 |
| Casos sin recepción declarada | 9 | 9 | 9 |
| Casos sin revisión declarada exigible | No procede | 9 | 9 |
| Puntuación sobre 100 | −88,89 | −88,89 | −88,89 |
| Esfuerzo de razonamiento | Medio | Medio | Medio |

La diferencia A1 → A2 es **0,00 puntos**, con cero correcciones y cero regresiones, tanto sustantivas como de cumplimiento completo. Las nueve transiciones SV son 1 → 1. Cuatro finales de A2 son idénticas, byte por byte, a sus antecedentes de A1. La pendiente descriptiva de A0–A2 es cero puntos por revisión; no se observa mejora bajo estas condiciones. Este resultado no demuestra incapacidad general de aprendizaje.

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

Todas las finales omiten `recepcion_documental` y `revision`, exigidas por la política. La recepción efectiva de ambas páginas está comprobada instrumentalmente; no sustituye la declaración solicitada ni acredita comprensión. El campo `pagina` de las citas es opcional y su ausencia no se penaliza. Las citas presentes se han cotejado con las fuentes.

A08 conserva el conflicto entre dos registros vigentes sin precedencia. La política exige EVIDENCIA_INSUFICIENTE y el candidato emite CONTRADICHA. Su explicación reconoce que no puede concluirse la autorización. Se preservan las [respuestas originales](RESPUESTAS-A2.json) y las [adjudicaciones estructuradas](RESULTADOS-A2.json), sin corregirlas retrospectivamente.

## Condiciones y reservas de interpretación

Cada caso recibió las mismas dos páginas y la misma política que en A0–A1, más sus dos finales anteriores completas. Los dieciocho antecedentes conservan orden e identidad. No se aportaron clave, resultados externos, etiquetas correctoras ni respuestas de otros casos. Se mantuvieron esfuerzo `medium`, selección determinista, semilla 42 y pesos intactos. El procedimiento observa autocorrección contextual con respuestas propias; no ejecuta entrenamiento de parámetros ni proporciona una señal externa de error al candidato.

La revisión del diseño identifica una limitación propia: la instrucción inicial y los ejemplos JSON ilustran cuatro campos, mientras que el párrafo final exige además recepción y revisión. No se ha demostrado que esta incoherencia de presentación cause las omisiones. La política tampoco ejemplifica el conflicto sin precedencia. A08 distingue entre autorización y posibilidad de concluirla, lo que limita atribuir el fallo a una incapacidad semántica general. La regla expresa sigue determinando el resultado bajo el criterio fijado.

En el análisis emitido de A08, el candidato reconoce el conflicto, reformula incorrectamente la regla hacia CONTRADICHA y menciona las clasificaciones de sus antecedentes. Es una observación del texto conservado; no demuestra la causa interna de la generación. Las comprobaciones de identidad y custodia tampoco equivalen a una comparación numérica independiente entre motores; esa comparación no se ha realizado aquí.

Las entradas efectivas contienen entre **2.525 y 2.774 tokens**, con al menos **21.802** de reserva tras la salida máxima de 8.192. Las emisiones completas contienen entre 565 y 919 tokens y terminaron normalmente. No se observa truncamiento de política, páginas ni antecedentes. El enmarcado Harmony y la transformación de roles corresponden a la plantilla del candidato; no se ha identificado una alteración de ésta.

La [guía oficial de Safeguard](https://developers.openai.com/cookbook/articles/gpt-oss-safeguard-guide) admite distintos esfuerzos de razonamiento y recomienda instrucciones y formatos coherentes. La revisión A3 introduce expresamente esfuerzo alto, conservando los demás factores fijados. Por ello, su comparación con A2 combinará una revisión adicional con mayor esfuerzo: no permitirá aislar el efecto causal de cada factor. A0–A2 y sus dictámenes permanecerán intactos.

## Custodia e instrumentación

A2 cerró el **03/10/2026 a las 20:04:22 UTC** —22:04:22 en Madrid—. Modelo y MCP terminaron con código cero y error nulo. Las tres unidades quedaron inactivas; no permanecieron procesos propios ni un servicio de inferencia expuesto. Los cotejos Rust de entradas y de cadena completa de custodia son conformes para las nueve respuestas, sin emisiones parciales.

Se conservan 26.984 registros exteriores, 119 registros MCP, 39 solicitudes y 38 respuestas, 6.537 cálculos finalizados, 324 comprobaciones de cuatro expertos y 6.334 muestras de telemetría. La notificación MCP no requiere respuesta. Intercambio y agotamiento permanecen en cero. Hubo presión contra las cotas; el contador conjunto `memory.events.max` terminó en 15.205. El máximo conjunto histórico fue 122.406.567.936 bytes bajo una cota de 114 GiB. No se atribuye íntegramente a A2 ni se afirma holgura estable.

- [Cotejo de entradas y emisiones](FINAL-A2.json).
- [Cotejo integral de custodia](CUSTODIA-A2.json).
- [Constancia de cierre](CIERRE.json).
- [Originales completos en custodia](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/releases/tag/safeguard-retroalimentacion-20261003-a2): `ORIGINALES-A2-20261003.tar.gz`, 70.085.090 bytes; SHA-256 `e5cfb5600a7253c842d9c685881ae0d103c10414b748fd24c9dbbb6e8d35fd5f`.
- [Cotejo de recuperación independiente](COTEJO-RECUPERACION-GITHUB.json): igualdad íntegra de bytes y SHA-256 comprobada en Rust.

## Continuación

A3 incorporará los 27 antecedentes propios completos de A0–A2 y esfuerzo alto, con identidad y admisión propias. A4 adicional permanece condicionada a la falta de conformidad de A3, sin quinta revisión automática. B continúa condicionado a A conforme; el examen de 25 preguntas no forma parte de esta ejecución. No se seleccionará retrospectivamente la mejor capa.

El Núcleo, la semántica V0.2, la IR 0.3 y los pesos permanecen intactos. No se acredita aptitud clínica.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
