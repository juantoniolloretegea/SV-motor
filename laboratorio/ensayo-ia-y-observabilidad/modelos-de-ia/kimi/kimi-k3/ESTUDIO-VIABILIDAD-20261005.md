# Kimi K3 · estudio de viabilidad para AMD

**Candidato en estudio · 5 de octubre de 2026 · AMD-CALCULO-RUST-20261005.**

Identidad documental: familia K3 de Moonshot AI; [ficha oficial Kimi-K3](https://huggingface.co/moonshotai/Kimi-K3). Se conserva la selección K3; Kimi Linear es un antecedente distinto y no lo reemplaza.

La evaluación es de viabilidad documental, no un ensayo de respuestas. No se han descargado pesos, instalado o ejecutado estos candidatos ni se les ha adjudicado puntuación, vector o dictamen Apto/No apto. Permanecen como candidatos en estudio; cerrar una búsqueda técnica delimitada no constituye su descarte definitivo.

La [entrega técnica fijada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/061ffb25f5bded239d50d8ec97e7b73a3b661c6d/amd/estudio-uso-5-10-29-v1/familia-independiente-20261005/INFORME.md), de acceso restringido, sustenta las limitaciones de alojamiento y realización; sus cálculos no se presentan como memoria o velocidad medidas. Los datos administrativos permanecen en la sede privada.

[CubeCL y rust-gpu](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/inferencia/cubecl-evaluacion-20261005/ESTUDIO.md) son componentes de cálculo en evaluación; no son modelos ni proporcionan por sí solos una arquitectura completa. [TT-0020](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0020.md), S39 revisión 39 y Acta 004 §33 reúnen las necesidades comunes. Resolver una operación matricial no acredita el candidato completo.

Continuación: conservar el candidato y sus impedimentos; recibir evidencia nueva de arquitectura completa mantenida, realización Rust admisible y capacidad efectiva antes de concretar una prueba material. No sustituir automáticamente el candidato por otra familia o versión más pequeña. Las pruebas, adversariales, límites y admisión necesitarán un protocolo propio; no se heredan puntuaciones de otros modelos. El Núcleo del SV, semántica V0.2 e IR 0.3 permanecen intactos.

## Impedimento específico conservado

La menor representación examinada en la entrega, UD-Q1_0, suma 466.369.456.320 bytes de pesos, antes de estados y temporales. Excede incluso la cota aritmética generosa de 192 GiB gráficos más 240 GiB de RAM del escenario estudiado; esas memorias no forman una memoria unificada. Es una limitación de esa configuración y representación, no una imposibilidad universal ni una evaluación intelectual del modelo.

La imagen de catálogo examinada emplea llama.cpp y ocho GPU; no satisface la condición de cálculo Rust ni acredita disponibilidad efectiva de ese recurso. Una realización admisible debe cubrir arquitectura, estados, expertos, cuantización, tokenizador y fin de respuesta, con contraste numérico independiente. No se supone que CubeCL resuelva esas necesidades.

**Dictamen de aptitud:** no emitido. **Despliegue:** no iniciado. **Descarte definitivo:** no acordado.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
