#!/usr/bin/env python3
"""
lab_03_ternarizacion_inmunologia_fase_2.py
================================
Verificación de los Ternarizadores del agente IMMUNO-2.

Autor:     Juan Antonio Lloret Egea
ORCID:     0000-0002-6634-3351
ITVIA:     IA eñ™ — https://www.itvia.online
ISSN:      2695-6411
Licencia:  CC BY-NC-ND 4.0
Titularidad y autoría: © Juan Antonio Lloret Egea, 2026.
Fecha:     4 de abril de 2026
Fecha y Versión: V.1 del conjunto
Versión:   V.1 del conjunto

Dominio:   SVperitus IMMUNO-2 — Riesgo infeccioso en inmunosupresión no trasplante
Población: Adultos ≥18 años en tratamiento activo o reciente (≤6 meses) con
           bDMARDs, tsDMARDs o IS convencionales mayores
Exclusiones: trasplante de órgano sólido, TPH/CAR-T, quimioterapia citotóxica pura, VIH
Motor:     SVperitus-dataset-main/agentes/inmunologia/fase_2/src/normative_engine.py

Referencias:
  P01 (edad):      Curtis JR, et al. Arthritis Rheum. 2012;64(10):3279-3288.
                   DOI:10.1002/art.34564
  P02 (comorbilidad): RABBIT Risk Score; Curtis 2012
  P06 (JAKi):      Ytterberg SR, et al. (ORAL Surveillance). N Engl J Med.
                   2022;386:316-326. DOI:10.1056/NEJMoa2109927
  P08 (corticoides): Singh JA, et al. Arthritis Care Res. 2015;67(2):151-166.
                   DOI:10.1002/acr.22535
  P10 (linfopenia): Winthrop KL, et al. Ann Rheum Dis. 2019;78(3):339-347.

Uso:
    # Desde la raíz de SV-motor-main basta con ejecutar:
    python laboratorio/etapa_6_manifiesto_de_arquitectura_y_garantias/lab_03_ternarizacion_inmunologia_fase_2.py
    # Para forzar un motor externo o alternativo:
    export SV_IMMUNO2_ENGINE=./SVperitus-dataset-main/agentes/inmunologia/fase_2/src
"""

import sys, os
from pathlib import Path
import importlib.util

_REPO_ROOT = Path(__file__).resolve().parents[2]
_DEFAULT_ENGINE_PATH = str((_REPO_ROOT.parent / "SVperitus-dataset-main/agentes/inmunologia/fase_2/src").resolve())
if not Path(_DEFAULT_ENGINE_PATH).exists():
    _DEFAULT_ENGINE_PATH = str((_REPO_ROOT.parent.parent / "SVperitus-dataset-main/agentes/inmunologia/fase_2/src").resolve())
ENGINE_PATH = os.environ.get(
    "SV_IMMUNO2_ENGINE",
    _DEFAULT_ENGINE_PATH
)

# Carga dinámica con alias para evitar colisión con IMMUNO-1
try:
    spec = importlib.util.spec_from_file_location(
        "ne_immuno2", os.path.join(ENGINE_PATH, "normative_engine.py")
    )
    ne2 = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(ne2)
except Exception as e:
    print(f"\n[ERROR] IMMUNO-2 no disponible: {e}")
    print(f"Ruta por defecto utilizada: {ENGINE_PATH}")
    print("Configurar alternativamente: export SV_IMMUNO2_ENGINE=/ruta/a/fase_2/src")
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
print("\n=== PARÁMETROS ESTRUCTURALES DE IMMUNO-2 ===")
check(f"N = {ne2.N} (tamaño de célula = 5²)", ne2.N == 25)
check(f"T(25) = {ne2.THRESHOLD}", ne2.THRESHOLD == 19)


# ─── P01: Edad ───────────────────────────────────────────────────────────────
print("\n=== P01: Edad — factor de riesgo basal ===")
print("    Referencia: Curtis 2012 (tasa 14.2/100pa en ≥65 vs 4.8 en <65)")

check("Edad 45 (18–64) → B₀ (0)", ne2.P01({"age": 45}) == "0")
check("Edad 65 (≥65) → B₁ (1)", ne2.P01({"age": 65}) == "1")
check("Edad 80 (≥65) → B₁ (1)", ne2.P01({"age": 80}) == "1")
check("Edad None → B_U (U honesta)", ne2.P01({"age": None}) == "U")
check("Edad 17 (<18, fuera de población diana) → B_U (U)",
      ne2.P01({"age": 17}) == "U")


# ─── P06: Tipo IS principal ──────────────────────────────────────────────────
print("\n=== P06: Tipo de fármaco inmunosupresor principal ===")
print("    Referencia: ORAL Surveillance (Ytterberg et al. NEJM 2022)")

