# Buscador-Semántico - Diferencial: resultado de admisión de Logicaffeine

**11 de octubre de 2026 · logicaffeine-language 0.10.1.**

## Resultado

**La configuración examinada no es apta para avanzar hacia la comparación documental prevista.** La función `compile` admite 8 de 12 enunciados especializados y rechaza 4. Entre los admitidos, al menos cuatro pierden información decisiva de las condiciones o de la acción descrita. Los dos controles elementales se admiten y producen las representaciones esperadas.

Esta comprobación evalúa la admisión y la conservación de significado en la salida pública de una función. No mide la detección de contradicciones, no examina los documentos completos y no demuestra que toda alternativa basada en análisis formal sea inviable.

| Grupo | Entradas | Admitidas | Rechazadas |
| --- | ---: | ---: | ---: |
| Controles | 2 | 2 | 0 |
| Medicina | 6 | 5 | 1 |
| Ciberseguridad | 6 | 3 | 3 |
| Total | 14 | 10 | 4 |

## Defectos observados

| Caso | Propiedad requerida | Resultado observado |
| --- | --- | --- |
| M01 | Restricción a pacientes con infección activa y prohibición hasta su control | Conserva prohibición y relación temporal, pero la salida no expresa la restricción de pacientes con infección activa. |
| M02 | Posibilidad de tratamiento para pacientes sin infección activa | La salida representa la posibilidad de recibir el tratamiento y omite la condición «sin infección activa». |
| M04 | Evaluación dentro de un día, equivalente temporal de M03 | Rechazo explícito ante la unidad «day». M03, expresado en 24 horas, sí se admite. |
| M05 | Contraindicación condicionada a insuficiencia renal grave | Ofrece dos formulaciones, una de contraindicación y otra de localización; ninguna conserva insuficiencia renal ni gravedad. |
| C03 | Ausencia de obligación de activar autenticación multifactor | Devuelve únicamente `¬Require(Guests)`, sin la acción ni su objeto. |
| C04–C05 | Conservación o borrado de registros con duración o plazo | Ambas entradas se rechazan antes de procesar el requisito completo. |
| C06 | Correspondencia entre versiones y requisitos TLS | Rechazo explícito en la oración que contrapone las dos versiones. |

M06 conserva de forma visible la relación entre las dos denominaciones compuestas. C01 y C02 conservan sujeto, acción, objeto y oposición de modalidad en sus representaciones. Esto no acredita todavía un procedimiento de detección entre documentos.

M03 conserva las menciones de paciente, obligación y plazo; su representación requiere además examinar el alcance de la variable de evento utilizada en `Within(e, 24 hours)`, situada fuera del cuantificador existencial que aparece en la cadena. No se declara formalmente correcta esa salida ni se utiliza para compensar las omisiones ya demostradas.

El símbolo `U` que aparece en la salida M01 es un operador temporal emitido por la biblioteca; **no es el valor U del SV**. Ninguna salida de esta prueba se incorpora al núcleo, a su semántica o a su representación intermedia.

## Método y reproducibilidad

Los catorce textos son sintéticos, nuevos y diagnósticos. Los dos controles proceden de ejemplos elementales empleados en la documentación del candidato; los doce restantes se redactaron para esta comprobación. No constituyen recomendaciones clínicas ni normas técnicas. Se fijaron entradas, referencias, protocolo, fuente del instrumento y Cargo.lock en Rust antes de cada ejecución.

Se realizaron dos ejecuciones. El [cotejo Rust](COTEJO-RUST.json) confirma identidad de las entradas y de los resultados sustantivos, integridad de la fijación previa, códigos de terminación correctos y ausencia de agotamiento del límite. Se conservan por separado los tiempos. No se modificaron términos ni frases para acomodarlos a la gramática y no se alteró el candidato.

La conformidad del cotejo acredita reproducción instrumental; no acredita conservación semántica. La identificación de las omisiones deriva del examen directo de las salidas completas frente a las propiedades fijadas. No se presenta como validación científica independiente.

## Identidad y dependencias

Se ejecutó la edición 0.10.1 publicada en crates.io, cuya referencia de procedencia es `d7c86c1bc55dc88abc38f08096192f86f9be82d0`. La revisión documental de main, `6cd62121820bebc2e77d20d93bbc4b53336c2295`, es posterior; no se confunden ambas. El manifiesto de la biblioteca y la explicación de su modo lógico coinciden por objeto Git entre esas revisiones. La compilación se realizó con Rust 1.98.1, dependencias fijadas y opciones predeterminadas desactivadas.

El árbol activo no incluye Z3, servicios de IA ni un motor de modelos. No se ejecutó el modo de generación de programas de Logicaffeine. Las bibliotecas logicaffeine conservan su licencia BUSL-1.1, que no es una licencia de código abierto sin restricciones; esta comprobación no implica decisión de despliegue, contratación ni adquisición de licencia.

## Consecuencia

No se inicia una integración operativa ni se presenta este candidato como solución del Buscador-Semántico - Diferencial. Una eventual corrección tendría que demostrar conservación de sujetos, condiciones, complementos y unidades, antes de evaluar contradicciones sobre casos independientes. No se justifica sustituir esa carencia por vocabularios preparados por los revisores ni por una IA que interprete los documentos.

Las pruebas de speccheck-core y Logicaffeine responden a objetos distintos y permanecen separadas. Sus cantidades no se suman para formar una puntuación conjunta.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
