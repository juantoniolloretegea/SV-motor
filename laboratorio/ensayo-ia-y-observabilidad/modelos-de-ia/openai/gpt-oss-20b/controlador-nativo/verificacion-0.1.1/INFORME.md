# Controlador nativo 0.1.1 · recursos y parada
Fecha: 23 de septiembre de 2026.

## Resultado
**16 pruebas registradas superadas: 14 funcionales y dos auxiliares.** Compilación de producción con Rust/Cargo 1.98.0 y dependencias fijadas. No se cargaron el motor ni los pesos de gpt-oss; los procesos y el servidor HTTP del banco son auxiliares Rust declarados. La versión 0.1.0 y sus evidencias se conservan como antecedente.

## Evidencia específica
- `RLIMIT_AS` aplicado por Linux: el hijo confirma una cota dura de 128 MiB y falla una reserva virtual de 256 MiB, sin ocupar esa memoria.
- Muerte abrupta del padre: el banco mata realmente el proceso padre, recoge al hijo y comprueba señal 9. No usa la salida normal del controlador para provocar ese cierre.
- Conservación incremental: un lector independiente recupera dos muestras ya escritas mientras el hijo sigue vivo. La segunda incluye RSS observada.
- Fallo y bloqueo del escritor después del arranque: se identifica el PID creado antes de inyectar el fallo y se comprueba su desaparición. Se conserva el resultado de custodia no conforme.
- Permanecen las comprobaciones de petición única, propiedad del puerto, JSON inválido, plazos, escalada TERM/KILL, SIGTERM del controlador y del hijo, error real de envío de señal, exclusión y limpieza durante pánico.
- Capacidad ausente o insuficiente rechazada. [Lectura del entorno local](CAPACIDAD_LOCAL.json) obtenida con el ejecutable de producción y `--capacidad`; no describe Codespaces ni PC/WSL2.

Durante el desarrollo se corrigieron dos defectos del banco: la lectura de PDEATHSIG desde el hilo nuevo del ejecutor de pruebas no representaba la configuración del hilo inicial; y un lector concurrente analizaba una última fila todavía incompleta. La prueba material de muerte del padre y la lectura exclusiva de filas completas evitan esas inferencias. No se atribuyen esos defectos al motor.

## Memoria del modelo
El registro anterior anunció 13 123 MiB, equivalentes a 13 760 462 848 bytes. La lectura anterior al arranque fue de 13 437 063 168 bytes disponibles: diferencia de 323 399 680 bytes. No hubo muestras RSS del modelo durante aquella carga. Esa comparación no demuestra el consumo máximo ni identifica el emisor del SIGTERM.

En la revisión exacta de mistral.rs 0.9.3, los expertos MXFP4 conservan bloques y escalas empaquetados. La alternativa CPU de `gather_forward_dequantize` crea copias de bloques y escalas y vectores auxiliares; no construye por ese recorrido una matriz completa BF16 de todos los expertos. Por ello no procede equiparar inventario, tamaño de archivos, espacio virtual y máximo residente.

Fuentes: [carga gpt-oss](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-core/src/models/gpt_oss.rs) y [MXFP4](https://github.com/EricLBuehler/mistral.rs/blob/24dbf5c256f232176ee5949485ba264049407fbe/mistralrs-quant/src/mxfp4/mod.rs). El formato Harmony permanece a cargo del motor ya seleccionado.

## Alcance
La admisión utiliza aquella estimación de carga y 1 GiB de reserva. Es un filtro conservador con disponibilidad instantánea, no una prueba de suficiencia. La cota virtual debe fijarse explícitamente para la instalación; no se confunde con RAM física. Las muestras de proceso y grupo visible conservan cobertura declarada y pueden quedar incompletas.

La terminación por muerte del padre cubre el hijo directo mientras conserve la configuración. No acredita contención de descendientes, custodia fuera del proceso ni una guarda exterior frente a bloqueo del anfitrión. SIGKILL puede impedir escribir el cierre final. Ninguna comprobación emplea modificaciones administrativas de permisos o cgroups.

La ejecución futura con el modelo sigue pendiente de capacidad efectiva y de una ventana concreta. Esta publicación no inicia esa ejecución ni declara integración conforme.

[Pruebas](PRUEBAS.log) · [Compilación](COMPILACION.log) · [Herramientas](HERRAMIENTAS.txt) · [SHA-256](SHA256SUMS.txt) · [Verificación anterior](../verificacion/INFORME.md).

Sistema Vectorial SV · [Aviso de licencias](../AVISO_LICENCIAS.json).
