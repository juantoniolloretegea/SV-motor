# EIO-JSON-01 · Revisión receptora y continuidad

Fecha: 20 de septiembre de 2026.
Corte recibido: fb901ff428a4b5774516f589661232e8fb9339d0.
Precompromiso ejecutado: 81fd7ea90c354e43edd222f9984c177e1fdfb7ab.
Run 35496525105; número 7, intento 1; job 106040396115.

## Dictamen acotado

Se reciben como comprobados en el registro de ejecución los 24 controles de regresión y los 35 del complemento, incluidos seis controles de sensibilidad. Se conserva una reserva documental explícita: no está acreditada la identidad binaria entre los archivos efímeros del runner y los segmentos recuperados de su log. Por ello no se declara cumplimiento íntegro de la custodia prevista ni aceptación científica general.

No se repite EIO-JSON-01 para suplir retrospectivamente esa ausencia. La corrección de emisión de huellas se incorpora como condición de preparación de la próxima ejecución que tenga un objetivo experimental propio. Las guardas permanecen cerradas y esta recepción no autoriza una ejecución número 8.

## Contraste realizado

1. Consulta directa de la API de GitHub: run 35496525105, número 7, intento 1, estado completed, conclusión success y SHA de precompromiso coincidentes.
2. Recuperación separada del log del job mediante el conector: la cadena decodificada coincide exactamente con JOB_106040396115.txt publicado. Su tamaño UTF-8 es 68169 bytes y su SHA-256 es be2d7ece815c8b73cc05ddf1f3ba807905c66ddbae5e88ef2160ff2eb61e828f. Esto compara lecturas del mismo servicio; no constituye un observador independiente del host ni acredita los bytes del ZIP original.
3. Recuento sobre ese log: 19 registros de adaptador, cinco de control del generador, 35 de EIO-JSON-01 y un resumen. Los 59 controles son conformes; el resumen no se cuenta como un control adicional. Los 60 registros coinciden, por contenido JSON, con CONTROLES.jsonl.
4. Cuatro fases con retorno 0, causa normal y ausencia de procesos residuales según el supervisor. El arnés Rust informa un test de integración satisfactorio, dentro del cual se ejecutan los 35 controles.
5. Tamaños UTF-8 y SHA-256 recalculados para las 19 entradas de MANIFIESTO.tsv: sin discrepancias. Los archivos vacíos se identifican asimismo mediante el blob Git vacío. Estas huellas corresponden a los productos publicados, no a originales efímeros no recuperados.
6. Comparación de los once blobs de fuentes fijadas entre el corte EIO-06 y el corte recibido: todos permanecen idénticos. El log registra también el cotejo previo y posterior de fuentes y paquete.
7. Las dos guardas publicadas están cerradas. No se ha lanzado ninguna ejecución durante esta revisión.

El contraste anterior es documental y de registros; no es una nueva ejecución del banco ni una reproducción independiente de inferencia.

## Reserva de custodia y responsabilidad de preparación

El supervisor transmite stdout, stderr y medidas como texto al log, pero el controlador preparado por recepción no exigió emitir previamente tamaño y SHA-256 de cada archivo efímero. Esta insuficiencia estaba en el procedimiento entregado; no procede atribuirla al ejecutor por haberlo seguido sin modificarlo.

La extracción elimina marcas temporales y separadores. Por tanto, las huellas calculadas después no pueden demostrar retrospectivamente identidad con el archivo anterior a la transmisión. Conservar el log literal permite revisar los resultados registrados y su extracción, pero no elimina esta reserva.

Para el siguiente precompromiso: emitir en el runner tamaño y SHA-256 de cada archivo de evidencia cerrado; transmitirlo sin pérdida por un mecanismo acotado y cotejar esas identidades después de recuperarlo. Distinguir archivo vacío, archivo ausente, truncamiento y error de transporte. No emitir huellas de archivos mientras sigan cambiando. Comprobar este mecanismo con testigos conocidos antes de utilizarlo como fundamento de conformidad. No requiere una biblioteca adicional de observabilidad.

## Alcance de la comprobación JSON

Quedan corroborados los resultados precomprometidos para duplicados tipados, objetos o texto adicionales, enteros fuera de contrato, Unicode y conservación de campos y secuencias. Las dos salidas históricas mantienen ESTRUCTURA. No se repara ni extrae JSON del cercado Markdown.

Los seis controles de alteración acreditan sensibilidad del comparador ante sus seis mutaciones concretas. No son una estimación de tasa de detección poblacional.

La célula SV (9,3) y las restantes estructuras constituidas conservan su obligación propia de identidad de parámetro, valor ternario y posición. El contrato de referencias A/B de este banco sólo exige cobertura: no constituye esa célula ni acredita su representación. Debe evitarse interpretar la expresión descriptiva «vector posicional» como una nueva categoría del SV.

Seguridad activa observada en este alcance: controles de admisión del receptor y rechazo antes del efecto sintético. Seguridad pasiva: custodia y trazabilidad de resultados, con la reserva indicada. No se acredita seguridad integral del sistema anfitrión.

## Continuidad del experimento original

Se corrige el resumen anterior que presentaba la comparación con/sin telemetría como no ejecutada. EIO-05 ya custodia siete inferencias: una llamada separada y tres pares on/off, con tokens equivalentes. COSTES.json informa diferencias pareadas de aproximadamente -84,300; 28,758 y 87,915 ms, con mediana 28,758 ms. El intervalo incluye carga e inferencia; no aísla el coste puro de OpenTelemetry. Tres pares no permiten una conclusión general de rendimiento. Se conserva esta evidencia sin repetirla por una omisión del resumen.

La secuencia continúa así:

1. Mantener recibido este complemento con su reserva; conservar EIO-05 y EIO-06, incluido el rechazo estructural de la salida del modelo. Un resultado adverso válido no exige modificar el contrato hasta lograr éxito.
2. Preparar EIO-P-12: ejecución real en navegador del componente WASM y su instrumentación, sobre un candidato identificado. La comprobación WASM de EIO-06 fue satisfactoria, pero no ejecutó el navegador. La preparación debe revisar enlace, reloj, dependencias de destino, límites, identidad del navegador y transporte de evidencia corregido; no instalar en el PC ni introducir otro modelo.
3. Fijar antes de ejecutar qué propiedades se comparan con la referencia nativa y qué discrepancias se conservarán. No cambiar simultáneamente el contrato JSON o las peticiones del modelo, para no confundir variación de entorno y variación funcional.
4. Tras la ejecución acotada que corresponda, consolidar la matriz EIO-P-01 a EIO-P-15 con resultados conformes, adversos y no comprobados. Emitir la conclusión experimental limitada y remitirla a la sede receptora del bloque (p1+P3)-Bis.

La preparación del navegador es el siguiente trabajo del ensayo. Su despliegue y ejecución necesitarán un precompromiso concreto y presupuesto propio; no quedan habilitados por esta recepción. Cloudflare y el estudio futuro de varios agentes permanecen fuera de esta continuación.
