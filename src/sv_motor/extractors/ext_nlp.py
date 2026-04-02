"""
sv_motor.extractors.ext_nlp
============================
Extractor de observables Ω_NLP desde texto en lenguaje natural.

MODO A — Determinista (sin ML):
  El humano soberano provee directamente los observables como JSON.
  Cero estadística. Máxima precisión.

MODO B — Puente ML (bridge):
  Un LLM local (Ollama) o remoto (HuggingFace Inference API) extrae
  los observables del texto bajo FT-SV-IA/001 estricto.
  El modelo NO decide κ₃. Solo extrae Ω_NLP.
  La evaluación algebraica sigue siendo código determinista.

MODO C — Anthropic API (para entornos donde está disponible):
  Usa la API de Anthropic bajo el mismo contrato FT-SV-IA/001.

El resultado de cualquier modo es un dict validado contra el dominio
declarado en Documento 2, §2.3. Si algún valor está fuera del dominio,
la posición se emite como U_d(B) antes de llegar a I_NLP.
"""
from __future__ import annotations

import json
from typing import Any, Dict, Optional

# Dominio declarado (Documento 2, §2.3)
_DOMAIN: Dict[str, set] = {
    "theta": {"coherente", "desvio", "indeterminado"},
    "pi":    {"sin-pregunta", "resuelta", "bloqueada", "indeterminada"},
    "kappa": {"coherente", "contradictoria", "indeterminada"},
    "eta":   {"completa", "defectuosa", "indeterminada"},
    "gamma": {"alineada", "bloqueada", "indeterminada"},
    "alpha": {"apropiada", "inapropiada", "indeterminada"},
    "mu":    {"sin-ambiguedad", "cerrada", "incompatible", "indeterminada"},
    "chi":   {"sin-solicitud", "atendida", "denegada", "indeterminada"},
    "psi":   {"en-curso", "cerrado", "bloqueado", "indeterminado"},
}

_NORMALIZE = {"desvío": "desvio", "sin-ambigüedad": "sin-ambiguedad"}

# Prompt base bajo FT-SV-IA/001
_SYSTEM_PROMPT = """Eres un extractor de observables para el Sistema Vectorial SV.

Tu única tarea es: dado un texto en español, extraer los nueve observables
del paquete Ω_NLP y devolverlos como JSON. No evalúas, no clasificas, no opinas.

Reglas absolutas:
1. Devuelves SOLO el JSON. Sin texto antes ni después.
2. Cada valor debe estar en el dominio declarado para su posición.
3. Si no puedes determinar un valor con certeza, usa "indeterminado" (o la variante aplicable para esa posición).
4. NO fabricas certeza donde no la hay.

Posiciones y dominios:
- theta (P1 coherencia temática): coherente | desvio | indeterminado
- pi    (P2 pregunta resuelta):   sin-pregunta | resuelta | bloqueada | indeterminada
- kappa (P3 sin contradicción):   coherente | contradictoria | indeterminada
- eta   (P4 completitud):         completa | defectuosa | indeterminada
- gamma (P5 filiación objetivo):  alineada | bloqueada | indeterminada
- alpha (P6 tipo de acto):        apropiada | inapropiada | indeterminada
- mu    (P7 ambigüedad):          sin-ambiguedad | cerrada | incompatible | indeterminada
- chi   (P8 clarificación):       sin-solicitud | atendida | denegada | indeterminada
- psi   (P9 estado objetivo):     en-curso | cerrado | bloqueado | indeterminado

Formato de respuesta (SOLO esto, sin ningún otro texto):
{
  "theta": "...",
  "pi": "...",
  "kappa": "...",
  "eta": "...",
  "gamma": "...",
  "alpha": "...",
  "mu": "...",
  "chi": "...",
  "psi": "..."
}"""


def _validate_and_collect(raw: Dict[str, str]) -> tuple[Dict[str, str], list[dict[str, str]]]:
    """
    Valida el dict extraído contra el dominio declarado.
    Valores fuera del dominio → U_d(B) marcado como indeterminación.
    """
    out: Dict[str, str] = {}
    u_d_items: list[dict[str, str]] = []
    for field, domain in _DOMAIN.items():
        raw_original = raw.get(field, "")
        raw_val = _NORMALIZE.get(raw_original, raw_original)
        if raw_val not in domain:
            fallback = "indeterminado" if field == "psi" else "indeterminada"
            out[field] = fallback
            u_d_items.append({
                "codigo": "U_d(B)",
                "campo": field,
                "valor_recibido": str(raw_original),
                "valor_emitido": fallback,
                "causa": "valor fuera del dominio declarado",
            })
        else:
            out[field] = raw_val
    return out, u_d_items


def validate_observables_dict(raw: Dict[str, str]) -> Dict[str, str]:
    """Valida y normaliza un paquete Ω_NLP sin devolver metadatos de protocolo."""
    return _validate_and_collect(raw)[0]


def validate_observables_with_ud(raw: Dict[str, str]) -> tuple[Dict[str, str], list[dict[str, str]]]:
    """Valida y normaliza un paquete Ω_NLP, devolviendo además las U_d(B) activas."""
    return _validate_and_collect(raw)


