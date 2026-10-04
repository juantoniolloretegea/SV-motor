# OpenAI · GPT-OSS-Safeguard-120B

**Corte documental:** 4 de octubre de 2026.  
**Identidad:** `openai/gpt-oss-safeguard-120b`.  
**Estado:** evaluación terminada. **No apto para acceder al examen en esta evaluación.** Instalación e inferencias acreditadas en sus expedientes; no se habilitan nuevas inferencias. [Cierre final y reservas](ensayos-reiterados-adversariales-y-aprendizaje/resultados/cierre-20261004/INFORME-FINAL.md).

La [guía oficial](https://developers.openai.com/cookbook/articles/gpt-oss-safeguard-guide) describe una especialización de GPT-OSS para clasificar contenido textual conforme a políticas proporcionadas en la consulta. Deben especificarse las categorías, los criterios y el formato de salida. Esta función no demuestra por sí misma corrección clínica ni capacidad para sustituir al [modelo ordinario](../gpt-oss-120b/README.md) como interlocutor especializado.

## Alcance del expediente

Esta carpeta separa la identidad, configuración, fuentes y pruebas de Safeguard. La descripción inicial del 1 de octubre se conserva en el historial; el estado vigente incorpora el cierre del 4 de octubre. El estudio de viabilidad del ordinario se conserva en su carpeta original y en TT-0015; sus cifras de memoria y sus conclusiones no son mediciones de Safeguard.

La [guía del sistema conjunto](https://github.com/juantoniolloretegea/SV-motor/blob/b8ff9198275ee0139dad7565d23733667aadb066/laboratorio/sistema-conjunto-lenguaje-computacion-ia-gobernada/README.md) contempla tanto la clasificación conforme a una política como la hipótesis de un interlocutor especializado. Son funciones por contrastar. El antecedente no fija una prioridad definitiva de ensayo de Safeguard. Su preparación exige delimitar la función elegida y el encargo correspondiente, sin ejecutar ambos modelos por el mero hecho de documentarlos.

## Conservación

La [conservación cifrada sin pesos](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/releases/tag/safeguard-imagen-20261004-v1), de acceso restringido, está publicada y recuperada íntegramente. El cotejo Rust acredita reconstrucción, descifrado, inventario y contenido: 281.859 entradas y 160 entradas de adjudicaciones conformes. Se conserva una corrección instrumental de tres nombres Linux, con sus antecedentes. [Procedimiento y comprobantes](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/tree/1abbaf1a50133938f92baa9cd1826c0f6a962c33/respuestas-ejecucion/SAFEGUARD-CONSERVACION-20261004). No se ha ensayado arranque restaurado; la recepción del archivo no acredita retirada administrativa ni habilita inferencias.

## Documentación

- [Ficha técnica y condiciones de prueba](FICHA_TECNICA.md).
- [Estado estructurado](ESTADO.json).
- [Modelo ordinario y antecedentes diferenciados](../gpt-oss-120b/README.md).
- [S39, seguimiento del ensayo](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md#s39).
- [TT-0015, antecedente del ordinario](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0015.md): no sustituye un encargo propio de Safeguard.

Los resultados conservan sus originales, referencias documentales y criterio de evaluación. La salida de clasificación del modelo no es, por sí misma, una adjudicación SV en la terna `(0,1,U)`. Los impedimentos instrumentales se identificarán separadamente.

[Índice OpenAI](../README.md) · [Catálogo de modelos](../../README.md) · [Ensayo](../../../README.md).

---

Sistema Vectorial SV · [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.es). Los componentes de terceros conservan sus licencias.
