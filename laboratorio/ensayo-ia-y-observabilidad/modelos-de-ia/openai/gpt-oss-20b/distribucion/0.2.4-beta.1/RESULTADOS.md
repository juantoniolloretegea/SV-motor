# Cierre experimental de GPT-OSS · 25 de septiembre de 2026

## Dictamen y alcance

Se cierra la campaña nativa conservada y se distribuye la aplicación 0.2.4 como entrega experimental 0.2.4-beta.1. El cierre permite fijar un punto de continuidad; no acredita aptitud clínica, integración productiva ni conformidad integral de la vía B. Las inferencias recibidas se ejecutaron antes de esta intervención. El cierre no añade inferencias ni ensayos prolongados.

La conversación histórica, la consulta documental del universo de inmunología y el banco sintético de tricoleucemia son tres conjuntos distintos. Sus resultados no se suman para producir una supuesta tasa general de acierto.

| Conjunto | Resultado conservado | Límite de interpretación |
| --- | --- | --- |
| Comparación conversacional histórica | Once preguntas previstas; diez turnos recibidos, incluidos un antecedente y un intento sin texto por límite temporal. Nueve respuestas parciales de 256 tokens; una pregunta no admitida. | Errores de identificación de enfermedad y referencias sin respaldo; comparación con Qwen sin igualdad de motor, contexto ni presupuesto. Los expedientes privados no se incorporan a esta distribución. |
| Consulta documental DOC01–DOC12 | Doce condiciones conformes con el contrato de extracción y ausencia de respaldo: cuatro antecedentes y ocho condiciones nuevas. | Cinco de los 27 parámetros de OP-IMM-001; consulta directa al motor. No constituye conversación libre ni ensayo de los 27 parámetros. Auditoría original conservada, sin nueva ejecución. |
| Tricoleucemia HCL-01–HCL-12 | Doce peticiones conservadas: diez terminaciones normales y dos por límite de generación. Cotejo Rust de doce conversaciones y 49 sucesos conforme. | Una respuesta por condición, casos sintéticos y documentación suministrada. Evaluación asistida inicial: siete no conformes y cinco pendientes de revisión; ninguna obtiene conformidad completa. |

Los antecedentes de tareas breves, memoria e identificación sintética mantienen el alcance de sus informes originales. Los resultados documentales favorables no corrigen retrospectivamente los fallos conversacionales. Las condiciones que no se ejecutaron siguen sin resultado.

## Banco sintético de tricoleucemia

Se recibieron doce conversaciones independientes, cada una con una sola consulta, entre las 05:47 y las 07:32 UTC del 25/09/2026. Configuración: CPU, canal final explícito, temperatura 0, reserva de 768 tokens de salida, límite individual de 900 s y presupuesto operativo de contexto de 4096 tokens. La semilla 299792458 se registra, pero el adaptador no la transmite al motor: no se acredita determinismo por semilla.

La rúbrica precede a las inferencias. Cada consulta tiene cuatro criterios de 0 a 2 puntos. Una invención relevante, recomendación peligrosa o confusión de desenlace impide conformidad aunque la suma sea alta. Las citas se comprueban separadamente. La valoración presente es una revisión asistida inicial, susceptible de revisión especializada; no es una validación clínica independiente ni una recomendación asistencial.

