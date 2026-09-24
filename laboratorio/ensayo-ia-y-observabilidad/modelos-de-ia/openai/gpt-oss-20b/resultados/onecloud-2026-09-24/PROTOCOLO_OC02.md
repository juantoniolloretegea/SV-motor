# Protocolo previo OC-02 · Contraste del ejecutable anterior

2026-09-24 · W-S39-02 · S39 / TT-0012.

OC-01 ha producido «Hay cinco elementos en total.» con finalización stop y siete tokens. El segundo caso contrasta el paquete ejecutable anterior en el mismo anfitrión, después de confirmar cierre de OC-01.

Se conserva la petición literal, pesos GGUF, tokenizador, Harmony, BF16, contexto 1024, máximo 16 tokens, red local privada, límites del controlador y propiedades de contención de OC-01. El criterio semántico previo sigue siendo cinco elementos, sin contradicción; HTTP y contenido se juzgan por separado.

La variable deliberadamente cambiada es el ejecutable de inferencia: mistralrs 0.9.3 anterior, SHA-256 0bd54bd0d17eb79be22b21cccaac0a3f32add5918f93ea162a888feccd7a2158. El controlador de referencia deriva de las mismas fuentes 0.1.24: cambia exclusivamente el hash esperado en main.rs y la ruta de ese ejecutable en lib.rs. Se conservan fuente, diff, compilación Rust 1.98.0 y hash del binario resultante. No se retiran controles de identidad.

Presupuesto: ventana 540 s, carga 180 s, petición 300 s, RLIMIT_AS 32768 MiB, reserva 512 MiB; systemd 570 s más hasta 15 s de parada, MemoryMax 32 GiB, swap 0, TasksMax 256. Un solo intento y cierre antes de otra ejecución.

Este contraste puede discriminar los dos paquetes ejecutables en OneCloud y mostrar si la migración basta por sí sola. **No es todavía una reconstrucción controlada de dos binarios que difieran exclusivamente en el parche:** el anterior procede de otra construcción. La regresión unitaria antes/después ya identifica el defecto de difusión en su caso específico; no se fusionan ambos alcances para proclamar causalidad exclusiva de toda la inferencia.

Se conservan SUCESOS.jsonl completo, motor.log, salida del controlador, identidad y propiedades de ejecución, observación exterior y cierre. La desaparición del cgroup al terminar se registra como límite de la lectura final; sus propiedades iniciales, observaciones durante la vida del servicio y métricas conservadas por systemd se distinguen.
