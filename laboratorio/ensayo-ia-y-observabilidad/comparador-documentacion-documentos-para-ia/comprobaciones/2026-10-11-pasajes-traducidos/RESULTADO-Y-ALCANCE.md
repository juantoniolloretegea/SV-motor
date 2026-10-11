# Buscador-Semántico - Diferencial: contraste de pasajes traducidos

**11 de octubre de 2026 · Pasos 4 y 5 / 8; dependencia lingüística del paso 6.**

## Resultado principal

El recorrido **texto español → traducción literal de Google → SC-XF-002 de speccheck-core 0.4.1** no produjo ninguna alerta en los doce casos preparados. Quedaron sin señalar las seis incompatibilidades introducidas de forma controlada. No se ha demostrado utilidad de este perfil para estos documentos y no se selecciona para integración. Esta conclusión se refiere al recorrido y al conjunto comprobados; no demuestra la imposibilidad de construir una ayuda parcial ni modifica el carácter voluntario del Buscador-Semántico - Diferencial.

Las doce comprobaciones de selección fueron conformes en el instrumento experimental Rust. El control positivo de la función existente produjo la alerta prevista. Ninguno de estos dos resultados compensa la falta de detección en los textos de estudio.

## Fuentes y método

Se reutilizó el [conjunto fijado antes de la ejecución](https://github.com/juantoniolloretegea/SV-motor/blob/491406c8fdc3b4be83fe9d411e85956ec977f181/laboratorio/ensayo-ia-y-observabilidad/comparador-documentacion-documentos-para-ia/informes/2026-10-11-paso3/PROTOCOLO-Y-ESTADO.md): ocho pasajes de NCI-PDQ, LLS, INCIBE y CYB25, agrupados en cuatro documentos parciales de referencia; doce documentos propuestos, seis de inmunología y seis de ciberseguridad. Los papeles de incorporación se simulan exclusivamente en el ensayo. No se alteró la administración documental real.

Las seis incompatibilidades son modificaciones o controles deliberados; no se atribuyen a los autores de las fuentes. Los cuatro casos compatibles y los dos de contexto insuficiente mantienen su referencia previa. No se examinaron documentos completos, otras versiones, otros dominios ni NIS2.

Rust cotejó los 23 archivos de origen disponibles contra las huellas de la publicación fijada: 42.489 bytes. Esta comprobación acredita identidad local respecto de las huellas conservadas; la recuperación independiente de aquella publicación consta en su recibo anterior. Los [intercambios con Google](CAPTURAS-GOOGLE.json) contienen las entradas y salidas visibles íntegras. Rust comprobó los veinte identificadores, su correspondencia con los textos españoles y la ausencia de entradas omitidas o duplicadas. Las [traducciones separadas](preparacion/TRADUCCIONES.json) conservan originales, salidas inglesas y sus huellas.

Se retiraron únicamente rótulos de la interfaz, identificadores y espacios exteriores al separar las respuestas. Las cabeceras Markdown identifican documentos y pasajes; no reformulan su contenido. No se sustituyeron expresiones para ajustarlas al candidato. Cada consulta de la función recibió exactamente el documento propuesto y una referencia seleccionada del mismo dominio. No se efectuaron comparaciones entre referencias ni entre dominios. Las referencias de evaluación se consultaron después de conservar el resultado del detector.

Se ejecutó una vez el conjunto de doce casos y, por separado, un control instrumental positivo basado en el ejemplo público de la función. Se realizaron trece llamadas a SC-XF-002 en total. La duración registrada del conjunto de doce casos fue **421.615 microsegundos**; no es una medición del coste completo de preparación, traducción, compilación o asistencia.

## Resultados

| Casos | Referencia previa | Resultado observado |
| --- | --- | --- |
| I01, I03, I05 | Tres incompatibilidades de inmunología | Ninguna alerta; tres omisiones |
| C01, C03, C04 | Tres incompatibilidades de ciberseguridad | Ninguna alerta; tres omisiones |
| I02, I04, C02, C05 | Cuatro compatibilidades delimitadas | Ninguna alerta |
| I06, C06 | Contexto insuficiente | Ninguna alerta; no se interpreta como resolución del contexto |

La [evaluación Rust](EVALUACION-RUST.json) registra **0/6 incompatibilidades señaladas**, 0/4 avisos en compatibles y 0/2 avisos en casos de contexto insuficiente. Como no hubo avisos, no se puede estimar su precisión. El silencio ante textos compatibles tampoco acredita reconocimiento de compatibilidad. Las proporciones describen este conjunto controlado; no son una estimación de rendimiento en una población documental.

El [control positivo](CONTROL-POSITIVO.json) contrapone las expresiones inglesas de obligación y prohibición del ejemplo de SC-XF-002. Produjo una alerta. Acredita que el analizador y la llamada a la función pueden generar el aviso esperado con su patrón previsto; no valida los pasajes bibliográficos ni aumenta el número de aciertos de la tabla.

Las [comprobaciones de selección](preparacion/SELECCIONES-RESULTADO.json) cubren selección individual, grupo, todos los accesibles del mismo dominio, rechazo de referencias ajenas y grupos mixtos, selección vacía, ayuda no solicitada, restricción por permisos, grupo inexistente y dominio discordante del documento propuesto. Las doce coinciden con las previsiones fijadas. Se trata de un instrumento aislado con permisos y estados simulados; no acredita la integración ni la seguridad de la aplicación operativa.

## Explicación y límites

SC-XF-002 es una función Rust existente, con licencia MIT. Busca formas `shall`, `must` o `will`, su negación con `not`, un verbo coincidente y solapamiento de palabras. No realiza una interpretación general de significado. La implementación inspeccionada, `src/rules/cross_file.rs` del paquete [speccheck-core 0.4.1](https://gitlab.com/neilfitzgerald1972/speccheck), tiene como procedencia e49eda84dcbb8f836d79b866cf6ae9f89704b375. El paquete original tiene SHA-256 e0fcbea6024b695434a8f257c3ad5ae43455108f9faf7af44a165efc269ecd90. Se conserva Cargo.lock y no se introducen dependencias nuevas.

El analizador previo no reconoce requisitos en varios pasajes descriptivos, entre ellos los de linaje celular. Además, las traducciones emplean `should not`, `is not necessary` o negaciones descriptivas que no coinciden con el patrón examinado. En C03, Google devuelve `must be continuously verified` para la referencia y `should not be continuously verified` para el propuesto. La variación de modalidad limita la equivalencia lingüística; no se corrige silenciosamente. Los originales y sus traducciones permiten examinar por separado el efecto de la traducción y la cobertura de la función.

Las traducciones de un mismo texto dentro de solicitudes distintas también varían: por ejemplo, `begin treatment` y `start treatment`. En LLS, Google reúne varias denominaciones españolas en la repetición de `hairy cell leukemia`. Se conservan estas salidas sin presentar la identidad de archivos como garantía de equivalencia semántica. Las referencias de evaluación fueron elaboradas antes del contraste, pero no cuentan con validación independiente.

El selector de la Biblioteca mostró inglés y el aviso de carga, aunque la página continuó en español y se registró un error del servicio. La [incidencia puntual](INCIDENCIA-TRADUCTOR.json) no acredita indisponibilidad permanente ni determina su causa. Se obtuvo la traducción mediante la interfaz oficial de Google, sin modificar la Biblioteca. Esta preparación resuelve la disponibilidad de inglés para los veinte textos; todavía no constituye un servicio de traducción integrado en el comparador.

El instrumento no altera el Núcleo, su representación intermedia, el Árbitro - Director, documentos, permisos o decisiones documentales. La función de detección no consulta modelos ni carga pesos. La dependencia speccheck-nli aparece en la compilación de speccheck-core; no se construye ni se invoca un detector NLI en este recorrido. Google interviene exclusivamente en la traducción instrumental autorizada.

## Continuación de los ocho pasos

El paso 3 conserva el conjunto preliminar. Los pasos 4 y 5 concluyen el contraste de **este perfil**, con resultado insuficiente para integrarlo. El paso 6 queda comprobado sólo para la obtención, conservación y correspondencia de estos veinte textos; no queda completada la integración lingüística ni la cobertura general de formatos. El paso 7 no se inicia con este perfil. La aportación editorial al libro permanece preparada para incorporación coordinada; el libro existente y sus cinco capítulos no se han modificado.

La siguiente decisión técnica deberá apoyarse en una función existente que pueda demostrar sugerencias útiles en estos textos, o en una modificación delimitada cuya necesidad y alcance se expliquen. No procede ampliar la campaña, repetir las mismas entradas ni construir un analizador semántico nuevo para ocultar este resultado.

## Evidencias y derechos

[Protocolo](PROTOCOLO.md), [salida íntegra del detector](RESULTADO.json), [entradas efectivas](preparacion/ENTRADAS-EN.json), [instrumento Rust](instrumento/src/bin/pasajes.rs) y [control instrumental](instrumento/src/bin/control_xf.rs). Las representaciones inglesas se conservan junto con sus originales y las referencias de procedencia del paso 3. Los textos de terceros mantienen sus derechos; los derivados de INCIBE conservan CC BY-NC-SA 4.0. La licencia editorial siguiente se aplica al informe propio y no sustituye aquellas condiciones ni la licencia MIT de speccheck.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
