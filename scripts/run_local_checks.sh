#!/usr/bin/env bash
set -euo pipefail
python -m py_compile   src/sv_motor/algebra/core.py   src/sv_motor/algebra/nlp.py   src/sv_motor/visual/validator.py   src/sv_motor/extractors/ext_nlp.py   tests/test_algebra.py   tests/test_extractors.py   tests/test_validator.py   laboratorio/etapa_1_nucleo_local/ejecutar_laboratorio_motor_etapa_1.py
PYTHONPATH=src pytest -q
python laboratorio/etapa_1_nucleo_local/ejecutar_laboratorio_motor_etapa_1.py
