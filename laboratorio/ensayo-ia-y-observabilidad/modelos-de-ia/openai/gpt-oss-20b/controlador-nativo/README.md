# Controlador nativo de gpt-oss-20b

**Versión de referencia 0.1.3 · 23 de septiembre de 2026 · Linux, Rust 1.98.0.**

Corrección del controlador instrumental utilizado en el [intento nativo del 23/09/2026](../resultados/2026-09-23/RESULTADO.md). Conserva la invocación corregida de mistral.rs 0.9.3 y adapta mecanismos de supervisión ya empleados en la conversación nativa de Qwen.

**Verificación local de 0.1.3:** 16 pruebas registradas superadas (14 funcionales y dos auxiliares), con compilación de producción completada. Su [ejecución real con el motor instalado](../resultados/continuacion-2026-09-23/INFORME.md) no alcanzó el servicio ni una respuesta de inferencia. Una traza distingue el SIGTERM exterior de la parada posterior del controlador, sin identificar el servicio emisor ni su motivo. El SIGTERM del primer intento histórico conserva su atribución pendiente.

## Cambios de 0.1.2–0.1.3 y variantes ensayadas

`--reserva-entorno-mib` admite una reserva explícita entre 512 y 4 096 MiB; el valor predeterminado permanece en 1 024 MiB. La reserva de 512 MiB utilizada en la continuación se conserva en cada registro de límites. No constituye una recomendación universal de capacidad.

Los errores de lectura identifican la ruta afectada. Si falla la atribución del puerto, se conservan el estado del proceso y la capacidad, con reconsulta durante 250 ms. El resto de controles continúa activo; no se envían solicitudes mientras no se atribuya el puerto al hijo. Durante una terminación concurrente puede registrarse un error de observación antes de que Linux permita recoger el estado final: las trazas externas y el resultado final deben cotejarse.

Las variantes experimentales 0.1.4–0.1.6, con topología selectiva y recuantización Q3K, quedan en el paquete de evidencias de la continuación. Ninguna consiguió completar la carga. La referencia 0.1.3 conserva los pesos y la configuración numérica originales; no adopta aquellas variantes como instalación operativa.

## Recursos y parada en 0.1.1

Se observa la disponibilidad del sistema y el margen del grupo cgroup visible, tomando el menor valor cuando ambas lecturas están disponibles. No equivale a reservar esa memoria ni a inventariar todos los ancestros. La admisión exige una estimación de carga de 13 123 MiB y la reserva declarada para el entorno, de 1 GiB por defecto. El primer valor procede del inventario del cargador anterior; no es una medición del consumo máximo.

La invocación exige `--limite-as-mib`: cota explícita de direcciones virtuales mediante `RLIMIT_AS`, distinta de la RAM residente y de una cuota agregada. No se fija a la RAM nominal: deben caber los mapeos y las asignaciones del motor. También se detiene ante exceso de RSS muestreada o agotamiento observado de la reserva. Las muestras se persisten durante la carga, con cadencia nominal de un segundo; si la conservación falla, se solicita el cierre.

Linux recibe `PR_SET_PDEATHSIG=SIGKILL` antes de ejecutar el motor y se comprueba la identidad del padre. La condición se refiere al hilo que creó el hijo. La prueba funcional mata realmente al padre y recoge la terminación del hijo por señal 9. No cubre descendientes separados, un motor que retire esa configuración ni un bloqueo global del anfitrión.

`--capacidad` consulta los recursos sin iniciar el motor. Estas lecturas y límites se aplican sin modificar grupos de control ni permisos administrativos.

## Correcciones conservadas de 0.1.0

| Carencia del controlador anterior | Comportamiento de esta versión |
| --- | --- |
| Caducidad absoluta incorporada al código | Ventana validada por ejecución y plazos mediante `Instant`; comprobación previa al arranque. |
| Pérdida de RSS al terminar anticipadamente | Medidas conservadas fuera de la operación y recogidas en el resultado final; `null` si no hubo muestra. |
| Señales sin trazabilidad | Registro previo de motivo, PID/PGID y señal; registro posterior del destino efectivo, retorno y error. |
| Limpieza incompleta ante errores | Cierre explícito y protección de ámbito `OwnedChild`; escalada TERM/KILL y esperas acotadas. |
| Escritura que podía bloquear el control | Escritor en otro hilo, cola limitada y acuse con plazo; un fallo de custodia conduce al cierre y se declara. |
| Puerto abierto considerado suficiente | Puerto libre antes del arranque; atribución del socket al PID y su identidad temporal; consulta HTTP de modelos antes de una única petición. |
| Identidad del controlador insuficientemente registrada | SHA-256 del ejecutable, de las fuentes principales, del archivo de dependencias y del motor en el registro inicial. |

La petición prevista conserva el caso aritmético sintético, máximo 96 tokens, temperatura 0 y esfuerzo bajo. Una respuesta HTTP válida no constituye una evaluación de calidad del contenido.

## Mecanismos adaptados de Qwen

Referencia: conversación nativa 0.1.3, corte `41c99e2a6b68382e7ba867ff55aa3a1fcc07676c`.

