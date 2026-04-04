import json

from sv_motor.cli import main


def test_cli_direct_mode_generates_traceable_output(tmp_path):
    obs = {
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
    session = {
        "activation_phrase": "Opera bajo FT-SV-IA/001.",
        "material_session": ["entrada-demo"],
        "doctrine_sv": ["Pliego", "FT-SV-IA/001"],
        "lagunas_declarables": [],
        "required_material": None,
    }
    obs_path = tmp_path / "obs.json"
    session_path = tmp_path / "session.json"
    out_path = tmp_path / "out.json"
    obs_path.write_text(json.dumps(obs, ensure_ascii=False), encoding="utf-8")
    session_path.write_text(json.dumps(session, ensure_ascii=False), encoding="utf-8")
    rc = main([
        "--modo", "direct",
        "--obs-file", str(obs_path),
        "--session-file", str(session_path),
        "--out", str(out_path),
    ])
    assert rc == 0
    payload = json.loads(out_path.read_text(encoding="utf-8"))
    assert payload["protocolo"]["declaracion_de_sesion"]["cabecera"] == "SESIÓN FT-SV-IA/001 ACTIVA"
    assert payload["cuerpo"]["resultado_motor"]["k3"] == "APTO"


def test_cli_requires_obs_or_obs_file(tmp_path):
    session = {
        "activation_phrase": "Opera bajo FT-SV-IA/001.",
        "material_session": ["entrada-demo"],
        "doctrine_sv": ["Pliego", "FT-SV-IA/001"],
        "lagunas_declarables": [],
        "required_material": None,
    }
    session_path = tmp_path / "session.json"
    session_path.write_text(json.dumps(session, ensure_ascii=False), encoding="utf-8")
    try:
        main(["--modo", "direct", "--session-file", str(session_path)])
    except SystemExit as exc:
        assert exc.code == 2
    else:
        raise AssertionError("Se esperaba SystemExit(2)")
