# Programación trazable en Lenguaje SV

## Objeto

Declarar el problema correcto de programación asistida en Lenguaje SV.

## Tesis

El sistema debe poder asistir a un desarrollador para que una intención formalizada desemboque en un programa `.svp` con resultado trazable, auditable y justificable.

## Problema matemático

Sea `ι` una intención normalizada, sea `R_lang` el conjunto de restricciones vigentes del Lenguaje SV y sea `p_svp` un programa con extensión `.svp`. El problema correcto es construir una transducción:

`T : (ι, R_lang) -> (p_svp, E)`

con `E` expediente estructural suficiente para reconstruir por qué el programa resultó en `.svp` y no en otra forma.

## Condiciones mínimas

1. `ι` debe quedar interpretada sin contradicción.
2. `R_lang` debe provenir del repositorio del Lenguaje SV.
3. `p_svp` debe ser válido respecto de `R_lang`.
4. `E` debe conservar interpretación, restricciones, pasos y límites.

## Estado actual

Esta capa está requerida por la dirección aplicada del motor, pero no está implementada aún en este repositorio.

Cuando exista `p_svp`, podrá emitirse además una representación paralela en Python estrictamente derivada del mismo expediente `E`, con salida JSON canónica y finalidad exclusiva de verificación cruzada. Esa representación no reemplaza al `.svp` ni crea una segunda verdad del programa: actúa como vara auxiliar para cotejo y difusión.

## Regla

No procede presentar como solución a este problema ninguna salida que no sea `.svp`, ni introducir gramática nueva por necesidades del motor.
