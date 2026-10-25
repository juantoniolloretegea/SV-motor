import pytest

from sv_motor import (
    U,
    K3_APTO,
    K3_INDETERMINADO,
    K3_NO_APTO,
    SVTernaryError,
    threshold,
    validate_cell_size,
    classify_cell,
    gate,
    gate_vector,
    run_agent,
    Observables,
    ACTIVATION_PHRASE,
    run_direct_ft_session,
    DevObservables,
    run_dev_agent,
)


def test_public_api_exports_minimos():
    assert U == "U"
    assert K3_APTO == "APTO"
    assert K3_INDETERMINADO == "INDETERMINADO"
    assert K3_NO_APTO == "NO_APTO"
    assert threshold(9) == 7
    assert validate_cell_size(9) == 3
    assert classify_cell([0] * 9) == K3_APTO
    assert gate(K3_APTO, K3_INDETERMINADO) == K3_INDETERMINADO
    assert gate_vector([0]*9, [0,1,0,0,0,0,0,0,0]) == [0,1,0,0,0,0,0,0,0]
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


def test_public_api_exports_protocol_minimos():
    assert ACTIVATION_PHRASE == "Opera bajo FT-SV-IA/001."
    out = run_direct_ft_session(
        {
            "theta": "coherente",
            "pi": "resuelta",
            "kappa": "coherente",
            "eta": "completa",
            "gamma": "alineada",
            "alpha": "apropiada",
            "mu": "cerrada",
            "chi": "sin-solicitud",
            "psi": "cerrado",
        },
        activation_phrase=ACTIVATION_PHRASE,
        material_session=["entrada-demo"],
        doctrine_sv=["Pliego", "FT-SV-IA/001"],
    )
    assert out["cuerpo"]["resultado_motor"]["k3"] == K3_APTO



def test_public_api_exports_dev_minimos():
    obs = DevObservables(
        conformidad_doctrinal="conforme",
        suficiencia_material="verificable",
        trazabilidad="trazable",
        frontera_ml_algebra="preservada",
        preservacion_u="preservada",
        paridad_doc_artefacto="alineada",
        soberania_humana="preservada",
        protocolo_entrada="alineado",
        reversibilidad="append-only",
    )
    out = run_dev_agent(obs)
    assert out["k3"] == K3_APTO


def test_public_api_rejects_out_of_domain_value():
    with pytest.raises(SVTernaryError):
        classify_cell([0]*8 + ["u"])


# SPDX-License-Identifier: CC-BY-NC-ND-4.0
# © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
# https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode.es
