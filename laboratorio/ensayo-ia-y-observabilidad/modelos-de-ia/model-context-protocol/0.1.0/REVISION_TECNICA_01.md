# Revisión técnica de la preparación documental MCP 0.1.0

Fecha: 26 de septiembre de 2026. Revisión documental y estática 01. **Estado resultante: preparación incompleta.**

## 1. Objeto, identidad y método

Se examinan el código, las evidencias conservadas, los límites declarados y las condiciones de recepción del prototipo. No se recompila ni ejecuta el servicio, no se accede a OneCloud y no se realizan inferencias. Los contraejemplos descritos son deducciones del código; no se presentan como nuevas pruebas ejecutadas.

La preparación examinada está fijada en [SV-sala-de-maquinas, commit 24e6204b3fc7c082a04a6dc785f9491c7ce8e238, entrega-01][origen]. El archivo `SV-MCP-DOCUMENTAL-0.1.0-20260926.tar.gz` ocupa 1 562 144 bytes y tiene SHA-256 `6390b1df7a324aa5b5f2590ea3636db7e182269b592540a3ac0f8f7d25df8aa3`. Se conservan sus 25 entradas del manifiesto interior. La copia versionada mantiene los 32 archivos de la entrega, sin modificación de fuentes, ejecutables, catálogo, registros o instrucciones originales.

Se contrastan directamente `mistralrs-mcp/src/client.rs`, `transport.rs`, `types.rs` y `lib.rs` en la revisión del motor `2370966bb91e2e3dafa0b1521b87c50fd5c01244`. La lectura de esas fuentes no demuestra las características de compilación del ejecutable instalado.

## 2. Seguimiento y antecedentes recibidos

Corte documental del Lenguaje: `b1d826981a329f0d3e1295e8989268ca349acf0f`, rama `main`. Se consultan el registro vigente de sucesos y su historial, el índice de TT y las fichas TT-0001, TT-0008, TT-0009, TT-0012 y TT-0013; los Pilares, el acta de perfiles y contratos, el acta de transición desde OP-IMM-001, la Acta 001 y los apartados pertinentes de la Acta 004, especialmente §22, de la continuación del 15/09/2026.

S39 permanece **pendiente**, con última revisión 22. TT-0013 está finalizado exclusivamente para la campaña y distribución GPT-OSS. TT-0008 conserva el resultado adverso de una campaña documental distinta; TT-0009 acredita una inspección delimitada de capacidades, no una garantía general de supervisión. Sus resultados no se trasladan al MCP ni a Qwen3.8-27B. En los registros examinados no consta una recepción técnica específica de este paquete MCP.

Este informe es evidencia preparatoria de laboratorio: no modifica el estado de los sucesos, no emite un TT de ejecución concluida y no constituye aceptación definitiva de Calidad. La recepción canónica posterior conserva su sede en `docs/calidad/tuberias-ia/continuacion-15-09-2026/`; el mapa histórico queda inalterado.

El [antecedente HCL fijado en ffed6b40339c4ac19c8da18a1ceaf75c1ce78578][hcl] mantiene cuatro controles lógicos conformes, HCL-01 en revisión, HCL-02 detenido por error diagnóstico relevante y HCL-03 a HCL-12 sin ejecutar. Esta revisión no modifica esos dictámenes.

## 3. Hallazgos que impiden declarar conformidad completa

### MCP-REV01-01 · Plazo y cierre material no acreditados

`Session::handle` establece un plazo de treinta segundos para `execute`. Las comprobaciones son cooperativas; no gobiernan la lectura de la trama, la sincronización del diario ni la escritura en la tubería. En el cliente propio, `send` escribe y vacía la entrada antes de iniciar `recv_timeout`. Por tanto, ese plazo tampoco cubre una escritura bloqueada. Además, `recv_timeout` transforma tanto el vencimiento como la desconexión del canal en un mismo texto diagnóstico, insuficiente para distinguir sus causas.

