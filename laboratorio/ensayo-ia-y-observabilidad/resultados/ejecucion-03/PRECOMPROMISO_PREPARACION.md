# EIO v3 · precompromiso de preparación

Autorización humana recibida para EIO-GITHUB-01/v3 del corte bbb7ffa476da7a030a1fa0e31bc46dc76179def4. Inicio local observado 2026-09-18T13:55:57.4447107+02:00 (reloj del host, no sello del mensaje). Se conserva candidato 6f8b86d06ad46a5d98d31cefe6076b130280ffab y revisión receptora c7a7fbb6240d59bc41e7a2154c080353570aadde. Este corte habilita **una preparación remota, sin compilar, ejecutar casos, descargar pesos ni inferir**. Ensayo=false.

## Aprovisionamiento verificable

Se sustituye el instalador desconocido del PATH por tres paquetes oficiales de Rust 1.98.0: rustc, cargo y rust-std para x86_64-unknown-linux-gnu. Patrón exacto https://static.rust-lang.org/dist/{componente}-1.98.0-x86_64-unknown-linux-gnu.tar.xz y su .sha256. Se obtiene la suma oficial por HTTPS, se comprueban sus bytes antes de extraer o ejecutar install.sh, incluido en el paquete cotejado. No se acepta como procedencia un hash aislado del instalador preexistente. Suma y paquete comparten autoridad/canal; no se afirma independencia criptográfica de fuentes ni validación GPG. Tamaños y sumas son pendientes hasta obtención, sin inventarlos.

Instalación exclusivamente en RUNNER_TEMP/eio/toolchain-1.98.0 (debe estar ausente al empezar), sin sudo/ldconfig, sin rustup ni cambios persistentes de PATH. Se verifica rustc -Vv y coincidencia exacta 1.98.0 antes de resolver dependencias; invocación absoluta de cargo, RUSTC y RUSTDOC del mismo prefijo. CARGO_HOME y TMPDIR aislados. Rust documenta los paquetes independientes en https://forge.rust-lang.org/infra/other-installation-methods.html. Los tres paquetes necesarios evitan el paquete completo con documentación/componentes no requeridos.

Cargo genera el lock y metadata --locked; no build, test, run ni inferencia. Versiones directas y Candle permanecen fijadas en Cargo.toml del candidato. Se registra metadata de transitivas/características/licencias, tamaños y SHA-256 de paquetes .crate descargados. Rust conserva sus avisos incluidos; licencias de transitivas declaradas por metadata, sin auditoría jurídica. Ningún paquete se adquiere en el PC.

## Ejecutor y evidencia

SV-motor confirmado público por API. Ubuntu-24.04 estándar, contents:read, sin secretos/credenciales persistidas, servicios, cachés ni upload-artifact. GitHub documenta gratuidad de ejecutores estándar para repositorios públicos y que logs/resúmenes no consumen cuota de artefactos: https://docs.github.com/en/billing/concepts/product-billing/github-actions. No se configura facturación ni almacenamiento de pago.

UI del flujo antes de publicar: 0 workflow runs. La consulta del contador por ruta de conector fue rechazada como endpoint no admitido (INVALID_ARGUMENT); no se repitió ni se infiere de ello denegación de lanzamiento. La UI ya había mostrado cero. Un único lanzamiento preparación; tres ejecuciones acumuladas máximo, sin reintentos. Corte solicitado=GITHUB_SHA=checkout HEAD, run_attempt=1.

Plazo global: 2280 s desde el primer paso de presupuesto, dejando nominalmente 120 s al límite de 40 min del job. Supervisor rehúsa iniciar fases cuyo máximo+5 s exceda el remanente y comprueba el plazo durante muestreo. Preparación limitada a 1800 s. No se amplía ninguna cota. RSS/disco/evidencia, cobertura de grupo de procesos y posibilidad de sobrepaso entre muestras siguen declaradas. Disco 10 GiB y reserva 2 GiB; evidencia 20 MiB, transmisión limitada. stdout/stderr/retornos separados en registros. Sin lectura de variables de autenticación.

Cargo.lock, DEPENDENCIAS.json y COMPONENTES.tsv se exportan íntegros como bloques base64 de hasta 512 KiB cada uno, con tamaño/SHA-256. El supervisor impide transmitir ficheros superiores a su cota; si el log se trunca no se avanza ni se reconstruye. La evidencia se recuperará mediante conector GitHub y se custodiará en resultados antes de recepción. Fallo HTTP, integridad, acceso o incompatibilidad detiene el paso sin cambiar canal, modelo o compilador.

## Correcciones de recepción

- P01: tiempo nuevo del intervalo completo, desde antes de inicializar telemetría hasta después del cierre/exportación; tiempo interno separado. Se conservan tres pares y tokens comparables. La primera llamada sólo puede calentar cachés del SO, no el estado de procesos posteriores.
- P02: la cadena adversarial se entrega como contenido de FuenteConsultada a ejecutar_con_fuentes; su tamaño se trata dentro de la frontera, las referencias se extraen allí, Condiciones sigue siendo autoridad exterior inmutable. Se conserva SIN_PERMISO y ausencia de efecto; evidencia incluye fuente recibida/bytes/permiso antes-después. Control directo, no prueba del modelo.
- P04: plazo global y negativa a iniciar fases sin margen. Se conserva lo pendiente sin repetición automática.
- P03: este corte resuelve el aprovisionamiento **preparatorio**. Antes de ensayo faltan adquisición verificada/repetida de pesos/tokenizador/destino WASM, lock recuperado y nuevo corte experimental. ensayar.sh permanece cerrado y no se lanza. Navegador real pendiente; no se acredita EIO-P-12.

Esperado: preparación con retorno 0, Rust exacto, lock/metadatos/identidades íntegros. Un resultado adverso se conserva. No aceptación del banco ni estado científico, no cambio a S32/BIS-03. Las correcciones de Rust siguen sin compilar.
