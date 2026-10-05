# CubeCL y rust-gpu: evaluación de cálculo Rust para AMD

**Expediente AMD-CALCULO-RUST-20261005 · revisión documental 1 · 5 de octubre de 2026.**  
Estado: evaluación documental concluida; realización y recepción experimental pendientes. [TT-0020](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0020.md) · S39 revisión 39 · Acta 004 §33 · RETP-2026-278.

## Objeto y decisión delimitada

Se incorpora **CubeCL como componente de cálculo en evaluación**, con prioridad provisional para estudiar una adaptación limitada a AMD Instinct MI300X. rust-gpu permanece como alternativa. Ambos proyectos presentan mantenimiento observable. Esta preferencia combina adecuación funcional y participación reciente; no constituye recepción de un motor, selección de un modelo ni autorización de infraestructura o inferencia.

El componente actuaría en la realización numérica externa al SV. No interpreta ni decide su terna, criticidad, umbrales o acceso al examen. El Núcleo del SV, la semántica V0.2 y la IR 0.3 permanecen intactos. SPIR-V es una representación intermedia de terceros y no es la IR del SV.

**Candidatos conservados en estudio:** [Kimi K3](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/kimi/kimi-k3/ESTUDIO-VIABILIDAD-20261005.md) y [GLM-5.3, con Flash como variante distinta](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/zai-org/glm-5.3/ESTUDIO-VIABILIDAD-20261005.md). La evaluación actual es documental y de viabilidad; no hay despliegue ni ensayo de respuestas. No se ha acordado su descarte definitivo. El cierre de la búsqueda técnica anterior y los impedimentos actuales no se convierten en un dictamen sobre su aptitud.

## Separación de funciones

| Componente | Función documentada | Alcance pendiente |
|---|---|---|
| CubeCL | Descripción y compilación de operaciones paralelas; ejecución en varios dispositivos | Operación matricial exacta y admisible en MI300X por la vía LLVM |
| CubeK | Operaciones reutilizables de álgebra matricial, atención, reducción y cuantización | Correspondencia con operaciones, precisiones y formatos del modelo elegido |
| Burn | Composición y ejecución de modelos sobre distintas realizaciones de cálculo | Arquitectura completa del candidato, pesos y recepción propia |
| rust-gpu / spirv-std | Compilación de Rust a SPIR-V y operaciones para GPU | Controlador, extensiones y cálculo efectivos en MI300X |
| SPIR-V / Vulkan / ROCm | Especificaciones, entornos y herramientas distintos | Compatibilidad de la combinación concreta; no se transfieren capacidades entre ellos |
| mistral.rs y modelos existentes | Conservan sus realizaciones y expedientes específicos | CubeCL no se incorpora automáticamente a mistral.rs ni a los candidatos |