| Consulta | Criterios | Total / 8 | Dictamen inicial | Tokens de salida | Tiempo total, s |
| --- | --- | ---: | --- | ---: | ---: |
| HCL-01 | 2 / 2 / 1 / 1 | 6 | no conforme | 609 | 497.456 |
| HCL-02 | 2 / 1 / 2 / 2 | 7 | revisión | 698 | 575.876 |
| HCL-03 | 2 / 2 / 2 / 1 | 7 | no conforme | 553 | 445.170 |
| HCL-04 | 2 / 2 / 2 / 2 | 8 | revisión | 581 | 472.814 |
| HCL-05 | 2 / 1 / 1 / 2 | 6 | no conforme | 608 | 519.457 |
| HCL-06 | 2 / 2 / 1 / 1 | 6 | revisión | 633 | 523.820 |
| HCL-07 | 2 / 1 / 2 / 2 | 7 | revisión | 551 | 475.744 |
| HCL-08 | 2 / 1 / 1 / 2 | 6 | no conforme | 768 | 656.666 |
| HCL-09 | 2 / 1 / 1 / 1 | 5 | no conforme | 667 | 543.342 |
| HCL-10 | 1 / 2 / 2 / 1 | 6 | no conforme | 768 | 628.859 |
| HCL-11 | 2 / 0 / 2 / 1 | 5 | no conforme | 611 | 513.701 |
| HCL-12 | 2 / 1 / 2 / 1 | 6 | revisión | 472 | 425.684 |

HCL-08 y HCL-10 alcanzaron el límite de generación. Todas las respuestas superan las 230 palabras solicitadas según recuento por separadores de espacio; ese recuento instrumental no pretende resolver todas las convenciones lingüísticas de palabra. El tiempo registrado es el total del servicio, no la latencia del primer token.

### HCL-01

La hipótesis y la confirmación integrada son pertinentes. No explica adecuadamente la fibrosis del aspirado seco y atribuye translocaciones a supuestos subtipos de tricoleucemia sin respaldo en la fuente. Esta adición relevante impide conformidad.

### HCL-02

Rechaza la equivalencia morfológica con enfermedad BRAF positiva y no promete duración de respuesta en la serie de nueve casos. El diagnóstico diferencial queda incompleto; añade pruebas cuya indicación no justifica y la referencia no cumple íntegramente el formato solicitado.

### HCL-03

Distingue observación y evaluación terapéutica, pero denomina la enfermedad leucemia linfocítica crónica y atribuye al PDQ un título, páginas y apartados que no corresponden al documento consultado.

### HCL-04

Rechaza el inicio automático, prioriza infección y función renal y condiciona las alternativas. La cita del PDQ es identificable, pero la localización por apartados y la presentación como contraindicación formal exceden la atribución documental comprobada. Requiere revisión de citas y alcance.

### HCL-05

Reconoce el ensayo aleatorizado de fase II, los 68 participantes, la toxicidad y el DOI. Sin embargo, rotula ausencia de enfermedad residual como RFS y convierte una mediana de seguimiento de 96 meses en un horizonte uniforme y máximo. La confusión de desenlaces impide conformidad según la rúbrica.

### HCL-06

No equipara positividad residual aislada con fracaso clínico ni impone tratamiento inmediato. La distinción entre valor pronóstico y utilidad para decidir tratamiento es incompleta; faltan precisión metodológica y respaldo específico de algunas definiciones.

### HCL-07

Identifica correctamente el denominador de pacientes con respuesta completa y rechaza una ventaja demostrada de supervivencia global entre series. Confunde mediana con media de seguimiento y añade un umbral temporal innecesario para la comparación.

### HCL-08

Hace visible la discordancia de fase y niega superioridad comparativa. Denomina repetidamente la enfermedad leucemia mieloide crónica y especula sobre una reclasificación a fase III por tamaño o seguimiento sin prueba documental. La respuesta termina por límite de generación.

### HCL-09

Distingue recaída tardía y refractariedad, pero añade ausencia de alteraciones TP53/IKZF1 como condición para repetir tratamiento y la atribuye al PDQ sin respaldo. La referencia final vuelve a identificar otra leucemia.

### HCL-10

Rechaza la jerarquía basada únicamente en porcentajes y distingue respuestas y tamaños muestrales. No completa la refutación de curación universal; exige tamaños y diseño sin justificación y propone criterios de respuesta inadecuadamente atribuidos. Referencias y conclusión quedan truncadas.

### HCL-11

Distingue normalización del hemograma y erradicación medular. Generaliza en exceso el resultado negativo de G-CSF y vuelve a denominar la enfermedad leucemia mieloide crónica. La caracterización retrospectiva del estudio ya estaba en el material proporcionado y no se atribuye exclusivamente al modelo.

