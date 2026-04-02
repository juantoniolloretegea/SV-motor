"""
sv_motor.visual.validator
=========================
Validador geométrico determinista para figuras N1 del SV.
Implementa C_material_svg^9 con métricas G1–G9.

No requiere ML. Opera sobre SVG XML.
"""
from __future__ import annotations

import xml.etree.ElementTree as ET
from collections import defaultdict
from pathlib import Path
from typing import Dict, List, Optional, Tuple

from sv_motor.algebra.core import U, summarize_cell, gate


# ─────────────────────────────────────────────────────────────
# Utilidades de parseo
# ─────────────────────────────────────────────────────────────
def _f(v: Optional[str], default: float = 0.0) -> float:
    if v is None:
        return default
    s = str(v).strip()
    if s.endswith("%"):
        return default
    try:
        return float(s)
    except Exception:
        return default


def _text_bbox(t: dict) -> dict:
    """Bounding box estimado de un elemento text."""
    fs  = t["font_size"]
    txt = t["text"]
    w   = max(len(txt) * fs * 0.50, fs * 0.80)
    h   = fs * 1.00
    x, y = t["x"], t["y"]
    if t["anchor"] == "middle":
        left = x - w / 2
    elif t["anchor"] == "end":
        left = x - w
    else:
        left = x
    top = y - h * 0.82
    return {"x1": left, "y1": top, "x2": left + w, "y2": top + h, "w": w, "h": h}


def _overlap(a: dict, b: dict) -> float:
    dx = max(0.0, min(a["x2"], b["x2"]) - max(a["x1"], b["x1"]))
    dy = max(0.0, min(a["y2"], b["y2"]) - max(a["y1"], b["y1"]))
    return dx * dy


def _classify_metric(value: Optional[float], good: float, warn: float) -> object:
    if value is None:
        return U
    if value >= good:
        return 0
    if value >= warn:
        return U
    return 1


# ─────────────────────────────────────────────────────────────
# Parser SVG → paneles, textos, líneas
# ─────────────────────────────────────────────────────────────
def parse_svg(path: str | Path) -> Tuple[float, float, list, list, list]:
    root = ET.parse(str(path)).getroot()
    width  = _f(root.get("width"),  1200.0)
    height = _f(root.get("height"), 800.0)
    texts, rects, lines = [], [], []
    for el in root.iter():
        tag = el.tag.split("}")[-1]
        if tag == "rect":
            rects.append({
                "x":  _f(el.get("x")),  "y": _f(el.get("y")),
                "w":  _f(el.get("width")), "h": _f(el.get("height")),
                "rx": _f(el.get("rx")),
            })
        elif tag == "text":
            txt = "".join(el.itertext()).strip()
            if txt:
                texts.append({
                    "x":         _f(el.get("x")),
                    "y":         _f(el.get("y")),
                    "font_size": _f(el.get("font-size"), 16.0),
                    "anchor":    el.get("text-anchor", "start"),
                    "text":      txt,
                    "fill":      el.get("fill"),
                })
        elif tag == "line":
            lines.append({
                "x1": _f(el.get("x1")), "y1": _f(el.get("y1")),
                "x2": _f(el.get("x2")), "y2": _f(el.get("y2")),
            })
    return width, height, rects, texts, lines


def _panels(rects: list, width: float, height: float) -> list:
    """Selecciona rectángulos que son paneles de contenido."""
    full = width * height
    return [
        r for r in rects
        if 8000 <= r["w"] * r["h"] <= full * 0.90
    ]