check("JAKi → B₁ (1)  [riesgo documentado en ORAL Surveillance]",
      ne2.P06({"main_is_drug": "jaki"}) == "1")
check("Anti-CD20 → B₁ (1)", ne2.P06({"main_is_drug": "anti_cd20"}) == "1")
check("Ciclofosfamida → B₁ (1)", ne2.P06({"main_is_drug": "cyclophosphamide"}) == "1")
check("IS convencional monoterapia → B₀ (0)",
      ne2.P06({"main_is_drug": "conventional"}) == "0")
check("Fármaco None → B_U (U honesta)",
      ne2.P06({"main_is_drug": None}) == "U")


# ─── P08: Dosis corticoides ──────────────────────────────────────────────────
print("\n=== P08: Dosis equivalente de corticoides sistémicos ===")
print("    Referencia: Singh JA et al. Arthritis Care Res 2015")

check("Pulso reciente ≥250mg metilprednisolona → B₁ (1)",
      ne2.P08({"prednisone_mg_day": 5, "recent_pulse_250mg": True}) == "1")
check("≥15 mg/d (cualquier duración) → B₁ (1)",
      ne2.P08({"prednisone_mg_day": 15, "prednisone_duration_weeks": 2,
               "recent_pulse_250mg": False}) == "1")
check("≥7.5 mg/d ≥4 semanas → B₁ (1)",
      ne2.P08({"prednisone_mg_day": 10, "prednisone_duration_weeks": 5,
               "recent_pulse_250mg": False}) == "1")
check("<7.5 mg/d → B₀ (0)",
      ne2.P08({"prednisone_mg_day": 5, "prednisone_duration_weeks": 2,
               "recent_pulse_250mg": False}) == "0")
check("Sin datos → B_U (U honesta)",
      ne2.P08({"prednisone_mg_day": None, "recent_pulse_250mg": None}) == "U")


# ─── P10: Linfopenia ─────────────────────────────────────────────────────────
print("\n=== P10: Linfopenia relevante ===")

check("Linfocitos < 500 → B₁ (1)", ne2.P10({"lymphocyte_abs": 400}) == "1")
check("Linfocitos 820, en JAKi → B_U (U)  [750≤recuento<1000 en contexto JAKi]",
      ne2.P10({"lymphocyte_abs": 820, "on_jaki": True}) == "U")
check("Linfocitos ≥ 1000 → B₀ (0)",
      ne2.P10({"lymphocyte_abs": 1200, "on_jaki": False}) == "0")
check("Linfocitos None → B_U (U honesta)",
      ne2.P10({"lymphocyte_abs": None}) == "U")


# ─── P25: Parámetro puente con IMMUNO-1 ──────────────────────────────────────
print("\n=== P25: Parámetro puente IMMUNO-1 → IMMUNO-2 ===")
print("    Referencia: Doc I del corpus — transmisión en serie por parámetro puente")
print("    Lloret Egea JA. Álgebra de composición intercelular — I. ITVIA 2026.")

check("IMMUNO-1 = APTO → P25 = 0",
      ne2.P25({"immuno1_class": "APTO"}) == "0")
check("IMMUNO-1 = NO_APTO → P25 = 1",
      ne2.P25({"immuno1_class": "NO_APTO"}) == "1")
check("IMMUNO-1 = INDETERMINADO → P25 = U  (incertidumbre propagada honestamente)",
      ne2.P25({"immuno1_class": "INDETERMINADO"}) == "U")
check("IMMUNO-1 no aplicado → P25 = U",
      ne2.P25({}) == "U")


# ─── Caso representativo ejecutado ───────────────────────────────────────────
print("\n=== Caso representativo: AR+JAKi, 68a, DM complicada, eGFR 55 ===")
print("    (datos representativos de presentación típica; no datos de paciente real)")

caso_rep = {
    "age": 68,
    "dm_complicated": True, "hba1c": 8.5,
    "heart_failure_nyha": 0, "egfr": 55, "ischemic_heart_event_12m": False,
    "fev1_percent_predicted": 82, "ild_diagnosed": False,
    "bronchiectasis": False, "respiratory_exacerbation_12m": False,
    "fibrosis_stage": 0, "cirrhosis": False,
    "hbv_chronic_active": False, "hcv_viremia": False,
    "bmi": 28.5, "albumin": 3.8, "frailty_fried": 1,
    "main_is_drug": "jaki", "is_combination": "standard_combo",
    "prednisone_mg_day": 5, "prednisone_duration_weeks": 8,
    "recent_pulse_250mg": False,
    "is_duration_months": 14, "line_changes": 1,
    "lymphocyte_abs": 820, "on_rituximab": False, "on_jaki": True,
    "skin_mucosa_intact": True, "central_venous_catheter": False,
    "prosthesis_recent": False, "prosthesis_infected": False,
    "major_surgery_30d": False, "asplenic": False,
    "hospitalization_48h_30d": False, "mdr_colonization_12m": False,
    "tb_endemic_exposure": "low",
    "respiratory_exposure_4w": False, "fungal_exposure": False,
    "severe_infection_12m": True, "igg_mg_dl": 620,
    "is_escalation_3m": True, "risk_eval_current": "gaps",
    "immuno1_class": None,
}

