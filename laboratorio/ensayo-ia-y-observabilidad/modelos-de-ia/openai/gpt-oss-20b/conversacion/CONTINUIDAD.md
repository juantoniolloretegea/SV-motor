# Punto de recuperación de la evaluación conversacional

**Última recepción: 25/09/2026, aplicación 0.2.4 y entrega 0.2.4-beta.1.** Prevalecen el [prospecto](../distribucion/0.2.4-beta.1/FICHA_TECNICA.md), los [resultados de cierre](../distribucion/0.2.4-beta.1/RESULTADOS.md) y las [instrucciones](../distribucion/0.2.4-beta.1/LEAME.md). Los apartados posteriores conservan su fecha histórica; no son órdenes vigentes para reanudar campañas terminadas.

Registro 2026-09-24T15:01:35.167Z. Estado observado al interrumpirse el acceso remoto; no sustituye una inspección posterior.

## Estado material

- Servicio sv-conversacion: fallido al iniciar 0.2.1; motor sv-conversacion-motor: inactivo, MainPID=0. No se dejó inferencia nueva en marcha.
- Banco inicial detenido a las 14:47:56 UTC; servicio anterior detenido a las 14:47:57. Inicio de 0.2.1 a las 14:50:22, fallo a las 14:50:32. Control-02 terminó por conexión rechazada a las 14:51:21.
- Máquina de cálculo: /opt/sv-lab/conversacion-20260924. Se conservan datos, evidencias/banco-01, evidencias/control-01, evidencias/control-02 y evidencias/TOKENIZADOR.json. Archivo CONSERVACION_0_2_0.tar.gz preserva el estado anterior.
- Binario 0.2.0 conservado: SHA-256 39ecff0055c8bbf33ccff5fc71ca93f8c1058230efa88ed8594bde239a76eadd. Binario 0.2.1 fallido: cb0643ea5298a4821e3d8e6230581702ce06712fb15c980401974595caa288aa.
- Tokenizador original: /opt/sv-lab/onecloud-20260924/diagnostico-20260924/tokenizador/tokenizer.json; SHA-256 7c704477f22686ec2a8d7490ed55c799d875d2033eb833070cdd07c949037fcc. No sustituir ni sobrescribir.
- Sources locales del Codespace: /workspaces/conversacion-20260924/src; compilación Rust 1.98.0. No restablecer el repositorio original con cambios del usuario.

## Procedimiento pendiente

1. Resolver por el cauce autorizado el rechazo automático de acceso al Codespace. No recurrir a otra vía para eludirlo. No solicitar contraseñas ni claves privadas.
2. Inspeccionar el vocabulario completo y la evidencia del inspector: nombres que ocupan los IDs especiales canónicos, tokens añadidos y resultado de codificación. Los nombres concretos de posibles entradas de relleno todavía no están verificados.
3. Corregir de forma aislada la correspondencia del tokenizador de la interfaz con la del motor. Si procede derivar un archivo normalizado, conservar el original y registrar transformación y huellas. Exigir preservación de tokens ordinarios, unicidad y coincidencia de IDs especiales; mantener la guardia que detectó el defecto.
4. Compilar, ejecutar pruebas acotadas, comprobar apertura y conservación de expedientes. Realizar la validación en un directorio nuevo. No sobrescribir control-01 ni control-02.
5. Reanudar solo M/L con --continue-context en directorio nuevo; Q01–Q12 ya están terminados. D se ejecutará después. Comprobar los nombres efectivos de unidades y dependencias antes de usar DIALOGO.sh.
6. La ventana automatizada común vence el 24/09/2026 a las 16:38:50 UTC. Una interrupción no la reinicia. Si vence, documentar condiciones no ejecutadas y acordar otra ventana antes de continuar el banco. El plazo inicial de ocho horas de la interfaz no equivale a disponibilidad real.
7. Verificar de nuevo conversación, exportación, cancelación y acceso privado antes de entregar la URL. Hasta entonces, la dirección consignada en RECUPERACION.md es una dirección prevista que respondió en la comprobación anterior, no un servicio disponible.

El rechazo automático afectó a una orden de lectura del vocabulario y del informe del inspector, antes de ejecutarse. Alegó que el origen había sido rechazado previamente. El rechazo anterior observado fue una recarga de chrome-error://chromewebdata/ por protocolo no autorizado. Ambos hechos deben conservarse separados y no justifican reintentos por vías alternativas.

Los cambios de esta recepción son documentales y el código del inspector ya ejecutado; no se declara una nueva corrección probada, despliegue adicional ni cierre de la incidencia.

## Reanudación posterior autorizada · 24/09/2026

El usuario confirmó expresamente la recuperación del acceso y añadió comparación con preguntas originales de Qwen y revisión de observabilidad. El navegador permitió nuevamente el origen; el Codespace se encontró detenido y se reanudó mediante su control normal. No se utilizó una vía alternativa para eludir el rechazo anterior.

Servicio 0.2.2 iniciado a las 15:40:39 UTC y disponible a las 15:40:50. Binario SHA-256 86c6c9abe4247930297d70cc2982cf96914bb6347a0ebf567e1c6fcaff7a0044. Fuentes de corrección dbaf4ed071388710e901b87c36842497893e10fb. Vocabulario base con huecos: se añaden allí los tokens declarados, sin sustituir entradas existentes. 21 pruebas unitarias aprobadas. El observador añade los procesos del cgroup del motor residente.

El control-03 fue rechazado porque el guion reutilizaba un identificador de petición del control antiguo. Se corrigió únicamente ese guion y se instaló un ejecutable auxiliar bin/eio-validacion-022, conservando el servicio desplegado. Control-04 aprobado: sesión inválida, exceso de contexto, delimitadores, idempotencia, cancelación con MainPID=0 y exportación. La incidencia del guion no se atribuye al modelo ni invalida el rechazo correcto del servicio.

