# Comprobación de admisión y conservación del significado

**Buscador-Semántico - Diferencial · 11 de octubre de 2026.**

## Objeto y alcance

Examinar si logicaffeine-language 0.10.1 admite enunciados ordinarios pertinentes para Medicina y Ciberseguridad antes de estudiar su posible uso como base del comparador. No se comprueban todavía la detección de contradicciones, la traducción ni los documentos completos. No se integra el candidato en el SV.

La lectura previa de documentación y código orienta esta comprobación diagnóstica. No es un ensayo ciego ni una muestra representativa. Se utilizan catorce entradas fijadas antes de ejecutar: dos controles de lógica elemental y doce enunciados sintéticos, seis por dominio. Los enunciados no son recomendaciones clínicas ni normas técnicas, no proceden literalmente de las fuentes originales y no alteran su contenido.

## Condiciones previas

- Versión exacta 0.10.1, opciones predeterminadas desactivadas y dependencias fijadas en Cargo.lock. No activar verificación Z3, servicios de IA ni interfaz de ejecución de programas.
- Examinar las licencias y conservar las de terceros. Estudio local sin despliegue de servicios ni adquisición de licencias.
- Mantener intactos los textos, sin sustitución de términos especializados, vocabularios preparados para los casos ni reescritura en lenguaje controlado.
- Conservar el hash de las entradas, protocolo, referencias y resolución de dependencias en Rust antes de llamar al candidato.

## Procedimiento

Para cada entrada se invoca únicamente la función pública compile, con opciones predeterminadas, en un proceso independiente y con un límite instrumental de cinco segundos. Se conservan texto, resultado íntegro o error íntegro, tiempo y terminación. El límite no convierte una incidencia en incompatibilidad ni en U.

Se repite la ejecución con las mismas entradas para contrastar la estabilidad de las salidas, excluyendo las duraciones de la comparación. Las referencias de significado permanecen separadas y no son argumentos del candidato. La instrumentación y el cotejo se realizan en Rust.

## Criterio previo

Los controles permiten distinguir un problema general de preparación de una limitación ante la prosa especializada. Para continuar hacia una comprobación de contradicciones deben admitirse las doce entradas del dominio y conservarse las propiedades pertinentes descritas en REFERENCIAS-ADMISION.json. Una salida lógica sin error no demuestra por sí sola esa conservación. Una omisión relevante o un rechazo impide dar por superada esta comprobación de acceso al análisis; no demuestra imposibilidad general de la técnica.

No se puntuará al candidato como detector de contradicciones en esta etapa. Un fallo de admisión se contabilizará como tal. No se aplicarán reparaciones sobre los mismos casos para presentarlos después como validación independiente.

## Evidencias y continuación

Se conservarán entradas, referencias, protocolo, fuente del instrumento, resolución de dependencias, fijación previa, salidas, contraste y valoración con límites. Si el acceso al análisis es insuficiente, se documentará el defecto y no se iniciará la integración operativa basada en este candidato.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
