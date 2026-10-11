# Paso 2. Resultado de la primera comprobación gráfica

Edición V.01 · 11 de octubre de 2026.

**Resultado: comprobación instrumental inicial favorable; integración operativa pendiente.** Hayro 0.8.0 representa los nueve PDF sintéticos propios del protocolo, con conservación de las propiedades geométricas examinadas, reproducción idéntica en las dos representaciones de cada caso y ampliación al doble de dimensiones.

## Evidencia y alcance

Los casos comprueban rectángulos y diagramas vectoriales, imagen RGB, superposición de imagen y texto, rotación, recorte de página, transparencia, región de recorte gráfico y tabla dibujada. Las imágenes de los casos 3, 8 y 9 se inspeccionaron además en su versión ampliada. Se conservan originales, imágenes, código y huellas en este expediente. No se utiliza bibliografía médica ni documentación privada para esta comprobación inicial.

Los cuatro rechazos previstos son conformes: página cero, página inexistente, escala no admitida y contenido no PDF. Tres controles adicionales del propio comprobador detectan separador interrumpido, borde interrumpido y tabla sin texto.

Se corrigió un defecto del instrumento: el umbral inicial de píxeles casi negros no caracterizaba correctamente una tabla con líneas suavizadas. Se conservan las dos ejecuciones iniciales y la [rectificación motivada](antecedentes/RECTIFICACION-DEL-COMPROBADOR.md). La tercera ejecución mantiene los mismos PDF y las mismas imágenes; la corrección afecta a su comprobación geométrica. Las repeticiones no se contabilizan como casos independientes ni como validación externa.

## Procedencia del componente

Se usa el paquete publicado Hayro 0.8.0. Su información de procedencia indica la revisión ea9c81dc70cde40c64205986fdcfcd481fe934d8. La revisión del repositorio consultada para documentación, 718f3c76571b4650a264d9319b63ff547efdfe51, es posterior y no se confunde con el código ejecutado. La resolución exacta de dependencias consta en Cargo.lock, SHA-256 28d626aa6ecf938f7a2ad0da1d94cc4de9d4bcd811c562ce5b9a33d9ad9e95f2. Se conservaron las licencias de los componentes; la licencia de este informe no las sustituye.

La composición examinada utiliza bibliotecas Rust y no incorpora PDFium, MuPDF ni un visor PDF del navegador. Esto no equivale a certificar todas sus dependencias. El proyecto declara expresamente un estado experimental y limitaciones. Las fuentes estándar pueden sustituirse por las incorporadas al componente; la fidelidad tipográfica de documentos reales requiere cotejo.

## Continuación del paso 2

Esta prueba resuelve la viabilidad gráfica básica, pero todavía no incorpora la función a la administración. La continuación debe comprobar documentos representativos autorizados y tratamiento de incidencias; después, recursos gráficos protegidos dentro de la lectura mdBook, procedencia por página, ampliación mediante teclado y aplicación de permisos, retirada y caducidad a cada imagen.

No se acredita reconocimiento semántico de figuras, OCR, extracción estructural de tablas, fidelidad de todos los PDF ni suministro gráfico a modelos. La presentación operativa existente conserva sus límites publicados. Los pasos 3 —libros extensos— y 4 —entrega efectiva— permanecen separados.

## Fuentes y material comprobable

- [Protocolo previo](paso-2-protocolo.md).
- [Resultados instrumentales](figuras/resultados.json).
- [Código del instrumento](instrumento/src/main.rs).
- [Hayro 0.8.0](https://crates.io/crates/hayro/0.8.0).
- [Alcance declarado por el proyecto](https://github.com/LaurenzV/hayro/tree/718f3c76571b4650a264d9319b63ff547efdfe51).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
