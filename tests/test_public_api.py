from sv_motor import (
    U,
    K3_APTO,
    K3_INDETERMINADO,
    K3_NO_APTO,
    threshold,
    classify_cell,
    gate,
    gate_vector,
    run_agent,
    Observables,
)


def test_public_api_exports_minimos():
    assert U == "U"
    assert K3_APTO == "APTO"
    assert K3_INDETERMINADO == "INDETERMINADO"
    assert K3_NO_APTO == "NO_APTO"
    assert threshold(9) == 7
    assert classify_cell([0] * 9) == K3_APTO
    assert gate(K3_APTO, K3_INDETERMINADO) == K3_INDETERMINADO
    assert gate_vector([0, U], [0, 1]) == [0, 1]
    obs = Observables(
        theta="coherente",
        pi="resuelta",
        kappa="coherente",
        eta="completa",
        gamma="alineada",
        alpha="apropiada",
        mu="cerrada",
        chi="sin-solicitud",
        psi="cerrado",
    )
    out = run_agent(obs)
    assert out["k3"] == K3_APTO
