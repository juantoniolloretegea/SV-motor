# Servicio documental MCP — preparación 0.1.0

Fecha: 26 de septiembre de 2026. Estado: código compilado y comprobado localmente; recepción en OneCloud e integración con el cliente real del motor pendientes. No constituye una acreditación clínica ni una autorización del paso 2.

## Entrega

Se incluyen código Rust, Cargo.lock, ejecutables Linux x86_64, catálogo documental, HTML original del NCI, pruebas, cliente técnico y evidencias. El paquete no contiene pesos, claves, historias clínicas ni respuestas del banco HCL. No utiliza Python.

Ejecutables:

- `sv-mcp-documental`: servicio JSON-RPC 2.0 por entrada y salida estándar. El registro se escribe en un archivo nuevo; la salida estándar se reserva al protocolo.
- `importar-pdq`: operación administrativa sin red; convierte un HTML previamente obtenido del PDQ en el catálogo. No es una herramienta del modelo.
- `comprobar-mcp`: cliente técnico Rust; inicia el servicio, comprueba el protocolo, reconstruye el documento completo y cierra el proceso. No contiene un motor de inferencia.

## Herramientas

`buscar_documentos({"consulta":"texto","limite":5})` busca todos los términos (máximo ocho), sin distinguir mayúsculas. La búsqueda es literal, con orden documental y sin clasificación semántica; no elimina acentos ni interpreta sinónimos. Devuelve hasta cinco secciones y señala coincidencias adicionales.

`leer_documento({"documento":"pdq-nci-hcl-es","seccion":"_1","pagina":0})` recupera 2.000 caracteres Unicode de la sección solicitada. `siguiente_pagina` indica contenido pendiente. Concatenar todas las páginas recupera exactamente la sección conservada. La procedencia incluye la URL, el identificador HTML, las fechas declaradas y huellas del original y de la sección.

El catálogo completo reside en memoria, validado contra una huella proporcionada por el administrador. No se admiten rutas, URL arbitrarias ni escritura documental desde las herramientas. El servicio no contiene funciones de red, shell ni acceso al núcleo.

## Fuente documental

Original: https://www.cancer.gov/espanol/tipos/leucemia/pro/tratamiento-celulas-pilosas-pdq

Adquisición HTTPS directa, sin seguimiento de redirecciones. Instante registrado a partir de la finalización del archivo: 2026-09-26T12:20:31Z. El HTML declara actualizaciones de 14-11-2024. Esta es una captura actual; no se afirma identidad con la captura histórica de la campaña.

Se conservan las cinco secciones principales, incluidas bibliografía, actualizaciones e información editorial. La conversión HTML a texto es mecánica mediante dependencias fijadas. Se conservaron el HTML y los enlaces para revisar cualquier diferencia de presentación. Los controles acreditan conservación del texto extraído al transportarlo, no equivalencia semántica certificada entre todas las representaciones HTML y texto.

La procedencia oficial y las huellas no convierten una interpretación generada en correcta. El catálogo no contiene soluciones del banco. Los textos externos tampoco otorgan permisos al servicio; su influencia sobre un modelo futuro debe evaluarse separadamente.

## Recursos y límites

- Catálogo: 2 MiB, hasta ocho documentos y 64 secciones por documento; una sección hasta 512 KiB.
- Solicitud: 16 KiB por línea; respuesta de herramienta hasta 8.000 caracteres JSON serializados.
- Consulta: 200 caracteres y hasta ocho términos; una llamada procesada cada vez.
- Sesión: 256 tramas y hasta 128 llamadas de herramientas; diario hasta 8 MiB.
- El cliente técnico impone 30 s de espera por respuesta y termina el proceso en caso de fallo. La configuración propuesta del cliente mistral.rs también fija 30 s y concurrencia uno.
- El servicio comprueba cooperativamente el plazo durante la operación en memoria. No garantiza por sí solo interrumpir una entrada/salida de disco o tubería bloqueada: el supervisor externo debe imponer el plazo y cerrar la conexión. Después de un vencimiento no se reutilizará la sesión.
- Las trazas son JSONL propias, con solicitud, resultado y duración. No se afirma instrumentación OpenTelemetry en esta versión.

El servicio es una aplicación con capacidades limitadas, no un aislamiento del sistema operativo ni una prueba de resistencia a un anfitrión comprometido. Para una instalación posterior, ejecutar con usuario sin privilegios, catálogo de solo lectura y directorio de diarios separado. No dar acceso al núcleo ni a secretos.

## Verificación y recepción

Leer `INSTRUCCION-RECEPCION.md`. `bash VERIFICAR.sh` verifica SHA256SUMS, exige Rust 1.98.0, recompila con Cargo.lock y ejecuta pruebas y el cliente sin modelo. Conserva una nueva carpeta `recepcion-*`; no sobrescribe diarios previos. El script tiene límites de tiempo pero la contención de memoria deberá imponerla el supervisor de OneCloud.

Los ejecutables incluidos se compilaron en Linux x86_64 y utilizan glibc/libgcc. Son auxiliares; la recepción recompila las fuentes en el servidor. No se promete identidad binaria entre anfitriones distintos.

`mcp-config.plantilla.json` contiene marcadores que deben sustituirse por rutas absolutas. No está aplicada al motor. El diario exige nombre nuevo por sesión. Una nueva sesión debe utilizar otro nombre. No activar esta configuración en esta entrega.

## Dictamen limitado

Diez pruebas automatizadas correctas. Cliente real propio por proceso: 32 peticiones, 25 páginas reconstruidas exactamente; errores reconocidos. Compilación release correcta con Rust 1.98.0. Ver evidencias.

Pendiente: comprobar el ejecutable y las opciones reales en OneCloud, integrar con el cliente real de mistral.rs y observar una llamada generada por Qwen. Ninguna de esas tres comprobaciones se presenta como realizada. El resultado clínico anterior se conserva sin cambios.
