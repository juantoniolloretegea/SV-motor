"""
Laboratorio de verificación Python — etapa 5.

Verifica la capa sv_motor.verification sobre 15 casos canónicos
y 6 adversariales de doble vara.

Casos canónicos:
  VC-NLP-1   NLP APTO canónico
  VC-NLP-2   NLP INDETERMINADO (theta indeterminado)
  VC-NLP-3   NLP NO_APTO (contradicción)
  VC-NLP-4   NLP con irreducible → NO_APTO + no gobernable
  VC-DEV-1   DEV APTO canónico
  VC-DEV-2   DEV NO_APTO (frontera ML delegada)
  VC-DEV-3   DEV INDETERMINADO (trazabilidad indeterminada)
  VC-CST-1   CUSTODIA APTO canónico
  VC-CST-2   CUSTODIA NO_APTO (deriva doctrinal)
  VC-CUS-1   CUSTOM vector todo-0 → APTO
  VC-CUS-2   CUSTOM vector todo-1 → NO_APTO
  VC-CUS-3   CUSTOM con fronteriza → INDETERMINADO
  VC-CUS-4   CUSTOM con resoluble → evalúa correctamente
  VC-CUS-5   CUSTOM con todos irreducibles → no gobernable
  VC-CUS-6   CUSTOM n=1 → caso límite threshold

Adversariales de doble vara:
  DV-1   Misma entrada Python vs Python → idéntico (reproducibilidad)
  DV-2   NLP NO_APTO vs NLP APTO → discrepancia detectada
  DV-3   U no colapsado en JSON → verificar serialización honesta
  DV-4   JSON canónico con Rust simulado idéntico → PASS
  DV-5   JSON canónico con Rust simulado con k3 distinto → FAIL detectado
  DV-6   domain mismatch no afecta comparación algebraica
"""
from __future__ import annotations

import json
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'src'))

from sv_motor.algebra.core import U
from sv_motor.verification import (
    run_nlp, run_dev, run_custodia, run_custom,
    compare, verify_reproducible,
)


# ─────────────────────────────────────────────────────────────────────────────
# Casos canónicos
# ─────────────────────────────────────────────────────────────────────────────

