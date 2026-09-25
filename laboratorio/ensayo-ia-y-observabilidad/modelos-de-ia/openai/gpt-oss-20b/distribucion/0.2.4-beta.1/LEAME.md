# Uso y conservación de la entrega

La entrega contiene software experimental y documentación de resultados favorables y adversos. Consulte primero FICHA_TECNICA.md y RESULTADOS.md. El paquete no contiene los pesos ni una imagen del sistema operativo. La instalación completa en una sede nueva no se ha ensayado durante este cierre.

## Archivos

- `sv-gpt-oss-0.2.4-beta.1-linux-x86_64.tar.gz`: ejecutables, fuentes, tokenizador, recurso Harmony, servicios, licencias y documentación.
- `sv-gpt-oss-evidencia-sintetica-20260925.tar.gz`: entradas, rúbrica y ejecución sintética HCL, con resultados y exportación. No contiene conversaciones personales históricas.
- `ARCHIVOS.sha256`: tamaño lógico mediante el manifiesto y huellas de los dos archivos comprimidos. El archivo interior `SHA256SUMS` coteja los archivos del paquete descomprimido.
- `MANIFIESTO.json`, `RECURSOS_MODELO.json`, prospecto, resultados y auditorías: alcance y procedencia de la entrega.

## Instalación en una sede nueva

El ejecutable conserva rutas absolutas de la instalación investigada. Se ofrece un procedimiento que reproduce esas rutas; no se acredita traslado arbitrario de directorios ni compatibilidad universal de binarios. La referencia observada es Ubuntu 26.04 LTS x86_64, glibc 2.43 y systemd. Un contenedor sin systemd no satisface este procedimiento.

1. Descargue los archivos de distribución y sus huellas desde la misma entrega de GitHub. Compruebe `sha256sum -c ARCHIVOS.sha256` y extraiga el paquete en un directorio de trabajo nuevo. No lo extraiga sobre una instalación activa.
2. Obtenga el GGUF desde la dirección fijada en `RECURSOS_MODELO.json`. Reserve espacio para la descarga y su copia de instalación: aproximadamente 25 GB para ambas copias de pesos, más el software y los datos. La referencia ensayada tiene 64 GB de memoria; esta entrega no establece un mínimo universal.
3. En el directorio `distribucion`, ejecute `sha256sum -c SHA256SUMS`. Revise las unidades de `servicios` y los límites del prospecto.
4. Ejecute, como administración del destino nuevo: `bash INSTALAR_EN_SEDE_NUEVA.sh http://127.0.0.1:3000 /ruta/absoluta/gpt-oss-20b-MXFP4.gguf`. Para acceso por túnel o intermediario, indique el origen real que utilizará el navegador. El procedimiento comprueba pesos, rechaza destinos existentes y no inicia servicios. Copia los pesos y conserva el archivo de descarga.
5. Cuando proceda iniciar una sesión, ejecute `systemctl start sv-conversacion.service` y consulte `systemctl status sv-conversacion.service`. La aplicación inicia el motor al admitir una consulta. `systemctl start sv-conversacion-motor.service` permite precargarlo deliberadamente; no forma parte de la comprobación documental.
6. Acceda por la interfaz local o un túnel autenticado. Mantenga los puertos locales y el origen concordante. Una consulta produce inferencia y consumo: no es una comprobación inocua de disponibilidad.
7. Para finalizar una sesión, `systemctl stop sv-conversacion.service` detiene también el servicio vinculado del motor. La duración máxima de ocho horas de la unidad conservada no equivale a servicio permanente.

La instalación requiere privilegios porque la aplicación gestiona la unidad systemd del motor. Esta arquitectura experimental no acredita separación completa de privilegios ni un servicio multiusuario. El procedimiento no configura reinicio automático al arrancar el sistema ni acceso público.

## Restauración documental sin inferencia

Trabaje siempre con una copia en un directorio nuevo. El registro HCL conservado se llama descriptivamente `EXPEDIENTE-ORIGINAL.jsonl` en el archivo de evidencia; para abrirlo con la aplicación, cópielo con el nombre `exp-1790315249328-1.jsonl`. El identificador del archivo debe coincidir con el contenido.

Ejecute `bin/eio-conversacion --check /ruta/copia-del-registro`. La comprobación reconstruye el estado y, si hubiera peticiones pendientes, registra su cierre en esa copia. No debe apuntarse a los originales ni a datos de un servicio activo. En el expediente cerrado de esta entrega se observaron un expediente, doce conversaciones, 49 sucesos y cero peticiones pendientes recuperadas; el registro copiado siguió siendo idéntico al original.

`verificacion/AUDITORIA_TECNICA.json` contiene el cotejo externo al ejecutor. Su código Rust y la rúbrica permiten revisar el método. La cadena local no prueba autenticidad externa ni ausencia de supresión de un sufijo entero. Los hashes identifican los materiales publicados, no la verdad de sus respuestas.

## Construcción desde fuentes

La aplicación y el motor conservan `Cargo.toml` y `Cargo.lock`. Utilice Rust 1.98.0. Para la aplicación: `cargo build --locked --release --bin eio-conversacion` en `fuentes/aplicacion`. Para el motor: `cargo build --locked --release --no-default-features -p mistralrs-cli` en `fuentes/motor`. La compilación puede requerir dependencias del sistema y acceso a los orígenes de Cargo; no se incluyen todas las cachés ni se ha repetido esta construcción en una sede limpia. El registro histórico de construcción permanece en el repositorio.

Las fuentes del motor incluyen la modificación numérica utilizada. El binario distribuido y sus huellas son la referencia material de las inferencias conservadas; una reconstrucción no se declara idéntica bit a bit sin cotejo. El código auxiliar de cotejo se compiló contra las dependencias ya disponibles de la sede; sus primeras incidencias de compilación se corrigieron antes de obtener el resultado conforme.

## Punto de continuidad

La entrega fija la configuración nativa y sus límites. No solicita migraciones, nuevas inferencias, eliminación de archivos ni cambios de Qwen. Quedan separados la eventual vía A/WebAssembly, la portabilidad mediante contenedores y la integración completa del consejo asistido en el Núcleo. Cada continuación necesita una hipótesis y condiciones explícitas propias.
