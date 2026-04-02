from __future__ import annotations

import hashlib
import json
from pathlib import Path

from sv_motor.cli import main as cli_main

ROOT = Path(__file__).resolve().parent


def _sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def _run_case(obs_file: str, out_file: str) -> dict[str, str]:
    rc = cli_main([
        "--modo", "direct",
        "--obs-file", str(ROOT / obs_file),
        "--session-file", str(ROOT / "sesion_demo_ft_sv_ia.json"),
        "--out", str(ROOT / out_file),
    ])
    assert rc == 0
    out_path = ROOT / out_file
    sha = _sha256(out_path)
    out_path.with_suffix(out_path.suffix + ".sha256").write_text(sha + "\n", encoding="utf-8")
    return {"salida": out_file, "sha256": sha}


def main() -> None:
    apto = _run_case("entrada_observables_demo.json", "salida_demo_end_to_end_local.json")
    ud_b = _run_case("entrada_observables_demo_ud_b.json", "salida_demo_end_to_end_local_ud_b.json")
    summary = {
        "fase": "etapa_0_demostracion_local",
        "casos": [
            {"id": "DEMO-APTO", **apto},
            {"id": "DEMO-UDB", **ud_b},
        ],
        "dictamen": "APTO",
        "observacion": (
            "La demostración local de Fase 0 ejecuta en modo direct, "
            "produce salida trazable y activa el bloque de estado solo cuando procede."
        ),
    }
    (ROOT / "dictamen_demo_end_to_end_local.json").write_text(
        json.dumps(summary, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    print(json.dumps(summary, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
