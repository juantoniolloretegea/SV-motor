#!/usr/bin/env python3
"""
lab_02_ternarizacion_inmunologia_fase_1.py
================================
Verificación de los Ternarizadores del agente IMMUNO-1.
Verifica que cada partición B₀/B₁/B_U implementa correctamente
el criterio clínico declarado en la guía de referencia.

Autor:     Juan Antonio Lloret Egea
ORCID:     0000-0002-6634-3351
ITVIA:     IA eñ™ — https://www.itvia.online
ISSN:      2695-6411
Licencia:  CC BY-NC-ND 4.0
Titularidad y autoría: © Juan Antonio Lloret Egea, 2026.
Fecha:     4 de abril de 2026
Fecha y Versión: V.1 del conjunto
Versión:   V.1 del conjunto

Dominio:   SVperitus IMMUNO-1 — Profilaxis infecciosa en neoplasias hematológicas
Población: Adultos con neoplasias hematológicas e inmunosupresión
Motor:     SVperitus-dataset-main/agentes/inmunologia/fase_1/src/normative_engine.py

Referencias clínicas de las particiones:
  P01 (neutropenia): ECIL-4 — Maschmeyer G, et al. Haematologica. 2013;98(12):1826-1835.
                     DOI:10.3324/haematol.2013.091025
  P06 (fase): ECIL-4, consenso para estratificación de riesgo por fase de neoplasia
  P16 (gripe): ECIL-7 — Averbuch D, et al. Haematologica. 2021;106:2206-2215.
  Resto: ESCMID, IDSA guidelines

Uso:
    # Desde la raíz de SV-motor-main basta con ejecutar:
    python laboratorio/etapa_6_manifiesto_de_arquitectura_y_garantias/lab_02_ternarizacion_inmunologia_fase_1.py
    # Para forzar un motor externo o alternativo:
    export SV_IMMUNO1_ENGINE=./SVperitus-dataset-main/agentes/inmunologia/fase_1/src
"""

import sys, os
from pathlib import Path

_REPO_ROOT = Path(__file__).resolve().parents[2]
_DEFAULT_ENGINE_PATH = str((_REPO_ROOT.parent / "SVperitus-dataset-main/agentes/inmunologia/fase_1/src").resolve())
if not Path(_DEFAULT_ENGINE_PATH).exists():
    _DEFAULT_ENGINE_PATH = str((_REPO_ROOT.parent.parent / "SVperitus-dataset-main/agentes/inmunologia/fase_1/src").resolve())
ENGINE_PATH = os.environ.get(
    "SV_IMMUNO1_ENGINE",
    _DEFAULT_ENGINE_PATH
)
if ENGINE_PATH not in sys.path:
    sys.path.insert(0, ENGINE_PATH)

try:
    from normative_engine import (
        P01, P02, P03, P04, P05, P06, P07, P08,
        P16, P21, P24, P25,
        evaluate, classify, explain,
        THRESHOLD, N
    )
except ImportError as e:
    print(f"\n[ERROR] IMMUNO-1 no disponible: {e}")
    print(f"Ruta por defecto utilizada: {ENGINE_PATH}")
    print("Configurar alternativamente: export SV_IMMUNO1_ENGINE=/ruta/a/fase_1/src")
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


# ─── Parámetros estructurales ─────────────────────────────────────────────────
print("\n=== PARÁMETROS ESTRUCTURALES DE IMMUNO-1 ===")
check(f"N = {N} (tamaño de célula = 5²)", N == 25)
check(f"T(25) = {THRESHOLD} = floor(7·25/9)", THRESHOLD == 19)


# ─── P01: Neutropenia ─────────────────────────────────────────────────────────
print("\n=== P01: Neutropenia actual y riesgo de neutropenia prolongada ===")
print("    Referencia: ECIL-4 (Maschmeyer et al. Haematologica 2013)")

check("ANC=200 (< 500 → neutropenia grave activa) → B₁ (1)",
      P01({"anc_actual": 200}) == "1")
check("ANC=800 (500–999, zona de incertidumbre) → B_U (U)",
      P01({"anc_actual": 800}) == "U")
check("ANC=1500 (≥1000), sin nadir problemático → B₀ (0)",
      P01({"anc_actual": 1500}) == "0")
check("ANC=None, nadir=80, duración=21d → B₁ (1)  [inducción LMA]",
      P01({"anc_actual": None,
           "anc_expected_nadir": 80,
           "neutropenia_duration_days_expected": 21}) == "1")
check("ANC=None, nadir=None → B_U (U honesta — sin datos)",
      P01({"anc_actual": None, "anc_expected_nadir": None}) == "U")


# ─── P06: Fase neoplasia ──────────────────────────────────────────────────────
print("\n=== P06: Tipo y fase de la neoplasia hematológica ===")
print("    Referencia: estratificación ECIL por fase de neoplasia")

