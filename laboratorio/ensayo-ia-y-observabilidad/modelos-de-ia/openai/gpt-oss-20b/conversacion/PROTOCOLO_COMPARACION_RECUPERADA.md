# Comparación con preguntas y fuentes recuperadas de Qwen

Fecha: 24/09/2026. Autorización expresa de repetición y contraste. Protocolo anterior a observar respuestas nuevas de estas condiciones. Fuentes recuperadas completas: dos exportaciones JSON de nueve y dos turnos, y cuatro peticiones documentales de consulta-documental-01. No se afirma que exista una tercera ronda recuperada.

## Preguntas y condiciones

Las once preguntas conservan su texto, orden y agrupación. Se sustituye únicamente el nombre de un familiar por [familiar]; se registra la transformación. No se publican los expedientes personales originales. Las respuestas de Qwen se conservan como referencia histórica, no como verdad de contraste. La ronda R02 se ejecuta antes de R01, sin mezclar sus antecedentes. Cada ronda de GPT-OSS conserva sus propias respuestas, incluidas las limitadas; no recibe respuestas ideales ni las de Qwen como si fueran propias.

Configuración GPT-OSS: canal final explícito, temperatura cero, reserva de 256 tokens, máximo 600 segundos por petición, contexto operativo 4096 incluido el espacio de respuesta. Qwen usó razonamiento habilitado y reserva de 2048 tokens en las rondas recuperadas. Son modelos, cuantizaciones y condiciones diferentes. Se comparan cualitativamente seguimiento, incertidumbre, correcciones, pertinencia y conservación de antecedentes; no se atribuye causalidad exclusiva al modelo ni igualdad de dificultad temporal. Un corte por generación se informa como respuesta incompleta, no como error semántico automático. Si el historial no cabe, se conserva el rechazo y no se recorta silenciosamente.

Las preguntas clínicas se tratan como material histórico anonimizado del ensayo, no como consultas asistenciales ni validación del dominio. El juicio clínico detallado requerirá contraste con fuentes primarias vigentes antes de emitir conclusiones específicas. Las premisas electorales se evalúan por el manejo de la incertidumbre; no se dan por ciertas por estar en la pregunta.

## Consulta del universo de inmunología

DOC01–DOC04 conservan exactamente los textos originales de sistema, pasaje y pregunta, incluida la versión OP-IMM-001-P10@1.0. Se cambia la plantilla Qwen por Harmony para GPT-OSS. Máximo 192 tokens de respuesta y 600 segundos. La configuración anterior utilizaba temperatura 0,7, Top-P 0,8, Top-K 20 y plazo 180 segundos; aquí temperatura cero. Se conserva explícitamente esa diferencia. El verificador compara el objeto JSON de tres campos con el esperado y exige terminación normal. No se admiten campos extra ni bloques Markdown.

DOC01 y DOC04: cita literal de la primera oración. DOC02 y DOC03: sin respaldo, sin citas inventadas. El texto bibliográfico referenciado no se considera suministrado. La verificación se realiza en Rust fuera del modelo; no constituye integración del contrato completo del SV.

## Ejecución y límites

Una sola inferencia simultánea. Primero concluye o se interrumpe documentadamente la campaña M/L/D anterior, que mantiene su plazo original. La comparación recién solicitada constituye un banco independiente, con máximo 60 minutos desde su inicio; no reinicia ni modifica el plazo de la campaña anterior. Orden: DOC01–DOC04, R02 y R01. Sin compras ni nuevas ramas. Las entradas y este protocolo se publican antes de ejecutar; los resultados adversos, tiempos, identificadores y errores se conservan en directorios nuevos.

Los procesos disponen de plazos exteriores. Un error de transporte del banco documental ordena detener el motor y comprobar MainPID=0. El banco conversacional utiliza la API y cancelación existentes. No se repiten peticiones con respuesta dudosa sin conciliar el identificador y documentar una causa concreta. Los límites temporales pueden dejar condiciones pendientes; no se declaran ejecutadas.
