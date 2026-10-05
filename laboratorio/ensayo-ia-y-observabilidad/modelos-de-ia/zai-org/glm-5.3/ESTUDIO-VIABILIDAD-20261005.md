# Familia GLM de zai-org · estudio de viabilidad para AMD

**Candidatos en estudio · 5 de octubre de 2026 · AMD-CALCULO-RUST-20261005.**

Identidad principal: [GLM-5.3 de zai-org](https://huggingface.co/zai-org/GLM-5.3). Se distingue GLM-5.3-Flash como candidato de arquitectura y resultados propios; no se transfieren las capacidades publicadas del modelo mayor ni se sustituye automáticamente por GLM4.

La evaluación es de viabilidad documental, no un ensayo de respuestas. No se han descargado pesos, instalado o ejecutado estos candidatos ni se les ha adjudicado puntuación, vector o dictamen Apto/No apto. Permanecen como candidatos en estudio; cerrar una búsqueda técnica delimitada no constituye su descarte definitivo.

La [entrega técnica fijada](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/061ffb25f5bded239d50d8ec97e7b73a3b661c6d/amd/estudio-uso-5-10-29-v1/familia-independiente-20261005/INFORME.md), de acceso restringido, sustenta las limitaciones de alojamiento y realización; sus cálculos no se presentan como memoria o velocidad medidas. Los datos administrativos permanecen en la sede privada.

[CubeCL y rust-gpu](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/inferencia/cubecl-evaluacion-20261005/ESTUDIO.md) son componentes de cálculo en evaluación; no son modelos ni proporcionan por sí solos una arquitectura completa. [TT-0020](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0020.md), S39 revisión 39 y Acta 004 §33 reúnen las necesidades comunes. Resolver una operación matricial no acredita el candidato completo.

Continuación: conservar el candidato y sus impedimentos; recibir evidencia nueva de arquitectura completa mantenida, realización Rust admisible y capacidad efectiva antes de concretar una prueba material. No sustituir automáticamente el candidato por otra familia o versión más pequeña. Las pruebas, adversariales, límites y admisión necesitarán un protocolo propio; no se heredan puntuaciones de otros modelos. El Núcleo del SV, semántica V0.2 e IR 0.3 permanecen intactos.

## Variantes y reservas

| Candidato | Hallazgo documental de la entrega | Pendiente |
|---|---|---|
| GLM-5.3 | La menor representación examinada suma 216.715.365.893 bytes; no cabe íntegra en 192 GiB gráficos | Una distribución de memoria y realización completa admisible con coste medido |
| GLM-5.3-Flash | UD-Q3_K_XL: 147.535.921.955 bytes; UD-IQ4_XS: 156.822.111.075 bytes | Encaje de pesos plausible en el escenario estudiado; falta memoria total y realización Rust/AMD contrastada |

Los tamaños proceden del inventario de archivos del estudio, sin descarga de pesos. No equivalen a memoria total de inferencia, exactitud de la cuantización ni velocidad. El caso Flash UD-Q4_K_XL muestra además por qué deben distinguirse GB y GiB: 199.707.321.347 bytes exceden 192 GB, pero no 192 GiB; tampoco ese margen por sí solo demuestra viabilidad.

Las iniciativas Rust examinadas no acreditan simultáneamente arquitectura exacta, cálculo admisible, dispositivo AMD y mantenimiento colectivo suficiente para este despliegue. Es una carencia de evidencia técnica del conjunto, no una declaración de abandono ni incapacidad del modelo.

**Dictamen de aptitud:** no emitido. **Despliegue:** no iniciado. **Descarte definitivo:** no acordado.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
