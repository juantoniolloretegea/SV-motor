# Matriz de requisitos y brechas

Estado común: fuentes preparadas y revisión estática productora; **no compiladas ni probadas**. La matriz no emite recepción.

| Requisito | Fuente/evidencia de preparación | Estado y límite |
|---|---|---|
| Separación web/inferencia/supervisión | nativa/servidor.rs, inferidor.rs, supervisor.rs; DISENO | Implementado en fuente Linux; aislamiento SO no probado |
| HTTP mantenido, dependencias explícitas | Cargo.toml, HABILITACION | Axum/Tokio seleccionados; nuevo lock y compatibilidad global pendientes |
| Peticiones fijadas, JSON heredado | pruebas/lib.rs, peticion.txt, json_complementario.rs | Copias y controles preservados; bancos no ejecutados aquí |
| Una tarea, estado/cancelación | Orden, Cierre, supervisor; frontera.rs | Implementado; carreras reales, latencias y procesos residuales pendientes |
| Terminación durante load/forward | killpg exterior + testigo bloqueo | Fuente concreta; guarda contra muerte del supervisor no implementada |
| Umbral 4 GiB con observador | medir; DISENO | Métrica/perímetro fijados; muestras/medidas inexistentes, no límite duro |
| Eventos incrementales, custodia | exportador, emitir, lector, Custodia | Implementado; cola/fsync/fallos no probados; pérdidas impiden completitud |
| Evidencia recuperable | endpoint limitado, cotejo.rs | Manifiesto original previsto; ninguna recuperación real de esta candidata |
| Costes/tiempos | supervisor journals/manifiesto | Campos previstos; coste propio del manifiesto/PSS/CPU detallada desconocidos |
| Texto original y juicio | inferidor, app.js | Separados; textContent/CSP; verificación visual pendiente |
| Seguridad de URL | bind loopback, Origin/Host, plan privado | Autenticación de plataforma pendiente; no servicio disponible |
| Orden e identidad SV | controles conservados; CONTRATO | No células ni vectores nuevos; nunca error→U |
| Codespaces y economía | configuración inactiva, HABILITACION | Cuota/saldo/barrera efectiva no verificados; no crear infraestructura |
| Preservación/sede | manifiesto, diff de publicación | Sólo carpeta permitida; índice/respuesta privados; originales intactos |

## Brechas bloqueantes antes de un modelo real

B01: nuevo lock/árbol transitivo, licencias, compatibilidad y compilación pendientes. No hay hashes de binarios ni imagen fija.

B02: control exterior independiente del supervisor ante SIGKILL/OOM, procesos D-state, grupos que escapen y confirmación de cierre de toda la familia. La fuente sólo usa grupos de sus hijos y Drop acotado. No afirmar contención hostil.

B03: contabilidad por /proc aproximada, descendientes fugaces o lectura fallida, E/S potencialmente bloqueante y latencia no acotada duramente. Falta prueba de excedencia y cuantificación del sobrepaso. No modificar umbral para aceptar un resultado.

B04: recepción/persistencia incremental pendiente de prueba; cola llena puede perder bytes ya leídos no encolados, dejando incompleto. Falta inyección real de disco/FS desechable, cotejo receptor y demostración de custodia tras muerte abrupta. El caso cola puede alcanzar otro límite primero; acreditar la rama exacta.

B05: costes parciales explícitos: no coste de escritura/hash del propio manifiesto, CPU por función, PSS o navegador. Estos huecos no se rellenan con cero. Integridad no acredita cobertura.

B06: oráculo exterior comprueba inventario/ID/secuencia y pérdidas, pero no todo el esquema semántico de resultado ni cada relación de trace/padre en un emisor hostil. Banco heredado comprueba correlación en su perímetro; no extrapolar. Revisar y ampliar antes de afirmar resistencia frente a tráfico manipulado.

B07: cuota real, gasto adicional cero, plataforma privada/autenticada, host/origin efectivo, imagen/recursos y mecanismo de parada/borrado no comprobados. El entorno no existe por esta preparación.

B08: sin evidencia dinámica N01–N18. Testigos preparados no demuestran calidad de inferencia. La propuesta de interfaz Qwen se recibirá separadamente, sin bloquear esta preparación ni conectarla ahora.

## Revisión estática realizada

Se contrastaron autoridad y recepción publicadas, cortes fuente, identidades documentales, archivos copiados, modificaciones derivadas, versiones de nuevas dependencias, rutas include_str, límites y flujos de cierre. Se corrigieron durante preparación la espera indefinida en Drop, la distinción de completitud tras revocación y la conservación acotada de fragmentos incompletos. No se ejecutó rustc, cargo, Bash, testigos, servidor ni herramienta de infraestructura.

La sintaxis JavaScript se analizó sin ejecutar la interfaz; JSON documental se parseó y los hashes se calcularon en memoria. Eso no verifica Rust, comportamiento de navegador, señales, filesystem ni economía. No hay dictamen de aceptación ni cambio del estado científico.
