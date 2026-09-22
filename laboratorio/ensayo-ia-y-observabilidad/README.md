# Ensayo de inteligencia artificial y observabilidad

**Edición documental:** 2.4 · 22 de septiembre de 2026.  
**Producto experimental:** EIO conversación 0.1.3 · Beta.  
**Estado:** inferencia nativa y observación verificadas en ámbitos delimitados; conformidad completa de la vía B pendiente.

## Objeto

El ensayo estudia la ejecución de un modelo auxiliar mediante Rust y Candle, su observación y la conservación verificable de las peticiones y resultados. Pertenece a la investigación lateral (p1+P3)-Bis del Lenguaje SV. Debe proporcionar evidencia sobre las necesidades de contratos, control, semántica y representación intermedia; la disponibilidad de una conversación no constituye por sí sola esa evidencia ni autoriza efectos en el SV.

El [contrato experimental](contrato/README.md) conserva las obligaciones y las fuentes rectoras. La propuesta del modelo, su admisibilidad técnica, la fidelidad del contenido y la autorización de una operación son juicios distintos.

## Versión distribuida

La [entrega EIO conversación 0.1.3-beta.1](https://github.com/juantoniolloretegea/SV-motor/releases/tag/eio-conversacion-v0.1.3-beta.1) identifica el ejecutable, las fuentes, la composición, las licencias, los requisitos y la evidencia disponible. La designación de la entrega no cambia la versión 0.1.3 del ejecutable.

El programa permite crear expedientes y conversaciones, formular preguntas libres, conservar el contexto exacto, consultar sucesos, cancelar una generación y exportar un expediente. La interfaz distingue disponibilidad del servicio y estado de la observación. La conservación local no proporciona disponibilidad permanente ni autenticación profesional individual.

La entrega documenta una configuración experimental. No acredita aptitud clínica, respuesta verdadera por defecto, reproducción determinista ni integración completa de conocimiento autorizado.

## Composición y función

| Componente | Identificación | Función y estado de implementación |
|---|---|---|
| Interfaz web | HTML, CSS y JavaScript incluidos en el ejecutable | Presentación, solicitudes y consulta de estado. No ejecuta la inferencia. |
| Servicio HTTP | Axum 0.8.9, Hyper 1.11.1 y Tokio 1.53.1 | Bibliotecas del proceso servidor; no son tres servicios independientes. |
| Modelo | Qwen3-0.6B, GGUF Q4_K_M | Pesos para inferencia en CPU. No corresponde a Qwen-Max. |
| Motor numérico | Candle, revisión `ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a` | Biblioteca ejecutada en el proceso hijo de inferencia. |
| Supervisión de la conversación | Rust 1.98.0 | Una generación admitida simultáneamente; control temporal, cancelación y observación de RSS del hijo. |
| Conservación | JSONL y huellas SHA-256 encadenadas | Registro sincronizado, reconstrucción y detección de alteraciones; sin sello exterior independiente. |
| Instrumentación | OpenTelemetry Rust 0.31.0 | Trazas con exportación local. Las medidas se conservan como atributos; no hay recolector externo de métricas. |
| Observador Linux | Proceso Rust separado | Muestreo del servicio y descendientes visibles. Cobertura y lecturas incompletas declaradas. |
| egui | No incorporado a esta aplicación | Su utilización futura requiere un circuito gráfico y un contrato propios. |

La instalación de una biblioteca, la existencia de un ejecutable y la actividad de un proceso son hechos diferentes. Los documentos de una versión describen su composición; el estado instantáneo se consulta en el servicio y en sus registros de ejecución.

## Correspondencia con la vía B

La vía B sitúa la inferencia en el anfitrión nativo. La vía A la sitúa en el navegador mediante WebAssembly. Comparten exigencias funcionales, pero sus resultados no son intercambiables.

| Obligación de la vía B | Realización en la conversación 0.1.3 | Límite pendiente |
|---|---|---|
| Interlocución web y frontera HTTP | Implementadas; inferencia y conservación comprobadas mediante ensayos nativos. | Disponibilidad externa dependiente de plataforma y sesión; autenticación profesional pendiente. |
| Inferencia en proceso propio | Implementada con Candle y pesos identificados. | No acredita aislamiento material de todas las operaciones del proceso. |
| Supervisión y terminación | Control del hijo, tiempo y RSS muestreada. | No equivale a la guarda exterior ni a una cuota agregada de grupo de control. |
| Custodia independiente del control | Escritura sincronizada en la aplicación. | La escritura puede compartir el camino de control; separación y pruebas de bloqueo pendientes. |
| Observación contrastable | Trazas y observador Linux; correlación técnica ensayada. | No constituye observación exhaustiva del anfitrión o del tráfico. |
| Fuentes y verificación de la propuesta | [Ensayo DOC-01](resultados/consulta-documental-01/PROTOCOLO.md), independiente de la conversación libre. | Su contrato literal no sustituye el verificador completo de la vía B ni un universo de conocimiento terminado. |
| Guarda exterior y cierre contractual completo | Diseño en las candidatas nativas. | Integración y comprobación de extremo a extremo pendientes. |

Los [diseños NAT02](resultados/preparacion-nativa-02/DISENO.md) y [NAT03](resultados/preparacion-nativa-03/README.md) conservan sus obligaciones. La conversación actual es una realización parcial de la vía B. Una capacidad pendiente no se contabiliza como satisfecha por disponer de una biblioteca o de un diagrama.

## Consulta documental inicial

El universo OP-IMM-001 permite preparar pruebas sobre material identificado antes de disponer de todos los textos de respaldo. DOC-01 utiliza un pasaje efectivamente suministrado y distingue sus referencias bibliográficas del contenido de esas obras. La consulta queda limitada por un contrato comprobable exterior al modelo.

La instrucción de utilizar únicamente determinadas fuentes no elimina el cálculo probabilístico ni garantiza obediencia. La evidencia debe mostrar qué se suministró, qué se pidió, qué produjo el modelo y qué admitió o rechazó el verificador. La comprobación de citas literales tiene un alcance menor que la validación de una explicación libre.

El trabajo siguiente debe recuperar la correspondencia funcional pendiente con la vía B sin exigir, para toda prueba instrumental, que el universo clínico esté terminado. La preparación de un corpus parcial no modifica el conocimiento canónico ni habilita decisiones clínicas. La devolución al encargo original requiere identificar resultados, carencias y necesidades concretas del Lenguaje; no se cierra por la publicación de esta Beta.

## Evidencia y continuidad

- [Aplicación y condiciones de uso](conversacion-nativa/README.md).
- [Observabilidad 0.1.3](conversacion-nativa/verificacion-0.1.3/INFORME.md), con distinción entre el binario de la inferencia sintética inicial y el binario instalado posterior.
- [Comparación de doce casos 0.1.2](conversacion-nativa/verificacion-0.1.2/INFORME.md), conservada como evidencia de aquella versión.
- [Protocolo de consulta documental](resultados/consulta-documental-01/PROTOCOLO.md).
- [Seguimiento S39](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md#s39) y [tiques técnicos](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TIQUES_TECNICOS.csv).
- [Edición documental anterior, 2.3](README_2_3_2026_09_20.md), conservada con su fecha y alcance.

## Licencias

Los avisos del SV y de terceros se identifican en [AVISO_LICENCIAS.json](conversacion-nativa/AVISO_LICENCIAS.json) y en la composición de la entrega. Cada componente conserva sus condiciones; la identificación de una versión no amplía derechos de uso o distribución.
