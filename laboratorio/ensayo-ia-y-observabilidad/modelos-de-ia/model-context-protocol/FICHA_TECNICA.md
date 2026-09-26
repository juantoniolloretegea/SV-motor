# Ficha técnica del servicio documental MCP local

**Revisión documental:** 1 · 26 de septiembre de 2026.  
**Seguimiento:** [TT-0014](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0014.md) · [S39](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/sucesos/SUCESOS_SV.md#s39) · [Especificación estructurada](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/json/TT-0014_MCP_DOCUMENTAL_LOCAL_2026-09-26.json).  
**Estado:** especificación experimental pendiente de ejecución y recepción. El antecedente 0.1.0 conserva el dictamen de preparación incompleta.

## Identidad y adscripción

Componente documental reutilizable del [Ensayo de inteligencia artificial y observabilidad](https://github.com/juantoniolloretegea/SV-motor/blob/6dd324a315fd6b9b952010d052ca7bf21b276e93/laboratorio/ensayo-ia-y-observabilidad/README.md), investigación lateral de (p1+P3)-Bis procedente de la adenda OP-CYB-001. El ensayo comprende inmunología y ciberseguridad inteligente; esta realización utiliza exclusivamente un documento de referencia relacionado con inmunología. No constituye el dominio inmunológico, una operación OP-IMM-001 ni una integración con el núcleo.

Esta ficha se incorpora para identificar el componente y sus dependencias. No es una nueva versión ejecutable ni sustituye las fichas de Qwen3-0.6B o GPT-OSS. Cada modelo conserva sus resultados y límites propios. El modelo Mistral y el motor Rust mistral.rs son componentes diferentes.

## Antecedente material

La [documentación técnica de 0.1.0](https://github.com/juantoniolloretegea/SV-motor/blob/6dd324a315fd6b9b952010d052ca7bf21b276e93/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/model-context-protocol/0.1.0/sv-mcp-documental-0.1.0/LEAME.md) identifica el servicio `sv-mcp-documental`, el importador administrativo `importar-pdq` y el cliente técnico `comprobar-mcp`. Se conservan fuentes Rust, `Cargo.lock`, catálogo, HTML y evidencias. El [estado técnico revisado](https://github.com/juantoniolloretegea/SV-motor/blob/6dd324a315fd6b9b952010d052ca7bf21b276e93/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/model-context-protocol/revisiones/0.1.0-revision-02/ESTADO_TECNICO.md) mantiene pendientes el plazo integral, la conservación de errores, la paginación con expansión JSON, el rechazo de claves repetidas y las pruebas de cierre ante fallos.

Las diez pruebas y la reconstrucción de 25 páginas pertenecen a las ejecuciones originales. No demuestran uso del MCP por Qwen. La versión original permanece inalterada; toda corrección debe recibir otra carpeta y versión.

## Especificación de la realización prevista

| Elemento | Identidad o requisito |
|---|---|
| Lenguaje de implementación | Rust 1.98.0; dependencias fijadas mediante `Cargo.lock` |
| Servicio | Proceso separado; MCP por entrada y salida estándar; exactamente dos herramientas |
| Búsqueda | `buscar_documentos`: consulta textual, orden determinista y máximo cinco resultados; sin embeddings |
| Lectura | `leer_documento`: identificador documental, sección y página; localizador, versión y continuación explícita |
| Catálogo | Copia documental local, validada e inmutable durante la ejecución; solo lectura efectiva para los procesos de consulta |
| Fuente | [PDQ profesional en español del NCI](https://www.cancer.gov/espanol/tipos/leucemia/pro/tratamiento-celulas-pilosas-pdq); HTML original y extracción completa verificable |
| Emulación | `emulacion-dominio-inmunologia`; espacio experimental separado del conocimiento admitido |
| Candidato de esta realización | `Qwen/Qwen3.8-27B`, revisión `1d4bf0f2ff6012fd82039f2fa52739d0dd7c60c0` |
| Motor | mistral.rs, revisión `2370966bb91e2e3dafa0b1521b87c50fd5c01244`; identidad efectiva por comprobar antes de ejecutar |
| Interfaz del candidato | Cliente Rust externo que conserva cada intercambio y controla el ciclo de llamadas MCP |
| Observabilidad | Registros originales JSON/JSONL; correlación con OpenTelemetry Rust y declaración de cobertura |
| Restricciones de acceso | Sin red durante la consulta, sin herramientas de shell, ejecución de código ni rutas arbitrarias |
| Dependencias excluidas | Búsqueda web incorporada, EmbeddingGemma y cualquier modelo auxiliar |
| Cotas | Una llamada simultánea, 8 000 caracteres por respuesta serializada completa y 30 segundos por transacción íntegra |
| Entorno | Instancia conservada 864618; sin ampliaciones ni nueva infraestructura |

Las identidades del candidato y del motor proceden de los antecedentes del TT; no representan una inspección remota realizada al publicar esta ficha. La nueva versión del servicio, sus huellas y las revisiones efectivas de sus dependencias quedan pendientes de la entrega.

## Flujo y autoridad

La adquisición administrativa identifica y verifica la fuente. El servicio consulta únicamente el catálogo admitido para el ensayo. El modelo solicita herramientas mediante una interfaz declarada; el cliente valida los argumentos, obtiene el resultado MCP, lo conserva y lo incorpora sin alterarlo a la siguiente petición del modelo.

La configuración, la selección documental, las transformaciones, las pérdidas de contenido y los límites forman parte de la identidad experimental. El detalle de pruebas, registros y criterios se fija en [TT-0014](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/tiques-tecnicos/TT-0014.md). Si una dependencia obligatoria o su evidencia falta, no se sustituye por conocimiento del modelo.

Rigen el [acta de rutas de conocimiento](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/ace76e62d28124022463dbe741cb4bbbd4da907d/dominios/ACTA_EVALUACION_Y_RECEPCION_DOCUMENTAL_RUTAS_CONOCIMIENTO_SV_2026_09_14.md) y el [marco técnico OP-IMM-001](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/bba2d3ae24cdc20e33b90375f295916928011985/dominios/inmunologia/marco-tecnico-de-universos-subdominios-y-modulos/01-op-imm-001-informacion-preinmunosupresion-adultos/Marco_tecnico_de_responsabilidad_trazabilidad_reproducibilidad_y_criticidad_OP-IMM-001_v0.1_2026-09-03.md). En consulta ordinaria no se habilita Internet. El acceso documental restringido no elimina la posibilidad de errores generativos ni otorga autoridad normativa a una respuesta. Las huellas prueban identidad e integridad, no verdad clínica; una coincidencia literal no basta para respaldar una conclusión.

El resultado de este ensayo no acredita aptitud clínica, calidad global de un modelo, cumplimiento regulatorio, admisión de conocimiento ni cierre de S39.
