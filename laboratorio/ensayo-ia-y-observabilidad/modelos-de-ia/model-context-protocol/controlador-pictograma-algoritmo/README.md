# Control documental asistido: controlador, algoritmo y pictogramas

**Edición de diseño 1 · 2 de octubre de 2026.** Estado: especificación documental; componente nuevo no implementado, no comprobado en ejecución y no desplegado.

Este diseño separa las obligaciones documentales que puede ejecutar un programa de la interpretación que corresponde al modelo. Un controlador desarrollado en Rust obtendrá mediante el servicio MCP todas las páginas fijadas para cada caso, comprobará su incorporación efectiva a la entrada y condicionará la admisión de la respuesta a requisitos verificables. Una evaluación independiente determinará después la corrección del contenido.

**La documentación completa debe llegar antes de la consulta. Su presencia y una cita auténtica no garantizan que la conclusión sea correcta.**

## Organización y lectura

| Documento | Contenido |
| --- | --- |
| [Diseño y diagramas](DISENO.md) | Responsabilidades, recorrido completo, intercambio entre componentes, obligaciones, estados, registro y límites. |
| [Comprobaciones y criterios de continuación](VERIFICACION.md) | Casos adversariales, recorrido conforme, integración y contraste inicial limitado. |
| [Aplicación prevista a Safeguard](../../openai/gpt-oss-safeguard-120b/CONTROL-DOCUMENTAL-ASISTIDO.md) | Primer candidato, relación con su expediente y separación de los resultados anteriores. |

La carpeta conserva el nombre `controlador-pictograma-algoritmo` y permanece bajo `model-context-protocol` por su relación con el servicio documental compartido. El controlador pertenece a la aplicación que utiliza MCP: no es una función que el protocolo proporcione automáticamente. El diseño común tiene aquí su única sede; cada modelo mantiene en su expediente su configuración, aplicación y resultados propios.

## Función de cada parte

- **Algoritmo:** determina condiciones, comprobaciones y transiciones permitidas mediante reglas explícitas.
- **Controlador:** ejecuta ese algoritmo, solicita los documentos, conserva evidencias y admite o bloquea el recorrido.
- **MCP documental:** permite leer la caché autorizada mediante un contrato identificado y auditable.
- **Modelo:** clasifica la afirmación y explica su fundamento conforme a la política y los pasajes recibidos.
- **Pictogramas:** representan para la revisión humana el estado de cada obligación y enlazan su evidencia. No ejecutan reglas ni constituyen una autoridad adicional.

Las capas pueden complementarse porque comprueban propiedades distintas. Su composición no demuestra por sí sola una mejora de fiabilidad ni compensa un incumplimiento crítico.

## Primer alcance

Adquisición documental íntegra y asistida, corpus cerrado, una consulta prevista por caso y salida estructurada. Primero se comprobará el controlador sin modelo; después, la integración; por último, se propone un contraste nuevo de tres casos reservados. Safeguard será el primer candidato previsto, con identidad y resultados del sistema compuesto separados de sus contrastes anteriores.

Se mantiene el servicio MCP existente si satisface el contrato. La primera versión no incorpora Stateright, Regorus, un editor visual de reglas ni representaciones braille o pictográficas como entrada del modelo. No requiere ajuste de pesos ni sustitución del motor por el empleado en un ejemplo de terceros.

La publicación de esta especificación no acredita implementación, disponibilidad actual del servidor, recepción favorable ni ejecución de nuevos casos. La continuación se rige por las etapas y límites de [verificación](VERIFICACION.md).

## Relación documental

[Ficha del servicio MCP](../FICHA_TECNICA.md) · [TT-0014, componente documental](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0014.md) · [Expediente Safeguard](../../openai/gpt-oss-safeguard-120b/FICHA_TECNICA.md).

Los originales de una futura ejecución tendrán su entrega identificada en SV-sala-de-maquinas; la aplicación y los resultados se registrarán en el expediente de cada modelo. Los sucesos, tiques y actas de Calidad conservarán la recepción que les corresponda, sin sustituirla por este diseño.

---

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es). Los componentes de terceros conservan sus licencias.
