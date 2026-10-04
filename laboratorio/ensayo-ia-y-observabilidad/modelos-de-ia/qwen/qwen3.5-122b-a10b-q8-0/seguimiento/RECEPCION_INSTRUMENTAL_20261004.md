# Recepción instrumental de Qwen3.5-122B-A10B Q8_0

**4 de octubre de 2026. Instalación y primera respuesta acreditadas; evaluación documental y examen pendientes.**

El candidato ha completado la carga y una única solicitud instrumental. La salida fue exactamente **LISTO**, íntegra y con cierre normal.

| Magnitud observada | Resultado |
|---|---:|
| Activación hasta disponibilidad observada | 327,368 s |
| Primera salida de la solicitud mínima | 39,407 s |
| Duración completa de la solicitud | 48,459 s |
| Tokens de entrada / salida informados | 21 / 3 |
| Velocidad de generación informada | 0,332 tokens/s |
| Memoria máxima del grupo, incluida caché de archivos | 225.506.820.096 bytes |
| Agotamientos de memoria / intercambio | 0 / 0 bytes |

La solicitud se realizó sin pensamiento, con temperatura cero y máximo 16 tokens. Esta primera medida corta no permite extrapolar velocidad a documentos largos. **La latencia es una reserva relevante para la viabilidad de la campaña.** El sondeo de disponibilidad introduce un intervalo de observación. La memoria indicada incluye caché de archivos, no sólo los pesos; hubo 1503 eventos sobre el límite suave, sin alcanzar el límite duro ni registrar OOM.

## Realización identificada

Los cuatro fragmentos Q8_0 y el componente visual se cotejaron mediante Rust con la revisión de distribución 51eab4d59d53f573fb9206cb3ce613f1d0aa392b. La referencia oficial es dc4d348443bc740c68e2d77492492c11606384d5.

El motor mistral.rs 0.9.4 parte de 4400935451da5e2dc7379a3f92fbbada66557f6c, con corrección local de selección y descompresión de expertos en CPU Q8_0. La variante r2 utilizada superó once contrastes y una referencia escalar independiente. La variante instrumental previa que falló permanece conservada. Estos contrastes no acreditan equivalencia numérica universal ni aptitud documental.

Condiciones: 48 CPU AMD EPYC 7542, RAM efectiva cercana a 251,65 GiB, sin GPU, activaciones F32, caché KV F16 y una secuencia. El máximo configurado es 32768 tokens, sin validación de una entrada de esa longitud. La fragmentación de entrada de 32 no está acreditada en CPU. El servicio sólo escucha localmente y su grupo de control rechazó la conexión exterior; el control separado previo sí alcanzó el destino.

## Tokenizador e incidencias

La primera carga se detuvo antes de inferir: 248070 entradas en el tokenizador oficial frente a 248320 en el GGUF. El cotejo Rust confirmó identidad de todos los identificadores compartidos y de las reglas de combinación. Las 250 entradas adicionales incluyen entradas de audio; no son exclusivamente relleno.

Se seleccionó la ruta nativa del tokenizador incorporado al GGUF, sin alterar pesos ni original oficial. El motor descarta por ello la configuración externa de generación; EOS GGUF es 248046. La recepción mínima conservó parámetros explícitos. No se presume igualdad con otra representación o servicio API.

Los fallos instrumentales y sus originales se conservan por separado, incluidas la interrupción previa de servicios durante actualizaciones y la espera de conexión SSH corregida. No se atribuyen al razonamiento del modelo.

## MCP PDF y límites pendientes

MCP 0.1.4-pdf.1 está instalado con revisión 3f12e5054523f313ce148d37f0bdaee16bcff98a: 69 fuentes cotejadas y 36 comprobaciones conformes, con 10 páginas y 30 fragmentos. Se verificaron extracción, localizadores, transporte, integridad, límites y aislamiento.

**Esto no acredita comprensión del PDF por Qwen.** Quedan pendientes el corpus concreto, su recorrido integrado, la entrada efectiva y la viabilidad de memoria y tiempos para los casos previstos. Antes de la campaña deberán fijarse política, banco, criticidad, revisiones y cotas. La evaluación y adjudicación son externas al candidato.

No se han modificado el Núcleo, la semántica V0.2 ni la IR 0.3. No se ha iniciado el examen ni se ha acreditado aptitud. Los antecedentes y comparativas conservan su alcance.

Documentación del Sistema Vectorial SV. Los componentes de terceros conservan sus licencias de origen.
[Datos estructurados y huella del motor](REALIZACION_20261004.json) · [Criterios de recepción](../evaluacion/CRITERIOS_DE_RECEPCION.md) · [Estado](ESTADO.json) · [Expediente](../readme.md).