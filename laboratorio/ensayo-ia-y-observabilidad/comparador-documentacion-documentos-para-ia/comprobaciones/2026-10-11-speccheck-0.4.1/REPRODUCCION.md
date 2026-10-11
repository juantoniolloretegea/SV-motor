# Reproducción de la comprobación

Esta carpeta conserva el protocolo fijado, los veintiséis archivos sintéticos de entrada, las referencias de evaluación separadas, las dos ejecuciones y sus cotejos Rust. Los documentos bibliográficos originales no se duplican aquí; sus identidades se conservan en FIJACION-RUST.json.

El código de comprobación está en codigo. Cargo.toml y Cargo.lock fijan la realización utilizada. Es necesario disponer de Rust compatible con la versión mínima 1.95 del candidato. La realización examinada se compiló con Rust 1.98.1 en Windows.

Para reconstruir el programa, desde esta carpeta:

```text
cargo build --locked --manifest-path codigo/Cargo.toml --bin comprobacion-buscador-semantico-diferencial
cargo run --locked --manifest-path codigo/Cargo.toml --bin comprobacion-buscador-semantico-diferencial -- ENTRADAS.json . RESULTADO-REPRODUCIDO.json
```

La segunda orden conserva una salida nueva y rechaza sobrescribir una existente. El programa recibe el manifiesto de entrada y los casos, nunca REFERENCIAS.json. Comprueba sus huellas antes de analizarlos. Las funciones NLI y los servicios externos permanecen excluidos por configuración y selección explícita de las seis reglas.

La comparación sustantiva debe excluir las duraciones y el instante de ejecución. La huella del ejecutable puede cambiar entre compiladores y sistemas; no debe confundirse ese cambio con una modificación de las alertas. El cotejo conservado comparó dos ejecuciones del mismo ejecutable.

cotejar.rs separa la fijación previa y la valoración posterior. Su operación de fijación también admite un archivo local de ubicaciones de originales, excluido de esta publicación por contener rutas particulares. No se necesita ese archivo para reproducir el análisis de los veinticuatro casos publicados. auditar_resultado.rs comprueba la correspondencia de casos y las localizaciones de la primera salida.

La dependencia speccheck-core conserva licencia MIT; los demás componentes conservan sus licencias. La documentación y los casos propios mantienen el siguiente aviso. El diagnóstico no autoriza incorporación operativa ni uso clínico.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
