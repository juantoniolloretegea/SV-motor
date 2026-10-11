# Paso 3 / 8 · Fuentes y casos del Buscador-Semántico - Diferencial

**11 de octubre de 2026 · Preparación anterior a la ejecución.**

## Objeto y delimitación

Preparar la comprobación de una ayuda opcional que compara un documento propuesto para la IA con documentos ya incorporados de su mismo dominio. El responsable podrá elegir un documento, un grupo o todos los documentos accesibles de ese dominio. Rust aplicará la selección y sus límites antes de entregar el conjunto al Buscador-Semántico - Diferencial.

Esta preparación utiliza un conjunto experimental independiente de la administración. Los cuatro documentos de referencia representan, exclusivamente dentro del ensayo, documentos ya incorporados. No se han alterado estados, permisos ni documentos de la biblioteca para simular esa incorporación. Inmunología y ciberseguridad permanecen separadas.

El estudio es preliminar y acotado a pasajes. No comprueba todavía documentos completos, no estima precisión general y no introduce una condición de detección perfecta. Los textos de prueba no son recomendaciones clínicas ni instrucciones para actuar sobre sistemas.

## Fuentes y cobertura

Se conserva la identidad de cinco originales previamente utilizados: la copia HTML de NCI-PDQ, la hoja informativa de LLS sobre tricoleucemia, los episodios CYB25, la guía facilitada de INCIBE y el PDF original NIS2. Sus huellas y la función de cada fuente constan en `FUENTES.json`.

Los doce casos emplean ocho pasajes procedentes de los cuatro primeros originales. La representación de LLS se ha recuperado de una revisión fija de GitHub y coincide con la huella anteriormente conservada. Para NCI se reutiliza el catálogo vinculado a la copia HTML histórica. Se distinguen esa copia y otras representaciones publicadas; no se presume identidad de bytes entre ellas. INCIBE conserva su extracción por páginas y CYB25 sus episodios sintéticos.

Se han inspeccionado visualmente las páginas físicas 1 y 5 de LLS y la página física 5 de INCIBE, cotejando los pasajes seleccionados con sus representaciones. Esta inspección no acredita recuperación integral de los PDF. En INCIBE se usa el ordinal físico, porque los totales impresos de algunos pies de página no son uniformes.

NIS2 se conserva como referencia documental adicional, sin asignarle pasajes a estos doce casos. La extracción anterior mediante el lector acotado no terminó porque sus 73 páginas superaban la cota admitida. Su síntesis de lectura no se utiliza como si fuera transcripción literal. La cobertura de ese original se atenderá en el paso de formatos; no se reinterpreta aquí el Derecho vigente.

## Casos fijados

| Dominio | Incompatibilidades controladas | Casos compatibles | Contexto insuficiente |
| --- | --- | --- | --- |
| Inmunología | I01: linaje celular; I03: universalidad de la necesidad inmediata; I05: necesidad de seguimiento. | I02: descripciones concordantes; I04: condiciones de aplicación distintas. | I06: casos y tratamiento sin identificar. |
| Ciberseguridad | C01: autenticación; C03: verificación de solicitudes; C04: facultad de actualización en un episodio ficticio. | C02: repetición literal; C05: medida concreta y función complementaria. | C06: entorno de aplicación no identificado. |

Las seis incompatibilidades son controles deliberados, no contradicciones descubiertas entre las publicaciones originales. Las sustituciones se describen en `REFERENCIAS.json` y no se atribuyen a NCI, LLS o INCIBE. Cuatro entradas conservan pasajes literales; cinco aplican sustituciones identificadas y tres son controles propios contextualizados. Se conserva el texto fuente y el texto propuesto por separado.

Los resultados esperados se refieren a los pasajes y al contexto fijado, no a la coherencia completa de los originales. Han sido preparados como referencias del estudio, sin constituir una validación independiente. No se asigna U del SV a los dos casos de contexto insuficiente.

## Separación de entradas y referencias

- `CORPUS.json`: cuatro documentos de referencia, ocho pasajes, procedencia, localizadores y grupos del ensayo.
- `referencias/`: representaciones Markdown de esos documentos parciales.
- `propuestos/`: doce documentos propuestos para la comparación.
- `ENTRADAS.json`: documentos, dominio y selección que podrá recibir el instrumento.
- `REFERENCIAS.json`: resultado esperado, fundamento y transformación. No se suministrará al candidato.
- `SELECCIONES-PREVISTAS.json`: doce comprobaciones previstas de selección y aislamiento; todavía no ejecutadas.

La separación se aplicará por los archivos que se suministren efectivamente, no por una mera declaración dentro del documento. Las entradas conservarán sus huellas; cualquier cambio posterior exigirá una nueva edición identificada, sin sobrescribir esta fijación.

## Comprobaciones de selección previstas

Se han definido solicitudes individuales, por grupo y completas en ambos dominios. También se han previsto selección vacía, ayuda no solicitada, grupo inexistente, documento sin permiso, grupo con una referencia ajena y discrepancia entre el dominio del documento propuesto y el de la solicitud.

El resultado esperado siempre identifica el conjunto exacto. «Todos» se limita a los documentos accesibles del dominio; una solicitud inválida no se convierte en una selección más amplia. Los casos rechazados, vacíos o no solicitados no deben producir llamadas al comparador. Estos requisitos se comprobarán en Rust durante la integración; su inclusión en este protocolo no acredita que ya estén implementados.

## Idioma y condición de ejecución

Las entradas actuales están en español. No se han traducido ni reescrito para acomodarlas a patrones del candidato. La selección del paso 4 debe comprobar conjuntamente función, idioma y dependencias. Si se utiliza un candidato que requiere inglés, la representación traducida deberá prepararse, conservarse y cotejarse antes de ejecutar el contraste; no se tratará una falta de reconocimiento del idioma como ausencia de contradicciones.

La traducción instrumental al inglés sigue dentro del alcance autorizado. Su necesidad puede anticipar una comprobación del paso 6 como dependencia del paso 5, conservando los ocho pasos y explicando esa relación. No autoriza añadir una IA como analizador ni sustituir el contenido por una formalización manual.

## Estado al cierre de la preparación

Doce casos y doce solicitudes de selección preparados; cinco originales identificados, ocho pasajes localizados y cuatro representaciones parciales de referencia. Los resultados esperados preceden a la ejecución. No se ha ejecutado el candidato ni se han medido precisión, ahorro de tiempo o utilidad en el trabajo de revisión. Las comprobaciones de integridad en Rust y la recuperación de esta publicación acreditan conservación documental, no capacidad semántica del comparador.

## Atribución y derechos

Los pasajes conservan la atribución y las condiciones de cada fuente. Los textos de LLS y NCI no quedan sujetos por esta publicación a la licencia editorial del informe. Los pasajes de INCIBE y los controles derivados de ellos conservan CC BY-NC-SA 4.0, con identificación de las alteraciones experimentales; no representan recomendaciones ni respaldo de INCIBE. El pie siguiente corresponde a la elaboración editorial propia y no sustituye los derechos de terceros identificados en `CORPUS.json` y `FUENTES.json`.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
