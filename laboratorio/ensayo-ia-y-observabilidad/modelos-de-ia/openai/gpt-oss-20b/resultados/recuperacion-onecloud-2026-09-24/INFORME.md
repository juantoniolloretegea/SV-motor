# Recepción de evidencias y recuperación en OneCloud · 24/09/2026

**Unidad receptora:** Agente Watson / W-S39-02. **Base de lectura:** VERIFICACION_ACOTADA.  
**Registro:** 2026-09-24T11:06:58Z. **Suceso:** S39, revisión 16. **Tique:** TT-0012. **Parte:** PTA-SVM-002.  
**Cortes de entrada:** SV-motor `f3746b719a3e355d75ea566d3d3abde2b6ea9e0e`; Lenguaje `d19bb1b33d5dea39926c1861aab5198e9556f186`.

## Dictamen

Se recuperan el ejecutable candidato y evidencias del diagnóstico; la copia en OneCloud conserva la identidad SHA-256 y ejecuta `--version`. El defecto del cálculo CPU MXFP4 está demostrado por una prueba de regresión específica antes/después. **La inferencia completa con el parche continúa pendiente.** S39 permanece en seguimiento y TT-0012 abierto. Esta recepción no acredita conformidad integral, respuesta útil ni aptitud productiva.

La unidad entrante recibe las ejecuciones históricas; no se atribuye su realización. La observación remota de las 11:00:05 UTC del 24/09 es nueva. No se ha iniciado una inferencia durante esta revisión documental.

## Método y fuentes

Se leen los registros canónicos, los archivos recuperados y las trazas nativas. Los ficheros de `evidencias/` conservan su contenido recuperado; los llamados `intento-N-seleccion.jsonl` son selecciones explícitas de líneas originales, no registros completos: todas las líneas salvo las de tipo `muestra_recursos`, sin editar sus campos. El manifiesto identifica las fuentes completas por SHA-256. Los archivos completos de recuperación se conservan en la máquina y en la entrega de continuidad. El máximo muestreado no se convierte en pico exacto.

## Conciliación de los intentos

| Registro | Resultado recibido | Interpretación admisible |
|---|---|---|
| 22 | Chat HTTP 200, ocho tokens en logprobs y `content:null`. | Generación observada; no respuesta final útil. |
| 23 | HTTP 400: `logprobs` booleano incompatible con el entero esperado en completions. | Error de preparación de nuestra petición; rechazo anterior a la generación. |
| 24 | Completions HTTP 200, 60 tokens de entrada y 16 de salida; texto `... <\|constrain\|>1. 1}... 1. 1... `. | La salida directa tampoco resuelve la suma solicitada; no basta explicar el defecto por ocultación en `content`. |
| 25 | Completions sin respuesta HTTP completa; `http_incompleto`, cierre del hijo confirmado, señal 15 y emisor no atribuido. | No se dispone de una salida completa para juzgar el contenido. No se declara otro resultado textual defectuoso. |
| 26–28 | Rechazos de admisión antes de crear el hijo. | No son inferencias del motor corregido ni fallos semánticos del modelo. |
| Revisión 32 | Seis observaciones de disponibilidad; admisión insuficiente; motor no iniciado. | La revisión del margen no consiguió iniciar la candidata. |

La recepción no renumera ejecuciones. Los antecedentes 29–31 permanecen en los paquetes de continuidad; no se les atribuye aquí un resultado de generación. La desconexión del editor es un incidente de infraestructura separado. No hay base en este cotejo para atribuir corrupción de pesos a Oryx o a Visual Studio.

## Cotejo numérico y corrección

Las muestras de normalización de las capas 0, 11 y 23 coinciden; los 216 bloques MXFP4 cotejados no presentan diferencias después de reconstruir su disposición. En doce matrices Q8_0, con 55 muestras por matriz, el orden directo presenta menor suma de error absoluto que las dos permutaciones examinadas. **Es un muestreo, no una certificación de todos los pesos.** Véanse los dos JSON de cotejo.

Motor de referencia: mistral.rs 0.9.3, revisión `24dbf5c256f232176ee5949485ba264049407fbe`. La rutina CPU confundía la dimensión intermedia 1 de una entrada compartida `[tokens,1,dimensión]` con el número de expertos seleccionados. El parche toma ese número de los índices y conserva la difusión de la entrada compartida.

Una prueba contiene tres representaciones equivalentes de la entrada: compartida tridimensional, bidimensional y expandida. Antes falla por obtener forma `[2,1,2]` en vez de `[2,2,2]`; después pasa y comprueba los ocho valores esperados. Resultado: **1 prueba aprobada, 340 filtradas**; los otros ejecutables del banco muestran cero pruebas seleccionadas. No se declara superada la suite completa. El parche incorpora la prueba y conserva la licencia MIT del componente.

