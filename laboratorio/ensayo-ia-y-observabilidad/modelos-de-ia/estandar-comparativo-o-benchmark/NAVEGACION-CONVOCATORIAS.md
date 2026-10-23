# Consulta de convocatorias, nodos y antecedentes

Fecha de revisión: 22 de octubre de 2026.

La [clasificación pública del SV](https://administracion.itvia.online/benchmark) permite consultar publicaciones revisadas mediante cuatro selectores comunes: **Nodo, Dominio, Prueba y Convocatoria**. Las tres pestañas —**Resultados y polígonos**, **Preguntas, fuentes y reglas**, **Métricas y evidencias**— conservan la misma convocatoria.

## Selección inicial y ausencia de coincidencias

La entrada general selecciona **Nodo 3**, por decisión vigente, y su última convocatoria por fecha de inicio del examen. Para fechas iguales se utiliza la finalización y después el identificador estable. Una revisión documental no modifica las fechas del examen ni desplaza artificialmente un antecedente a la primera posición.

Un enlace permanente a una convocatoria conserva su identidad y su nodo, incluidos los nodos 1 y 2. Si sólo hay una publicación, siempre se muestra. **Ver todas las convocatorias** elimina las restricciones de nodo, dominio y prueba. El catálogo completo permanece accesible en un desplegable, aunque haya filtros activos.

La pantalla identifica la convocatoria visible y diferencia el total publicado, las coincidencias con la selección y la entrada presentada. La dependencia es **Nodo → Dominio → Prueba → Convocatoria**. Cambiar un selector envía una consulta y retira las elecciones dependientes anteriores. Un enlace con filtros incompatibles conserva el nodo válido elegido, retira sólo los filtros incompatibles y presenta la última convocatoria compatible, explicando el ajuste. Las opciones ofrecidas corresponden a ese ámbito. **Consultar** también permite enviar el formulario sin JavaScript. La consulta se sitúa en el selector; el cambio de pestaña, en su contenido.

Un filtro modifica la consulta; no demuestra que las pruebas ocultas no existan. El catálogo reúne las publicaciones incorporadas y revisadas, no todos los trabajos en preparación ni los expedientes sujetos a acceso restringido.

## Nombres y ayuda contextual

Los selectores presentan nombres comprensibles: tema y número de preguntas en Prueba, y fechas y candidatos en Convocatoria. Los códigos se conservan en los detalles de identificación. Las ayudas junto a cada campo se abren a petición y se cierran con Cerrar o Escape; no alteran la selección. R0 es la respuesta inicial; R1, la revisión autocrítica; R2, la verificación final, sólo en los contratos históricos que utilizan esas etapas. Una etapa intermedia mejor no sustituye la entrega final.

Los filtros y la elección de idioma conservan información de privacidad accesible mediante una ayuda breve. Esta presentación no sustituye los avisos de los formularios que recogen datos o registran actuaciones. El consentimiento de cookies opcionales mantiene su mecanismo propio, sin aceptaciones genéricas añadidas.

## Publicaciones incorporadas

| Nodo | Prueba y candidatos | Fechas del examen | Célula o alcance |
| --- | --- | --- | --- |
| 3 | [Ciberseguridad, dieciséis preguntas y ocho candidatos](https://administracion.itvia.online/benchmark?convocatoria=eet-cyb16-20261021) | 21–22/10/2026 | (16,4), categorías general y adaptada separadas. |
| 3 | [Leucemia de células pilosas, veinticinco preguntas: Astra y Grok](https://administracion.itvia.online/benchmark?convocatoria=pdq25-api-20261007) | 07–08/10/2026 | (25,5), contratos y etapas originales. |
| 3 | [Leucemia de células pilosas, dieciséis preguntas: Qwen](https://administracion.itvia.online/benchmark?convocatoria=pdq16-qwen-20261008) | 08/10/2026 | (16,4), vector original y recepción posterior diferenciados. |
| 3 | [Ciberseguridad, dieciséis preguntas: Astra y GLM](https://administracion.itvia.online/benchmark?convocatoria=cyb16-api-20261008) | 08/10/2026 | (16,4), R0/R1/R2 del contrato histórico. |
| 2 | [Ciberseguridad, nueve preguntas: Claude mediante Kaggle](https://administracion.itvia.online/benchmark?convocatoria=cyb09-kaggle-20261010) | 10/10/2026 | (9,3), entrega final R2 y reserva de recepción independiente. |
| 1 | [Leucemia de células pilosas, veinticinco preguntas: Qwen Instruct](https://administracion.itvia.online/benchmark?convocatoria=pdq25-nodo1-20260929) | 29–30/09/2026 | (25,5) prevista; incidencias técnicas y cierre administrativo conservados. |

Las fechas describen la ejecución original, no la fecha de incorporación al catálogo. Las modalidades de nodo corresponden al acoplamiento: inferencia propia, plataforma gestionada y API directa del proveedor, respectivamente.

## Lectura y límites

Cada antecedente conserva preguntas, fuentes, etapas, resultados, reservas y métricas de sus documentos originales. No se recalcula con la plantilla actual ni se incorpora a una clasificación común de contratos diferentes. Una recepción documental posterior no sustituye el vector recibido. Un impedimento técnico no se convierte en U. Una etapa intermedia mejor no sustituye la entrega final.

**Abrir polígono SV** abre una consulta con escala y disposición comunes. El enlace **Visor original conservado** recupera la presentación histórica íntegra, comprobada por SHA-256. Las seis entregas publicadas de Kaggle, Astra, GLM, Qwen y Grok mantienen sus datos y etapas originales, sin recalificación. El visor queda aislado del origen y sin conexiones de red. Los enlaces a los archivos de GitHub permanecen disponibles como fuentes. Si una prueba no dispone de polígono publicado, no se fabrica uno a partir de un resumen incompleto. La célula pertenece a la prueba: (16,4) no constituye un límite del SV.

El resultado completo EET-CYB16, revisión 197, permanece intacto. Las categorías general y adaptada mantienen sus clasificaciones. Publicar resultados no equivale a recomendar un modelo ni autorizar su uso en producción.

Las soluciones públicas de esta edición conservan la limitación ya declarada. En nuevas convocatorias, las soluciones y elementos que permitan deducirlas permanecen reservados hasta que terminen todos los candidatos.

## Custodia y consulta técnica

[CONVOCATORIAS.json](CONVOCATORIAS.json) identifica las seis entradas. Las fichas de [antecedentes](antecedentes/) remiten a fuentes públicas por revisión y huella. Los enlaces anteriores a resultados, polígonos y datos de EET-CYB16 siguen asociados a esa convocatoria; una convocatoria futura no los reasigna. Una identidad desconocida solicitada como datos o polígono no devuelve los datos de otro examen.

Rust selecciona y presenta las publicaciones revisadas. El navegador permite consultar formularios, pestañas y fragmentos de navegación; no obtiene credenciales, no consulta proveedores y no inicia exámenes. La custodia privada conserva sus permisos.

## Presentación común del polígono

La presentación común utiliza la misma escala para la misma anchura disponible: un recinto cuadrado de hasta 460 unidades lógicas, con margen exterior de 48 y radios en proporción 1:2:3. La altura de la ventana y el número de parámetros no reducen la escala; se permite el desplazamiento vertical. La primera posición queda arriba y el recorrido es horario. Los números indican el orden del vector; el detalle conserva el identificador original. Los botones de selección están siempre encima del dibujo. En pantalla amplia, el detalle queda a la derecha; en pantalla estrecha, debajo. Esta organización visual no modifica símbolos, criticidades, etapas, dictámenes ni reservas. La célula pertenece a la prueba: la cota actualmente autorizada (49,7) no agota la formulación b² del SV.

El índice de navegación incluye **Justificación del método**, que abre su explicación dentro de Preguntas, fuentes y reglas y conserva la convocatoria seleccionada. Se mantienen las tres pestañas de contenido. Los filtros actualizan la consulta automáticamente cuando JavaScript está disponible; **Consultar** conserva el envío explícito y permite consultar sin JavaScript.

Una participación puede evaluar una célula, varias de un subdominio o las de un dominio completo. El detalle celular no fija el número de polígonos de un candidato. La ampliación prevista separa conjunto, rutas declaradas y detalle de célula; no infiere conexiones ni genera posiciones ausentes. Véase [Evaluaciones, células y rutas](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/estandar-comparativo-o-benchmark/PRESENTACION-CELULAS-Y-RUTAS.md), que distingue la consulta actual de los criterios de aceptación futuros.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
