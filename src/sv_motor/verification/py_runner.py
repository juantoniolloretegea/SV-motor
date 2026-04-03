"""
sv_motor.verification.py_runner
================================
Capa de verificación Python del Sistema Vectorial SV.

POSICIÓN DOCTRINAL
------------------
El motor de ejecución canónico del SV es el backend Rust compilado desde
código `.svp`. Este módulo no lo reemplaza.

Su función es producir, en Python puro, una salida JSON canónica idéntica
en estructura a la que el backend Rust producirá cuando el `.svp` esté
implementado. Eso permite tres usos legítimos:

  1. VERIFICACIÓN CRUZADA: un programador no experto en SV puede ejecutar
     el runner Python, obtener JSON, y compararlo con la salida Rust del
     `.svp` usando el comparador (comparator.py).

  2. DOBLE VARA: dos implementaciones del mismo álgebra — Python y Rust —
     produciendo JSON idéntico es la prueba más sólida de corrección.

  3. UNIVERSALIDAD: Python + JSON hace el SV accesible a cualquier
     ecosistema sin modificar la especificación algebraica.

INVARIANTES DE DISEÑO
---------------------
- Sin dependencias externas. Solo stdlib + sv_motor.algebra.
- Sin runtimes. El runner Python no instala nada en tiempo de ejecución.
- Sin estado global. Cada llamada a `run_sv_program` es pura.
- JSON como único formato de intercambio. El mismo esquema que usará Rust.
- Toda U permanece como "U" en el JSON — nunca se colapsa a 0 o 1.

ESQUEMA JSON CANÓNICO (compartido con el backend Rust/.svp)
-----------------------------------------------------------
{
  "sv_version":   "0.1.5",
  "engine":       "python",          # "rust" cuando venga del .svp
  "domain":       "NLP|DEV|CUSTODIA|CUSTOM",
  "programa": {
    "observables": {...},
    "horizonte":   {"1": [0,1], "2": [0], ...}   # support_map
  },
  "traza": {
    "C_frame":          [...],
    "gamma_h_labels":   {"1": "irreducible|fronteriza|resoluble", ...},
    "C_gob":            [...],
    "A_agente":         [...],
    "U_irr":            [...],
    "gobernable":       true|false
  },
  "dictamen": {
    "k3":       "APTO|INDETERMINADO|NO_APTO",
    "politica": "CERRAR_FRAME|CONTINUAR_EN_W(T,k)|PROPONER_FORK",
    "obligaciones": [...]        # solo en DEV/CUSTODIA
  }
}
"""
from __future__ import annotations

import json
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional, Sequence, Set

from sv_motor.algebra.core import (
    U,
    gamma_h_labels,
    gamma_bar_h,
    gate_vector,
    kappa3,
    resolve_policy,
)
from sv_motor.algebra.nlp import (
    Observables as NLPObservables,
    i_nlp,
    H_NLP_SUPPORT_BASE,
    run_agent as _run_nlp,
)
from sv_motor.algebra.dev import (
    DevObservables,
    i_dev,
    DEV_SUPPORT_BASE,
    run_dev_agent as _run_dev,
    build_dev_obligations,
    POLICY_DEV,
)
from sv_motor.security.custodia_estructural import (
    CustodiaMotorObservables,
    i_custodia_motor,
    CUSTODIA_SUPPORT_BASE,
    run_custodia_motor as _run_custodia,
)

SV_VERSION = "0.1.5"
_ENGINE_PYTHON = "python"


# ─────────────────────────────────────────────────────────────────────────────
# Serialización de vectores ternarios para JSON
# (U → "U", 0 → 0, 1 → 1 — U nunca se colapsa)
# ─────────────────────────────────────────────────────────────────────────────

def _ser_vector(v: Sequence[object]) -> List[Any]:
    """Serializa un vector ternario para JSON. U se emite como cadena 'U'."""
    return [("U" if x == U else x) for x in v]


def _ser_labels(labels: Dict[int, str]) -> Dict[str, str]:
    """Serializa gamma_h_labels para JSON (claves como string)."""
    return {str(k): v for k, v in labels.items()}


def _ser_support(support: Dict[int, Set[int]]) -> Dict[str, List[int]]:
    """Serializa el horizonte (support_map) para JSON."""
    return {str(k): sorted(v) for k, v in support.items()}


# ─────────────────────────────────────────────────────────────────────────────
# Resultado canónico — estructura idéntica a la que producirá el backend Rust
# ─────────────────────────────────────────────────────────────────────────────

