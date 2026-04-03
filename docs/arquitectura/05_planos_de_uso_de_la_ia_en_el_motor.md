# Planos de uso de la IA en el motor

## Objeto

Separar con rigor algebraico los distintos usos de la IA dentro del frente motor.

## Notación

- `x_nl`: entrada en lenguaje natural.
- `x_img`: entrada visual.
- `C_n`: célula o composición de células en el suceso `n`.
- `H`: horizonte declarado.
- `Σ_n`: conjunto de sugerencias auxiliares.

## Plano 1 — Configuración del dominio

La IA puede asistir en la fase de arranque de un agente especializado, pero no funda por sí misma el dominio. El cierre de parámetros, capas, puentes y meta-células sigue siendo experto.

## Plano 2 — Inteligencia Lógica

`σ_nl = L_nl(x_nl, C_n, H)`

Capa conversacional subordinada. Produce una sugerencia orientativa. No funda verdad, no clausura `U` y no sustituye al experto.

## Plano 3 — Capa visual especializada

`Ĉ_n = V(x_img)`

La capa visual transforma imagen en representación estructural utilizable por el SV. Cuando ya existe representación poligonal, puede además producir sugerencias auxiliares de corrección o priorización:

`σ_vis = L_vis(x_img, C_n)`

La capa visual puede aprender de la corrección experta posterior. No clasifica soberanamente en `K₃`.

## Plano 4 — Álgebra y custodia

`κ_n = A(C_n, H)`

La clasificación estructural, la preservación de `U`, la evaluación de compuerta y la coherencia de fase permanecen aquí.

## Plano 5 — Cierre experto

`C_(n+1) = h(C_n, Σ_n)`

Para un estado `C_n` y un conjunto de sugerencias `Σ_n = {σ_nl, σ_vis}`, el experto aplica una corrección soberana que da lugar al siguiente suceso.

## Plano 6 — Programación trazable en `.svp`

`p_svp = T(ι, R_lang)`

La intención `ι` puede transducirse a código `.svp` solo mediante reglas subordinadas al Lenguaje SV. El resultado debe conservar expediente estructural de justificación.

## Regla de separación

Ningún plano inferior corrige silenciosamente a uno superior. En particular:
- la Inteligencia Lógica no sustituye al álgebra;
- la capa visual no sustituye al experto;
- y la transducción a `.svp` no introduce gramática nueva.
