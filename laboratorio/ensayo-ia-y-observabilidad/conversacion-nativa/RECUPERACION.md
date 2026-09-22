# Acceso y recuperación de la conversación experimental

Esta página se conserva en GitHub y puede consultarse cuando el servicio de conversación no responde. Conviene guardar su dirección entre los marcadores del navegador.

**Estado de esta revisión:** procedimiento y código candidato 0.1.2 preparados; su compilación, ejecución y despliegue están pendientes. La reanudación del Codespace no acredita que el servicio de conversación esté iniciado.

## Secuencia de recuperación

1. Abra [sus Codespaces](https://github.com/codespaces) y seleccione la instancia de `SV-motor`. Si está detenida, reanúdela con su cuenta de GitHub. Complete la autenticación que GitHub solicite. El puerto 3000 debe conservar su visibilidad privada.
2. Compruebe el estado del servicio en la terminal. Después de una parada del entorno, el proceso HTTP necesita un nuevo arranque; conservar su ejecutable y sus expedientes no lo mantiene en ejecución.
3. Para la versión instalada, la orden nativa de arranque es `/workspaces/eio-instalacion-nativa-20260922/target/release/eio-conversacion`. La terminal debe confirmar `EIO_CONVERSACION_LISTA`. La versión candidata 0.1.2 añade exclusión de una segunda instancia, cotejo de identidades y comprobación del registro antes de declarar disponibilidad. No sustituya el ejecutable por el candidato sin compilarlo y verificarlo previamente.
4. En el panel **Puertos**, utilice **Abrir en el navegador** para el puerto 3000. Si conserva una pestaña de una instancia anterior, recárguela para recibir la nueva clave del proceso. Esta clave no sustituye la autenticación personal de GitHub.
5. Compruebe el estado de la petición pendiente antes de enviar otra. El candidato recupera su identificador desde la pestaña y consulta si fue admitida. Si no existe en el registro, ofrece repetir el mismo envío con el mismo identificador. No genera una nueva petición automáticamente.

El borrador depende del almacenamiento de sesión de la pestaña. Cerrar una ventana privada o borrar los datos del navegador puede eliminarlo; el expediente confirmado se conserva en el servidor. Una captura de pantalla con conversaciones previas no demuestra disponibilidad actual.

## Ausencia y parada del entorno

La política de GitHub distingue actividad en el editor o la terminal de la mera existencia de una página abierta. El valor predeterminado documentado es de 30 minutos; el valor efectivo de esta instancia debe comprobarse antes de atribuirle una parada concreta. Esta revisión no modifica el plazo ni introduce actividad ficticia para mantener encendido el entorno.

El indicador de la página informa de la última comprobación del servicio, con texto y color. Ante una interrupción de red, un HTTP 404 o una respuesta ajena al servicio, declara causa no determinada. No puede certificar desde una respuesta ausente que la plataforma esté apagada o que haya caducado una credencial.

El registro `datos/servicio/ciclo.jsonl` del candidato conserva identidad de instancia, arranque disponible, señal de parada recibida y cierre observado. La ausencia de cierre no identifica la causa de una parada abrupta. El proceso detenido no puede observar ni reactivar por sí mismo la plataforma que lo aloja.

## Alcance

La recuperación experimental no constituye autenticación profesional individual, bloqueo por ausencia, control de permisos clínicos ni supervisión autónoma de la infraestructura. Esas obligaciones permanecen en su fase específica. Las generaciones pendientes se recuperan como interrumpidas, conservando la última salida registrada; no se continúan silenciosamente.

Fuentes: [inactividad en Codespaces](https://docs.github.com/en/codespaces/setting-your-user-preferences/setting-your-timeout-period-for-github-codespaces) y [seguridad de Codespaces](https://docs.github.com/en/codespaces/reference/security-in-github-codespaces).
