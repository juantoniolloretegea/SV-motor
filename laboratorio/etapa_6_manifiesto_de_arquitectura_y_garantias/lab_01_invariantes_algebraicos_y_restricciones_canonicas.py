#!/usr/bin/env python3
"""
lab_01_invariantes_algebraicos_y_restricciones_canonicas.py
=====================================
Laboratorio de verificación independiente del Sistema Vectorial SV.
Verifica los invariantes algebraicos fundamentales del motor normativo.

Autor:     Juan Antonio Lloret Egea
ORCID:     0000-0002-6634-3351
ITVIA:     IA eñ™ — https://www.itvia.online
ISSN:      2695-6411
Licencia:  CC BY-NC-ND 4.0
Titularidad y autoría: © Juan Antonio Lloret Egea, 2026.
Fecha:     4 de abril de 2026
Fecha y Versión: V.1 del conjunto
Versión:   V.1 del conjunto

Uso:
    python -m pip install -e .[dev]
    python lab_01_invariantes_algebraicos_y_restricciones_canonicas.py

Resultado esperado: 0 tests fallidos.
Si hay fallos, la implementación no es conforme con el corpus doctrinal del SV.

Referencia doctrinal:
    Lloret Egea JA. Fundamentos algebraico-semánticos del Sistema Vectorial SV.
    Release 3. ITVIA, 2026. ISSN: 2695-6411.
    https://www.itvia.online/pub/fundamentos-algebraico-semanticos-del-sistema-vectorial-sv/release/3
"""

import sys
import inspect

# ── Importaciones del motor ────────────────────────────────────────────────────
try:
    from sv_motor.algebra.core import (
        U, K3_APTO, K3_INDETERMINADO, K3_NO_APTO,
        SVTernaryError,
        threshold, validate_cell_size, normalize_vector,
        classify_cell, summarize_cell,
        gate_value, gate_vector,
        gamma_h_labels, gamma_bar_h,
        kappa3,
    )
except ImportError as e:
    print(f"\n[ERROR] Motor SV no disponible: {e}")
    print("Instalar con: python -m pip install -e .[dev]")
    sys.exit(1)

# ── Utilidad de test ──────────────────────────────────────────────────────────
_PASS = _FAIL = 0

def check(desc: str, cond: bool) -> None:
    global _PASS, _FAIL
    if cond:
        print(f"  [OK]   {desc}")
        _PASS += 1
    else:
        print(f"  [FAIL] {desc}")
        _FAIL += 1


# ═════════════════════════════════════════════════════════════════════════════
# TEST 1 — Umbral canónico T(n) = ⌊7n/9⌋
# Referencia: Fundamentos §3 — Proposición 1
# ═════════════════════════════════════════════════════════════════════════════
print("\n=== TEST 1: Umbral canónico T(n) = floor(7n/9) ===")