@dataclass
class SVProgramResult:
    """
    Resultado canónico de ejecución de un programa SV.

    El campo `json_canonical` es el único formato que se intercambia
    con el backend Rust. `comparator.py` lo usa para la doble vara.
    """
    sv_version:   str
    engine:       str
    domain:       str
    programa:     Dict[str, Any]
    traza:        Dict[str, Any]
    dictamen:     Dict[str, Any]

    def to_dict(self) -> Dict[str, Any]:
        """Devuelve una copia profunda — segura para mutación en tests y comparadores."""
        import copy
        return copy.deepcopy({
            "sv_version": self.sv_version,
            "engine":     self.engine,
            "domain":     self.domain,
            "programa":   self.programa,
            "traza":      self.traza,
            "dictamen":   self.dictamen,
        })

    def json_canonical(self, indent: int = 2) -> str:
        """JSON canónico — mismo esquema que producirá el backend Rust/.svp."""
        return json.dumps(self.to_dict(), ensure_ascii=False, indent=indent)


# ─────────────────────────────────────────────────────────────────────────────
# Runner NLP
# ─────────────────────────────────────────────────────────────────────────────

def run_nlp(
    observables: Dict[str, str],
    support_override: Optional[Dict[int, Set[int]]] = None,
) -> SVProgramResult:
    """
    Ejecuta el programa SV sobre el dominio NLP y devuelve SVProgramResult.

    Args:
        observables: dict con las 9 claves de Ω_NLP (theta, pi, kappa, …).
        support_override: horizonte alternativo al H_NLP_SUPPORT_BASE.

    Returns:
        SVProgramResult con JSON canónico listo para comparación con Rust.
    """
    from sv_motor.algebra.nlp import observables_from_dict
    obs = observables_from_dict(observables)
    support = dict(support_override or H_NLP_SUPPORT_BASE)

    c_frame  = i_nlp(obs)
    labels   = gamma_h_labels(c_frame, support)
    c_gob    = gamma_bar_h(c_frame, support)
    a_agente = gate_vector(c_gob, c_frame)
    k3       = kappa3(c_gob, a_agente)
    politica = resolve_policy(k3)
    u_irr    = [idx for idx, cls in labels.items() if cls == "irreducible"]

    return SVProgramResult(
        sv_version = SV_VERSION,
        engine     = _ENGINE_PYTHON,
        domain     = "NLP",
        programa   = {
            "observables": dict(observables),
            "horizonte":   _ser_support(support),
        },
        traza = {
            "C_frame":        _ser_vector(c_frame),
            "gamma_h_labels": _ser_labels(labels),
            "C_gob":          _ser_vector(c_gob),
            "A_agente":       _ser_vector(a_agente),
            "U_irr":          u_irr,
            "gobernable":     len(u_irr) == 0,
        },
        dictamen = {
            "k3":       k3,
            "politica": politica,
        },
    )


# ─────────────────────────────────────────────────────────────────────────────
# Runner DEV
# ─────────────────────────────────────────────────────────────────────────────

def run_dev(
    observables: Dict[str, str],
    support_override: Optional[Dict[int, Set[int]]] = None,
) -> SVProgramResult:
    """
    Ejecuta el programa SV sobre el dominio DEV y devuelve SVProgramResult.

    Args:
        observables: dict con las 9 claves de Ω_DEV.
        support_override: horizonte alternativo al DEV_SUPPORT_BASE.

    Returns:
        SVProgramResult con JSON canónico listo para comparación con Rust.
    """
    from sv_motor.algebra.dev import dev_observables_from_dict, build_dev_obligations
    obs = dev_observables_from_dict(observables)
    support = dict(support_override or DEV_SUPPORT_BASE)

    c_frame  = i_dev(obs)
    labels   = gamma_h_labels(c_frame, support)
    c_gob    = gamma_bar_h(c_frame, support)
    a_agente = gate_vector(c_gob, c_frame)
    k3       = kappa3(c_gob, a_agente)
    politica = POLICY_DEV[k3]
    u_irr    = [idx for idx, cls in labels.items() if cls == "irreducible"]
    oblig    = build_dev_obligations(c_frame, labels)

    return SVProgramResult(
        sv_version = SV_VERSION,
        engine     = _ENGINE_PYTHON,
        domain     = "DEV",
        programa   = {
            "observables": dict(observables),
            "horizonte":   _ser_support(support),
        },
        traza = {
            "C_frame":        _ser_vector(c_frame),
            "gamma_h_labels": _ser_labels(labels),
            "C_gob":          _ser_vector(c_gob),
            "A_agente":       _ser_vector(a_agente),
            "U_irr":          u_irr,
            "gobernable":     len(u_irr) == 0,
        },
        dictamen = {
            "k3":          k3,
            "politica":    politica,
            "obligaciones": oblig,
        },
    )


