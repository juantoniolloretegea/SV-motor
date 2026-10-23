# Evaluaciones, células y rutas críticas: organización de la presentación

Fecha: 23 de octubre de 2026. Estado: criterio de organización y necesidades de ampliación. La consulta actual representa las células de los exámenes publicados. La vista de conjuntos y recorridos aquí descrita requiere contratos y comprobaciones posteriores; no se presenta como una función de producción ya disponible.

## 1. La evaluación determina el alcance

Un candidato puede participar en distintas evaluaciones. Cada participación conserva convocatoria, prueba y edición, modelo y versión, nodo, categoría, intento, respuesta, corrección y evidencias. El número de candidatos no determina el número de polígonos de una participación.

Una evaluación puede abarcar una célula, varias células de un subdominio o las que requiera un dominio completo. Su alcance se declara antes de ejecutarla. El número de células se obtiene de la constitución aplicable, nunca del número de preguntas, de Universos candidatos o de espacios disponibles en pantalla. No se inventan células ni se completan posiciones ausentes con U.

| Objeto | Identidad y función |
| --- | --- |
| Participación del candidato | Identifica una evaluación y su intento; no se reduce al nombre del modelo. |
| Alcance evaluado | Identifica dominio, subdominio u operación, versión y cobertura exigida. Un alcance parcial no se presenta como dominio completo. |
| Célula aplicada | Conserva identidad propia, constitución, dimensión, orden de parámetros, estado, contrato, resultado y evidencia. |
| Relación y punto de contacto | Identifica los extremos, sus posiciones o usos cuando proceda, dirección, tipo, condiciones y fuente de su definición. |
| Ruta exigida | Conserva identidad, versión, tarea, dependencias, ramas, condiciones, obligatoriedad y consecuencias declaradas. |
| Recorrido observado | Identifica la ejecución y sus acontecimientos, su orden y evidencia. Se distingue de la ruta prescrita y de lo que sólo declara el modelo. |

Un Universo no equivale automáticamente a una célula. Un parámetro puede intervenir en varios usos sin duplicar su definición. El par (36,6) y (9,3) conserva dos células: no crea una célula de 45 posiciones. Una relación entre Universos tampoco prueba por sí sola una ruta crítica.

## 2. Tres niveles coordinados de consulta

**Conjunto.** La entrada identifica candidato, evaluación, alcance y versión. Un índice consultable relaciona las células recibidas y las que el contrato exige, con sus estados y evidencias. Permite buscar y seleccionar sin desplegar todos los polígonos a la vez. Las cantidades visibles, filtradas y pendientes se indican expresamente; ocultar por comodidad visual no cambia la cobertura.

**Rutas críticas.** La vista muestra las relaciones constituidas y permite seleccionar un recorrido. Sus puntos de contacto abren la célula y posición correspondientes. Distingue lo exigido, lo observado y lo declarado, así como las ramas que todavía no cuentan con evidencia. Una dependencia crítica omitida no desaparece al filtrar el dibujo. Los ciclos, bifurcaciones y condiciones de terminación se conservan según el contrato; la interfaz no los convierte arbitrariamente en una lista lineal.

**Célula.** El detalle conserva el vector ordenado y su polígono, parámetros y evidencias. Mantiene el contexto de la evaluación y el retorno al conjunto o ruta seleccionados. La comparación entre candidatos usa el mismo alcance, versión y escala; comparar células de dimensiones diferentes no convierte sus áreas o perímetros en notas comparables.

Las vistas de conjunto y rutas no requieren representar simultáneamente 32, 59 o más polígonos completos. Se abre el detalle solicitado y, cuando proceda, una comparación expresamente seleccionada. Esas cantidades sirven para preparar comprobaciones de carga; no se fijan como límites del SV. Una representación reducida del conjunto se identifica como tal y no se confunde con la escala del detalle celular.

## 3. Criterio común del detalle celular

La misma anchura disponible produce la misma escala: recinto cuadrado de hasta 460 unidades lógicas, margen de 48 y radios proporcionales 1:2:3. La primera posición está arriba; el orden es horario. Los números exteriores indican el orden del vector, y el detalle conserva el identificador original. Cuando su separación resulta insuficiente, los rótulos se disponen a dos distancias alternas, sin reducir la letra ni cambiar la escala del polígono. Los botones están encima del polígono, con acceso por teclado. La ventana baja admite desplazamiento vertical; no reduce el polígono para acomodar los textos.

