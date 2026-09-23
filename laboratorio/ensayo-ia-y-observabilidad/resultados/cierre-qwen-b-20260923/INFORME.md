# Cierre delimitado de la campaña Qwen/B
Fecha: 23 de septiembre de 2026.

**Resultado: campaña concluida como realización parcial con limitaciones identificadas. No se acredita conformidad integral de la vía B.**

## Decisión y objeto
Se da por terminado el trabajo de esta campaña sobre Qwen3-0.6B Q4_K_M, Candle y conversación nativa. Se conservan la instalación, la distribución 0.1.3 · Beta 1 y todas las evidencias. No se reactivan los controles administrativos interrumpidos ni se condiciona este cierre a completar todas las combinaciones de modelo y soporte.

El seguimiento general S39 continúa para la investigación EIO, gpt-oss y la devolución de resultados a la adenda y a (p1+P3)-Bis. Una futura campaña Qwen tendrá objeto y condiciones nuevos, enlazados con este antecedente. Esta decisión no declara imposible la vía A ni descarta todas las realizaciones del modelo.

## Resultados y carencias
| Aspecto | Evidencia conservada | Conclusión de cierre |
| --- | --- | --- |
| Instalación y conversación | Pesos y tokenizador identificados; inferencias, cancelación y reconstrucción documentadas | Funcionamiento en los casos ensayados; no integración integral. |
| Rendimiento | Doce generaciones de 0.1.2; contexto sin antecedentes 15,79–28,92 s y con antecedentes 132,17–152,47 s; carga 1,21–2,42 s | Limitación de latencia aceptada para concluir esta campaña. No se ha acreditado una mejora de velocidad ni un contexto práctico de 16 384 tokens. |
| Fidelidad | Definiciones incorrectas, restas correctas y conservación de entidad con incumplimiento del formato | La configuración no queda aceptada para las operaciones documentales profesionales examinadas; no se determina una causa única entre modelo, cuantización y motor. |
| DOC-01 | Cuatro peticiones completas; cero aceptaciones contractuales | Ensayo terminado con resultado adverso. Las correcciones del servicio no cambian ese resultado. |
| Licencias y acceso | Avisos incorporados, exportaciones instrumentadas y comprobaciones HTTP locales | La comprobación visual externa y la continuidad de navegador no quedan acreditadas en esta recepción. |
| Guarda exterior y contención agregada | CAPACIDAD-CGROUP-01 y antecedentes de controles NAT03 | Delegación exclusiva y control superior no acreditados. Se cierra con limitación; no se ordenan nuevas gestiones administrativas. |
| Custodia y observación | Cadenas locales, OpenTelemetry y observador Linux con ámbitos declarados | Persisten la falta de anclaje externo, de custodia independiente y de observación completa durante la generación. |
| Corrección reutilizable | Fuentes candidatas 0.1.4, compilación y banco local | Mejora concreta de supervisión y recuperación; sin despliegue ni nueva inferencia. |

## Recepción de la revisión externa
La recepción distingue el corte auditado de la corrección posterior del controlador. La intervención administrativa sí figuraba en el encargo histórico; esa autorización no se convierte en una tarea actual. La guarda instrumental incorporaba registros durante la ejecución. La clave de sesión no autentica personas, pero ese hecho no demuestra exposición pública del puerto. Los originales de la auditoría permanecen intactos.

Las correcciones publicadas son una respuesta técnica con verificación local propia. No constituyen una conformidad externa nueva ni eliminan las reservas que exceden su alcance.

## Continuidad técnica
Los tiques TT-0002, TT-0003, TT-0004 y TT-0006 terminan en el alcance parcial declarado: latencia, fidelidad y verificación externa conservan sus limitaciones. TT-0009 conserva el cierre de su inspección inicial. El tratamiento local de supervisión y recursos se identifica separadamente en TT-0011.

El controlador gpt-oss 0.1.1 queda compilado y probado con auxiliares Rust. El modelo sigue sin respuesta obtenida; el emisor de su SIGTERM anterior continúa sin identificar. La instalación futura en PC/WSL2 se mantiene pendiente en S42/TT-0010. La recepción de la auditoría y la corrección local no acreditan por sí solas capacidad suficiente en ese destino.

## Evidencias
- [Comparación 0.1.2](../../conversacion-nativa/verificacion-0.1.2/INFORME.md).
- [Observabilidad 0.1.3 y atribución de binarios](../../conversacion-nativa/verificacion-0.1.3/INFORME.md).
- [DOC-01](../consulta-documental-01/INFORME.md).
- [CAPACIDAD-CGROUP-01](../capacidad-cgroup-01/README.md).
- [Verificación local 0.1.4](../../conversacion-nativa/verificacion-0.1.4/INFORME.md).
- [Controlador gpt-oss 0.1.1](../../modelos-de-ia/openai/gpt-oss-20b/controlador-nativo/verificacion-0.1.1/INFORME.md).
- [Registro canónico y tiques](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TIQUES_TECNICOS.csv).

Sistema Vectorial SV · [Licencias del ensayo](../../README.md#licencias).
