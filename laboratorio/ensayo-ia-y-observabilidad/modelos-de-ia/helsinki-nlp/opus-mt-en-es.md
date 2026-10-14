# Helsinki-NLP/opus-mt-en-es: contraste del Selector CPU/TPU

14 de octubre de 2026.

## Resultado y alcance

El **Selector del Árbitro - Director CPU/GPU/TPU/NPU** ha dirigido la ejecución del traductor Helsinki-NLP/opus-mt-en-es a CPU o TPU conforme a la solicitud. La serie final comprende **22 traducciones, seis decisiones ejecutadas y dos rechazos lógicos por TPU declarada no disponible**. El cotejo numérico y la atribución de las trazas son conformes para los dos casos fijados.

La TPU reduce el tiempo de generación, pero la conservación completa de sus trazas aumenta el tiempo total de la solicitud. Al medir ese recorrido completo, el Selector elige **CPU en ambos casos**. No existe prioridad automática de un acelerador sobre la CPU.

El ensayo demuestra esta capacidad en el perfil descrito. No valida el nodo 1, no incorpora todavía el adaptador al servicio documental activo y no acredita calidad general de las traducciones. La función de traducción pertenece a la infraestructura administrada por el SV y puede requerir selección de dispositivo aunque otras consultas se dirijan a modelos externos.

## Modelo y equipo

