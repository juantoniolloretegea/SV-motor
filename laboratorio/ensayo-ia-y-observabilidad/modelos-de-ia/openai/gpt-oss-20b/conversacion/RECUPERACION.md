# Acceso y recuperación del servicio de conversación

**Estado actualizado el 24/09/2026: servicio 0.2.2 activo y acceso privado comprobado. Banco automatizado en curso; una sola generación simultánea. La indisponibilidad del corte anterior quedó resuelta. Véase [continuidad](CONTINUIDAD.md).**

URL privada: https://didactic-chainsaw-p49vp5w7qg62r7j7-3000.app.github.dev/

Requiere la cuenta de GitHub autorizada, el Codespace `didactic-chainsaw-p49vp5w7qg62r7j7` iniciado y el túnel SSH activo. Los expedientes se conservan en la máquina de cálculo, independientemente del navegador. La interfaz no es una publicación anónima.

El servicio tiene una duración inicial máxima de ocho horas. Si el Codespace se detiene, la URL deja de responder aunque los datos y la máquina de cálculo sigan disponibles. La detención del Codespace no constituye evidencia de fallo del modelo.

Para iniciar o recuperar el servicio desde el Codespace autorizado:

```bash
bash /workspaces/conversacion-20260924/ACCESO.sh
```

El guion comprueba la clave del servidor, inicia el servicio si está detenido y mantiene el túnel local. No copia claves privadas, no altera los expedientes y no envía de nuevo una petición sin confirmación. La clave de sesión HTTP cambia cuando se reinicia el servicio: recargue la página. El texto pendiente se conserva en la pestaña y su identificador se concilia con el registro.

Cada petición dispone de un máximo de 900 segundos; la respuesta predeterminada reserva 256 tokens y 600 segundos. La conversación completa, junto con esa reserva, debe caber en 4096 tokens. Cuando no cabe, se rechaza sin eliminar antecedentes. Puede exportar el expediente y abrir otra conversación expresamente.

La primera petición tras iniciar o cancelar el motor incluye su carga. El servicio muestra la respuesta completa cuando termina: no ofrece medición ni transmisión del primer token. La cancelación detiene el motor residente y confirma su parada antes de admitir otra generación.

Los expedientes tienen una cota lógica de 512 MiB; las trazas auxiliares tienen sus propios límites. No se borran expedientes automáticamente. Exportarlos es recomendable antes de retirar la máquina contratada. El límite temporal del servicio no cancela ni destruye la máquina del proveedor.

Código y resultados se publican en `main`; los expedientes que cree el usuario no se publican automáticamente. Los registros encadenados permiten comprobar consistencia local y no acreditan por sí solos la exactitud de las respuestas.
