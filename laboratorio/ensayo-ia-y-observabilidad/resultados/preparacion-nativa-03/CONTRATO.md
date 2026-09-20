# EIO-NAT/2 · sucesión explícita de frontera experimental

Cambia la versión experimental por el campo obligatorio sello en recuperación, estados de custodia y admisibilidad común. No modifica el contrato SV ni los bytes de peticiones, plantilla o verificador. EIO-NAT/1 queda en preparación-nativa-01 y en antecedentes; no se acepta como inicio de esta candidata.

## Entradas y estados

GET /, GET /app.js y POST /api, cuerpo máximo 8192 bytes, Origin/Host exactos y application/json. Serde tipado rechaza campos extra/duplicados. Mismos códigos 400/403/409/413/429/503 del antecedente. Referencia o estructurada con texto UTF-8 exactamente igual a sus archivos; no normalización ni texto libre.

~~~json
{"op":"iniciar","contrato":"EIO-NAT/2","peticion":"referencia","texto":"<literal del archivo fijado>"}
{"op":"estado"}
{"op":"cancelar","id":"<campana>-01"}
{"op":"evidencia","archivo":"MANIFIESTO.json","offset":0,"sello":"<sha256 del manifiesto recibido en estado>"}
~~~

Los marcadores <...> son metanotación, no entrada válida. API de inicio usa una campaña fijada y un único intento. Solicitar otro tras fallo/cierre se rechaza sin cola de inferencia.

Estado de tarea: ausente, activa, cancelacion_solicitada, sellando, terminada, interrumpida, desconocida o parada_no_confirmada. Estado de custodia: recepcion, drenaje, sellando, sellada o fallida. Un cierre técnico desconocido/interrumpido/fallido nunca admite resultado.

Admision::admisible exige ocho condiciones: proceso_ok, eof_completos, secuencia_completa, !revocada, !fallo_observacion, !fallo_custodia, oraculo_ok, sellado_ok. Esta función gobierna cierre y API; interfaz usa su indicador. juicio_verificador no es condición: se muestra ESTRUCTURA junto al texto original íntegro cuando el transporte/custodia son admisibles.

resultado es null cuando no admisible. evidencia_parcial identifica el conjunto recuperable no admisible; no se convierte en resultado. Los datos originales pueden existir en prefijo sellado incompleto. No convertir ningún error técnico o ausencia en U.

## Eventos y custodia

Frame EIO-NAT/2 con id, seq desde 1, mono_ns del emisor, civil_unix_ms, tipo y datos. Máximo 65536 bytes incluida LF; marca/otel/resultado. Se mantienen seis marcas, cinco spans y resultado final; ninguna invención de esperados por conteo recibido. trace/span/padre no adquieren cobertura semántica completa por esta corrección: reserva B06 conservada.

Barrera de custodia fija corte de trabajos encolados. Tras dos segundos de drenaje sin EOF se sella sólo un prefijo con laguna_tardia_posible=true. No hay reescritura ni apéndice a los archivos definitivos después del sello. Los bytes tardíos se cuentan de forma volátil y se descartan; el volumen que no pudo observarse permanece desconocido. No hay suplemento implícito.

El sello es SHA-256 del manifiesto exacto. Fragmentos incluyen sello, nombre, offset, total, hex, fin y parcial; hasta 32768 bytes originales. Mismo sello durante todo el conjunto. Archivos: inferidor.jsonl, inferidor.stderr, supervision.jsonl, entrada.txt y MANIFIESTO.json. Sin rutas arbitrarias. Snapshot se mantiene inmutable por el ciclo de vida de la aplicación; la recuperación no abre archivos en disco.

Sólo hay conjunto disponible tras ACK completo de escritura/sync/manifiesto/cierre; errores o invalidación bloquean recuperación API. Se distinguen los archivos parciales recuperables manualmente de un conjunto sellado. El manifiesto no se auto-hashea; su SHA está en el sello/estado y debe cotejarse en receptor.

## Interfaz y pruebas

Original y juicio separados; textContent, CSP y ninguna ejecución/navegación según texto del modelo. El banco api-real atraviesa HTTP Axum y socket de control. INTERFAZ.js permite contrastar API y DOM real, sin iniciar tareas. Ningún banco se ha ejecutado.

Guardas anteriores permanecen cerradas. No URL operativa, lock nuevo, binarios, medidas o recepción científica. Qwen3-0.6B sigue siendo modelo fijado; la alternativa de desarrollador de interfaz queda retirada.

## Precisión NAT03 (compatible con EIO-NAT/2)

Se añaden campos de diagnóstico a Estado sin nueva interfaz ni cambio de entradas: estimulo (modo/caso/contrato), reaped, salida_proceso (codigo/senal/texto), parada_solicitada, senales, registro_waitpid_fallido, diagnosticos_volatiles y error_custodia. No son resultados científicos ni prueba de persistencia. La API sigue ocultando resultado salvo admisión técnica.

Fallo de custodia implica invalidación técnica y revocada=true, incluso tras reap; terminal interrumpida sin sello válido. Omisión, exit no cero y EOF ausente con custodia sana mantienen terminal desconocida y exigen prefijo sellado incompleto. Cancelación con custodia sana mantiene interrumpida y exige su prefijo. Los esperados concretos están fijados en CASOS-NAT03, sin aceptar errores genéricos.

El journal añade recibos tipo trabajo con ordinal, clase y contenido identificable, sin alterar frames del emisor ni las cuatro entradas de inventario. El corte incluye esos trabajos, no los recibos como nuevos trabajos recursivos. Cada recibo se escribe después de la operación correspondiente y antes de la barrera.

El banco impone cotas globales y coherencia descritas en ORACULOS. API_INDISPONIBLE y PARADA_NO_CONFIRMADA son categorías de fallo que requieren evidencia exterior, nunca un resultado conforme alternativo. Sin reap tras 5 s, la ventana diagnóstica de 2 s es un intento acotado, no garantía de disponibilidad ante fallo de servidor o kernel.
