# Verificación de la revisión 0.1.2 y comparación acotada de Qwen

**Fecha:** 22 de septiembre de 2026. **Ámbito:** conversación experimental EIO, lateral respecto al núcleo y la IR del Lenguaje SV. **Fuente de la aplicación compilada:** `feefbe68a4131c43c46673b841a0dd72b590e4f7`. El binario, las condiciones y las huellas se identifican en [VERIFICACION.json](VERIFICACION.json).

## Resultado operativo

La revisión 0.1.2 se ha compilado con Rust/Cargo 1.98.0 y dependencias fijadas. Las seis pruebas unitarias han superado sus comprobaciones. Una prueba adicional del proceso HTTP ha confirmado arranque, dos paradas mediante SIGTERM, renovación de identidad y clave al reiniciar, rechazo de la clave anterior, rechazo de una segunda instancia y conservación exacta de un expediente sintético. El tratamiento de un JSON malformado devuelve HTTP 400; la clave no admitida devuelve HTTP 403.

El primer intento de esta prueba no abrió el puerto dentro de los 15 segundos previstos y terminó con retorno 101. En la observación posterior se comprobó un proceso vivo esperando lectura de disco, sin puerto a los 11 segundos y disponible en la lectura realizada a los 32 segundos. Se amplió a 60 segundos el plazo de arranque del ensayo, sin cambiar el programa, y se conservaron el diagnóstico y el resultado inicial. El segundo intento pasó en 2,62 segundos. Estos datos no permiten atribuir una causa exclusiva a la variación de arranque. [Registros de ambos intentos](VERIFICACIONES.json).

El servicio real se ha iniciado con el binario verificado; se conserva una copia de la versión 0.1.1. La comprobación nativa de la instalación obtuvo HTTP 200 para la página, JavaScript y CSS, y confirmó la presencia del aviso de titularidad y licencias, del indicador, de la última comprobación y de la recuperación de peticiones. No ejecuta JavaScript. Las huellas de los dos expedientes existentes permanecieron idénticas antes de las pruebas, después de la campaña y después del arranque real. [Constancia del despliegue](DESPLIEGUE.json).

El acceso externo desde el navegador **no se ha verificado en esta revisión**: el control de navegación de esta sesión bloqueó la URL privada. La prueba local no acredita la sesión de GitHub, el reenvío externo ni el comportamiento visual. El puerto debe permanecer privado. La disponibilidad comprobada es puntual y no constituye un servicio permanentemente activo.

## Qué modelo se está examinando

