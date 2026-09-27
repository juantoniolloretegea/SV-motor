# Seguimiento experimental de Qwen3.8-27B

**Corte: 27/09/2026.** La secuencia distingue ejecución, acceso documental y selección del candidato. Los expedientes enlazados conservan originales; los enlaces privados requieren autorización de lectura.

| Fecha y etapa | Resultado acreditado | Alcance y fuente |
|---|---|---|
| 26/09 · revisión de arquitectura | La objeción inicial confundía la compuerta de atención completa con la normalización con compuerta GatedDeltaNet. Sigmoid y SiLU tienen funciones distintas en la revisión inspeccionada. | Se conserva la rectificación; no se modificó sigmoid para sustituirla por Swish. La lectura estática no acreditó por sí sola ejecución ni calidad. |
| 26/09 · diagnóstico HCL01 | 2.048 tokens, terminación por longitud y cero bytes de respuesta final. | [Diagnóstico](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/tree/7a116cddac3f0b98bdc52fb84e1ed1e0f2b23791/respuestas-ejecucion/QWEN38-HCL01-DIAGNOSTICO-20260926/entrega-01). Contenido clínico no evaluable. |
| 26/09 · primer archivo | Imagen compacta cifrada, sin pesos Qwen; integridad y descarga cotejadas. | [Antecedente de conservación](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/tree/7a116cddac3f0b98bdc52fb84e1ed1e0f2b23791/respuestas-ejecucion/QWEN38-HCL01-DIAGNOSTICO-20260926/imagen-onecloud). Sin restauración ejecutada. |
| 26/09 · modo directo | HCL01 produjo respuesta final; evaluación registrada de 7/8. | [Ensayo directo](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/tree/7a116cddac3f0b98bdc52fb84e1ed1e0f2b23791/respuestas-ejecucion/QWEN38-HCL01-DIAGNOSTICO-20260926/modo-directo-20260926). El caso conocido no constituye una prueba independiente. |
| 26/09 · continuación directa | Cuatro controles lógicos conformes; HCL02 terminó normalmente, pero presentó una atribución documental incorrecta relevante. | [Continuación](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/tree/7a116cddac3f0b98bdc52fb84e1ed1e0f2b23791/respuestas-ejecucion/QWEN38-HCL01-DIAGNOSTICO-20260926/continuacion-directa-20260926). Se detuvo la secuencia; HCL03–HCL12 no se ejecutaron. |
| 26/09 · búsqueda web | La inicialización requirió el modelo auxiliar EmbeddingGemma. La consulta recuperó resultados de búsqueda, pero no acreditó extracción completa ni respuesta final dentro del plazo. | [Expediente web](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/tree/7a116cddac3f0b98bdc52fb84e1ed1e0f2b23791/respuestas-ejecucion/QWEN38-BUSQUEDA-WEB-DECISION-20260926/entrega-02). Impedimento instrumental; no es un resultado clínico adverso. |
| 26/09 · acceso documental local | La recuperación MCP permitió reconstruir secciones; el recorrido generativo no obtuvo respuesta final dentro de su plazo. | [Recepción documental](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/tree/7a116cddac3f0b98bdc52fb84e1ed1e0f2b23791/respuestas-ejecucion/MCP-DOCUMENTAL-PREPARACION-20260926/entrega-05). La recepción instrumental y la evaluación del candidato son juicios distintos. |
| 27/09 · selección mínima final | Una generación: HTTP 200, `end_turn`, 909 tokens de entrada y 230 de salida; 227,316 s con conservación. Respuesta completa, con omisión material y deficiencias de citas y localización. | [Informe final](https://github.com/juantoniolloretegea/SV-sala-de-maquinas/tree/7a116cddac3f0b98bdc52fb84e1ed1e0f2b23791/respuestas-ejecucion/QWEN38-SELECCION-MINIMA-LOCAL-20260927/entrega-01). **NO PASA**. No se repite el caso ni se inicia el paso 2. |
| 27/09 · archivo de cierre | Segunda imagen cifrada, sin pesos, con los expedientes posteriores. Cotejo de archivos, revisión del sistema de archivos y descifrado comprobados. | [Manifiesto](imagen-onecloud/MANIFIESTO.json). Sin ensayo de arranque ni restauración de esta imagen. |

## Interpretación del cierre

La selección final utilizó una sección literal recuperada mediante tres páginas MCP en aislamiento de red y cotejada con su huella. La respuesta llegó completa. La retirada de esta configuración tiene, por tanto, un fundamento diferente de las interrupciones anteriores: incumplió el criterio documental y de contenido fijado para ese caso.

El caso no es ciego y no permite estimar una tasa general de error médico. La conclusión se limita a la configuración y a la selección actual. No demuestra que todo entrenamiento futuro resulte inútil ni que otros modelos o configuraciones fracasen.

Se conservaron 274 muestras de recursos en la comprobación final; no se registraron OOM ni OOM-kill. La medida de E/S no estuvo disponible. No se convierte el límite de memoria aplicado en un requisito mínimo general.

## Referencias técnicas de la rectificación

- [Atención completa del motor fijado](https://github.com/EricLBuehler/mistral.rs/blob/2370966bb91e2e3dafa0b1521b87c50fd5c01244/mistralrs-core/src/vision_models/qwen3_5/text.rs).
- [Normalización GatedDeltaNet con SiLU](https://github.com/EricLBuehler/mistral.rs/blob/2370966bb91e2e3dafa0b1521b87c50fd5c01244/mistralrs-core/src/gdn/norm.rs).
- [Configuración del modelo fijada](https://huggingface.co/Qwen/Qwen3.8-27B/blob/1d4bf0f2ff6012fd82039f2fa52739d0dd7c60c0/config.json).

La existencia de una entrega de software o de una imagen recuperable no equivale a aceptación médica ni a recepción integral del servicio documental.
