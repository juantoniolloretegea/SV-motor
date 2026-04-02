"""Interfaz de línea de órdenes mínima del frente motor."""
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from sv_motor.protocols.ft_sv_ia import (
    ACTIVATION_PHRASE,
    render_protocol_output,
    run_direct_ft_session,
)


def _load_json_string_or_file(raw: str | None, file_path: str | None) -> dict[str, Any]:
    if raw and file_path:
        raise ValueError("Use solo una de estas opciones: --obs o --obs-file.")
    if not raw and not file_path:
        raise ValueError("Debe proporcionar --obs o --obs-file.")
    if raw:
        return json.loads(raw)
    return json.loads(Path(file_path).read_text(encoding="utf-8"))


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="sv-nlp",
        description="Demostración end-to-end local del frente motor del SV en modo direct.",
    )
    parser.add_argument("--modo", choices=["direct"], default="direct")
    parser.add_argument("--obs", help="JSON inline del paquete Ω_NLP.")
    parser.add_argument("--obs-file", help="Ruta a un archivo JSON con el paquete Ω_NLP.")
    parser.add_argument("--session-file", required=True, help="Ruta al archivo JSON de sesión FT-SV-IA/001.")
    parser.add_argument("--out", help="Ruta de salida para persistir el resultado JSON.")
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    try:
        observables_payload = _load_json_string_or_file(args.obs, args.obs_file)
        session_payload = json.loads(Path(args.session_file).read_text(encoding="utf-8"))
        output = run_direct_ft_session(
            observables_payload,
            activation_phrase=session_payload.get("activation_phrase", ACTIVATION_PHRASE),
            material_session=session_payload.get("material_session", []),
            doctrine_sv=session_payload.get("doctrine_sv", []),
            lagunas_declarables=session_payload.get("lagunas_declarables", []),
            required_material=session_payload.get("required_material"),
        )
        rendered = render_protocol_output(output)
        if args.out:
            Path(args.out).write_text(rendered + "\n", encoding="utf-8")
        print(rendered)
        proto = output.get("protocolo", {})
        if "declaracion_de_sesion" in proto:
            active = proto["declaracion_de_sesion"].get("sesion_activa", True)
        else:
            active = proto.get("sesion_activa", True)
        return 0 if active else 2
    except Exception as exc:  # pragma: no cover
        parser.exit(2, f"sv-nlp: error: {exc}\n")


if __name__ == "__main__":
    raise SystemExit(main())
