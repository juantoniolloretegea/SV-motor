# EIO-05 · resultado y cierre de ejecución única

Fecha: 20/09/2026, Europe/Madrid. Holmes / UE-LOCAL-CODEX-WINDOWS. Entrega para revisión receptora; no aceptación científica.

La ejecución [35476464030](https://github.com/juantoniolloretegea/SV-motor/actions/runs/35476464030), número **5**, intento **1**, terminó **failure**, retorno **101** en la comprobación WASM. Se ejecutaron antes el aprovisionamiento, la compilación nativa, los 24 controles directos, las siete inferencias y el cálculo de costes. No se corrigió ni se reintentó. La cuenta histórica conserva campaña3/3, diagnóstico extraordinario1/1 y continuación05 1/1; no autoriza número6.

## Autorización, corte y transporte

Autorización humana expresa de la propuesta privada d9ddd06a3ee0497db8dc2a139af8600682d2691c y propuesta pública dd2583d4c1738151ed582e2db599e1e8628a3b7a. [Precompromiso](https://github.com/juantoniolloretegea/SV-motor/blob/65596d83fee823bf0458a92d64106cc7c2b08b2b/laboratorio/ensayo-ia-y-observabilidad/resultados/continuacion-05/PRECOMPROMISO.md) publicado y releído antes del único dispatch. Corte ejecutado **65596d83fee823bf0458a92d64106cc7c2b08b2b**.

Se aplicó literalmente el parche publicado, cotejando todos sus contextos. Blobs efectivos: adquirir.sh ad43a76c51e303bf4b9265e5105f8beb3b8423bb; comprobar-transporte.sh 82f193de613154e84f0e33065f6fa38d818907ad; workflow cba675685582419249f6454aa61b258eeb17c7e2. Único host añadido: us.aws.cdn.hf.co bajo hf. Las 32 comprobaciones sintéticas pertenecen a la preparación receptora; no se repitieron aquí.

Ahora se observó para GGUF y tokenizador: huggingface.co HTTP302, us.aws.cdn.hf.co HTTP200, seguido de concordancia de bytes y SHA-256. Seis archivos conformes en IDENTIDADES.tsv. Cargo.lock: OK. Versiones literales:
```
rustc 1.98.0 (88d9e12ae 2026-08-18)
cargo 1.98.0 (797e8a9bc 2026-08-05)
```
No se usaron credenciales HF. No se confunde el nombre del CDN con la localización física. Configuración de plantilla mantenida como referencia documental, no descargada ni ejecutada como Jinja.

## Resultados observados

- Compilación nativa release --locked --offline, dos trabajos: retorno0.
- Banco: 24/24 controles directos conformes, retorno0. Son controles del adaptador y función de parada, no ensayos científicos del SV.
- Inferencias: siete retornos0; todas generaron los mismos74 tokens y terminaron EOS, entrada120 tokens. Todas se clasificaron **ESTRUCTURA**. Se conserva literalmente el texto generado, incluido el cercado Markdown, sin extraer/reparar JSON para mejorar el resultado.
- Cuatro inferencias on con registros OpenTelemetry; tres off sin esa cobertura. Tokens equivalentes no acreditan semántica conforme.
- Costes: calentamiento separado; tres pares on/off, off/on, on/off. Diferencias on menos off: 0.028757819s, 0.087914788s y -0.084300398s; mediana0.028757819s, rango0.172215186s. Intervalo incluye creación/cierre/exportación de telemetría, lectura/carga y generación; excluye arranque del proceso y serialización de evidencia. No se atribuye causalidad general a tres pares ni se presenta como mejora de rendimiento.
- Check WASM: retorno101. Error literal:
```
error: The wasm32-unknown-unknown targets are not supported by default; you may need to enable the "wasm_js" configuration flag. Note that enabling the `wasm_js` feature flag alone is insufficient. For more information see: https://docs.rs/getrandom/0.3.4/#webassembly-support
error: could not compile `getrandom` (lib) due to 1 previous error
```
No se alteraron dependencias/configuración para resolverlo. No hubo ejecución real en navegador.

## Recursos, tiempos y cierre

Runner declara Ubuntu24.04.5, imagen ubuntu-24.04/20260907.300.1, X64. Creación/inicio run19/09/2026 23:33:54UTC; actualización final completed19/09 23:38:46UTC (20/09 01:38:46Madrid). Diferencia292s; UI indicó job4m47s. Esas marcas proceden de GitHub, no del mensaje humano. Límite40min y plazo interno2280s conservados.

265 muestras a intervalo nominal1s. Máximo disco contabilizado2740518912bytes; mínimo libre89677512704bytes; máximo evidencia147456bytes; máximo RSS compilación1982584KiB; inferencia1610664KiB. Son medidas del runner, no del PC. No hay muestra de banco/costes por su brevedad: pico0 no equivale a consumo nulo. Muestreo no acota picos entre muestras ni constituye vigilancia independiente.

Doce fases informan residuales_al_wait=0 y residuales_tras_limpieza=0. Causa normal en WASM significa fin natural con error101, no éxito. Finalización y limpieza del job observadas; no se afirma un inventario exhaustivo del host remoto. Ambas guardas se cierran en este commit de resultados; workflow sigue restringido a número5/intento1. No hay operación experimental propia en curso tras completed.

## Custodia y límites

JOB_105986448506.txt conserva íntegra la cadena de texto obtenida del conector de logs de GitHub, incluidas marcas horarias y secuencias ANSI; no se atribuye identidad de bytes al ZIP original de Actions, no recuperado. RUN.json es selección explícita de metadatos más jobs. Los demás TSV/JSON son derivados de ese log: se quitaron prefijos horarios para analizar y se serializaron objetos; el log conserva la salida literal. No se publican cabeceras ni URL firmadas del transporte.

Banco, inferencias y costes remiten a su stdout; telemetría on está en stderr del log. FASES.tsv conserva retornos; MEDIDAS.tsv conserva265 muestras; MATRIZ.tsv distingue cobertura y límites de las15 obligaciones. Tokens equivalentes, campos copiados de permiso y cancelación de función no sustituyen observaciones independientes ni cancelación de inferencia en curso.

Coordinación mediante conector GitHub y navegador; transformación documental en memoria. No instalación, pesos, compilación, inferencia ni archivos del ensayo en el PC; no shell local, contenedores locales, WSL, VSCode, cámara, micrófono o capturas del usuario. No se accedió a almacenes de credenciales. No se tocaron canónicos, Profesor, S32/BIS-03, ni se amplió telemetría. Sin reintentos, nueva ejecución, caché de proyecto o upload-artifact. Las operaciones automáticas del navegador/plataforma y la infraestructura subyacente no fueron inventariadas.

El resultado no acredita privacidad, cualificación, H1, paridad entre plataformas ni aceptación científica. Pendiente revisión receptora.
