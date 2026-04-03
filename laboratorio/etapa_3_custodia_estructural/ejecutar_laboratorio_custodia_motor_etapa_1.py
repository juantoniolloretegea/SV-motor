from __future__ import annotations

import json
from collections import Counter
from pathlib import Path

from sv_motor.security.custodia_estructural import (
    custodia_observables_from_dict,
    run_custodia_motor,
    sensitive_step_is_allowed,
)

ROOT = Path(__file__).resolve().parent
INPUT = ROOT / "casos_canonicos_custodia_motor_etapa_1.json"
OUTPUT = ROOT / "salida_casos_canonicos_custodia_motor_etapa_1.json"
DICTAMEN = ROOT / "dictamen_custodia_motor_etapa_1.json"
ANALYSIS = ROOT / "analisis_resultados_custodia_motor_etapa_1.md"


def main() -> None:
    payload = json.loads(INPUT.read_text(encoding="utf-8"))
    results = []
    k3_counter = Counter()
    allow_counter = Counter()

    for case in payload["casos"]:
        obs = custodia_observables_from_dict(case["observables"])
        result = run_custodia_motor(obs)
        assert result["k3"] == case["esperado_k3"], f"{case['id']} no produjo el K3 esperado"
        allowed = sensitive_step_is_allowed(result)
        k3_counter[result["k3"]] += 1
        allow_counter["permite"] += int(allowed)
        allow_counter["bloquea_o_detiene"] += int(not allowed)
        results.append(
            {
                "id": case["id"],
                "descripcion": case["descripcion"],
                "esperado_k3": case["esperado_k3"],
                "resultado": result,
                "avance_permitido": allowed,
            }
        )

    summary = {
        "etapa": "etapa_3_custodia_estructural",
        "casos_totales": len(results),
        "distribucion_k3": {
            "APTO": k3_counter.get("APTO", 0),
            "INDETERMINADO": k3_counter.get("INDETERMINADO", 0),
            "NO_APTO": k3_counter.get("NO_APTO", 0),
        },
        "resumen_paso": dict(allow_counter),
        "dictamen": "APTO",
    }

    OUTPUT.write_text(
        json.dumps({"summary": summary, "cases": results}, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    DICTAMEN.write_text(json.dumps(summary, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    ANALYSIS.write_text(
        "# Análisis de resultados — Custodia estructural del motor\n\n"
        f"Casos totales: {summary['casos_totales']}\n\n"
        f"Distribución K3: APTO={summary['distribucion_k3']['APTO']}, "
        f"INDETERMINADO={summary['distribucion_k3']['INDETERMINADO']}, "
        f"NO_APTO={summary['distribucion_k3']['NO_APTO']}\n\n"
        f"Avances permitidos: {summary['resumen_paso'].get('permite', 0)}\n\n"
        f"Bloqueos o detenciones: {summary['resumen_paso'].get('bloquea_o_detiene', 0)}\n\n"
        "La compuerta se considera apta cuando bloquea o detiene, con trazabilidad, los movimientos que presionan el SV, el Lenguaje SV o la preservación de U.\n",
        encoding="utf-8",
    )
    print(json.dumps(summary, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
