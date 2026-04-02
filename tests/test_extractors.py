from sv_motor.extractors.ext_nlp import extract_direct, extract


def test_extract_direct_normalizes_and_falls_back_to_indeterminacy():
    data = {
        "theta": "coherente",
        "pi": "valor-no-valido",
        "kappa": "coherente",
        "eta": "completa",
        "gamma": "alineada",
        "alpha": "apropiada",
        "mu": "sin-ambigüedad",
        "chi": "sin-solicitud",
        "psi": "fuera-de-dominio",
    }
    result = extract_direct(data)
    assert result["mu"] == "sin-ambiguedad"
    assert result["pi"] == "indeterminada"
    assert result["psi"] == "indeterminado"


def test_extract_direct_interface_matches_unified_direct_mode():
    data = {
        "theta": "coherente",
        "pi": "resuelta",
        "kappa": "coherente",
        "eta": "completa",
        "gamma": "alineada",
        "alpha": "apropiada",
        "mu": "cerrada",
        "chi": "sin-solicitud",
        "psi": "cerrado",
    }
    assert extract("", mode="direct", observables=data) == extract_direct(data)


def test_extract_unknown_mode_raises():
    try:
        extract("hola", mode="desconocido")
    except ValueError as exc:
        assert "Modo desconocido" in str(exc)
    else:
        raise AssertionError("Se esperaba ValueError para un modo desconocido")
