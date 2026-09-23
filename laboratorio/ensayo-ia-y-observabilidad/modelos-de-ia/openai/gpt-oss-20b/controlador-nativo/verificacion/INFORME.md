# Verificación local del controlador nativo 0.1.0

**23 de septiembre de 2026 · Resultado: conforme en el alcance instrumental comprobado.**

Compilador Rust 1.98.0, Cargo 1.98.0, destino `x86_64-unknown-linux-gnu`. Las versiones completas se conservan en [COMPILADOR.txt](COMPILADOR.txt) y [CARGO.txt](CARGO.txt).

## Ejecuciones

| Operación | Resultado |
| --- | --- |
| `cargo test --offline --locked -- --test-threads=1` | Código 0. Diez comprobaciones funcionales y una entrada auxiliar del banco; once resultados satisfactorios en el registro, cero fallos. |
| `cargo build --release --offline --locked` | Código 0. Ejecutable de producción compilado con desenrollado de pila ante pánico. |
| `eio-controlador-oss --licencias` | Código 0. Aviso JSON y pie de licencia emitidos; no crea un proceso de inferencia. |

Se utilizan un proceso auxiliar Rust y respuestas HTTP sintéticas. No se han ejecutado los pesos ni el motor de inferencia. Los binarios de prueba admiten estímulos mediante código condicionado a `cfg(test)`; ese mecanismo no forma parte del ejecutable de producción.

## Comprobaciones funcionales

| Comprobación | Evidencia exigida |
| --- | --- |
| Petición única y parada trazada | Respuesta recibida, una petición prevista, orden y resultado de la señal registrados; hijo recogido. |
| SIGTERM anticipado del hijo | Señal 15, memoria conservada, ninguna señal atribuida al controlador y ninguna petición. |
| Plazo y escalada | Agotamiento del plazo abreviado; TERM seguido de KILL; cierre acotado. |
| SIGTERM del controlador | Cancelación registrada y cierre del hijo. |
| Puerto ocupado | Rechazo antes de crear el hijo. |
| JSON inválido | Respuesta original conservada, cierre del hijo y ausencia de petición de inferencia. |
| Ventana y exclusión | Rechazo de duración inválida y de instalación con bloqueo activo antes del arranque. |
| Error y bloqueo del escritor | Custodia declarada no conforme; el hijo termina en ambos estímulos, dentro del plazo del banco. |
| Pánico | Limpieza del hijo al abandonar el ámbito durante el desenrollado de pila. |
| Error del envío de señal | Un envío inválido real devuelve error y queda conservado; la parada posterior recoge el hijo. |

El registro de Cargo incluye `tests::auxiliar`, que sirve para iniciar los procesos del banco. No se cuenta como una undécima comprobación funcional.

## Identidad y conservación

[SHA256SUMS.txt](SHA256SUMS.txt) identifica las fuentes, Cargo.toml, Cargo.lock, el aviso legal y el ejecutable local de producción. La huella del ejecutable identifica ese artefacto; no implica que compilaciones en entornos diferentes produzcan bytes idénticos. El ejecutable no se distribuye en este directorio.

Los registros [PRUEBAS.log](PRUEBAS.log) y [COMPILACION.log](COMPILACION.log) son copias con una única normalización: la ruta absoluta local del proyecto se sustituye por `<directorio-del-controlador>`. No contienen datos de acceso. [LICENCIAS_SALIDA.json](LICENCIAS_SALIDA.json) y [PIE_LICENCIAS.txt](PIE_LICENCIAS.txt) conservan las salidas de la consulta de licencia.

Las comprobaciones acreditan los recorridos enumerados. No identifican el emisor del SIGTERM del intento previo, no evalúan respuestas del modelo y no constituyen una verificación de la guarda exterior ni de la memoria agregada.

---

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es) · [Aviso del SV y licencias de terceros](../AVISO_LICENCIAS.json).
