# Casos precomprometidos; ninguno ejecutado

Candidata EIO-NAT/1. Preparación estática del 20/09/2026. No reutilizar los éxitos 24/24 y 35/35 históricos como resultados de esta copia.

## Banco futuro y separación de fases

Toda orden se registrará mediante registro-futuro.sh: argumentos exactos con escape Bash, stdout, stderr, retorno y fechas UTC; directorio nuevo por orden. La duración monotónica corresponde a los journals de la candidata. Capturar aparte versiones de SO/kernel, libc, linker, rustc -Vv, cargo -Vv, lock y hashes de binarios. Este script tampoco se ha ejecutado.

Tras habilitación económica y recepción de fuentes, resolver primero el lock en un directorio aislado y conservar el diff respecto al antecedente. Después, con Rust 1.98.0 exacto:

~~~sh
cargo +1.98.0-x86_64-unknown-linux-gnu test --locked --test frontera
cargo +1.98.0-x86_64-unknown-linux-gnu test --locked --test json_complementario -- --nocapture
cargo +1.98.0-x86_64-unknown-linux-gnu run --locked --bin banco
cargo +1.98.0-x86_64-unknown-linux-gnu build --locked --bins
~~~

Esperados: frontera 5 tests conformes; JSON complementario 35 controles/0 fallos; banco 24 controles/0 fallos. El último build construye servidor/supervisor/testigo/cotejo/banco, sin Candle. No es todavía autorización para build --features inferencia.

## Preparación de cada escenario instrumental

No existen comandos implícitos de creación de Codespaces. Sólo tras autorización, preparar una copia de INACTIVA.json fuera del checkout inmutable. Habilitar esa copia, fijar origen HTTPS privado efectivamente observado, campana ASCII nueva, modo testigo y uno de los casos de la tabla. El origen debe coincidir exactamente con Origin y Host que lleguen al servidor; comprobar la plataforma antes del modelo. Mantener un único proceso supervisor y un único escenario por lanzamiento. No ejecutar escenarios en paralelo.

Orden de lanzamiento futura, desde la sede autorizada y con rutas absolutas verificadas:

~~~sh
/path/candidata/target/debug/supervisor /path/config-habilitada.json /path/evidencia-NUEVA /path/entradas-sinteticas
~~~

El supervisor crea su servidor hermano, enlaza 127.0.0.1:3000, usa control.sock y espera intervención humana. No lanza inferencia por abrir la página. Entradas sintéticas puede estar vacío. El testigo no lee pesos. Registrar PID, start_ticks, grupo y salida del supervisor exterior. Comprobar puerto privado y prueba negativa sin autenticación en sesión separada. No sustituir esos controles por el candado del navegador.

Abrir únicamente la URL privada ya verificada. Pulsar una petición fijada; consultar estado. La API es POST /api y cuerpo JSON documentado en CONTRATO.md. Todas las operaciones API, incluso consultas, requieren Origin exacto, Host exacto y application/json. Usar consola/red del navegador autenticado para observar códigos; no copiar cookies, tokens ni URLs firmadas al informe público.

