# Capa Python de verificación y doble vara

## Objeto

Declarar el rol, la arquitectura y los límites de la capa Python de verificación del Sistema Vectorial SV.

## Posición en el motor

El motor de ejecución canónico del SV es el backend Rust compilado desde código `.svp`. La capa Python no lo reemplaza. Su función es producir, en Python puro, una salida JSON canónica idéntica en estructura a la que el backend Rust producirá. Eso la hace útil como herramienta de verificación, no como herramienta de ejecución primaria.

## Tres usos legítimos

**1. Verificación cruzada.** Un programador no experto en SV puede ejecutar el runner Python, obtener JSON, y compararlo con la salida Rust del `.svp` usando el comparador. Si coinciden, la implementación Rust es algebraicamente correcta.

**2. Doble vara.** Dos implementaciones del mismo álgebra — Python y Rust — produciendo JSON idéntico es la prueba más sólida de corrección. Si difieren, el comparador señala el campo exacto y el valor en cada motor.

**3. Universalidad.** Python + JSON hace el SV accesible a cualquier ecosistema — R, Julia, JavaScript, sistemas embebidos — sin modificar la especificación algebraica ni comprometer la ejecución Rust.

**4. Cotejo de desconfianza razonable.** Un desarrollador que aún no conozca el Lenguaje SV puede inspeccionar el artefacto Python derivado, ejecutar el laboratorio y verificar que la salida coincide con el JSON canónico asociado al `.svp`. Esto convierte a Python en instrumento de verificación y difusión, no en sede soberana del programa.

## Separación entre Python y `.svp`/Rust

| Propiedad | Python (verificación) | `.svp` + Rust (ejecución) |
|---|---|---|
| Rol | Verificar corrección | Ejecutar en producción |
| Compilación | Ninguna | Backend Rust soberano |
| Memoria | Gestión Python | Punteros Rust, sin GC |
| Runtime | Python ≥ 3.10 | Sin runtime externo |
| JSON output | Canónico | Canónico (idéntico en campos algebraicos) |
| Velocidad | Secundaria | Primaria |

## Esquema JSON canónico

El esquema es compartido entre Python y Rust. Los campos algebraicos deben ser idénticos en ambas salidas para una misma entrada:

```json
{
  "sv_version":   "0.1.5",
  "engine":       "python",
  "domain":       "NLP|DEV|CUSTODIA|CUSTOM",
  "programa": {
    "observables": {},
    "horizonte":   {"1": [0, 1], "2": [0]}
  },
  "traza": {
    "C_frame":          [0, "U", 1, 0, 0, 0, 0, 0, 0],
    "gamma_h_labels":   {"2": "fronteriza"},
    "C_gob":            [0, "U", 0, 0, 0, 0, 0, 0, 0],
    "A_agente":         [0, "U", 0, 0, 0, 0, 0, 0, 0],
    "U_irr":            [],
    "gobernable":       true
  },
  "dictamen": {
    "k3":         "INDETERMINADO",
    "politica":   "CONTINUAR_EN_W(T,k)"
  }
}
```

La U se preserva como cadena `"U"` en el JSON. Nunca se colapsa a 0 o 1.

## Invariantes de diseño

- Sin dependencias externas. Solo stdlib + `sv_motor.algebra`.
- Sin estado global. Cada llamada a `run_nlp`, `run_dev`, `run_custodia` o `run_custom` es pura.
- `to_dict()` devuelve copia profunda — segura para mutación en tests y comparadores.
- El comparador excluye los campos `engine`, `sv_version` y metadatos de implementación.
- Los campos algebraicos comparados son: `traza.C_frame`, `traza.gamma_h_labels`, `traza.C_gob`, `traza.A_agente`, `traza.U_irr`, `traza.gobernable`, `dictamen.k3`, `dictamen.politica`.

## Uso mínimo

```python
from sv_motor.verification import run_nlp, compare

result = run_nlp({
    "theta": "coherente", "pi": "sin-pregunta", "kappa": "coherente",
    "eta": "completa",    "gamma": "alineada",  "alpha": "apropiada",
    "mu":  "sin-ambiguedad", "chi": "sin-solicitud", "psi": "en-curso",
})
print(result.json_canonical())

# Doble vara con la salida del backend Rust:
# verification = compare(result.json_canonical(), rust_json_string)
# assert verification.verificado, verification.discrepancias
```

## Deuda viva de esta capa

- La comparación con el backend Rust real quedará pendiente hasta que DV-SVM-002 esté implementado.
- El runner CUSTOM acepta cualquier vector {0,1,U}^n — no valida que el vector provenga de un transductor de dominio declarado. Esa validación es responsabilidad del programador.
