# EIO-NAT/1 · contrato experimental

Versión de esta frontera exclusivamente. No sustituye contrato SV, Pilares, perfiles o transición. Ausencia, rechazo técnico, interrupción y desconocimiento no se convierten en U.

## Entrada HTTP

GET /: HTML identificado; GET /app.js: JavaScript identificado. POST /api: JSON UTF-8, máximo 8192 bytes de cuerpo completo; application/json exacto, Origin/Host configurados. Serde con deny_unknown_fields; no extraer JSON de Markdown ni corregirlo. Duplicados de campos definidos se rechazan en deserialización tipada.

Órdenes:
~~~json
{"op":"iniciar","contrato":"EIO-NAT/1","peticion":"referencia","texto":"<bytes exactos de pruebas/peticion.txt>"}
{"op":"estado"}
{"op":"cancelar","id":"<campana>-01"}
{"op":"evidencia","archivo":"MANIFIESTO.json","offset":0}
~~~

Los signos <...> anteriores son metanotación documental, no entradas válidas. peticion admite referencia/estructurada exclusivamente. texto es string cuyo UTF-8 decodificado debe coincidir exactamente con el archivo respectivo: no trim, normalización Unicode ni cambios de saltos. La representación JSON exterior puede escapar caracteres sin cambiar el string; se guarda entrada.txt con los bytes literales ya fijados. ID de tarea configurado antes, sin identidad tomada del modelo.

400 para estructura, versión o petición no fijada; 403 para origen/host/tipo; 413 para cuerpo excesivo; 429 si cuatro operaciones de API ya ocupan el semáforo; 409 para sesión consumida/no cancelable/archivo no disponible; 503 si supervisor no confirma. Cualquier error de transporte impide interpretar la respuesta ausente como éxito. No hay reintentos automáticos.

## Estados y resultados

- ausente: no se admitió intento.
- activa: hijo iniciado; resultado aún no admisible.
- cancelacion_solicitada: revocación solicitada, parada no acreditada.
- interrumpida: revocada antes de compromiso, hijo recogido y cierre exterior; evidencia puede ser incompleta.
- terminada: exit 0, dos EOF, sin error de custodia y oráculo completo; NO significa juicio OK.
- desconocida: salida/error/cobertura/custodia sin requisitos suficientes.
- parada_no_confirmada: tras señales no se acredita desaparición; detener desde plataforma y conservar incertidumbre.

Un solo intento por lanzamiento, sin reusar ni reiniciar. Consulta devuelve ID, estado, revocación, evidencia_cerrada, última muestra de memoria y resultado cuando admisible. La última muestra lleva reloj en journal; no se la presenta como medida instantánea. Salida_original y juicio_verificador son campos distintos, sin ejecutar accion del modelo. La interfaz inserta texto mediante textContent; nunca innerHTML, eval ni navegación a URLs del modelo.

## Frames del inferidor

Objeto con contrato, id, seq (desde 1), mono_ns del propio proceso, civil_unix_ms, tipo y datos. Máximo 65536 bytes incluida LF. Tipos marca, otel, resultado; un resultado final, ninguno tras él. OTel conserva trace/span/padre y pérdidas. Sus intervalos no se inventan a partir de relojes de otro proceso.

Inventario independiente: marcas inicio, pesos.leidos, modelo.antes, modelo.despues, forward.antes, forward.despues; spans consulta.A, consulta.B, generacion.inicio, generacion.fin, peticion. Orden intraclase exigido, además de seq global contigua. Eventos ausentes/duplicados/fuera de orden no se reparan; frames originales se conservan. Un aborto legítimo puede carecer de marcas finales: se declara incompleto.

Resultado del modelo: salida_original con texto, fin, entrada, generados, segundos y tokens ordenados del adaptador; juicio_verificador literal, efectos_ejecutados=0. Se exige seguir usando el verificador existente. En el modo testigo se identifica sintetico=true y tokens vacíos; no equivale a resultado Candle. Esta revisión exterior aún no valida cada campo semántico del frame de resultado ni todas las relaciones trace/padre frente a emisor hostil: brecha explícita.

## Evidencia

Sólo nombres permitidos: inferidor.jsonl, inferidor.stderr, supervision.jsonl, entrada.txt, MANIFIESTO.json. offset u64 dentro del tamaño; respuesta hasta 32768 bytes en hexadecimal, archivo, offset, total y fin. Ninguna ruta arbitraria. Sólo después de cierre; antes, el archivo persiste localmente pero el endpoint no lo ofrece.

Hash y tamaño de cuatro originales en manifiesto; estado de escritura incompleto no se regulariza. SHA-256 de manifiesto se obtiene aparte y se conserva en ambos extremos. Autenticidad de transporte, identidad de archivos, cobertura instrumental, corrección contractual y aceptación científica son comprobaciones distintas.