| ID | Estímulo reproducible | Esperado independiente |
|---|---|---|
| N01 | Caso normal, referencia; recuperar entrada.txt | Bytes idénticos a pruebas/peticion.txt; un ID campana-01, secuencias 1..12, cierre terminada y cuatro archivos concordantes |
| N02 | Repetir con estructurada en nueva sesión | Bytes idénticos a petición estructurada; no normalización |
| N03 | POST JSON roto, clave op/id repetida, contrato EIO-NAT/2 o referencia libre | 400 ESTRUCTURA/VERSION/PETICION_NO_FIJADA, sin proceso testigo; estado ausente |
| N04 | Caso bloqueo; dos solicitudes iniciar próximas | Primera activa, segunda 409 SESION_CONSUMIDA_O_BLOQUEADA; exactamente un hijo; no cola de trabajo |
| N05 | Caso hostil | Texto literal de script/img; juicio ESTRUCTURA; cero ejecución HTML, cero solicitudes de ese texto, cero efectos |
| N06 | Caso bloqueo; cancelar inmediatamente después de inicio | Respuesta cancelacion_solicitada no prueba parada; SIGTERM, SIGKILL tras 250 ms, waitpid confirmado y pipes cerrados; interrumpida y evidencia incompleta |
| N07 | Caso abrupto | Retorno 77; conservar primer frame; desconocida, nunca terminada/OK |
| N08 | Caso omision (B ausente, secuencia renumerada) | Oráculo exige nombres precomprometidos: incompleta aunque no haya salto |
| N09 | Caso duplicado | seq 1 repetida: rechazo, revocación y bytes originales conservados hasta límite |
| N10 | Caso orden | seq 3 después de 1: rechazo; no corregir ni reordenar |
| N11 | Cancelar justo al acabar normal; repetir sólo en campañas sintéticas futuras expresamente presupuestadas | Si recibida antes del compromiso exterior: interrumpida; si ya comprometida: 409 NO_CANCELABLE. No aceptar resultado tardío |
| N12 | Caso escritura (fallo inyectado después del primer frame); además, montaje exterior de prueba con menos de 2 GiB disponibles | Error de escritura/margen; no cierre completo, no nueva tarea ni bucle de reintentos. Inyectar sólo en montaje desechable autorizado; nunca llenar el disco del PC |
| N13 | Caso memoria en entorno sintético habilitado con barrera exterior | Superar 4294967296 RSS agregada dispara revocación; registrar muestra, tiempos de señales/wait, máximo observado y sobrepaso. No rebajar umbral |
| N14 | Casos cola (4096 frames con 32 KiB de relleno) y registro (65537 bytes sin LF) | Cola/límite explícitos, bytes no recuperados declarados, ningún cierre completo. El caso cola puede alcanzar primero otro límite; sólo acreditar la rama cola si consta ese motivo, no inferirla de cualquier rechazo |
| N15 | Origin distinto/ausente, Host distinto, tipo distinto, cuerpo >8192, op/ruta/archivo ajenos | 403/413/400/409 según frontera; ninguna ruta del sistema controlable desde el modelo |
| N16 | Descargar los cinco archivos desde otra sesión y ejecutar cotejo sobre copia | Cuatro tamaños/SHA concordantes y retorno 0; hash del manifiesto calculado también en ambos extremos por sha256sum; no acredita completitud |
| N17 | SIGTERM al supervisor durante bloqueo | Revoca hijo, cierra servidor; conservar stderr/custodia. Comprobar procesos en plataforma; SIGKILL al supervisor requiere guarda exterior pendiente |
| N18 | Modelo futuro, únicamente después de N01–N17 y resolver brechas | Misma plantilla/pesos/tokenizador, marcas antes/después de carga y primer forward; salida original y juicio separados, cualquiera que sea el juicio |

El montaje exterior de N12 y la guarda independiente ante muerte del supervisor necesitan preparación adicional revisada. N14 tiene inyector, pero la cobertura determinista de cola frente a otros límites está pendiente de medida. N11 necesita planificación de varias carreras: no autoriza reintentar el modelo. El caso memoria puede ser peligroso sin contención externa y no está habilitado por esta entrega.

## Comprobación del cierre

La salida terminada es estado técnico del proceso, no conformidad del modelo ni aceptación SV. Leer el juicio separado. Verificar orden y correlación de spans con el banco heredado; la comprobación exterior de esta candidata exige inventario y secuencia, pero todavía no valida todos los IDs padre/trace en tráfico hostil. No afirmar exhaustividad.

Se conservarán originales antes de cualquier análisis. Hash de manifiesto y archivos mediante sha256sum en emisor y receptor independiente. Ejecutar cotejo DIRECTORIO_RECUPERADO con el binario previamente identificado. Alterar sólo una copia para prueba negativa: un byte diferente debe producir NO_CONFORME_IDENTIDAD. Reordenar eventos en una copia debe fallar el oráculo, aunque alguien recalcule un manifiesto: huellas y cobertura son propiedades distintas.
