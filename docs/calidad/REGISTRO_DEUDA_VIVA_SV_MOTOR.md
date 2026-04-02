# Registro de deuda viva del frente motor

Este documento enumera la deuda viva efectiva del repositorio `SV-motor` y distingue entre límites estructurales aún abiertos y correcciones ya resueltas.

La versión tabular operativa se encuentra en `REGISTRO_DEUDA_VIVA_SV_MOTOR.csv`.

## Deuda viva actualmente abierta

- **DV-SVM-001** — Integración con el Lenguaje SV: no procede abrir backend soberano ni integración fuerte con el entorno de ejecución.
- **DV-SVM-002** — Sincronización con el estado vigente del Documento 3 del frente NLP.
- **DV-SVM-003** — Validación adicional de los modos opcionales de extracción subordinada. La compuerta equivalente a FT-SV-IA/001 ya existe en código para el carril local, pero los modos opcionales siguen sin validación de entorno ni activación real.

## Correcciones ya resueltas en la versión 0.1.2

- **DV-SVM-004** — Exportación de la API pública mínima del paquete.
- **DV-SVM-005** — Derivación de los recuentos del laboratorio desde los casos efectivamente ejecutados.
- **DV-SVM-006** — Alineación de la lista mínima de publicación con el estado material acreditado.
- **DV-SVM-007** — Declaración formal de dependencias de desarrollo.
- **DV-SVM-008** — Incorporación del caso canónico adicional del frente NLP en el lote JSON del laboratorio mínimo.

## Criterio de lectura

Una corrección no permanece declarada como deuda viva abierta en el mismo estado material que la cierra. Cuando detección y corrección ocurren en un mismo ciclo de trabajo, el registro final debe reflejar el cierre material ya producido.


## DV-SVM-009 — Apertura de la capa generativa del dominio DEV

La capa generativa del dominio DEV no debe abrirse todavía. La presente versión solo declara y verifica la capa evaluadora y su laboratorio mínimo.
