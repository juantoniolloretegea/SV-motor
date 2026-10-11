# Buscador-Semántico - Diferencial: resultado de la comprobación preliminar

**11 de octubre de 2026 · Candidato: speccheck-core 0.4.1 · Estado: no apto para su incorporación como detector recibido.**

La comprobación ejecutada en Rust no acredita que este candidato satisfaga la función prevista del **Buscador-Semántico - Diferencial**. Detectó ciertos requisitos normativos opuestos, pero omitió incompatibilidades expresadas en prosa corriente y confundió diferencias de condiciones, vigencias, actores y unidades con conflictos. No se incorpora al recorrido documental del SV.

Esta conclusión corresponde al candidato y a la configuración ensayada. No demuestra la inviabilidad general de una realización determinista, no constituye el examen de un modelo de IA y no modifica las funciones del **Árbitro - Director**, el núcleo ni su semántica.

## Objeto, referencias y método

El objeto es comprobar si un recurso existente de la comunidad puede aportar la detección automática requerida antes de justificar un desarrollo propio. Se conserva el [protocolo anterior a la ejecución](PROTOCOLO.md), con [referencias independientes de la entrada](REFERENCIAS.json) y [fijación íntegra en Rust](FIJACION-RUST.json).

Se identificaron y cotejaron en Rust los cinco originales indicados para el estudio:

| Original | Tamaño y alcance | SHA-256 |
| --- | --- | --- |
| Caché NCI-PDQ profesional sobre leucemia de células pilosas | Caché histórica; no sustituida por la página actual | `00018ac31108eecc4709f0ce80439d4ca2d56b02262c5c334a184b9c0944e04d` |
| LLS, *Leucemia de células peludas*, FS16S8/18 | Diez páginas; edición histórica | `21322ed388c999bd0713ba5ec5c7ab34402d4bb4cd6100903aa5ea6d8bf35d9c` |
| Banco de evaluación CYB25 | Veinticinco casos de ciberseguridad; enunciados conservados | `51ce2242387cdd7954fe4f178bf6ab8aa2567f8b6e0a8e7304dadd02151b33f5` |
| INCIBE, *Medidas de ciberseguridad y ciberresiliencia ante modelos de IA de frontera* | Nueve páginas; versión facilitada | `614e39907fc9edac6d42b4dfc6299f8f4332fe91b979f0f7d4ec3440a282eb44` |
| Directiva NIS2, texto original de 2022 en español | Setenta y tres páginas; no se declara Derecho consolidado vigente | `798b7edb6046ef51f48d42d80107858290cae6e968518d0b226594d19026974a` |

Los tamaños exactos constan en la fijación. Se preservan sus ubicaciones y derechos; este informe no reproduce los originales ni acredita una custodia remota nueva de éstos.

El ensayo algorítmico utiliza **veinticuatro casos propios en inglés**, doce por dominio, derivados de problemas conceptuales presentes en esas referencias. No se presenta como examen integral de los cinco documentos. Los casos son sintéticos: no se atribuyen sus afirmaciones a NCI, LLS, INCIBE o la Directiva. Tampoco se presentan como traducciones Google, recomendaciones clínicas o interpretación jurídica.

El detector recibió únicamente los [archivos de entrada y sus huellas](ENTRADAS.json). No recibió las referencias, una lista de conceptos seleccionados ni las soluciones. El lector Markdown propio del candidato decidió qué afirmaciones reconocer; no se forzó cada oración a adoptar la forma de un requisito. El código de comprobación no aplica correcciones ni autoriza documentos. La fijación y el contraste posterior se realizaron mediante otro programa Rust.

Las referencias fueron preparadas para este diagnóstico después de examinar el código del candidato. No constituyen una evaluación ciega, una muestra aleatoria ni recepción científica humana independiente. Los resultados son válidos para localizar defectos concretos; sus proporciones no estiman rendimiento general.

## Resultados

