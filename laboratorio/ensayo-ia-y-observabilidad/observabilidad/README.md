# Observabilidad mediante OpenTelemetry Rust

## Selección y finalidad

Se selecciona [OpenTelemetry Rust](https://github.com/open-telemetry/opentelemetry-rust) como único sistema de instrumentación y exportación de telemetría de este ensayo. Su función consiste en describir operaciones instrumentadas y medir el comportamiento de la aplicación dentro del [perímetro experimental](../README.md).

El diseño inicial utiliza su API y su SDK con las características estrictamente necesarias. No prevé un recolector independiente, un servidor de almacenamiento ni un segundo sistema de instrumentación. Las dependencias transitivas deberán declararse; su existencia no se confundirá con la implantación de otra aplicación. Las versiones y características concretas se fijarán al comprobar la construcción nativa y WebAssembly.

## Señales previstas

| Señal | Contenido mínimo |
|---|---|
| Identidad de ejecución | Identificador de ensayo y referencias a artefactos verificados. |
| Operación | Clase, inicio, fin y relación con la petición correspondiente. |
| Autorización | Referencia a la decisión del componente responsable, cuando proceda. |
| Resultado | Terminación normal, rechazo, cancelación, truncamiento o fallo. |
| Recursos | Duración, volumen de entrada y salida y medidas disponibles de memoria. |
| Accesos instrumentados | Recurso o destino declarado y resultado expuesto por la interfaz. |
| Cobertura | Fuentes de medida, pérdidas, errores de exportación y campos no observables. |

Las trazas y métricas no deberán atribuir una causa o intención que el punto de observación no permita comprobar. Los datos declarados por el modelo conservarán esa condición.

## Fronteras de validez

OpenTelemetry Rust no concede permisos ni impide por sí mismo operaciones. Tampoco registra automáticamente todas las llamadas del sistema operativo, todos los servicios, conexiones TCP o escrituras a disco.

Un registro emitido por el programa describe lo que ese punto de instrumentación expone. No constituye prueba independiente frente a un fallo capaz de alterar simultáneamente el programa y su registro. No se deducirá ausencia de actividad a partir de ausencia de trazas sin acreditar previamente la cobertura.

En navegador, la observación queda limitada por las interfaces disponibles. Las mediciones del ejecutor nativo y las del navegador deberán distinguirse. Una magnitud inaccesible se consignará como no observada.

## Volumen y privacidad

El diseño deberá acotar el número y tamaño de atributos, eventos, colas y lotes. La evidencia total exportada no superará 20 MiB por ejecución. Se contabilizarán los eventos descartados y se declarará cualquier truncamiento; no se afirmará completitud cuando existan pérdidas materiales.

Se registrarán metadatos mínimos. No se incorporarán por defecto consultas completas, respuestas completas, credenciales, identificadores personales ni rutas que permitan identificar a una persona. El cálculo de una huella de contenido sensible no se considerará anonimización suficiente.

La primera exportación será un artefacto acotado bajo control explícito del ensayo, sin envío automático a servicios externos. La implementación deberá comprobar el vaciado final y declarar el comportamiento ante fallos del destino.

## Coste de la instrumentación

Se compararán ejecuciones equivalentes con instrumentación habilitada y deshabilitada. Se medirán duración, memoria y bytes exportados, indicando repeticiones, dispersión y perturbaciones conocidas. No se fijará una sobrecarga porcentual como hecho antes de medirla.

## Estado

Componente seleccionado para experimentación. Integración, compatibilidad por plataforma y cobertura efectiva pendientes de comprobación.
