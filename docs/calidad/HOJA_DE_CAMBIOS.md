## 0.1.5
- se incorpora compuerta ejecutable mínima de custodia estructural del frente motor;
- se añade laboratorio de custodia estructural y su integración en el flujo local de comprobación;
- se actualizan pruebas, deuda viva y documentación de fase.

# Hoja de cambios

## 2026-04-02

- refuerzo del bloque `docs/calidad/` con procedimiento, registros, deuda viva, tablero de bloques y matriz de sincronización;
- materialización del laboratorio mínimo reproducible de la Etapa 1 del núcleo local;
- ampliación de la suite con pruebas de extractores y validación geométrica;
- regularización de dependencias opcionales `bridge` en `pyproject.toml`.

- exportación de una API pública mínima en `src/sv_motor/__init__.py`;
- corrección del laboratorio de la Etapa 1 para derivar los recuentos desde los casos realmente ejecutados;
- incorporación de un caso canónico adicional del frente NLP al lote JSON del laboratorio;
- declaración formal de dependencias de desarrollo y umbral mínimo de cobertura en integración continua;
- saneamiento terminológico en documentos orientados a terceros.


## 2026-04-02 — Corrección exacta 2

- se cierra registralmente DV-SVM-004 a DV-SVM-008 tras contraste con la auditoría externa de la versión 0.1.2;
- se corrige la trazabilidad del registro de deuda viva, evitando mantener abiertas correcciones ya materialmente resueltas;
- se depuran varias expresiones internas o coloquiales en documentos expuestos a terceros.


## 2026-04-03 — Demostración local y protocolo ejecutable

- cierre material de la demostración end-to-end local de Fase 0 mediante `sv-nlp` en modo `direct`;
- incorporación de un módulo ejecutable equivalente a FT-SV-IA/001 para el carril local;
- almacenamiento de salidas trazables y verificables de la demostración;
- incorporación de pruebas específicas para la interfaz de línea de órdenes y para la compuerta de protocolo;
- actualización de versión a 0.1.3.


## 2026-04-03 — Declaración inicial del dominio DEV

- declaración formal inicial del dominio `𝔇_DEV`;
- incorporación de `src/sv_motor/algebra/dev.py` como evaluador algebraico inicial del dominio de desarrollo;
- incorporación de laboratorio mínimo del dominio DEV;
- actualización de API pública, documentación y plan de fases del motor;
- actualización de versión a 0.1.4.