r = ne2.explain(caso_rep)
n0 = r["counts"]["n0"]
n1 = r["counts"]["n1"]
nU = r["counts"]["nU"]
cls = r["class"]

check("P01 = 1 (edad ≥65)", r["vector"]["P01"] == "1")
check("P02 = 1 (DM complicada + ERC G3a)", r["vector"]["P02"] == "1")
check("P06 = 1 (JAKi)", r["vector"]["P06"] == "1")
check("P21 = 1 (infección grave <12m)", r["vector"]["P21"] == "1")
check("P23 = 1 (escalada IS reciente)", r["vector"]["P23"] == "1")
check("P24 = 1 (evaluación con gaps)", r["vector"]["P24"] == "1")
check(f"n = {n0+n1+nU} = N", n0+n1+nU == 25)
check(f"Clasificación = INDETERMINADO (N₁={n1} < T=19, N₀={n0} < T=19)", cls == "INDETERMINADO")

# Criticidad Γ
t = ne2.THRESHOLD
d0, d1 = t - n0, t - n1
gamma = nU - min(d0, d1)
# Con nU=3, d0=4, d1=12: Γ = 3 - min(4,12) = 3-4 = -1
check(f"Γ = {gamma} ≤ 0 (clasificación estructuralmente determinada — U no cambia κ₃)",
      gamma <= 0)

print(f"\n  Motor ejecutado: N₀={n0}, N₁={n1}, N_U={nU}, T=19, κ₃={cls}, Γ={gamma}")
if gamma <= 0:
    print("  Nota: Γ<0 indica que resolver las U no cambiará la clasificación global.")


# ─── Caso APTO: AR bien gestionada ───────────────────────────────────────────
print("\n=== Caso APTO: AR en monoterapia convencional, sin comorbilidades ===")

caso_apto = {
    "age": 45, "dm_complicated": False, "hba1c": 6.2,
    "heart_failure_nyha": 0, "egfr": 88, "ischemic_heart_event_12m": False,
    "fev1_percent_predicted": 95, "ild_diagnosed": False,
    "bronchiectasis": False, "respiratory_exacerbation_12m": False,
    "fibrosis_stage": 0, "cirrhosis": False,
    "hbv_chronic_active": False, "hcv_viremia": False,
    "bmi": 24.0, "albumin": 4.2, "frailty_fried": 0,
    "main_is_drug": "conventional", "is_combination": "monotherapy",
    "prednisone_mg_day": 5, "prednisone_duration_weeks": 2,
    "recent_pulse_250mg": False, "is_duration_months": 6, "line_changes": 0,
    "lymphocyte_abs": 1400, "on_rituximab": False, "on_jaki": False,
    "skin_mucosa_intact": True, "central_venous_catheter": False,
    "prosthesis_recent": False, "prosthesis_infected": False,
    "major_surgery_30d": False, "asplenic": False,
    "hospitalization_48h_30d": False, "mdr_colonization_12m": False,
    "tb_endemic_exposure": "low", "respiratory_exposure_4w": False,
    "fungal_exposure": False, "severe_infection_12m": False,
    "igg_mg_dl": 900, "is_escalation_3m": False,
    "risk_eval_current": "complete", "immuno1_class": "APTO",
}

r2 = ne2.explain(caso_apto)
check(f"N₀=25, N₁=0, N_U=0 → APTO (sistema puede producir APTO con datos completos)",
      r2["class"] == "APTO" and r2["counts"]["n1"] == 0 and r2["counts"]["nU"] == 0)

print(f"\n  Motor ejecutado: {r2['counts']} → {r2['class']}")


# ─── Resumen ──────────────────────────────────────────────────────────────────
print(f"\n{'='*65}")
print(f"  RESULTADO: {_PASS} tests OK  |  {_FAIL} tests FALLIDOS")
if _FAIL == 0:
    print("  [CONFORME] Los Ternarizadores de IMMUNO-2 son algebraicamente correctos.")
else:
    print("  [NO CONFORME] Revisar las particiones del motor normativo IMMUNO-2.")
print(f"{'='*65}\n")

sys.exit(0 if _FAIL == 0 else 1)
