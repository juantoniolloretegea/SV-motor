# Lote de entrega y verificación

## Objeto

Este lote reúne una versión pública, trazable y técnicamente verificable del repositorio `SV-motor` adecuada para subida única al repositorio remoto, evitando parches dispersos o remiendos posteriores en la fase actual.

## Qué incluye

- paquete Python local instalable;
- suite de pruebas local ampliada;
- laboratorio mínimo reproducible de la Etapa 1 del núcleo local;
- documentación de gobierno, arquitectura y calidad;
- registros CSV mínimos del frente motor;
- integración documental con las sedes superiores mediante matriz de sincronización.

## Qué no incluye todavía

- backend soberano;
- integración fuerte con el Lenguaje SV;
- cierre material del frente NLP;
- extractores online declarados como parte constitutiva del dictamen.

## Verificación previa de este lote

El lote se entrega tras verificación local del siguiente ciclo mínimo:

1. `PYTHONPATH=src pytest -q`
2. `python laboratorio/etapa_1_nucleo_local/ejecutar_laboratorio_motor_etapa_1.py`
3. `./scripts/run_local_checks.sh`

## Regla de uso

Este lote debe subirse como actualización única, revisarse en el remoto y sólo después utilizarse como base para nuevos desarrollos del frente motor.