Los archivos cotejados corresponden a **Qwen3-0.6B, cuantización Q4_K_M, CPU**, con Candle en la revisión `ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a`. La identidad ensayada no corresponde a Qwen Max. El tamaño del archivo, sus huellas y la identificación efectiva del ejecutable prevalecen sobre una denominación atribuida verbalmente al modelo. [Referencia del fabricante para Qwen3-0.6B](https://huggingface.co/Qwen/Qwen3-0.6B/blob/c1899de289a04d12100db370d81485cdf75e47ca/README.md).

La ejecución carga pesos y tokenizador. No se ha identificado un archivo de diccionario español ausente; el banco de idiomas del SV no participa en esta ruta. El modelo recibe una instrucción general de respuesta en español y los antecedentes de la conversación. No dispone aquí del dominio acotado de conocimiento ni de las restricciones de consejo previstas para el SV. No se ha cambiado el modelo, la cuantización, el tokenizador ni Candle para obtener otro resultado.

Cada petición crea un proceso de inferencia, carga de nuevo los pesos y procesa el contexto seleccionado. La memoria de cálculo del historial no se reutiliza entre peticiones. La ventana operativa configurada de 16.384 tokens no es una longitud de conversación útil acreditada; tampoco se ha validado el máximo anunciado de 32.768. Una finalización normal acredita terminación del proceso, sin validar el contenido ni la suficiencia profesional de la respuesta.

## Diseño de la comparación

Se ejecutaron doce generaciones: tres preguntas, dos condiciones de antecedentes y dos modos de generación, con una ejecución por condición. La prueba léxica pide explicar «tonto»; la textual exige repetir exclusivamente «tricoleucemia»; la cuantitativa plantea 17 piezas menos 5. La prueba textual no evalúa conocimiento sanitario. Los criterios se fijaron antes de ejecutar y se conservan junto a cada resultado.

Los antecedentes consisten en 24 registros sintéticos y neutros, identificados como tales, sobre una etiqueta de un contenedor. No reproducen la conversación sanitaria aportada por el titular ni permiten concluir que se haya reproducido su deriva temática. Se reservaron 384 tokens de salida y 180 segundos por petición, con semilla 299792458. El modo con razonamiento utiliza temperatura 0,6, Top-P 0,95 y Top-K 20; el directo utiliza 0,7, 0,8 y 20. Cambiar el modo cambia también el muestreo: no se aísla solamente el razonamiento.

Las generaciones fueron secuenciales, sin otra inferencia ni compilación de esta intervención en paralelo. No se registró continuamente toda la actividad de CPU de la máquina. Los resultados se sincronizaron después de cada caso. La campaña finalizó a las 12:43:56,649 UTC; la suma de los tiempos de petición fue de 1.230,11 segundos. [Resultados nativos completos](RESUMEN.json) y [valoración explícita](VALORACION.json), conservados por separado.

## Resultados observados

| Pregunta | Antecedentes | Razonamiento | Entrada, tokens | Preparación del contexto, s | Petición, s | Valoración |
| --- | --- | --- | ---: | ---: | ---: | --- |
| Léxico | No | Desactivado | 127 | 16,31 | 34,11 | Significado incorrecto |
| Léxico | No | Activado | 123 | 15,79 | 55,96 | Significado incorrecto |
| Léxico | Sí | Desactivado | 833 | 132,57 | 148,11 | Significado incorrecto |
| Léxico | Sí | Activado | 829 | 132,17 | 180,13 | Interrupción por tiempo |
| Entidad | No | Desactivado | 139 | 18,09 | 22,39 | Término conservado; formato incumplido |
| Entidad | No | Activado | 135 | 17,58 | 37,03 | Término conservado; formato incumplido |
| Entidad | Sí | Desactivado | 845 | 134,55 | 139,21 | Término conservado; formato incumplido |
| Entidad | Sí | Activado | 841 | 147,07 | 180,21 | Interrupción por tiempo |
| Cantidad | No | Desactivado | 140 | 28,92 | 34,95 | Conforme |
| Cantidad | No | Activado | 136 | 23,99 | 60,88 | Conforme |
| Cantidad | Sí | Desactivado | 846 | 152,47 | 157,02 | Conforme |
| Cantidad | Sí | Activado | 842 | 151,07 | 180,12 | Interrupción por tiempo |

Hubo nueve terminaciones normales y tres interrupciones por tiempo, estas últimas en las condiciones con antecedentes y razonamiento. Una interrupción conserva su salida parcial y no se contabiliza como respuesta completa semánticamente incorrecta.

Las tres definiciones completas reconocen que «tonto» pertenece al español, pero no explican correctamente su significado. Se observaron definiciones tautológicas, atribuciones de genialidad y afirmaciones de procedencia inglesa. La entrada del [Diccionario de la lengua española, RAE y ASALE](https://dle.rae.es/tonto), consultada el 22 de septiembre de 2026, no respalda esas atribuciones. Activar el modo con razonamiento no corrigió el error en el caso sin antecedentes; esta observación no estima su eficacia general.

Las tres respuestas textuales completas conservan «tricoleucemia», pero añaden una oración. Por tanto, se distingue conservación de la entidad de incumplimiento de la instrucción de salida exclusiva. Las tres restas completas responden «Quedan 12 piezas.» y satisfacen el criterio. No corresponde reducir estos resultados diferentes a una sola etiqueta global de capacidad del modelo.

La preparación del contexto consumió entre **15,79 y 28,92 segundos sin antecedentes** y entre **132,17 y 152,47 segundos con ellos**; las entradas fueron de 123–140 y 829–846 tokens, respectivamente. La carga de pesos osciló entre 1,21 y 2,42 segundos. En esta muestra, la preparación del contexto explica una parte dominante de la espera previa a la salida. Los intervalos del proceso de inferencia no incluyen red ni preparación anterior en el servidor. No se extrapolan las fases incompletas.

## Consecuencias y límites

La revisión permite observar mejor las demoras y recuperar el estado de las peticiones. No acredita una mejora de velocidad ni una corrección de la capacidad lingüística. La espera resulta explicable técnicamente, pero sigue siendo una limitación de uso para conversaciones con antecedentes. El arranque, la autenticación y el procesamiento del contexto deben evaluarse como fenómenos distintos.

Antes de atribuir los errores exclusivamente al modelo o a su configuración, queda por contrastar fidelidad de implementación con otro motor y analizar por separado cuantización, plantilla y parámetros de generación. Esta campaña no contiene esa comparación ni autoriza una conclusión causal definitiva. Los tiques de rendimiento y fidelidad permanecen abiertos.

La exclusión de instancias y la recuperación de registros se han probado en Rust; la continuidad de la pestaña y la confirmación de reenvío requieren todavía comprobación de navegador. La autenticación profesional, el bloqueo por ausencia y el relevo de persona conservan su fase específica. No se ha implantado una reactivación autónoma de Codespaces ni se ha atribuido a una renovación de token de una integración acceso hostil al entorno.

## Reproducción acotada

Las seis pruebas se ejecutan mediante `cargo test --release --locked --offline --bin eio-conversacion`, con Rust/Cargo 1.98.0 y las dependencias presentes. La prueba [tests/service.rs](../tests/service.rs) requiere el puerto 3000 libre, pesos identificados y un directorio nuevo indicado en `EIO_SERVICE_CHECK_DIR`; se invoca con `cargo test --release --locked --offline --test service -- --ignored`. No debe ejecutarse sobre el directorio de expedientes real.

La campaña se inicia mediante `eio-conversacion --compare DIRECTORIO_NUEVO`. La comprobación [comprobar_instalacion.rs](comprobar_instalacion.rs) se compila directamente con rustc 1.98.0 y consulta únicamente la página y sus recursos locales; no realiza inferencias ni publica claves o contenido de expedientes. [Procedimiento de recuperación del servicio](../RECUPERACION.md).
