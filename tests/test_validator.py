from pathlib import Path

from sv_motor.visual.validator import evaluate_svg, c2_enriched


FIXTURES = Path(__file__).parent / "fixtures"


def test_evaluate_svg_apto_fixture():
    result = evaluate_svg(FIXTURES / "figure_apta.svg")
    assert result["C_material_svg9"]["class"] == "APTO"
    assert result["metrics"]["G1_footer_presente"] == 0
    assert result["metrics"]["G9_footer_con_legibilidad_minima"] == 0


def test_evaluate_svg_defectuosa_fixture():
    result = evaluate_svg(FIXTURES / "figure_defectuosa.svg")
    assert result["C_material_svg9"]["class"] in {"INDETERMINADO", "NO_APTO"}
    assert result["metrics"]["G9_footer_con_legibilidad_minima"] != 0


def test_c2_enriched_preserves_local_gate_logic():
    result = c2_enriched(FIXTURES / "figure_apta.svg")
    assert result["A_auditoria_enriquecida"] == "APTO"
