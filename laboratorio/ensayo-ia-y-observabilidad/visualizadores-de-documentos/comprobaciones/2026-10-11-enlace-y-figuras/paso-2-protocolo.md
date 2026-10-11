# Paso 2. Representación de figuras y páginas PDF

Edición V.01 · 11 de octubre de 2026 · Protocolo anterior a la ejecución.

## Finalidad

Comprobar una representación gráfica independiente del visor PDF del navegador. La lectura seguirá organizada mediante mdBook. Una página representada conserva el contexto de imágenes, trazos, tablas dibujadas y texto superpuesto; no se presenta como extracción semántica ni reconocimiento automático de figuras.

El candidato es Hayro 0.8.0, biblioteca Rust de representación PDF. El proyecto declara un estado experimental y limitaciones de compatibilidad y rendimiento. Su evaluación no implica aceptación general ni sustitución del extractor textual existente. La edición publicada en crates.io se fija mediante Cargo.lock; la consulta de la documentación del repositorio corresponde a la revisión 718f3c76571b4650a264d9319b63ff547efdfe51.

## Primera comprobación

Se generan nueve PDF sintéticos propios, con un elemento por caso y criterios definidos antes de ejecutarlos:

| Caso | Propiedad | Comprobación instrumental |
| --- | --- | --- |
| 1 | Rectángulo vectorial | Dimensiones y superficie azul |
| 2 | Imagen RGB incrustada | Superficies de colores conocidas |
| 3 | Imagen, trazos y texto superpuestos | Conservación conjunta de color y trazo |
| 4 | Rotación de página | Dimensiones intercambiadas y superficie conservada |
| 5 | Recorte de página | Dimensiones y cobertura del recorte |
| 6 | Transparencia uniforme | Composición del azul al 50 % sobre blanco |
| 7 | Región de recorte gráfico | Superficie visible acotada |
| 8 | Texto y tabla dibujada | Presencia de trazos y texto; inspección visual adicional |
| 9 | Diagrama vectorial | Conservación de áreas y líneas; inspección visual adicional |

Cada caso se representa dos veces y se compara su identidad. También se representa a doble escala y se comprueban sus dimensiones. Se rechazan página cero, página inexistente, escala no admitida y archivo inválido. Se conservan original, imagen, huellas y duración de la representación. Los umbrales geométricos no sustituyen una evaluación editorial de fidelidad.

La prueba usa una página por solicitud, escalas 1 y 2, un máximo de 4.096 píxeles por dimensión y ocho millones de píxeles por imagen. Son cotas de esta prueba, no límites generales de mdBook ni de los libros. No se envían documentos a demostraciones públicas ni a modelos de IA.

## Condiciones para la incorporación

El resultado favorable del conjunto sintético sólo habilita a estudiar la integración. Antes de recibirla deberán comprobarse: documentos representativos autorizados, errores y omisiones, conservación de procedencia y página, protección de cada recurso gráfico, retirada y caducidad de permisos, límites de recursos, navegación y ampliación mediante teclado y separación entre representación visible y contenido efectivamente entregado al modelo.

Las imágenes se asociarán a la edición y huella del original. Una figura no recuperada deberá constar como no disponible; el texto alternativo no sustituye su contenido. Los recursos externos no se recuperarán automáticamente. Un PDF con una característica no admitida no podrá presentarse como reproducción íntegra acreditada.

La incorporación gráfica no concederá permisos de uso por IA, no decidirá licencias ni alterará decisiones anteriores. Los libros extensos y la presentación reunida de la entrega efectiva permanecen en los pasos 3 y 4.

## Fuentes

- [Hayro: alcance y limitaciones](https://github.com/LaurenzV/hayro/tree/718f3c76571b4650a264d9319b63ff547efdfe51).
- [Edición 0.8.0](https://crates.io/crates/hayro/0.8.0).
- [Libro de reglas de lectura y entrega documental](https://documentos-sv.itvia.online/documentacion/reglas-usos-descripciones-documentales/index.html).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
