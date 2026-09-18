# Segundo lanzamiento preparatorio · EIO v3

Autorizada la adenda REINTENTO_PREPARACION_01 del commit 71b70e039165badfa31aeaa69b31867fa677598a. Corte público base 2e3d8527bb2b8826331e514ce5cbbcc01a11993d. Inicio observable 2026-09-18T16:16:10.4552164+02:00 (reloj local; no sello de recepción humana).

Se conserva el [precompromiso previo](https://github.com/juantoniolloretegea/SV-motor/blob/0cd2721adb44fc9d578aff4393028c9979bb2252/laboratorio/ensayo-ia-y-observabilidad/resultados/ejecucion-03/PRECOMPROMISO_PREPARACION.md), sus componentes, fuentes y cotas, salvo el contador actualizado: preparación sólo número2, ensayo sólo número3; run_attempt=1, máximo acumulado3, sin recrear flujo ni Re-run. El primer run 35342612464 permanece concluido con startup_failure y jobs vacíos. La UI antes de publicar muestra exactamente 1 workflow run y ningún activo.

## Identidades previas

| Archivo | Identidad |
|---|---|
| .github/workflows/ensayo-ia-observabilidad.yml | Blob 3c4d4978e71fb3733a83ec111a242fa51dbb1b59; 2805 bytes UTF-8; SHA-256 e366706d5fb50e850f7598b29fa16d5472be61c4cad8d9686b22be9c39aa8116. Recalculado localmente sobre texto obtenido del corte base. |
| pruebas/preparar.sh | Blob 50e100acb500b4bb450675934962129aa7945452 en el mismo corte; sin cambio. |
| pruebas/estado-acceso.json | Se actualiza sólo fundamento de autorización; adquisición=true, ensayo=false. Su identidad queda fijada por este commit. |

Flujo ya corregido: rutas RUNNER_TEMP en primer paso y GITHUB_ENV; no expresiones runner.temp en env de trabajo. Lectura estática, sin alegación de validación completa de GitHub. El SHA solicitado debe coincidir con GITHUB_SHA y HEAD.

Una preparación ordinaria en Ubuntu24.04 estándar público, sin servicios/secretos/cachés/upload-artifact ni instalación local. Se reutiliza la comprobación de gratuidad y la vía de logs/resumen ya documentadas, sin nuevos pagos. Paquetes oficiales rustc/cargo/rust-std 1.98.0 Linux x86-64; HTTPS y checksum oficial antes de extracción/instalador. No usar Rust del PATH. Una fuente compartida para paquete/suma no equivale a independencia. Versiones/bytes/sumas de componentes siguen pendientes de adquisición real.

Resolver Cargo.lock y metadata --locked; no compilar, ejecutar banco, descargar pesos o inferir. Recuperar íntegros lock, DEPENDENCIAS.json y COMPONENTES.tsv, cada uno con tamaño/SHA/base64; no reconstruir logs truncados. Evidencia publicada antes de ensayo.

Se conservan 40min, plazo global2280s y margen de cierre, supervisor de preparación1800s, disco10GiB/reserva2GiB, evidencia20MiB por ejecución/60MiB campaña. Muestreo1s y cobertura del grupo de procesos no son límites duros. Presupuesto local100MiB y30GiB libres. Las correcciones del banco aún no están compiladas.

Esperado: retorno0 y manifiestos íntegros, Rust1.98.0 observado. Si falla esta segunda preparación, conservar error/punto/retorno y detenerse; número3 no está autorizado para otra preparación. Si es satisfactoria, completar P03 y demás requisitos, publicar y releer corte experimental antes del único ensayo restante. No se aceptan anticipadamente compatibilidad, navegador, calidad ni estado científico. Antecedentes intactos.
