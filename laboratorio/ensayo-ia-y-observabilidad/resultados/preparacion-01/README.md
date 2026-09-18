# Preparación EIO: candidato no compilado ni ejecutado

Se publican fuentes y esperados para revisión. No se ha resuelto el acceso pendiente, instalado herramientas, obtenido pesos, resuelto Cargo ni activado Actions. Contador material de campaña: **0/3**. Ningún caso se declara conforme por esta entrega.

- [Manifiesto](MANIFIESTO.md): componentes, identidades conocidas y pendientes.
- [Matriz de preparación](MATRIZ.md): correspondencia EIO-P-01 a EIO-P-15.
- [Huellas del candidato](ARCHIVOS.tsv): bytes UTF-8, SHA-256 y blob Git de los archivos; excluye su propia fila.
- [Banco](../../pruebas/banco.rs), [casos y esperados](../../pruebas/casos.json), [adaptador real](../../inferencia/adaptador.rs), [telemetría](../../observabilidad/telemetria.rs).

## Frontera experimental

Hay 19 casos JSON y cinco controles directos de finalización: 24 testigos preparados. El oráculo determinista es exterior al modelo; sólo puede mutar un booleano sintético. Sus respuestas inyectadas se etiquetan adaptador_directo. El otro ejecutable carga realmente Candle y pesos identificados: no sustituye la generación por respuestas prefabricadas. Se conservan los token IDs y texto generado; sólo se retira de la decodificación final el token EOS terminal, conservándolo en la lista de tokens. No se repara JSON. El prompt fijo incorpora fuentes A/B sintéticas; las referencias no pretenden acreditar recuperación documental externa.

La calidad semántica se revisará por separado: el prompt exige identificar conjuntamente A y B; una respuesta bien formada no basta. El criterio semántico y su evaluación receptora no se declaran satisfechos. No se reconstruye razonamiento interno.

## Flujo conservado inactivo

Un único workflow_dispatch diferencia preparación (resolver, sin compilar) y ensayo (--locked). El commit solicitado debe coincidir con GITHUB_SHA y el checkout; checkout fijado por SHA, contents:read, sin credenciales persistidas, cachés, upload-artifact, recolector, contenedores o secretos. Tres activaciones como máximo, sin repetición automática, 40 min cada una, concurrencia uno. Su contador run_number debe contrastarse con la campaña antes de cualquier lanzamiento; no sustituye el registro externo de run ID.

Las guardas adquisición=false y ensayo=false permanecen cerradas. No se autoriza cambiarlas mediante esta entrega. Antes de un eventual lanzamiento faltan el fundamento de acceso, revisión del aprovisionamiento y evidencia de gratuidad aplicable; no utilizar el ejecutor remoto como elusión.

## Medida prevista, no observada

Supervisor propio por grupo de procesos, muestreo cada segundo: suma simultánea de RSS (puede contar memoria compartida varias veces), disco asignado de directorio temporal y checkout, espacio libre y evidencia. No suma picos individuales. Límites: 10 GiB de disco atribuible (conteo conservador del checkout completo), reserva 2 GiB, objetivo 4 GiB de procesos de inferencia/navegador; compilación separada. 120 s por inferencia, con terminación y comprobación de residuales; limpieza posterior puede añadir 2 s. Puede haber sobrepaso entre muestras y descendientes que cambien de grupo no quedan cubiertos: no es una cota dura ni acreditación de cobertura exhaustiva. El límite de 40 min del job puede impedir conservar el cierre; ese caso no permite afirmar que se comprobaron sus procesos.

Hasta 20 MiB de evidencia por ejecución y 60 MiB de campaña; cada fichero transmitido a logs se limita a 1 MiB. Sin almacenamiento de artefactos de pago. Cargo.lock se transmitiría íntegro en base64, con tamaño y huella, sólo si cabe. Custodiar logs completos en resultados antes de recepción. Preparación, compilación y ejecución permanecen diferenciadas.

Un calentamiento y tres pares on/off, off/on, on/off: siete inferencias previstas, bajo doce. El conductor de costes conserva individuales, mediana, mínimo, máximo, rango y diferencias pareadas; comprueba igualdad de tokens. El tiempo interno incluye carga y tokenización, excluye parte de arranque/exportación: no mide todo el coste de proceso. RSS se conserva individualmente; si falta medida no se acredita. No extrapolar fiabilidad poblacional.

## Pendientes que impiden declarar listo el ensayo

Cargo.lock, transitivas/características/licencias, integridad de rustup antes de invocarlo, hashes de distribución Rust y tokenizador/configuración/plantilla, tamaño exacto y verificación local de pesos, aprovisionamiento repetido de ejecutores efímeros sin resolver versiones nuevas, recuperación completa de evidencia, APIs Candle/OpenTelemetry/tokenizers, compilación y todos los resultados. El archivo COMPONENTES_VERIFICADOS.sha256 no existe deliberadamente. El script de ensayo falla si faltan estas identidades o las entradas; no las inventa.

El [enlace mínimo de navegador](../../pruebas/NAVEGADOR_PENDIENTE.md) también es candidato. El workflow no automatiza todavía su ejecución real. Reloj std::time::Instant, WASM transitivo, exportador síncrono, memoria lineal, navegador/versiones y supervisor quedan por comprobar. Una construcción WASM no acredita EIO-P-12. No acredita Windows, WASI ni memoria del PC.

No cambia el contrato, la matriz, las sedes canónicas, S32/BIS-03 ni las reservas científicas. Las entregas precedentes se conservan. Punto de parada: revisión receptora de fuentes.