CASOS = [
    # ── NLP ──────────────────────────────────────────────────────────────────
    {
        "id": "VC-NLP-1",
        "descripcion": "NLP APTO canónico",
        "runner": "nlp",
        "input": {
            "theta":"coherente","pi":"sin-pregunta","kappa":"coherente",
            "eta":"completa","gamma":"alineada","alpha":"apropiada",
            "mu":"sin-ambiguedad","chi":"sin-solicitud","psi":"en-curso",
        },
        "esperado_k3": "APTO",
        "esperado_gobernable": True,
        "esperado_u_irr": [],
        "esperado_politica": "CERRAR_FRAME",
    },
    {
        "id": "VC-NLP-2",
        "descripcion": "NLP INDETERMINADO (theta indeterminado)",
        "runner": "nlp",
        "input": {
            "theta":"indeterminado","pi":"sin-pregunta","kappa":"coherente",
            "eta":"completa","gamma":"alineada","alpha":"apropiada",
            "mu":"sin-ambiguedad","chi":"sin-solicitud","psi":"en-curso",
        },
        "esperado_k3": "INDETERMINADO",
        "esperado_gobernable": True,
        "esperado_u_irr": [],
        "esperado_politica": "CONTINUAR_EN_W(T,k)",
    },
    {
        "id": "VC-NLP-3",
        "descripcion": "NLP NO_APTO — contradicción estructural",
        "runner": "nlp",
        "input": {
            "theta":"desvio","pi":"sin-pregunta","kappa":"contradictoria",
            "eta":"completa","gamma":"alineada","alpha":"apropiada",
            "mu":"sin-ambiguedad","chi":"sin-solicitud","psi":"en-curso",
        },
        "esperado_k3": "NO_APTO",
        "esperado_gobernable": True,
        "esperado_u_irr": [],
        "esperado_politica": "PROPONER_FORK",
    },
    {
        "id": "VC-NLP-4",
        "descripcion": "NLP con soporte vacío en P1 → irreducible → no gobernable",
        "runner": "nlp",
        "input": {
            "theta":"indeterminado","pi":"sin-pregunta","kappa":"coherente",
            "eta":"completa","gamma":"alineada","alpha":"apropiada",
            "mu":"sin-ambiguedad","chi":"sin-solicitud","psi":"en-curso",
        },
        "support_override": {
            1: set(),    # P1 sin soporte → irreducible
            2:{0,1}, 3:{0,1}, 4:{0}, 5:{0,1}, 6:{0,1}, 7:{0,1}, 8:{0,1}, 9:{0,1},
        },
        "esperado_k3": "NO_APTO",
        "esperado_gobernable": False,
        "esperado_u_irr": [1],
        "esperado_politica": "PROPONER_FORK",
    },
    # ── DEV ──────────────────────────────────────────────────────────────────
    {
        "id": "VC-DEV-1",
        "descripcion": "DEV APTO canónico",
        "runner": "dev",
        "input": {
            "conformidad_doctrinal":"conforme","suficiencia_material":"verificable",
            "trazabilidad":"trazable","frontera_ml_algebra":"preservada",
            "preservacion_u":"preservada","paridad_doc_artefacto":"alineada",
            "soberania_humana":"preservada","protocolo_entrada":"alineado",
            "reversibilidad":"append-only",
        },
        "esperado_k3": "APTO",
        "esperado_gobernable": True,
        "esperado_u_irr": [],
        "esperado_politica": "CIERRE_TECNICO_PENDIENTE_DE_HS",
    },
    {
        "id": "VC-DEV-2",
        "descripcion": "DEV NO_APTO — frontera ML delegada",
        "runner": "dev",
        "input": {
            "conformidad_doctrinal":"conforme","suficiencia_material":"verificable",
            "trazabilidad":"trazable","frontera_ml_algebra":"delegada",
            "preservacion_u":"preservada","paridad_doc_artefacto":"alineada",
            "soberania_humana":"preservada","protocolo_entrada":"alineado",
            "reversibilidad":"append-only",
        },
        "esperado_k3": "NO_APTO",
        "esperado_gobernable": True,
        "esperado_u_irr": [],
        "esperado_politica": "PROPONER_FORK",
    },
    {
        "id": "VC-DEV-3",
        "descripcion": "DEV INDETERMINADO — trazabilidad indeterminada",
        "runner": "dev",
        "input": {
            "conformidad_doctrinal":"conforme","suficiencia_material":"verificable",
            "trazabilidad":"indeterminada","frontera_ml_algebra":"preservada",
            "preservacion_u":"preservada","paridad_doc_artefacto":"alineada",
            "soberania_humana":"preservada","protocolo_entrada":"alineado",
            "reversibilidad":"append-only",
        },
        "esperado_k3": "INDETERMINADO",
        "esperado_gobernable": True,
        "esperado_u_irr": [],
        "esperado_politica": "CORREGIR_BAJO_OBLIGACIONES",
    },
    # ── CUSTODIA ─────────────────────────────────────────────────────────────
    {
        "id": "VC-CST-1",
        "descripcion": "CUSTODIA APTO canónico",
        "runner": "custodia",
        "input": {
            "anclaje_doctrinal":"anclado",
            "presion_sobre_lenguaje":"respetada",
            "frontera_ml_algebra":"preservada",
            "paridad_documento_laboratorio":"alineada",
            "preservacion_u":"preservada",
            "limites_de_fase":"respetados",
            "trazabilidad":"trazable",
            "protocolo_activo":"activo",
            "dependencia_superior_respetada":"respetada",
        },
        "esperado_k3": "APTO",
        "esperado_gobernable": True,
        "esperado_u_irr": [],
    },
    {
        "id": "VC-CST-2",
        "descripcion": "CUSTODIA NO_APTO — deriva doctrinal",
        "runner": "custodia",
        "input": {
            "anclaje_doctrinal":"deriva",
            "presion_sobre_lenguaje":"respetada",
            "frontera_ml_algebra":"preservada",
            "paridad_documento_laboratorio":"alineada",
            "preservacion_u":"preservada",
            "limites_de_fase":"respetados",
            "trazabilidad":"trazable",
            "protocolo_activo":"activo",
            "dependencia_superior_respetada":"respetada",
        },
        "esperado_k3": "NO_APTO",
        "esperado_gobernable": True,
        "esperado_u_irr": [],
    },
    # ── CUSTOM ───────────────────────────────────────────────────────────────
    {
        "id": "VC-CUS-1",
        "descripcion": "CUSTOM vector todo-0 → APTO",
        "runner": "custom",
        "c_frame": [0]*9,
        "support_map": {},
        "esperado_k3": "APTO",
        "esperado_gobernable": True,
    },
    {
        "id": "VC-CUS-2",
        "descripcion": "CUSTOM vector todo-1 → NO_APTO",
        "runner": "custom",
        "c_frame": [1]*9,
        "support_map": {},
        "esperado_k3": "NO_APTO",
        "esperado_gobernable": True,
    },
    {
        "id": "VC-CUS-3",
        "descripcion": "CUSTOM con fronteriza → INDETERMINADO",
        "runner": "custom",
        "c_frame": [U, 0, 0, 0, 0, 0, 0, 0, 0],
        "support_map": {1: {0, 1}},
        "esperado_k3": "INDETERMINADO",
        "esperado_gobernable": True,
    },
    {
        "id": "VC-CUS-4",
        "descripcion": "CUSTOM con resoluble → gobernable, clasifica correctamente",
        "runner": "custom",
        "c_frame": [U, 0, 0, 0, 0, 0, 0, 0, 0],
        "support_map": {1: {0}},
        "esperado_k3": "INDETERMINADO",
        "esperado_gobernable": True,
        "esperado_u_irr": [],
    },
    {
        "id": "VC-CUS-5",
        "descripcion": "CUSTOM todos irreducibles → no gobernable → NO_APTO",
        "runner": "custom",
        "c_frame": [U]*9,
        "support_map": {i: set() for i in range(1, 10)},
        "esperado_k3": "NO_APTO",
        "esperado_gobernable": False,
        "esperado_u_irr": list(range(1, 10)),
    },
    {
        "id": "VC-CUS-6",
        "descripcion": "CUSTOM n=1 — caso límite threshold T(1)=0",
        "runner": "custom",
        "c_frame": [1],
        "support_map": {},
        "esperado_k3": "NO_APTO",
        "esperado_gobernable": True,
    },
]