| Elemento | Identificación comprobada |
|---|---|
| Modelo | [Helsinki-NLP/opus-mt-en-es](https://huggingface.co/Helsinki-NLP/opus-mt-en-es) |
| Revisión | `fdaddf76f50fcc1583ba42f95965862a7ab30f97` |
| Arquitectura | Marian: seis capas de codificación y seis de decodificación; dimensión 512; ocho cabezas |
| Pesos | Safetensors, 312 062 468 bytes; SHA-256 `b3ecbf954573c2fd95d05d3ad4618baf961793db0da07c2add2e8a3a6cd78d0b` |
| Referencia CPU | Rust, Candle 0.11.0, F32 y generación voraz |
| Infraestructura del contraste | Google Cloud, `us-central1-a`, una TPU v5e, tipo `v5litepod-1` |
| CPU | AMD EPYC 7B13, 24 procesadores lógicos; cuatro trabajadores de Rayon |
| Memoria principal | 50 499 756 032 bytes |
| Memoria TPU comunicada por PJRT | 16 909 336 064 bytes; dispositivo 0 |
| Sistema y compilación | Ubuntu 22.04, glibc 2.35, Rust 1.98.1; dependencias Cargo fijadas |
| Biblioteca del fabricante | libtpu 0.0.50; SHA-256 `eb7cbd2bc7c7283c9f6e89a8d23dfbd2b275a3215b121c8f9bc3face842fc7c7` |
| Ejecutable de la serie final | SHA-256 `3355a85d4d1272a51c9a75a5baa1d07b49c34e9aa95d45470d0a987dba6a27d4` |

El modelo ya funcionaba en el servicio documental. Se conserva [el motor original](codigo/motor), [su integración](codigo/integracion/traduccion.rs) y [su procedencia](codigo/PROCEDENCIA.json). El servicio utiliza aislamiento seccomp y Landlock; ese aislamiento no se atribuye al proceso experimental que accede a libtpu. Tampoco se trasladan al ensayo todos los límites temporales y de longitud del servicio.

## Realización en Rust y dependencias

El adaptador expresa el codificador y el decodificador completos de Marian en StableHLO. Rust construye las operaciones, controla la generación, invoca el Selector y conserva la observación propia. La TPU calcula ambos programas; la CPU conserva tokenización, selección del siguiente token, control, recuperación y composición del texto. No se presenta toda la solicitud como ejecución exclusiva en TPU.

PJRT y la biblioteca nativa de Google realizan compilación y ejecución en TPU. Por tanto, **esta vía no constituye una infraestructura íntegramente escrita en Rust**. Las cabeceras C/C++ se utilizan para generar la interfaz de enlace durante la compilación. StableHLO pertenece al adaptador del modelo; no sustituye ni modifica la representación intermedia del SV.

El [código completo del contraste](codigo/contraste-cpu-tpu) incluye Selector, adaptador, observación y dependencias locales. Su [descripción de compilación y ejecución](codigo/contraste-cpu-tpu/LEAME-SV.md) identifica cada elemento. El Selector conserva SHA-256 `2b6cd91da365f15ab314ea69ef6eeec1b12826ddf0de01fcca58191ef5afff05`.

## Protocolo y resultados medidos

El [protocolo](evidencias/contraste-cpu-tpu-20261014/PROTOCOLO.md) fija dos fragmentos anteriores: `NCI_4-0` y `NCI_178-3`. Se conserva su contenido, sus pesos y tokenizadores; no se modifica la indicación para mejorar resultados. La ejecución experimental calcula el prefijo causal completo, con 64 posiciones, frente a la memoria incremental de la referencia CPU. Una salida que no termine dentro del límite se rechaza.

Se comprobaron los codificadores y las puntuaciones de los 54 pasos de generación. Se exigieron valores finitos, mismos tokens y traducciones, diferencia absoluta máxima de 0,005 y error cuadrático medio normalizado máximo de 0,0001. El cotejo independiente recalculó **56 comparaciones vectoriales** a partir de los archivos originales: diferencia absoluta máxima **2,28881836E-05** y error normalizado máximo **3,09781649E-06**.

| Caso | Generación CPU, ms | Generación TPU, ms | Solicitud completa CPU, ms | Solicitud completa TPU, ms | Selección automática |
|---|---:|---:|---:|---:|---|
| NCI_4-0 | 281,263 | 106,164 | 327,797 | 1.238,158 | CPU |
| NCI_178-3 | 541,283 | 195,340 | 588,563 | 2.013,101 | CPU |

Son medianas de tres mediciones por dispositivo y caso, con pesos y programas ya disponibles. La generación incluye tokenización, inferencia, transferencias, elección de tokens, composición del texto y conservación de puntuaciones. La solicitud completa añade inicio y cierre del observador, recuperación, análisis y conservación de sus trazas. Carga del modelo y compilación iniciales se miden por separado. La escritura del propio registro temporal queda fuera del intervalo que declara. No se extrapolan estas medidas a un servicio sin esa instrumentación ni a otras longitudes o cargas simultáneas.

La serie final ejecutó **12 traducciones CPU y 10 TPU**. Las solicitudes obligatorias respetaron su destino; las automáticas utilizaron las medianas completas. La indisponibilidad se comprobó mediante dos cambios explícitos del inventario presentado al Selector, sin nuevas inferencias ni desconexión física del dispositivo.

## Observación, correcciones y conservación

Se conservan puntuaciones F32, textos, tokens, decisiones, medidas CPU, fases, programas StableHLO, trazas originales XSpace y su interpretación. Las trazas identifican los programas del modelo y sus operaciones en TPU 0; las ejecuciones CPU carecen de actividad TPU. Los eventos de transferencia se distinguen de una medida física del tráfico del dispositivo. No se midieron energía ni tráfico físico del bus.

El muestreo CPU de la serie final contiene 322 observaciones; su intervalo máximo fue 149,023 ms y el máximo de memoria residente observado fue 6412808192 bytes. El muestreo periódico no sustituye los contadores por fase ni la traza del acelerador.

Se conservaron las incidencias de compatibilidad con glibc y de preparación de la compilación. El primer programa TPU se rechazó por una sintaxis de reducción incorrecta; se corrigió conforme a [los ejemplos oficiales de StableHLO](https://github.com/openxla/stablehlo/blob/main/stablehlo/tests/transforms/stablehlo_aggressive_folder.mlir), sin cambiar las operaciones ni tolerancias. El aviso del cargador sobre alineación de páginas grandes permanece en el registro; no impidió la ejecución ni produjo errores en las trazas cotejadas.

La primera serie completa midió sólo la generación y seleccionó TPU. Su resultado permanece [identificado como medición parcial](evidencias/contraste-cpu-tpu-20261014/resultado-previo-medicion-parcial.json). La [adenda previa a la repetición](evidencias/contraste-cpu-tpu-20261014/ADENDA-MEDICION-COMPLETA.md) amplió la medida al recorrido completo. **La conclusión temporal de esta edición corresponde a la serie 04.** Se realizaron 44 traducciones entre ambas series completas; no deben confundirse con las 22 de la serie final ni con las comprobaciones numéricas CPU anteriores.

- [Resultado final](evidencias/contraste-cpu-tpu-20261014/RESULTADO.json), [cotejo independiente en Rust](evidencias/contraste-cpu-tpu-20261014/COTEJO-INDEPENDIENTE.json) y [código del cotejo](codigo/comprobacion-ensayo).
- [Inventario de archivos de entrega y SHA-256](evidencias/contraste-cpu-tpu-20261014/ARCHIVOS-ENTREGA.json).
- [Descarga de originales, ejecutables y fuentes fijadas](https://github.com/juantoniolloretegea/SV-motor/releases/tag/helsinki-cpu-tpu-20261014).

Los archivos se recuperaron y cotejaron antes de retirar el recurso temporal y su solicitud. La ausencia de ambos se comprobó mediante la API de Google. El importe atribuible sigue pendiente de conciliación; tiempo observado y tiempo facturado son magnitudes distintas.

## Antecedentes y límites de interpretación

Las [trece entradas](evidencias/ENTRADAS-CPU-ANTERIORES.jsonl), [sus salidas](evidencias/SALIDAS-CPU-ANTERIORES.jsonl) y [sus mediciones](evidencias/MEDICIONES-ANTERIORES.json) permanecen intactas: 239 tokens de entrada, 270 de salida, 10 127 ms de generación, 2 179 ms de carga y 676 488 KiB de memoria máxima. Pertenecen a otra ejecución y no se suman a los tiempos del contraste CPU/TPU.

Persisten las limitaciones lingüísticas del modelo. El contraste acredita selección y ejecución del perfil experimental, no aptitud clínica, cobertura documental general ni disponibilidad productiva del nodo 1. El gestor y el libro de bases no se modifican. Las traducciones documentales facilitan la lectura; los modelos evaluados reciben los originales conforme a las reglas del sistema.

[MANIFIESTO.json](MANIFIESTO.json) identifica esta edición. Modelo, código y dependencias conservan sus licencias respectivas; la licencia documental no reemplaza las de terceros.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).