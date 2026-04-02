"""SV-motor: núcleo local mínimo y auditable del frente motor del SV."""

from sv_motor.algebra.core import (
    U,
    K3_APTO,
    K3_INDETERMINADO,
    K3_NO_APTO,
    threshold,
    classify_cell,
    summarize_cell,
    gate,
    gate_chain,
    gate_value,
    gate_vector,
    gamma_h_labels,
    gamma_bar_h,
    kappa3,
    resolve_policy,
)
from sv_motor.algebra.nlp import Observables, i_nlp, observables_from_dict, run_agent
from sv_motor.extractors.ext_nlp import validate_observables_dict, validate_observables_with_ud
from sv_motor.protocols.ft_sv_ia import (
    ACTIVATION_PHRASE,
    INACTIVE_MESSAGE,
    build_session_declaration,
    build_state_block,
    render_protocol_output,
    run_direct_ft_session,
)

__all__ = [
    "U",
    "K3_APTO",
    "K3_INDETERMINADO",
    "K3_NO_APTO",
    "threshold",
    "classify_cell",
    "summarize_cell",
    "gate",
    "gate_chain",
    "gate_value",
    "gate_vector",
    "gamma_h_labels",
    "gamma_bar_h",
    "kappa3",
    "resolve_policy",
    "Observables",
    "i_nlp",
    "observables_from_dict",
    "run_agent",
    "validate_observables_dict",
    "validate_observables_with_ud",
    "ACTIVATION_PHRASE",
    "INACTIVE_MESSAGE",
    "build_session_declaration",
    "build_state_block",
    "render_protocol_output",
    "run_direct_ft_session",
]
