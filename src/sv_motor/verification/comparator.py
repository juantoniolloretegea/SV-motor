"""
sv_motor.verification.comparator
==================================
Comparador de salidas JSON canónicas entre el runner Python y el backend Rust/.svp.

OBJETO
------
Dada la misma entrada algebraica, el runner Python y el backend Rust
DEBEN producir JSON con resultados idénticos en los campos algebraicos:
  - traza.k3         ← dictamen algebraico
  - traza.U_irr      ← posiciones irreducibles
  - traza.gobernable ← precondición del Teorema 1
  - dictamen.k3
  - dictamen.politica

Los campos de metadatos (engine, sv_version, timestamps) pueden diferir.

PROTOCOLO DE DOBLE VARA
------------------------
  1. Programador ejecuta mismo programa en Python → JSON_py
  2. Programador ejecuta mismo programa en Rust/.svp → JSON_rust
  3. Comparador ingiere ambos JSON
  4. Si los campos algebraicos coinciden: VERIFICADO
  5. Si difieren: DISCREPANCIA con campo exacto y valores — revisar el .svp

CAMPOS EXCLUIDOS DE LA COMPARACIÓN
------------------------------------
  - engine (siempre diferirá: "python" vs "rust")
  - sv_version (puede diferir entre versiones)
  - programa.observables (puede tener representaciones distintas)
  - cualquier campo de metadata de implementación
"""
from __future__ import annotations

import json
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional


# ─────────────────────────────────────────────────────────────────────────────
# Campos algebraicos que DEBEN ser idénticos en ambas salidas
# ─────────────────────────────────────────────────────────────────────────────

_ALGEBRAIC_FIELDS = [
    ("traza", "C_frame"),
    ("traza", "gamma_h_labels"),
    ("traza", "C_gob"),
    ("traza", "A_agente"),
    ("traza", "U_irr"),
    ("traza", "gobernable"),
    ("dictamen", "k3"),
    ("dictamen", "politica"),
]

_OPTIONAL_FIELDS = [
    ("dictamen", "obligaciones"),  # solo en DEV/CUSTODIA — comparar si ambos lo tienen
]


# ─────────────────────────────────────────────────────────────────────────────
# Resultado de comparación
# ─────────────────────────────────────────────────────────────────────────────

@dataclass
class ComparisonResult:
    """
    Resultado de la comparación entre dos salidas JSON canónicas.

    verificado: True si todos los campos algebraicos coinciden.
    discrepancias: lista de campos que difieren, con valor Python y valor Rust.
    campos_auditados: número de campos comparados.
    """
    verificado:       bool
    discrepancias:    List[Dict[str, Any]] = field(default_factory=list)
    campos_auditados: int = 0
    engine_python:    str = "python"
    engine_rust:      str = "rust"
    domain:           str = ""

    def to_dict(self) -> Dict[str, Any]:
        return {
            "verificado":       self.verificado,
            "campos_auditados": self.campos_auditados,
            "engine_python":    self.engine_python,
            "engine_rust":      self.engine_rust,
            "domain":           self.domain,
            "discrepancias":    self.discrepancias,
            "dictamen_comparacion": (
                "DOBLE_VARA_PASS" if self.verificado else "DOBLE_VARA_FAIL"
            ),
        }

    def to_json(self, indent: int = 2) -> str:
        return json.dumps(self.to_dict(), ensure_ascii=False, indent=indent)


# ─────────────────────────────────────────────────────────────────────────────
# Comparador principal
# ─────────────────────────────────────────────────────────────────────────────

def compare(
    json_python: str | Dict[str, Any],
    json_rust:   str | Dict[str, Any],
) -> ComparisonResult:
    """
    Compara dos salidas JSON canónicas del runner Python y del backend Rust/.svp.

    Args:
        json_python: JSON string o dict producido por py_runner.
        json_rust:   JSON string o dict producido por el backend Rust/.svp.

    Returns:
        ComparisonResult con veredicto y lista de discrepancias.

    Nota: los campos engine, sv_version y metadata de implementación
          se excluyen de la comparación — se espera que difieran.
    """
    py_d  = json.loads(json_python) if isinstance(json_python, str) else json_python
    ru_d  = json.loads(json_rust)   if isinstance(json_rust,   str) else json_rust

    domain       = py_d.get("domain", ru_d.get("domain", "?"))
    engine_py    = py_d.get("engine", "python")
    engine_ru    = ru_d.get("engine", "rust")
    discrepancias: List[Dict[str, Any]] = []
    campos = 0

    # Campos algebraicos obligatorios
    for section, key in _ALGEBRAIC_FIELDS:
        py_val = py_d.get(section, {}).get(key)
        ru_val = ru_d.get(section, {}).get(key)
        campos += 1

        # Normalizar: listas vs listas, dicts vs dicts
        py_norm = _normalize(py_val)
        ru_norm = _normalize(ru_val)

        if py_norm != ru_norm:
            discrepancias.append({
                "campo":   f"{section}.{key}",
                "python":  py_val,
                "rust":    ru_val,
                "gravedad": "ALTA" if key in ("k3", "politica", "U_irr") else "MEDIA",
            })

    # Campos opcionales — solo si ambos los tienen
    for section, key in _OPTIONAL_FIELDS:
        py_has = key in py_d.get(section, {})
        ru_has = key in ru_d.get(section, {})
        if py_has and ru_has:
            py_val = py_d[section][key]
            ru_val = ru_d[section][key]
            campos += 1
            if _normalize(py_val) != _normalize(ru_val):
                discrepancias.append({
                    "campo":   f"{section}.{key}",
                    "python":  py_val,
                    "rust":    ru_val,
                    "gravedad": "MEDIA",
                })

    return ComparisonResult(
        verificado       = len(discrepancias) == 0,
        discrepancias    = discrepancias,
        campos_auditados = campos,
        engine_python    = engine_py,
        engine_rust      = engine_ru,
        domain           = domain,
    )


def compare_files(path_python: str, path_rust: str) -> ComparisonResult:
    """
    Compara dos ficheros JSON en disco.

    Args:
        path_python: ruta al JSON producido por py_runner.
        path_rust:   ruta al JSON producido por el backend Rust/.svp.
    """
    with open(path_python, encoding="utf-8") as f:
        py_json = f.read()
    with open(path_rust, encoding="utf-8") as f:
        ru_json = f.read()
    return compare(py_json, ru_json)


# ─────────────────────────────────────────────────────────────────────────────
# Comparación simulada: Python vs Python (para tests)
# Permite verificar que dos ejecuciones Python del mismo programa
# producen resultados idénticos (reproducibilidad).
# ─────────────────────────────────────────────────────────────────────────────

def verify_reproducible(
    json_run1: str | Dict[str, Any],
    json_run2: str | Dict[str, Any],
) -> ComparisonResult:
    """
    Verifica que dos ejecuciones del runner Python producen resultados idénticos.
    Útil para confirmar la propiedad de reproducibilidad antes de comparar con Rust.
    """
    return compare(json_run1, json_run2)


# ─────────────────────────────────────────────────────────────────────────────
# Utilidades internas
# ─────────────────────────────────────────────────────────────────────────────

def _normalize(val: Any) -> Any:
    """
    Normaliza valores para comparación robusta:
    - listas → tuplas ordenadas solo si son listas de primitivos
    - dicts  → dicts con claves string
    - U como string → preservado
    """
    if isinstance(val, list):
        # Si son listas de valores simples (ternario), mantener orden
        return [_normalize(x) for x in val]
    if isinstance(val, dict):
        return {str(k): _normalize(v) for k, v in sorted(val.items())}
    if val is None:
        return None
    return val