En el cliente de la revisión fijada de mistral.rs, el vencimiento produce un error, pero esa rama no llama a `close`. `ProcessTransport::send_request` toma el resultado de la siguiente línea sin comprobar el identificador recibido. La concurrencia uno no impide que una respuesta tardía se confunda con la petición siguiente si se reutiliza la sesión. El alcance de esta deducción es el código examinado; no se afirma haber observado ese fallo en OneCloud.

**Consecuencia:** antes de declarar conformidad se necesita una prueba sintética de vencimiento con captura de la causa, cierre del proceso y prohibición efectiva de reutilización de la sesión. La supervisión debe abarcar toda la operación y su conservación. El plazo exterior de sesenta segundos del cliente en `VERIFICAR.sh` no demuestra el límite de treinta segundos por llamada.

### MCP-REV01-02 · Conservación incompleta en las vías de fallo

`VERIFICAR.sh` comprueba las huellas antes de crear la carpeta de recepción. `set -e` interrumpe el guion si una orden falla; no existe un registro incondicional del código final y del cierre. La invocación del cliente conserva la salida estándar, pero no redirige su salida de error a un archivo. El cliente hereda la salida de error del servicio.

El servicio abre el catálogo antes de crear el diario. Un catálogo ausente o inválido puede, por ello, terminar únicamente con diagnóstico por la salida de error. Un fallo al leer una trama también sale antes de registrar esa operación. Las trazas de las llamadas completadas no cubren todos estos casos.

**Consecuencia:** el receptor debe conservar, desde antes de la primera comprobación, órdenes, entradas pertinentes, ambas salidas, códigos de terminación y evidencia de cierre en un destino nuevo. No basta conservar `resultado.json` ni interpretar una ausencia de resultado como vencimiento.

### MCP-REV01-03 · Paginación no suficiente para todo texto admitido

Las páginas se forman con 2 000 caracteres del texto original. Después, el contenido se serializa como JSON dentro de una cadena de otra respuesta JSON. La cota de 8 000 caracteres se comprueba al final; si se supera, se sustituye la respuesta por `RESPUESTA_SUPERIOR_AL_LIMITE`.

Contraejemplo estático: una sección sintética formada por 2 000 comillas dobles, con metadatos válidos y huella correcta, satisface las restricciones de `Catalog::validate`. Cada comilla ocupa cuatro caracteres después de las dos capas de serialización; sólo el texto ya consume 8 000 caracteres, antes de añadir metadatos y estructura. La página se rechaza y no se ofrece un tamaño menor ni un cursor alternativo que permita recuperarla.

**Consecuencia:** existe una insuficiencia de paginación para el conjunto de textos admitidos. El error es explícito; no se afirma truncamiento silencioso. La reconstrucción conservada de las 25 páginas del PDQ sigue siendo válida para ese documento. La reparación deberá calcular la cota sobre la respuesta efectiva o restringir explícitamente el contrato; requiere una versión posterior y un contraejemplo ejecutado, no una alteración de esta copia.

### MCP-REV01-04 · La validación estricta no conserva claves repetidas

`main.rs` interpreta cada trama como `serde_json::Value` antes de aplicar `deny_unknown_fields`. En `serde_json` 1.0.149, `ValueVisitor::visit_map` utiliza `values.insert`; una clave posterior sustituye la anterior. Por tanto, la validación posterior ya no puede detectar ambas apariciones.

Contraejemplo estático: un objeto de argumentos que contenga dos miembros `limite`, primero 6 y después 5, llega al validador con el segundo valor. La cota final continúa impuesta, pero la entrada ambigua no se rechaza como tal. No se demuestra acceso fuera del catálogo ni ejecución de órdenes.

