# Conexión experimental con la administración documental

**Buscador-Semántico - Diferencial · Paso 8 / 8 · 11 de octubre de 2026.**

Está preparada y comprobada una conexión experimental en Rust entre la consulta opcional, el registro administrativo por dominio y la presentación mdBook. La ejecución final superó **35 controles conjuntos**. Las pruebas de módulos reúnen **57 controles distintos conformes**: 56 en la ejecución general y uno adicional de regresión, con cuatro controles repetidos. La función no está desplegada y el paso 8 permanece abierto.

## Funcionamiento preparado

La interfaz permite continuar sin comparar o elegir un documento, un grupo o la totalidad admisible dentro del mismo dominio. Reutiliza la identificación existente, los permisos documentales, la revisión del registro y la recuperación de contenido. Rust valida la selección antes de obtener los textos y comprueba de nuevo identidad, permisos y revisión antes de mostrar el resultado.

La comparación se realiza en un proceso separado con el perfil Linux de los lectores: sin credenciales, sin rutas de originales, con acceso a archivos, red y ejecución restringido. Se aplican 768 MiB de memoria virtual, 60 segundos de CPU y un plazo exterior de 90 segundos. El control de plazo comprueba también la finalización del proceso. No se introducen inferencias ni un nuevo criterio de detección: el archivo comunitario de reglas permanece idéntico al fijado en el paso 7.

La consulta presenta la selección efectiva, ambas localizaciones y los textos conservados de cada representación. El resultado permite examinar su procedencia y descargar la consulta con sus mediciones. El contenido activo de los documentos se presenta como texto. Las vistas temporales se vinculan a la identidad y al dominio, caducan a los cinco minutos y vuelven a comprobar permisos y revisión en cada acceso.

## Evidencia obtenida

| Comprobación | Resultado y límite |
| --- | --- |
| Separación por dominio | Registros sintéticos independientes de inmunología y ciberseguridad, con identificadores coincidentes y textos distintos. |
| Sugerencias y localización | Una oposición explícita produce un aviso con ambos pasajes; el documento compatible no lo produce. Son controles de funcionamiento, no una nueva medición de eficacia científica. |
| Identidad y selección | Rechazo de otra identidad, otro dominio, referencia ausente, original alterado, texto no UTF-8 y representación excesiva. El vacío no provoca recuperación de contenido. |
| Vigencia | Rechazo por caducidad sin cambio de revisión y por retirada posterior de un permiso. |
| Registro | La consulta conserva el historial; la retirada deliberada del permiso está identificada por separado. |
| Aislamiento y plazo | Restricciones de archivos, red y ejecución comprobadas. El vencimiento del plazo no se presenta como una consulta sin sugerencias. |
| Presentación | Generación real mediante mdBook 0.5.4 y cotejo posterior de cada original en su propia sección. Cuatro presentaciones finales examinadas en navegador. |
| Acceso HTTP | Rechazo sin sesión y de rutas de otro dominio, sin entrega de resultados. No sustituye una prueba autenticada positiva con GitHub. |

Se conservan tres ejecuciones de los mismos 35 controles, sin sumarlas como controles distintos. La primera pasó los controles entonces disponibles, pero falló en la inspección visual. La segunda registró cuatro rechazos del cotejo de presentación. La tercera resultó conforme. [Incidencias y correcciones](INCIDENCIAS.md) explica la diferencia y preserva los antecedentes.

Cada ejecución contiene cuatro comparaciones supervisadas concluidas, seis llamadas conocidas a la regla por pares y dos interrupciones deliberadas por plazo. Las llamadas internas que pudieran comenzar en los procesos interrumpidos no se miden y no se contabilizan como cero. En las tres ejecuciones hay dieciocho llamadas conocidas de procesos concluidos y seis interrupciones. Ninguna es una inferencia de IA.

## Compatibilidad y límites

Se utilizó Rust 1.95.0 oficial, instalado únicamente en el directorio de trabajo de la comprobación. Esa edición satisface el mínimo del componente comunitario; no se sustituyó el compilador del sistema. Las dependencias se compilaron desde la disponibilidad local. Se conservan las versiones, los manifiestos, las huellas de los ejecutables y los avisos de las dependencias. La compilación de interfaces de IA presentes en una dependencia no significa que se instancie un detector ni que se invoque un modelo.

El formulario preparado admite representaciones inglesas cuyo idioma se declara al elegirlas. La conexión administrativa para recuperar una traducción español–inglés conservada todavía falta. El selector de idioma de la Biblioteca permite una traducción visual, pero no acredita por sí solo una traducción identificada y recuperable para esta operación. Un documento español sin esa preparación produce una indicación expresa. No se incorporó una cuenta, un servicio económico ni un envío de originales privados.

Las cotas son 128 referencias, 256 KiB por representación y 4 MiB para el conjunto, con salida supervisada de hasta 2 MiB y 256 sugerencias. El exceso se rechaza. No se presentan estas cotas como una prueba de capacidad máxima o una garantía de rendimiento. Subsisten los falsos avisos y las omisiones documentados en los contrastes anteriores; la ausencia de aviso no demuestra ausencia de contradicciones.

## Continuación delimitada

Antes de utilizar la función en la administración deben integrarse estos cambios con su revisión vigente y comprobar el recorrido autenticado completo. La copia experimental conserva su base identificada y no sobrescribe modificaciones concurrentes. El adaptador de traducciones conservadas es una dependencia adicional para trabajar con documentos españoles. Estos pendientes impiden declarar disponible la función completa, aunque los controles sintéticos finales sean conformes.

La aportación al libro permanece separada y requiere autorización expresa antes de incorporarse. No se ha modificado el libro, su estructura, el Núcleo del SV, su representación intermedia ni el Árbitro - Director. Las sugerencias continúan siendo una ayuda voluntaria y no cambian clasificaciones, licencias, aprobaciones ni suministro documental.

El código, los registros originales y las trazas se conservan con acceso restringido. La documentación pública conserva resultados sintéticos, límites y constancia de cotejo, sin exponer referencias reservadas. [Descripción del funcionamiento](ALCANCE-DEL-PASO.md) · [Protocolo](PROTOCOLO.md) · [Resultados finales](ejecucion-03/RESULTADO.json) · [Revisión de presentación](REVISION-VISUAL.json).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
