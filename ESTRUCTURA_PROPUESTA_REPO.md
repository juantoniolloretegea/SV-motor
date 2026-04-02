# Estructura propuesta del repositorio

```text
SV-motor/
├── README.md
├── ESTRUCTURA_PROPUESTA_REPO.md
├── pyproject.toml
├── .gitignore
├── docs/
│   ├── gobierno/
│   │   ├── 00_estatuto_del_repositorio.md
│   │   ├── 01_dependencias_y_prevalencia.md
│   │   ├── 02_alcance_y_no_alcance.md
│   │   ├── 03_criterios_de_parada_sincronizacion_y_avance.md
│   │   ├── 04_compuerta_de_seguridad_aplicable.md
│   │   └── 05_plan_de_fases_del_motor.md
│   ├── arquitectura/
│   │   ├── 00_arquitectura_minima_local_y_auxiliar_online.md
│   │   ├── 01_relacion_con_lenguaje_sv.md
│   │   └── 02_relacion_con_nlp_y_banco_de_idiomas.md
│   └── calidad/
│       ├── README.md
│       ├── HOJA_DE_CAMBIOS.md
│       ├── CHECKLIST_CUSTODIA_MOTOR.md
│       ├── CHECKLIST_PUBLICACION_MINIMA.md
│       ├── REGISTRO_DECISIONES_MOTOR.csv
│       ├── REGISTRO_SINCRONIZACIONES.csv
│       └── REGISTRO_ADVERSARIALES.csv
├── laboratorio/
│   ├── README.md
│   ├── etapa_0_preparacion/
│   │   └── README.md
│   └── etapa_1_nucleo_local/
│       └── README.md
├── scripts/
│   └── run_local_checks.sh
├── .github/
│   └── workflows/
│       └── ci.yml
├── src/
│   └── sv_motor/
│       ├── __init__.py
│       ├── algebra/
│       │   ├── __init__.py
│       │   ├── core.py
│       │   └── nlp.py
│       ├── visual/
│       │   ├── __init__.py
│       │   └── validator.py
│       └── extractors/
│           ├── __init__.py
│           └── ext_nlp.py
└── tests/
    └── test_algebra.py
```

## Criterio de composición

La estructura separa desde el inicio:

- gobierno y límites;
- arquitectura técnica;
- control de calidad y trazabilidad;
- laboratorio mínimo;
- código local soberano;
- y pruebas automáticas.

No se abre todavía ninguna carpeta que sugiera backend definitivo, entrenamiento o soberanía online.
