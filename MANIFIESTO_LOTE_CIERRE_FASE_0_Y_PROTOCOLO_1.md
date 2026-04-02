# Manifiesto del lote — cierre de Fase 0 y protocolo ejecutable 1

## Objeto

Este lote incorpora, en un único empuje, los dos pasos inmediatos del frente motor:

1. cierre material de la demostración end-to-end local de Fase 0;
2. incorporación en código de la compuerta equivalente a FT-SV-IA/001 para el carril local, previa a cualquier activación real de modos opcionales.

## Cambios incluidos

- interfaz de línea de órdenes mínima `sv-nlp` en modo `direct`;
- módulo `src/sv_motor/protocols/ft_sv_ia.py`;
- demostración reproducible en `laboratorio/etapa_0_demostracion_local/`;
- actualización de versión a `0.1.3`;
- ampliación del flujo local y del flujo continuo para ejecutar la demostración;
- pruebas nuevas para CLI y protocolo;
- actualización de documentación y de deuda viva para reflejar el nuevo estado.

## Verificación material realizada antes de la entrega

- `44` pruebas superadas;
- laboratorio de Etapa 1 ejecutado correctamente;
- demostración end-to-end local ejecutada en dos casos (`DEMO-APTO`, `DEMO-UDB`);
- `sv-nlp` instalado como comando real del entorno.

## Lo que este lote no hace

- no activa extractores opcionales de entorno local o en línea;
- no sincroniza todavía con el estado vigente completo del Documento 3 NLP;
- no abre backend soberano ni integración fuerte con el lenguaje;
- no incorpora generador SVG paramétrico.

## Commit recomendado

**Cierre material de Fase 0 y compuerta ejecutable FT-SV-IA/001**

## Descripción recomendada

**Se incorpora la demostración end-to-end local de Fase 0, la interfaz mínima `sv-nlp` en modo `direct`, el módulo ejecutable equivalente a FT-SV-IA/001 para el carril local y las actualizaciones de documentación, deuda viva y pruebas asociadas.**
