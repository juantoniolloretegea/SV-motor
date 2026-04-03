"""
Compuerta ejecutable mínima de custodia estructural del frente motor del SV.

Esta pieza no introduce gramática nueva, no modifica el Lenguaje SV y no abre
capa generativa alguna. Su función es bloquear o detener, antes de producir
efectos, cualquier avance del motor que presione ilegítimamente al SV.
"""
from __future__ import annotations

from dataclasses import asdict, dataclass
from typing import Dict, List, Optional, Sequence, Set

from sv_motor.algebra.core import (
    U,
    K3_APTO,
    K3_INDETERMINADO,
    K3_NO_APTO,
    gamma_h_labels,
    gamma_bar_h,
    gate_vector,
    kappa3,
)


@dataclass(frozen=True)
class CustodiaMotorObservables:
    anclaje_doctrinal: str
    presion_sobre_lenguaje: str
    frontera_ml_algebra: str
    paridad_documento_laboratorio: str
    preservacion_u: str
    limites_de_fase: str
    trazabilidad: str
    protocolo_activo: str
    dependencia_superior_respetada: str


_MAP_ANCLAJE = {"anclado": 0, "deriva": 1, "indeterminada": U}
_MAP_LENGUAJE = {"respetada": 0, "presiona": 1, "indeterminada": U}
_MAP_FRONTERA = {"preservada": 0, "delegada": 1, "indeterminada": U}
_MAP_PARIDAD = {"alineada": 0, "desalineada": 1, "indeterminada": U}
_MAP_U = {"preservada": 0, "clausura-falsa": 1, "indeterminada": U}
_MAP_FASE = {"respetados": 0, "sobreactuacion": 1, "indeterminada": U}
_MAP_TRAZABILIDAD = {"trazable": 0, "ausente": 1, "indeterminada": U}
_MAP_PROTOCOLO = {"activo": 0, "bloqueado": 1, "indeterminada": U}
_MAP_DEPENDENCIA = {"respetada": 0, "quiebra": 1, "indeterminada": U}

# Posiciones irreducibles: P2 y P8. Si quedan en U, la compuerta bloquea.
CUSTODIA_SUPPORT_BASE: Dict[int, Set[int]] = {
    1: {0, 1},
    2: set(),
    3: {0, 1},
    4: {0, 1},
    5: {0, 1},
    6: {0, 1},
    7: {0, 1},
    8: set(),
    9: {0, 1},
}

POSITION_LABELS = {
    1: "P1 anclaje doctrinal",
    2: "P2 presión sobre lenguaje, DSL o IR",
    3: "P3 frontera ML/álgebra",
    4: "P4 paridad documento–artefacto–laboratorio",
    5: "P5 preservación de U",
    6: "P6 límites de fase respetados",
    7: "P7 trazabilidad suficiente",
    8: "P8 protocolo activo de paso",
    9: "P9 dependencia superior respetada",
}

OBLIGATION_MAP = {
    1: "Aportar anclaje doctrinal explícito y verificable.",
    2: "Detener el avance y elevar cualquier presión sobre Lenguaje SV, IR, N4/Uso o backend.",
    3: "Restituir la separación entre extracción y evaluación algebraica.",
    4: "Alinear documento, artefacto, laboratorio y estado declarado.",
    5: "Eliminar cualquier clausura falsa de la indeterminación honesta.",
    6: "Corregir cualquier sobreactuación de fase o de alcance del frente.",
    7: "Añadir manifiesto, huella y soporte suficiente de trazabilidad.",
    8: "Activar correctamente la compuerta de paso antes de producir efectos.",
    9: "Restituir la subordinación a doctrina, lenguaje, seguridad y agentes.",
}

POLICY_CUSTODIA = {
    K3_APTO: "PERMITIR_AVANCE_CONTROLADO",
    K3_INDETERMINADO: "DETENER_Y_REVISAR",
    K3_NO_APTO: "BLOQUEAR_AVANCE",
}


def i_custodia_motor(obs: CustodiaMotorObservables) -> List[object]:
    return [
        _MAP_ANCLAJE[obs.anclaje_doctrinal],
        _MAP_LENGUAJE[obs.presion_sobre_lenguaje],
        _MAP_FRONTERA[obs.frontera_ml_algebra],
        _MAP_PARIDAD[obs.paridad_documento_laboratorio],
        _MAP_U[obs.preservacion_u],
        _MAP_FASE[obs.limites_de_fase],
        _MAP_TRAZABILIDAD[obs.trazabilidad],
        _MAP_PROTOCOLO[obs.protocolo_activo],
        _MAP_DEPENDENCIA[obs.dependencia_superior_respetada],
    ]


def custodia_observables_from_dict(d: dict) -> CustodiaMotorObservables:
    return CustodiaMotorObservables(**{k: str(v) for k, v in d.items()})


def resolve_custodia_policy(k3: str) -> str:
    return POLICY_CUSTODIA[k3]


def build_custodia_obligations(
    c_dev: Sequence[object],
    labels: Dict[int, str],
) -> List[dict[str, str]]:
    obligations: List[dict[str, str]] = []
    for idx, value in enumerate(c_dev, start=1):
        if value == 0:
            continue
        entry = {
            "posicion": f"P{idx}",
            "parametro": POSITION_LABELS[idx],
            "obligacion": OBLIGATION_MAP[idx],
        }
        if value == 1:
            entry["tipo"] = "VIOLACION"
        else:
            label = labels.get(idx, "indeterminada")
            entry["tipo"] = f"U_{label.upper()}"
        obligations.append(entry)
    return obligations


def run_custodia_motor(
    obs: CustodiaMotorObservables,
    support_override: Optional[Dict[int, Set[int]]] = None,
) -> Dict[str, object]:
    support = dict(CUSTODIA_SUPPORT_BASE)
    if support_override:
        for k, v in support_override.items():
            support[int(k)] = set(v)

    c_cust = i_custodia_motor(obs)
    labels = gamma_h_labels(c_cust, support)
    c_gob = gamma_bar_h(c_cust, support)
    a_cust = gate_vector(c_gob, c_cust)
    k3 = kappa3(c_gob, a_cust)
    policy = resolve_custodia_policy(k3)
    u_irr = [idx for idx, cls in labels.items() if cls == "irreducible"]
    evaluable = len(u_irr) == 0
    obligations = build_custodia_obligations(c_cust, labels)

    return {
        "observables": asdict(obs),
        "C_custodia9": c_cust,
        "gamma_h_labels": labels,
        "C_gob9": c_gob,
        "A_CUSTODIA": a_cust,
        "U_irr": u_irr,
        "evaluable": evaluable,
        "k3": k3,
        "politica": policy,
        "obligaciones": obligations,
    }


def sensitive_step_is_allowed(result: Dict[str, object]) -> bool:
    return result.get("k3") == K3_APTO
