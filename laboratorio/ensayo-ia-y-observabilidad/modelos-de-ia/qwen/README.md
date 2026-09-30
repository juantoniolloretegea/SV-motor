# Qwen

**Índice de modelos · 30 de septiembre de 2026.**

Esta carpeta reúne configuraciones experimentales de Qwen dentro del [ensayo de inteligencia artificial y observabilidad](../../README.md). Cada modelo conserva identidad, realización, evidencias y dictamen propios. Los resultados de una configuración no se transfieren a otra variante ni a la familia completa.

| Configuración | Evidencia disponible y estado | Documentación |
|---|---|---|
| Qwen3-0.6B | Inferencia nativa y controles parciales. Campaña cerrada con limitaciones; cuatro consultas DOC-01 sin conformidad contractual completa. | [Ficha y versiones](qwen3-0.6b/README.md). |
| Qwen3.8-27B | **No pasa** la selección examinada. Se conservan las deficiencias documentales y los archivos de cierre. | [Selección, seguimiento e imágenes](qwen3.8-27b/README.md). |
| Qwen3-Next-80B-A3B-Instruct · UQFF Q4K | Dos consultas documentales completas con MCP recibidas. El examen posterior es parcial: ocho respuestas comunicadas, interrupción en P09 y recepción pendiente. No hay dictamen del banco completo. | [Registro de realización](qwen3-next-80b-a3B-instruct/REGISTRO-INSTALACION-20260929.md) · [TT-0016](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0016.md). |
| Qwen3-Next-80B-A3B-Thinking · UQFF Q4K | Prueba independiente preparada y transmisión humana comunicada. Carpeta propia abierta para incorporar la documentación; instalación e inferencia no acreditadas por un resultado recibido. | [Carpeta del modelo](qwen3-next-80b-a3b-thinking) · [TT-0017](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0017.md). |

**Corte experimental:** 30/09/2026, 08:42 UTC. La incorporación posterior de la carpeta Thinking acredita su apertura documental, no el funcionamiento del modelo. Las comunicaciones de avance se distinguen de la recepción independiente.

El acceso a las fuentes se realiza mediante el [MCP documental](../model-context-protocol/0.1.1/LEAME.md) sobre una captura local identificada; la consulta no dispone de acceso libre a Internet. La clave de corrección permanece separada del candidato. La primera [ronda de 25 preguntas](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/6f1118a323bcefac9400d01274e4f8d1db3636ff/dominios/inmunologia/tes-examenes-conocimientos/ronda-01-pdq-tricoleucemia-20260929/PREGUNTAS.md), su [protocolo](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/6f1118a323bcefac9400d01274e4f8d1db3636ff/dominios/inmunologia/tes-examenes-conocimientos/ronda-01-pdq-tricoleucemia-20260929/PROTOCOLO.md) y el [seguimiento S39](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md#s39) permiten reconstruir el alcance del examen.

Una respuesta conservada no equivale a una respuesta correcta. Los fallos técnicos y las preguntas pendientes no se convierten en U. Sólo después de la corrección independiente de todas las posiciones procede un vector completo, su frame y el dictamen **Apto, No apto o U**, limitado al banco evaluado.

Se conservan las carpetas y los antecedentes históricos. [Índice anterior](https://github.com/juantoniolloretegea/SV-motor/blob/2e51917bb5864689cb6d2735fef9f2b1db35ce42/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/README.md) · [Catálogo de modelos](../README.md) · [Documentación general del ensayo](../../README.md).
