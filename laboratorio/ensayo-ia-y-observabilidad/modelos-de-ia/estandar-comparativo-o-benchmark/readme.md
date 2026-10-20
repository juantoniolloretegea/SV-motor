# Estándar comparativo de modelos de inteligencia artificial

**Ciberseguridad Inteligente · Nodo 3 · Edición de 20 de octubre de 2026**

Esta comparación examina la respuesta de servicios de IA externos a dieciséis preguntas sobre documentos identificados. Publica el examen, las soluciones, sus reglas y la procedencia de la evidencia para permitir una revisión reproducible.

**Estado: edición fijada, sin exámenes ejecutados ni dictámenes.** La lista de candidatos indica el orden alfabético de examen; no es una clasificación por mérito. Las comprobaciones de conexión o del instrumento no son resultados de candidatos.

## 1. Examen y soluciones

| Consulta | Contenido |
| --- | --- |
| [Preguntas e instrucciones](ediciones/ciberseguridad/eet-cyb16-multiformato-20261020-v1/PREGUNTAS.md) | Dieciséis enunciados, reglas de respuesta y correspondencia documental. |
| [Soluciones y criticidad](ediciones/ciberseguridad/eet-cyb16-multiformato-20261020-v1/SOLUCIONES.md) | Clave y fundamento de cada pregunta; regla de corrección. |
| [Definición pública del examen](ediciones/ciberseguridad/eet-cyb16-multiformato-20261020-v1/EXAMEN.json) | Preguntas, instrucciones, fuentes, formatos y huellas en JSON. |
| [Plantilla correctora](ediciones/ciberseguridad/eet-cyb16-multiformato-20261020-v1/plantilla-correctora.json) | Claves del corrector, separadas de la entrega al candidato. |
| [Identidad de la edición](EDICION.json) | Edición fijada, tiempos, regla, cruce de formatos y referencias. |
| [Manifiesto de publicación](MANIFIESTO.json) | Tamaño y SHA-256 de los archivos publicados. |

Las soluciones son públicas para la revisión del examen. El Árbitro–Director entrega al candidato únicamente los documentos y las preguntas autorizados; no entrega la plantilla ni permite consultar Internet durante el examen. La publicación previa no permite descartar por sí sola que un servicio conociera ya estos materiales: ese límite debe acompañar la interpretación de los resultados.

<details>
<summary><strong>Originales conservados: fuentes y cuadernillos</strong></summary>

| Función | PDF | HTML | Markdown |
| --- | --- | --- | --- |
| Fuente de información | [NIST SP 800-207](ediciones/ciberseguridad/eet-cyb16-multiformato-20261020-v1/fuentes/d1.pdf) | [RFC 8996](ediciones/ciberseguridad/eet-cyb16-multiformato-20261020-v1/fuentes/d2.html) | [Acta OP-CYB-001](ediciones/ciberseguridad/eet-cyb16-multiformato-20261020-v1/fuentes/d3.md) |
| Cuadernillo de preguntas | [Seis preguntas](ediciones/ciberseguridad/eet-cyb16-multiformato-20261020-v1/cuadernillos/cuadernillo-pdf.pdf) | [Cinco preguntas](ediciones/ciberseguridad/eet-cyb16-multiformato-20261020-v1/cuadernillos/cuadernillo-html.html) | [Cinco preguntas](ediciones/ciberseguridad/eet-cyb16-multiformato-20261020-v1/cuadernillos/cuadernillo-md.md) |

Son copias íntegras de los archivos fijados, sin conversión. Las fuentes conservan sus condiciones de uso, identificadas en [Procedencia y licencias](PROCEDENCIA-Y-LICENCIAS.md). La vista previa de GitHub no sustituye la descarga del archivo original.

</details>

## 2. Dos dimensiones de formato

El formato de la **fuente de información** y el formato del **cuadernillo de preguntas** son independientes. Seis preguntas utilizan fuentes PDF, cinco HTML y cinco Markdown. Los cuadernillos contienen igualmente seis, cinco y cinco preguntas, con las nueve combinaciones representadas.

![Distribución de las dieciséis preguntas entre formatos de fuente y cuadernillo](CRUCE-FORMATOS.svg)

| Fuente ↓ / Cuadernillo → | PDF | HTML | Markdown | Total |
| --- | ---: | ---: | ---: | ---: |
| PDF | 2 | 2 | 2 | 6 |
| HTML | 2 | 2 | 1 | 5 |
| Markdown | 2 | 1 | 2 | 5 |
| **Total** | **6** | **5** | **5** | **16** |

Esta distribución describe la cobertura. No aísla estadísticamente el efecto del formato: las fuentes también difieren en materia e idioma. Las preguntas vinculadas a una figura pueden apoyarse en su texto contiguo; no constituyen una prueba exclusiva de percepción visual.

## 3. Candidatos y orden de examen

| Orden | Modelo | Proveedor | Estado de esta edición |
| ---: | --- | --- | --- |
| 1 | Claude Opus 5.5 | Anthropic | Pendiente; sin examen. |
| 2 | DeepSeek-V4.1-Flash-1017 | DeepSeek | Compatibilidad documental pendiente; sin dictamen. |
| 3 | Gemini 4 Argon | Google | Pendiente; sin examen. |
| 4 | GLM-5.3 | Z.ai | Compatibilidad documental pendiente; sin dictamen. |
| 5 | GPT-6 Astra | OpenAI | Pendiente; sin examen. |
| 6 | Grok 4.7 | xAI | Recorrido documental pendiente; sin dictamen. |
| 7 | Kimi K3 | Kimi | Recorrido documental pendiente; sin dictamen. |
| 8 | Qwen3.8-Max-0902 | Qwen | Pendiente; sin examen. |

[Lista estructurada de candidatos](CANDIDATOS.csv). La denominación identifica la selección; cada intento deberá acreditar el identificador efectivo del servicio, su configuración y las condiciones recibidas. Los antecedentes no transfieren un dictamen a esta edición.

## 4. Resultados y evidencias

La [tabla comparativa](COMPARACION.csv) y su [representación JSON](COMPARACION.json) todavía no contienen resultados. No se dibujan barras, posiciones ni polígonos de candidatos sin datos efectivos.

Cada dictamen emitido se representará exclusivamente mediante **0 — Apto**, **1 — No apto** o **U — Indeterminado**. La ausencia de dictamen se conserva vacía o nula, con su estado en otra columna. Un examen no ejecutado o detenido por un fallo instrumental no equivale a U.

El [método comparativo](METODO.md) establece corrección, tiempos, ordenación y límites. El [contrato de evidencias públicas](EVIDENCIAS.md) define los datos necesarios para relacionar respuestas, mediciones, gráficos y polígono con un intento identificado. La copia pública debe ser idéntica al conjunto expresamente seleccionado para publicación dentro del archivo custodiado.

## 5. Alcance

El resultado describirá esta edición, esta configuración y este intento. No acredita competencia general en ciberseguridad, aptitud clínica ni capacidad para recorrer un dominio completo. La recomendación del SV, la autorización institucional y la selección de un modelo para producción son actos distintos.

La [clasificación del Gestor](https://administracion.itvia.online/benchmark) y el [libro de evaluación](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/) ofrecen las otras vías de consulta. La publicación documental no inicia exámenes ni habilita producción.

---

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