Fuentes: [CubeCL](https://github.com/tracel-ai/cubecl), [CubeK](https://github.com/tracel-ai/cubek), [Burn](https://github.com/tracel-ai/burn#supported-backends) y [rust-gpu](https://github.com/Rust-GPU/rust-gpu). La tabla AMD de Burn reúne alternativas: no acredita una recepción MI300X ni hace admisible LibTorch bajo la condición exclusiva de Rust. Los nombres de cuantización tampoco prueban compatibilidad con cada formato GGUF.

## Identidades y mantenimiento

| Indicador al corte consultado | CubeCL | rust-gpu |
|---|---|---|
| Revisión examinada | 2a8814843c319ae07b15453ef35dce32fd8b9997 | c49441a4c472098f5b221984f5bea1dbf866ad10 |
| Publicación reciente consultada | [v0.11.0-pre.4](https://github.com/tracel-ai/cubecl/releases/tag/v0.11.0-pre.4), 22/09/2026; preliminar | [v0.10.0](https://github.com/Rust-GPU/rust-gpu/releases/tag/v0.10.0), 01/10/2026 |
| Propuestas integradas entre 05/09 y corte del 05/10 | 90 | 11 |
| Cuentas autoras distintas de esas propuestas | 19 | 2 |

Consulta común: `is:pr is:merged merged:2026-09-05..2026-10-05`, mediante búsqueda pública de GitHub, hasta cien resultados por página. Ambas respuestas consultadas indicaron búsqueda completa. [Registro reproducible](FUENTES.json) conserva consultas, identificadores de propuestas y recuentos. El intervalo termina en el corte de consulta, no al final futuro del día. Se cuentan cuentas autoras, no personas, empresas ni mantenedores. La muestra no mide calidad, estabilidad, seguridad o continuidad futura.

La participación observada es más amplia en CubeCL; rust-gpu también está activo. Su [transición comunitaria de 2024](https://rust-gpu.github.io/blog/transition-announcement/) y publicaciones recientes impiden describirlo como abandonado. Su advertencia de desarrollo temprano sigue siendo relevante aunque una etiqueta no incluya un sufijo preliminar.

## Obstáculo concreto de CubeCL en MI300X

La [selección del compilador HIP](https://github.com/tracel-ai/cubecl/blob/2a8814843c319ae07b15453ef35dce32fd8b9997/crates/cubecl-hip/src/compiler.rs) contempla LLVM y C++. Deben fijarse las opciones y demostrar la vía efectiva; una dependencia puede alterar la selección. HIP no equivale por sí solo a generación C++, y escribir una operación en Rust no prueba toda su realización.

El [filtro de capacidades LLVM](https://github.com/tracel-ai/cubecl/blob/2a8814843c319ae07b15453ef35dce32fd8b9997/crates/cubecl-llvm/src/shared/lowered_features.rs#L96) conserva WMMA de RDNA con operandos f16, excluye MFMA de CDNA y retira BF16. La [tabla AMD](https://github.com/tracel-ai/cubecl/blob/2a8814843c319ae07b15453ef35dce32fd8b9997/crates/cubecl-ir/src/amd.rs) y el [ejecutor](https://github.com/tracel-ai/cubecl/blob/2a8814843c319ae07b15453ef35dce32fd8b9997/crates/cubecl-hip/src/runtime.rs) permiten localizar la consecuencia para gfx942. La lectura estática indica ausencia de esa aceleración matricial especializada por esta vía; no demuestra ausencia de todo cálculo GPU ni permite estimar velocidad.

El enlace con [OCML](https://github.com/tracel-ai/cubecl/blob/2a8814843c319ae07b15453ef35dce32fd8b9997/crates/cubecl-llvm/src/amdgpu/ocml.rs) exige delimitar funciones numéricas, compilador y controlador. No se admite una sustitución silenciosa del cálculo por una biblioteca excluida. Las pruebas de generación de código existentes no sustituyen el contraste numérico independiente en una GPU.

La [guía de instalación](https://burn.dev/books/cubecl/getting-started/installation.html) ejemplifica 0.10.0; la publicación preliminar posterior incorpora cambios AMDGPU/LLVM. Se fijará una revisión concreta antes de compilar. No se mezclan capacidades de la rama principal, etiquetas y manuales como si fueran una distribución única recibida.

## Alternativa rust-gpu y papel de Khronos

rust-gpu 0.10.0 incorpora matrices cooperativas KHR y constantes f16; la [documentación de spirv-std](https://rust-gpu.github.io/rust-gpu/api/spirv_std/cooperative_matrix/index.html) describe las operaciones. Su cadena de compilación está fijada a nightly-2026-07-03, rustc 1.98.0. La [guía de plataformas](https://rust-gpu.github.io/rust-gpu/book/platform-support.html) no acredita por sí sola una generación de GPU concreta. No se ha acreditado aquí recepción MI300X con Vulkan y las extensiones matriciales requeridas; esa falta de evidencia no demuestra imposibilidad.

[Khronos](https://www.khronos.org/spirv/) publicó SPIR-V 1.6 revisión 8 el 10/09/2026. El [apartado de cambios desde revisión 7](https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#_changes_from_version_1_6_revision_7) corresponde a esa revisión 8. La evolución del estándar no prueba su implementación por cada compilador o controlador.

El [artículo de AMD sobre SPIR-V en ROCm](https://rocm.blogs.amd.com/software-tools-optimization/spir-v-rocm/README.html), de 20/07/2026, describe amdgcnspirv, un ejemplo HIP y mediciones en otras generaciones. Acredita trabajo de AMD sobre ese formato; no acredita interoperabilidad directa del SPIR-V de rust-gpu con ROCm ni una ejecución Rust recibida en MI300X.

## Necesidades y condiciones de una prueba posterior

| Necesidad | Justificación | Evidencia de resolución exigible |
|---|---|---|
| N-C01: fijar realización efectiva | Evitar selección implícita de C++ o dependencias excluidas | Versiones, opciones, dependencias, código generado y funciones numéricas identificados |
| N-C02: operación matricial CDNA | El obstáculo localizado es MFMA por LLVM | Multiplicación f16 con acumulación f32 para gfx942, código generado inspeccionado |
| N-C03: exactitud independiente | Compilar y recuperar archivos no demuestra corrección numérica | Referencia escalar Rust independiente, casos y tolerancias fijados antes; errores máximos y casos fallidos conservados |
| N-C04: coste operativo medido | No trasladar velocidades de otros motores o dispositivos | Tiempo de compilación, ejecución y transferencias separados; memoria máxima y calentamiento registrados |
| N-C05: mantenimiento de la adaptación | Evitar un motor completo propio sin cota | Cambio delimitado y revisable, relación con fuentes públicas y límite de ampliación explícito |

La prueba propuesta se limita a una operación representativa: matrices rectangulares, dimensiones no múltiplos del bloque, ceros, signos y magnitudes distintas. La tolerancia deberá contemplar cuantización de entrada y acumulación; se fijará antes de observar resultados. La inspección de código puede preceder al uso de GPU; el contraste numérico y temporal exige el dispositivo efectivo. Una suma vectorial o detectar la GPU no basta.

Antes de ejecutar se deberán fijar cotas de tiempo, memoria y salida y resolver las dependencias de lenguaje. Si requiere reconstruir un compilador, incorporar una arquitectura completa de modelo o ampliar recursos, se documentará el impedimento y se someterá ese alcance a decisión. Esta publicación no inicia esa prueba. Un resultado favorable sólo acreditaría la operación examinada; la inferencia completa, el candidato y su admisión requieren evidencias propias.

## Trazabilidad y conservación

El [expediente AMD de acceso restringido](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/main/amd/estudio-uso-5-10-29-v1/calculo-rust-20261005/ADENDA.md) conserva la relación con el estudio previo y las limitaciones de infraestructura. [TT-0020](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0020.md) registra las necesidades pendientes; S39 revisión 39, Acta 004 §33 y RETP-2026-278 enlazan la incorporación. Los informes y manifiestos anteriores conservan sus cortes; no se reescriben resultados de Qwen ni de Safeguard. El control documental Rust y los cotejos de publicación distinguen integridad de archivos, comprobación estática y futura validación numérica.

Los componentes de terceros conservan sus licencias originales. Este documento no redistribuye su código.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
