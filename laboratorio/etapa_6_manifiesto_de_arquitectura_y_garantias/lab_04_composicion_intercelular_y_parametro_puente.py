#!/usr/bin/env python3
"""
lab_04_composicion_intercelular_y_parametro_puente.py
=====================================
Verificación del compositor intercelular IMMUNO-1 → IMMUNO-2.
Prueba la transmisión en serie por parámetro puente (Doc I del corpus).

Autor:     Juan Antonio Lloret Egea
ORCID:     0000-0002-6634-3351
ITVIA:     IA eñ™ — https://www.itvia.online
ISSN:      2695-6411
Licencia:  CC BY-NC-ND 4.0
Titularidad y autoría: © Juan Antonio Lloret Egea, 2026.
Fecha:     4 de abril de 2026
Fecha y Versión: V.1 del conjunto
Versión:   V.1 del conjunto

Referencia doctrinal:
    Lloret Egea JA. Álgebra de composición intercelular del marco SV —
    I. Transmisión en serie por parámetro puente. v1, Release 4. ITVIA, 2026.
    https://www.itvia.online/pub/algebra-de-composicion-intercelular-del-marco-sv/release/4

El conector φ_{IMMUNO-1 → IMMUNO-2}^{(P25)} implementa la función:
    κ₃(IMMUNO-1) = APTO          →  P25 de IMMUNO-2 = 0
    κ₃(IMMUNO-1) = NO_APTO       →  P25 de IMMUNO-2 = 1
    κ₃(IMMUNO-1) = INDETERMINADO →  P25 de IMMUNO-2 = U  (propagación honesta)

Propiedad fundamental: la incertidumbre se propaga honestamente.
IMMUNO-1 INDETERMINADO → P25 = U  (no inventa certeza).

Uso:
    # Desde la raíz de SV-motor-main basta con ejecutar:
    python laboratorio/etapa_6_manifiesto_de_arquitectura_y_garantias/lab_04_composicion_intercelular_y_parametro_puente.py
    # Para forzar motores externos o alternativos:
    export SV_IMMUNO1_ENGINE=./SVperitus-dataset-main/agentes/inmunologia/fase_1/src
    export SV_IMMUNO2_ENGINE=./SVperitus-dataset-main/agentes/inmunologia/fase_2/src
"""

import sys, os, importlib.util
from pathlib import Path

_ROOT = Path(__file__).resolve().parents[3]
E1 = os.environ.get("SV_IMMUNO1_ENGINE",
     str((_ROOT / "SVperitus-dataset-main/agentes/inmunologia/fase_1/src").resolve()))
E2 = os.environ.get("SV_IMMUNO2_ENGINE",
     str((_ROOT / "SVperitus-dataset-main/agentes/inmunologia/fase_2/src").resolve()))

def _load(name, path, filename="normative_engine.py"):
    spec = importlib.util.spec_from_file_location(name, os.path.join(path, filename))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m

try:
    ne1 = _load("ne_i1", E1)
    ne2 = _load("ne_i2", E2)
except Exception as e:
    print(f"\n[ERROR] Motores no disponibles: {e}")
    print(f"Rutas por defecto utilizadas: E1={E1} ; E2={E2}")
    print("Configurar alternativamente: export SV_IMMUNO1_ENGINE=... SV_IMMUNO2_ENGINE=...")
    sys.exit(1)

_PASS = _FAIL = 0

def check(desc, cond):
    global _PASS, _FAIL
    if cond:
        print(f"  [OK]   {desc}")
        _PASS += 1
    else:
        print(f"  [FAIL] {desc}")
        _FAIL += 1


# ─── Conector φ: propagación honesta de incertidumbre ─────────────────────────
print("\n=== Conector φ_{IMMUNO-1 → IMMUNO-2}: propagación de κ₃ ===")

check("κ₃(I1) = APTO → P25(I2) = 0  (estado adecuado propagado)",
      ne2.P25({"immuno1_class": "APTO"}) == "0")
check("κ₃(I1) = NO_APTO → P25(I2) = 1  (riesgo propagado)",
      ne2.P25({"immuno1_class": "NO_APTO"}) == "1")
check("κ₃(I1) = INDETERMINADO → P25(I2) = U  (incertidumbre propagada honestamente)",
      ne2.P25({"immuno1_class": "INDETERMINADO"}) == "U")
check("IMMUNO-1 no aplicado → P25 = U  (ausencia = U, no se imputa)",
      ne2.P25({}) == "U")


# ─── Composición en dos pasos ─────────────────────────────────────────────────
print("\n=== Composición en dos pasos: IMMUNO-1 → IMMUNO-2 ===")

