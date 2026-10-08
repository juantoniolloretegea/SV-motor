# Cotejo de las dos ejecuciones MD01

**Versión 1.1 · 08/10/2026.** El cotejo reconstruye las condiciones documentadas de Astra y Grok. No ejecuta una nueva prueba.

## Identidad del contenido

Las siguientes huellas coinciden en el [control de Astra](https://github.com/juantoniolloretegea/SV-motor/blob/0246a7df1e9436f4bff4f0d94ac6d36e1f29c1e0/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/replica-md01-20261008/suministro/CONTROL-ARBITRO.json) y la [preparación de Grok](https://github.com/juantoniolloretegea/SV-motor/blob/091b50a884c893fd4322bbf211b0feb0132bb258/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/xai/grok-4.7/md01-diagnostica-20261008/IDENTIDAD-PREPARACION.json):

| Objeto | SHA-256 |
|---|---|
| Catálogo | `96944f3142ddd4e153a5efa4099944e46a121494bdb2c171deb8cea19aa013c9` |
| Fuentes | `0d1628111f672ba7a6512b0c49ec615f53614a2509f434248b02528b19d0daa6` |
| Banco de preguntas | `0475ec0901a32414a59993871c2e6aed955404bf417affa20994a59113e7625b` |
| Fuente y solicitud base MD01 | `6cab1c5102ca7659b50d3b5cb9cf071a52b0a1375d6fe4318506ced490ff1445` |

Los contratos temporales contienen la misma regla sustantiva y composición de antecedentes. La ruta de inclusión del contrato base es distinta. El [informe de Astra](https://github.com/juantoniolloretegea/SV-motor/blob/0246a7df1e9436f4bff4f0d94ac6d36e1f29c1e0/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/replica-md01-20261008/INFORME.md) y el [informe de Grok](https://github.com/juantoniolloretegea/SV-motor/blob/091b50a884c893fd4322bbf211b0feb0132bb258/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/xai/grok-4.7/md01-diagnostica-20261008/INFORME.md) documentan la pregunta, los resultados y sus límites. La custodia íntegra de las respuestas originales no es pública en todos los ensayos; el cotejo cuantitativo aquí publicado no equivale a reproducir toda la adjudicación.

## Configuración y suministro

| Campo | Astra | Grok |
|---|---|---|
| Razonamiento solicitado | medium; resumen auto | medium; campo de resumen omitido |
| Máximo de salida solicitado | 8192 | 8192; reserva de semántica respecto del razonamiento |
| Recepción continua | Sí | Sí |
| Herramientas disponibles en la solicitud | Ninguna; selección none | Ninguna; campo de selección omitido |
| Almacenamiento solicitado | No | No; retención cero comunicada por el proveedor |
| Instrucciones | Contrato del ensayo | Contrato más licencia y aviso |

Solicitar medium en dos proveedores no acredita igualdad de cómputo interno. Las solicitudes completas no son idénticas y contienen cantidades diferentes de tokens. Se examinaron la [construcción base](https://github.com/juantoniolloretegea/SV-motor/blob/0246a7df1e9436f4bff4f0d94ac6d36e1f29c1e0/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/replica-md01-20261008/controlador/src/bin/astra-md01-replica/suministro.rs#L82) y la [adaptación para xAI](https://github.com/juantoniolloretegea/SV-motor/blob/091b50a884c893fd4322bbf211b0feb0132bb258/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/cliente-api-rust/src/lib.rs#L25).

## Definición y origen de la medida temporal

Ambos instrumentos registran la primera aparición de un evento `response.output_text.delta`, sin exigir que su fragmento de texto sea no vacío. Por ello, se publica **demora hasta el primer evento de texto registrada por el cliente**, no tiempo acreditado hasta el primer carácter visible.

En Astra el reloj comienza después de construir el cliente HTTP y guardar la constancia de envío. En Grok comienza antes de ambas operaciones. Los dos instrumentos escriben y sincronizan el bloque recibido antes de analizar sus eventos. La medida incluye actividad local y comunicación, y no es una medida aislada del servidor.

El cotejo del [instrumento de Astra](https://github.com/juantoniolloretegea/SV-motor/blob/0246a7df1e9436f4bff4f0d94ac6d36e1f29c1e0/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/replica-md01-20261008/controlador/src/bin/astra-md01-replica/manual.rs#L123) y el [de Grok](https://github.com/juantoniolloretegea/SV-motor/blob/091b50a884c893fd4322bbf211b0feb0132bb258/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/cliente-api-rust/src/lib.rs#L77) no proporciona una cota cuantificada para descontar retrospectivamente esa diferencia. **No se ha aplicado una corrección inventada ni se atribuye toda la diferencia al modelo.**

## Valores conservados y límite comparativo

| Modelo | Etapa | Primer evento de texto, ms | Entrada | Salida | Total |
|---|---|---:|---:|---:|---:|
| Astra | R0 | 5.579 | 18.348 | 1.028 | 19.376 |
| Grok | R0 | 64.239 | 21.546 | 7.779 | 29.325 |
| Astra | R1 | 3.519 | 20.165 | 1.645 | 21.810 |
| Grok | R1 | 117.640 | 25.217 | 12.247 | 37.464 |
| Astra | R2 | 3.621 | 22.597 | 1.762 | 24.359 |

Fuentes cuantitativas: [Astra](https://github.com/juantoniolloretegea/SV-motor/blob/0246a7df1e9436f4bff4f0d94ac6d36e1f29c1e0/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/openai/gpt-6-astra/manual-mdbook-20261008/replica-md01-20261008/METRICAS-RUST.json), [Grok R0](https://github.com/juantoniolloretegea/SV-motor/blob/091b50a884c893fd4322bbf211b0feb0132bb258/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/xai/grok-4.7/md01-diagnostica-20261008/mediciones/MD01-R0.json) y [Grok R1](https://github.com/juantoniolloretegea/SV-motor/blob/091b50a884c893fd4322bbf211b0feb0132bb258/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/acoplamientos-con-el-sv/03-api-directa/xai/grok-4.7/md01-diagnostica-20261008/mediciones/MD01-R1.json).

Se publican las dos observaciones R0 **sin asignar puestos**. R0 tiene una pregunta y corpus comunes, pero las reservas instrumentales anteriores impiden elevarlo a una comparación controlada. R1 y R2 incorporan respuestas previas diferentes y no se promedian con R0 para obtener un supuesto ganador global. R2 de Grok no se envió.

La selección de este subconjunto es retrospectiva. No fue una clasificación previamente definida ni dispone de repeticiones que permitan estimar variabilidad. Las diferencias observadas no permiten generalizar superioridad, ahorro o latencia esperada.

[Datos propios](DATOS-SV.json) · [Observaciones MD01](OBSERVACIONES-MD01.json) · [Resultados por ensayo](RESULTADOS-SV.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
