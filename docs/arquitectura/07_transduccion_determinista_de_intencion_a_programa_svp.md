# Transducción determinista de intención a programa `.svp`

## Objeto

Fijar el problema correcto de programación asistida en Lenguaje SV.

## Tesis

El frente motor debe poder asistir a un programador o desarrollador que formule intención en lenguaje natural o en especificación técnica, de modo que el resultado final sea un programa `.svp` trazable, auditable y justificable.

## Delimitación negativa

No se trata de “generar código” por conveniencia textual. Se trata de transducir intención a construcciones legítimas del Lenguaje SV sin inventar gramática, sin deformar la IR y sin forzar al repositorio del lenguaje.

## Notación

- `ι`: intención de programación.
- `R_lang`: conjunto de restricciones vigentes del Lenguaje SV.
- `p_svp`: programa resultante en extensión `.svp`.
- `E`: expediente de justificación.

La operación legítima es:

`T : (ι, R_lang) -> (p_svp, E)`

## Condiciones necesarias

1. `ι` debe poder normalizarse sin contradicción.
2. `R_lang` debe provenir del repositorio del Lenguaje SV y no del frente motor.
3. `p_svp` debe construirse solo con elementos permitidos por `R_lang`.
4. `E` debe conservar, al menos:
   - interpretación aceptada de `ι`;
   - restricciones activas;
   - transformaciones aplicadas;
   - puntos dejados en `U`;
   - y justificación de por qué el programa resultó en `.svp` y no en otra representación.

## Estado actual

Esta capa está **requerida** por la dirección aplicada del motor, pero **no está todavía implementada** en el repositorio.

Lo que procede hoy es:
- declarar el problema correctamente;
- fijar su delimitación algebraica;
- y reservarle un carril estructural compatible con el repositorio del Lenguaje SV.

## Prohibiciones

No procede:
- presentar una salida Python como si resolviera este problema;
- introducir gramática nueva por necesidad del motor;
- ni declarar cerrada esta capa antes de su alineación explícita con el repositorio del Lenguaje SV.
