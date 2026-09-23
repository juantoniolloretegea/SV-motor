# Controlador nativo 0.1.3 · Verificación local y alcance

**23 de septiembre de 2026 · Rust/Cargo/rustdoc 1.98.0.**

Se verificó la versión de referencia que añade reserva explícita y diagnóstico de observación sobre 0.1.1. La ejecución secuencial del banco existente pasó sus 16 registros: 14 pruebas funcionales y dos auxiliares. No son 16 inferencias. Se completó la compilación de producción con `--offline --locked`.

Las fuentes principales y Cargo.lock concuerdan en SHA-256 con las identificadas por el controlador remoto de los intentos 03 y 04. Los ejecutables local y remoto se identifican por separado; no se presume identidad binaria entre entornos.

- [PRUEBAS.log](PRUEBAS.log): salida original del banco.
- [COMPILACION.log](COMPILACION.log): compilación local de producción.
- [SHA256SUMS.txt](SHA256SUMS.txt): fuentes, dependencias fijadas y ejecutable local; rutas relativas a la raíz del crate.
- [Continuación real y variantes](../../resultados/continuacion-2026-09-23/INFORME.md): registros de los ocho intentos, fuentes intermedias y trazas. Ninguno alcanzó una respuesta.

La variante posterior 0.1.6 pasó además una [comprobación focalizada de petición y cierre con servicio sintético](../../resultados/continuacion-2026-09-23/PRUEBA_HTTP_VARIANTE_016.log). Ese tipo de comprobación no prueba compatibilidad de argumentos con el motor real: éste rechazó la opción de contexto de 0.1.4. La referencia publicada conserva 0.1.3 y las variantes permanecen como evidencia de diagnóstico, sin adopción operativa.

Los controles siguen siendo instrumentales: RSS muestreada del hijo, límites virtuales por proceso, custodia en un hilo y cierre del hijo directo. No acreditan cuota agregada, guarda exterior, integración completa de B ni calidad de contenido.

Sistema Vectorial SV · [Aviso y licencias](../AVISO_LICENCIAS.json).
