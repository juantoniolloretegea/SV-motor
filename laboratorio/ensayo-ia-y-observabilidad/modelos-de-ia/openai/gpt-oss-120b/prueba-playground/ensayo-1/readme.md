# Ensayo 1: revisión adversarial de una respuesta documental

**GPTOSS-PLAYGROUND-E1-20261003 · 3 de octubre de 2026. Estado: preparado; pendiente de ejecución.**

Se examina si una relectura obligatoria del pasaje suministrado, seguida de una crítica de las respuestas anteriores A y B, permite elaborar una respuesta documentalmente fiel a la misma pregunta. Se utiliza la [demostración pública de GPT-OSS](https://gpt-oss.com/), con selección visible gpt-oss-120b y razonamiento High.

El recorrido consta de recepción documental en JSON, cotejo externo en Rust y, sólo si éste resulta conforme, revisión adversarial y respuesta final. Las huellas SHA-256 acreditan identidad de los textos; no constituyen una prueba de lectura interna o comprensión. La justificación de cada afirmación se evalúa contra la fuente.

| Documento | Objeto |
|---|---|
| [Protocolo](PROTOCOLO.md) | Hipótesis, condiciones, límites, criterios y puntuación. |
| [Primera entrada](ENTRADA-01.txt) | Petición de recepción documental antes de responder. |
| [Segunda entrada condicionada](ENTRADA-02-CONDICIONAL.txt) | Pregunta original, fuente y respuestas A/B para su revisión. |
| [Fuente segmentada](FUENTE-SEGMENTADA.json) | Texto con identificadores para el cotejo. |
| [Identidad de la fuente](IDENTIDAD-FUENTE.json) | Procedencia, huellas y regla de segmentación. |
| [Manifiesto](MANIFIESTO.json) | Identidad de los archivos preparados. |

Los resultados de la exploración anterior permanecen intactos. Este ensayo se realiza sobre una pregunta conocida y no acredita por sí mismo generalización ni aptitud clínica. No se incorporan datos de cuentas o de infraestructura.

La [autoría y licencia de la carpeta principal](../readme.md#autoría-licencia-y-alcance) se aplican al trabajo propio de este ensayo; los materiales de terceros conservan sus derechos.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
