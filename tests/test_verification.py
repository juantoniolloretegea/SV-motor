from sv_motor.verification import run_nlp, compare, verify_reproducible


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
