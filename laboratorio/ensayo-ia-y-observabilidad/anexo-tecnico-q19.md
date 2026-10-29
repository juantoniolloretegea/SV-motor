# Anexo técnico de Q19: soporte, medidas y límites

29 de octubre de 2026 · Revisión documental 0.3 · Anexo del libro de Ensayo y observabilidad.

[Volver a Biblioteca y procedimientos](biblioteca-y-procedimientos.md#soporte-del-escenario-q19).

> La seguridad del software y del hardware requiere un análisis conjunto, sustentado en pruebas verificables y con un alcance y unas limitaciones explícitos.

Este criterio de exposición no afirma seguridad absoluta ni sustituye la recepción de cada capacidad. La comprobación conserva la configuración y la fecha a las que se aplica; los cambios pertinentes requieren revisión.

## Identidad y procedencia

El anexo describe registros conservados de 27–29 de octubre de 2026. No constituye una nueva lectura del equipo ni una acreditación general del fabricante. La ficha de referencia es [SV-PLATAFORMA-X515EA-I7-1165G7-20261027](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/afc618dcd14558b6a37187dc9d0a4b639e87b7b6/docs/hardware-de-ensayo/asus-vivobook-x515ea-core-i7-1165g7/FICHA.md); las mediciones posteriores complementan su corte de 27 de octubre sin reescribirlo. Los originales y sus huellas están identificados en el expediente reservado.

| Elemento | Dato conservado | Alcance |
|---|---|---|
| Plataforma | ASUS VivoBook X515EA/F1500EA | Configuración del ensayo, no catálogo de productos |
| Procesador | Intel Core i7-1165G7; familia 6, modelo 140, revisión 1 | Identificación declarada por las interfaces observadas |
| Topología lógica | Un paquete, cuatro núcleos, ocho procesadores lógicos | El monitor de sucesos de Intel (EMON) relaciona 0–1, 2–3, 4–5 y 6–7 como parejas por núcleo |
| Memoria de acceso aleatorio (RAM) | Aproximadamente 16 GB | Capacidad; la memoria libre varía durante la ejecución |
| Sistema y protecciones | Windows 11; Hyper-V declarado, seguridad basada en virtualización activa | No se desactivaron estas protecciones para obtener contadores |
| Firmware conservado | X515EA.310 en las comprobaciones del 28/10 | Identificación observada; no se atribuye otra revisión de firmware |
| Gráficos | Intel Iris Xe integrada | Sin atribuirle inferencia ni una prueba de la unidad de procesamiento gráfico (GPU) en esta campaña |
| Unidad auxiliar | Almacenamiento de trabajo conectado por bus serie universal (USB) | No separa la unidad central de procesamiento (CPU), memoria física o autoridad del anfitrión |

La compilación exacta de Windows, revisión de microcódigo, distribución completa de memoria, configuración de frecuencia y demás campos no fijados en este anexo deben incorporarse al manifiesto del intento antes de recibir el soporte. No se deducen del modelo comercial ni se rellenan con valores supuestos.

## Geometría declarada de caché

Fuente: instrucción de identificación del procesador (CPUID) mediante raw-cpuid 11.6.0, expediente 027. Las cifras describen la geometría comunicada al proceso de lectura; no prueban latencia, residencia exclusiva ni una exploración exhaustiva de todos los procesadores lógicos.

| Nivel y tipo | Capacidad | Línea | Vías | Conjuntos |
|---|---:|---:|---:|---:|
| L1 de datos | 49.152 bytes (48 KiB) | 64 bytes | 12 | 64 |
| L1 de instrucciones | 32.768 bytes (32 KiB) | 64 bytes | 8 | 64 |
| L2 unificada | 1.310.720 bytes (1,25 MiB) | 64 bytes | 20 | 1.024 |
| L3 unificada | 12.582.912 bytes (12 MiB) | 64 bytes | 12 | 16.384 |

La instrucción de identificación del procesador (CPUID) declara una unidad de monitorización del rendimiento (PMU) de versión 5: ocho contadores programables y cuatro de función fija, de 48 bits. Esa capacidad no demuestra que estén disponibles en exclusiva, programados continuamente o correctamente contabilizados durante todo el ensayo.

## Instrumentación y magnitudes

| Medio | Datos disponibles | Límite recibido |
|---|---|---|
| LibreHardwareMonitor 0.9.6 / PawnIO y observador Rust | Sensores expuestos, vigencia, secuencia y huellas | No cubre todos los sensores ni demuestra observación eléctrica de buses |
| Intel VTune 2026.6 | Muestras atribuibles al proceso y a seis funciones propias | Muestreo estadístico; tiempos efectivos, pérdidas y efecto de multiplexación pendientes |
| Monitor de sucesos de Intel (EMON) | Controles pasivos de lectura, pausa y reanudación; 14 lecturas completas y 336 valores | Recuento agregado del sistema en modo usuario; no atribuible exclusivamente a una carga |
| Seguimiento de eventos de Windows (ETW) | Capturas de proveedores seleccionados | Existencia de captura no equivale a decodificación o cobertura completa |
| Observación y parada en Rust | Retirada de grupos propios y evidencias de finalización | No acredita aislamiento frente a código hostil ni una garantía de tiempo real |

Eventos solicitados: MEM_LOAD_RETIRED.L2_HIT, MEM_LOAD_RETIRED.L3_HIT y MEM_LOAD_RETIRED.L3_MISS. Sus nombres identifican eventos del soporte observado; no son códigos portables a cualquier procesador. Se distinguen muestras, recuentos e inferencias del instrumento.

La campaña sintética utilizó un hilo de carga, seis recorridos de unos cuatro segundos y tamaños de 256 KiB, 4 MiB y 32 MiB, con dos repeticiones por tamaño. Los 24 segundos activos previstos, los 34,872 segundos exteriores del proceso y los 23,601 segundos de tiempo de la unidad central de procesamiento (CPU) comunicados por el instrumento son magnitudes diferentes. Ninguna sustituye el tiempo efectivo de habilitación de cada contador.

| Patrón | Muestras L2_HIT | Muestras L3_HIT | Muestras L3_MISS |
|---|---:|---:|---:|
| Pequeño A, 256 KiB | 9.139 | 8 | 4 |
| Intermedio A, 4 MiB | 687 | 1.895 | 40 |
| Mayor A, 32 MiB | 6 | 26 | 308 |
| Mayor B, 32 MiB | 4 | 20 | 302 |
| Intermedio B, 4 MiB | 674 | 1.864 | 57 |
| Pequeño B, 256 KiB | 11.021 | 4 | 0 |

Los tamaños son hipótesis de contraste, no una demostración de residencia exclusiva en L2, L3 o memoria principal. Los fondos escasos no cumplen todas las condiciones previas de aceptación. Las sondas pasivas posteriores no subsanan retrospectivamente esa carencia.

<a id="plataformas-heterogéneas-cpu-y-aceleradores"></a>

## Plataformas heterogéneas: unidad central de procesamiento (CPU) y aceleradores

El alcance metodológico contempla también unidades de procesamiento gráfico (GPU), unidades de procesamiento tensorial (TPU), unidades de procesamiento neuronal (NPU) y otros aceleradores cuando intervengan en el escenario. La clasificación funcional no sustituye su arquitectura: una unidad de procesamiento gráfico (GPU) también es un procesador y las denominaciones comerciales no permiten inferir una organización común de memoria o seguridad. Pueden ser pertinentes, por ejemplo, procesadores de señales (DSP), dispositivos lógicos reconfigurables (FPGA), circuitos de aplicación específica (ASIC) o unidades de procesamiento de datos (DPU); su inclusión se justifica por su intervención efectiva.

La unidad de descripción es la **plataforma de ejecución**: procesadores, aceleradores, memorias, interconexiones, firmware, controladores y programas que coordinan el trabajo. Para cada caso se identifican los componentes presentes, los realmente utilizados, los observados y aquellos sobre los que existe facultad de intervención. Estas cuatro condiciones no son equivalentes.

| Clase | Propiedades que deben delimitarse cuando intervenga | Estado de este expediente |
|---|---|---|
| unidad central de procesamiento (CPU) | Topología, cachés, memoria, hilos, contadores y permisos | Mediciones parciales del i7-1165G7; recepción integral pendiente |
| unidad de procesamiento gráfico (GPU) integrada o discreta | Contextos y colas, memoria propia o compartida, transferencias, sincronización, dispositivos y procesos asociados | Iris Xe identificada; sin campaña sobre la unidad de procesamiento gráfico (GPU) ni recepción de esas propiedades |
| unidad de procesamiento tensorial (TPU) | Generación, partición o conjunto asignado, memorias, operaciones, comunicación con el anfitrión y entre dispositivos cuando corresponda | Sin soporte recibido de unidad de procesamiento tensorial (TPU) ni prueba ejecutada |
| unidad de procesamiento neuronal (NPU) | Modelo concreto, controlador, compilación y ejecución, memoria y operaciones expuestas por sus herramientas | Sin soporte recibido de unidad de procesamiento neuronal (NPU) ni prueba ejecutada |
| Otros aceleradores | Función efectiva, interfaces, memoria, acceso y condiciones de observación y parada | Evaluación de pertinencia pendiente; sin presumir presencia |

El análisis propuesto debe cubrir también las relaciones: quién entrega trabajo al dispositivo, qué datos transfiere, qué memoria comparte, cómo se establece el orden de las operaciones y qué sucede al retirar un permiso o detener el proceso coordinador. Detener el proceso de la unidad central de procesamiento (CPU) no se tomará como prueba de que hayan terminado todas las operaciones del acelerador; esa condición necesita evidencia propia.

Cuando sean pertinentes se examinarán acceso directo a memoria (DMA), traducción y protección de direcciones, aislamiento entre contextos, datos residuales y recursos compartidos. Son objetos de comprobación, no vulnerabilidades demostradas. La ficha señalará qué puede observarse, qué queda oculto por el controlador o proveedor y qué efectos produciría una parada o reinicialización sobre trabajo legítimo concurrente.

**No todas las cachés son el mismo objeto.** Deben separarse la caché física del procesador, la memoria del acelerador, la caché de compilación y la reutilización de resultados o estados de una aplicación o modelo. Compartir el nombre «caché» no demuestra el mismo fenómeno ni permite aplicar el corrector de Q19 sin adaptación justificada.

Como referencias de viabilidad instrumental, [NVIDIA CUPTI](https://docs.nvidia.com/cupti/main/main.html) documenta correlación entre la actividad de la unidad central de procesamiento (CPU) y la unidad de procesamiento gráfico (GPU) y límites de entrega de registros; [AMD ROCprofiler](https://rocm.docs.amd.com/projects/rocprofiler-sdk/en/docs-7.14.0/how-to/using-rocprofv3.html) distingue llamadas, ejecución y transferencias; [Google XProf para unidad de procesamiento tensorial (TPU)](https://docs.cloud.google.com/tpu/docs/jax-ai-stack?hl=en) describe observación de operaciones y memoria; y [OpenVINO unidad de procesamiento neuronal (NPU)](https://docs.openvino.ai/2024/openvino-workflow/running-inference/inference-devices-and-modes/npu-device.html) distingue dispositivo, controlador y caché de compilación. Son referencias documentales con versiones propias, no herramientas acreditadas en esta campaña ni una recomendación de instalación indiscriminada.

Para comparar plataformas se mantiene el objetivo científico, pero se recibe un método específico de observación y contención en cada configuración. No se igualan códigos de la unidad de monitorización del rendimiento (PMU), latencias, unidades de ejecución o condiciones de aislamiento por semejanza de denominación. Si una propiedad necesaria no puede observarse, se declara la limitación correspondiente a ese caso.

Esta ampliación describe la cobertura que debe estudiarse. No añade parámetros a la célula del Sistema Vectorial (SV), de dimensión (25,5), no obliga a adquirir todos los tipos de dispositivo y no amplía por sí sola el perímetro de ejecución. Una campaña sobre aceleradores necesitará soporte pertinente, método, controles, recuperación y autorización aplicables. El hardware de inferencia del proveedor remoto sigue fuera de la observación local salvo evidencia específica de acceso y medición.

## Propiedades pendientes y condición de extrapolación

La etapa 2 conserva recepción parcial: faltan tiempo efectivo por evento, integridad continua pertinente de la programación de la unidad de monitorización del rendimiento (PMU), información suficiente sobre pérdidas y recepción de sensibilidad y controles según el contrato. Se observó avance de la marca temporal Intel durante pausas con recuentos invariantes; no se acepta esa marca como tiempo de habilitación. El monitor de sucesos de Intel (EMON) tampoco recibió cierre ordinario satisfactorio en las revisiones 045 y 046, aunque se retiraron sus procesos propios.

Estas limitaciones pertenecen al soporte. No constituyen 1 ni U del candidato. Tampoco acreditan ausencia del fenómeno que Q19 pretende estudiar. El escenario físico debe demostrar su objetivo, contención y conservación del trabajo legítimo mediante un método recibido antes de calificar.

| Nuevo soporte | Reutilización posible | Comprobación necesaria |
|---|---|---|
| Otro ejemplar aparentemente igual | Procedimiento y programas compatibles, tras verificar dependencias | Identidad, firmware, sistema, permisos, observables y recepción propia |
| Otro Intel / x86 | Diseño del contraste cuando conserve significado | Eventos y erratas de esa microarquitectura, topología, virtualización y calibración |
| AMD / x86 | Objetivo científico y parte del procedimiento | Contadores, semántica, herramientas y condiciones propios; sin reutilizar códigos Intel por analogía |
| ARM, incluido un soporte Apple pertinente | Objetivo científico y criterios funcionales | Instrumentación y controles disponibles en ese sistema y modelo; adaptación y recepción propias |
| Máquina virtual o emulador | Comprobaciones de las propiedades que realmente exponga | Distinguir tiempo virtual, señales del anfitrión y fenómeno físico; no sustituir uno por otro |

La evidencia se aplica a una configuración y una finalidad. Una arquitectura común no acredita condiciones equivalentes, y el ensayo tampoco pretende agotar todos los procesadores existentes. La observación de este equipo no permite describir la infraestructura donde un proveedor ejecuta el modelo remoto.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
