from sv_motor import (
    CustodiaMotorObservables,
    K3_APTO,
    K3_INDETERMINADO,
    K3_NO_APTO,
    run_custodia_motor,
    sensitive_step_is_allowed,
)


def _base_obs(**overrides):
    payload = {
        "anclaje_doctrinal": "anclado",
        "presion_sobre_lenguaje": "respetada",
        "frontera_ml_algebra": "preservada",
        "paridad_documento_laboratorio": "alineada",
        "preservacion_u": "preservada",
        "limites_de_fase": "respetados",
        "trazabilidad": "trazable",
        "protocolo_activo": "activo",
        "dependencia_superior_respetada": "respetada",
    }
    payload.update(overrides)
    return CustodiaMotorObservables(**payload)


def test_custodia_apta_permite_avance():
    result = run_custodia_motor(_base_obs())
    assert result["k3"] == K3_APTO
    assert sensitive_step_is_allowed(result) is True


def test_presion_sobre_lenguaje_indeterminada_bloquea():
    result = run_custodia_motor(_base_obs(presion_sobre_lenguaje="indeterminada"))
    assert result["k3"] == K3_NO_APTO
    assert 2 in result["U_irr"]
    assert sensitive_step_is_allowed(result) is False


def test_paridad_indeterminada_detiene_y_revisa():
    result = run_custodia_motor(_base_obs(paridad_documento_laboratorio="indeterminada"))
    assert result["k3"] == K3_INDETERMINADO
    assert sensitive_step_is_allowed(result) is False


def test_frontera_delegada_bloquea_avance():
    result = run_custodia_motor(_base_obs(frontera_ml_algebra="delegada"))
    assert result["k3"] == K3_NO_APTO
    assert sensitive_step_is_allowed(result) is False


def test_protocolo_indeterminado_es_irreducible():
    result = run_custodia_motor(_base_obs(protocolo_activo="indeterminada"))
    assert result["k3"] == K3_NO_APTO
    assert 8 in result["U_irr"]


def test_obligaciones_presentes_en_no_apto():
    result = run_custodia_motor(_base_obs(frontera_ml_algebra="delegada"))
    assert any(item["posicion"] == "P3" for item in result["obligaciones"])
