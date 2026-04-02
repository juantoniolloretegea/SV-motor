#!/usr/bin/env bash
set -euo pipefail
python -m py_compile src/sv_motor/algebra/core.py src/sv_motor/algebra/nlp.py src/sv_motor/visual/validator.py src/sv_motor/extractors/ext_nlp.py tests/test_algebra.py
PYTHONPATH=src pytest -q tests/test_algebra.py