La revisión comprende (9,3), (16,4), (25,5), (36,6) y (49,7). La última es la cota actualmente autorizada, no un máximo matemático de la familia n=b². Las comprobaciones sintéticas de presentación no constituyen nuevos exámenes ni acreditan un dominio profesional.

## 4. Separación de responsabilidades

El componente gráfico representa una célula identificada. La selección de evaluación, conjunto o ruta pertenece a la organización de la consulta. El algoritmo competente aplica las reglas constituidas y produce resultados y evidencias; la presentación no decide rutas, no recalcula un resultado global por mayoría y no adjudica criticidad.

La realización del SV y sus controles siguen en Rust. HTML y JavaScript sirven la presentación y sus interacciones autorizadas. Mostrar una relación no concede acceso al Universo de destino ni habilita una operación. Cada consulta respeta los permisos sobre los objetos y evidencias; una respuesta pública no incluye datos privados escondidos visualmente.

La ampliación deberá conservar las identidades y enlaces existentes mediante una correspondencia explícita entre cada resultado histórico y su célula. Las futuras colecciones tendrán una versión propia y referencias verificables; no se reinterpretará silenciosamente el documento actual de una célula como si ya incluyera un conjunto o una ruta. La regla contractual de una prueba tampoco se generaliza al dominio completo.

