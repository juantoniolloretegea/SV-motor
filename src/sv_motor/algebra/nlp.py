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
    U, K3_APTO, K3_INDETERMINADO, K3_NO_APTO,
    gamma_h_labels, gamma_bar_h, gate_vector, kappa3, resolve_policy,
)

# ─────────────────────────────────────────────────────────────
# Horizonte H_NLP heredado del Documento 2
# Supp(Pj) ⊆ {0,1}  —  {0,1}=fronteriza · {0}=resoluble · ∅=irreducible
# ─────────────────────────────────────────────────────────────
H_NLP_SUPPORT_BASE: Dict[int, Set[int]] = {
    1: {0, 1},
    2: {0, 1},
    3: {0, 1},
    4: {0},        # P4 resoluble: único soporte {0}
    5: {0, 1},
    6: {0, 1},
    7: {0, 1},
    8: {0, 1},
    9: {0, 1},
}

# ─────────────────────────────────────────────────────────────
# Paquete observacional Ω_NLP  (Documento 2, §2.3)
# ─────────────────────────────────────────────────────────────
@dataclass(frozen=True)
class Observables:
    theta: str   # P1 coherencia temática
    pi:    str   # P2 pregunta abierta resuelta
    kappa: str   # P3 ausencia de contradicción
    eta:   str   # P4 completitud del enunciado
    gamma: str   # P5 filiación al objetivo
    alpha: str   # P6 tipo de acto esperado
    mu:    str   # P7 ambigüedad resuelta
    chi:   str   # P8 solicitud de clarificación resuelta
    psi:   str   # P9 estado del objetivo


# ─────────────────────────────────────────────────────────────
# Transductor I_NLP  (Documento 2, §3.2)
# ─────────────────────────────────────────────────────────────
_MAP_THETA = {"coherente": 0, "desvio": 1, "indeterminado": U}
_MAP_PI    = {"sin-pregunta": 0, "resuelta": 0, "bloqueada": 1, "indeterminada": U}
_MAP_KAPPA = {"coherente": 0, "contradictoria": 1, "indeterminada": U}
_MAP_ETA   = {"completa": 0, "defectuosa": 1, "indeterminada": U}
_MAP_GAMMA = {"alineada": 0, "bloqueada": 1, "indeterminada": U}
_MAP_ALPHA = {"apropiada": 0, "inapropiada": 1, "indeterminada": U}
_MAP_MU    = {"sin-ambiguedad": 0, "cerrada": 0, "incompatible": 1, "indeterminada": U}
_MAP_CHI   = {"sin-solicitud": 0, "atendida": 0, "denegada": 1, "indeterminada": U}
_MAP_PSI   = {"en-curso": 0, "cerrado": 0, "bloqueado": 1, "indeterminado": U}

# Normalización de variantes acentuadas
_NORMALIZE = {
    "desvío":        "desvio",
    "sin-ambigüedad":"sin-ambiguedad",
}

def _norm(v: str) -> str:
    return _NORMALIZE.get(v, v)

def i_nlp(obs: Observables) -> List[object]:
    """
    I_NLP : Ω_NLP → {0,1,U}⁹

    Aplicación total, honesta y pertinente (Proposición 1+2, Documento 2).
    """
    return [
        _MAP_THETA[_norm(obs.theta)],
        _MAP_PI   [_norm(obs.pi)],
        _MAP_KAPPA[_norm(obs.kappa)],
        _MAP_ETA  [_norm(obs.eta)],
        _MAP_GAMMA[_norm(obs.gamma)],
        _MAP_ALPHA[_norm(obs.alpha)],
        _MAP_MU   [_norm(obs.mu)],
        _MAP_CHI  [_norm(obs.chi)],
        _MAP_PSI  [_norm(obs.psi)],
    ]

def observables_from_dict(d: dict) -> Observables:
    """Construye Observables desde un diccionario (salida de EXT-NLP)."""
    return Observables(**{k: _norm(str(v)) for k, v in d.items()})


# ─────────────────────────────────────────────────────────────
# Agente completo  (Documento 3)
# ─────────────────────────────────────────────────────────────
def run_agent(
    obs: Observables,
    support_override: Optional[Dict[int, Set[int]]] = None,
) -> Dict[str, object]:
    """
    Ejecuta la cadena completa del agente NLP:

      ω → C_frame^9 → C_gob^9 → A_NLP → κ₃ → política

    Devuelve un diccionario completamente trazable y verificable.
    """
    support = dict(H_NLP_SUPPORT_BASE)
    if support_override:
        for k, v in support_override.items():
            support[int(k)] = set(v)

    c_frame    = i_nlp(obs)
    labels     = gamma_h_labels(c_frame, support)
    c_gob      = gamma_bar_h(c_frame, support)
    a_nlp      = gate_vector(c_gob, c_frame)
    k3         = kappa3(c_gob, a_nlp)
    policy     = resolve_policy(k3)
    u_irr      = [idx for idx, cls in labels.items() if cls == "irreducible"]
    gobernable = len(u_irr) == 0

    return {
        "observables":          asdict(obs),
        "C_frame9":             c_frame,
        "gamma_h_labels":       labels,
        "C_gob9":               c_gob,
        "A_NLP":                a_nlp,
        "U_irr":                u_irr,
        "gobernable":           gobernable,
        "k3":                   k3,
        "politica":             policy,
    }


def batch_run(cases: list[dict]) -> list[dict]:
    """Ejecuta run_agent sobre una lista de casos con expected_k3 y aserciones."""
    results = []
    for case in cases:
        obs = observables_from_dict(case["observables"])
        override = case.get("override_support")
        result = run_agent(obs, support_override=override)
        result["id"] = case.get("id", "?")
        # Aserciones automáticas si están declaradas
        if "clase_esperada" in case:
            assert result["k3"] == case["clase_esperada"], (
                case.get("id"), result["k3"], case["clase_esperada"]
            )
        if "politica_esperada" in case:
            assert result["politica"] == case["politica_esperada"], (
                case.get("id"), result["politica"], case["politica_esperada"]
            )
        results.append(result)
    return results
