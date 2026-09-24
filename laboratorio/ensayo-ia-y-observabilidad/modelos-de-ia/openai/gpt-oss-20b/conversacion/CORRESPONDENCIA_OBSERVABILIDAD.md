# Correspondencia de observación y control con las rutas documentadas

Corte: 24/09/2026. Se cotejan diagramas via-a.mmd y via-b.mmd de SV-motor y el inventario de la vía B de 22/09/2026. La herramienta recordada es OpenTelemetry Rust 0.31.0. La ejecución actual pertenece a la familia nativa B; la página web no transforma la inferencia en ejecución WASM.

| Necesidad | Realización actual | Alcance y pendiente |
|---|---|---|
| Instrumentación y correlación | OpenTelemetry Rust; marcas de peticiones, supervisión y exportación local | Puntos instrumentados; sin recolector exterior ni cobertura exhaustiva |
| Observación de procesos | Muestreo Linux cada cinco segundos, identidad por PID y tiempo de inicio, CPU, RSS, hilos, E/S, archivos y sockets | 0.2.2 añade el cgroup del motor residente, que no era descendiente del servidor. No captura tráfico ni garantiza detectar procesos más breves que el periodo |
| Presupuesto exterior | Unidades systemd, cotas de memoria de 32 GiB y 2 GiB, sin intercambio, plazos y parada vinculada | No equivale a todas las pruebas de guarda independiente y latidos del diseño NAT03 |
| Cancelación | Supervisor independiente del escritor; parada del motor y comprobación MainPID=0 | Cancelación real de carga/cálculo comprobada; no auditoría de todos los fallos posibles |
| Conservación | JSONL sincronizado, cadena de huellas, originales y exportación | No es custodia independiente sellada con ACK, fragmentos y copia inmutable conforme al diseño completo |
| Contrato del dominio | Banco documental externo con criterio exacto | El chat libre no invoca el verificador contractual general ni acredita recorrido profesional de fuentes |
| Presentación | HTML/CSS/JavaScript y API Rust | Egui y representación de polígonos no integrados; conservan su sede futura |
| Acceso | URL privada de GitHub y túnel SSH al servicio local | No autenticación profesional individual; disponibilidad dependiente del Codespace |
| Vía A | Diagrama de Worker Rust/WASM, supervisor y custodia de NAV-02 revisado | No desplegada por esta intervención; se preservan sus obligaciones y límites documentados |

No se confunde la presencia de telemetría con aislamiento, ausencia de comunicaciones, veracidad de contenido ni conformidad del conjunto. Las necesidades pendientes permanecen identificadas; no se cierran por habilitar el chat o completar pruebas semánticas.

La versión 0.2.2 carga la derivación del tokenizador conservando los IDs existentes y completando exclusivamente huecos declarados. La guardia comprueba todas las declaraciones añadidas y los cinco delimitadores utilizados por Harmony. 21 pruebas unitarias aprobadas, incluidas conservación de IDs y rechazo de colisiones. El archivo original permanece intacto. El guion de validación necesitó además un identificador de petición nuevo por ejecución: el servicio había rechazado correctamente reutilizar el de un control previo.