| Magnitud | Medicina | Ciberseguridad | Total |
| --- | ---: | ---: | ---: |
| Incompatibilidades de referencia | 6 | 6 | 12 |
| Incompatibilidades con alerta | 3 | 3 | 6 |
| Incompatibilidades omitidas | 3 | 3 | 6 |
| Casos compatibles | 4 | 4 | 8 |
| Casos compatibles con alerta | 3 | 2 | 5 |
| Contexto insuficiente | 2 | 2 | 4 |
| Contexto insuficiente con alerta | 2 | 1 | 3 |

La sensibilidad por presencia de alerta fue **6/12 (50 %)**. Se emitieron alertas en **5/8 (62,5 %)** de los casos compatibles. Los cuatro casos sin contexto suficiente se informan aparte: no se convierten en incompatibilidades, falsos negativos ni resultados U de un modelo.

Cada ejecución produjo veinticinco mensajes distribuidos en catorce casos. Varias reglas pueden señalar el mismo caso; no se contabilizan como contradicciones adicionales. La coincidencia entre una alerta y la referencia tampoco prueba comprensión del fundamento.

Las dos ejecuciones dieron **resultados sustantivos idénticos**, con entradas y referencias intactas. Duraciones instrumentales totales: 3,555841 s y 3,450061 s. Son tiempos de la comprobación local, no latencias de modelos ni una medición general de rendimiento. No se midió el consumo eléctrico ni un coste económico atribuible. No hubo inferencias de IA.

Evidencias: [primera ejecución](EJECUCION-01.json), [repetición](EJECUCION-02.json), [cotejo Rust](COTEJO-RUST.json).

## Defectos observados y contraste del fundamento

| Casos | Observación | Consecuencia |
| --- | --- | --- |
| M02, M05, M06, C03 | Las incompatibilidades en prosa ordinaria no producen requisitos reconocidos ni alertas | El silencio del detector procede de falta de cobertura, no de coherencia documental |
| M03 | Confunde condiciones positiva y negativa de una prueba | No mantiene la condición de aplicación de la afirmación |
| M04, C04 | Declara incompatibles 24 horas y 1 día, y 60 segundos y 1 minuto | Compara valores sin establecer equivalencia dimensional; la regla SAT reproduce el defecto |
| M08 | Señala conflicto entre protocolos de vigencias distintas y sustitución declarada | No separa adecuadamente versiones históricas |
| C06 | Confunde facultades de actores diferentes | La coincidencia de palabras no establece identidad del sujeto |
| C10 | Omite una contradicción entre dos epígrafes del mismo documento | La comparación local por sección pierde relaciones documentales |
| C11 | Omite intervalos numéricos incompatibles entre archivos | La regla entre archivos no aporta la cobertura numérica requerida |
| M07, M11, C07 | Emite mensajes de conflicto sin resolver ámbitos, unidades o vigencias ausentes | No acredita una distinción fundamentada entre contradicción y contexto insuficiente |

Las alertas originales conservan su redacción. SC-CON-004 habla de tensión potencial y SC-XF-002 de posible contradicción; no se atribuye a todos los mensajes idéntica fuerza. Sin embargo, los cinco casos compatibles también activaron reglas que afirmaban conflicto directo o numérico. El problema no se reduce al tono prudente de una alerta.

Los mensajes conservan la localización del segundo pasaje y citan ambos textos, en ocasiones abreviados. No ofrecen siempre dos localizaciones estructuradas completas. Los archivos originales del ensayo permiten reconstruir la pareja; esta posibilidad no equivale a una interfaz de revisión ya recibida.

## Configuración e incidencias instrumentales

Se utilizó Rust 1.98.1 para compilar la comprobación. Se invocaron SC-CON-001, SC-CON-002, SC-CON-003, SC-CON-004, SC-CON-006 y **SC-XF-002**. **Errata material del protocolo fijado:** donde figura «SC-XFL-002» debe leerse «SC-XF-002». La denominación correcta procede del identificador emitido por la biblioteca y consta en ambas ejecuciones; la errata no cambia selección, criterios ni resultados. Se preserva el protocolo original para no alterar retrospectivamente su huella.

