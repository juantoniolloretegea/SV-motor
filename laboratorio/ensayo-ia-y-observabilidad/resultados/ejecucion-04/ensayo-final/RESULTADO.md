# Resultado EIO · bloqueo documentado

20/09/2026 Europe/Madrid. [Run35474691239](https://github.com/juantoniolloretegea/SV-motor/actions/runs/35474691239), número3, intento1, fase ensayo, failure. Commit solicitado/efectivo48a2bb568d24e3cc3c6bf2834e78d5b740578c46. Inicio19/09 22:55:44UTC; actualización final22:56:13UTC (20/09 00:55:44 y00:56:13 Madrid). Duración UI29s, trabajo23s. Contador3/3 agotado, sin reintento.

## Instalación observada

Inicio:92418494464bytes libres y15147376KiB de memoria disponible. Ubuntu24.04.5, imagen20260907.300.1, runner2.337.0. Los cuatro paquetes Rust del manifiesto se descargaron de static.rust-lang.org con HTTP200, tamaño/SHA coincidentes antes de extracción. Instalación aislada rustc,cargo,std nativo y std WASM en /home/runner/work/_temp/eio/toolchain-1.98.0. rustc1.98.0 (88d9e12ae178fab0fb5cc050a94da85685d449ea), LLVM22.1.8; cargo1.98.0 (797e8a9bca276c1c9f9f738d2a20f484fa4eea9d). Cargo.lock:OK. cargo fetch --locked llegó a la siguiente orden sin compilar.

## Bloqueo exacto

La primera petición del GGUF devolvió302 y el descargador propio rechazó el destino antes de solicitarlo:

```text
GET host=huggingface.co salto=0 destino=Qwen3-0.6B-Q4_K_M.gguf
HTTP=302
DESTINO_NO_ADMITIDO
residuales_tras_limpieza=0
fase=aprovisionamiento retorno=65 causa=normal pico_rss_kib=142436 residuales_al_wait=0
```

No es un rechazo HTTP403 ni incompatibilidad demostrada del modelo. Host/Location rechazado no quedó registrado; su identidad y motivo específico se desconocen. No se investiga otra ruta ni amplía la lista. No se adquirieron/cargaron pesos; la secuencia no solicitó el tokenizador. causa=normal sólo indica ausencia de parada por umbral del supervisor, no éxito del hijo. Código65 propagado; GitHub terminó el trabajo y limpieza. Cero residuos observados en el grupo, sin afirmar vigilancia exhaustiva del ejecutor.

## Compilación, ejecución y medidas

No hubo compilación nativa/WASM, banco, inferencia, costes o navegador. Los24 controles no ejecutados no son conformidades ni fallos observados. Memoria del modelo/compilador, calidad, tokens y coste de telemetría no medidos. [Matriz](MATRIZ.tsv).

17 muestras de aprovisionamiento: pico RSS142436KiB; máximo de directorios vigilados1651216384bytes (no incremento neto exclusivo); mínimo libre90764513280bytes; evidencia observada hasta16384bytes. Intervalo nominal1s, salto13 a15s sin muestra intermedia inventada. No exceso detectado en muestras; no techo impuesto. [Medidas](MEDIDAS.tsv).

Checkout usó SHA fijado y retiró credenciales. GitHub impuso Node24 al checkout que declara Node20: aviso conservado sin cambiar políticas. “Cache mode: write” del runner no prueba caché del ensayo; no se usaron acciones de caché/upload-artifact. Marcadores *** ya enmascarados por GitHub.

## Custodia y reproducción

JOB_105981800036.txt conserva íntegro el texto decodificado devuelto por el conector, con stdout/stderr/medidas; no se afirma identidad binaria de un ZIP remoto. RUN.json y JOBS.json guardan metadatos/pasos. MANIFIESTO.tsv fija bytes/SHA de copias textuales UTF-8 y se excluye a sí mismo. Sin pesos/target/binarios en Git. Relectura se documenta en la entrega privada.

Procedimiento exacto fijado en48a2bb568d24e3cc3c6bf2834e78d5b740578c46: workflow_dispatch fase ensayo con SHA completo, distribución directa, fetch --locked y secuencia de ensayar.sh. Permite revisión/reproducción técnica bajo las mismas cotas, pero no autoriza relanzar:3/3 agotado y guarda cerrada en resultados. Sólo instalación instrumental remota y rechazo de redirección acreditados. Sin base para recomendar inferencia local, aceptar nativo+navegador o promover plataforma. Sin aceptación científica/cambio S32/BIS-03. Parada para revisión receptora.
