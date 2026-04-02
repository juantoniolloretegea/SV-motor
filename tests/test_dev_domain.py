import pytest

from sv_motor.algebra.core import U, K3_APTO, K3_INDETERMINADO, K3_NO_APTO, threshold
from sv_motor.algebra.dev import (
    DevObservables,
    i_dev,
    run_dev_agent,
    resolve_dev_policy,
    build_dev_obligations,
)


def _dev(**kw):
    defaults = dict(
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
    defaults.update(kw)
    return DevObservables(**defaults)


def test_i_dev_apto():
    obs = _dev()
    assert i_dev(obs) == [0] * 9


def test_i_dev_indeterminado():
    obs = _dev(trazabilidad="indeterminada")
    assert i_dev(obs) == [0, 0, U, 0, 0, 0, 0, 0, 0]


def test_run_dev_agent_apto():
    result = run_dev_agent(_dev())
    assert result["k3"] == K3_APTO
    assert result["politica"] == "CIERRE_TECNICO_PENDIENTE_DE_HS"


def test_run_dev_agent_false_closure_blocked():
    obs = _dev(conformidad_doctrinal="indeterminada", preservacion_u="indeterminada")
    result = run_dev_agent(obs)
    c_dev = result["C_dev9"]
    assert sum(v == 0 for v in c_dev) >= threshold(len(c_dev))
    assert result["k3"] == K3_INDETERMINADO


def test_run_dev_agent_no_apto_por_frontera_delegada():
    obs = _dev(frontera_ml_algebra="delegada")
    result = run_dev_agent(obs)
    assert result["k3"] == K3_NO_APTO
    assert result["politica"] == "PROPONER_FORK"


def test_run_dev_agent_irreducible_via_override():
    obs = _dev(protocolo_entrada="indeterminada")
    result = run_dev_agent(obs, support_override={8: set()})
    assert result["k3"] == K3_NO_APTO
    assert 8 in result["U_irr"]


def test_build_dev_obligations_nonempty():
    obs = _dev(trazabilidad="indeterminada")
    result = run_dev_agent(obs)
    assert result["obligaciones"]
    assert any(item["posicion"] == "P3" for item in result["obligaciones"])
