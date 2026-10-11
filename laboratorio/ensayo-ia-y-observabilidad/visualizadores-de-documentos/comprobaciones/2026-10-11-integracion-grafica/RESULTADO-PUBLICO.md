# Paso 2 / 5. Representación gráfica de páginas PDF

11 de octubre de 2026. Edición de integración y comprobación delimitada.

Se incorpora la consulta de páginas PDF completas dentro de mdBook, con selección de página física, ampliación normal o doble, desplazamiento por teclado, identificación del original y de la imagen, avisos e informe. Hayro 0.8.0 realiza la representación en Rust. La extracción textual se mantiene como operación distinta. No se identifican ni interpretan automáticamente figuras o tablas.

## Comprobaciones y alcance

Nueve PDF sintéticos propios y dos originales autorizados para ensayo, de nueve y cuatro páginas, produjeron 44 imágenes de 22 páginas a dos escalas. Las once repeticiones previstas conservaron la misma huella. Dieciocho imágenes sintéticas coinciden byte por byte con las de la prueba anterior, cotejadas en Rust; esa coincidencia permite conservar la referencia geométrica ya comprobada, sin presentarla como una revisión científica nueva.

Se comprobaron cinco clases de rechazo por documento: página cero, página fuera de la cota, página inexistente, ampliación no admitida y archivo que no es PDF. Son 55 ejecuciones de rechazo de cinco clases, no 55 clases independientes. La corrección posterior de la información contextual se comprobó de nuevo con un caso sintético y con las pruebas de módulos. El intérprete gráfico no cambió durante esa corrección.

La inspección visual cotejó tres páginas del original de nueve páginas —1, 5 y 9— y las cuatro páginas del segundo original, frente a representaciones independientes. No se observaron omisiones manifiestas en esas páginas. No se compararon todos los píxeles de los originales reales ni se acredita fidelidad universal. El uso de fuentes incluidas en el lector se detectó en dos casos sintéticos; no aparecieron avisos del intérprete en este conjunto. La ausencia de avisos no demuestra ausencia de errores.

La recepción funcional comprende 45 pruebas Rust de módulos y 31 controles HTTPS, incluidos rechazos de página, imagen e informe sin sesión en ambos dominios. Se comprueban permisos documentales, caducidad, retirada, clasificación automática y conservación del registro. No se añadieron originales ni aprobaciones de demostración a los dominios operativos. El ensayo no incluyó inferencias de modelos.

## Incidencias corregidas antes de la activación

El informe de dimensiones se ajustó al truncamiento empleado por el intérprete. La primera presentación reutilizaba una explicación de consulta del original incompatible con la nueva vista gráfica; se sustituyó por información específica de página, método y cotejo pendiente. Los antecedentes y los resultados gráficos se conservaron. Ninguno de estos resultados autoriza por sí mismo suministro a una IA.

## Trazabilidad y límites

El Calificador de actividades documentales asigna TRA-02, «Preparación gráfica de una página PDF», dentro de Transformación. Se registra la preparación efectivamente recibida, su responsable, momento, original y resultado. No se afirma que el solicitante haya leído o aprobado la página. La imagen temporal no sustituye el informe persistente ni la autorización vigente.

La consulta exige acceso al dominio y usos documentales acreditados. Cada imagen se protege igual que su página. Una vista caducada se vuelve a abrir desde la ficha. Permanecen separados la consulta, la autorización de suministro y la entrega efectiva.

No se acreditan OCR, estructura de tablas complejas, comprensión científica, compatibilidad universal, tratamiento integral de tratados extensos ni envío de imágenes a modelos. La portada más lenta del conjunto requirió aproximadamente 41 segundos a doble escala; las mediciones no constituyen un coste económico. Los derechos de los originales y de los componentes de terceros no se amplían por esta prueba.

La explicación de uso forma parte de la V.02 de [Reglas, usos y descripciones documentales de documentos para el conocimiento de la IA](https://documentos-sv.itvia.online/documentacion/reglas-usos-descripciones-documentales/index.html). [Resultados instrumentales](COMPROBACION-GRAFICA-PUBLICA.json).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).