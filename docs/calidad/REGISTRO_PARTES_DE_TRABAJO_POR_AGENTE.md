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

## PTA-SVM-005 · Calidad parcial y suspensión de la integración conversacional

Registro 2026-09-24T15:01:35Z. Unidad de ejecución experimental; VERIFICACION_ACOTADA.

Doce tareas breves con contenido correcto, diez con formato estricto conforme. Adaptación de interfaz, piloto web y exportación comprobados antes de la suspensión. La versión 0.2.1 compila y aprueba 19 pruebas unitarias, pero su guardia de inicio detecta discrepancias del tokenizador. Servicio detenido; conversación larga pendiente. El rechazo automático del acceso al Codespace se registra como impedimento instrumental independiente.

[Informe parcial](../../laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/conversacion/RESULTADO_PARCIAL.md) y [continuidad](../../laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/conversacion/CONTINUIDAD.md). S39 revisión 19; Acta004 §19; RETP-2026-272; PTA-2026-014. Sin compras ni ramas adicionales. TT-0012 conserva su cierre acotado.

## PTA-SVM-006 · Recepción y continuación comparativa

Registro 2026-09-24T17:57:00Z. Unidad de ejecución experimental; VERIFICACION_ACOTADA.

M01–M04 y cuatro condiciones L completas; D01–D04 correctas y cuatro condiciones pendientes por plazo. DOC01–DOC04 conformes frente a cero conformes estrictas en la referencia Qwen, con diferencias de configuración expresamente conservadas. R02-01 produjo respuesta parcial y una pérdida del tramo de cierre de telemetría; R02-02 fue bloqueada antes de admisión. Originales y trazas conservados.

Corrección 0.2.3: resumen numérico y huella en traza; resultado íntegro en expediente. Rust 1.98.0, 22 pruebas aprobadas y huellas cotejadas en despliegue. La corrección de una ruta del controlador no modifica el límite 19:52:28 UTC. Continuación activa desde 17:55:35, con conciliación del antecedente. URL privada conectada. [Incidencia y continuidad](../../laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/conversacion/INCIDENCIA_EXPORTACION.md).

Recepción correlativa prevista: S39 revisión 21, Acta004 §21, RETP-2026-274 y PTA-2026-016. S39 sigue abierto; TT-0012 conserva su cierre material acotado. Pendientes resultados comparativos, evaluación final y comprobación de disponibilidad libre.
