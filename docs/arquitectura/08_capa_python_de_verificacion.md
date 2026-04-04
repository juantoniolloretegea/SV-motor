# Capa Python de verificación y doble vara

## Objeto

Declarar el rol, la arquitectura y los límites de la capa Python de verificación del Sistema Vectorial SV.

## Posición en el motor

El motor de ejecución canónico del SV sigue siendo el backend Rust compilado desde código `.svp`. La capa Python no lo reemplaza. Su función es producir, en Python puro, un JSON canónico local del runner que sirva como vara de cotejo reproducible mientras no exista todavía el adaptador formal con la IR v0.2 del Lenguaje SV.

## Tres usos legítimos

**1. Verificación cruzada.** Un programador puede ejecutar el runner Python, obtener JSON y contrastarlo con una salida futura del backend Rust mediante un adaptador explícito.

**2. Doble vara local.** Dos implementaciones del mismo álgebra pueden compararse campo a campo cuando comparten expediente y contrato de salida.

**3. Universalidad.** Python + JSON hace el SV accesible a otros ecosistemas sin modificar la especificación algebraica ni comprometer la ejecución Rust.

## Separación entre Python y `.svp`/Rust

| Propiedad | Python (verificación) | `.svp` + Rust (ejecución) |
|---|---|---|
| Rol | Verificar corrección | Ejecutar en producción |
| Compilación | Ninguna | Backend Rust soberano |
| Runtime | Python ≥ 3.10 | Sin runtime externo |
| JSON output | Canónico local del runner | A definir por puente formal |
| Velocidad | Secundaria | Primaria |

## Esquema JSON canónico local

El runner Python produce un esquema estable y trazable para el frente motor. No debe declararse todavía identidad de esquema con la IR v0.2 del Lenguaje SV mientras el adaptador formal siga abierto.

```json
{
  "sv_version": "<autodetectada>",
  "engine": "python",
  "domain": "NLP|DEV|CUSTODIA|CUSTOM",
  "programa": {
    "observables": {},
    "horizonte": {"1": [0, 1], "2": [0]}
  },
  "traza": {
    "C_frame": [0, "U", 1, 0, 0, 0, 0, 0, 0],
    "gamma_h_labels": {"2": "fronteriza"},
    "C_gob": [0, "U", 0, 0, 0, 0, 0, 0, 0],
    "A_agente": [0, "U", 0, 0, 0, 0, 0, 0, 0],
    "U_irr": [],
    "gobernable": true
  },
  "dictamen": {
    "k3": "INDETERMINADO",
    "politica": "CONTINUAR_EN_W(T,k)"
  }
}
```

La U se preserva como cadena `"U"` en el JSON. Nunca se colapsa a 0 o 1.

## Invariantes de diseño

- Sin dependencias externas en el runner.
- Sin estado global.
- `to_dict()` devuelve copia profunda.
- El comparador excluye `engine`, `sv_version` y metadatos de implementación.
- El puente formal con la IR v0.2 del Lenguaje SV sigue abierto y queda registrado como deuda viva.