El paquete speccheck-core 0.4.1 descargado de su registro oficial tiene SHA-256 `e0fcbea6024b695434a8f257c3ad5ae43455108f9faf7af44a165efc269ecd90`. Su metadato de procedencia identifica la revisión `e49eda84dcbb8f836d79b866cf6ae9f89704b375` del proyecto. Se conserva su licencia MIT. No se modificaron sus fuentes.

La resolución inicial de dependencias incluía pest 2.9.3, stacker y psm. Se interrumpió esa preparación y se conservaron sus referencias. La configuración ejecutada fija pest y sus tres bibliotecas asociadas en 2.8.6, tal como aparecen en el Cargo.lock publicado con el candidato. El árbol activo resultante excluye aquella vía de compilación nativa. Esta fijación no sustituye una auditoría integral de las dependencias.

speccheck-nli figura como dependencia de tipos de interfaz, con **cero opciones activadas**. No se activa candle, languagetool ni llm; no se construye un detector NLI ni se llama a SC-CON-005. Las seis reglas invocadas son deterministas. La inspección del código y del árbol acredita ese recorrido; no se declara una prueba nueva de aislamiento de red del sistema operativo.

El programa de comprobación ejecutado tiene SHA-256 `b7bd3cba8c360a655195d7c6e90c4603c05958e1027d2d2c928ba39671800bb6`. El código y la resolución se conservan en [codigo](codigo/). La carpeta de compilación estuvo separada de los servicios operativos.

## Dictamen delimitado y continuación

El candidato **no supera el criterio previo** y no se incorpora como detector acreditado del Buscador-Semántico - Diferencial. Sus reglas normativas pueden servir como referencia técnica limitada, pero corregir únicamente el lector Markdown no resolvería los defectos observados de sujeto, condición, vigencia, unidad y comparación entre secciones o documentos.

No corresponde ampliar ahora esta configuración a documentos extensos, traducciones o integración operativa como si la capacidad principal estuviera recibida. La traducción al inglés facilita la entrada, pero los fallos se han observado ya en inglés y no pueden atribuirse al español o al traductor.

La continuación deberá justificar qué recursos comunitarios permiten cubrir esas operaciones semánticas y qué adaptación propia sería necesaria, contrastándola con casos independientes. No se presupone que unas expresiones regulares adicionales basten. La preparación documental y el libro pueden continuar con las capacidades acreditadas, sin presentar un detector funcional inexistente.

## Relación con el libro publicado

Se ha leído la edición V.01 del libro **«Reglas, usos y descripciones documentales de documentos para el conocimiento de la IA»**, incluido su índice y sus cinco capítulos, en la revisión [db7ed5e83979714b6c29ec0c7cbd18ac6f284b83](https://github.com/IA-en/biblioteca-documental/tree/db7ed5e83979714b6c29ec0c7cbd18ac6f284b83/libros/visualizadores/src).

Se conserva su estructura y su separación entre original, representación, suministro y autoridad humana. Las evidencias de este ensayo permanecen en la sede propia del Buscador-Semántico - Diferencial. La [aportación editorial propuesta](APORTACION-AL-LIBRO.md) se ajusta al apartado de información disponible para la revisión; no constituye otro libro ni una modificación de la edición publicada.

## Fuentes técnicas

- [speccheck-core 0.4.1: documentación y procedencia](https://docs.rs/crate/speccheck-core/0.4.1).
- [Código publicado de las reglas](https://docs.rs/speccheck-core/0.4.1/src/speccheck_core/rules/contradiction.rs.html).
- [Proyecto principal y revisión de origen](https://gitlab.com/neilfitzgerald1972/speccheck/-/tree/e49eda84dcbb8f836d79b866cf6ae9f89704b375).
- [Literatura de tricoleucemia conservada en el SV](https://github.com/juantoniolloretegea/SVperitus-dataset/tree/dominio-inmunologia/dominios/inmunologia/literatura-tricoleucemia).
- [Sede de la guía INCIBE](https://www.incibe.es/incibe-cert/guias-y-estudios/guias/medidas-de-ciberseguridad-y-ciberresiliencia-ante-modelos-de-ia-de-frontera).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