**Consecuencia:** no puede atribuirse rechazo de claves homónimas al analizador actual. Debe fijarse y comprobarse esa condición antes de afirmar una validación estricta que la incluya. El defecto pertenece a la frontera MCP; no modifica ni invalida los cierres históricos del analizador del Lenguaje SV.

### MCP-REV01-05 · Compatibilidad documental y compatibilidad ejecutada son distintas

Las fuentes fijadas respaldan el transporte por entrada y salida estándar y la configuración de procesos. `McpToolResult` declara `is_error` sin renombrado; la emisión simultánea de `isError` e `is_error` es una adaptación explícita. La prueba propia sólo deserializa una estructura parcial; no ejercita el cliente completo del motor.

Se ha descartado una objeción aparente: aunque `list_tools_via_transport` transmite internamente `Value::Null`, `build_jsonrpc_request` lo normaliza a `{}` antes de enviarlo. Ese punto no constituye una incompatibilidad con el servidor examinado. En cambio, `initialize_transport` no acredita por sí mismo la validación de la versión devuelta: obtiene el resultado y continúa con la notificación.

**Consecuencia:** siguen pendientes identidad, ayuda, opciones y características de compilación del ejecutable de OneCloud, así como el intercambio con el cliente real sin inferencia. La disponibilidad del proceso HTTP no acredita disponibilidad MCP. La utilización efectiva de las herramientas por Qwen continúa pendiente y fuera de esta entrega.

### MCP-REV01-06 · Contención, procedencia y observación conservan límites

La apertura del catálogo es de lectura y las herramientas sólo reciben identificadores; el proceso documental no implementa órdenes del sistema ni acceso de red. Esto no impone aislamiento del sistema operativo. La configuración de `ProcessTransport` tampoco elimina por sí sola el entorno heredado ni reduce los privilegios. Faltan evidencias del usuario efectivo, permisos de archivos, acceso restringido a secretos y núcleo, y límite conjunto de memoria en OneCloud.

El diario registra una respuesta preparada antes de escribirla en la tubería. Su campo `duracion_us` no incluye toda la sincronización en disco ni la entrega al consumidor. No demuestra incorporación a la conversación. Las trazas son JSONL propias; no son instrumentación OpenTelemetry ni observación completa del proceso.

El HTML, el catálogo y sus huellas están conservados. La identidad entre las páginas reconstruidas y el texto extraído no acredita por sí sola la exhaustividad de la extracción respecto del HTML. La fecha de recuperación se declara a partir de la finalización del archivo; no existe en el paquete una captura completa de cabeceras y redirecciones que permita reconstruir toda la adquisición. El importador administrativo no tiene red; no implementa un descargador con control de redirecciones para fuentes futuras.

**Consecuencia:** no se completan esas garantías mediante una declaración documental. Deben conservarse como pendientes y medirse en el entorno receptor dentro del alcance permitido. Las huellas acreditan identidad e integridad, no veracidad clínica ni suficiencia de una cita.

## 4. Evidencia positiva que se conserva

- Rust 1.98.0 identificado en el registro original; fuentes y dependencias fijadas en Cargo.lock.
- Diez pruebas automatizadas correctas según el registro conservado.
- Cliente técnico propio por proceso: 32 peticiones y reconstrucción exacta de 25 páginas del catálogo oficial; 35 registros de diario, incluyendo inicio, fin y notificación.
- Dos herramientas enumeradas; rechazo probado de identificadores fuera del catálogo, recorridos de directorios y varios argumentos inválidos.
- Original documental y cinco secciones conservados; catálogo separado de rúbricas y respuestas del banco.
- Ausencia de carga de pesos e inferencias en las evidencias de la preparación.

Estas observaciones no se suman como pruebas independientes de compatibilidad, seguridad o fidelidad clínica. Ninguna ejecución nueva se atribuye a esta revisión.

## 5. Decisión y siguiente comprobación