Reproducción pendiente en un árbol limpio: obtener la revisión exacta, aplicar `CORRECCION_MXFP4_CPU.patch` y ejecutar con la configuración original el filtro `sv_cpu_shared_input_routes_to_each_selected_expert` del paquete `mistralrs-quant`. El registro de compilación no sustituye el contraste de la inferencia. La denominación F32 de una configuración anterior tampoco acredita aritmética íntegramente F32: el diagnóstico recibió una conversión interna a F16 en atención CPU.

## Memoria: revisión 32

La auditoría histórica registra máximo muestreado de memoria no respaldada por fichero de **12 896 575 488 B**. Redondeo y margen experimental de 384 MiB producen mínimo de carga de **13 300 137 984 B**; con reserva global de **536 870 912 B**, el umbral total es **13 837 008 896 B**.

La mejor disponibilidad de sus seis muestras fue **13 811 625 984 B**, inferior en **25 382 912 B**. El ejecutor concluyó sin iniciar el motor. El margen es una decisión experimental de ingeniería, no una cota matemática de todas las asignaciones. Los datos proceden del motor anterior al parche y no prueban suficiencia para la candidata.

## Recuperación y observación actual

El GGUF y el árbol de construcción dejaron de estar en sus antiguas rutas de `/tmp` tras recuperar el Codespace. No se ha determinado el instante o mecanismo de su desaparición ni se afirma que toda la caché se haya perdido. A las 08:39:28 UTC el registro histórico aún observaba el tamaño del GGUF, sin volver a calcular entonces su hash.

Se copiaron a `/opt/sv-lab/recuperacion-20260924/` el ejecutable candidato y tres archivos de evidencias, comprobando coincidencia de huellas en origen y destino:

| Artefacto | SHA-256 |
|---|---|
| mistralrs-candidato-20260924 | `2d6856918349d85a073fea59190c59886e780a62bcd94c6d7789060d04e99fb1` |
| EVIDENCIAS_RECUPERACION_ASTRA_XXI_20260924.tar.gz | `27a7df02746020957aca4dcac18d92a08b4a84722eec6dfbb135bbf7fb6e504e` |
| EVIDENCIAS_DIAGNOSTICO_PARCIAL_20260924.tar.gz | `04c339a64d1b7e96e5c3588f296e1416391bc8840ffac3f89483e17cbe57439b` |
| EVIDENCIAS_REVISION_MEMORIA_32_20260924.tar.gz | `e1a88589b07be24f19158432f9858aa258138fa7070c46edaacad01e86e28861` |

La observación de las **11:00:05 UTC** registra Ubuntu 26.04 LTS, kernel 7.0.0-14-generic, KVM, 12 CPU virtuales, AMD EPYC 7502, **67 417 477 120 B** de memoria total, **66 528 256 000 B** disponibles, sin swap y **639 364 923 392 B** disponibles en el sistema de archivos raíz. Son valores del invitado en ese instante. No acreditan núcleos físicos exclusivos ni rendimiento de inferencia.

El acceso SSH desde Codespace se comprobó con clave independiente y verificación estricta de la clave del servidor. No se publican credenciales ni datos de acceso. El motor devuelve `mistralrs 0.9.3`; esto acredita arranque básico, no todos los recorridos de instrucciones o dependencias.

## Identidades pendientes de restituir

- GGUF ggml-org/gpt-oss-20b-GGUF, revisión `b97cbb20d1995efd41dce8c4dd1ddf86e8db375b`: 12 109 566 624 B; hash histórico esperado `27cd6c432c7672cb812a92f611cf3ba7bbc35928262bb1e1253ff4ee6ae35901`. Su presencia e integridad en OneCloud aún no están comprobadas.
- Controlador revisado 0.1.24: huella histórica `dfc8afe8ce7c76019fc35687c481927b429ea731e69058444b86a6c151c6554f`. Falta comprobar su instalación operativa en OneCloud.
- Rust, dependencias y configuración efectiva en OneCloud: pendientes de inventario y preparación.

## Continuidad y límites de calidad

Antes de ejecutar: restituir y verificar los pesos, recuperar fuente/configuración del controlador, comprobar sus guardas en el anfitrión nuevo y fijar petición, motor, parámetros, límites y criterios de aceptación. Después, realizar una sola inferencia acotada con conservación de petición, respuesta, recursos y cierre. Para atribuir una mejora al parche será necesario un contraste controlado en el mismo anfitrión, pues la migración cambia el entorno.

Los resultados Qwen/B mantienen su cierre parcial y sus limitaciones. S42 y TT-0010, relativos al PC propio, no quedan resueltos por disponer de OneCloud. No se modifican contratos, gramática, IR, mapa HTML ni resultados históricos. La conciliación incorpora el desfase registral con fecha actual; no inventa asientos retroactivos ni pretende un barrido integral del proyecto.

Referencias canónicas: [S39](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md#s39), [TT-0012](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0012.md), Acta 004 §16, RETP-2026-269 y PTA-2026-011. La publicación queda identificada por el commit que contiene este informe.

© Juan Antonio Lloret Egea, 2026 · Sistema Vectorial SV. Los componentes de terceros conservan sus licencias; véase `evidencias/LICENSE_MOTOR_MIT`.
