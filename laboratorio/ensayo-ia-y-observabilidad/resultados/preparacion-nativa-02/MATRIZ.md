# Matriz de corrección y reservas

Todas las fuentes son **no compiladas y no ejecutadas**. La inspección estática productora no equivale a recepción.

| Reparo | Corrección preparada | Pruebas fuente | Estado separado |
|---|---|---|---|
| R1 | Admision común para cierre/API; UI sigue indicador; original/juicio separados | frontera, api-real R1-01..05/P, INTERFAZ.js | Corregido en fuente; aceptación y comportamiento pendientes |
| R2 | Barrera FIFO, drenaje finito, Disco consumido al sellar, snapshot fijo y sello obligatorio; tardíos descartados con laguna explícita | cola_pendiente, eof_ausente, fallo_sync, bloqueo_sellado; recuperación concurrente y repetida | Corregido en fuente; fsync/carreras/cotejo dinámico pendientes |
| R3 | Custodia en hilo sin espera del controlador; hilo TERM/KILL con deadline250ms; guarda cgroup exterior inactiva | FIFO antes/entre; observador; muerte de supervisor y hoja vacía | Corrección y mecanismo preparados; viabilidad/permisos/latencias no demostrados |

No se declara «reparo recibido como resuelto» por la propia ejecutora. La recepción decidirá.

## Brechas heredadas

| Brecha | Estado NAT02 |
|---|---|
| B01 lock, árbol, licencias, compilación, binarios/imagen | Pendiente; ninguna dependencia nueva, lock antecedente intacto, sin inventar resolución |
| B02 guarda exterior, D-state, muerte de supervisor, residuales | Guarda concreta preparada; disponibilidad/permisos no comprobados; muerte de la propia guarda/migración privilegiada/kernel bloqueado requieren control superior |
| B03 RSS aproximada, sondeo y latencia | 4 GiB conservados; hoja completa más guarda; no PSS/límite duro; sin medidas |
| B04 custodia, colas, FS, recuperación | Corrección R2 preparada; pérdidas postcorte declaradas; falta ejecutar casos y recuperación independiente |
| B05 costes incompletos | RAM de hilos/snapshot/guarda contabilizable; coste final de manifiesto/PSS/CPU detallada todavía desconocidos |
| B06 semántica de resultado/trazas hostiles | No ampliada fuera del encargo; oráculo técnico/inventario y controles existentes, no validación exhaustiva de emisor hostil |
| B07 cuenta/gasto cero/autenticación/imagen | Pendiente; sin abrir Codespaces, cambiar cuenta o elegir nuevo alojamiento |
| B08 evidencia dinámica | Cero compilaciones/pruebas/servicios/inferencias de NAT02; interfaz alternativa Qwen retirada, modelo conservado |

## Ajustes explícitos de recursos y contrato

EIO-NAT/2 sucede a /1 por recuperación ligada a sello y estados de custodia. Mismas peticiones sintéticas/verificador/modelo/contexto. Añade cola de custodia64, snapshot hasta ~20MiB y proceso guarda; incluye overhead en RSS total, nunca eleva4294967296. Custodia permanece dentro del proceso supervisor, medida conjunta. Observador de pruebas debe pertenecer a la hoja; cliente/CLI/navegador externos se declaran como instrumentos, sin presentar RSS experimental como consumo completo del host.

Recepción tardía: se elige descartar con advertencia permanente de laguna y contador volátil, no suplemento. Esta limitación es intencionada y requiere revisión receptora. Error de sello bloquea descargas API, aunque existan archivos parciales recuperables manualmente.

## Revisiones estáticas efectuadas

Se contrastaron tres reparos y rutas del código; retirada de write/sync del controlador, condición única, sello/ACK, clausura por propiedad del escritor, FIFO real de inyección y deadline independiente. Se revisaron parámetros de medición para incluir guarda/huérfanos, manejo de PID antes de escalada, rutas include_str y versión de contrato. JSON parseado, JS analizado sin invocación, copias y cambios cotejados por contenido/tamaño/hash. Nada de ello acredita compilación Rust ni las pruebas negativas.

S37/S38, CSV/Markdown, S32/BIS-03 y núcleo SV intactos. Sin aceptación científica, privacidad acreditada, servicio disponible ni resolución del pico de memoria.
