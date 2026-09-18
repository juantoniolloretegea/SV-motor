# Inferencia experimental

## Finalidad

Comprobar el acoplamiento de Candle y Qwen3-0.6B dentro del [protocolo del ensayo](../README.md). La salida probabilística se recibe como propuesta auxiliar y no constituye autorización de ejecución ni decisión soberana del SV.

## Configuración seleccionada

- Motor: Candle; revisión de referencia `ddf1b879dc3a1760cbcb3f3c4a7c6467850cec4a`.
- Modelo: Qwen3-0.6B.
- Cuantización inicial: Q4_K_M, distribuida por Unsloth; su procedencia se distinguirá de la publicación original de los pesos.
- Ejecución inicial mediante CPU, sin dependencia de aceleración gráfica.
- Una conversación; hasta 2.048 tokens totales retenidos y 128 tokens generados por caso.

La revisión exacta de los pesos, sus bytes, su SHA-256 y la identidad del tokenizador están pendientes de fijación. No se habilitará una sustitución automática de modelo.

## Comprobaciones previas

1. Verificar la compatibilidad entre arquitectura, formato de pesos, tokenizador y plantilla de conversación.
2. Utilizar los identificadores de finalización establecidos por el modelo y comprobarlos mediante casos específicos.
3. Diferenciar terminación normal, truncamiento, cancelación y error.
4. Acotar contexto, generación y memoria; declarar el mecanismo efectivo de control.
5. Evitar la inclusión automática de consultas y respuestas completas en consola o telemetría.
6. Comprobar las características de compilación y las dependencias nativas o transitivas necesarias.
7. Mantener identificados los cambios respecto del ejemplo de referencia y las condiciones de distribución de los componentes incorporados.

El ejemplo publicado es material de referencia, no un artefacto validado para este protocolo.

## Variación experimental

Cada cambio de pesos, cuantización, plantilla, tokenizador, versión del motor o entorno requiere identificar las pruebas afectadas. La aprobación de un modelo pequeño no se transfiere a modelos mayores ni a otra arquitectura. La igualdad de resultados de un verificador determinista se evaluará separadamente de la variación numérica o textual de la inferencia.

## Estado

No hay implementación ni resultados de ejecución en esta versión.
