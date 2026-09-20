# EIO-06 · propuesta de corrección acotada

Estado: preparación publicada; sin autorización de una sexta ejecución, compilación nueva ni aceptación científica.

## Recepción de EIO-05

Base examinada: `f595bbbe67bbef07e795844b1c2e744aa0196842`. La [ejecución 35476464030](https://github.com/juantoniolloretegea/SV-motor/actions/runs/35476464030) acredita aprovisionamiento y compilación nativa, 24 controles directos conformes, siete salidas iguales rechazadas por estructura y fallo de comprobación WASM. Se conservan [resultados y límites](../../continuacion-05/RESULTADO.md).

Las siete generaciones son repeticiones de una petición para comparar costes de observación. No permiten estimar una tasa de error del modelo. El cercado Markdown y el objeto envolvente sustituyendo al campo `peticion` son dos defectos diferentes. El verificador de EIO-05 permanece intacto.

## Preguntas y cambios fijados

1. ¿Se reproduce la salida anterior con la misma petición y el mismo modelo?
2. ¿La única variante estructurada publicada produce una respuesta admitida por el contrato anterior?
3. ¿La selección de la fuente de aleatoriedad para navegador permite superar la comprobación de compilación WASM?

[peticiones.patch](peticiones.patch) admite exactamente las dos peticiones fijadas por contenido y registra su identificador. No modifica la generación, el muestreo ArgMax, los pesos, el tokenizador, el contrato ni la reparación de respuestas. La [petición estructurada](peticion-estructurada.txt) contiene un ejemplo con respuesta vacía que el modelo debe completar. Esto examina seguimiento de formato con ayuda explícita; no acredita razonamiento independiente ni suficiencia semántica.

[Cargo.navegador.toml](Cargo.navegador.toml) incorpora `getrandom 0.3.4` como dependencia opcional de `wasm32` con sistema `unknown`, activa `wasm_js` sólo mediante la característica `navegador` y añade un receptor de resultados escrito en Rust. No añade otro producto de observabilidad.

### Corrección de la interpretación inicial de getrandom

El mensaje registrado afirma que la característica por sí sola es insuficiente. Sin embargo, el [README oficial de v0.3.4](https://github.com/rust-random/getrandom/blob/v0.3.4/README.md#webassembly-support) y la [selección de backend](https://github.com/rust-random/getrandom/blob/v0.3.4/src/backends.rs) indican que `wasm_js` basta en ese destino; la configuración adicional anteriormente requerida dejó de ser necesaria. La propuesta sigue el código y la documentación de esa versión. No introduce RUSTFLAGS globales ni aleatoriedad simulada.

El lock contiene también getrandom 0.4.3. Su mera presencia no prueba que se compile para navegador. Se conserva y se registra el árbol efectivo de características; no se activa a ciegas otra versión. Esta corrección responde al primer error conocido y no garantiza ausencia de otros errores.

## Archivo de dependencias: preparación y fijación antes de medir

No se inventa un Cargo.lock nuevo ni se solicita su huella antes de que Cargo lo produzca. La ejecución propuesta comprende expresamente esta preparación:

1. Aprovisionar con el manifiesto y lock originales, mediante el guion ya utilizado y las identidades de EIO-05.
2. Obtener metadatos originales sin red. Sustituir el manifiesto exclusivamente en el checkout efímero por el candidato publicado.
3. Permitir a Cargo ajustar el lock sin red, conservando el anterior como punto de partida. No utilizar `cargo update`, `generate-lockfile` ni resolución adicional en red.
4. Exigir igualdad completa de nombre, versión y origen de los paquetes antes y después. Un paquete nuevo o una versión distinta detienen la preparación. Las relaciones de dependencia y características sí cambian como consecuencia de la declaración publicada.
5. Conservar íntegros el lock resultante, el manifiesto, el inventario y el árbol WASM; registrar sus tamaños y SHA-256 antes de compilar. Las compilaciones posteriores usan `--locked --offline` y las huellas se vuelven a cotejar.

El lock derivado será una salida de la preparación técnica incluida en esta ejecución; su identidad no se presenta como conocida antes del lanzamiento. Las entradas del modelo y el método experimental sí quedan fijados antes. [Procedimiento completo](resolver-dependencias.sh).

## Ejecución y criterios

[ensayar-06.sh](ensayar-06.sh) conserva el supervisor anterior y ejecuta:

| Fase | Límite |
|---|---:|
| Aprovisionamiento | 480 s |
| Ajuste y fijación de dependencias | 60 s |
| Compilación nativa | 600 s |
| Banco anterior | 30 s |
| Inferencia de referencia, con telemetría | 120 s |
| Inferencia estructurada, con telemetría | 120 s |
| Recepción determinista de salidas | 30 s |
| Comprobación WASM | 240 s |
| Suma de límites de fase | 1680 s |

Se mantienen 40 minutos externos, plazo interno de 2280 s, dos trabajos de compilación, límites de disco y memoria del supervisor, y entradas y salidas acotadas del adaptador. La diferencia de 600 s respecto al plazo interno absorbe preparación y otros costes; no es una predicción de duración. No se repite la comparación de costes on/off.

[recepcion.rs](recepcion.rs) exige reproducción literal de texto y tokens de la referencia y la misma clasificación ESTRUCTURA. Para la variante exige contrato OK y fin EOS. Reevalúa el texto sin extraerlo de Markdown ni repararlo. La validez del campo `respuesta` conserva el criterio anterior: no vacío; no se presenta como evaluación semántica. El código 2 indica resultado contractual adverso. Este resultado se conserva y permite realizar la comprobación WASM independiente; un fallo técnico de recepción o una interrupción del supervisor detienen la secuencia.

La comprobación WASM debe retornar 0 para declararse conforme en compilación. No se genera una afirmación de funcionamiento en navegador, de paridad, de WASI ni de conformidad integral. Relojes, enlace JavaScript y ejecución real en navegador conservan su pendiente.

## Perímetro y seguridad

La tarea pertenece al laboratorio de inferencia y observabilidad y conserva el perímetro de [su README](../../../README.md). No modifica el núcleo, S32/BIS-03 ni los dominios científicos.

Seguridad pasiva: delimitación de capacidades y recursos. Seguridad activa: validación y rechazo dentro de la frontera examinada. Observabilidad: eventos del adaptador y registros de ejecución. Estos registros no son vigilancia independiente del agente ejecutor ni evidencia de las operaciones internas de razonamiento del modelo.

## Aplicación posterior a autorización

Aplicar únicamente el parche de peticiones; conservar el Cargo.toml activo original, el lock original, los oráculos, el adaptador y la política de transporte. Copiar [workflow-ensayo-06.yml](workflow-ensayo-06.yml) al workflow existente únicamente tras autorización de una ejecución adicional. Publicar el fundamento y el SHA del precompromiso, abrir guardas sólo para esa ejecución y cerrarlas al entregar sus resultados. No crear otro workflow ni reiniciar la cuenta. Un intento, número 6; ninguna corrección o repetición posterior queda autorizada por esta propuesta.

El PC conserva la exclusión de instalaciones, pesos, compilación e inferencia. Los nuevos cambios de manifiesto durante la preparación ocurren sólo en el checkout remoto y quedan documentados.

## Comprobaciones preparatorias

Véase [COMPROBACION.tsv](COMPROBACION.tsv). Se ha comprobado sintaxis Bash y aplicación del parche a la base. La documentación y el código oficiales se han leído. No hay Rust/Cargo disponibles en el entorno revisor: el candidato Rust, la resolución del lock y las compilaciones siguen pendientes de la ejecución propuesta. La revisión estática no se presenta como prueba ejecutada.