def _run_case(caso: dict):
    runner = caso["runner"]
    supp   = caso.get("support_override")
    if runner == "nlp":
        return run_nlp(caso["input"], support_override=supp)
    elif runner == "dev":
        return run_dev(caso["input"])
    elif runner == "custodia":
        return run_custodia(caso["input"])
    elif runner == "custom":
        return run_custom(caso["c_frame"], caso["support_map"])
    raise ValueError(f"Unknown runner: {runner}")


# ─────────────────────────────────────────────────────────────────────────────
# Adversariales de doble vara
# ─────────────────────────────────────────────────────────────────────────────

def _adversarial_dv():
    fallos = []

    # DV-1: reproducibilidad — mismo JSON dos veces → PASS
    r1 = run_nlp({"theta":"coherente","pi":"sin-pregunta","kappa":"coherente",
                  "eta":"completa","gamma":"alineada","alpha":"apropiada",
                  "mu":"sin-ambiguedad","chi":"sin-solicitud","psi":"en-curso"})
    r2 = run_nlp({"theta":"coherente","pi":"sin-pregunta","kappa":"coherente",
                  "eta":"completa","gamma":"alineada","alpha":"apropiada",
                  "mu":"sin-ambiguedad","chi":"sin-solicitud","psi":"en-curso"})
    cmp = verify_reproducible(r1.to_dict(), r2.to_dict())
    if not cmp.verificado:
        fallos.append(f"DV-1: reproducibilidad falla: {cmp.discrepancias}")

    # DV-2: NLP NO_APTO vs NLP APTO → comparador detecta discrepancia
    r_noapto = run_nlp({"theta":"desvio","pi":"sin-pregunta","kappa":"contradictoria",
                        "eta":"completa","gamma":"alineada","alpha":"apropiada",
                        "mu":"sin-ambiguedad","chi":"sin-solicitud","psi":"en-curso"})
    cmp2 = compare(r1.to_dict(), r_noapto.to_dict())
    if cmp2.verificado:
        fallos.append("DV-2: comparador no detectó diferencia APTO vs NO_APTO")
    if not any(d["campo"] == "dictamen.k3" for d in cmp2.discrepancias):
        fallos.append("DV-2: discrepancia en k3 no reportada")

    # DV-3: U no colapsado en JSON
    r_u = run_custom([U, 0, 0, 0, 0, 0, 0, 0, 0], {1: {0, 1}})
    j = json.loads(r_u.json_canonical())
    if j["traza"]["C_frame"][0] != "U":
        fallos.append(f"DV-3: U colapsado en JSON, value={j['traza']['C_frame'][0]}")

    # DV-4: JSON canónico "Rust simulado" idéntico → PASS
    rust_sim = r1.to_dict()
    rust_sim["engine"] = "rust"
    cmp4 = compare(r1.to_dict(), rust_sim)
    if not cmp4.verificado:
        fallos.append(f"DV-4: comparador falla con Rust idéntico: {cmp4.discrepancias}")

    # DV-5: JSON canónico con Rust que tiene k3 distinto → FAIL detectado
    # r1 es APTO; rust_mal tiene k3=NO_APTO → comparador debe detectar discrepancia
    r_apto = run_nlp({"theta":"coherente","pi":"sin-pregunta","kappa":"coherente",
                      "eta":"completa","gamma":"alineada","alpha":"apropiada",
                      "mu":"sin-ambiguedad","chi":"sin-solicitud","psi":"en-curso"})
    rust_mal = r_apto.to_dict()
    rust_mal["engine"] = "rust"
    rust_mal["dictamen"]["k3"] = "NO_APTO"     # ← diferente del Python (APTO)
    rust_mal["traza"]["C_frame"] = [1]*9       # ← diferente del Python
    cmp5 = compare(r_apto.to_dict(), rust_mal)
    if cmp5.verificado:
        fallos.append("DV-5: comparador no detectó k3 distinto en Rust")

    # DV-6: domain mismatch no afecta comparación algebraica de campos idénticos
    rust_diff_domain = r1.to_dict()
    rust_diff_domain["engine"] = "rust"
    rust_diff_domain["domain"] = "CUSTOM"  # domain diferente
    cmp6 = compare(r1.to_dict(), rust_diff_domain)
    if not cmp6.verificado:
        fallos.append(f"DV-6: domain mismatch incorrectamente marcado como discrepancia: {cmp6.discrepancias}")

    return fallos


