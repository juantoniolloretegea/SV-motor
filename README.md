# SV-motor

Repositorio público de estudio y materialización mínima del frente motor del Sistema Vectorial SV.

## Estatuto

Este repositorio no funda doctrina, no sustituye al pliego, no redefine el lenguaje SV y no equivale al motor completo del SV. Su función es más restringida: materializar, con trazabilidad pública, un núcleo local mínimo, determinista y auditable, junto con la documentación, los registros de calidad y los laboratorios necesarios para someterlo a prueba por terceros.

## Qué puede afirmarse hoy

- Existe un núcleo algebraico local sin dependencias externas de ML para clasificación, compuertas, gobierno NLP mínimo y validación geométrica básica.
- El repositorio contiene un paquete Python instalable, una suite de pruebas local y un laboratorio mínimo reproducible de la Etapa 1 del frente motor.
- La extracción desde lenguaje natural o imagen, cuando comparece, queda subordinada al núcleo local y no decide por sí misma el dictamen del SV.
- La publicación de cualquier avance de este repositorio debe quedar respaldada por una versión material auditada del repositorio, un laboratorio reproducible y documentación legible por terceros.

## Qué no puede afirmarse hoy

- No procede declarar resuelto el motor propio del SV.
- No procede declarar backend soberano, runner definitivo ni integración fuerte con el lenguaje.
- No procede desplazar la autoridad del álgebra hacia corpus, proveedores externos o artefactos técnicos.
- No procede presentar la capa online como sede del dictamen.
- No procede declarar cerrado el frente NLP ni usarlo como fundamento suficiente del motor.

## Dependencias superiores

Este repositorio queda subordinado, al menos, a las siguientes piezas y sedes:

1. Pliego de condiciones del Sistema Vectorial SV.
2. FT-SV-IA/001.
3. Células especializadas del Sistema Vectorial SV.
4. Célula especializada de seguridad estructural.
5. Agente Especializado en Lenguaje Natural para el Sistema Vectorial SV.
6. Repositorios doctrinal, del lenguaje y del banco de idiomas del proyecto.

## Regla operativa principal

Todo lo que sea algebraicamente determinista, clasificatorio o de custodia debe permanecer local, reproducible y verificable por inspección del código. Cualquier capa online sólo puede comparecer como extracción subordinada o soporte auxiliar, nunca como sede del cierre.

## Estado de trabajo acreditado por este lote

Este lote deja acreditados los siguientes mínimos:

- paquete local instalable;
- suite de pruebas local ampliada;
- laboratorio mínimo reproducible de la Etapa 1 del núcleo local;
- registros de calidad básicos del frente motor;
- y documentación pública sobria sobre alcance y límites.

## Ejecución local mínima

```bash
python -m pip install -e ".[dev]"
PYTHONPATH=src pytest -q
python laboratorio/etapa_1_nucleo_local/ejecutar_laboratorio_motor_etapa_1.py
```


## Demostración end-to-end local de Fase 0

Esta versión añade una demostración local, repetible y trazable del frente motor en modo `direct`, junto con una implementación ejecutable mínima del protocolo equivalente a FT-SV-IA/001 para el carril local.

### Comando canónico de demostración

```bash
sv-nlp --modo direct   --obs-file laboratorio/etapa_0_demostracion_local/entrada_observables_demo.json   --session-file laboratorio/etapa_0_demostracion_local/sesion_demo_ft_sv_ia.json   --out laboratorio/etapa_0_demostracion_local/salida_demo_end_to_end_local.json
```

### Qué demuestra

- que la cadena local completa puede ejecutarse desde terminal;
- que la clasificación K₃ y la política de salida se obtienen sin red y sin ML decisional;
- y que el bloque de estado equivalente a FT-SV-IA/001 aparece solo cuando existe algo que declarar.

## Estructura relevante

- `docs/gobierno/` — estatuto, alcance, dependencia y fases.
- `docs/arquitectura/` — arquitectura mínima local y relación con lenguaje, NLP y banco.
- `docs/calidad/` — procedimiento, registros, deuda viva, tablero de bloques y checklist.
- `laboratorio/etapa_0_demostracion_local/` — demostración end-to-end local de Fase 0.
- `laboratorio/etapa_1_nucleo_local/` — laboratorio mínimo reproducible y salidas.
- `src/sv_motor/` — paquete Python del núcleo local.
- `tests/` — pruebas algebraicas, de extractores y de validación geométrica.