La definición y la representación quedan subordinadas a las invariantes constitutivas del SV. El [libro de Dominios y Universos](https://documentos-sv.itvia.online/dominios/universos/parametros-rutas.html#invariantes-y-admisibilidad-de-la-ampliación) precisa esa condición. El [contraste de fuentes de consecuencias](CONSECUENCIAS-Y-FUENTES.md) distingue las clasificaciones sectoriales y jurídicas de los niveles previstos; no establece una equivalencia automática ni incorpora probabilidad, tiempo soberano o inferencia opaca al método del SV.

## 5. Inventarios cotejados

- **Inmunología:** el mapa INMUNO v0.8 identifica **32 Universos operacionales candidatos**. Expresamente distingue esa cantidad de parámetros, técnicas, enfermedades y futuros usos. No constituye 32 células ni autoriza multiplicar 32 por los parámetros de OP-IMM-001. [Mapa versionado](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/9939d600b3c3ffc3f4237f229ca0670de8f3693d/dominios/inmunologia/cambio-rumbo/03-base-documental-candidata/16-mapa-operaciones/Mapa_finito_universos_operacionales_catalogo_INMUNO_v0.8_v0.1_2026-09-03.md).
- **Ciberseguridad:** el contraste v0.3 explicita 59 productos candidatos, agrupados en escenarios de **29, 38 y 59 Universos**. Son hipótesis de planificación; 59 no es un máximo probado ni 29 un mínimo matemático. No determinan cuántas células necesitará su constitución. [Contraste versionado](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/bbac1b44b1d3b845305e9cde492a08221206d631/dominios/ciberseguridad-inteligente/dominio-04-09-26/universos/OP-CYB-001/ESTIMACION_Y_CONTRASTE_CON_LENGUAJE_v0.3.md).
- **Rutas:** el acta recibida distingue el subgrafo obligatorio, su ejecución observada y la suficiencia de su cobertura. Una secuencia de consultas no sustituye las dependencias constituidas. [Acta, apartados 2 y 4](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/217f981b9724b406a6c94e30e0c77042cd019d85/dominios/ACTA_EVALUACION_Y_RECEPCION_DOCUMENTAL_RUTAS_CONOCIMIENTO_SV_2026_09_14.md).

## 6. Revisión adversarial y recepción futura

| Caso | Resultado exigido |
| --- | --- |
| Un candidato, dos intentos o alcances distintos | Identidades separadas; no se mezclan células, rutas ni evidencias. |
| Una célula; conjunto de 32, de 59 y de mayor tamaño | Consulta por identidad, sin exigir dibujar todas las células ni fijar esas cantidades como máximos. |
| Dimensiones heterogéneas o par de células | Cada vector conserva su dimensión y orden; no se concatena en una célula ficticia. |
| Universo aún sin constitución celular | Pendencia explícita; no se dibuja una célula inventada ni se rellena con U. |
| Dos células con la misma etiqueta de parámetro | El destino se identifica por evaluación, célula y posición; no por el texto de la etiqueta. |
| Contacto inexistente, versión incompatible o referencia ambigua | Rechazo o incidencia explícita; no se resuelve por semejanza de nombres. |
| Ruta prescrita con rama obligatoria omitida | La cobertura queda incompleta aunque todas las células visibles resulten aptas. |
| Cambio de filtros o selección | No altera alcance, resultado, cobertura ni obligaciones; declara qué está mostrando. |
| Declaración del modelo sin evidencia de ejecución | Se conserva como declaración, separada de lo observado por el SV. |
| Cambio de permisos, identidad o versión | Nueva comprobación antes de servir el destino; ningún dato oculto concede acceso. |
| Ventana estrecha, zoom o teclado | Lectura y selección verificables; ninguna dependencia crítica depende sólo del color. |
| Regla global no constituida | No se fabrica un dictamen por suma, promedio, mayoría o forma del polígono. |

Esta tabla fija criterios de aceptación para la ampliación. No afirma que se haya ejecutado un conjunto profesional de 32 o 59 células. La incorporación actual unifica el detalle de los exámenes conservados y mantiene su trazabilidad; la definición y recepción del algoritmo de rutas conservan su sede competente.

## 7. Estudio específico de los métodos de rutas

La criticidad expresa las consecuencias de omitir conocimiento necesario para la operación, aunque ese conocimiento pertenezca a otro Universo. Los niveles previstos son **bajo, medio, alto, grave y muy grave**. Son categorías de consecuencias que deberán definirse y validarse por dominio; no son valores nuevos de la terna ni sustituyen los criterios de corrección de los exámenes ya fijados.

La clase de documento o de actuación omitida no determina por sí sola la gravedad. Deben considerarse el contexto, la dependencia, el momento en que el conocimiento es necesario y las consecuencias justificadas de su omisión. La asignación requiere criterios establecidos y revisados por el experto competente. La ausencia de información tampoco justifica asignar por defecto el nivel menor.

La definición de rutas críticas y la comparación de métodos constituyen una fase propia, pendiente. Antes de implementar un método se precisarán la operación, sus obligaciones y el significado de un resultado correcto. Las propuestas humanas y las alternativas se someterán a revisión formal, documental y experimental según su alcance, sin elegir una por su mera facilidad de representación.

La efectividad se evaluará mediante corrección del resultado, cobertura de dependencias obligatorias, tratamiento de bifurcaciones y ciclos, condiciones de terminación, incertidumbre y fallo, reproducibilidad y trazabilidad. Tiempo, memoria y otros recursos se medirán una vez demostrada la validez requerida; una ventaja de velocidad no compensa una omisión crítica. Dos métodos sólo se compararán bajo las mismas condiciones y obligaciones, conservando sus diferencias pertinentes.

La revisión adversarial deberá incluir rutas incompletas que produzcan una conclusión aparentemente correcta, saltos entre Universos no autorizados, omisiones de ramas conjuntas, ciclos sin término, evidencias de versiones incompatibles, fallos de observación y trazas fabricadas o incompletas. Los criterios de recepción se establecerán antes de la comparación y con autoridad competente. La interfaz mostrará las evidencias de ese análisis, sin sustituirlo.

Esta reserva exige preparar la organización de la consulta, pero no abre ahora una implementación, una campaña de pruebas de rutas ni una decisión entre algoritmos.

## 8. Panel y visibilidad

**Rutas críticas** es el panel descriptivo del Evaluador en EET-IA, con acceso destacado desde Inicio para los perfiles autorizados de Evaluador y Administrador. Su estado es **En preparación**. Delimita alcance, conjuntos de células y rutas; no permite todavía definirlas, autorizarlas ni ejecutarlas. El uso experto en producción conserva su espacio y sus permisos propios.

Las rutas concretas, sus contactos y evidencias se consultarán con sesión y permisos sobre el alcance correspondiente. No se publican por estar ocultas detrás de un control visual. El libro de Dominios y Universos describe los conceptos y sus límites, sin exponer recorridos privados. Compartir una representación entre evaluación y producción no comparte datos ni autoridad.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
