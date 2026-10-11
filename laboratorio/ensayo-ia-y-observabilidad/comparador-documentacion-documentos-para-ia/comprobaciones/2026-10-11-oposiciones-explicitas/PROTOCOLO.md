# Adaptación experimental delimitada de SC-XF-002

**Buscador-Semántico - Diferencial · 11 de octubre de 2026 · Paso 4 / 8.**

## Necesidad y límite del cambio

El contraste conservado en SV-motor@5e382c7b02cadae31f8ef41b276427fa75241c3a no produjo avisos en seis incompatibilidades preparadas. La inspección identifica dos limitaciones relacionadas: el analizador de speccheck-core admite should, pero su función entre documentos no lo contempla; además, la comparación por palabras descarta términos que pueden distinguir sujetos, condiciones y momentos. El cambio se limita a una copia experimental de esa función comunitaria. No introduce un analizador semántico nuevo, vocabularios preparados por revisores ni una IA de análisis. No modifica el Núcleo, el IR, el Árbitro - Director o la aplicación documental.

Se contrastarán tres perfiles: original sin ampliación; ampliación a should sin otra restricción; ampliación a should con coincidencia del resto de la afirmación. Este último exige conservar todas las palabras, cifras, signos y su orden, salvo el verbo modal y la negación inmediata, ignorando solamente mayúsculas y espacios repetidos. Es una restricción conservadora sobre una función existente. Las entradas no se reescriben. Las diferencias de modalidad y los textos completos permanecen visibles. No se equiparan obligación, recomendación y predicción ni se convierte una oposición local en contradicción concluyente del documento.

La distinción entre obligación y recomendación tiene un ejemplo formal en [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119.txt). [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174.txt) delimita aquella convención a sus usos previstos en mayúsculas. No se impone esa convención a la prosa clínica ni a las traducciones de Google; se cita para evitar identificar automáticamente should, must y will.

## Comprobaciones fijadas antes de ejecutar

Se fijan dieciséis controles nuevos, propios y en inglés: seis incompatibilidades, ocho compatibilidades y dos contextos insuficientes. Incluyen condiciones, sujetos, momentos, encabezamientos históricos, cuantificadores, referencias ambiguas, repetición y oposición sin negación. Son controles adversariales del mecanismo, no documentación clínica ni una muestra representativa de producción. Sus referencias se conservan en archivo separado y no se suministran al detector. Las evaluaciones no son independientes.

Los tres perfiles recibirán esos dieciséis controles. Los dos perfiles modificados recibirán también los doce casos documentales ya conocidos, con las traducciones conservadas sin cambios. Esta segunda parte es regresión y exploración sobre casos conocidos, no nueva validación independiente; el resultado original de cero avisos permanece intacto y no se vuelve a ejecutar esa combinación. Total previsto: 72 llamadas a la función, en una ejecución por combinación de caso y perfil. No se ajustarán reglas después de ver los resultados de este conjunto.

Antes de valorar integración se comprobará: generación de avisos en oposiciones explícitas; efecto sobre avisos espurios de sujetos y condiciones; conservación de contexto, modalidades y referencias; y persistencia de los límites no resueltos. Una mejora aislada no autoriza a presentar detección general ni integración operativa. Los avisos del original y las variantes conservarán sus identificadores de perfil. La confianza fija del componente no se presentará como probabilidad calibrada. Una ausencia de avisos no se calificará como compatibilidad.

## Conservación

Paquete de origen: speccheck-core 0.4.1, SHA-256 e0fcbea6024b695434a8f257c3ad5ae43455108f9faf7af44a165efc269ecd90, revisión e49eda84dcbb8f836d79b866cf6ae9f89704b375, licencia MIT. Se conservarán diferencia exacta del código, huellas, entradas, resultados y evaluación posterior en Rust. Las dependencias restantes conservarán sus ediciones. Los textos previos mantienen los derechos de sus fuentes, incluidos los derivados de INCIBE bajo CC BY-NC-SA 4.0. El pie siguiente se aplica al protocolo propio.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
