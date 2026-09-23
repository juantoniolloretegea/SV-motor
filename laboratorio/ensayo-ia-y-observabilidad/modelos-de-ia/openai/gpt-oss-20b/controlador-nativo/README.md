# Controlador nativo de gpt-oss-20b

**Versión 0.1.0 · 23 de septiembre de 2026 · Linux, Rust 1.98.0.**

Corrección del controlador instrumental utilizado en el [intento nativo del 23/09/2026](../resultados/2026-09-23/RESULTADO.md). Conserva la invocación corregida de mistral.rs 0.9.3 y adapta mecanismos de supervisión ya empleados en la conversación nativa de Qwen.

**Verificación:** diez comprobaciones funcionales locales superadas y compilación de producción completada. Las pruebas utilizan auxiliares Rust y un servicio HTTP sintético. Queda pendiente ejecutar este controlador con el motor y los pesos instalados; no se ha realizado otra inferencia ni identificado el emisor del SIGTERM original.

## Correcciones

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
./target/release/eio-controlador-oss --instalacion /ruta/instalacion-local --evidencias /ruta/evidencias-nuevas --ventana-segundos 1000 --carga-segundos 600 --peticion-segundos 300
```

`--offline` puede añadirse si las dependencias ya están disponibles. La verificación publicada se realizó con `--offline --locked`. Las pruebas deben ser secuenciales porque ejercitan señales del proceso.

La instalación existente debe contener `motor/mistralrs`, `modelo/` y `harmony/`. El SHA-256 admitido del ejecutable del motor es `0bd54bd0d17eb79be22b21cccaac0a3f32add5918f93ea162a888feccd7a2158`. El controlador no descarga componentes. Rechaza una carpeta de evidencias ya existente y una instalación bloqueada por otra ejecución.

El servicio escucha únicamente en `127.0.0.1:8089`. Se conservan `SUCESOS.jsonl` y `motor.log`; la salida estándar contiene el resultado final. El pie de licencia se emite por la salida de error para mantener el resultado JSON separado. `--licencias` muestra el aviso completo.

## Perímetro técnico

- Los plazos máximos configurables llegan a 1.500 segundos de operación, más el cierre. Las esperas posteriores a TERM y KILL son de 500 ms cada una; cada acuse del registro admite 250 ms. Son límites instrumentales de las esperas programadas, no una garantía de tiempo real frente a bloqueos del núcleo o del sistema de archivos.
- El máximo RSS es muestreado y corresponde al hijo de inferencia. No representa memoria agregada ni impone una cuota de memoria.
- La parada confirmada corresponde al hijo directo. Las señales se dirigen a su grupo cuando puede comprobarse; no se acredita la ausencia de descendientes que abandonen ese grupo o sobrevivan al líder.
- El escritor reside en otro hilo del mismo proceso. No equivale a un custodio independiente ni a la guarda exterior; SIGKILL del controlador o pérdida del sistema pueden impedir la limpieza.
- El controlador no detiene la instancia anfitriona. La verificación local no cierra la vía B completa ni acredita ejecución en navegador/WebAssembly.

## Evidencias y licencias

[Informe de verificación](verificacion/INFORME.md), [pruebas](verificacion/PRUEBAS.log), [compilación](verificacion/COMPILACION.log) y [huellas](verificacion/SHA256SUMS.txt).

[AVISO_LICENCIAS.json](AVISO_LICENCIAS.json) identifica el SV, el modelo, el motor y Harmony. [DEPENDENCIAS.json](DEPENDENCIAS.json) recoge las 21 dependencias resueltas del controlador y sus licencias; no es un inventario del motor externo. Este paquete contiene fuentes, archivo de dependencias y evidencia de verificación; no incluye pesos ni ejecutables.

---

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es) · [Aviso del SV y licencias de terceros](AVISO_LICENCIAS.json).
