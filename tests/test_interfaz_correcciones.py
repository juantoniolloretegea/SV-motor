import json
from unittest import mock

import pytest

from sv_motor.algebra.core import SVTernaryError, classify_cell, gate_value
from sv_motor.algebra.dev import dev_observables_from_dict
from sv_motor.algebra.nlp import i_nlp, Observables
from sv_motor.extractors import ext_nlp
from sv_motor.verification.py_runner import run_nlp


def test_classify_cell_accepts_string_aliases():
    assert classify_cell(["0"] * 9) == "APTO"
    assert classify_cell(["1"] * 9) == "NO_APTO"


def test_gate_value_rejects_out_of_domain_and_bool():
    with pytest.raises(SVTernaryError):
        gate_value(0, "u")
    with pytest.raises(SVTernaryError):
        gate_value(0, True)


def test_i_nlp_reports_field_name_for_invalid_value():
    with pytest.raises(ValueError) as exc:
        i_nlp(Observables(
            theta="indeterminada",
            pi="sin-pregunta",
            kappa="coherente",
            eta="completa",
            gamma="alineada",
            alpha="apropiada",
            mu="cerrada",
            chi="sin-solicitud",
            psi="cerrado",
        ))
    assert "theta" in str(exc.value)


def test_dev_observables_from_dict_is_public_entrypoint():
    obs = dev_observables_from_dict({
        "conformidad_doctrinal": "conforme",
        "suficiencia_material": "verificable",
        "trazabilidad": "trazable",
        "frontera_ml_algebra": "preservada",
        "preservacion_u": "preservada",
        "paridad_doc_artefacto": "alineada",
        "soberania_humana": "preservada",
        "protocolo_entrada": "alineado",
        "reversibilidad": "append-only",
    })
    assert obs.protocolo_entrada == "alineado"


def test_runner_emits_detected_version_shape():
    payload = run_nlp({
        "theta": "coherente",
        "pi": "resuelta",
        "kappa": "coherente",
        "eta": "completa",
        "gamma": "alineada",
        "alpha": "apropiada",
        "mu": "cerrada",
        "chi": "sin-solicitud",
        "psi": "cerrado",
    }).to_dict()
    assert payload["sv_version"]
    assert payload["sv_version"] != "0.1.5" or payload["sv_version"] == "0.0.0-dev"


def test_validate_observables_with_ud_tracks_fallbacks():
    normalized, ud = ext_nlp.validate_observables_with_ud({
        "theta": "coherente",
        "pi": "valor-no-valido",
        "kappa": "coherente",
        "eta": "completa",
        "gamma": "alineada",
        "alpha": "apropiada",
        "mu": "sin-ambigüedad",
        "chi": "sin-solicitud",
        "psi": "fuera-de-dominio",
    })
    assert normalized["mu"] == "sin-ambiguedad"
    assert normalized["pi"] == "indeterminada"
    assert ud


def test_extract_ollama_mocked(monkeypatch):
    import sys
    import types
    fake_resp = mock.Mock()
    fake_resp.json.return_value = {"message": {"content": json.dumps({
        "theta": "coherente",
        "pi": "resuelta",
        "kappa": "coherente",
        "eta": "completa",
        "gamma": "alineada",
        "alpha": "apropiada",
        "mu": "cerrada",
        "chi": "sin-solicitud",
        "psi": "cerrado",
    })}}
    fake_resp.raise_for_status.return_value = None
    fake_httpx = types.ModuleType("httpx")
    fake_httpx.post = mock.Mock(return_value=fake_resp)
    monkeypatch.setitem(sys.modules, "httpx", fake_httpx)
    out = ext_nlp.extract_ollama("hola")
    assert out["psi"] == "cerrado"


def test_extract_hf_mocked():
    fake_resp = mock.Mock()
    fake_resp.json.return_value = [{"generated_text": json.dumps({
        "theta": "coherente",
        "pi": "resuelta",
        "kappa": "coherente",
        "eta": "completa",
        "gamma": "alineada",
        "alpha": "apropiada",
        "mu": "cerrada",
        "chi": "sin-solicitud",
        "psi": "cerrado",
    })}]
    fake_resp.raise_for_status.return_value = None
    with mock.patch("requests.post", return_value=fake_resp):
        out = ext_nlp.extract_hf_api("hola")
    assert out["theta"] == "coherente"


def test_extract_anthropic_mocked():
    fake_client = mock.Mock()
    fake_message = mock.Mock()
    fake_message.content = [mock.Mock(text=json.dumps({
        "theta": "coherente",
        "pi": "resuelta",
        "kappa": "coherente",
        "eta": "completa",
        "gamma": "alineada",
        "alpha": "apropiada",
        "mu": "cerrada",
        "chi": "sin-solicitud",
        "psi": "cerrado",
    }))]
    fake_client.messages.create.return_value = fake_message
    fake_module = mock.Mock(Anthropic=mock.Mock(return_value=fake_client))
    with mock.patch.dict("sys.modules", {"anthropic": fake_module}):
        out = ext_nlp.extract_anthropic("hola", api_key="x")
    assert out["gamma"] == "alineada"


def test_classify_cell_accepts_u_string_alias():
    assert classify_cell(["U"] * 9) == "INDETERMINADO"


def test_gate_vector_and_kappa3_reject_length_mismatch():
    from sv_motor.algebra.core import gate_vector, kappa3
    with pytest.raises(ValueError):
        gate_vector([0]*9, [0]*16)
    with pytest.raises(ValueError):
        kappa3([0]*9, [0]*16)
