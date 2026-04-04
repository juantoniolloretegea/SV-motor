# Declaración canónica de actores de modelo del ecosistema SV

**Autor:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**ITVIA — IA eñ™ | ISSN: 2695-6411**  
**Licencia:** CC BY-NC-ND 4.0  
**Fecha:** 4 de abril de 2026 | Motor v0.1.8  
**Sede:** SV-motor-main/docs/arquitectura/

**Fecha y Versión: V.1 del conjunto**  
**Versión del conjunto:** V.1 del conjunto  
**Titularidad y autoría:** © Juan Antonio Lloret Egea, 2026.

---

## Posición doctrinal

Este documento formaliza la declaración de actores de modelo para que cualquier implementador pueda derivar, reproducir o auditar la pila de modelos del ecosistema SV sin ambigüedad. Todo lo que aquí se declara está subordinado al compilador doctrinal (Documento 8 del corpus) y a los Fundamentos algebraico-semánticos del SV. Los antecedentes donde el análisis produce una elección distinta a la originalmente implementada se declaran explícitamente para preservar la trazabilidad del cambio.

---

## Antecedentes declarados

**Antecedente 1 — Extractor NLP: de claude-sonnet-4-6 a claude-haiku-4-5**

El motor (v0.1.7) declara `claude-sonnet-4-6` como modelo Anthropic por defecto. Esta elección se hizo durante la fase de definición de la tarea, cuando un modelo de mayor capacidad daba confianza sobre la corrección de la extracción. Un análisis de la tarea concreta muestra que el extractor recibe texto en español, opera sobre un dominio cerrado de 9 posiciones con vocabularios finitos por posición, y debe devolver JSON estrictamente estructurado. Esta es una tarea de seguimiento de instrucciones con salida canónica finita. `claude-haiku-4-5-20251001` es suficiente, más rápido y de menor coste. Sonnet se mantiene disponible como `mode="anthropic_audit"` para validación adversarial del extractor.

**Antecedente 2 — Backbone CNN: de ResNet34 a ConvNeXt-Tiny**

ResNet34 fue la línea base válida de los Documentos 2–7 del corpus, con error rate < 0,5% en todas las configuraciones evaluadas. El Documento 8 establece ConvNeXt-Tiny como referencia desde ese documento en adelante, con justificación en Liu et al. (2022): menor varianza entre ejecuciones (crítico en clínica) y superioridad en fine-tuning con datos limitados. Los yamls de todos los módulos se actualizan preservando `historical_baseline: "resnet34"` para trazabilidad.

**Antecedente 3 — Modelo Ollama: de qwen2.5:7b a qwen3:4b**

Qwen 2.5 7B estaba disponible cuando se diseñó la capa NLP. Un análisis de la tarea mostró que 7B parámetros son sobredimensionados para extracción de observables de vocabulario cerrado. Qwen 3 4B (Apache 2.0, lanzado abril 2025) se propone aquí como opción preferente con menor consumo de memoria (~2,5 GB vs ~4 GB cuantizado). Si Qwen 3 4B no está disponible en la instalación de Ollama, `qwen2.5:7b` es válido como fallback documentado.

**Nota de precisión sobre la comparación Qwen.** La preferencia por Qwen 3 4B debe leerse como decisión de diseño razonada y no como benchmark universal cerrado mientras no exista una batería pública del SV con el mismo conjunto de casos, mismas semillas y mismo entorno de inferencia.

---

## Tabla de actores canónicos

