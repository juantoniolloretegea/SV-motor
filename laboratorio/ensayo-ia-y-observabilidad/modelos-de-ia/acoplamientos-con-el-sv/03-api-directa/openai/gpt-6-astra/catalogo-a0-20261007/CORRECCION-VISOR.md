# Corrección de la representación y entrega del visor Astra A0

Fecha: 07/10/2026. Versión del visor: 0.2.0. Seguimiento: S39 y TT-0021.

## Defecto constatado

La entrega anterior denominada POLIGONO-EGUI.html contenía una imagen estática de una ventana egui. La ventana representaba nueve botones en fila; no dibujaba el polígono. La indicación de seleccionar una posición dentro de la imagen no tenía efecto en el HTML. La comprobación de datos y captura realizada entonces no acreditaba geometría poligonal ni interacción del archivo entregado. Se rectifica expresamente la afirmación de presentación completa de S39 revisión 47, Acta 004 §41 y RETP-2026-286 en ese alcance gráfico.

La edición anterior permanece recuperable en la revisión a88ddfe7be8f5f8c8f535f0c46d61fd518e92786. No se alteran CAPA.json, las respuestas, la adjudicación, la puntuación ni la instrumentación de inferencia.

## Entrega corregida

[POLIGONO-EGUI.html](POLIGONO-EGUI.html) contiene el programa Rust compilado a WebAssembly, las fuentes gráficas, los datos adjudicados y el enlace técnico JavaScript generado por wasm-bindgen. Ese enlace inicia egui y transmite los acontecimientos del navegador; la geometría, la correspondencia posicional, la validación de la fuente y la interacción documental se realizan en Rust. No se afirma que el navegador o toda su infraestructura estén escritos en Rust.

El archivo es autónomo: no necesita descargar bibliotecas ni contactar con el modelo. La política del documento impide conexiones mediante `connect-src 'none'`. Debe descargarse el archivo completo, abrirse en un navegador con WebAssembly y WebGL habilitados y seleccionar un vértice, su rótulo o un botón A01–A09. GitHub muestra el código del archivo, no ejecuta su interfaz. No es necesario instalar Rust para consultarlo.

Se utiliza una figura cerrada de nueve posiciones en orden horario desde A01, arriba, con los radios gráficos 1, 2 y 3 para los símbolos 0, 1 y U. Se adopta explícitamente la leyenda 0 verde, 1 rojo y U ocre. Los nueve resultados de esta capa son 0: la figura queda en el círculo interior. No es un porcentaje pequeño ni un resultado insuficiente. Los radios codifican símbolos; no expresan magnitud clínica. La representación no convierte el vector posicional en un espacio vectorial ni introduce operaciones nuevas.

Referencia: [adenda de encaje visual del SV, 12/09/2026](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/76722442d2899300c69d89ab932890a4d7d74c2f/docs/calidad/tuberias-ia/frame-significado-humano-trazabilidad-y-fidelidad/ADENDA_ENCAJE_VISUAL_EXPERTO_AGENTE_Y_LOGO_SV_2026_09_12.md). La convención gráfica y su correspondencia con A01–A09 quedan declaradas para esta edición; no se trasladan parámetros clínicos de antecedentes históricos.

Cada posición presenta su fundamento, dificultad, condición crítica, los dos pasajes contrastados y las huellas de respuesta y auditoría. Las páginas 0 y 1 del archivo original se muestran como 1 y 2 para lectura humana, con indicación explícita de esa conversión. La interfaz no recalifica resultados.

## Comprobación y límites

Cuatro pruebas Rust conformes: rechazo de entradas incompletas o mal ordenadas, correspondencia radio/símbolo y orden, trazado cerrado y pulsación de los nueve vértices y los nueve botones con verificación del fundamento dibujado e inmutabilidad de la fuente.

En la vista HTTP local del mismo HTML se comprobaron A01, selección de A06 mediante vértice, selección de A09 mediante botón, ocultación y restitución de pasajes y despliegue de huellas. Sin errores ni avisos en la consola observada. La automatización del navegador no permite el protocolo file://: no se declara comprobada la apertura directa del archivo descargado. El programa y sus recursos están incluidos, sin solicitudes externas necesarias.

[Constancia](COTEJO-EGUI.json), [manifiesto del documento](MANIFIESTO-VISOR.json) y [fuentes e instrucciones de compilación](visor-egui/COMPILACION.md). La comprobación es de realización y presentación; no constituye auditoría científica independiente.

Esta corrección realizó cero llamadas nuevas al candidato y no repite la campaña A0. El consumo de las nueve inferencias originales continúa en sus informes económicos. No se atribuyen a esta corrección importes de asistencia o cuotas globales que no estén desglosados.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
