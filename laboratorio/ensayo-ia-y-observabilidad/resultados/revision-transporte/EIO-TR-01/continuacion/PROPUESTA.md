# Recepción de EIO-TR-01 y propuesta de continuación 05

## Dictamen

La ejecución 35475928967, número 4 e intento 1, sobre fb963fbb8ac2a94c3066e9f81072ded4087d3f9e, obtuvo HTTP 302 y observó us.aws.cdn.hf.co como destino. El código 65 corresponde al rechazo de la lista local; no se siguió la redirección. Los metadatos de Actions y el diagnóstico publicado concuerdan. La cuenta histórica permanece: tres ejecuciones de la campaña inicial y una diagnóstica extraordinaria.

La documentación oficial de Hugging Face identifica expresamente us.aws.cdn.hf.co como nodo CDN de Estados Unidos y explica que las descargas se redirigen desde el Hub a hosts de almacenamiento. Fuente: [Downloading models, apartado Downloading behind a proxy or firewall](https://huggingface.co/docs/hub/models-downloading#downloading-behind-a-proxy-or-firewall).

El diagnóstico y la fuente oficial proporcionan fundamento para añadir exactamente ese host a la política hf del transporte experimental. No demuestran la identidad de los bytes del modelo, la ausencia de otras redirecciones ni cuál fue el host perdido en la ejecución 3. El nombre del host no acredita por sí mismo el lugar físico de procesamiento.

## Modificación concreta preparada

Base pública: 5a491664bc2762addfa4d2c6e330bf724d207467. Archivo transporte-cdn.patch: incorpora us.aws.cdn.hf.co a la lista exacta de adquirir.sh y añade cuatro comprobaciones al banco auxiliar de transporte. No se añade un comodín ni se altera la política rust. La modificación se entrega como parche pendiente de autorización, no aplicada a los guiones activos.

Con el candidato: 32 comprobaciones conformes mediante respuestas sintéticas sin red, retorno 0, GNU Bash 5.2.21. Se añaden admisión directa y por redirección del host, rechazo de un sufijo engañoso y rechazo bajo política rust. Resultado en COMPROBACION.tsv. Son pruebas del transporte Bash, no del SV, del modelo ni de TLS real.

Se mantiene obligatorio verificar bytes y SHA-256 antes de instalar o cargar. Pesos: 396705472 bytes, SHA-256 ac2d97712095a558e31573f62f466a3f9d93990898b0ec79d7c974c1780d524a. Tokenizador y distribuciones Rust conservan las identidades del manifiesto. No se modifican versiones, plantilla, dependencias, oráculos o límites.

## Continuación propuesta

Una ejecución adicional del mismo workflow existente: número 5, intento 1, máximo 40 minutos; plazo interno 2280 segundos. Plantilla completa preparada en workflow-ensayo-05.yml, fuera del directorio activo de workflows. Restaura la secuencia del ensayo, elimina la fase de preparación del selector y exige exactamente el número 5. No reinicia contadores.

Después de autorización expresa: aplicar el parche, instalar esa plantilla en la ruta del workflow existente, publicar precompromiso con SHA completo y fundamento de la apertura de las dos guardas, y ejecutar una sola vez fase ensayo. Confirmar antes que las cuatro ejecuciones anteriores terminaron y no hay otra en curso.

Se conservan dos trabajos de compilación, cotas de disco/memoria/evidencia y secuencia de aprovisionamiento, compilación, banco, siete inferencias, costes y comprobación de construcción WASM. No hay instalación, pesos, compilación o inferencia en el PC. Sin credenciales, nuevos recolectores, cachés o upload-artifact.

El descargador evaluará cada destino y se detendrá ante otro host no admitido. No se ampliará la lista automáticamente, no se reintentará y no se aplicarán correcciones sobre la marcha. Un fallo de compilación o inferencia se documentará sin inventar resultados posteriores. La comprobación WASM no acredita ejecución real en navegador.

## Estado receptor

Se admite el resultado diagnóstico como evidencia del host observado y se recomienda la modificación puntual descrita. No se concede aceptación científica al conjunto. Hasta una nueva autorización, guardas, transporte y workflow activos conservan su estado. Los cuatro archivos de esta propuesta son preparatorios. No se ha contactado con el CDN ni lanzado otra ejecución.
