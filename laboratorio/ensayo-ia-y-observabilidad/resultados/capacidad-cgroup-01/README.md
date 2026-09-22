# Comprobación inicial de capacidad de control · CAPACIDAD-CGROUP-01

**Fecha de ejecución:** 22 de septiembre de 2026, 19:04:03 UTC.  
**Estado:** comprobación finalizada; capacidad requerida no acreditada.  
**Vinculación:** S39 · TT-0009.  
**Corte de referencia:** SV-motor `a14ea31b3903d49a98f08b912206b1c8c9eeaf74`, rama `main`.

## Objeto y método

Determinar si una inspección mínima de la vista cgroup v2 del contenedor permite acreditar el requisito exterior de [NAT03](../preparacion-nativa-03/DISENO.md#guarda-exterior-preparada-inactiva), antes de iniciar pruebas de bloqueo. NAT03 exige una hoja exclusiva y delegada, con una guarda situada fuera de ella; requiere además control superior para fallos que la propia guarda no puede resolver.

Se compiló [comprobar.rs](comprobar.rs) con `rustc 1.98.0 (88d9e12ae 2026-08-18)` mediante la biblioteca estándar.

El programa leyó `/proc/self/mountinfo`, con un límite de 128 KiB, e intentó abrir tres archivos de control para escritura. Cerró cada descriptor sin escribir. No creó grupos, migró procesos, modificó permisos ni ejecutó inferencias. Se conserva [RESULTADO.json](RESULTADO.json), transcrito de la salida de terminal con formato legible y los mismos valores.

## Resultado

| Observación | Resultado |
|---|---|
| Montajes cgroup v2 visibles | Uno. |
| Montaje raíz identificado como de solo lectura | No. |
| Apertura de `/sys/fs/cgroup/cgroup.procs` sin escritura | Permitida. |
| Apertura de `/sys/fs/cgroup/cgroup.kill` sin escritura | Denegada: error de permisos 13. |
| Apertura de `/sys/fs/cgroup/cgroup.subtree_control` sin escritura | Permitida. |
| Escrituras de control, grupos creados e inferencias | Cero. |

No se identificó una hoja exclusiva asignada al ensayo ni se acreditó control superior de plataforma. El rechazo en el grupo raíz examinado **no demuestra** que una hoja delegada distinta sea imposible. Las aperturas permitidas tampoco prueban que una escritura futura vaya a aceptarse ni que una guarda funcione.

## Resolución y límite

No se habilitan pruebas de bloqueo con estos datos. Una continuación necesitaría identificar y acreditar la delegación concreta y su control exterior; si no están disponibles dentro del alcance y recursos autorizados, se registra esa limitación sin forzar privilegios.

El Codespace estaba detenido antes de la inspección; se inició para esta comprobación y se restituyó al estado detenido después. No se activó gasto adicional ni se cambió el presupuesto de la cuenta. La observación no determina las causas de incidencias anteriores de acceso ni certifica la actividad de todos los servicios tras el arranque.

La comprobación se cierra con evidencia insuficiente para habilitar la guarda. No cierra NAT03 ni acredita conformidad integral de la vía B.
