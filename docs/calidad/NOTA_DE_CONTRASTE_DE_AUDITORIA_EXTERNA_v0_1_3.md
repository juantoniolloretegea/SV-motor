# Nota de contraste de auditoría externa — v0.1.3

## Objeto

Dejar constancia de que la versión 0.1.3 resuelve dos puntos abiertos del informe consolidado del lote anterior:

1. la ausencia de una demostración end-to-end local y trazable de Fase 0;
2. la ausencia de una compuerta equivalente a FT-SV-IA/001 en código antes de cualquier activación real de modos opcionales.

## Resultado

La versión 0.1.3 incorpora:
- `sv-nlp` como interfaz mínima de terminal en modo `direct`;
- `src/sv_motor/protocols/ft_sv_ia.py` como compuerta ejecutable del carril local;
- y un lote de demostración reproducible en `laboratorio/etapa_0_demostracion_local/`.

## Límites que permanecen abiertos

- La sincronización material con el estado vigente del Documento 3 del frente NLP sigue abierta.
- Los modos opcionales de extracción continúan sin activación real ni validación completa de entorno.
- El generador SVG paramétrico no forma parte de esta corrección.
