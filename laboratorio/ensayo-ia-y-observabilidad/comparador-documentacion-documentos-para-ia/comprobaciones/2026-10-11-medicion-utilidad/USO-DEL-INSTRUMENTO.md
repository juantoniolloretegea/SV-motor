# Registro y cálculo en Rust

El programa es un instrumento de esta comprobación, separado de los componentes operativos. No llama a modelos ni ejecuta el comparador. Reutiliza `serde_json 1.0.149` y `sha2 0.10.9`; `Cargo.lock` fija la resolución completa.

## Preparación

`cargo build --locked --offline --manifest-path codigo/Cargo.toml`

La operación `preparar RAIZ SALIDA` reconstruye los materiales desde los antecedentes locales fijados, comprueba originales, traducciones y distribución, y rechaza una salida ya existente. Los materiales publicados permiten realizar la revisión sin reconstruir los antecedentes.

## Registro temporal

`bsd-medicion-utilidad iniciar materiales/PLAN.json I02 registros/I02-inicio.json`

Tras conservar la respuesta:

`bsd-medicion-utilidad cerrar registros/I02-inicio.json registros/I02-final.json`

El directorio de registros debe existir. Ambos archivos se crean exclusivamente si no existen; el registro inicial no se sobrescribe. El reloj es exterior al modelo. Los intervalos de distintas sesiones no deben solaparse: la revisión es secuencial. Las pausas o fallos se anotan como incidencias; no se eliminan del historial ni se descuentan sin una medición identificable.

El archivo agregado contiene `{"sesiones": [...]}`. Cada entrada usa los siguientes campos:

```json
{
  "id": "I02",
  "dominio": "inmunologia",
  "condicion": "ordinaria",
  "estado": "completa",
  "inicio_unix_ms": null,
  "fin_unix_ms": null,
  "fuente_reloj": null,
  "espera_instrumental_ms": null,
  "revision_avisos_ms": null,
  "tokens_observados": null,
  "fuente_tokens": null,
  "decision": "compatibilidad",
  "justificacion": "Sustituir por la explicación efectivamente emitida.",
  "citas": ["Cita literal del documento propuesto", "Cita literal de la referencia"],
  "fundamento_contrastado": null,
  "nota_contraste": null
}
```

La plantilla ilustra el formato; no es una respuesta ni una sesión realizada. `decision` acepta `incompatibilidad`, `compatibilidad` o `contexto_insuficiente`; en un fallo queda `null`. `fundamento_contrastado` acepta `true`, `false` o `null` y se completa después de examinar las citas y la explicación, no por coincidencia de etiquetas. Se conserva por separado la respuesta literal original y la evaluación posterior.

## Cálculo

`bsd-medicion-utilidad calcular materiales/PLAN.json materiales/evaluacion/REFERENCIAS.json REGISTRO.json RESULTADO.json`

Se rechazan sesiones duplicadas, identificadores desconocidos, mezcla de dominios o modalidades, relojes invertidos, duraciones imposibles y declaraciones de tokens sin fuente. Las sesiones fallidas no producen un resultado evaluable. Los valores ausentes y los denominadores nulos no generan ceros ficticios. La salida no impone un dictamen de utilidad.

El instrumento verifica estructura y aritmética. La pertinencia científica de citas y explicaciones se contrasta por separado. Los porcentajes de comparación requieren tiempos completos; el rendimiento fundado requiere además el contraste de todos los fundamentos. Los casos incompletos conservan su recuento explícito.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
