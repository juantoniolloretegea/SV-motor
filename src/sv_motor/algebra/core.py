"""
sv_motor.algebra.core
=====================
Núcleo algebraico determinista del Sistema Vectorial SV.

Fuente doctrinal: Colección I, Documentos 1-3.
Toda función de este módulo es determinista, reproducible y sin dependencias externas.
Ninguna función aquí usa estadística, inferencia ni temporalismo.
"""
from __future__ import annotations

from typing import Dict, Iterable, List, Sequence, Set

# ─────────────────────────────────────────────────────────────
# Alfabeto ternario canónico  Σ = {0, 1, U}
# ─────────────────────────────────────────────────────────────
U: str = "U"
K3_APTO         = "APTO"
K3_INDETERMINADO = "INDETERMINADO"
K3_NO_APTO      = "NO_APTO"

# ─────────────────────────────────────────────────────────────
# Umbral canónico  T(n) = ⌊7n/9⌋
# ─────────────────────────────────────────────────────────────
def threshold(n: int) -> int:
    """Umbral canónico del SV: T(n) = ⌊7n/9⌋."""
    return (7 * n) // 9


# ─────────────────────────────────────────────────────────────
# Clasificación de célula en K₃
# ─────────────────────────────────────────────────────────────
def classify_cell(values: Sequence[object]) -> str:
    """
    Clasifica un vector ternario en K₃ mediante umbral T(n).

    Regla:
      n1 ≥ T(n)  → NO_APTO
      n0 ≥ T(n)  → APTO
      resto       → INDETERMINADO
    """
    vals = list(values)
    n = len(vals)
    n1 = sum(v == 1 for v in vals)
    n0 = sum(v == 0 for v in vals)
    t  = threshold(n)
    if n1 >= t:
        return K3_NO_APTO
    if n0 >= t:
        return K3_APTO
    return K3_INDETERMINADO


def summarize_cell(values: Sequence[object]) -> Dict[str, object]:
    """Resumen numérico + dictamen K₃ de una célula."""
    vals = list(values)
    n  = len(vals)
    n1 = sum(v == 1 for v in vals)
    n0 = sum(v == 0 for v in vals)
    nU = n - n0 - n1
    t  = threshold(n)
    return {"n": n, "n0": n0, "n1": n1, "nU": nU, "threshold": t,
            "class": classify_cell(vals)}


# ─────────────────────────────────────────────────────────────
# Compuerta conservadora sobre K₃
# ─────────────────────────────────────────────────────────────
_GATE_TABLE: Dict[tuple, str] = {
    (K3_APTO,         K3_APTO):          K3_APTO,
    (K3_APTO,         K3_INDETERMINADO): K3_INDETERMINADO,
    (K3_APTO,         K3_NO_APTO):       K3_NO_APTO,
    (K3_INDETERMINADO,K3_APTO):          K3_INDETERMINADO,
    (K3_INDETERMINADO,K3_INDETERMINADO): K3_INDETERMINADO,
    (K3_INDETERMINADO,K3_NO_APTO):       K3_NO_APTO,
    (K3_NO_APTO,      K3_APTO):          K3_NO_APTO,
    (K3_NO_APTO,      K3_INDETERMINADO): K3_NO_APTO,
    (K3_NO_APTO,      K3_NO_APTO):       K3_NO_APTO,
}

def gate(left: str, right: str) -> str:
    """Compuerta conservadora: NO_APTO domina, INDET sobre APTO."""
    return _GATE_TABLE[(left, right)]


def gate_chain(values: Iterable[str]) -> str:
    """Aplica gate() en cadena sobre una secuencia de K₃."""
    it = iter(values)
    try:
        current = next(it)
    except StopIteration:
        raise ValueError("gate_chain requiere al menos un valor")
    for v in it:
        current = gate(current, v)
    return current


# ─────────────────────────────────────────────────────────────
# Compuerta posicional sobre vectores ternarios {0,1,U}
# ─────────────────────────────────────────────────────────────
def gate_value(left: object, right: object) -> object:
    """
    Compuerta conservadora posición a posición sobre Σ = {0, 1, U}.
    1 domina · U domina sobre 0 · 0 solo si ambas son 0.
    """
    if left == 1 or right == 1:
        return 1
    if left == U or right == U:
        return U
    return 0


def gate_vector(
    left: Sequence[object], right: Sequence[object]
) -> List[object]:
    """Aplica gate_value() posición a posición."""
    return [gate_value(a, b) for a, b in zip(left, right)]


# ─────────────────────────────────────────────────────────────
# Γbar_H — célula supervisora de gobernabilidad
# ─────────────────────────────────────────────────────────────
def gamma_h_labels(
    vector: Sequence[object],
    support_map: Dict[int, Set[int]],
) -> Dict[int, str]:
    """
    Clasifica cada posición U del vector según su soporte en H.
    Devuelve {posición_1based: 'irreducible'|'resoluble'|'fronteriza'}.
    """
    out: Dict[int, str] = {}
    for idx, value in enumerate(vector, start=1):
        if value != U:
            continue
        support = support_map.get(idx, set())
        if not support:
            out[idx] = "irreducible"
        elif len(support) == 1:
            out[idx] = "resoluble"
        else:
            out[idx] = "fronteriza"
    return out


def gamma_bar_h(
    vector: Sequence[object],
    support_map: Dict[int, Set[int]],
) -> List[object]:
    """
    Γbar_H : {0,1,U}⁹ × H → {0,1,U}⁹

    Regla por posición:
      vj ≠ U  → 0   (el 1 se propaga vía compuerta, C_gob no lo duplica)
      vj = U, irreducible → 1
      vj = U, fronteriza  → U
      vj = U, resoluble   → U   (preserva honestidad de la U pendiente)
    """
    labels = gamma_h_labels(vector, support_map)
    supervisor: List[object] = []
    for idx, value in enumerate(vector, start=1):
        if value != U:
            supervisor.append(0)
        else:
            supervisor.append(1 if labels[idx] == "irreducible" else U)
    return supervisor


# ─────────────────────────────────────────────────────────────
# Clasificador del agente — κ₃
# ─────────────────────────────────────────────────────────────
def kappa3(
    supervisor: Sequence[object],
    architecture: Sequence[object],
) -> str:
    """
    κ₃(C_gob, A_NLP) → K₃

    NO_APTO si hay 1 en supervisor o en architecture.
    INDETERMINADO si hay U en architecture (y no hay 1).
    APTO en caso contrario.
    """
    if any(v == 1 for v in supervisor) or any(v == 1 for v in architecture):
        return K3_NO_APTO
    if any(v == U for v in architecture):
        return K3_INDETERMINADO
    return K3_APTO


# ─────────────────────────────────────────────────────────────
# Política de salida
# ─────────────────────────────────────────────────────────────
_POLICY = {
    K3_APTO:          "CERRAR_FRAME",
    K3_INDETERMINADO: "CONTINUAR_EN_W(T,k)",
    K3_NO_APTO:       "PROPONER_FORK",
}

def resolve_policy(k3: str) -> str:
    """Política canónica asociada a cada estado K₃."""
    return _POLICY[k3]
