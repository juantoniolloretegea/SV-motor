"""
Evaluador algebraico inicial del dominio 𝔇_DEV del motor SV.

Este módulo declara y ejecuta, de forma determinista, la célula C_dev^9
para la evaluación de artefactos técnicos producidos o corregidos en el
marco del desarrollo del Sistema Vectorial SV.

No activa ninguna capa generativa. No introduce semántica nueva. No decide
cierre soberano humano. Su función es estrictamente evaluadora y trazable.
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
class DevObservables:
    conformidad_doctrinal: str
    suficiencia_material: str
    trazabilidad: str
    frontera_ml_algebra: str
    preservacion_u: str
    paridad_doc_artefacto: str
    soberania_humana: str
    protocolo_entrada: str
    reversibilidad: str


_MAP_CONFORMIDAD = {"conforme": 0, "ajena": 1, "indeterminada": U}
_MAP_SUFICIENCIA = {"verificable": 0, "inverificable": 1, "indeterminada": U}
_MAP_TRAZABILIDAD = {"trazable": 0, "ausente": 1, "indeterminada": U}
_MAP_FRONTERA = {"preservada": 0, "delegada": 1, "indeterminada": U}
_MAP_U = {"preservada": 0, "clausura-falsa": 1, "indeterminada": U}
_MAP_PARIDAD = {"alineada": 0, "desalineada": 1, "indeterminada": U}
_MAP_HS = {"preservada": 0, "bloqueada": 1, "indeterminada": U}
_MAP_PROTOCOLO = {"alineado": 0, "bloqueado": 1, "indeterminada": U}
_MAP_REVERSIBILIDAD = {"append-only": 0, "reescribe-historia": 1, "indeterminada": U}

DEV_SUPPORT_BASE: Dict[int, Set[int]] = {
    1: {0, 1},
    2: {0, 1},
    3: {0, 1},
    4: {0},
    5: {0, 1},
    6: {0, 1},
    7: {0, 1},
    8: {0},
    9: {0, 1},
}

POSITION_LABELS = {
    1: "P1 conformidad doctrinal",
    2: "P2 suficiencia material",
    3: "P3 trazabilidad",
    4: "P4 frontera ML/álgebra",
    5: "P5 preservación de U",
    6: "P6 paridad documento–artefacto",
    7: "P7 soberanía humana",
    8: "P8 protocolo de entrada",
    9: "P9 reversibilidad",
}

OBLIGATION_MAP = {
    1: "Aportar contraste doctrinal suficiente del artefacto con el SV.",
    2: "Aportar prueba material, instalación o verificación reproducible.",
    3: "Añadir manifiesto, origen y huella verificable del artefacto.",
    4: "Restituir la separación entre extracción ML y evaluación algebraica.",
    5: "Eliminar cualquier clausura falsa de la indeterminación honesta.",
    6: "Alinear lo que el artefacto hace con lo que declara documentalmente.",
    7: "Garantizar revisión, reversión y cierre soberano del humano competente.",
    8: "Hacer pasar la entrada por la compuerta ejecutable del protocolo.",
    9: "Preservar el carácter append-only del suceso técnico evaluado.",
}

POLICY_DEV = {
    K3_APTO: "CIERRE_TECNICO_PENDIENTE_DE_HS",
    K3_INDETERMINADO: "CORREGIR_BAJO_OBLIGACIONES",
    K3_NO_APTO: "PROPONER_FORK",
}


def i_dev(obs: DevObservables) -> List[object]:
    """Transductor I_DEV : Ω_DEV → {0,1,U}⁹."""
    return [
        _MAP_CONFORMIDAD[obs.conformidad_doctrinal],
        _MAP_SUFICIENCIA[obs.suficiencia_material],
        _MAP_TRAZABILIDAD[obs.trazabilidad],
        _MAP_FRONTERA[obs.frontera_ml_algebra],
        _MAP_U[obs.preservacion_u],
        _MAP_PARIDAD[obs.paridad_doc_artefacto],
        _MAP_HS[obs.soberania_humana],
        _MAP_PROTOCOLO[obs.protocolo_entrada],
        _MAP_REVERSIBILIDAD[obs.reversibilidad],
    ]


def dev_observables_from_dict(d: dict) -> DevObservables:
    return DevObservables(**{k: str(v) for k, v in d.items()})


def resolve_dev_policy(k3: str) -> str:
    return POLICY_DEV[k3]


def build_dev_obligations(
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


def run_dev_agent(
    obs: DevObservables,
    support_override: Optional[Dict[int, Set[int]]] = None,
) -> Dict[str, object]:
    support = dict(DEV_SUPPORT_BASE)
    if support_override:
        for k, v in support_override.items():
            support[int(k)] = set(v)

    c_dev = i_dev(obs)
    labels = gamma_h_labels(c_dev, support)
    c_gob = gamma_bar_h(c_dev, support)
    a_dev = gate_vector(c_gob, c_dev)
    k3 = kappa3(c_gob, a_dev)
    policy = resolve_dev_policy(k3)
    u_irr = [idx for idx, cls in labels.items() if cls == "irreducible"]
    evaluable = len(u_irr) == 0
    obligations = build_dev_obligations(c_dev, labels)

    return {
        "observables": asdict(obs),
        "C_dev9": c_dev,
        "gamma_h_labels": labels,
        "C_gob9": c_gob,
        "A_DEV": a_dev,
        "U_irr": u_irr,
        "evaluable": evaluable,
        "k3": k3,
        "politica": policy,
        "obligaciones": obligations,
    }
