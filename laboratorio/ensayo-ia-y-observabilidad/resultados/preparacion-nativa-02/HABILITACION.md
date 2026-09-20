# Habilitación pendiente de NAT02

Preparación exclusivamente. Los comandos indicados no se han ejecutado. No existe nuevo lock, vendor, compilación, binario, servicio ni imagen fijada para esta candidata.

NAT02 conserva íntegramente versiones/features de dependencias de NAT01. Sólo añade bins guarda/api-real/observador y módulos de corrección; versión del paquete 0.2.1. cgroup v2 usa interfaces del kernel mediante std/nix ya declarados, sin biblioteca nueva. curl es cliente de prueba, no dependencia del servicio; su versión efectiva se registrará en la fase futura, no se instala ahora.

## Selección de biblioteca

El inventario nativo anterior contiene Candle, tokenizers, Serde y OpenTelemetry, sin servidor HTTP suficiente. Se propone Axum, no un parser casero. Alternativas no necesarias para la frontera mínima añadirían otro stack sin antecedente; no se adopta la lista completa del artículo auxiliar. La selección queda sujeta a resolución y compilación.

| Dependencia directa | Versión/características | Justificación y contraste |
|---|---|---|
| Rust/Cargo | 1.98.0 x86_64-unknown-linux-gnu | Selección explícita futura; conservar las predeterminadas |
| axum | =0.8.9; default-features=false; http1,json,tokio | HTTP/tipos de respuesta; release oficial 14/04/2026; MSRV 1.80 |
| tokio | =1.53.1; default-features=false; rt-multi-thread,macros,net,sync,time,signal | Runtime HTTP, IPC acotado, señales del supervisor; release oficial 20/07/2026; MSRV 1.71 |
| nix | =0.31.3; default-features=false; signal,process,fs,feature | killpg, statvfs y PAGE_SIZE mediante API segura; Cargo oficial confirma versión/MSRV 1.69; docs.rs build 11/05/2026, no confundido con fecha de release |
| sha2 | =0.10.9; std | Identidad SHA-256 incremental de evidencia; compilación y auditoría transitiva pendientes |
| serde / serde_json | =1.0.228 derive / =1.0.145 | Contrato y controles heredados |
| opentelemetry / opentelemetry_sdk | =0.31.0; trace, sin defaults | Mismos API/SDK; exportación exterior añadida |
| Candle core/transformers | git ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a; sin defaults | CPU, modelo/algoritmo conservados; opcionales mediante inferencia |
| tokenizers | =0.23.1; fancy-regex, sin defaults | Tokenizador existente, opcional |

MSRV de las dependencias nuevas enumeradas no demuestra compatibilidad global con Rust 1.98.0. Resolver árbol y comprobar transitorias; no usar versiones flotantes para presentarlas como reproducibles. El Cargo.lock nativo actual se copia en antecedentes, sin trasladarlo a la raíz de la candidata como si resolviera nuevas dependencias. Ninguna huella de binario inexistente.

Fuentes primarias consultadas el 20/09/2026:
- https://github.com/tokio-rs/axum/releases/tag/axum-v0.8.9
- https://github.com/tokio-rs/axum/blob/axum-v0.8.9/axum/Cargo.toml
- https://github.com/tokio-rs/axum/blob/axum-v0.8.9/Cargo.toml
- https://github.com/tokio-rs/tokio/releases/tag/tokio-1.53.1
- https://github.com/tokio-rs/tokio/blob/tokio-1.53.1/tokio/Cargo.toml
- https://github.com/nix-rust/nix/blob/v0.31.3/Cargo.toml
- https://docs.rs/crate/nix/0.31.3
- https://docs.rs/sha2/0.10.9/sha2/

La consulta docs.rs Router no devolvió contenido utilizable; se contrastó Cargo oficial. releases/latest de nix devolvió 404; no se infiere abandono y se usa tag/Cargo y fecha de build diferenciada. No se realizó inventario general de bibliotecas ni instalación.

## Recursos: estimaciones distintas de medidas

Pesos fijados 396705472 bytes y tokenizador 11422654 bytes, identidades y procedencia en ENTRADAS_ENSAYO.json heredado. La candidata lee ambos completos y conserva el buffer de pesos durante el cálculo; no elimina automáticamente picos. 4096 posiciones KV por capa siguen vigentes. Binarios, árbol transitivo, compilación y memoria final: tamaños todavía desconocidos. No se garantiza que 8 GB basten.

Límite experimental agregado 4 GiB de RSS, no cgroup duro. Evidencia hasta 20 MiB de tres journals + entrada/manifiesto; margen runtime 2 GiB; compilación y dependencias pueden necesitar mucho más y deben presupuestarse antes. No reservar o llenar disco para comprobarlo en este encargo. Tiempo de sesión máximo propuesto 40 min, inferidor 120 s incluyendo lectura/cotejo/carga; la guarda de plataforma debe detener además servidor/supervisor si éstos mueren o no responden.

