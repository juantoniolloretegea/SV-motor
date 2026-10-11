# Buscador-Semántico - Diferencial: revisión de alternativas

**Paso 4 de la secuencia de trabajo · 11 de octubre de 2026.**

## Conclusión

La revisión no permite seleccionar todavía una solución que cumpla el alcance requerido: descubrimiento de contradicciones de significado en documentos médicos y de ciberseguridad, realización en Rust, sin IA en el análisis y sin trasladar al revisor la preparación esencial del significado. Se examinó una alternativa adicional mediante ejecución, Logicaffeine 0.10.1; su salida omite condiciones relevantes en algunos enunciados que admite. El [resultado específico](RESULTADO-ADMISION.md) conserva los casos y las limitaciones.

No se concluye imposibilidad general. Tampoco se justifica comenzar un desarrollo amplio para rellenar las carencias encontradas. El recurso que falta no es un visor, un índice de texto ni un comprobador de fórmulas: falta demostrar el recorrido desde las afirmaciones originales hasta una comparación que conserve su significado.

## Método

Se realizaron búsquedas dirigidas sobre detección de contradicciones, análisis lógico del lenguaje, Rust y enfoques deterministas. Se contrastaron descripciones con documentación y código de sus repositorios. Las referencias precisas, objetos Git y datos de actividad están en [FUENTES-ALTERNATIVAS.json](FUENTES-ALTERNATIVAS.json). La búsqueda no es exhaustiva. Los anuncios de funcionalidad y los recuentos de popularidad no se consideran demostraciones de rendimiento.

## Recursos examinados

| Recurso y sede | Función comprobada documentalmente | Resultado para este alcance |
| --- | --- | --- |
| [Logicaffeine](https://github.com/Brahmastra-Labs/logicaffeine) | Analizador determinista de inglés y representación lógica; bibliotecas Rust con perfil sin Z3. | Justificó una comprobación local. No supera conservación del significado en la configuración ejecutada. |
| [Legalis-RS](https://github.com/cool-japan/legalis) | Representaciones jurídicas estructuradas, lenguaje formal y verificación. El ejemplo multilingüe examinado reconoce patrones de edad, ingresos o residencia; el módulo nlp revisado genera explicaciones de diferencias estructuradas. | No demuestra extracción general del significado de nuestros documentos. No se ejecutó. |
| [OxigenAI](https://github.com/cool-japan/oxigenai) | Su conversión de artículos no reconocidos utiliza Gemini para producir la representación formal; el código conserva una alternativa de texto Custom si no logra convertirlos. | Excluido como vía de análisis. La traducción instrumental admitida no autoriza esta interpretación mediante IA. |
| [Paralegal compiler](https://github.com/brownsys/paralegal-compiler) | Compila políticas expresadas en lenguaje controlado a comprobaciones de programas Rust. | Exige una formalización previa y atiende a políticas sobre programas; no resuelve el análisis documental solicitado. |
| [Rustling](https://github.com/sonos/rustling) | Infraestructura de análisis con reglas y clasificación de candidatos; el perfil general contiene un módulo ml. | No acredita contradicciones documentales complejas; la actividad de la rama examinada es antigua. No se integra ni se ejecuta. |
| [Elenchus](https://github.com/m62624/elenchus) | Comprueba consistencia de hechos y premisas escritos en un lenguaje específico. Su documentación distingue ese lenguaje de la prosa libre. | El motor lógico no resuelve por sí mismo la obtención fiel de los hechos. No se ejecutó. |
| [solo-steward 0.11.5](https://docs.rs/solo-steward/0.11.5/solo_steward/contradiction/index.html) | Filtra parejas de hechos y encomienda la decisión de contradicción a un LLM. | Excluido por el método empleado. |
| [spool-memory 0.2.4](https://docs.rs/spool-memory/0.2.4/spool/contradiction/index.html) | Describe heurísticas sobre resúmenes de memoria. | Evidencia insuficiente para el alcance; no se recuperó el repositorio indicado ni se ejecutó. |

[Tantivy](https://github.com/quickwit-oss/tantivy) mantiene su posible función de recuperación textual. No se vuelve a presentar como detector de contradicciones ni se encarga al revisor la preparación de vocabularios para sustituir ese análisis. Los conversores, lectores y la administración ya existentes conservan sus funciones.

## Actividad y condiciones de reutilización

Los metadatos consultados muestran 42 estrellas y 3 bifurcaciones para Logicaffeine; 30 y 2 para Legalis; 8 y 0 para OxigenAI; 2 y 2 para Paralegal compiler; 84 y 20 para Rustling; y 0 y 0 para Elenchus. Son observaciones fechadas y variables. No se obtuvo un recuento verificado de contribuidores. No se afirma que estas cifras representen comunidades amplias o mantenimiento suficiente.

Se distingue la última actualización del repositorio de la fecha del último cambio en la rama examinada. Por ejemplo, Rustling muestra actividad de repositorio en 2021, pero su revisión principal examinada es de 2019. Logicaffeine se examinó documentalmente en una revisión de julio de 2026; el paquete ejecutado procede de una revisión anterior del mismo mes, identificada por separado.

Logicaffeine utiliza BUSL-1.1. Su código accesible no debe describirse como disponible bajo una licencia de código abierto sin restricciones. No se ha adoptado una decisión de despliegue ni adquirido licencias. Legalis declara Apache-2.0 y Elenchus MIT; esas licencias no corrigen sus limitaciones funcionales.

## Contraste adversarial de la conclusión

**Posible objeción: analizar primero en inglés elimina el problema.** Reduce una dificultad lingüística, pero la comprobación inglesa de Logicaffeine ya pierde condiciones. Traducir no corrige ese defecto posterior.

**Posible objeción: añadir un demostrador lógico basta.** Puede comprobar la fórmula recibida, pero no recuperar una condición que el analizador omitió. Un resultado formal correcto sobre una representación incompleta no acredita la comparación del original.

**Posible objeción: ampliar el vocabulario permite continuar.** Podría corregir admisiones concretas; no demuestra conservación de restricciones, complementos, unidades y versiones. Adaptar únicamente los ejemplos y repetirlos no sería una comprobación independiente.

**Posible objeción: aceptar más frases supone mejor resultado.** No necesariamente. Los rechazos explícitos delimitan cobertura; una aceptación con pérdida de condiciones puede ocultar el defecto. Ambos resultados deben conservarse por separado.

## Situación de los pasos

1. Identificación y alcance: fijados.
2. Introducción en el libro: [aportación revisada](APORTACION-AL-LIBRO-V2.md), con terminología funcional; la incorporación se coordina sobre la edición existente.
3. Referencias y casos: conservados los cinco originales y la primera comprobación; añadidas catorce entradas diagnósticas para un objeto distinto.
4. Selección técnica: revisión completada al corte, sin candidato seleccionado para integración.
5. Comprobación semántica: realizada la comprobación previa de admisión de una alternativa; su resultado impide avanzar a una prueba de contradicciones basada en esa configuración.
6. Idiomas y formatos: no se declara cumplida esta etapa para el comparador.
7. Integración: no iniciada.
8. Documentación: resultados separados, fuentes precisas, límites y aportación editorial actualizados.

La continuación técnica necesita una alternativa o una corrección delimitada que demuestre conservación del significado sobre casos independientes. No se amplía el alcance ni se modifica el núcleo del SV para acomodar los candidatos.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