# Caso: paciente de hematología que también recibe IS convencional
caso_i1 = {
    "anc_actual": 1200,
    "anc_expected_nadir": 80,
    "neutropenia_duration_days_expected": 21,
    "cd4_count": 620,
    "lymphocyte_total": 1800,
    "t_cell_depleting_therapy_12m": False,
    "igg_mgdl": 820,
    "bacterial_infections_severe_12m": 0,
    "anatomic_asplenia": False,
    "functional_hyposplenism": False,
    "encapsulated_vaccines_complete": None,
    "asplenia_prophylaxis": None,
    "mucositis_grade": 0,
    "skin_ulcers_relevant": False,
    "cvc_present": True,
    "cvc_complications": False,
    "hematology_phase": "induction",
    "chemo_intensity": "high",
    "expected_myelosuppression_grade": 4,
    "anti_cd20_therapy": False,
    "btk_inhibitor": False,
    "venetoclax": False,
    "sct_car_t_6m": False,
    "systemic_corticosteroids": True,
    "corticosteroids_dose_mgd": 20,
    "severe_bacterial_infections_12m": 0,
    "invasive_fungal_infection_history": False,
    "chronic_viral_infections": False,
    "mdr_colonization": False,
    "recent_hospitalization_30d": True,
    "flu_vaccine_current_season": False,
    "pneumococcal_pcv_years": 3,
    "covid19_booster_12m": True,
    "hbsab_positive": True,
    "other_inactivated_vaccines_complete": None,
    "pjp_prophylaxis": False,
    "antiviral_prophylaxis": False,
    "antifungal_prophylaxis": False,
    "antibacterial_prophylaxis_neutropenia": False,
    "infectious_risk_review_plan": False,
}

# Paso 1: evaluar IMMUNO-1
r1 = ne1.explain(caso_i1)
k3_i1 = r1["class"]

print(f"\n  Paso 1 — IMMUNO-1:")
print(f"    N₀={r1['counts']['n0']}, N₁={r1['counts']['n1']}, N_U={r1['counts']['nU']}")
print(f"    κ₃(IMMUNO-1) = {k3_i1}")

check(f"IMMUNO-1 produce {k3_i1}",
      k3_i1 in ("APTO", "NO_APTO", "INDETERMINADO"))

# Paso 2: inyectar en IMMUNO-2
caso_i2 = {
    "age": 58,
    "dm_complicated": False, "hba1c": 5.8,
    "heart_failure_nyha": 0, "egfr": 78, "ischemic_heart_event_12m": False,
    "fev1_percent_predicted": 88, "ild_diagnosed": False,
    "bronchiectasis": False, "respiratory_exacerbation_12m": False,
    "fibrosis_stage": 0, "cirrhosis": False,
    "hbv_chronic_active": False, "hcv_viremia": False,
    "bmi": 26.0, "albumin": 3.9, "frailty_fried": 0,
    "main_is_drug": "conventional", "is_combination": "monotherapy",
    "prednisone_mg_day": 5, "prednisone_duration_weeks": 2,
    "recent_pulse_250mg": False, "is_duration_months": 4, "line_changes": 0,
    "lymphocyte_abs": 1800, "on_rituximab": False, "on_jaki": False,
    "skin_mucosa_intact": True, "central_venous_catheter": True,
    "prosthesis_recent": False, "prosthesis_infected": False,
    "major_surgery_30d": False, "asplenic": False,
    "hospitalization_48h_30d": True, "mdr_colonization_12m": False,
    "tb_endemic_exposure": "low", "respiratory_exposure_4w": False,
    "fungal_exposure": False, "severe_infection_12m": False,
    "igg_mg_dl": 800, "is_escalation_3m": False,
    "risk_eval_current": "gaps",
    # Inyección del conector:
    "immuno1_class": k3_i1,
}

r2 = ne2.explain(caso_i2)
p25_val = r2["vector"]["P25"]
k3_i2   = r2["class"]

print(f"\n  Paso 2 — IMMUNO-2 (con κ₃_I1 inyectado en P25):")
print(f"    P25 = {p25_val}  (conector: {k3_i1} → {p25_val})")
print(f"    N₀={r2['counts']['n0']}, N₁={r2['counts']['n1']}, N_U={r2['counts']['nU']}")
print(f"    κ₃(IMMUNO-2) = {k3_i2}")

# Verificar que el conector es correcto
expected_p25 = {"APTO": "0", "NO_APTO": "1", "INDETERMINADO": "U"}[k3_i1]
check(
    f"P25 = {expected_p25} (conector correcto para κ₃_I1={k3_i1})",
    p25_val == expected_p25
)
check("κ₃(IMMUNO-2) es determinista y válido",
      k3_i2 in ("APTO", "NO_APTO", "INDETERMINADO"))

# Propiedad clave: si IMMUNO-1 es INDETERMINADO, P25 no fabrica certeza
if k3_i1 == "INDETERMINADO":
    check("Propiedad: IMMUNO-1 INDETERMINADO → P25=U (no fabrica certeza)",
          p25_val == "U")


# ─── Propiedad de composición: determinismo end-to-end ───────────────────────
print("\n=== Determinismo end-to-end de la composición ===")

# Ejecutar la composición 10 veces, verificar resultado idéntico
resultados = []
for _ in range(10):
    r_test = ne1.explain(caso_i1)
    caso_i2_test = dict(caso_i2)
    caso_i2_test["immuno1_class"] = r_test["class"]
    resultados.append(ne2.explain(caso_i2_test)["class"])

check("10 ejecuciones end-to-end producen el mismo κ₃(IMMUNO-2)",
      len(set(resultados)) == 1)


# ─── Resumen ──────────────────────────────────────────────────────────────────
print(f"\n{'='*65}")
print(f"  RESULTADO: {_PASS} tests OK  |  {_FAIL} tests FALLIDOS")
if _FAIL == 0:
    print("  [CONFORME] El compositor intercelular preserva los invariantes del SV.")
    print("  [CONFORME] La incertidumbre se propaga honestamente sin fabricar certeza.")
else:
    print("  [NO CONFORME] Revisar el compositor intercelular.")
print(f"{'='*65}\n")

sys.exit(0 if _FAIL == 0 else 1)
