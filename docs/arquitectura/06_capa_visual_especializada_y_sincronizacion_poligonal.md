# Capa visual especializada y sincronización poligonal

## Tesis

La capa visual especializada es necesaria, pero subordinada.

## Dos carriles de uso

### A. Carril editorial y de corpus
La capa visual puede:
- detectar desbordes, solapes y defectos compositivos;
- proponer correcciones visuales;
- y aprender de los dictámenes `APTO / NO APTO / U` del experto sobre figuras y portadas.

### B. Carril sensorial de dominio
La capa visual puede:
- leer RX, TAC, documentos o imágenes sin entrada estructurada previa;
- proponer observables;
- proyectarlos a células y polígonos;
- y aprender de correcciones soberanas posteriores.

## Sincronización poligonal

La transformación ilegítima sería:

`imagen -> sentencia`

La transformación correcta es:

`imagen -> observables -> célula/polígono -> diálogo experto–Inteligencia Lógica -> corrección soberana -> nuevo suceso`

El polígono no sustituye al álgebra ni a la imagen fuente. Su función es hacer visible la estructura y permitir trabajo compartido fino con el experto.

## Regla de prioridad de aprendizaje

La capa visual debe aprender primero de:

1. la representación estructurada ya corregida por el experto;
2. el polígono renderizado;
3. y solo después, o en paralelo, de la imagen cruda.

## Fórmula de subordinación

Sea `x_img` una imagen de entrada y sea `h` la corrección soberana del experto. El aprendizaje admisible es de la forma:

`θ_(n+1) = U_vis(θ_n, x_img, C_(n+1))`

donde `θ_n` es el estado interno de la capa visual y `C_(n+1)` es ya el resultado post-corrección.

No se admite aprendizaje para fundar verdad ni para cerrar `U` por sí sola.

## Prohibiciones

La capa visual no puede:
- cerrar `U` por sí sola;
- desplazar al experto;
- sustituir al álgebra;
- ni imponerse al Lenguaje SV o a la custodia estructural.