# ─────────────────────────────────────────────────────────────────────────────
# Runner CUSTODIA
# ─────────────────────────────────────────────────────────────────────────────

def run_custodia(
    observables: Dict[str, str],
    support_override: Optional[Dict[int, Set[int]]] = None,
) -> SVProgramResult:
    """
    Ejecuta el programa SV sobre el dominio CUSTODIA y devuelve SVProgramResult.

    Args:
        observables: dict con las 9 claves de Ω_CUSTODIA.
        support_override: horizonte alternativo al CUSTODIA_SUPPORT_BASE.

    Returns:
        SVProgramResult con JSON canónico listo para comparación con Rust.
    """
    from sv_motor.security.custodia_estructural import (
        custodia_observables_from_dict,
        build_custodia_obligations,
        POLICY_CUSTODIA,
    )
    obs     = custodia_observables_from_dict(observables)
    support = dict(support_override or CUSTODIA_SUPPORT_BASE)

    c_frame  = i_custodia_motor(obs)
    labels   = gamma_h_labels(c_frame, support)
    c_gob    = gamma_bar_h(c_frame, support)
    a_agente = gate_vector(c_gob, c_frame)
    k3       = kappa3(c_gob, a_agente)
    politica = POLICY_CUSTODIA[k3]
    u_irr    = [idx for idx, cls in labels.items() if cls == "irreducible"]
    oblig    = build_custodia_obligations(c_frame, labels)

    return SVProgramResult(
        sv_version = SV_VERSION,
        engine     = _ENGINE_PYTHON,
        domain     = "CUSTODIA",
        programa   = {
            "observables": dict(observables),
            "horizonte":   _ser_support(support),
        },
        traza = {
            "C_frame":        _ser_vector(c_frame),
            "gamma_h_labels": _ser_labels(labels),
            "C_gob":          _ser_vector(c_gob),
            "A_agente":       _ser_vector(a_agente),
            "U_irr":          u_irr,
            "gobernable":     len(u_irr) == 0,
        },
        dictamen = {
            "k3":          k3,
            "politica":    politica,
            "obligaciones": oblig,
        },
    )


# ─────────────────────────────────────────────────────────────────────────────
# Runner CUSTOM — cualquier dominio con célula declarada manualmente
# ─────────────────────────────────────────────────────────────────────────────

def run_custom(
    c_frame: Sequence[object],
    support_map: Dict[int, Set[int]],
    domain_name: str = "CUSTOM",
    observables_raw: Optional[Dict[str, Any]] = None,
) -> SVProgramResult:
    """
    Ejecuta el programa SV sobre un dominio declarado manualmente.

    Permite al programador declarar directamente el vector C_frame y
    el horizonte support_map sin pasar por un transductor de dominio.
    Es la entrada más general — válida para cualquier dominio futuro.

    Args:
        c_frame:       vector ternario {0,1,U}^n.
        support_map:   dict[posicion_1based, set[valores_soporte]].
        domain_name:   etiqueta del dominio (libre).
        observables_raw: metadata opcional del programador.

    Returns:
        SVProgramResult con JSON canónico.
    """
    labels   = gamma_h_labels(list(c_frame), support_map)
    c_gob    = gamma_bar_h(list(c_frame), support_map)
    a_agente = gate_vector(c_gob, list(c_frame))
    k3       = kappa3(c_gob, a_agente)
    politica = resolve_policy(k3)
    u_irr    = [idx for idx, cls in labels.items() if cls == "irreducible"]

    return SVProgramResult(
        sv_version = SV_VERSION,
        engine     = _ENGINE_PYTHON,
        domain     = domain_name,
        programa   = {
            "observables": observables_raw or {},
            "horizonte":   _ser_support(support_map),
            "C_frame_input": _ser_vector(c_frame),
        },
        traza = {
            "C_frame":        _ser_vector(c_frame),
            "gamma_h_labels": _ser_labels(labels),
            "C_gob":          _ser_vector(c_gob),
            "A_agente":       _ser_vector(a_agente),
            "U_irr":          u_irr,
            "gobernable":     len(u_irr) == 0,
        },
        dictamen = {
            "k3":       k3,
            "politica": politica,
        },
    )