| ID | Modelo | Licencia | Contexto | Tarea | Output |
|---|---|---|---|---|---|
| ACTOR-NLP-LOCAL | Qwen 3 4B / Ollama | Apache 2.0 | Sensible / clínico | Extracción Ω_NLP | dict[str,str] |
| ACTOR-NLP-API-LIGHT | claude-haiku-4-5-20251001 | Commercial Terms | No sensible / dev | Extracción Ω_NLP | dict[str,str] |
| ACTOR-NLP-API-HF | Qwen/Qwen3-4B-Instruct / HF API | Apache 2.0 | CI/dev | Extracción Ω_NLP | dict[str,str] |
| ACTOR-NLP-AUDIT | claude-sonnet-4-6 | Commercial Terms | Auditoría del extractor | Validación adversarial | dict[str,str] |
| ACTOR-CNN-A | ConvNeXt-Tiny / torchvision | BSD-3 | SVcustos/SVperitus | Clasificación polígono | K₃ auxiliar |
| ACTOR-CNN-B | ConvNeXt-Tiny (cabeza posicional) | BSD-3 | Hito 5 (no activo) | Features posicionales | {pos: Σ}ⁿ |
| ACTOR-TRANSLATE | Qwen 3 8B o claude-haiku-4-5 | Apache 2.0 / Commercial | Presentación | Traducción JSON→texto clínico | texto del dominio |
| ACTOR-ALG | motor algebraico Python | MIT (sv-motor) | Siempre | κ₃, U_irr, política | SVProgramResult |
| ACTOR-RUST | backend Rust/WASM | MIT | SVperitus / cotejo | Doble vara algebraica | JSON canónico |

---

**Nota de precisión sobre Anthropic.** Los términos comerciales y de retención deben leerse con precisión contractual: la documentación pública comercial describe retención estándar de 30 días salvo modalidades especiales como Zero Data Retention, y la eventual disponibilidad de BAA depende del servicio elegible y del contrato aplicable.

## Cadena de autoridad (invariante del sistema)

```
1. Corpus doctrinal publicado (Fundamentos + Docs I–VI)
2. Motor algebraico — ACTOR-ALG
3. YAML de configuración del módulo (dominio declarado)
4. Cotejo Rust — ACTOR-RUST
5. CNN — ACTOR-CNN-A / ACTOR-CNN-B
6. Extractor NLP — ACTOR-NLP-*
7. Presentación — ACTOR-TRANSLATE
```

Los actores estadísticos ocupan los últimos lugares. Nunca tienen autoridad sobre la clasificación algebraica. La inversión de este orden es el mecanismo de fallo documentado en IBM Watson for Oncology [Schmidt, JNCI 2017; Strickland, IEEE Spectrum 2019].

---

## Vector de entrada: bloque deployment en SVP

```yaml
sv_model_stack:
  sv_version: "0.1.8"
  module: "IMMUNO-1"
  n: 25
  deployment_context: "clinical_local"

  nlp_extractor:
    active: false
    actor_id: "ACTOR-NLP-LOCAL"
    model: "qwen3:4b"
    backend: "ollama"
    fallback_model: "qwen2.5:7b"
    privacy_class: "sensitive"
    feeds: "algebra.nlp.i_nlp()"

  cnn_polygon:
    active: true
    actor_id: "ACTOR-CNN-A"
    model: "convnext_tiny"
    historical_baseline: "resnet34"
    image_size: 224
    head: "nn.Linear(in_features, 3)"
    feeds: "señal auxiliar — no sobrescribe motor normativo"

  cnn_features:
    active: false
    actor_id: "ACTOR-CNN-B"
    model: "convnext_tiny"
    head: "cabeza posicional — n salidas"
    status: "no implementado — DV-HITO5-003, DV-HITO5-004"

  algebra:
    actor_id: "ACTOR-ALG"
    model: null
    active: true

  presentation_rule: "positions_1_first"
```

---

## Deudas derivadas de esta declaración

| ID | Descripción |
|---|---|
| DV-SVM-013 | Actualizar yamls resnet34 → convnext_tiny |
| DV-SVM-014 | Modelo Anthropic por defecto: haiku |
| DV-SVM-015 | train_convnext.py en SVcustos |
| DV-SVM-016 | Lectura deployment_profile en py_runner |
| DV-SVM-017 | Clase CaptureResult con admisibilidad (FRONTERA B.6) |

---

*Juan Antonio Lloret Egea | ORCID: 0000-0002-6634-3351 | ITVIA — IA eñ™ | ISSN: 2695-6411 | CC BY-NC-ND 4.0*
