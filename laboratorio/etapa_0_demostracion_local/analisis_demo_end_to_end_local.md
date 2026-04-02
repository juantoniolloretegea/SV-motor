# Análisis de la demostración end-to-end local

La demostración materializa el cierre verificable de la Fase 0 del frente motor.

## Caso DEMO-APTO

El comando en modo directo recibe un paquete Ω_NLP válido, ejecuta la cadena completa del agente y produce clasificación `APTO`, política `CERRAR_FRAME` y ausencia de bloque de estado adicional.

## Caso DEMO-UDB

El comando en modo directo recibe un valor fuera de dominio en `pi`. La validación no fabrica certeza: normaliza el campo a `indeterminada`, declara `U_d(B)` en el bloque de estado y mantiene la trazabilidad completa de la salida.

## Lectura

La Fase 0 queda demostrada como sistema local, repetible y auditable. Esta demostración no abre todavía extractores opcionales de entorno local o en línea, no sustituye la futura sincronización con el Documento 3 vigente y no equivale a motor completo.
