"""Compuerta ejecutable de custodia estructural del frente motor."""

from sv_motor.security.custodia_estructural import (
    CustodiaMotorObservables,
    CUSTODIA_SUPPORT_BASE,
    custodia_observables_from_dict,
    i_custodia_motor,
    resolve_custodia_policy,
    run_custodia_motor,
    sensitive_step_is_allowed,
)

__all__ = [
    "CustodiaMotorObservables",
    "CUSTODIA_SUPPORT_BASE",
    "custodia_observables_from_dict",
    "i_custodia_motor",
    "resolve_custodia_policy",
    "run_custodia_motor",
    "sensitive_step_is_allowed",
]


# SPDX-License-Identifier: CC-BY-NC-ND-4.0
# © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
# https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode.es
