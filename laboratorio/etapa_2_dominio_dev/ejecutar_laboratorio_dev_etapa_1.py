from __future__ import annotations

import json
from collections import Counter
from pathlib import Path

from sv_motor.algebra.dev import dev_observables_from_dict, run_dev_agent

ROOT = Path(__file__).resolve().parent
CASES_PATH = ROOT / "casos_canonicos_dev_etapa_1.json"
OUT_PATH = ROOT / "salida_casos_canonicos_dev_etapa_1.json"
ANALYSIS_PATH = ROOT / "analisis_resultados_dev_etapa_1.md"
DICT_PATH = ROOT / "dictamen_custodia_dev_etapa_1.json"


def main() -> None:
    cases = json.loads(CASES_PATH.read_text(encoding="utf-8"))
    results = []
    counter: Counter[str] = Counter()
    type_counter: Counter[str] = Counter()

    for case in cases:
        obs = dev_observables_from_dict(case["observables"])
        override = case.get("override_support")
        if override:
            override = {int(k): set(v) for k, v in override.items()}
        result = run_dev_agent(obs, support_override=override)
        result["id"] = case["id"]
        result["descripcion"] = case["descripcion"]
        assert result["k3"] == case["clase_esperada"], (case["id"], result["k3"], case["clase_esperada"])
        assert result["politica"] == case["politica_esperada"], (case["id"], result["politica"], case["politica_esperada"])
        results.append(result)
        counter[result["k3"]] += 1
        type_counter[case["tipo"]] += 1

    summary = {
        "etapa": "etapa_2_dominio_dev",
        "casos_totales": len(cases),
        "casos_dev_agent": type_counter.get("dev_agent", 0),
        "distribucion_k3": {
            "APTO": counter.get("APTO", 0),
            "INDETERMINADO": counter.get("INDETERMINADO", 0),
            "NO_APTO": counter.get("NO_APTO", 0),
        },
        "dictamen": "APTO",
    }

    payload = {"summary": summary, "results": results}
    OUT_PATH.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

    analysis = (
        "# Análisis de resultados del dominio DEV — Etapa 1\n\n"
        f"- Casos totales: {len(cases)}\n"
        f"- APTO: {counter.get('APTO', 0)}\n"
        f"- INDETERMINADO: {counter.get('INDETERMINADO', 0)}\n"
        f"- NO_APTO: {counter.get('NO_APTO', 0)}\n\n"
        "La capa generativa del dominio DEV no queda activada por este laboratorio.\n"
    )
    ANALYSIS_PATH.write_text(analysis, encoding="utf-8")

    DICT_PATH.write_text(
        json.dumps(
            {
                "etapa": "etapa_2_dominio_dev",
                "estado": "APTO",
                "deuda_viva": [
                    "La capa generativa del dominio DEV sigue deliberadamente no abierta.",
                    "El dominio DEV requiere todavía extractor específico y suite ampliada antes de cualquier activación generativa.",
                ],
            },
            ensure_ascii=False,
            indent=2,
        ) + "\n",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
