# Integración experimental de Qwen con el Árbitro - Director y su Selector

14 de octubre de 2026. Preparación previa a la ejecución.

Se comprueba el gobierno efectivo de la generación de Qwen3-8B-Q4_K_M mediante el Selector del Árbitro - Director CPU/GPU/TPU/NPU. Se reutilizan el Selector sin modificaciones, el motor SV-Aprender instrumentado y las operaciones públicas LIG/0.1 del núcleo documental. El perfil experimental del Árbitro admite la entrada, fija la política, consume una autorización de ejecución única y coteja la devolución. No se modifica el núcleo ni se constituye autoridad productiva R1.

El diseño canónico del Árbitro se encuentra en SV-motor, modelos-de-ia/model-context-protocol/controlador-pictograma-algoritmo/DISENO.md. La realización histórica de admisión documental es específica de otros ensayos; se reutilizan sus operaciones públicas de ligadura, no sus casos ni sus autorizaciones. Esta integración corresponde a un perfil de inferencia local; no incorpora MCP, documentos profesionales, servicios de red ni evaluación semántica.

## Condiciones fijadas

- Pesos oficiales Qwen/Qwen3-8B-GGUF, Qwen3-8B-Q4_K_M.gguf, revisión 7c41481f57cb95916b40956ab2f0b139b296d974; SHA-256 d98cdcbd03e17ce47681435b5150e34c1417f50b5c0019dd560e4882c5745785.
- Selector SHA-256 2b6cd91da365f15ab314ea69ef6eeec1b12826ddf0de01fcca58191ef5afff05. Núcleo documental 1c6b84238e62e867c4d5c5ab8069c9ae8f67dccc, sin modificaciones.
- CPU de cuatro procesadores lógicos y una Tesla T4 de Kaggle. La segunda T4 se observa separadamente. No se presupone su identidad antes de iniciar la sesión.
- Entrada de continuación textual: «El resultado de sumar dos y dos es». Se utiliza la codificación BPE del archivo oficial, sin plantilla conversacional ni adición de instrucciones. La finalidad es comprobar el recorrido de inferencia; no examinar conocimientos ni asignar aptitud.
- Generación por máximo determinista, hasta dieciséis tokens o fin de secuencia, contexto de 64 posiciones y nueva caché por solicitud. Las emisiones se conservan íntegras, incluidos los marcadores especiales. Una terminación por cota se identifica y no se presenta como respuesta completa.
- Pesos preparados en CPU y GPU. Tres pares de calibración, con orden alterno; la estimación se obtiene de la mediana de tiempos completos de generación y conservación. La preparación se mide aparte. Las estimaciones sólo son válidas en esta sesión y para esta entrada y configuración.
- Ocho solicitudes sucesivas: automática, CPU, GPU, CPU, GPU, automática, CPU, GPU. Cada solicitud tiene identidad única y política fijada por el Árbitro. No son ocho casos científicos independientes ni una célula SV. Esta serie comprueba repetición controlada y liberación de contexto; no constituye saturación concurrente ni validación prolongada.

## Criterios

Se cotejan entrada efectiva, ligaduras, política, decisión, dispositivo y resultado. Los vectores completos de cada salida generada se conservan. Se exige identidad de secuencia generada entre CPU y GPU, coseno al menos 0,99 y error relativo L2 como máximo 0,05 para los vectores correspondientes. Una divergencia detiene el contraste y se conserva; no se corrige la respuesta ni se reintenta automáticamente.

Las funciones de cálculo GPU permanecen definidas en Rust y emitidas como PTX, mediante CUDA Driver API y el controlador NVIDIA. Se mantienen los rechazos de cuBLAS, cuBLASLt y WGPU. La ejecución CPU requiere cero lanzamientos CUDA en su intervalo. La ejecución GPU requiere lanzamientos efectivos, correspondencia con el PTX conservado y observación del proceso propio. La CPU de control se mide también cuando se utiliza GPU.

Antes del uso físico se revalidan identidad, memoria, vigencia y destino. Una indisponibilidad impide ejecutar; no produce sustitución silenciosa. Se comprueban solicitudes desconocidas, entrada alterada, repetición, decisión vencida y recurso retirado. Los controles negativos no generan una nueva inferencia.

Se conserva la entrada original, tokens, salida original, vectores, decisiones, registro del Árbitro, trazas GPU, observación CPU/GPU, incidencias y manifiesto. La recepción posterior reconstruye las decisiones y verifica los archivos en Rust. Después se apaga la sesión y se publican resultados delimitados y código en la carpeta del modelo ya autorizada. La atribución económica se registra aparte; un consumo agregado de cuenta no se presenta como coste exclusivo.

La conformidad acredita exclusivamente esta integración experimental y este perfil CPU/T4. No valida el nodo 1, la aptitud del modelo, TPU/NPU, el motor de traducción ni un servicio productivo general del Árbitro.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).