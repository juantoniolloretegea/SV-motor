# Conversación nativa 0.1.4 · verificación local
Fecha: 23 de septiembre de 2026.

## Resultado
Fuentes candidatas compiladas con Rust/Cargo 1.98.0, dependencias fijadas y Candle `ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a`. **16 pruebas registradas superadas: 15 funcionales y un auxiliar.** Las dos pruebas HTTP que requieren los pesos permanecen explícitamente ignoradas. La compilación de producción está documentada por separado.

La verificación se realiza en un contenedor Linux local de revisión. No corresponde a una ejecución en Codespaces ni en el PC/WSL2 previsto. No se sustituyó el ejecutable distribuido 0.1.3 y no hubo nuevas inferencias.

El primer intento de compilación de producción encontró un objeto vacío de candle-nn al construir su archivo. Se conservaron el diagnóstico y el intento fallido; había espacio libre y no se atribuye una causa no comprobada. Se retiraron únicamente los productos de compilación de ese paquete y la reconstrucción secuencial terminó correctamente. La compilación final incorpora una reserva instrumental de 37 MiB, incluidos los archivos de estado. [Primer intento](COMPILACION_INTENTO_01.log) · [Reconstrucción](COMPILACION_RECONSTRUCCION.log).

## Corrección y comprobación
| Problema | Cambio | Evidencia y límite |
| --- | --- | --- |
| Registro y observación en el recorrido de control | Hilo propietario del hijo, independiente de la base de datos y OpenTelemetry | Un hijo real termina por tiempo mientras se mantiene bloqueado el mismo mutex de registro usado por la aplicación. Se ejercitan también RSS, salida, cancelación, señal 15 y fallo comunicado de conservación. |
| Esperas de lectura y diagnóstico | Líneas y cola acotadas; diagnóstico limitado; consulta de finalización del control | El banco verifica las causas de parada con procesos Rust. No simula una inferencia Candle ni un bloqueo del núcleo. |
| Terminación ambigua y memoria ausente | Señal y código separados; RSS opcional y contadores de muestras | La terminación por SIGTERM se conserva como señal 15. Una ausencia de medición no se convierte en cero. |
| Escritura incompleta | Recuperación expresa en directorio nuevo, copia íntegra y manifiesto SHA-256 | Se añade una cola JSONL incompleta real, se rechaza la apertura ordinaria y se valida el prefijo derivado; original idéntico. |
| Presupuesto insuficiente para cerrar | Cota individual y reserva de cierre de 4 MiB; cómputo inicial de archivos auxiliares y reserva instrumental | Una escritura parcial por encima del margen se rechaza y el cierre de una petición real del banco se conserva y reconstruye. Presupuesto lógico, no cuota física de disco. |
| Identidad de entradas sólo al inicio del servicio | SHA-256 del tokenizador recibido y del mismo descriptor GGUF que utiliza el hijo | Revisión de código y compilación. No se ha ejecutado esa carga con pesos en esta revisión. No protege de escrituras concurrentes sobre el archivo abierto por un anfitrión privilegiado. |
| Dirección de escucha por defecto | Bucle local `127.0.0.1:3000` | La clave de sesión sigue sin autenticar personas; acceso remoto sujeto a controles de plataforma. |

## Límites conservados
El monitor sigue dentro del proceso de servicio: no constituye la guarda exterior, un custodio independiente ni una cuota agregada de recursos. Un registro bloqueado puede impedir entregar el resultado al usuario aunque el hijo ya haya terminado. La parada confirmada corresponde al hijo directo. La cadena local no detecta por sí sola la supresión de un sufijo completo sin referencia externa.

La disponibilidad del observador continúa comprobándose principalmente en la admisión; no se declara corregida toda la cobertura de observación durante la generación. No se han verificado nuevamente el acceso externo, la ejecución de JavaScript ni la continuidad de navegador. Los resultados semánticos históricos no cambian.

## Reproducción y procedencia
`cargo test --locked --offline -- --test-threads=1` y `cargo build --release --locked --offline -j 2`, con rustc, cargo y rustdoc 1.98.0 explícitos. El modo sin red requiere dependencias ya disponibles.

[Pruebas](PRUEBAS.log) · [Compilación](COMPILACION.log) · [Herramientas](HERRAMIENTAS.txt) · [SHA-256](SHA256SUMS.txt). Las rutas locales de compilación de los registros se han normalizado; no se alteran resultados ni diagnósticos.

Base: [fuentes 0.1.3](https://github.com/juantoniolloretegea/SV-motor/tree/4fafb8ccf158a028820168ab7f7b822c608ee6ba/laboratorio/ensayo-ia-y-observabilidad/conversacion-nativa). [Cierre de campaña Qwen/B](../../resultados/cierre-qwen-b-20260923/INFORME.md).

Sistema Vectorial SV · [Aviso de licencias](../AVISO_LICENCIAS.json).