La entrega puede recibirse como **prototipo identificado con evidencia local y reparos explícitos**. No está acreditada como preparación técnica conforme. Las [precisiones complementarias de recepción](RECEPCION_PRECISIONES_01.md) concretan la conservación y la parada que deben acompañar al archivo original. Los casos de vencimiento, pérdida de salida, ausencia de fuente, serialización expansiva y claves duplicadas deberán tratarse separadamente del catálogo oficial.

La copia conserva la versión 0.1.0 porque no se ha corregido el programa. No se publican nuevas cifras de rendimiento, consumo o coste: no ha habido recepción remota en esta revisión. El último antecedente remoto documenta la instancia 864618 conservada, contratada y con el motor detenido; esta revisión no verifica de nuevo su estado ni modifica su contratación. La permanencia sigue generando coste según el régimen contratado.

**Parada:** entregar para revisión. No cargar pesos, iniciar inferencias, repetir HCL-02, habilitar una conexión al núcleo ni aplicar la plantilla MCP al motor.

## Referencias de comprobación

- [Sucesos, corte examinado][sucesos], [índice de TT][tt] y [Acta 004][acta004].
- [Código del servicio examinado][codigo] y [guion original de recepción][guion].
- Motor exacto: [cliente][cliente], [transporte][transporte], [tipos][tipos] y [configuración][config].
- `serde_json` 1.0.149: `src/value/de.rs`, `ValueVisitor::visit_map`, dependencia fijada en Cargo.lock. La inspección corresponde a la copia conservada de esa dependencia, no a una versión posterior.

[origen]: https://github.com/juantoniolloretegea/SV-sala-de-maquinas/tree/24e6204b3fc7c082a04a6dc785f9491c7ce8e238/respuestas-ejecucion/MCP-DOCUMENTAL-PREPARACION-20260926/entrega-01
[hcl]: https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/ffed6b40339c4ac19c8da18a1ceaf75c1ce78578/respuestas-ejecucion/QWEN38-HCL01-DIAGNOSTICO-20260926/continuacion-directa-20260926/INFORME.md
[sucesos]: https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/b1d826981a329f0d3e1295e8989268ca349acf0f/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.csv
[tt]: https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/b1d826981a329f0d3e1295e8989268ca349acf0f/docs/calidad/Inventario-sv/tiques-tecnicos/TIQUES_TECNICOS.csv
[acta004]: https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/b1d826981a329f0d3e1295e8989268ca349acf0f/docs/calidad/tuberias-ia/continuacion-15-09-2026/ACTA_004_FINALIDAD_ALCANCE_Y_CONTINUIDAD_EIO_2026_09_22.md
[codigo]: https://github.com/juantoniolloretegea/SV-sala-de-maquinas/tree/24e6204b3fc7c082a04a6dc785f9491c7ce8e238/respuestas-ejecucion/MCP-DOCUMENTAL-PREPARACION-20260926/entrega-01/sv-mcp-documental-0.1.0/src
[guion]: https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/24e6204b3fc7c082a04a6dc785f9491c7ce8e238/respuestas-ejecucion/MCP-DOCUMENTAL-PREPARACION-20260926/entrega-01/sv-mcp-documental-0.1.0/VERIFICAR.sh
[cliente]: https://github.com/EricLBuehler/mistral.rs/blob/2370966bb91e2e3dafa0b1521b87c50fd5c01244/mistralrs-mcp/src/client.rs
[transporte]: https://github.com/EricLBuehler/mistral.rs/blob/2370966bb91e2e3dafa0b1521b87c50fd5c01244/mistralrs-mcp/src/transport.rs
[tipos]: https://github.com/EricLBuehler/mistral.rs/blob/2370966bb91e2e3dafa0b1521b87c50fd5c01244/mistralrs-mcp/src/types.rs
[config]: https://github.com/EricLBuehler/mistral.rs/blob/2370966bb91e2e3dafa0b1521b87c50fd5c01244/mistralrs-mcp/src/lib.rs
