# Reconstrucción de la comprobación

La configuración requiere Rust y las dependencias fijadas en Cargo.lock. Obtener speccheck-core 0.4.1 de crates.io y comprobar el SHA-256 del archivo .crate: e0fcbea6024b695434a8f257c3ad5ae43455108f9faf7af44a165efc269ecd90. Extraerlo como speccheck-core-0.4.1, en un directorio contiguo a instrumento. Aplicar SC-XF-002.patch desde la raíz del paquete. No aplicar el cambio a otra versión ni a una instalación operativa.

Compilar mediante cargo build --locked --manifest-path instrumento/Cargo.toml. No habilitar características adicionales. En la ejecución documentada se utilizó --offline porque las dependencias ya estaban disponibles. Para recuperar los doce casos previos, utilizar la carpeta preparacion de la comprobación 2026-10-11-pasajes-traducidos en la revisión 5e382c7b02cadae31f8ef41b276427fa75241c3a; sus huellas se comprueban antes de ejecutar.

El instrumento admite dos operaciones separadas:

```text
comprobacion-buscador-semantico-diferencial detectar ENTRADAS-ADVERSARIALES.json PREPARACION_ANTERIOR DIRECTORIO_NUEVO
comprobacion-buscador-semantico-diferencial evaluar DIRECTORIO_NUEVO/RESULTADO.json REFERENCIAS-ADVERSARIALES.json REFERENCIAS-ANTERIORES.json EVALUACION_NUEVA.json
```

La primera no recibe referencias de evaluación. El directorio de salida debe ser nuevo; el instrumento rechaza la sustitución de resultados. La segunda exige las 72 combinaciones y contrasta las referencias después de la ejecución. La evaluación no es independiente. El registro JSONL conserva cada llamada terminada; una interrupción requeriría examinarlo antes de cualquier nueva ejecución. La reproducción es una instrucción técnica, no una declaración de que se haya ejecutado una segunda vez.

La fijación previa incluye código, entradas, referencias y ejecutable. El cotejo local anterior a la prueba acredita identidad en ese momento, no conservación remota ni revisión independiente. La conservación remota se comprueba después de publicar mediante recuperación por revisión y cotejo Rust.

Licencia del componente y de su cambio: MIT, manteniendo la atribución y licencia del proyecto de origen. Los documentos y pasajes previos conservan sus atribuciones y condiciones originales. El pie siguiente se aplica a esta explicación propia.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
