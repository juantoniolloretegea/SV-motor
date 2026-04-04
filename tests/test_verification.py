from sv_motor.verification import run_nlp, run_dev, run_custodia, run_custom, compare, compare_files, verify_reproducible


def test_verification_runner_emits_canonical_json_and_preserves_u():
    result = run_nlp({
        "theta": "coherente",
        "pi": "indeterminada",
        "kappa": "coherente",
        "eta": "completa",
        "gamma": "alineada",
        "alpha": "apropiada",
        "mu": "sin-ambiguedad",
        "chi": "sin-solicitud",
        "psi": "en-curso",
    })
    payload = result.to_dict()
    assert payload["engine"] == "python"
    assert payload["traza"]["C_frame"][1] == "U"
    assert payload["dictamen"]["k3"] == "INDETERMINADO"


def test_verification_compare_detects_and_locates_discrepancy():
    left = run_nlp({
        "theta": "coherente",
        "pi": "sin-pregunta",
        "kappa": "coherente",
        "eta": "completa",
        "gamma": "alineada",
        "alpha": "apropiada",
        "mu": "sin-ambiguedad",
        "chi": "sin-solicitud",
        "psi": "en-curso",
    }).json_canonical()
    right = left.replace('"APTO"', '"NO_APTO"', 1)
    comparison = compare(left, right)
    assert not comparison.verificado
    assert any(d.get('campo') == 'dictamen.k3' for d in comparison.discrepancias)


def test_verification_reproducible_passes_on_identical_json():
    payload = run_nlp({
        "theta": "coherente",
        "pi": "sin-pregunta",
        "kappa": "coherente",
        "eta": "completa",
        "gamma": "alineada",
        "alpha": "apropiada",
        "mu": "sin-ambiguedad",
        "chi": "sin-solicitud",
        "psi": "en-curso",
    }).json_canonical()
    check = verify_reproducible(payload, payload)
    assert check.verificado
    assert check.discrepancias == []


def test_verification_run_dev_and_run_custodia_emit_obligaciones():
    dev = run_dev({
        "conformidad_doctrinal": "conforme",
        "suficiencia_material": "verificable",
        "trazabilidad": "trazable",
        "frontera_ml_algebra": "preservada",
        "preservacion_u": "preservada",
        "paridad_doc_artefacto": "alineada",
        "soberania_humana": "preservada",
        "protocolo_entrada": "alineado",
        "reversibilidad": "append-only",
    }).to_dict()
    cust = run_custodia({
        "anclaje_doctrinal": "anclado",
        "presion_sobre_lenguaje": "respetada",
        "frontera_ml_algebra": "preservada",
        "paridad_documento_laboratorio": "alineada",
        "preservacion_u": "preservada",
        "limites_de_fase": "respetados",
        "trazabilidad": "trazable",
        "protocolo_activo": "activo",
        "dependencia_superior_respetada": "respetada",
    }).to_dict()
    assert dev["dictamen"]["obligaciones"] == []
    assert cust["dictamen"]["obligaciones"] == []


def test_verification_run_custom_validates_cell_size():
    import pytest
    with pytest.raises(Exception):
        run_custom([0]*10, {i: {0,1} for i in range(1,11)})


def test_compare_files_reads_from_disk(tmp_path):
    payload = run_nlp({
        "theta": "coherente",
        "pi": "sin-pregunta",
        "kappa": "coherente",
        "eta": "completa",
        "gamma": "alineada",
        "alpha": "apropiada",
        "mu": "sin-ambiguedad",
        "chi": "sin-solicitud",
        "psi": "en-curso",
    }).json_canonical()
    p1 = tmp_path / "a.json"
    p2 = tmp_path / "b.json"
    p1.write_text(payload, encoding="utf-8")
    p2.write_text(payload, encoding="utf-8")
    cmp = compare_files(str(p1), str(p2))
    assert cmp.verificado


def test_comparison_result_serialization_and_optional_obligations():
    left = {
        "engine": "python",
        "domain": "DEV",
        "traza": {
            "C_frame": [0]*9,
            "gamma_h_labels": {},
            "C_gob": [0]*9,
            "A_agente": [0]*9,
            "U_irr": [],
            "gobernable": True,
        },
        "dictamen": {"k3": "APTO", "politica": "CERRAR_FRAME", "obligaciones": []},
    }
    right = {
        "engine": "rust",
        "domain": "DEV",
        "traza": {
            "C_frame": [0]*9,
            "gamma_h_labels": {},
            "C_gob": [0]*9,
            "A_agente": [0]*9,
            "U_irr": [],
            "gobernable": True,
        },
        "dictamen": {"k3": "APTO", "politica": "CERRAR_FRAME", "obligaciones": [{"p": 1}]},
    }
    cmp = compare(left, right)
    assert not cmp.verificado
    payload = cmp.to_dict()
    assert payload["dictamen_comparacion"] == "DOBLE_VARA_FAIL"
    assert "DOBLE_VARA_FAIL" in cmp.to_json()


def test_detect_version_fallback_reads_pyproject(monkeypatch):
    import importlib.metadata as im
    from sv_motor.verification import py_runner as runner

    def _raise(*args, **kwargs):
        raise im.PackageNotFoundError("sv-motor")

    monkeypatch.setattr(im, "version", _raise)
    result = runner._detect_version()
    assert result == "0.1.8"


def test_run_custom_n16_extended():
    support = {i: {0, 1} for i in range(1, 17)}
    vector = [0] * 16
    result = run_custom(vector, support, domain_name="CUSTOM_N16", observables_raw={"origen": "test_hito5"})
    payload = result.to_dict()
    assert payload["domain"] == "CUSTOM_N16"
    assert payload["dictamen"]["k3"] == "APTO"
    assert payload["traza"]["gobernable"] is True
