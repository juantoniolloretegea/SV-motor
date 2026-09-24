# Registro de partes de trabajo por agente

Este registro deja constancia sobria de los trabajos realizados por unidades operativas sobre el repositorio `SV-motor`.

La versión tabular operativa está en `REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE.csv`.

## PTA-SVM-002 · Recepción y recuperación OneCloud

- **Fecha de registro:** 2026-09-24T11:06:58Z.
- **Agente:** Agente Watson / W-S39-02; relevo de W-S39 en el seguimiento S39.
- **Base:** VERIFICACION_ACOTADA sobre el corte `f3746b719a3e355d75ea566d3d3abde2b6ea9e0e`, los registros recuperados y la observación remota de las 11:00:05 UTC.
- **Actuación:** conciliación de intentos, recepción de la regresión CPU, recuperación del ejecutable y comprobación de sus huellas y arranque básico.
- **Dictamen:** inferencia completa de la candidata pendiente; no hay certificación integral ni nueva inferencia en esta intervención documental.
- **Evidencia y siguiente acción:** [informe de recuperación](../../laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/recuperacion-onecloud-2026-09-24/INFORME.md); restituir pesos y controlador, comprobar guardas y fijar el protocolo antes de ejecutar.
- **Trazabilidad:** S39 revisión 16; TT-0012; recepción correlativa en Lenguaje mediante RETP-2026-269 y PTA-2026-011. Los partes anteriores conservan su autoría y alcance.

## PTA-SVM-003 · Inferencia y contraste OneCloud

- **Registro:** 2026-09-24T11:44:10Z; Agente Watson / W-S39-02, VERIFICACION_ACOTADA.
- **Actuación:** restitución y cotejo de pesos y auxiliares; guardas Rust 1.98.0 (nueve pruebas, incluida auxiliar); protocolos publicados antes de ejecutar; OC-01 correcto y OC-02 semánticamente adverso; cierres y originales íntegros conservados.
- **Dictamen:** objetivo material de TT-0012 conseguido; continuación S39 con limitaciones de calidad general, rendimiento e integración. Diferencia entre ejecutables demostrada para un caso; no causalidad exclusiva del parche.
- **Evidencia:** [informe y manifiesto](../../laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/onecloud-2026-09-24/RESULTADO.md).
- **Trazabilidad:** recepción canónica prevista S39 rev17, Acta004 §17, RETP-2026-270 y PTA-2026-012. Sin otras inferencias ni modificación doctrinal.

## PTA-SVM-004 · Optimización y ejecución residente

Registro 2026-09-24T12:36:09.219Z. Unidad de ejecución experimental; VERIFICACION_ACOTADA.

Se compila un único motor con referencia secuencial y variante paralela; se conservan tres pruebas numéricas, nueve guardas, Cargo check, diez peticiones en dos sesiones, auditoría instrumental y cierre. Cinco casos aceptados; mediana 94,002 a 18,168 s. Cumplido el criterio de primera fase, no se activa llama.cpp. No hay inferencia activa ni compra de recursos.

[Informe y límites](../../laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/onecloud-2026-09-24/optimizacion/RESULTADO.md). S39 revisión 18; Acta004 §18; RETP-2026-271; PTA-2026-013. Sin certificación general ni modificación doctrinal.