# ─────────────────────────────────────────────────────────────────────────────
# Ejecutar laboratorio
# ─────────────────────────────────────────────────────────────────────────────

def ejecutar_laboratorio() -> dict:
    resultados = []
    fallos = 0
    aciertos = 0

    for caso in CASOS:
        try:
            result = _run_case(caso)
        except Exception as e:
            fallos += 1
            resultados.append({"id": caso["id"], "passed": False, "error": str(e)})
            continue

        d = json.loads(result.json_canonical())

        checks = {}
        if "esperado_k3" in caso:
            checks["k3"] = d["dictamen"]["k3"] == caso["esperado_k3"]
        if "esperado_politica" in caso:
            checks["politica"] = d["dictamen"]["politica"] == caso["esperado_politica"]
        if "esperado_gobernable" in caso:
            checks["gobernable"] = d["traza"]["gobernable"] == caso["esperado_gobernable"]
        if "esperado_u_irr" in caso:
            checks["u_irr"] = sorted(d["traza"]["U_irr"]) == sorted(caso["esperado_u_irr"])

        # JSON parseado correctamente (engine, sv_version, domain presentes)
        checks["json_schema"] = all(
            k in d for k in ["sv_version", "engine", "domain", "programa", "traza", "dictamen"]
        )
        # U se preserva como "U" en JSON si había U en C_frame
        if U in (caso.get("c_frame") or []):
            u_in_json = "U" in d["traza"]["C_frame"]
            checks["u_preserved"] = u_in_json

        passed = all(checks.values())
        if passed:
            aciertos += 1
        else:
            fallos += 1

        resultados.append({
            "id":          caso["id"],
            "descripcion": caso["descripcion"],
            "passed":      passed,
            "checks":      checks,
            "k3":          d["dictamen"]["k3"],
            "politica":    d["dictamen"].get("politica"),
            "gobernable":  d["traza"]["gobernable"],
            "u_irr":       d["traza"]["U_irr"],
            "engine":      d["engine"],
            "sv_version":  d["sv_version"],
        })

    # Adversariales
    dv_fallos = _adversarial_dv()
    dv_aciertos = 6 - len(dv_fallos)

    total = len(CASOS) + 6
    total_pass = aciertos + dv_aciertos
    total_fail = fallos + len(dv_fallos)

    dictamen = {
        "total":           total,
        "canonicos":       {"total": len(CASOS), "aciertos": aciertos, "fallos": fallos},
        "doble_vara":      {"total": 6, "aciertos": dv_aciertos, "fallos": len(dv_fallos),
                            "detalle": dv_fallos},
        "total_aciertos":  total_pass,
        "total_fallos":    total_fail,
        "dictamen":        "APTO" if total_fail == 0 else "NO_APTO",
        "nota": (
            "Capa de verificación Python verificada: JSON canónico producido, "
            "U preservada como 'U', doble vara funcional, comparador detecta discrepancias."
            if total_fail == 0 else
            f"{total_fail} caso(s) fallaron."
        ),
    }

    import json as _json
    print(_json.dumps(dictamen, ensure_ascii=False, indent=2))
    return {"dictamen_global": dictamen, "casos": resultados}


if __name__ == "__main__":
    salida = ejecutar_laboratorio()
    out = os.path.join(os.path.dirname(__file__), "salida_laboratorio_verificacion.json")
    with open(out, "w", encoding="utf-8") as f:
        import json as _json
        _json.dump(salida, f, ensure_ascii=False, indent=2)
    print(f"\nSalida completa: {out}")