### HCL-12

Rechaza una prohibición universal y no extrapola autorización estadounidense a España. Reconoce nivel C3 y actualización de 2024, pero omite el contexto de 2021 y desarrolla de manera insuficiente las comprobaciones de guías clínicas vigentes.

## Crítica de las fuentes proporcionadas

El material previo sintetiza editorialmente el PDQ español del NCI, actualizado el 14/11/2024 y consultado el 25/09/2026. La fuente no es infalible. La discrepancia entre la fase III indicada en un pasaje del PDQ y la fase II del estudio original de dabrafenib con trametinib se había incluido expresamente en HCL-08. El artículo original permite resolver el diseño; no justifica inventar una reclasificación. La descripción retrospectiva del estudio de filgrastim en el material previo tampoco debe atribuirse únicamente al modelo: el original describe un estudio de fase II con comparación histórica.

Referencias de contraste, consultadas el 25/09/2026:

- [NCI, PDQ de tratamiento de la leucemia de células pilosas, versión profesional española](https://www.cancer.gov/espanol/tipos/leucemia/pro/tratamiento-celulas-pilosas-pdq).
- [Ensayo aleatorizado de fase II de cladribina con rituximab simultáneo o diferido](https://doi.org/10.1200/JCO.19.02250), texto completo [PMC7213585](https://pmc.ncbi.nlm.nih.gov/articles/PMC7213585/).
- [Dabrafenib y trametinib en enfermedad recurrente o refractaria: estudio de fase II](https://doi.org/10.1182/blood.2021013658), texto completo [PMC10163281](https://pmc.ncbi.nlm.nih.gov/articles/PMC10163281/).
- [Estudio de filgrastim para fiebre neutropénica inducida por cladribina](https://pubmed.ncbi.nlm.nih.gov/10194424/).

## Integridad, conservación y límites

El auditor independiente del ejecutor, compilado con Rust 1.98.0, coteja entradas, peticiones, perfiles, contextos, huellas, resultados del motor, exportación final y cadena local de sucesos. Obtiene doce conversaciones, doce turnos y 49 sucesos conformes. La última huella de la cadena es a5b6d048c9fbf6906a704085b0cc397b062c85d97e393b9a68c48a064388879f. La conformidad de los archivos no acredita verdad clínica ni custodia externa independiente; tampoco excluye la supresión de un sufijo completo de una cadena no anclada externamente.

La comprobación de restauración utiliza una copia del registro y el ejecutable distribuido. El nombre del archivo debe conservar el identificador exp-1790315249328-1.jsonl. Un primer intento con el nombre descriptivo EXPEDIENTE-ORIGINAL.jsonl fue rechazado por la comprobación de identidad, sin alterar el original. La segunda comprobación obtuvo un expediente, doce conversaciones, 49 sucesos y cero peticiones pendientes recuperadas. La comparación binaria de la copia con el original fue conforme. No se ha ensayado una instalación completa en un anfitrión limpio.

Los fallos iniciales de compilación del auditor se corrigieron antes del cotejo satisfactorio; no se presentan como pruebas superadas. Las trazas instrumentales y los originales permanecen en OneCloud. Esta distribución publica únicamente la evidencia sintética seleccionada y las síntesis de los antecedentes; no incorpora conversaciones personales retenidas anteriormente.

## Aportación al estudio del Núcleo

La evidencia exige separar identidad del modelo y del ejecutable, contrato de la tarea, contexto efectivo, procedencia de la fuente, terminación técnica y aceptación semántica. La autoridad para decidir no se transfiere al modelo. La cadena local y el muestreo de recursos no sustituyen la custodia independiente ni una guarda exterior completa.

Esta recepción aporta requisitos experimentales a la continuidad de la adenda. No cambia gramática, IR, contratos ni semántica del Núcleo. S39 continúa abierto para el alcance de integración pendiente; el cierre de esta campaña y su entrega no cierra todo el programa de investigación.