Banco M/L: unidad sv-conversacion-contexto, evidencias/contexto-03, iniciado 15:46:20 UTC. Conserva el vencimiento 16:38:50 UTC. M04 recuperó correctamente identificador, cantidad corregida 11 y almacén Norte. La comparación recién solicitada se publicó antes de ejecución en f34a2cbf5d31c1cb7df130202b53baf1915ab4ed. Incluye once preguntas en dos rondas anonimizadas y cuatro consultas de OP-IMM-001-P10@1.0; no se declara una tercera ronda recuperada.

Unidad sv-comparacion-recuperada: ejecuta CONTINUAR_BANCOS.sh. Espera al banco M/L; si termina correctamente ejecuta diálogo-02 dentro del plazo original. Solo tolera la falta de plazo registrada del diálogo como motivo para pasar al banco nuevo; otros fallos suspenden la secuencia. A continuación fija VENTANA_COMPARACION.json con 60 minutos para documental-01 y comparacion-01. Orden documental, R02, R01. Una generación simultánea. RuntimeMaxSec=7200 y MemoryMax=2G para el controlador, sin aumentar recursos contratados. Los plazos pueden dejar condiciones sin ejecutar y deben declararse.

La URL privada volvió a responder y mostró los expedientes conservados. No enviar una prueba manual mientras haya otra generación activa. El servicio no transmite tokens incrementales y rechaza historias que excedan 4096 tokens incluida la reserva. Resultados y cierres de las campañas en curso todavía pendientes de recepción final.

## Recuperación posterior al corte y corrección 0.2.3

M01–M04 y L0256–L3072 completos; resultados evaluados en `RESULTADO_CONTEXTO.md`. D01–D04 completos y correctos; D05–D08 no admitidos por falta de plazo. DOC01–DOC04 completos y conformes. Tres exportaciones Qwen conciliadas: dos rondas distintas y once preguntas; el tercer archivo duplica el primer turno R01 y no añade una ronda.

R02-01 generó una respuesta parcial de 256 tokens. La traza duplicaba 35 959 bytes de datos del motor y excedió la cota individual. R02-02 no fue admitida. El controlador falló y el servicio conservó el resultado; no es fallo semántico del modelo. Originales en `1b58b75`, trazas cerradas en `cbfeb9de8b2bd1837f1f72310ab798e7b2b9277a`. La ventana anterior venció a las 17:26:33 UTC.

La versión 0.2.3 usa resumen numérico y huella en la traza, manteniendo el resultado íntegro en el expediente. 22 pruebas aprobadas; binario `00d5b666c61c8ca22acf899eaefa15208f997b8355f893305490c74ca230ef99`, controlador `f21d2583bb66a38e05cb81ce6bd6e53c9c63e887292325b2edfd60db93aac376`. Fuentes `e668d8e1efa18dd5462e9519eb9c5f73cd4e48a0`; verificación `2a112ac`. Servicio reiniciado con PID 32742 tras parada comprobada, ejecutables anteriores conservados.

Procedimiento de continuación: `CONTINUAR_COMPARACION_02.sh`, unidad prevista `sv-comparacion-continuada`, directorio nuevo `evidencias/comparacion-02`; ventana nueva de 7200 segundos en `evidencias/VENTANA_COMPARACION_02.json`. El controlador recibe `evidencias/comparacion-01/CONSERVACION-0.json`, concilia el historial y retoma R02-02; después R01. Plazo individual 900 segundos, justificado por la latencia medida, reserva 256 tokens sin recortes de historia. Comprobar primero estado y archivos: no lanzar dos veces el controlador ni sobrescribir directorios.

La continuación queda preparada; confirmar en la siguiente inspección el inicio y el plazo efectivo. Si se interrumpe la sesión, los resultados deben recibirse del servidor, sin asumir éxito ni repetir peticiones. El servicio puede seguir observando mientras el Codespace se suspende. El túnel privado puede necesitar recuperación mediante `ACCESO.sh`; no implica reiniciar el motor. Mantener una sola generación y exportar los originales antes de evaluar. Permanecen pendientes la comparación final, actualización de calidad y entrega de URL verificada.

Confirmación posterior: primer guion fallido por ruta sin `src/`, antes de admitir inferencia; corregido y conservado en `evidencias/CONTROLADOR_RUTA_02.log`. Unidad `sv-comparacion-continuada` activa desde 17:55:35 UTC. R02-02 admitida y antecedente conciliado. La ventana mantiene inicio administrativo 17:52:28 y límite 19:52:28 UTC, sin extensión por esta corrección. Servicio 0.2.3 iniciado a las 17:51:04 UTC. URL privada comprobada a las 17:56:21; mostró conexión, observación y generación activa. No confundir la respuesta histórica DISPONIBLE de su expediente con una nueva inferencia de control.

## Cierre del 25/09/2026

Campaña nativa y distribución delimitadas. No quedan inferencias encargadas por este cierre. Se conservan la instalación de OneCloud, los originales y los resultados adversos. La documentación diferencia el banco documental de doce condiciones y la campaña sintética de doce consultas; esta última no acredita aptitud clínica. El cotejo Rust y la restauración documental son conformes dentro de su alcance.

Los servicios mantienen sus límites temporales. La disponibilidad futura necesita una observación nueva; no se promete permanencia indefinida. No se eliminan archivos, no se migra Qwen y no se inicia la vía A/WebAssembly. S39 permanece abierto para las necesidades de integración de la adenda, con esta campaña terminada.