## Economía y entorno

Codespaces personal Free como candidato: documentación consultada ofrece 120 horas de núcleo y 15 GB-mes. En dos núcleos son 60 h teóricas si no hubo consumo. Saldo real, método de bloqueo y disponibilidad de máquina no comprobados. No existe promesa de coste cero basada en el nombre Free.

Antes de crear: comprobar saldo real suficiente para preparación y ensayo, recursos, almacenamiento previo y **barrera efectiva de gasto adicional cero**. Una alerta no basta. No solicitar contraseñas, copiar facturación ni cambiar plan/tarjeta/presupuestos. Si no puede demostrarse barrera efectiva, no crear el entorno.

Máximo un Codespace de dos núcleos; sin prebuild ni arranque automático; URL sólo privada autenticada. codespaces-propuesta.json es inventario inactivo, no .devcontainer. Imagen/base y digest quedan pendientes de elegir y revisar en habilitación; no usar una etiqueta mutable como identidad final.

Cerrar pestaña no detiene Codespaces; parado conserva almacenamiento facturable. Plazo propuesto: detener inmediatamente al terminar las comprobaciones y, en todo caso, a 40 min; antes recuperar evidencia incremental y final. Verificar parada en plataforma, cotejar copia receptora y eliminar instancia/almacenamiento sólo después de esa custodia y con autorización correspondiente. Si falla recuperación, no borrar a ciegas: parar y comunicar conservación, límite y consumo potencial.

Fuentes:
- https://docs.github.com/en/billing/concepts/product-billing/github-codespaces
- https://docs.github.com/en/codespaces/developing-in-a-codespace/forwarding-ports-in-your-codespace
- https://docs.github.com/en/codespaces/about-codespaces/understanding-the-codespace-lifecycle
- https://docs.github.com/en/billing/how-tos/set-up-budgets

## Secuencia que deberá autorizarse después

1. Recibir esta candidata por commit y resolver brechas críticas de MATRIZ.md. Verificar cuenta/recursos/gasto cero, imagen fija y control exterior independiente de parada/custodia.
2. Autorizar una preparación remota acotada distinta del ensayo: obtener fuentes exactas, Rust 1.98.0 aislado, dependencias; generar nuevo Cargo.lock y manifiesto transitivo/licencias; conservar el antecedente. No cambiar herramientas predeterminadas.
3. Registrar versiones, órdenes y resultados por fase. Resolver lock no es compilar; compilar no es ejecutar; ejecutar el banco no acredita modelo. Compilar --locked y correr primero siete tests de frontera, 35 complementarios y 24 controles originales.
4. Verificar hoja cgroup v2 vacía/delegada y mecanismo superior de plataforma sin modificar permisos por suposición; si faltan, no lanzar bloqueos. Construir sin feature inferencia y probar testigos heredados y R1/R2/R3 en directorios nuevos, un intento activo; demostrar cierre, persistencia, recuperación y autenticación. Si falla, detener; corrección y nueva autorización antes de repetir campaña, nunca repetir modelo por defecto.
5. Publicar corte compilado/pruebas/lock/hashes/brechas resueltas y solicitar habilitación específica de la primera inferencia nativa. Sólo después compilar --features inferencia y cotejar pesos/tokenizador fijados; no cambiar Candle, contexto ni cuantización.
6. La prueba real tendrá entrada/ID/tiempo/contador precomprometidos; registrar cualquier rechazo del modelo sin reparación. Cerrar procesos y plataforma, recuperar desde otra sesión, cotejar y entregar para revisión. Sin continuidad automática, S37/S38 pendientes.

Las guardas anteriores de Actions permanecen cerradas y no se reutilizan como permiso de esta candidata. No se ha abierto Codespaces, puerto, servicio, workflow ni sesión de inferencia.

## Condiciones adicionales NAT02

Arrancar siempre mediante guarda, nunca supervisor directo. La guarda preparada no demuestra disponibilidad de cgroup.kill/delegación en Codespaces. No seleccionar otro alojamiento ni conceder privilegios como solución implícita. Debe recibirse el mecanismo y acreditarse contención efectiva/gasto cero antes de una habilitación remota.

Las nuevas pruebas atraviesan HTTP real y recuperan conjuntos por sello. No activar feature inferencia ni descargar pesos para R1/R2/R3. Mantener Qwen3-0.6B; ninguna interfaz alternativa de Qwen forma parte de esta tarea. El siguiente encargo, si se autoriza, fijará presupuesto de campañas sintéticas y cierre de la plataforma. Esta entrega termina tras publicación/cotejo.
