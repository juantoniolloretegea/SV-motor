# Punto de recuperación de la evaluación conversacional

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
