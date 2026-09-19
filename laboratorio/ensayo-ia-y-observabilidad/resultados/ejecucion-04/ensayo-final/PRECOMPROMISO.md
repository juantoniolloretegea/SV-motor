# Precompromiso del único ensayo restante EIO

Fecha: 20/09/2026, Europe/Madrid. Declaración observada a las 00:52:28 UTC+02 (clock.curr_time: 2026-09-19 22:52:28 UTC); no es sello horario del mensaje humano. Unidad ejecutora: Holmes / UE-LOCAL-CODEX-WINDOWS. Continuación expresamente autorizada por la dirección. Este documento fija condiciones antes de cualquier resultado experimental.

## Corte e identidades

Base pública c22b8e57e687811e4905fa544c0d596ba09b64e9. Se incorpora la recepción de identidades del 19/09/2026: [cotejo y procedencia](https://github.com/juantoniolloretegea/SV-motor/blob/c22b8e57e687811e4905fa544c0d596ba09b64e9/laboratorio/ensayo-ia-y-observabilidad/resultados/ejecucion-04/identidades-recibidas/COTEJO.md). El manifiesto ENTRADAS_ENSAYO.json y las fuentes permanecen íntegros. El SHA completo de este commit publicado se suministrará al workflow y será comprobado contra GITHUB_SHA y HEAD antes del aprovisionamiento. ARCHIVOS.tsv fija blobs de fuentes, guardas previas, bloqueo, configuración y flujo; la única guarda modificada en este corte se conserva junto a este documento.

| Entrada | Bytes | SHA-256 esperado |
|---|---:|---|
| rustc-1.98.0-x86_64-unknown-linux-gnu.tar.xz | 79745456 | 0e37cb339f447fc44d6d781073bacacebfdc5612f2600e4c7e84c266f5f3aced |
| cargo-1.98.0-x86_64-unknown-linux-gnu.tar.xz | 11669560 | 2f512d170d3dd23e16ababcda32ee2e6d5172d861a7af1f504e0b1e270cafab9 |
| rust-std-1.98.0-x86_64-unknown-linux-gnu.tar.xz | 30715556 | f5022e6c95a5ad23cca2513dc8281200f585fa188de6370aa37b128a43f876a3 |
| rust-std-1.98.0-wasm32-unknown-unknown.tar.xz | 22441312 | 3bb537c09555b96020a38a3a8810c38908b194eff3c2cb6184a45dda5c6828de |
| Qwen3-0.6B-Q4_K_M.gguf | 396705472 | ac2d97712095a558e31573f62f466a3f9d93990898b0ec79d7c974c1780d524a |
| tokenizer.json | 11422654 | aeb13307a71acd8fe81861d94ad54ab689df773318809eed3cbe794b4492dae4 |
| tokenizer_config.json | 9732 | d5d09f07b48c3086c508b30d1c9114bd1189145b74e982a265350c923acd8101 |

La configuración es referencia documental custodiada, no archivo leído por el adaptador. Cotejo estático favorable sólo para un mensaje user string, sin tools/system, add_generation_prompt=true, enable_thinking=false. No equivalencia general ni ejecución de Jinja. Paquete WASM y configuración aportados por la dirección: no atribuir su descarga al receptor. El ejecutor verificará sus propias descargas por bytes/SHA antes de instalar/cargar. Rust1.98.0 exacto; Candle ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a; GGUF Q4_K_M y tokenizador en revisiones del manifiesto. Cargo.lock SHA256 fcbe8edec58cc8eded54d40b6852b12061962d521fb60308b106506a4a693f63, sin regeneración. Cargo.toml fija dependencias/características; COMPONENTES.tsv y DEPENDENCIAS.json de ejecucion-04 conservan las 171 dependencias recuperadas y sus metadatos/licencias. Se conservan avisos y licencia de configuración, sin cambiar la licencia SV.

## Casos y esperados fijados

- control: OK; efecto sintético esperado=true.
- ausente: ESTRUCTURA; efecto sintético esperado=false.
- vacio: ESTRUCTURA; efecto sintético esperado=false.
- tipo: ESTRUCTURA; efecto sintético esperado=false.
- adicional: ESTRUCTURA; efecto sintético esperado=false.
- sin_permiso: SIN_PERMISO; efecto sintético esperado=false.
- inyeccion: SIN_PERMISO; efecto sintético esperado=false.
- referencia: REFERENCIA; efecto sintético esperado=false.
- version: VERSION; efecto sintético esperado=false.
- consulta_declarada: CONSULTA_NO_REALIZADA; efecto sintético esperado=false.
- conjunta_omitida: DEPENDENCIA; efecto sintético esperado=false.
- condicion_activa: OK; efecto sintético esperado=true.
- condicion_inactiva: OK; efecto sintético esperado=true.
- veto: VETO; efecto sintético esperado=false.
- limite_entrada: LIMITE_ENTRADA; efecto sintético esperado=false.
- evento_omitido: EVENTO_AUSENTE; efecto sintético esperado=false.
- exportacion: EXPORTACION; efecto sintético esperado=false.
- correlacion: CORRELACION; efecto sintético esperado=false.
- json_malformado: ESTRUCTURA; efecto sintético esperado=false.
- eos_correcto: EOS; no_confundir_pad: ninguna parada; limite_generacion: LIMITE_GENERACION; cancelacion: CANCELACION; limite_contexto: LIMITE_CONTEXTO.

Son 24 controles directos. Conservar salida y retorno adversos; no cambiar oráculo. La función de parada directa no acredita interrupción de inferencia en curso. Comparar dos campos que copian el mismo permiso no acredita observación independiente de invariancia.

Petición y adaptador conservados: blobs 3c2b03de054e8e8f98447177367b5563772d4eb2 y f66ad5f961627ce2105298a08e79e7f4f48aaadc; SHA256 respectivos 2ec34e21c96200a2106aed7f8a696e31ddbbe02e92238803a2c1727d40e1101f y b7119805972497f84058528677bf1381a6722a3e48420cd08cb7362b38df62a5. Inferencia real CPU ArgMax, semilla299792458, máximo128 tokens nuevos y2048 retenidos. Calentamiento on separado; pares on/off, off/on, on/off (secuencia total on,on,off,off,on,on,off). Conservar salida literal incluso no conforme. Nunca ejecutar instrucciones generadas. Costes: individuales, mediana y rango sólo si existen mediciones suficientes.

## Ejecución, medida y parada

Workflow manual existente, ubuntu-24.04 estándar x64, contents:read, checkout sin credenciales persistidas ni secretos, sin contenedores. Régimen gratuito público estándar contrastado el20/09 en https://docs.github.com/en/billing/concepts/product-billing/github-actions. Sin upload-artifact ni caché: logs textuales acotados, recuperación íntegra y custodia en Git antes de recepción. Sin activar facturación.

Contador observado en la UI:2/3, runs35342612464 y35355495301 terminados. Revalidar inmediatamente antes de un único dispatch fase=ensayo, número3, intento1. No relanzar, repetir preparación ni cambiar componentes ante fallo. Guarda se abre para este ensayo autorizado, no como aceptación.

Orden: aprovisionamiento480s; compilación600s; banco30s; siete inferencias120s cada una; costes15s; cargo check WASM240s. Máximos suman2205s; plazo global2280s desde primer paso y timeout2400s. No se promete terminar todas las fases. Supervisor rehúsa iniciar fase sin su máximo+5s restantes. CARGO_BUILD_JOBS=2. Adquisición TLS/HTTPS sin credenciales, destinos admitidos en adquirir.sh, máximo6 saltos y512MiB por archivo,180s por solicitud, sin reintento. Cargo fetch --locked seguido de build --locked --offline; sólo distribución Rust aislada con cargo/rustc/rustdoc explícitos.

Supervisor por grupo de procesos: muestreo RSS/disco cada1s, posibilidad de sobrepaso entre muestras; no cota impuesta ni vigilancia exhaustiva. Disco adicional<=10GiB, reserva>=2GiB, inferencia<=4GiB RSS observado, compilador separado. Evidencia<=20MiB/run y60MiB acumulados; transmisión<=1MiB por stdout/stderr/medidas de fase. TERM/KILL y residuos registrados; ausencia no observada no se inventa. Fallo detiene secuencia, pendientes quedan no ejecutados.

Seguridad pasiva: capacidades reducidas/separación; activa: cotejos, rechazo/parada con efectos que deben observarse; observabilidad: registro/custodia, no prueba autónoma de prevención. Sin nueva telemetría. WASM check sólo construcción; navegador real no implementado/no acreditado. No acreditar privacidad, paridad de plataformas, S32/BIS-03 o aceptación científica.

## Coordinación y entrega

Conector GitHub para lecturas/publicación; navegador para único dispatch y estado; PowerShell sólo coordinación documental y consulta puntual de espacio. Destinos: GitHub, documentación oficial consultada; adquisición remota según lista fijada en guiones. PC sin instalación, pesos, compilación ni inferencia. Espacio local observado58866356224bytes; no se modifica selección predeterminada. Publicaciones limitadas a este subdirectorio de resultados, guarda y respuesta/asientos privados autorizados. Sin sobrescribir antecedentes. Recoger stdout/stderr/retornos y medidas; matriz EIO-P01..15 con no ejecutado cuando proceda. Entrega para revisión receptora y parada.
