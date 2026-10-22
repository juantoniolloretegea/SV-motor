# Evidencias de la comparación

## 1. Estado de la edición

La publicación revisada del 22 de octubre de 2026 contiene cuatro resultados de la edición general `eet-cyb16-multiformato-20261021-v2` y tres de la adaptada `eet-cyb16-markdown-adaptada-20261021-v2`: DeepSeek, GLM y Grok. Kimi permanece pendiente, sin dictamen. La relación completa figura en [COMPARACION.json](COMPARACION.json).

Cada carpeta publicada reúne `respuesta.txt`, `resultado.json` y `poligono.json`. `respuesta.txt` conserva, sin reescritura, el texto recibido del modelo que se entregó al corrector; puede contener JSON. No es el contenedor íntegro de eventos de la API. `respuesta_textual_sha256` identifica ese archivo; `respuesta_original_sha256` identifica la respuesta de transporte conservada con acceso restringido. Son objetos diferentes y sus huellas no son intercambiables.

`resultado.json` recoge el identificador público del intento, la edición, el modelo comunicado, las huellas de paquete y petición, las calificaciones, la evaluación del SV y las mediciones seleccionadas. `poligono.json` conserva el contrato visual, las dieciséis posiciones ordenadas, sus enunciados y criticidades, κ y el dictamen. La tabla CSV, el JSON comparativo y la figura de tiempos proceden de la misma clasificación recibida.

Esta publicación selecciona los resultados científicos y los contadores comunicados. Excluye datos de cuenta, identidades de acceso, credenciales, registros administrativos, contenedores nativos de transporte y registros JSONL del proceso compartido. Estos últimos permanecen bajo custodia restringida; no se afirma que su contenido íntegro sea público. El coste comparable todavía no está acreditado. Una ausencia de contador no se convierte en cero. Las declaraciones de recursos, procedimientos y revisiones se consultan en el texto del modelo y se distinguen de las mediciones observadas.

La [revisión común del 22 de octubre](REVISION-CORRECCIONES-20261022.md) conserva la rectificación formal de GLM y el cotejo de todos los originales recibidos. Su explicación defectuosa permanece literal en la respuesta y señalada en los datos. No se reparan respuestas, no se repiten exámenes y no se ocultan antecedentes.

La [segunda revisión común](REVISION-CIERRE-AUXILIAR-20261022.md) coteja los siete originales y rectifica únicamente Grok: falta una llave en su explicación final, sin ambigüedad en las dieciséis respuestas. Se conserva el defecto y el dictamen formal anterior. No añade consumo ni otra entrega al proveedor.

## 2. Conjunto de un intento

La evidencia de cada resultado se organiza mediante los elementos siguientes bajo un identificador público de intento y una carpeta de su candidato:

| Elemento | Relación exigida |
| --- | --- |
| Identidad | Edición, categoría, porcentaje y limitación declarada, candidato, modelo efectivo comunicado, modalidad y configuración pertinente. |
| Entrega | Huellas de fuentes, cuadernillos, paquete común y adaptación efectivamente utilizada; orden de preguntas. |
| Respuesta | Respuesta original publicable, hash, formato y relación con la entrega. |
| Corrección | Respuestas por identificador, claves, calificaciones, criticidades, recuentos, κ y dictamen. |
| Explicaciones | Explicación y revisiones adversariales recibidas por pregunta, separadas de la puntuación; ausencias declaradas. |
| Tiempo | Tiempo efectivo, pausas, duración del transporte, primer fragmento y estado de disponibilidad. |
| Consumo | Campos nativos de tokens y caché, unidades, procedencia y disponibilidad; sin duplicar subconjuntos. |
| Costes | Importe comunicado, cálculo comparable y conciliación posterior, con fuentes y diferencias explícitas. |
| Observación local | Recibo, huella y JSONL autorizado; ámbito del proceso compartido y límites de la medición. |
| Representación | Datos del vector, figura del polígono y gráficos comparativos derivados del mismo resultado. |
| Procedencia | Manifiesto de archivos, tamaños y SHA-256; recepción y rectificaciones identificadas. |

La ubicación es `resultados/<candidato>/<intento-publico>/`. Sólo se incorpora cuando exista evidencia recibida para ese intento. El nombre de la carpeta no sustituye la identificación de su contenido.

## 3. Correspondencia con la custodia

El archivo íntegro conserva los originales y las actuaciones con sus permisos. El conjunto publicable se selecciona expresamente y se identifica antes de copiarlo. Cada archivo publicado debe coincidir byte a byte con ese conjunto; el manifiesto permite comprobarlo.

Si un original contiene información no publicable, no se expone ni se presenta una versión recortada como original íntegro. Se conserva el original restringido y, cuando proceda, una proyección pública identificada como tal, con campos omitidos y motivo documentados. Credenciales, cabeceras de autenticación, datos de cuentas, saldos privados, destinatarios y registros ajenos quedan fuera de esta publicación.

Una huella acredita correspondencia entre archivos cotejados; no sustituye controles de acceso, custodia, recepción competente ni firma independiente. El enlace privado no se publica como si permitiera acceso público a la evidencia.

## 4. Estados y valores ausentes

- `dictamen`: exclusivamente `"0"`, `"1"` o `"U"` cuando se haya emitido; vacío en CSV o `null` en JSON cuando no exista.
- `kappa`: clasificación matemática separada del dictamen contractual, cuando exista vector válido.
- `estado` y `causa_ausencia`: describen preparación, detención, incompatibilidad pendiente o falta de medición.
- Magnitudes desconocidas: vacío o `null`, nunca cero por defecto.
- Incidencia instrumental: sin dictamen ni clasificación del candidato.

El resultado completo puede ser reproducible aunque no exista una medición opcional del proveedor. En cambio, la pérdida de observación local obligatoria impide cerrar el intento con dictamen.

## 5. Gráficos y rectificaciones

Las tablas, gráficos y polígonos se construyen desde el mismo conjunto de resultados recibidos. Cada figura debe declarar edición, categoría, candidatos, unidades, ausencias y fuente de datos. No se dibujan datos desconocidos en cero, ni un vector incompleto como resultado completo.

Una rectificación conserva la versión anterior y el motivo de la nueva. Cualquier defecto común del examen debe tratarse de forma uniforme para los resultados afectados. La publicación no recalifica silenciosamente los antecedentes ni concede autorización de producción.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
