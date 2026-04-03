# Compuerta ejecutable de custodia estructural del frente motor

## Naturaleza
Esta pieza incorpora al repositorio del motor una compuerta ejecutable mínima de custodia estructural.

No introduce gramática nueva, no modifica la IR canónica ni altera el N4/Uso del Lenguaje SV. Su objeto es más restringido: impedir, con salida visible en K₃, que un avance sensible del motor presione ilegítimamente al SV.

## Regla de paso
Todo avance del motor que roce alguno de estos frentes debe pasar por la compuerta de custodia antes de producir efectos:

1. lenguaje, DSL, IR o N4/Uso;
2. frontera ML/álgebra;
3. preservación de U;
4. límites de fase del motor;
5. paridad entre documento, artefacto, laboratorio y estado declarado.

## Política
- APTO → permitir avance controlado.
- INDETERMINADO → detener y revisar.
- NO APTO → bloquear avance.

## Delimitación negativa
Esta compuerta no sustituye a la célula de seguridad estructural doctrinal completa. No reescribe la lógica del motor, no vive en bucle permanente y no autoriza cierres soberanos.
