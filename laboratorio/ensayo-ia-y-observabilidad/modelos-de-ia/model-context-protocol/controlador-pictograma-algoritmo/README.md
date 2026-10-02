# Árbitro Director del Lenguaje de Computación SV

**Diseño definido · 2 de octubre de 2026. Realización y comprobación experimental pendientes.**

El **Árbitro Director**, abreviado **Árbitro SV**, es el componente previsto del Lenguaje de Computación SV que dirige y supervisa la ejecución de un modelo concreto conforme a las reglas del Sistema Vectorial. Coordina la entrada documental, las herramientas, la generación, la conservación y la admisión de resultados. Su respuesta ante incidencias se determina mediante reglas y algoritmos explícitos.

El Árbitro supervisa también el servicio MCP: su identidad, sus reglas efectivas, sus límites de acceso y cada solicitud y devolución. La autoridad procede del SV y de los contratos del Lenguaje; una declaración del servicio o del candidato no sustituye su comprobación.

## Organización

| Documento | Contenido |
| --- | --- |
| [Diseño y diagramas](DISENO.md) | Gobierno, funciones, recorrido, intercambios, obligaciones, terna y evidencias. |
| [Verificación](VERIFICACION.md) | Comprobaciones instrumentales, integración y límites de la evaluación. |
| [Aplicación a Safeguard](../../openai/gpt-oss-safeguard-120b/CONTROL-DOCUMENTAL-ASISTIDO.md) | Primer contraste asistido y relación con los resultados del candidato. |

## Funciones

- **Algoritmo:** expresa condiciones, relaciones y reacciones permitidas.
- **Árbitro:** ejecuta esas reglas, comprueba sus precondiciones y controla las fronteras del recorrido.
- **MCP documental:** proporciona contenido de la caché autorizada mediante un contrato verificable.
- **Modelo:** interpreta los documentos y produce una respuesta conforme a la política recibida.
- **Pictogramas:** representan para la revisión humana la obligación, su valoración válida en `(0,1,U)` y la evidencia que la sustenta.
- **Evaluación competente:** adjudica el contenido y conserva la diferencia entre conformidad instrumental y aptitud para el dominio.

El Árbitro opera sobre un modelo identificado en cada ejecución. La reutilización con otros candidatos requiere comprobar su perfil, sus interfaces y sus límites. El diseño común conserva una sola sede documental y cada modelo mantiene su aplicación y sus resultados.

## Gobierno y alcance

La terna conserva el significado del SV: **0 = Apto, 1 = No apto, U = indeterminado**. Un fallo de transporte, una página ausente, una identidad desconocida o una pérdida de custodia se describen como incidencias instrumentales; no se convierten automáticamente en U. La constitución de parámetros, la asignación de valores y las reglas de aptitud pertenecen a las fuentes competentes del SV.

El control determinista permite exigir propiedades observables, como incorporar todas las páginas previstas antes de inferir. No convierte en determinista la interpretación del modelo ni garantiza la verdad de una conclusión. La evaluación del contenido sigue siendo necesaria.

El primer alcance utiliza Rust y el servicio documental local ya disponible. Los pictogramas proceden del registro comprobado y se destinan a la presentación humana. No se incorpora otra inteligencia artificial como directora ni se modifican los pesos del candidato.

## Fundamento

La especificación se vincula a los [pilares del Lenguaje](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md), a sus [perfiles y contratos](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md) y a los [fundamentos matemáticos y semánticos del SV](https://github.com/juantoniolloretegea/SV-matematica-semantica/blob/b8fd32978292d25adf9b87cf71e409005dce642c/documentos/fundamentos/README.md).

---

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es). Los componentes de terceros conservan sus licencias.
