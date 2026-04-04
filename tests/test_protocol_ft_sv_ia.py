from sv_motor.protocols.ft_sv_ia import (
    ACTIVATION_PHRASE,
    INACTIVE_MESSAGE,
    build_session_declaration,
    build_state_block,
    run_direct_ft_session,
)


def test_build_session_declaration_active():
    out = build_session_declaration(
        ACTIVATION_PHRASE,
        material_session=["a", "b"],
        doctrine_sv=["Pliego", "FT-SV-IA/001"],
        lagunas_declarables=["demo local"],
    )
    assert out["sesion_activa"] is True
    assert out["cabecera"] == "SESIÓN FT-SV-IA/001 ACTIVA"
    assert out["Estado operativo"] == "LISTO"


def test_build_session_declaration_inactive():
    out = build_session_declaration(
        "frase incorrecta",
        material_session=[],
        doctrine_sv=[],
    )
    assert out["sesion_activa"] is False
    assert out["mensaje"] == INACTIVE_MESSAGE


def test_build_state_block_omits_when_clean():
    assert build_state_block() is None


def test_run_direct_ft_session_declares_ud_b_when_input_is_out_of_domain():
    out = run_direct_ft_session(
        {
            "theta": "coherente",
            "pi": "valor-no-valido",
            "kappa": "coherente",
            "eta": "completa",
            "gamma": "alineada",
            "alpha": "apropiada",
            "mu": "sin-ambiguedad",
            "chi": "sin-solicitud",
            "psi": "cerrado",
        },
        activation_phrase=ACTIVATION_PHRASE,
        material_session=["entrada"],
        doctrine_sv=["Pliego", "FT-SV-IA/001"],
    )
    state = out["ESTADO_DE_SALIDA"]
    assert state is not None
    assert state["cabecera"] == "ESTADO DE SALIDA"
    assert state["U_d activas"][0]["codigo"] == "U_d(B)"


def test_run_direct_ft_session_blocks_when_custodia_is_not_apta():
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
        material_session=["entrada"],
        doctrine_sv=["Pliego", "FT-SV-IA/001"],
        custodia_observables={
            "anclaje_doctrinal": "anclado",
            "presion_sobre_lenguaje": "indeterminada",
            "frontera_ml_algebra": "preservada",
            "paridad_documento_laboratorio": "alineada",
            "preservacion_u": "preservada",
            "limites_de_fase": "respetados",
            "trazabilidad": "trazable",
            "protocolo_activo": "activo",
            "dependencia_superior_respetada": "respetada",
        },
    )
    assert out["cuerpo"]["resultado_motor"] is None
    assert out["ESTADO_DE_SALIDA"]["Bloqueos"][-1]["dictamen"] == "NO_APTO"