check("Fase 'induction' → B₁ (1)", P06({"hematology_phase": "induction"}) == "1")
check("Fase 'consolidation' → B₁ (1)", P06({"hematology_phase": "consolidation"}) == "1")
check("Fase 'relapse_high_risk' → B₁ (1)", P06({"hematology_phase": "relapse_high_risk"}) == "1")
check("Fase 'maintenance' → B₀ (0)", P06({"hematology_phase": "maintenance"}) == "0")
check("Fase 'chronic_stable' → B₀ (0)", P06({"hematology_phase": "chronic_stable"}) == "0")
check("Fase None → B_U (U honesta)", P06({"hematology_phase": None}) == "U")
check("Fase desconocida → B_U (U honesta)", P06({"hematology_phase": "desconocida"}) == "U")


# ─── P16: Vacuna antigripal ───────────────────────────────────────────────────
print("\n=== P16: Vacunación antigripal de temporada ===")
print("    Referencia: ECIL-7 (Averbuch et al. Haematologica 2021)")

check("Vacunado esta temporada → B₀ (0)",
      P16({"flu_vaccine_current_season": True}) == "0")
check("Sin vacuna esta temporada → B₁ (1)",
      P16({"flu_vaccine_current_season": False}) == "1")
check("Dato ausente → B_U (U honesta)",
      P16({"flu_vaccine_current_season": None}) == "U")


# ─── P21: Profilaxis PJP ──────────────────────────────────────────────────────
print("\n=== P21: Profilaxis frente a Pneumocystis jirovecii (PJP) ===")

check("Criterios cumplidos + profilaxis activa → B₀ (0)",
      P21({"pjp_prophylaxis_criteria_met": True,
           "pjp_prophylaxis_active": True}) == "0")
check("Criterios cumplidos + sin profilaxis → B₁ (1)",
      P21({"pjp_prophylaxis_criteria_met": True,
           "pjp_prophylaxis_active": False}) == "1")
check("Sin criterios (no la necesita) → B₀ (0)",
      P21({"pjp_prophylaxis_criteria_met": False,
           "pjp_prophylaxis_active": False}) == "0")
check("Criterios no evaluables → B_U (U honesta)",
      P21({"pjp_prophylaxis_criteria_met": None}) == "U")


# ─── Propiedades globales de clasificación ───────────────────────────────────
print("\n=== Propiedades globales: clasificación IMMUNO-1 ===")

# Precedencia desfavorable: N₁=19 → NO_APTO
check("N₁=19=T(25) → NO_APTO (precedencia desfavorable)",
      classify(["1"]*19 + ["0"]*6) == "NO_APTO")

# APTO con N₀=19
check("N₀=19=T(25), N₁=0, nU=6 → APTO",
      classify(["0"]*19 + ["U"]*6) == "APTO")

# INDETERMINADO
check("N₀=5, N₁=3, nU=17 → INDETERMINADO [caso LMA pre-inducción]",
      classify(["0"]*5 + ["1"]*3 + ["U"]*17) == "INDETERMINADO")

# U honesta no produce valor por defecto
v_all_u = ["U"] * 25
check("Vector todo-U (25 campos ausentes) → INDETERMINADO (no fabrica certeza)",
      classify(v_all_u) == "INDETERMINADO")


# ─── Caso representativo ejecutado ───────────────────────────────────────────
print("\n=== Caso representativo: neoplasia hematológica pre-inducción ===")
print("    (datos representativos de presentación típica; no datos de paciente real)")

caso_rep = {
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

r = explain(caso_rep)
n0 = r["counts"]["n0"]
n1 = r["counts"]["n1"]
nU = r["counts"]["nU"]
cls = r["class"]

check(f"P01 = 1 (neutropenia grave esperada)", r["vector"]["P01"] == "1")
check(f"P06 = 1 (fase inducción)", r["vector"]["P06"] == "1")
check(f"P16 = 1 (sin vacuna antigripal)", r["vector"]["P16"] == "1")
check(f"P12 = 0 (sin IFI previa)", r["vector"]["P12"] == "0")
check(f"n = {n0+n1+nU} = N (todos los parámetros evaluados)", n0+n1+nU == 25)
check(f"Clasificación = INDETERMINADO (N₁={n1} < T=19, N₀={n0} < T=19)", cls == "INDETERMINADO")

# Función de criticidad Γ
t = THRESHOLD
d0 = t - n0
d1 = t - n1
gamma = nU - min(d0, d1)
check(f"Γ = {gamma} > 0 (resolver U puede cambiar la clasificación)", gamma > 0)

print(f"\n  Motor ejecutado: N₀={n0}, N₁={n1}, N_U={nU}, T=19, κ₃={cls}, Γ={gamma}")


# ─── Resumen ──────────────────────────────────────────────────────────────────
print(f"\n{'='*65}")
print(f"  RESULTADO: {_PASS} tests OK  |  {_FAIL} tests FALLIDOS")
if _FAIL == 0:
    print("  [CONFORME] Los Ternarizadores de IMMUNO-1 son algebraicamente correctos.")
else:
    print("  [NO CONFORME] Revisar las particiones del motor normativo IMMUNO-1.")
print(f"{'='*65}\n")

sys.exit(0 if _FAIL == 0 else 1)
