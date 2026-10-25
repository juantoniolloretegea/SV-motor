"""Compuertas de protocolo y salida verificable del frente motor."""

from sv_motor.protocols.ft_sv_ia import (
    ACTIVATION_PHRASE,
    INACTIVE_MESSAGE,
    build_session_declaration,
    build_state_block,
    render_protocol_output,
    run_direct_ft_session,
)

__all__ = [
    "ACTIVATION_PHRASE",
    "INACTIVE_MESSAGE",
    "build_session_declaration",
    "build_state_block",
    "render_protocol_output",
    "run_direct_ft_session",
]


# SPDX-License-Identifier: CC-BY-NC-ND-4.0
# © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
# https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode.es
