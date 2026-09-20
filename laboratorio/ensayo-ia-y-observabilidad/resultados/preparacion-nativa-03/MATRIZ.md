# NAT03 · matriz y reservas

Fuentes no compiladas ni ejecutadas. Los reparos recibidos eran estáticos; no se afirma haber reproducido sus fallos.

| Reparo | Cambios preparados | Comprobación fuente prevista | Límite |
|---|---|---|---|
| NAT02-A | tabla por caso, identidad de estímulo, prefijos y recibos de trabajos, error por fase, límites/progreso globales | api-real, oraculos_nat03, cuatro tests de sensibilidad añadidos a frontera | pendiente compilar/ejecutar; inspección no acredita carreras ni durabilidad |
| NAT02-B | capturar fallo de registro waitpid, conservar reap real, error de observación sin salida inadvertida, diagnósticos acotados | escritor_desconectado, terminal API, señales/reap y cierre exterior | API puede ser imposible por fallo de transporte; categoría explícita no conforme, evidencia exterior obligatoria |
| Siguiente fase | ficha inactiva con gasto cero y cotas | habilitación receptora previa; comprobación de cgroup y control superior | cuota/permisos/plataforma no comprobados |

Las correcciones R1/R2/R3 y las reservas B01–B08 de [NAT02](https://github.com/juantoniolloretegea/SV-motor/tree/d82bbe2f5f396eb31da5b095ba0849404cb352b0/laboratorio/ensayo-ia-y-observabilidad/resultados/preparacion-nativa-02/MATRIZ.md) siguen vigentes. En particular: lock/transitivas/licencias/compilación/imagen pendientes; viabilidad de guarda y D-state no probados; RSS aproximada, no PSS ni límite duro; pérdidas tras corte declaradas; coste propio de manifiesto y otros costes no medidos; semántica completa de trazas no demostrada; gasto cero/autenticación pendientes; ninguna evidencia dinámica NAT03.

No se declara reparo recibido como resuelto por la unidad productora. La corrección en fuente y la concordancia documental sólo preparan su revisión.

Se conserva literalmente Cargo.toml, inferidor, adaptador, verificador, telemetría, entradas/modelo/contexto/cuantización, interfaz, guarda y parada. Se modifica sólo la copia sucesora autorizada. S32/BIS-03, S37/S38, núcleo y registros canónicos quedan intactos.

La futura recuperación receptora, los once tests de frontera, 35 controles JSON, 24 del banco y trece casos HTTP no se han ejecutado. No hay aceptación científica, privacidad acreditada, servicio disponible ni solución nueva al pico de memoria.
