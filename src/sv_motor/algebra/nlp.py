"""
sv_motor.algebra.nlp
====================
Agente NLP del SV — Colección I, Documentos 1-3.

Implementa la cadena completa:
  ω → I_NLP → C_frame^9 → Γbar_H → C_gob^9 → T_NLP → A_NLP → κ₃ → K₃
"""
from __future__ import annotations

from dataclasses import dataclass, asdict
from typing import Dict, List, Optional, Set

from sv_motor.algebra.core import (
    U,
    K3_APTO,
    K3_INDETERMINADO,
    K3_NO_APTO,
    gamma_h_labels,
    gamma_bar_h,
    gate_vector,
    kappa3,
    resolve_policy,
)

H_NLP_SUPPORT_BASE: Dict[int, Set[int]] = {
    1: {0, 1},
    2: {0, 1},
    3: {0, 1},
    4: {0},
    5: {0, 1},
    6: {0, 1},
    7: {0, 1},
    8: {0, 1},
    9: {0, 1},
}


@dataclass(frozen=True)
class Observables:
    theta: str
    pi: str
    kappa: str
    eta: str
    gamma: str
    alpha: str
    mu: str
    chi: str
    psi: str


_MAP_THETA = {"coherente": 0, "desvio": 1, "indeterminado": U}
_MAP_PI = {"sin-pregunta": 0, "resuelta": 0, "bloqueada": 1, "indeterminada": U}
_MAP_KAPPA = {"coherente": 0, "contradictoria": 1, "indeterminada": U}
_MAP_ETA = {"completa": 0, "defectuosa": 1, "indeterminada": U}
_MAP_GAMMA = {"alineada": 0, "bloqueada": 1, "indeterminada": U}
_MAP_ALPHA = {"apropiada": 0, "inapropiada": 1, "indeterminada": U}
_MAP_MU = {"sin-ambiguedad": 0, "cerrada": 0, "incompatible": 1, "indeterminada": U}
_MAP_CHI = {"sin-solicitud": 0, "atendida": 0, "denegada": 1, "indeterminada": U}
_MAP_PSI = {"en-curso": 0, "cerrado": 0, "bloqueado": 1, "indeterminado": U}

_NORMALIZE = {
    "desvío": "desvio",
    "sin-ambigüedad": "sin-ambiguedad",
}

_MAPPINGS = {
    "theta": _MAP_THETA,
    "pi": _MAP_PI,
    "kappa": _MAP_KAPPA,
    "eta": _MAP_ETA,
    "gamma": _MAP_GAMMA,
    "alpha": _MAP_ALPHA,
    "mu": _MAP_MU,
    "chi": _MAP_CHI,
    "psi": _MAP_PSI,
}

_FIELDS = ("theta", "pi", "kappa", "eta", "gamma", "alpha", "mu", "chi", "psi")


def _norm(v: str) -> str:
    return _NORMALIZE.get(v, v)


def _build_observables_map(obs: Observables) -> Dict[str, str]:
    return {field: _norm(str(getattr(obs, field))) for field in _FIELDS}


def i_nlp(obs: Observables) -> List[object]:
    """
    I_NLP : Ω_NLP → {0,1,U}⁹

    Aplicación total, honesta y pertinente (Proposición 1+2, Documento 2).
    En caso de valor fuera de dominio, emite ValueError con el campo exacto.
    """
    normalized = _build_observables_map(obs)
    out: List[object] = []
    for field in _FIELDS:
        value = normalized[field]
        mapping = _MAPPINGS[field]
        if value not in mapping:
            raise ValueError(f"Valor fuera de dominio para {field}: {value!r}")
        out.append(mapping[value])
    return out


def observables_from_dict(d: dict) -> Observables:
    """Construye Observables desde un diccionario (salida de EXT-NLP)."""
    return Observables(**{k: _norm(str(v)) for k, v in d.items()})


def run_agent(obs: Observables, support_override: Optional[Dict[int, Set[int]]] = None) -> Dict[str, object]:
    """
    Ejecuta la cadena completa del agente NLP:
      ω → C_frame^9 → C_gob^9 → A_NLP → κ₃ → política
    Devuelve un diccionario completamente trazable y verificable.
    """
    support = dict(H_NLP_SUPPORT_BASE)
    if support_override:
        for k, v in support_override.items():
            support[int(k)] = set(v)

    c_frame = i_nlp(obs)
    labels = gamma_h_labels(c_frame, support)
    c_gob = gamma_bar_h(c_frame, support)
    a_nlp = gate_vector(c_gob, c_frame)
    k3 = kappa3(c_gob, a_nlp)
    policy = resolve_policy(k3)
    u_irr = [idx for idx, cls in labels.items() if cls == "irreducible"]
    gobernable = len(u_irr) == 0

    return {
        "observables": asdict(obs),
        "C_frame9": c_frame,
        "gamma_h_labels": labels,
        "C_gob9": c_gob,
        "A_NLP": a_nlp,
        "U_irr": u_irr,
        "gobernable": gobernable,
        "k3": k3,
        "politica": policy,
    }


def batch_run(cases: list[dict]) -> list[dict]:
    """Ejecuta run_agent sobre una lista de casos con expected_k3 y verificaciones."""
    results = []
    for case in cases:
        obs = observables_from_dict(case["observables"])
        override = case.get("override_support")
        result = run_agent(obs, support_override=override)
        result["id"] = case.get("id", "?")
        if "clase_esperada" in case and result["k3"] != case["clase_esperada"]:
            raise ValueError((case.get("id"), result["k3"], case["clase_esperada"]))
        if "politica_esperada" in case and result["politica"] != case["politica_esperada"]:
            raise ValueError((case.get("id"), result["politica"], case["politica_esperada"]))
        results.append(result)
    return results