| Fuente | Adaptación |
| --- | --- |
| [src/main.rs](https://github.com/juantoniolloretegea/SV-motor/blob/41c99e2a6b68382e7ba867ff55aa3a1fcc07676c/laboratorio/ensayo-ia-y-observabilidad/conversacion-nativa/src/main.rs) | Propiedad del hijo, plazos monótonos y cierre con conservación de resultados. Las esperas de esta adaptación tienen plazo explícito. |
| [src/lifecycle.rs](https://github.com/juantoniolloretegea/SV-motor/blob/41c99e2a6b68382e7ba867ff55aa3a1fcc07676c/laboratorio/ensayo-ia-y-observabilidad/conversacion-nativa/src/lifecycle.rs) | Exclusión de ejecuciones simultáneas mediante bloqueo de archivo y directorios nuevos de evidencia. |
| [src/observation.rs](https://github.com/juantoniolloretegea/SV-motor/blob/41c99e2a6b68382e7ba867ff55aa3a1fcc07676c/laboratorio/ensayo-ia-y-observabilidad/conversacion-nativa/src/observation.rs) | Identidad PID + `start_ticks`, lecturas acotadas y atribución del socket por inodo. |
| [tests/service.rs](https://github.com/juantoniolloretegea/SV-motor/blob/41c99e2a6b68382e7ba867ff55aa3a1fcc07676c/laboratorio/ensayo-ia-y-observabilidad/conversacion-nativa/tests/service.rs) | Cliente HTTP local instrumental con límites de lectura y tiempo. |

Esta adaptación no incorpora la interfaz de conversación ni declara integrado el conjunto Candle, Tokio y OpenTelemetry de Qwen. Harmony permanece en el motor de inferencia.

## Compilación y uso

Desde esta carpeta, con Rust, Cargo y rustdoc 1.98.0 disponibles:

```sh
cargo test --locked -- --test-threads=1
cargo build --release --locked
./target/release/eio-controlador-oss --licencias
./target/release/eio-controlador-oss --instalacion /ruta/instalacion-local --evidencias /ruta/evidencias-nuevas --limite-as-mib COTA_VIRTUAL_MIB --ventana-segundos 1000 --carga-segundos 600 --peticion-segundos 300
```

`--offline` puede añadirse si las dependencias ya están disponibles. La verificación publicada se realizó con `--offline --locked`. Las pruebas deben ser secuenciales porque ejercitan señales del proceso.

La instalación existente debe contener `motor/mistralrs`, `modelo/` y `harmony/`. El SHA-256 admitido del ejecutable del motor es `0bd54bd0d17eb79be22b21cccaac0a3f32add5918f93ea162a888feccd7a2158`. El controlador no descarga componentes. Rechaza una carpeta de evidencias ya existente y una instalación bloqueada por otra ejecución.

El servicio escucha únicamente en `127.0.0.1:8089`. Se conservan `SUCESOS.jsonl` y `motor.log`; la salida estándar contiene el resultado final. El pie de licencia se emite por la salida de error para mantener el resultado JSON separado. `--licencias` muestra el aviso completo.

## Perímetro técnico

- Los plazos máximos configurables llegan a 1.500 segundos de operación, más el cierre. Las esperas posteriores a TERM y KILL son de 500 ms cada una; cada acuse del registro admite 250 ms. Son límites instrumentales de las esperas programadas, no una garantía de tiempo real frente a bloqueos del núcleo o del sistema de archivos.
- El máximo RSS es muestreado y corresponde al hijo de inferencia. Sus umbrales provocan una solicitud de parada; no representan una cuota agregada. RLIMIT_AS limita direcciones virtuales por proceso.
- La parada confirmada corresponde al hijo directo. Las señales se dirigen a su grupo cuando puede comprobarse; no se acredita la ausencia de descendientes que abandonen ese grupo o sobrevivan al líder.
- El escritor reside en otro hilo del mismo proceso. No equivale a un custodio independiente ni a la guarda exterior; la muerte del controlador solicita por el núcleo la terminación del hijo directo; puede quedar incompleta la evidencia final. Una pérdida del sistema queda fuera de ese mecanismo.
- El controlador no detiene la instancia anfitriona. La verificación local no cierra la vía B completa ni acredita ejecución en navegador/WebAssembly.

## Evidencias y licencias

[Verificación 0.1.3](verificacion-0.1.3/INFORME.md), [pruebas](verificacion-0.1.3/PRUEBAS.log), [compilación](verificacion-0.1.3/COMPILACION.log) y [huellas](verificacion-0.1.3/SHA256SUMS.txt). Se conservan las verificaciones históricas [0.1.1](verificacion-0.1.1/INFORME.md) y [0.1.0](verificacion/INFORME.md).

[AVISO_LICENCIAS.json](AVISO_LICENCIAS.json) identifica el SV, el modelo, el motor y Harmony. [DEPENDENCIAS.json](DEPENDENCIAS.json) recoge las 21 dependencias resueltas del controlador y sus licencias; no es un inventario del motor externo. Este paquete contiene fuentes, archivo de dependencias y evidencia de verificación; no incluye pesos ni ejecutables.

---

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es) · [Aviso del SV y licencias de terceros](AVISO_LICENCIAS.json).