# ─────────────────────────────────────────────────────────────
# Evaluación de las 9 métricas G1–G9
# ─────────────────────────────────────────────────────────────
def evaluate_svg(path: str | Path) -> Dict[str, object]:
    """
    Evalúa un SVG y devuelve C_material_svg^9 con métricas G1–G9.

    G1 footer presente
    G2 footer no pisado
    G3 textos críticos sin orfandad
    G4 sin colisión texto-texto
    G5 respiración vertical mínima
    G6 respiración horizontal mínima
    G7 cabecera dentro del lienzo
    G8 densidad modular controlada
    G9 legibilidad mínima del footer
    """
    width, height, rects, texts, lines = parse_svg(path)
    panels = _panels(rects, width, height)

    # Footer
    footer_line = next(
        (ln for ln in lines
         if abs(ln["y1"] - ln["y2"]) < 1 and ln["y1"] > height * 0.85),
        None,
    )
    footer_texts  = [t for t in texts if t["y"] > height * 0.90]
    header_texts  = [t for t in texts if t["y"] < min(170, height * 0.22)]

    # Asignación de textos a paneles
    assignments: dict = defaultdict(list)
    strict:      dict = defaultdict(list)
    orphans: list = []
    for t in texts:
        bb = _text_bbox(t)
        cx = (bb["x1"] + bb["x2"]) / 2
        cy = (bb["y1"] + bb["y2"]) / 2
        if cy < 170 or cy > height - 120:
            continue
        candidates = []
        for i, r in enumerate(panels):
            if (r["x"] - 20 <= cx <= r["x"] + r["w"] + 20 and
                    r["y"] - 70 <= cy <= r["y"] + r["h"] + 40):
                dx = 0 if r["x"] <= cx <= r["x"] + r["w"] else min(
                    abs(cx - r["x"]), abs(cx - (r["x"] + r["w"])))
                dy = 0 if r["y"] <= cy <= r["y"] + r["h"] else min(
                    abs(cy - r["y"]), abs(cy - (r["y"] + r["h"])))
                candidates.append((dx + dy, i))
        if candidates:
            _, idx = min(candidates)
            assignments[idx].append((t, bb))
            r = panels[idx]
            if r["x"] <= cx <= r["x"] + r["w"] and r["y"] <= cy <= r["y"] + r["h"]:
                strict[idx].append((t, bb))
        else:
            orphans.append((t, bb))

    # Métricas geométricas
    min_v = min_h = None
    max_overlap = max_density = 0.0
    for i, r in enumerate(panels):
        items = strict.get(i, [])
        if not items:
            continue
        area_sum = 0.0
        for _, bb in items:
            area_sum += bb["w"] * bb["h"]
            for val in [bb["y1"] - r["y"], r["y"] + r["h"] - bb["y2"]]:
                min_v = val if min_v is None else min(min_v, val)
            for val in [bb["x1"] - r["x"], r["x"] + r["w"] - bb["x2"]]:
                min_h = val if min_h is None else min(min_h, val)
        max_density = max(max_density, area_sum / max(r["w"] * r["h"], 1.0))
        for a in range(len(items)):
            for b in range(a + 1, len(items)):
                max_overlap = max(max_overlap, _overlap(items[a][1], items[b][1]))

    # Desbordamiento de cabecera
    header_overflow = sum(
        1 for t in header_texts
        if (bb := _text_bbox(t)) and (bb["x1"] < 0 or bb["x2"] > width or bb["y1"] < 0)
    )

    # G8 densidad
    if max_density <= 0.26:
        g8: object = 0
    elif max_density <= 0.34:
        g8 = U
    else:
        g8 = 1

    # G9 legibilidad footer
    footer_size = min((t["font_size"] for t in footer_texts), default=None)
    if footer_size is None:
        g9: object = 1
    elif footer_size >= 12:
        g9 = 0
    elif footer_size >= 10:
        g9 = U
    else:
        g9 = 1

    footer_ok    = bool(footer_texts)
    footer_clear = (footer_ok and footer_line is not None and
                    min(t["y"] for t in footer_texts) > footer_line["y1"] + 10)

    metrics = {
        "G1_footer_presente":             0 if footer_ok    else 1,
        "G2_footer_no_pisado":            0 if footer_clear else 1,
        "G3_textos_criticos_sin_orfandad":
            0 if not orphans else (U if len(orphans) <= 2 else 1),
        "G4_sin_colision_texto_texto":
            0 if max_overlap == 0 else (U if max_overlap < 80 else 1),
        "G5_respiracion_vertical_minima":
            _classify_metric(min_v, 12, 6),
        "G6_respiracion_horizontal_minima":
            _classify_metric(min_h, 12, 6),
        "G7_cabecera_dentro_del_lienzo":  0 if header_overflow == 0 else 1,
        "G8_densidad_modular_controlada": g8,
        "G9_footer_con_legibilidad_minima": g9,
    }

    c_material = summarize_cell(list(metrics.values()))

    return {
        "archivo":                  str(path),
        "width":                    width,
        "height":                   height,
        "paneles_detectados":       len(panels),
        "orphans":                  len(orphans),
        "max_overlap":              round(max_overlap, 3),
        "min_vertical_breathing":   None if min_v is None else round(min_v, 3),
        "min_horizontal_breathing": None if min_h is None else round(min_h, 3),
        "max_density":              round(max_density, 4),
        "metrics":                  metrics,
        "C_material_svg9":          c_material,
    }


# ─────────────────────────────────────────────────────────────
# C2 enriquecida
# ─────────────────────────────────────────────────────────────
def c2_enriched(
    svg_path: str | Path,
    c_contrato:       str = "APTO",
    c_material_decl:  str = "APTO",
    c_representacion: str = "APTO",
    c_traza:          str = "APTO",
) -> Dict[str, object]:
    """
    C2 visual enriquecida con validación geométrica automática.
    """
    svg_result   = evaluate_svg(svg_path)
    mat_svg      = svg_result["C_material_svg9"]["class"]
    mat_enriched = gate(c_material_decl, mat_svg)
    s_base       = gate(c_contrato, mat_enriched)
    s_fiel       = gate(c_representacion, c_traza)
    a_audit      = gate(s_base, s_fiel)
    return {
        "C_contrato":               c_contrato,
        "C_material_declarado":     c_material_decl,
        "C_material_svg9":          mat_svg,
        "C_material_enriquecido":   mat_enriched,
        "C_representacion":         c_representacion,
        "C_traza":                  c_traza,
        "S_base":                   s_base,
        "S_fiel":                   s_fiel,
        "A_auditoria_enriquecida":  a_audit,
        "svg_detail":               svg_result,
    }
