# EIO-NAT-PREP-01 · candidata nativa observable

Preparación de fuentes, 20/09/2026. **No compilada, no ejecutada, sin infraestructura habilitada.** No representa un servicio operativo, una solución acreditada del pico de memoria ni aceptación científica.

- [Diseño y correspondencia](DISENO.md).
- [Contrato experimental EIO-NAT/1](CONTRATO.md).
- [Oráculos y órdenes futuras](pruebas/ORACULOS.md).
- [Dependencias y habilitación](HABILITACION.md).
- [Matriz y brechas](MATRIZ.md).
- [Manifiesto de esta preparación](MANIFIESTO.json).

El servidor web Rust comunica con un supervisor/custodio exterior al inferidor mediante socket Unix. El inferidor es un proceso nativo Candle independiente, sin herramientas ni cliente HTTP. Se conservan las dos peticiones sintéticas fijadas y el verificador JSON. La página sólo presenta datos y controles; no ejecuta inferencia WASM.

Configuración publicada inactiva. No existe nuevo Cargo.lock resuelto, binario, imagen de contenedor fijada ni URL de servicio. El lock anterior se conserva sólo en antecedentes/Cargo.lock. Ninguna modificación de workflows, guardas, Cargo.toml raíz o configuración activa de Codespaces.

El uso posterior requiere recepción, lock nuevo, compilación y pruebas instrumentales independientes, saldo real de cuota y barrera efectiva de gasto adicional cero. La preparación no autoriza esas operaciones. S37/S38 permanecen pendientes; no se modifican registros canónicos.
