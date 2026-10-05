# Valoración del rendimiento en CPU de Qwen3.5-122B-A10B Q8_0

Fecha de observación: 5 de octubre de 2026. Preparación documental local vinculada a QWEN35-PRE-20261004/r1; no modifica la condición experimental ni constituye recepción independiente.

La lentitud de la realización examinada está acreditada. La explicación recibida mezcla datos de instalación con hipótesis causales que esos datos no demuestran. Las mediciones de cuatro casos documentales completos ofrecen una base más representativa que la recepción inicial de tres tokens de salida.

| Caso | Entrada, tokens | Salida nativa, tokens | Entrada nativa, s | Generación nativa, s | Generación, tokens/s |
| --- | ---: | ---: | ---: | ---: | ---: |
| A01/A0 | 2121 | 1691 | 391,886 | 7839,775 | 0,21569496 |
| A02/A0 | 2117 | 2784 | 392,123 | 12770,411 | 0,21800394 |
| A03/A0 | 2109 | 1747 | 387,989 | 8265,584 | 0,21135834 |
| A04/A0 | 2123 | 1916 | 393,529 | 9116,369 | 0,21017139 |

Fuentes: originales y medición nativa conservados en los hitos [A01](https://github.com/juantoniolloretegea/SV-motor/tree/1d611d8639d64d4790b402e3ebe16fe5960a51af/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3.5-122b-a10b-q8-0/preevaluacion-20261004/resultados/hitos/A01-A0), [A02](https://github.com/juantoniolloretegea/SV-motor/tree/bd72953e0931d1a7d382cb63e5c03c97b9154666/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3.5-122b-a10b-q8-0/preevaluacion-20261004/resultados/hitos/A02-A0), [A03](https://github.com/juantoniolloretegea/SV-motor/tree/11cc9f986ac45afd0c8316d4b443064e954db135/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3.5-122b-a10b-q8-0/preevaluacion-20261004/resultados/hitos/A03-A0) y [A04](https://github.com/juantoniolloretegea/SV-motor/tree/149c4b848475802942af35ab39e7335081398480/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3.5-122b-a10b-q8-0/preevaluacion-20261004/resultados/hitos/A04-A0). Publicaciones recuperadas y cotejadas en Rust. El total nativo de salida comprende los canales conservados; su recodificación externa se distingue de los identificadores nativos generados.

## Alcance de las explicaciones

**Memoria.** La ocupación del grupo de procesos incluye memoria anónima, archivos y otras partidas; no es una medida del tráfico por token. La [ficha oficial](https://huggingface.co/Qwen/Qwen3.5-122B-A10B#model-overview) declara 122 mil millones de parámetros totales y 10 mil millones activados. Ni esa arquitectura ni el máximo de memoria permiten afirmar que cada token lea toda la ocupación del grupo. Falta una medición del tráfico efectivo para establecer un límite físico de velocidad.

**Presión de memoria.** Los eventos de memory.high corresponden al mecanismo de control descrito en la [documentación del núcleo Linux](https://docs.kernel.org/admin-guide/cgroup-v2.html#memory-interface-files). No miden saturación del ancho de banda. A01 incrementó el contador de 1503 a 1582; A02–A04 no lo incrementaron. Los cuatro casos terminaron sin intercambio, agotamiento de memoria ni reinicios observados. Esta evidencia no demuestra un riesgo inminente de agotamiento por incorporar cualquier PDF ni excluye una limitación de ancho de banda.

**Paralelismo.** Las 48 CPU virtuales recibidas no acreditan 48 hilos de cálculo simultáneos ni recursos físicos exclusivos. Las variables de concurrencia configuradas son 24. La observación de sólo lectura del 05/10 a las 10:56:42 UTC distingue el proceso medidor del proceso del modelo y cuenta 122 hilos existentes en este último. Ese inventario incluye hilos que pueden estar en espera: no identifica el paralelismo efectivo, el óptimo ni el coste de coordinación. Por tanto, tampoco demuestra que 8–16 hilos sean mejores en esta realización.

**Corrección del motor.** La ruta CPU r2 delimitada selecciona los expertos antes de descomprimir sus bloques y requiere Q8_0, entrada F32 y CPU x86_64. El contraste sintético de una proyección mejoró de 4,814602741 a 0,084708226 segundos para tres repeticiones, con igual suma de control. Es un antecedente de instalación; no prueba una aceleración equivalente del modelo completo. Otro formato de pesos requiere comprobar qué ruta utilizaría. La mejora inmediata de dos o tres veces y una pérdida de precisión aceptable no están acreditadas.

**Contexto y documentación.** El máximo configurado de 32768 tokens no equivale a la longitud real de las entradas, próximas a 2100 tokens en estos casos. A04 requiere 8267 tokens al sumar entrada, salida máxima de 4096 y reserva de 2048; una capacidad de 8192 no satisface esa condición. La eventual recepción PDF prevista es condicional y se refiere a la primera página física íntegra, no a incorporar sistemáticamente las diez páginas del documento. Elegir sólo fragmentos relevantes cambiaría la exigencia de lectura íntegra del ensayo.

**Duración.** La generación lenta y la longitud emitida explican gran parte del tiempo total observado, aunque no identifican qué operaciones causan la lentitud por token. La carga inicial de aproximadamente 327 segundos se registra separadamente y no se repite por cada solicitud. Los registros no permiten afirmar un crecimiento exponencial de duración con el número de páginas.

## Límite de la valoración

La causa completa del rendimiento permanece sin determinar: no se ha realizado un perfil integral de cálculo, tráfico de memoria y concurrencia del motor durante esta fase. Los datos sí acreditan una velocidad de generación muy baja para la condición CPU examinada y permiten rechazar las promesas cuantitativas que carecen de contraste específico. A05 continúa en su solicitud única al corte de esta valoración; su salida no se adjudica anticipadamente.

La observación puntual se conserva en ejecucion/qwen-preevaluacion-20261004/preparacion/OBSERVACION-MEMORIA-CONCURRENCIA-20261005.json. La lectura estática y las observaciones no modificaron pesos, motor, cotas, contexto ni fuentes y no añadieron inferencias. Las recepciones y el resultado final de fase se consignan con sus propios alcances.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
