# Conversación experimental con GPT-OSS-20B

Interfaz derivada de `conversacion-nativa` 0.1.4, recibida en SV-motor `d4e62b29713a2044be0d1d4a7fb463155d3999c3`. Se conservan sus expedientes, conversaciones, vista previa del contexto, registro encadenado, exportación y controles de reenvío. La integración del modelo se sustituye por un cliente Rust de un motor residente local. Los resultados de Qwen no se transfieren a GPT-OSS.

[Acceso y recuperación](RECUPERACION.md) · [Protocolo previo](PROTOCOLO.md)

## Ejecución

Rust/Cargo 1.98.0; `Cargo.lock` fija las dependencias. `cargo test --bin eio-conversacion -- --test-threads=1`, `cargo build --release` y `cargo check` constituyen las comprobaciones de compilación. El binario contiene la interfaz; JavaScript se limita al transporte y la presentación en el navegador. No se utiliza Python.

El modelo reside en una unidad systemd separada, accesible únicamente mediante la dirección local. El servicio prepara Harmony y abre explícitamente el canal final. Temperatura cero; una generación activa; reserva predeterminada de 256 tokens; plazo predeterminado de 600 segundos. El usuario puede reservar hasta 1024 tokens y fijar hasta 900 segundos. Ventana operativa: 4096 tokens, incluida la respuesta. El recuento del tokenizador de la interfaz se contrasta con el uso declarado por el motor. Una discrepancia bloquea nuevas operaciones hasta intervenir y se conserva como incidencia de integración.

El transporte devuelve respuestas completas. No hay transmisión incremental ni medida del tiempo hasta el primer token. La latencia incluye la preparación del contexto y, si corresponde, la carga del motor. La memoria se informa por ámbitos: cliente Rust muestreado y cgroup del motor; el máximo del cgroup es acumulado durante la sesión residente.

## Control y conservación

Las unidades establecen 32 GiB para el motor y 2 GiB para el servicio, sin intercambio. El motor se detiene cuando se detiene el servicio. Una cancelación o terminación anómala del cliente solicita la parada del motor y comprueba `MainPID=0`. La admisión permanece bloqueada si el cierre no se confirma. La escucha se restringe a `127.0.0.1`; se declara filtrado de red de las unidades para permitir solo direcciones locales. La comprobación de configuración no equivale a auditoría exhaustiva del tráfico.

La identidad personal depende de la protección del puerto privado de GitHub. El token de sesión interno evita peticiones ajenas a la página, pero no constituye una autenticación profesional independiente. El modelo no recibe herramientas de ejecución ni acceso a los expedientes fuera del contexto preparado.

Se conservan el historial completo, las respuestas incompletas y la causa de interrupción. Los contextos excesivos se rechazan, sin recorte ni resumen automático. Los expedientes tienen una cota de 475 MiB y existen reservas y cotas auxiliares dentro del presupuesto lógico de 512 MiB. Una cadena SHA-256 local detecta determinadas alteraciones; no es firma externa ni prueba de veracidad.

## Verificación y banco

`--validate DIRECTORIO_NUEVO` comprueba contra el servicio real la sesión inválida, el contexto excesivo, los delimitadores reservados, la idempotencia, la cancelación y la exportación. Conserva un expediente sintético independiente.

`--compare DIRECTORIO_NUEVO` ejecuta el banco fijado en `src/comparison.rs` mediante la misma API de la interfaz. No modifica expedientes del usuario. Guarda resultados originales y exportación antes de emitir el resumen. Doce tareas breves, cuatro turnos reales y cuatro tamaños de contexto; el límite temporal y cualquier interrupción quedan explícitos. Las valoraciones de contenido se redactan separadamente de los registros originales.

La finalización técnica no acredita calidad general. El informe de resultados debe distinguir banco sintético, conversación real breve y recuperación de un dato con distractores extensos. Ninguno equivale por sí solo a una validación profesional o a una evaluación prolongada de uso real.

## Reanudación por otra unidad de trabajo

Máquina de cálculo: `/opt/sv-lab/conversacion-20260924`; fuentes `src`, binario `bin/eio-conversacion`, expedientes `datos`, resultados del banco `evidencias`. La instalación optimizada anterior se conserva en `/opt/sv-lab/optimizacion-20260924`.

Codespace: `/workspaces/conversacion-20260924`; fuentes y compilación, registro de acceso y `ACCESO.sh`. El repositorio original del usuario contiene cambios previos y no debe restablecerse. La copia aislada usada para publicar en `main` es `/workspaces/optimizacion-20260924/custodia-main`.

No reinicie el banco sobre un directorio existente ni repita automáticamente una petición dudosa. Consulte su identificador, las unidades systemd y los registros. Una pérdida del navegador o del túnel se registra separadamente del modelo. No se adquieren recursos adicionales ni se crean ramas de trabajo.
