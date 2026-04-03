# Estrategia de directorios y crecimiento controlado

## Objeto

Fijar una estructura de directorios que no estreche el carril del motor ni lo ensanche sin evidencia.

## Regla general

Todo bloque del repositorio debe satisfacer simultáneamente:

1. **anclaje algebraico**;
2. **anclaje aplicado**;
3. **delimitación negativa explícita**;
4. **soporte laboratorial o verificable** cuando proceda;
5. **compatibilidad con el Lenguaje SV y con la custodia estructural**.

Si un bloque no cumple esas condiciones, no debe abrirse.

## Estructura estable

### `docs/gobierno/`
Normas de prevalencia, hitos, alcance y estrategia de crecimiento.

### `docs/arquitectura/`
Planos internos del motor, capas de IA, dominio `𝔇_DEV`, custodia y capa visual.

### `docs/fundamentos/`
Piezas algebraicas y de estado del corpus necesarias para sostener al motor sin sobreactuar cierres.

### `docs/agentes/`
Subordinación del motor a agentes especializados, arranque de dominios y realimentación experta.

### `docs/interfaces/`
Interacción experto–Inteligencia Lógica, proyección poligonal y futura transducción a `.svp`.

### `docs/estado/`
Estado verificable del frente motor y posición frente al proyecto hermano.

### `docs/calidad/`
Solo registros y matrices necesarios para sostener trazabilidad real. No debe convertirse en crónica de proceso.

### `docs/archivo/`
Material retirado de primera línea por ser histórico, transitorio o impropio de una lectura científica principal.

## Regla editorial

La capa activa del repositorio no debe contener:
- manifiestos de lote;
- actas de proceso;
- nombres personales o de unidad;
- dramatización del trabajo;
- ni justificaciones narrativas del tipo “se hace esto porque pasó aquello”.

La capa activa debe contener estado, alcance, límites, decisiones y deuda viva.

## Regla de compatibilidad futura

La estructura debe dejar espacio explícito para:
- ampliación de la capa visual especializada;
- una futura especificación fuerte de programación en `.svp`;
- y una posible pieza especializada en IA, solo si el camino documental 4→9 la habilita suficientemente.

No debe anticipar esas piezas como si ya existieran.
