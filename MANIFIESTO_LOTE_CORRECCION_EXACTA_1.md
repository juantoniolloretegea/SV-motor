# Lote exacto de corrección 1 — SV-motor

## Objeto
Este lote corrige exactamente los defectos y observaciones confirmados tras el contraste del informe de auditoría externa con la versión material recientemente descargada del repositorio.

## Alcance exacto
El lote no abre nuevas fases del frente motor, no introduce capa en línea adicional y no modifica el alcance doctrinal del repositorio. Su finalidad es exclusivamente correctiva y de saneamiento técnico-documental.

## Correcciones incluidas
1. Exportación de una API pública mínima en `src/sv_motor/__init__.py`.
2. Derivación dinámica de los recuentos del laboratorio de Etapa 1.
3. Alineación del checklist mínimo de publicación con el estado material acreditado.
4. Declaración formal de dependencias de desarrollo en `pyproject.toml`.
5. Integración de medición de cobertura con umbral mínimo en el flujo continuo.
6. Incorporación de un caso canónico adicional del frente NLP en el lote JSON del laboratorio.
7. Saneamiento terminológico en documentos orientados a terceros.
8. Actualización de la deuda viva y de la hoja de cambios con los hallazgos confirmados.

## Verificación material previa a la entrega
- Paquete instalable en modo editable.
- Suite de pruebas en verde.
- Cobertura del bloque `sv_motor.algebra` por encima del umbral fijado.
- Laboratorio de Etapa 1 ejecutado con salidas regeneradas.

## Resultado de verificación local
- `38` pruebas superadas.
- Cobertura del bloque `sv_motor.algebra`: `98.83%`.
- Laboratorio de Etapa 1: `APTO`.

## Observación operativa
Este lote está preparado para copiarse sobre el clon local del repositorio y generar un único commit de corrección exacta.

## Mensaje de commit recomendado
`Corrección exacta 1 del frente motor tras auditoría externa`

## Descripción de commit recomendada
`Se corrigen la API pública mínima del paquete, los recuentos del laboratorio de Etapa 1, el checklist mínimo de publicación, las dependencias de desarrollo, la cobertura del flujo continuo, un caso canónico adicional del lote JSON y el lenguaje técnico de varios documentos expuestos a terceros.`
