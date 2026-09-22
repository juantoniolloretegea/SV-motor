# Candidato 0.1.2: recuperación, correlación y medición

**Fecha:** 22 de septiembre de 2026. **Corte de origen:** `eec0d87fd9e41a3341b799540bb4545c6dd686d9`, SV-motor. **Estado:** preparado, sin compilación ni despliegue acreditados.

El titular autorizó corregir la continuidad del ensayo y comparar el comportamiento del modelo. La revisión conserva Qwen3-0.6B Q4_K_M, el tokenizador, la revisión de Candle y las versiones de dependencias. No modifica el núcleo, la IR ni la constitución de los dominios.

## Cambios propuestos

- Indicador con texto y color, fecha de la última comprobación y distinción de una sesión del servicio no admitida. Las consultas se espacian tras fallos, se suspenden cuando la pestaña está oculta y se reanudan al recuperarla; no mantienen artificialmente activo el Codespace.
- Conservación del identificador, texto, perfil y huella de contexto antes de enviar. Recuperación mediante `request_status`; una repetición explícita utiliza el mismo identificador. La recuperación contrasta conversación, contexto, texto y perfil. Los borradores se separan por conversación.
- Exclusión entre instancias que custodian el mismo directorio; reserva del puerto antes de reconstruir el registro. Cierre solicitado mediante SIGTERM o SIGINT, cancelación de la generación y conservación del resultado antes de confirmar la parada cuando se dispone de tiempo suficiente.
- Registro del proceso en `servicio/ciclo.jsonl`, separado de los expedientes. Este registro no está firmado ni encadenado criptográficamente y no atribuye la causa de parada de GitHub. Los registros históricos de conversación conservan su formato y sus bytes previos.
- Medición independiente de carga del tokenizador, carga del modelo, procesamiento inicial del contexto y generación. Los valores ausentes o correspondientes a fases interrumpidas no se extrapolan.
- Procedimiento de [recuperación](RECUPERACION.md) accesible en GitHub cuando la aplicación no puede servirse.

## Pruebas preparadas en Rust

Las pruebas de `src/checks.rs` cubren repetición sin duplicación, colisión de identificador con otro contenido, consulta de una petición ausente, recuperación de una petición interrumpida sin modificar el prefijo del registro, ausencia de un segundo cierre, rechazo de una clave obsoleta, rechazo entre orígenes y rechazo de operaciones durante la parada. La prueba de `src/lifecycle.rs` cubre exclusión entre instancias y rechazo de un registro de servicio incompleto.

Estas pruebas están escritas, pero **no se han ejecutado**. Deben compilarse y ejecutarse con Rust/Cargo 1.98.0 y las dependencias fijadas. La comprobación de sintaxis del JavaScript no acredita comportamiento de navegador ni sustituye las pruebas de continuidad.

## Comparación preparada

El modo nativo `--compare DIRECTORIO_NUEVO` prepara doce ejecuciones: tres preguntas —léxico, conservación literal de una entidad y conservación de cantidades—, dos condiciones de antecedentes y dos modos de generación. Cada condición se ejecuta una vez. Los antecedentes son sintéticos, identificados expresamente, y el directorio debe ser nuevo. No se abre el directorio de expedientes del titular ni se sirve una API HTTP.

Límites: 384 tokens de salida y 180 segundos por petición. El límite máximo nominal de generación de la campaña es de 36 minutos, además de su preparación y cierre. No debe ejecutarse otra inferencia ni compilarse simultáneamente. Los modos utilizan sus respectivos perfiles recomendados; la comparación no aísla solamente el efecto del texto de razonamiento. Una sola ejecución por condición permite observar diferencias, no establecer significación estadística ni atribuir causalidad exclusiva.

`RESULTADOS.jsonl` se sincroniza después de cada caso; `RESUMEN.json` conserva identidades, condiciones, resultados y límites. Los oráculos se fijan antes de ejecutar. La valoración semántica se mantiene pendiente hasta examinar las salidas; una interrupción por tiempo o longitud se clasifica por separado.

## Interrupción operativa de esta revisión

La lectura inicial del editor mostró `Codespace is stopped`. Se solicitó su reanudación, que dio paso al editor. VS Code mostró después `Do you trust the authors of the files in this folder?`, con la explicación `Creating a terminal process requires executing code`. La confirmación `Trust Folder & Continue` quedó sin aceptar. No se eludió mediante otra terminal ni se ejecutó el candidato.

El entorno de edición local permite preparar archivos, pero no dispone de Rust/Cargo. La ejecución del plan queda pendiente de la confirmación de confianza de esa carpeta, exigida por la regla de interacción del navegador para cambios de protección en ese momento. Tampoco se ha acreditado un nuevo acceso externo a la página de conversación.
