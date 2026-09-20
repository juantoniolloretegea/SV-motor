# EIO-06 · resultado y cierre

20/09/2026. Holmes / UE-LOCAL-CODEX-WINDOWS. Entrega para revisión receptora.

**Resultado global: failure, retorno2 contractual; comprobación WASM retorno0.** La referencia reproduce el resultado anterior; la única variante estructurada no satisface el contrato. No se corrigió ni repitió.

## Corte y autorización

Propuesta privada0d20e7a270564632df294af9f6f380d49956d87b; paquete públicoef656846092afaf23c1feb5e27879649776e6d6a. Autorización humana expresa de una ejecución adicional6/intento1, máximo40min, incluyendo preparación offline del lock con inventario idéntico. [Precompromiso](https://github.com/juantoniolloretegea/SV-motor/blob/01d9f7c87e0ee38775f079c6841a09b93b57bf6c/laboratorio/ensayo-ia-y-observabilidad/resultados/continuacion-06/PRECOMPROMISO.md) publicado y releído antes del único dispatch.

Commit ejecutado **01d9f7c87e0ee38775f079c6841a09b93b57bf6c**; [run35489025503](https://github.com/juantoniolloretegea/SV-motor/actions/runs/35489025503), número6 intento1; job106020582816. Cinco ejecuciones anteriores terminadas antes del lanzamiento. Cuenta histórica: campaña3/3, diagnóstico1/1, EIO-05 1/1 y EIO-06 1/1. No autoriza número7 ni reintento.

Parche de peticiones aplicado literalmente cotejando contexto. Fuente resultante blob65d552662baee14d81cbb20c5768daf218f37423; workflow exacto de propuesta blobb2047f79c0d2636b185776df26c5d813e16ff755. Sólo cuatro rutas alteradas en precompromiso: fuente, workflow existente, guardas, declaración. Cargo.toml y Cargo.lock activos del repositorio no se sustituyeron.

## Preparación y recuperación

Aprovisionamiento0; seis descargas coincidentes en bytes/SHA256 (IDENTIDADES_ENTRADAS.tsv). Versiones literales:
```
rustc 1.98.0 (88d9e12ae 2026-08-18)
cargo 1.98.0 (797e8a9bc 2026-08-05)
```
Cargo.lock original verificado antes de aprovisionar. Preparación dependencias-06 retorno0: metadatos offline, cambio de manifiesto sólo en checkout remoto, inventario conservado y fijación antes de compilar. Sin cargo update/generate-lockfile ni resolución nueva en red.

Se recuperaron los cinco bloques base64 completos emitidos por el runner, decodificados en memoria; bytes y SHA256 recalculados concordantes. Detalle en COTEJO_RECUPERACION.tsv:
- Cargo.lock: 41530 bytes; SHA256 1a3015a1ea23fbed270d2771e445e2e19e2b1e1d79efe2059a78c42a367e06e9.
- Cargo.toml: 1779 bytes; SHA256 1017c326ff92201a34716aae1bbd4d40643d952e4b1081bfc6e3690739ce607e.
- nueva-identidades.json: 22433 bytes; SHA256 8a4ea4de41204c07e7aa8a9e63f5056c676370ee126854bcaec22739f01e0062.
- arbol-wasm.txt: 73459 bytes; SHA256 683b0de28532d440b144409ead56b4e22a0fce8d97731156a1cbf0e39098af3c.
- FIJACION.sha256: 569 bytes; SHA256 5b5758692818a2317a378e09c1e0adce2caeb740ab0d8a20f12f3c610c9d0637.

Lock derivado custodiado en dependencias/Cargo.lock; original41482bytes/SHA256fcbe8edec58cc8eded54d40b6852b12061962d521fb60308b106506a4a693f63 permanece en raíz del ensayo. Se cotejó además el inventario del lock original y derivado contra nueva-identidades.json:171 paquetes en cada uno, mismos nombres/versiones/orígenes. Las relaciones/características sí cambian según propuesta. Manifiesto recuperado idéntico al candidato publicado. Las seis huellas de FIJACION.sha256 se cotejaron contra archivos recuperados/fuentes del commit; el runner también informa OK antes de compilar y al final. COTEJO_FIJACION.json conserva este contraste. SHA256 documental calculado en memoria, comprobado con vectores vacío/abc y huella conocida del lock original; no compilación local.

## Compilación y ejecución observadas

| Fase | Retorno | Alcance |
|---|---:|---|
| Aprovisionamiento | 0 | Seis entradas verificadas |
| Preparación de dependencias | 0 | Inventario conservado, lock/manifiesto fijados |
| Compilación nativa release offline | 0 | Dos trabajos, --locked --offline |
| Banco | 0 |24/24 controles directos conformes |
| Inferencia referencia | 0 |120 tokens entrada,74 generados, EOS; contrato ESTRUCTURA |
| Inferencia estructurada | 0 |212 tokens entrada,59 generados, EOS; contrato ESTRUCTURA |
| Recepción Rust | 2 |referencia_reproducida=true; conforme=false |
| Comprobación WASM | 0 |cargo check --locked --offline --target wasm32-unknown-unknown --features navegador --lib -j2 |

La referencia conserva literalmente texto y tokens de EIO-05, incluida su salida adversa. La variante también emite un cercado Markdown, aunque la petición prohíbe expresamente ese formato. No se extrajo ni reparó el JSON. Texto literal disponible en el log e INFERENCIAS.jsonl. El contrato anterior permaneció intacto; proceso0 no significa contratoOK.

El receptor Rust volvió a evaluar ambas salidas. Su retorno2 se conservó y, como estaba previsto, permitió la comprobación WASM independiente. Resumen literal:
```
EIO_06_RESUMEN contrato=2 wasm=0 navegador_ejecutado=false
```
La compilación WASM supera el fallo anterior de getrandom; no acredita ejecución en navegador, relojes, enlace JavaScript, paridad, WASI o semántica. No se ensayaron variantes adicionales ni comparación on/off. Los dos casos no sirven para estimar fiabilidad general.

## Recursos, tiempos y parada

GitHub registra inicio20/09/2026 04:23:34UTC y actualización final completed04:28:31UTC (06:28:31Madrid), diferencia297s. No son marcas del mensaje humano. Runner declara Ubuntu24.04.5, imagen ubuntu-24.04/20260907.300.1. Límite40min/plazo2280s conservados.

262 muestras nominales1s: máximo disco contabilizado3244355584bytes, mínimo libre89172869120bytes, máximo evidencia2101248bytes, máximo RSS compilación1951128KiB e inferencia1644780KiB. No son medidas del PC ni garantías entre muestras. Banco/recepción sin muestras; pico0 no significa consumo nulo. Ocho cierres informan residuales_al_wait=0 y residuales_tras_limpieza=0. No parada forzada ni vigilancia independiente acreditadas. Causa normal sólo describe terminación natural y no convierte retorno2 en conformidad.

Ambas guardas quedan false en este commit de cierre; workflow mantiene número6/intento1. No se alteran fuentes para corregir los resultados. Job terminado y limpieza automática observada; sin ejecución experimental propia en curso, sin afirmar inventario exhaustivo de infraestructura.

## Custodia, operaciones y límites

JOB_106020582816.txt conserva íntegra la cadena recuperada del conector de logs con timestamps/ANSI; no se afirma identidad del ZIP original de Actions. Los cinco archivos base64 sí conservan identidad binaria cotejada. JSON/TSV restantes son derivados explícitos del log y no sustituyen sus salidas literales stdout/stderr. Telemetría on está en stderr; ambos casos informan descartados0/errores0. MANIFIESTO.tsv describe los archivos documentales nuevos (excluye su propia huella).

Coordinación mediante conector GitHub y pestaña temporal del navegador. Una consulta PowerShell/rg de sólo lectura al índice de memoria al inicio, sin datos útiles para EIO; ningún archivo local creado o alterado. Timeout de lectura visual CDP durante seguimiento, recuperado sin nuevo dispatch. Procesamiento documental/base64/SHA256 en memoria. No instalación, pesos, compilación/inferencia en PC, contenedores locales, WSL, VSCode, cámara/micrófono, almacenes de credenciales ni capturas de supervisión. No inventario general, limpieza ni actuación en X:/Z:. No comunicaciones a terceros. No se inspecciona persistencia interna de herramientas o infraestructura.

Seguridad pasiva: perímetro/recursos. Seguridad activa: cotejos/rechazos dentro del contrato probado. Observabilidad: registros del adaptador/supervisor, no observación independiente ni razonamiento interno del modelo. Sin nuevo producto de observabilidad, credenciales HF, nuevos hosts, caché de proyecto o upload-artifact.

Originales y antecedentes conservados. Sin cambios canónicos, núcleo, S32/BIS-03, dominios, Profesor ni otros encargos. No acredita privacidad, H1, cualificación ni aceptación científica. **Pendiente revisión receptora; detenerse sin otro ensayo.**
