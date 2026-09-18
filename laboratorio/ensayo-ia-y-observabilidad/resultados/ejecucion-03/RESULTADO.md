# Resultado de la única preparación v3

**No satisfactoria: startup_failure antes de iniciar trabajos.**

[Run 35342612464](https://github.com/juantoniolloretegea/SV-motor/actions/runs/35342612464), intento 1, workflow_dispatch, commit 0cd2721adb44fc9d578aff4393028c9979bb2252. Creado 2026-09-18T12:02:39Z, actualizado 12:02:40Z. La API devuelve status=completed, conclusion=startup_failure y jobs=[]; la UI identifica Invalid workflow file. Se conservan [metadatos y anotación literal](RUN_35342612464.json).

## Causa y corrección

Error propio de implementación: uso de la expresión runner.temp en env del trabajo, contexto no admitido por GitHub. Error literal mostrado por GitHub:

> (Line: 26, Col: 25): Unrecognized named-value: 'runner'. Located at position 1 within expression: runner.temp, (Line: 27, Col: 19): Unrecognized named-value: 'runner'. Located at position 1 within expression: runner.temp, (Line: 28, Col: 20): Unrecognized named-value: 'runner'. Located at position 1 within expression: runner.temp, (Line: 29, Col: 15): Unrecognized named-value: 'runner'. Located at position 1 within expression: runner.temp

Se retiran las cuatro expresiones de env y se calculan las rutas a partir de RUNNER_TEMP dentro del primer paso Bash, escribiéndolas en GITHUB_ENV para los pasos posteriores. No cambian destino, componentes, permisos o presupuesto. Archivo corregido: .github/workflows/ensayo-ia-observabilidad.yml, SHA-256 f9f1e233f2b2629f10c88c6ff0d3986a57f4a670e83034120545d375e51fcb19. La corrección se publica **sin relanzarla**; no se declara validada por GitHub ni ejecutada.

## Evidencia y alcance

No hubo job que ejecutase la instalación, Cargo o comprobación de versiones. No existen stdout/stderr de esos comandos ni código de retorno de proceso: startup_failure es conclusión de plataforma, no exit code. Rust exacto observado: ninguno. Cargo.lock, DEPENDENCIAS.json y COMPONENTES.tsv no se generaron. No se descargaron componentes por este flujo; no se probó desde el runner el acceso a la distribución. No atribuir el fallo a red, Rust o modelo.

Recursos del ejecutor no medidos porque los pasos no comenzaron. No inferencia, compilación, casos ni navegador ejecutados. Medidas y esperado experimental pendientes. El trabajo remoto figura terminado y no tiene jobs; no se lanzó proceso local persistente.

**Contador acumulado: 1/3**, incluida esta activación fallida; cero ensayos. La autorización de v3 permitía una única preparación. No se reutilizan las dos activaciones restantes para repetirla automáticamente. Queda pendiente decisión receptora para otra preparación; el ensayo sigue cerrado. Esta entrega no solicita ni presume una ampliación del presupuesto total.

Las correcciones de P01/P02/P04 y el aprovisionamiento preparatorio se publicaron antes del intento y siguen sin compilar. P03 experimental (aprovisionamiento de pesos/tokenizador/WASM por cada runner) permanece pendiente; no se promueve el candidato. Antecedentes, inventarios y revisión receptora se conservan. No aceptación experimental ni cambio a S32/BIS-03.
