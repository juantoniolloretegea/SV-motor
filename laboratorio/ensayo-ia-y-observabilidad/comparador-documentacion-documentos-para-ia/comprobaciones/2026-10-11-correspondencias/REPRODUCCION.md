# Reproducción de la correspondencia y la presentación

El auxiliar Rust de esta carpeta comprueba exclusivamente la evidencia del paso 6. No contiene el detector ni consulta un servicio de traducción. Sus 28 pares, veinte traducciones y siete localizaciones son condiciones del conjunto experimental fijado, no capacidades generales de producción.

1. Recupere los archivos indicados en FUENTES-REMOTAS.json, en las rutas relativas de ENTRADAS-PUBLICACION.json y dentro de un directorio destinado a la reproducción. Las referencias fijan revisión y huella de cada entrada. Conserve la separación de las referencias de evaluación: no se requieren para esta comprobación.
2. Compile instrumento/Cargo.toml con Cargo.lock y Rust 1.98.1. Use `cargo build --locked --manifest-path instrumento/Cargo.toml`; si las dependencias ya están disponibles, puede añadir `--offline`.
3. Ejecute `bsd-correspondencias preparar ENTRADAS-PUBLICACION.json RAIZ SALIDA_NUEVA`. La salida no debe existir. Se comprueban identidad, correspondencia y doce controles de integridad antes de generar la muestra.
4. Con mdBook 0.5.4, ejecute `mdbook build SALIDA_NUEVA/consulta-mdbook`. Las fuentes preservan ambos contextos y muestran los textos como contenido inerte.
5. Ejecute `bsd-correspondencias verificar-html SALIDA_NUEVA/CORRESPONDENCIAS.json SALIDA_NUEVA/consulta-mdbook/book RESULTADO_HTML_NUEVO.json`.
6. Consulte la representación mediante un servicio local de lectura. La inspección visual debe registrarse separadamente del cotejo de texto.

La compilación comprobada utilizó serde_json 1.0.149, sha2 0.10.9 con force-soft y html-escape 0.2.15. El archivo COMPONENTE.json identifica el código, el ejecutable utilizado y mdBook. El ejecutable Windows se conserva entre los instrumentos de ejecución; la publicación contiene el código para su reproducción. Las licencias de dependencias y fuentes documentales conservan su aplicación.

Los informes históricos de cotejo HTML, anteriores a la normalización instrumental, se conservan como resultados no conformes. COMPROBACION-HTML-FINAL.json corresponde al código publicado. IDENTIDAD-FINAL-RUST.json acredita que las correcciones no cambiaron las correspondencias ni el HTML inspeccionado.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