# ─────────────────────────────────────────────────────────────
# MODO A — sin ML
# ─────────────────────────────────────────────────────────────
def extract_direct(observables_dict: Dict[str, str]) -> Dict[str, str]:
    """
    Modo A: el humano provee los observables directamente.
    Solo normaliza y valida contra el dominio.
    """
    return validate_observables_dict(observables_dict)


# ─────────────────────────────────────────────────────────────
# MODO B — Ollama (modelo local)
# ─────────────────────────────────────────────────────────────
def extract_ollama(
    text: str,
    model: str = "qwen2.5:7b",
    base_url: str = "http://localhost:11434",
) -> Dict[str, str]:
    """
    Modo B: extracción via Ollama local.

    Requiere: pip install sv-motor[bridge] + Ollama instalado con el modelo.
    Ollama: https://ollama.ai
    Modelo recomendado: qwen2.5:7b  (ollama pull qwen2.5:7b)
    """
    try:
        import httpx
    except ImportError:
        raise ImportError("Instala con: pip install sv-motor[bridge]")

    payload = {
        "model": model,
        "messages": [
            {"role": "system", "content": _SYSTEM_PROMPT},
            {"role": "user",   "content": text},
        ],
        "stream": False,
        "format": "json",
    }
    resp = httpx.post(f"{base_url}/api/chat", json=payload, timeout=60)
    resp.raise_for_status()
    content = resp.json()["message"]["content"]
    try:
        raw = json.loads(content)
    except json.JSONDecodeError:
        # Fallback: todo indeterminado
        raw = {}
    return validate_observables_dict(raw)


# ─────────────────────────────────────────────────────────────
# MODO B — HuggingFace Inference API
# ─────────────────────────────────────────────────────────────
def extract_hf_api(
    text: str,
    model: str = "Qwen/Qwen2.5-7B-Instruct",
    hf_token: Optional[str] = None,
) -> Dict[str, str]:
    """
    Modo B: extracción via HuggingFace Inference API.
    Requiere token HF con acceso al modelo.
    """
    try:
        import requests
    except ImportError:
        raise ImportError("Instala con: pip install sv-motor[bridge]")

    headers = {"Content-Type": "application/json"}
    if hf_token:
        headers["Authorization"] = f"Bearer {hf_token}"

    prompt = f"{_SYSTEM_PROMPT}\n\nTexto a analizar:\n{text}"
    payload = {
        "inputs": prompt,
        "parameters": {"max_new_tokens": 256, "return_full_text": False},
    }
    url = f"https://api-inference.huggingface.co/models/{model}"
    resp = requests.post(url, headers=headers, json=payload, timeout=60)
    resp.raise_for_status()
    content = resp.json()[0]["generated_text"]
    # Extraer JSON del output
    try:
        start = content.index("{")
        end   = content.rindex("}") + 1
        raw   = json.loads(content[start:end])
    except (ValueError, json.JSONDecodeError):
        raw = {}
    return validate_observables_dict(raw)


# ─────────────────────────────────────────────────────────────
# MODO C — Anthropic API
# ─────────────────────────────────────────────────────────────
def extract_anthropic(
    text: str,
    model: str = "claude-sonnet-4-6",
    api_key: Optional[str] = None,
) -> Dict[str, str]:
    """
    Modo C: extracción via Anthropic API bajo FT-SV-IA/001.

    El modelo actúa SOLO como extractor de Ω_NLP.
    La evaluación algebraica la hace el núcleo determinista.
    """
    try:
        import anthropic
    except ImportError:
        raise ImportError("Instala con: pip install anthropic")

    client = anthropic.Anthropic(api_key=api_key)
    message = client.messages.create(
        model=model,
        max_tokens=512,
        system=_SYSTEM_PROMPT,
        messages=[{"role": "user", "content": text}],
    )
    content = message.content[0].text
    try:
        start = content.index("{")
        end   = content.rindex("}") + 1
        raw   = json.loads(content[start:end])
    except (ValueError, json.JSONDecodeError):
        raw = {}
    return validate_observables_dict(raw)


# ─────────────────────────────────────────────────────────────
# Interfaz unificada
# ─────────────────────────────────────────────────────────────
def extract(
    text: str,
    mode: str = "ollama",
    **kwargs: Any,
) -> Dict[str, str]:
    """
    Extrae Ω_NLP desde texto en lenguaje natural.

    mode:
      "direct"    → Modo A (sin ML, requiere kwarg observables=dict)
      "ollama"    → Modo B via Ollama local (por defecto)
      "hf"        → Modo B via HuggingFace API
      "anthropic" → Modo C via Anthropic API
    """
    if mode == "direct":
        return extract_direct(kwargs["observables"])
    elif mode == "ollama":
        return extract_ollama(text, **kwargs)
    elif mode == "hf":
        return extract_hf_api(text, **kwargs)
    elif mode == "anthropic":
        return extract_anthropic(text, **kwargs)
    else:
        raise ValueError(f"Modo desconocido: {mode!r}. Usa: direct | ollama | hf | anthropic")
