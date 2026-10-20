# Evidencias de la comparación

## 1. Estado de la edición

No existen todavía intentos ni resultados de `eet-cyb16-multiformato-20261020-v1`. La publicación contiene el examen fijado y su plantilla. La tabla comparativa carece de filas de resultados; no se publican respuestas prefijadas como respuestas de candidatos.

## 2. Conjunto de un intento

Cada resultado recibido deberá reunir los elementos siguientes bajo un identificador público de intento y una carpeta de su candidato:

| Elemento | Relación exigida |
| --- | --- |
| Identidad | Edición, candidato, modelo efectivo comunicado, modalidad y configuración pertinente. |
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

La ubicación prevista es `resultados/<candidato>/<intento-publico>/`. Sólo se incorpora cuando exista evidencia recibida para ese intento. El nombre de la carpeta no sustituye la identificación de su contenido.

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

Las tablas, gráficos y polígonos se construyen desde el mismo conjunto de resultados recibidos. Cada figura debe declarar edición, candidatos, unidades, ausencias y fuente de datos. No se dibujan datos desconocidos en cero, ni un vector incompleto como resultado completo.

Una rectificación conserva la versión anterior y el motivo de la nueva. Cualquier defecto común del examen debe tratarse de forma uniforme para los resultados afectados. La publicación no recalifica silenciosamente los antecedentes ni concede autorización de producción.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
