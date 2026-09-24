# Recuperación de datos y latencia con contexto creciente

Banco ejecutado el 24/09/2026, de 15:46:20 a 16:16:40 UTC. Ocho condiciones terminadas normalmente mediante la misma API de la interfaz. Originales recibidos en `7253de278938e82be453c3d709515742ae13b3b7`, directorio `verificacion/0.2.2/evidencias/contexto-03`. Servicio 0.2.2, GPT-OSS-20B MXFP4, motor Rust corregido, CPU, temperatura cero y canal final explícito. Identidades completas en `IDENTIDAD.json` y criterios en `src/comparison.rs`.

La conversación de cuatro turnos conservó el identificador AZ-204, sustituyó 18 por 11 piezas y recuperó el almacén Norte. El último turno devolvió exactamente los tres campos solicitados. Los tres reconocimientos anteriores añadieron un punto final a RECIBIDO o ACTUALIZADO: contenido pertinente, pero incumplimiento del formato literal exigido. Resultado: cuatro contenidos correctos, un formato estricto correcto de cuatro; no se oculta la desviación formal.

| Condición | Tokens de entrada reales | Tiempo total, s | Procesamiento de entrada declarado por el motor, s | Respuesta |
|---|---:|---:|---:|---|
| L0256 | 268 | 73,374 | 70,486 | CL-7319 |
| L0768 | 772 | 199,254 | 196,182 | CL-7319 |
| L1536 | 1556 | 419,201 | 413,900 | CL-7319 |
| L3072 | 3096 | 891,512 | 885,329 | CL-7319 |

Las cuatro condiciones recuperaron literalmente el identificador situado antes del relleno. La salida tuvo cinco tokens declarados, incluido el terminador. No es una prueba de comprensión general de documentos extensos ni una conversación de miles de turnos. Hay una observación por tamaño, sin repeticiones para estimar dispersión.

La latencia es la principal limitación observada. En la condición mayor, el motor atribuye unos 885 de 891 segundos al procesamiento de la entrada. Es una descomposición informada por el motor, no una medición independiente del tiempo hasta el primer token. La interfaz entrega la respuesta completa al finalizar. La caché de prefijos estaba desactivada y cada petición incorporaba íntegramente sus antecedentes. Estos datos orientan una eventual investigación del procesamiento y reutilización del contexto; no demuestran todavía la eficacia de ninguna optimización nueva.

El pico acumulado del grupo de memoria del motor fue 17.072.148.480 bytes, dentro de su cota de 32 GiB. Los contadores conservados no registran OOM ni terminaciones por falta de memoria en estas ocho peticiones. El pico pertenece a la sesión residente, no a un incremento aislado de la última condición. RSS del proceso y consumo del grupo son medidas distintas y no se suman.

Se observaron ocho terminaciones normales y ocho contenidos correctos dentro de los criterios limitados de este banco; cinco salidas cumplieron el formato estricto. No se fusionan estos resultados con las doce tareas de la versión 0.2.0 para atribuir una aprobación homogénea a una única versión. Diálogo acumulativo, comparación con preguntas históricas y prueba documental se informan por separado.

Entre aproximadamente 16:12 y 16:14 UTC el editor mostró reconexión. Se recuperó mediante su dirección habitual; se conservaron la terminal, el túnel y el proceso de cálculo. La condición L3072 terminó sin reenvío. La incidencia de acceso no se atribuye al modelo.
