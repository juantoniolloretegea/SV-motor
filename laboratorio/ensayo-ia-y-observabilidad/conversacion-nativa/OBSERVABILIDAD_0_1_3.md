# Observación del servicio de conversación: criterios previos

Fecha: 22 de septiembre de 2026. Ámbito: ensayo lateral de conversación Qwen mediante Candle, sin modificación del núcleo, de la semántica ni de la representación intermedia.

La versión 0.1.2 conservaba sucesos propios y duraciones, pero no integraba OpenTelemetry. La versión propuesta incorpora la API y el SDK de OpenTelemetry Rust 0.31.0, con exportación exclusivamente local. No se añade un servidor recolector ni otro sistema de telemetría.

## Dos fuentes de observación

El supervisor instrumenta la recepción de operaciones, la creación del proceso de inferencia, las fases declaradas por este, la terminación y la conservación del resultado. La petición tiene una traza que relaciona sus tramos mediante identificadores OpenTelemetry. Las duraciones comunicadas por el inferidor se identifican como declaraciones de ese proceso; el supervisor mide por separado su duración total.

Un proceso observador utiliza el mismo SDK y consulta Linux cada cinco segundos. Del servicio y de sus descendientes visibles obtiene identidad PID y tiempo de creación, estado, CPU acumulada en unidades del reloj del sistema, memoria residente, hilos, contadores de entrada/salida, descriptores, archivos abiertos y sockets atribuibles por su identificador. No exporta preguntas, respuestas, variables de entorno, credenciales ni direcciones IP. Los tamaños de archivos abiertos se deduplican por dispositivo e identificador de archivo dentro de cada proceso. No deben sumarse indiscriminadamente entre procesos. La reserva del sistema de archivos se informa por separado y no se atribuye a Qwen.

El observador está separado del supervisor, pero comparte máquina y cuenta. Puede advertir que aquel desaparece; no puede certificar la causa de una parada de GitHub ni sobrevivir al apagado de su propia máquina. No sustituye la autenticación, la supervisión de privilegios ni el aislamiento. Los sockets son una muestra de descriptores vivos, no una captura de tráfico ni una prueba de ausencia de comunicaciones. Las lecturas no constituyen una instantánea atómica.

## Cotas y degradación explícita

Cada exportador dispone de diez MiB por ejecución: veinte MiB entre supervisor y observador. Cada registro tiene un máximo de 32 KiB. Se conservan contadores de escrituras confirmadas, errores y registros descartados; una pérdida queda señalada y no se transforma en éxito. Los archivos históricos se conservan: el límite por ejecución no es una cuota acumulada de toda la instalación.

Cada observación limita los procesos a 16, la cola a 32, los hilos examinados a 128 por proceso y los descriptores a 512. Las lecturas de archivos tienen límites de longitud. El recorrido comprueba un presupuesto de 250 ms entre procesos; no constituye un plazo estricto para cada llamada del sistema. Las ausencias, carreras y recortes se consignan. No se recorre el árbol completo de archivos de Cargo ni del espacio de trabajo.

El observador termina al perder la identidad del proceso, alcanzar 24 horas o detectar una exportación incompleta. Su estado se actualiza mediante sustitución de un archivo JSON. Una muestra de más de veinte segundos se considera no reciente. La interfaz distingue la conexión HTTP de la observación activa. La admisión de nuevas inferencias se detiene si la exportación propia deja de ser íntegra o el observador no confirma actividad reciente; la consulta y exportación de expedientes siguen disponibles. No se repite automáticamente ninguna petición.

## Verificación prevista antes de activar

1. Compilar y probar exclusivamente con Rust y Cargo 1.98.0, manteniendo las dependencias anteriores y fijando OpenTelemetry 0.31.0.
2. Comprobar parentesco de trazas, duraciones, límite de exportación y fallo real de escritura mediante `/dev/full`.
3. Rechazar una identidad PID/tiempo de creación incorrecta; detectar un archivo de un MiB borrado mientras permanece abierto; atribuir al proceso un socket de escucha creado para la prueba.
4. Comprobar la desaparición de un proceso sintético sin depender de que este declare su finalización.
5. Realizar una única inferencia no clínica en datos sintéticos y un puerto local separado; relacionar su suceso final, su traza y el proceso observado. Medir el coste de las lecturas y el volumen exportado. No presentar esta ejecución como prueba de determinismo ni como comparación del rendimiento con y sin instrumentación.
6. Activar la versión verificada cuando no haya una inferencia del usuario en curso. Confirmar que avanzan los registros y las muestras y conservar las evidencias de la sustitución.

## Recuperación de ensayos anteriores

Los controles anteriores de instrumentación EIO-05 y su comparación de costes se conservan como antecedentes. No acreditan el coste de esta aplicación nueva. La matriz conversacional de doce casos de 0.1.2 tampoco se repite.

De la cualificación del medidor de disco se recupera el contraejemplo del archivo borrado y todavía abierto. La prueba masiva de archivos pequeños ya mostró que recorrer un árbol puede superar el tiempo disponible: no se repite sin una nueva cuestión experimental. Un límite de tiempo agotado expresa una medición incompleta, no una cuota de disco excedida.

De la preparación de controles nativos se recupera la separación entre declaración del proceso y observación de su terminación. El nuevo ensayo no convierte en ejecutados los controles históricos pendientes ni acredita sus condiciones de grupos de control o restitución de permisos.

La comparación determinista y la revisión de la adenda de ciberseguridad quedan para pasos posteriores. Esta intervención aporta mediciones y criterios de cobertura al encargo original; no declara completada la integración de la IA en las rutas del núcleo.
