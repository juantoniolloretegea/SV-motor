from __future__ import annotations

import json
from pathlib import Path
from collections import Counter

from sv_motor.algebra.nlp import Observables, run_agent
from sv_motor.extractors.ext_nlp import extract_direct

ROOT = Path(__file__).resolve().parent


def _load_cases() -> list[dict]:
    return json.loads((ROOT / "casos_canonicos_motor_etapa_1.json").read_text(encoding="utf-8"))


def main() -> None:
    cases = _load_cases()
    results: list[dict] = []
    counter = Counter()
    type_counter = Counter()

    for case in cases:
        if case["tipo"] == "nlp_agent":
            type_counter["nlp_agent"] += 1
            obs = Observables(**case["observables"])
            out = run_agent(obs)
            assert out["k3"] == case["clase_esperada"]
            assert out["politica"] == case["politica_esperada"]
            counter[out["k3"]] += 1
            results.append({
                "id": case["id"],
                "descripcion": case["descripcion"],
                "tipo": case["tipo"],
                "k3": out["k3"],
                "politica": out["politica"],
                "U_irr": out["U_irr"],
                "gobernable": out["gobernable"],
            })
        elif case["tipo"] == "extract_direct":
            type_counter["extract_direct"] += 1
            out = extract_direct(case["observables"])
            for field in case["campos_indeterminados_esperados"]:
                expected = "indeterminado" if field == "psi" else "indeterminada"
                assert out[field] == expected
            results.append({
                "id": case["id"],
                "descripcion": case["descripcion"],
                "tipo": case["tipo"],
                "salida": out,
            })
        else:
            raise ValueError(f"Tipo de caso no soportado: {case['tipo']}")

    (ROOT / "salida_casos_canonicos_motor_etapa_1.json").write_text(
        json.dumps(results, ensure_ascii=False, indent=2), encoding="utf-8"
    )

    summary = {
        "etapa": "etapa_1_nucleo_local",
        "casos_totales": len(cases),
        "casos_nlp_agent": type_counter.get("nlp_agent", 0),
        "casos_extract_direct": type_counter.get("extract_direct", 0),
        "distribucion_k3": {
            "APTO": counter.get("APTO", 0),
            "INDETERMINADO": counter.get("INDETERMINADO", 0),
            "NO_APTO": counter.get("NO_APTO", 0),
        },
        "dictamen": "APTO",
        "observacion": "La etapa acredita un núcleo local mínimo con preservación explícita de la U y sin cierre delegado a capas externas.",
    }
    (ROOT / "dictamen_custodia_motor_etapa_1.json").write_text(
        json.dumps(summary, ensure_ascii=False, indent=2), encoding="utf-8"
    )

    analysis = f"""# Análisis de resultados — Etapa 1 del núcleo local

Se ejecutaron {len(cases)} casos del lote canónico mínimo, con recuento derivado de los tipos efectivamente procesados.

## Resultado global

- APTO: {counter.get('APTO', 0)}
- INDETERMINADO: {counter.get('INDETERMINADO', 0)}
- NO_APTO: {counter.get('NO_APTO', 0)}

## Lectura

La etapa acredita una base mínima real del frente motor: el paquete local ejecuta, clasifica, conserva la U cuando no existe cierre legítimo y corrige entradas fuera de dominio en la capa extractora directa sin fabricar certeza.

## Límite

Este resultado no equivale a motor completo ni a integración fuerte con lenguaje, backend o NLP más allá del piloto subordinado ya fijado.
"""
    (ROOT / "analisis_resultados_motor_etapa_1.md").write_text(analysis, encoding="utf-8")

    print(json.dumps(summary, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
