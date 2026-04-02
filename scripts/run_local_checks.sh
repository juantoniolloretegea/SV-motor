#!/usr/bin/env bash
set -euo pipefail
python -m py_compile   src/sv_motor/algebra/core.py   src/sv_motor/algebra/nlp.py   src/sv_motor/visual/validator.py   src/sv_motor/extractors/ext_nlp.py   src/sv_motor/protocols/ft_sv_ia.py   src/sv_motor/cli.py   tests/test_algebra.py   tests/test_extractors.py   tests/test_validator.py   tests/test_protocol_ft_sv_ia.py   tests/test_cli.py   laboratorio/etapa_1_nucleo_local/ejecutar_laboratorio_motor_etapa_1.py
PYTHONPATH=src pytest -q
python laboratorio/etapa_1_nucleo_local/ejecutar_laboratorio_motor_etapa_1.py

python laboratorio/etapa_0_demostracion_local/ejecutar_demo_end_to_end_local.py
