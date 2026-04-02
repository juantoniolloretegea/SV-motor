"""
Compuerta ejecutable mínima equivalente a FT-SV-IA/001 para el frente motor.

Esta implementación no sustituye el documento normativo. Lo materializa en el
carril local para la demostración end-to-end de Fase 0 y para impedir la
activación real de modos opcionales sin una estructura de salida verificable.
"""
from __future__ import annotations

import json
from typing import Any, Dict, Iterable, Optional

from sv_motor.algebra.nlp import observables_from_dict, run_agent
from sv_motor.extractors.ext_nlp import validate_observables_with_ud

ACTIVATION_PHRASE = "Opera bajo FT-SV-IA/001."
INACTIVE_MESSAGE = (
    "FT-SV-IA/001 no está activo en esta sesión. "
    "Para activarlo, enunciar la frase de activación."
)
GAP_IRREDUCIBLE = "presente por naturaleza del modelo (§5.4)"


def build_session_declaration(
    activation_phrase: str,
    material_session: Iterable[str],
    doctrine_sv: Iterable[str],
    lagunas_declarables: Optional[Iterable[str]] = None,
    required_material: Optional[str] = None,
) -> Dict[str, Any]:
    if activation_phrase != ACTIVATION_PHRASE:
        return {"sesion_activa": False, "mensaje": INACTIVE_MESSAGE}
    estado = "LISTO" if not required_material else f"REQUIERE_MATERIAL: {required_material}"
    return {
        "sesion_activa": True,
        "cabecera": "SESIÓN FT-SV-IA/001 ACTIVA",
        "M_s": list(material_session),
        "D": list(doctrine_sv),
        "Lagunas declarables": list(lagunas_declarables or []),
        "Gap irreducible": GAP_IRREDUCIBLE,
        "Estado operativo": estado,
    }


def build_state_block(
    blockages: Optional[Iterable[Any]] = None,
    u_d_activas: Optional[Iterable[Any]] = None,
    action_required: Optional[str] = None,
) -> Optional[Dict[str, Any]]:
    bloqueos = list(blockages or [])
    u_d = list(u_d_activas or [])
    if action_required is not None and not isinstance(action_required, str):
        raise TypeError("La acción requerida del HS debe ser una cadena o None.")
    if not bloqueos and not u_d and not action_required:
        return None
    return {
        "cabecera": "ESTADO DE SALIDA",
        "Bloqueos": bloqueos or "NINGUNO",
        "U_d activas": u_d or "NINGUNA",
        "Acción requerida del HS": action_required or "NINGUNA",
    }


def render_protocol_output(payload: Dict[str, Any]) -> str:
    return json.dumps(payload, ensure_ascii=False, indent=2)


def run_direct_ft_session(
    observables_payload: Dict[str, str],
    *,
    activation_phrase: str,
    material_session: Iterable[str],
    doctrine_sv: Iterable[str],
    lagunas_declarables: Optional[Iterable[str]] = None,
    required_material: Optional[str] = None,
    support_override: Optional[Dict[int, set[int]]] = None,
) -> Dict[str, Any]:
    session = build_session_declaration(
        activation_phrase=activation_phrase,
        material_session=material_session,
        doctrine_sv=doctrine_sv,
        lagunas_declarables=lagunas_declarables,
        required_material=required_material,
    )
    if not session.get("sesion_activa"):
        return {"protocolo": session, "cuerpo": None, "ESTADO_DE_SALIDA": None}

    normalized, u_d_activas = validate_observables_with_ud(observables_payload)
    obs = observables_from_dict(normalized)
    result = run_agent(obs, support_override=support_override)
    state_block = build_state_block(
        blockages=[] if not required_material else [{"causa": "material específico ausente", "pieza": required_material}],
        u_d_activas=u_d_activas,
        action_required=(f"Aportar la pieza específica: {required_material}" if required_material else None),
    )
    return {
        "protocolo": {"declaracion_de_sesion": session},
        "cuerpo": {
            "modo": "direct",
            "observables_entrada": observables_payload,
            "observables_normalizados": normalized,
            "resultado_motor": result,
        },
        "ESTADO_DE_SALIDA": state_block,
    }
