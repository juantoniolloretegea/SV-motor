#!/usr/bin/env bash
set -euo pipefail
python -m py_compile   src/sv_motor/algebra/core.py   src/sv_motor/algebra/nlp.py   src/sv_motor/visual/validator.py   src/sv_motor/extractors/ext_nlp.py   src/sv_motor/protocols/ft_sv_ia.py   src/sv_motor/security/custodia_estructural.py   src/sv_motor/verification/py_runner.py   src/sv_motor/verification/comparator.py   src/sv_motor/cli.py   tests/test_algebra.py   tests/test_extractors.py   tests/test_validator.py   tests/test_protocol_ft_sv_ia.py   tests/test_cli.py   tests/test_custodia_estructural.py   tests/test_verification.py   laboratorio/etapa_1_nucleo_local/ejecutar_laboratorio_motor_etapa_1.py
PYTHONPATH=src pytest -q
python laboratorio/etapa_1_nucleo_local/ejecutar_laboratorio_motor_etapa_1.py

python laboratorio/etapa_0_demostracion_local/ejecutar_demo_end_to_end_local.py

python laboratorio/etapa_3_custodia_estructural/ejecutar_laboratorio_custodia_motor_etapa_1.py

python laboratorio/etapa_5_verificacion_python/ejecutar_laboratorio_verificacion.py
