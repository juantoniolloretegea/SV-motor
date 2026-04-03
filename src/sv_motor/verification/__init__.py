"""
sv_motor.verification
======================
Capa de verificación Python del Sistema Vectorial SV.

El runner Python produce JSON canónico idéntico en estructura al que
producirá el backend Rust/.svp. El comparador contrasta ambas salidas
campo a campo para la doble vara de verificación.

Uso mínimo:
    from sv_motor.verification import run_nlp, run_dev, run_custom, compare

    result = run_nlp({
        "theta": "coherente", "pi": "sin-pregunta", "kappa": "coherente",
        "eta": "completa", "gamma": "alineada", "alpha": "apropiada",
        "mu": "sin-ambiguedad", "chi": "sin-solicitud", "psi": "en-curso",
    })
    print(result.json_canonical())

    # Doble vara con salida Rust:
    # verification = compare(result.json_canonical(), rust_json_string)
    # assert verification.verificado
"""
from sv_motor.verification.py_runner import (
    SVProgramResult,
    run_nlp,
    run_dev,
    run_custodia,
    run_custom,
)
from sv_motor.verification.comparator import (
    ComparisonResult,
    compare,
    compare_files,
    verify_reproducible,
)

__all__ = [
    "SVProgramResult",
    "run_nlp",
    "run_dev",
    "run_custodia",
    "run_custom",
    "ComparisonResult",
    "compare",
    "compare_files",
    "verify_reproducible",
]