TABLA_THRESHOLD = {9: 7, 16: 12, 25: 19, 36: 28, 49: 38, 64: 49}
for n, t_exp in TABLA_THRESHOLD.items():
    t_calc = threshold(n)
    check(f"T({n:2d}) = {t_exp}  [floor(7·{n}/9) = floor({7*n/9:.4f})]",
          t_calc == t_exp and t_calc == (7 * n) // 9)


# ═════════════════════════════════════════════════════════════════════════════
# TEST 2 — Restricción de tamaño n = b², b ≥ 3
# Referencia: Fundamentos §2 — Definición de célula
# ═════════════════════════════════════════════════════════════════════════════
print("\n=== TEST 2: Restricción de tamaño n = b², b ≥ 3 ===")

for n in [9, 16, 25, 36, 49, 64]:
    try:
        b = validate_cell_size(n)
        check(f"n={n:2d} aceptado  (b={b})", True)
    except SVTernaryError:
        check(f"n={n:2d} aceptado", False)

for n in [1, 4, 7, 8, 10, 15, 17, 24, 26, 80]:
    try:
        validate_cell_size(n)
        check(f"n={n:3d} rechazado (DEBE rechazarse)", False)
    except SVTernaryError:
        check(f"n={n:3d} rechazado correctamente", True)


# ═════════════════════════════════════════════════════════════════════════════
# TEST 3 — Integridad del alfabeto Σ = {0, 1, U}
# Referencia: FRONTERA_NORMATIVA D.1 — "Ninguna operación puede convertir U
# silenciosamente en 0, 1, false, null, NaN, -1 ni equivalente."
# ═════════════════════════════════════════════════════════════════════════════
print("\n=== TEST 3: Integridad del alfabeto Σ = {0, 1, U} ===")

for v in [0, 1, U, "0", "1", "U"]:
    try:
        r = normalize_vector([v])[0]
        check(f"Valor {v!r:5s} admitido → {r!r}", True)
    except SVTernaryError:
        check(f"Valor {v!r:5s} admitido", False)

for v in [True, False, None, "u", "U_", 2, -1, 0.5, float("nan")]:
    try:
        normalize_vector([v])
        check(f"Valor {v!r:8s} rechazado (DEBE rechazarse)", False)
    except (SVTernaryError, TypeError):
        check(f"Valor {v!r:8s} rechazado correctamente", True)

# U no colapsa tras normalize_vector
v_u = [0, U, 1, U, 0, 0, 0, 0, 0]
norm = normalize_vector(v_u)
check("U en posición 2 se preserva tras normalize_vector", norm[1] == U)
check("U en posición 4 se preserva tras normalize_vector", norm[3] == U)

# U no colapsa a APTO/NO_APTO
check("Vector todo-U → INDETERMINADO (U no colapsa)",
      classify_cell([U] * 9) == K3_INDETERMINADO)


# ═════════════════════════════════════════════════════════════════════════════
# TEST 4 — Clasificación con precedencia desfavorable
# Referencia: Fundamentos §3 — Proposición 6 (unicidad de clasificación fuerte)
# ═════════════════════════════════════════════════════════════════════════════
print("\n=== TEST 4: Clasificación determinista con precedencia desfavorable ===")

CASOS = [
    # (vector, clase_esperada, descripción)
    ([1]*7 + [0]*2,     K3_NO_APTO,      "n=9: N₁=7=T(9) → NO_APTO"),
    ([0]*9,             K3_APTO,         "n=9: N₀=9 → APTO"),
    ([1]*9,             K3_NO_APTO,      "n=9: N₁=9 → NO_APTO"),
    ([U]*9,             K3_INDETERMINADO,"n=9: N_U=9 → INDETERMINADO"),
    ([0]*7+[1]*2,       K3_APTO,         "n=9: N₀=7=T, N₁=2<T → APTO"),
    ([0]*19+[1]*6,      K3_APTO,         "n=25: N₀=19=T, N₁=6<T → APTO"),
    ([0]*19+[U]*6,      K3_APTO,         "n=25: N₀=19=T, N_U=6, N₁=0 → APTO"),
    ([1]*19+[0]*6,      K3_NO_APTO,      "n=25: N₁=19=T → NO_APTO"),
]

for v, exp, desc in CASOS:
    # Para los casos mixtos, usamos classify_cell (sin horizonte)
    res = classify_cell(v)
    check(f"{desc}: {res}", res == exp)

# Determinismo: 100 llamadas idénticas → mismo resultado
v_test = [0]*3 + [1]*2 + [U]*4
r0 = classify_cell(v_test)
check("100 llamadas idénticas → mismo resultado (determinismo)",
      all(classify_cell(v_test) == r0 for _ in range(100)))


# ═════════════════════════════════════════════════════════════════════════════
# TEST 5 — Precedencia desfavorable: NO_APTO antes que APTO
# Prueba que los dos casos son mutuamente excluyentes (Proposición 6)
# ═════════════════════════════════════════════════════════════════════════════
print("\n=== TEST 5: Precedencia desfavorable — mutua exclusividad ===")

# N₁=T implica N₀ < T (Proposición 6)
for n in [9, 25, 36]:
    t = threshold(n)
    # Construir vector con exactamente N₁=t y resto 0
    v = [1]*t + [0]*(n-t)
    res = classify_cell(v)
    n0 = v.count(0)
    check(
        f"n={n}, N₁=T={t}, N₀={n0}: N₀ {'≥' if n0>=t else '<'} T → {res}",
        res == K3_NO_APTO
    )


# ═════════════════════════════════════════════════════════════════════════════
# TEST 6 — Compuerta conservadora gate_value
# Referencia: Fundamentos — 1 domina; U domina sobre 0
# ═════════════════════════════════════════════════════════════════════════════
print("\n=== TEST 6: Compuerta conservadora gate_value ===")

TABLA_GATE = [
    (0,  0,  0,  "0 ⊕ 0 = 0"),
    (0,  1,  1,  "0 ⊕ 1 = 1  (1 domina)"),
    (1,  0,  1,  "1 ⊕ 0 = 1  (1 domina)"),
    (1,  1,  1,  "1 ⊕ 1 = 1"),
    (0,  U,  U,  "0 ⊕ U = U  (U domina sobre 0)"),
    (U,  0,  U,  "U ⊕ 0 = U"),
    (1,  U,  1,  "1 ⊕ U = 1  (1 domina sobre U)"),
    (U,  1,  1,  "U ⊕ 1 = 1"),
    (U,  U,  U,  "U ⊕ U = U"),
]
for a, b, exp, desc in TABLA_GATE:
    check(f"gate({a!r}, {b!r}) = {exp!r}  — {desc}", gate_value(a, b) == exp)


# ═════════════════════════════════════════════════════════════════════════════
# TEST 7 — Posiciones irreducibles → 1 en gamma_bar_h
# Referencia: FRONTERA_NORMATIVA B.6 — defecto desfavorable
# ═════════════════════════════════════════════════════════════════════════════
print("\n=== TEST 7: Posiciones irreducibles → 1 (defecto desfavorable) ===")

v = [0, U, U, 0, 0, 0, 0, 0, 0]  # n=9
H = {2: set(), 3: {0, 1}}          # pos 2 irreducible, pos 3 resoluble

labels = gamma_h_labels(v, H)
C_gob  = gamma_bar_h(v, H)

check("Posición 2 (H=∅) → 'irreducible'",   labels.get(2) == "irreducible")
check("Posición 3 (H≠∅) → 'fronteriza'/'resoluble'",
      labels.get(3) in ("resoluble", "fronteriza"))
check("C_gob[1] (pos 2, irreducible) = 1",  C_gob[1] == 1)
check("C_gob[2] (pos 3, resoluble) = U",    C_gob[2] == U)
check("C_gob[0] (pos 1, valor 0) = 0",      C_gob[0] == 0)


# ═════════════════════════════════════════════════════════════════════════════
# TEST 8 — Tests adversariales: intentos de violar invariantes
# Referencia: FRONTERA_NORMATIVA D.1, D.9 — coerciones prohibidas
# ═════════════════════════════════════════════════════════════════════════════
print("\n=== TEST 8: Tests adversariales ===")

# Probabilidades rechazadas
for p in [0.7, 0.51, 0.99, 0.0001]:
    try:
        normalize_vector([p])
        check(f"Probabilidad {p} rechazada (DEBE rechazarse)", False)
    except SVTernaryError:
        check(f"Probabilidad {p} rechazada correctamente", True)

# gate(0, U) debe producir U, no 0
check("gate(0, U) = U  — U no se convierte en 0 por compuerta",
      gate_value(0, U) == U)

# Tamaños inválidos rechazados
for n_bad in [10, 15, 20, 24, 30]:
    try:
        classify_cell([0]*n_bad)
        check(f"Tamaño n={n_bad} rechazado (DEBE rechazarse)", False)
    except SVTernaryError:
        check(f"Tamaño n={n_bad} rechazado correctamente", True)

# threshold() no acepta parámetros externos
src = inspect.getsource(threshold)
check("threshold() no tiene parámetros de override externos",
      "override" not in src.lower() and src.count("def ") == 1)


# ═════════════════════════════════════════════════════════════════════════════
# RESUMEN FINAL
# ═════════════════════════════════════════════════════════════════════════════
print(f"\n{'='*65}")
print(f"  RESULTADO: {_PASS} tests OK  |  {_FAIL} tests FALLIDOS")
if _FAIL == 0:
    print("  [CONFORME] El motor algebraico cumple los invariantes del SV.")
    print("  [CONFORME] No se detectaron violaciones del corpus doctrinal.")
else:
    print("  [NO CONFORME] Revisar la implementación del motor.")
print(f"{'='*65}\n")

sys.exit(0 if _FAIL == 0 else 1)
