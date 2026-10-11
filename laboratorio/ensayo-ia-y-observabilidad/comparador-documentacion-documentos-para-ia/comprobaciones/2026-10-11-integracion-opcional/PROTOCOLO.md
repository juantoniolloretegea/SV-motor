# Paso 7 / 8 · Integración experimental de la consulta opcional

**Buscador-Semántico - Diferencial · 11 de octubre de 2026.**

La integración reutilizará el registro transaccional y sus comprobaciones vigentes de admisión, consulta, usos y situación documental. Se fijan copias de las fuentes existentes para no modificar la administración concurrente. El resultado será una función Rust comprobada con registros sintéticos; no se desplegará en el servicio ni modificará el Núcleo, su representación intermedia o el Árbitro - Director.

La solicitud permitirá omitir la ayuda, elegir un documento, un conjunto explícito o todos los documentos comparables y accesibles del mismo dominio. Una selección vacía, duplicada, ajena o no autorizada no se ampliará. Se comprobará la admisión de todo el conjunto antes de recuperar contenido y comparar. Una modificación del registro durante la preparación impedirá entregar el resultado como vigente. La consulta no escribirá decisiones documentales.

Se utilizarán representaciones Markdown recibidas e identificadas por huella, con texto inglés original o traducción española–inglesa previamente conservada. El proveedor de contenido será una función del servicio autenticado; no realizará en este ensayo nuevas extracciones, traducciones o solicitudes de red. Los textos completos y los encabezamientos se conservarán. Un original PDF o HTML sin representación Markdown admisible no se convertirá automáticamente ni se omitirá para presentar una cobertura completa.

La regla experimental restrictiva conservará ambas localizaciones al emitir el aviso, sin cambiar sus condiciones de comparación. Se corregirá además el recorte de etiquetas que podía cortar un carácter UTF-8: afecta a la presentación del diagnóstico, no al criterio de detección. Los resultados históricos no se reconstruirán ni sobrescribirán. Las comprobaciones nuevas son instrumentales y no se añadirán a las cifras científicas de detección del contraste anterior.

Se comprobarán: omisión voluntaria, selección individual y por grupo, selección completa autorizada, conjunto vacío, identificadores duplicados, inclusión del propio documento, referencia ausente o de otro dominio, falta de permiso, admisión vencida o inexistente, original discordante, traducción sin correspondencia o permiso, representación no admitida, cambio concurrente y permanencia de ambos intervalos UTF-8. El control de selección completa excluirá los documentos sin los usos necesarios y los aún no recibidos; la cobertura efectiva quedará enumerada. Se comprobará que no se producen comparaciones entre referencias sin el documento propuesto.

Las cotas experimentales serán 128 referencias, 256 KiB por representación y 4 MiB para el conjunto. Superarlas producirá un resultado de operación no disponible, sin recortes silenciosos. Las pruebas registrarán lecturas, llamadas al comparador y duración. La corrección de selección, conservación y localización no acreditará detección general de contradicciones ni autenticación real mediante navegador.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
